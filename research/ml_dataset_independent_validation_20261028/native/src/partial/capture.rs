//! Independent committed-prefix replay. Raw bytes, including LF, are hashed;
//! opaque event bodies never run a solver. Read handling follows std::io::Read
//! (https://doc.rust-lang.org/std/io/trait.Read.html): short reads are allowed,
//! Interrupted is retried, and no read requests bytes beyond the size sentinel.
use super::{
    budget::{Budget, LINE_MAX, MAX_ITEMS, MAX_STREAM, SEGMENT_MAX, SNAPSHOT_ARTIFACTS},
    io,
    schema::*,
};
use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::{ErrorKind, Read};

/// The frozen infallible genesis API allocates one fixed-size digest String.
/// Every caller must reserve this envelope BEFORE calling it. prepare() does.
pub(crate) const GENESIS_ALLOCATION: usize = 128;
const BUFFER: usize = 16 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CaptureState {
    pub cursor: Cursor,
    pub stop: Option<Stop>,
}

pub(crate) trait CaptureReader {
    fn open(&mut self, artifact: &Artifact) -> Result<Box<dyn Read>>;
}

pub(crate) fn genesis(binding_sha256: &str) -> Cursor {
    let mut hash = Sha256::new();
    hash.update(b"crypto.ml-dataset.capture-genesis/v1\n");
    hash.update(binding_sha256.as_bytes());
    hash.update(b"\n");
    let mut last_event_sha256 = String::with_capacity(64);
    replace_hex(&mut last_event_sha256, &hash.finalize());
    Cursor {
        byte_end: 0,
        event_count: 0,
        last_event_sha256,
        next_chunk: 0,
        payload_count: 0,
        payload_bytes: 0,
    }
}

fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b).context("partial capture counter overflow")
}

fn validate_cursor(c: &Cursor, initial: &Cursor) -> Result<()> {
    ensure!(
        crate::common::valid_hex(&c.last_event_sha256, 64)
            && c.byte_end <= MAX_STREAM
            && c.event_count <= MAX_ITEMS
            && c.next_chunk <= MAX_ITEMS
            && c.payload_count <= c.event_count
            && c.payload_bytes <= MAX_STREAM
            && ((c.byte_end == 0) == (c.event_count == 0))
            && (c.event_count != 0 || c == initial)
            && (c.event_count == 0 || (c.next_chunk > 0 && c.next_chunk <= c.event_count)),
        "partial capture invalid cursor"
    );
    Ok(())
}

fn payload_path(path: &str) -> Result<()> {
    validate_path(path)?;
    let name = path
        .strip_prefix("payloads/")
        .context("partial payload namespace")?;
    ensure!(
        !name.is_empty() && name.len() <= 255 && !name.contains('/'),
        "partial payload basename"
    );
    Ok(())
}

/// Borrowed identities avoid unbounded path/Artifact clones. Sort is in-place.
struct Index<'a> {
    entries: Vec<&'a Artifact>,
}
impl<'a> Index<'a> {
    fn new(artifacts: &'a [Artifact], budget: &mut Budget) -> Result<Self> {
        let n = artifacts.len();
        let charge = n
            .checked_mul(std::mem::size_of::<&Artifact>())
            .and_then(|v| v.checked_add(64))
            .context("consumer_resource_limit: capture index")?;
        budget.charge(charge, "capture borrowed descriptor index")?;
        let mut entries = Vec::new();
        entries.try_reserve_exact(n)?;
        entries.extend(artifacts.iter());
        entries.sort_unstable_by(|a, b| a.path.cmp(&b.path));
        ensure!(
            entries.windows(2).all(|pair| pair[0].path != pair[1].path),
            "partial capture duplicate artifact path"
        );
        Ok(Self { entries })
    }
    fn get(&self, path: &str) -> Result<(usize, &'a Artifact)> {
        let i = self
            .entries
            .binary_search_by(|a| a.path.as_str().cmp(path))
            .map_err(|_| anyhow::anyhow!("partial capture reference lacks same-delta artifact"))?;
        Ok((i, self.entries[i]))
    }
}

struct Prepared<'a> {
    start: Cursor,
    declared: CaptureState,
    index: Index<'a>,
    binding_sha: String,
}

fn chunk_path_matches(path: &str, counter: u64) -> bool {
    // Format on the stack: exactly 12 ASCII digits, no heap path allocation.
    let Some(digits) = path
        .strip_prefix("events/")
        .and_then(|s| s.strip_suffix(".jsonl"))
    else {
        return false;
    };
    digits.len() == 12
        && digits.bytes().all(|b| b.is_ascii_digit())
        && digits.parse::<u64>() == Ok(counter)
}

fn prepare<'a>(
    delta: &'a Delta,
    prior: Option<&CaptureState>,
    binding: &Binding,
    budget: &mut Budget,
) -> Result<Prepared<'a>> {
    validate_binding(binding)?;
    ensure!(
        &delta.binding == binding,
        "partial capture binding mismatch"
    );
    ensure!(
        delta.segments.len() as u64 <= MAX_ITEMS && delta.artifacts.len() <= SNAPSHOT_ARTIFACTS,
        "partial capture member limit"
    );
    // Genesis and two owned state cursors each retain only one hex64 String.
    // Keep this cumulative even when a preceding structural pass was dropped.
    budget.charge(GENESIS_ALLOCATION + 384, "capture genesis and owned states")?;
    let binding_sha = binding_digest(binding, budget)?;
    let initial = genesis(&binding_sha);
    let start = prior.map_or(&initial, |p| &p.cursor);
    validate_cursor(start, &initial)?;
    validate_cursor(&delta.cursor, &initial)?;
    let c = &delta.cursor;
    let h = &delta.head;
    ensure!(
        h.schema == HEAD_SCHEMA
            && h.binding_sha256 == binding_sha
            && h.event_count == c.event_count
            && h.byte_end == c.byte_end
            && h.last_event_sha256 == c.last_event_sha256
            && h.payload_count == c.payload_count
            && h.payload_bytes == c.payload_bytes,
        "partial capture head/cursor mismatch"
    );
    ensure!(
        c.event_count >= start.event_count
            && c.byte_end >= start.byte_end
            && c.next_chunk >= start.next_chunk
            && c.payload_count >= start.payload_count
            && c.payload_bytes >= start.payload_bytes
            && ((c.byte_end == start.byte_end) == (c.event_count == start.event_count)),
        "partial capture cursor regression"
    );
    if c.byte_end == start.byte_end {
        ensure!(c == start, "partial unchanged prefix changed its cursor");
    }
    if let Some(old) = prior.filter(|p| p.stop.is_some()) {
        ensure!(
            h.stop == old.stop && c == &old.cursor,
            "partial stopped capture advanced or changed stop"
        );
    }
    let index = Index::new(&delta.artifacts, budget)?;
    let mut total = 0;
    let mut event_members = 0u64;
    let mut payload_members = 0u64;
    let mut payload_bytes = 0u64;
    for a in &delta.artifacts {
        validate_artifact(a)?;
        total = add(total, a.bytes)?;
        ensure!(total <= MAX_STREAM * 2, "partial capture delta byte limit");
        if a.path.starts_with("events/") {
            ensure!(
                a.bytes > 0 && a.bytes <= SEGMENT_MAX,
                "partial capture segment size"
            );
            event_members = add(event_members, 1)?;
        } else {
            payload_path(&a.path)?;
            payload_members = add(payload_members, 1)?;
            payload_bytes = add(payload_bytes, a.bytes)?;
        }
    }
    ensure!(
        event_members == delta.segments.len() as u64
            && add(start.payload_count, payload_members)? == c.payload_count
            && add(start.payload_bytes, payload_bytes)? == c.payload_bytes
            && payload_members <= c.event_count - start.event_count,
        "partial capture declared payload/member totals"
    );
    let mut end = start.byte_end;
    let mut sequence = start.event_count;
    let mut chunk = start.next_chunk;
    let mut last_hash = start.last_event_sha256.as_str();
    for s in &delta.segments {
        let (_, a) = index.get(&s.artifact.path)?;
        ensure!(
            a == &s.artifact
                && chunk_path_matches(&a.path, chunk)
                && a.role == "committed-solver-event-prefix"
                && a.encoding == "jsonl"
                && s.byte_start == end
                && s.byte_end == add(end, a.bytes)?
                && s.first_sequence == add(sequence, 1)?
                && s.last_sequence >= s.first_sequence
                && s.last_sequence <= MAX_ITEMS
                && s.last_sequence - sequence <= a.bytes
                && s.preceding_event_sha256 == last_hash
                && crate::common::valid_hex(&s.last_event_sha256, 64),
            "partial capture segment order/range/identity"
        );
        end = s.byte_end;
        sequence = s.last_sequence;
        last_hash = &s.last_event_sha256;
        chunk = add(chunk, 1)?;
    }
    ensure!(
        end == c.byte_end
            && sequence == c.event_count
            && chunk == c.next_chunk
            && last_hash == c.last_event_sha256,
        "partial capture declared segment end mismatch"
    );
    Ok(Prepared {
        start: start.clone(),
        declared: CaptureState {
            cursor: c.clone(),
            stop: h.stop,
        },
        index,
        binding_sha,
    })
}

/// Structural admission only: this state is NOT a consumed-byte proof.
pub(crate) fn admit_delta(
    delta: &Delta,
    prior: Option<&CaptureState>,
    binding: &Binding,
    budget: &mut Budget,
) -> Result<CaptureState> {
    Ok(prepare(delta, prior, binding, budget)?.declared)
}

fn open(
    reader: &mut dyn CaptureReader,
    a: &Artifact,
    budget: &mut Budget,
) -> Result<Box<dyn Read>> {
    let charge = a
        .path
        .len()
        .checked_mul(8)
        .and_then(|n| n.checked_add(1024))
        .context("consumer_resource_limit: capture open path")?;
    budget.charge(charge, "capture descriptor-rooted open scratch")?;
    reader.open(a)
}

fn read(input: &mut dyn Read, buf: &mut [u8], budget: &mut Budget) -> Result<usize> {
    loop {
        match input.read(buf) {
            Ok(n) => {
                ensure!(n <= buf.len(), "partial reader violated Read contract");
                budget.read(n as u64)?;
                return Ok(n);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e.into()),
        }
    }
}

fn hash_matches(bytes: &[u8], expected: &str) -> bool {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    expected.len() == bytes.len() * 2
        && bytes.iter().enumerate().all(|(i, &b)| {
            expected.as_bytes()[2 * i] == HEX[usize::from(b >> 4)]
                && expected.as_bytes()[2 * i + 1] == HEX[usize::from(b & 15)]
        })
}

fn verify_payload(a: &Artifact, reader: &mut dyn CaptureReader, budget: &mut Budget) -> Result<()> {
    let mut input = open(reader, a, budget)?;
    let mut buf = [0u8; BUFFER];
    let mut count = 0u64;
    let mut hash = Sha256::new();
    loop {
        let limit = usize::try_from((a.bytes - count + 1).min(BUFFER as u64))?;
        let n = read(input.as_mut(), &mut buf[..limit], budget)?;
        if n == 0 {
            break;
        }
        count = add(count, n as u64)?;
        ensure!(count <= a.bytes, "partial payload exceeds declared size");
        hash.update(&buf[..n]);
    }
    ensure!(
        count == a.bytes && hash_matches(&hash.finalize(), &a.sha256),
        "partial payload consumed bytes mismatch"
    );
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload<'a> {
    path: &'a str,
    role: &'a str,
    encoding: &'a str,
    sha256: &'a str,
    bytes: u64,
}

fn record(
    line: &[u8],
    binding_sha: &str,
    expected: &mut Cursor,
    index: &Index<'_>,
    seen: &mut [bool],
    reader: &mut dyn CaptureReader,
    budget: &mut Budget,
) -> Result<()> {
    ensure!(
        !line.is_empty() && line.len() as u64 <= LINE_MAX && line.ends_with(b"\n"),
        "partial incomplete or oversized committed event"
    );
    let scratch = line
        .len()
        .checked_add(1)
        .and_then(|n| n.checked_mul(4))
        .context("consumer_resource_limit: event decoder scratch")?;
    budget.charge(scratch, "capture fresh event decoder scratch")?;
    let row: EventRecord = io::typed(line, budget)?;
    let sequence = add(expected.event_count, 1)?;
    ensure!(
        sequence <= MAX_ITEMS
            && row.schema == EVENT_SCHEMA
            && row.binding_sha256 == binding_sha
            && row.sequence == sequence
            && row.previous_event_sha256 == expected.last_event_sha256,
        "partial committed event schema/binding/sequence/hash"
    );
    if row.event.get("kind").and_then(serde_json::Value::as_str)
        == Some("capture_payload_committed")
    {
        let value = row
            .event
            .get("artifact")
            .context("partial payload event missing artifact")?;
        // Deserialize from the already-admitted Value by borrowing strings.
        // No Value clone or owned Artifact copy is allocated here.
        let p = Payload::deserialize(value)?;
        payload_path(p.path)?;
        let (i, a) = index.get(p.path)?;
        ensure!(
            !seen[i]
                && p.role == a.role
                && p.encoding == a.encoding
                && p.sha256 == a.sha256
                && p.bytes == a.bytes,
            "partial payload lacks unique matching same-delta artifact"
        );
        verify_payload(a, reader, budget)?;
        seen[i] = true;
        expected.payload_count = add(expected.payload_count, 1)?;
        expected.payload_bytes = add(expected.payload_bytes, p.bytes)?;
        ensure!(
            expected.payload_count <= sequence && expected.payload_bytes <= MAX_STREAM,
            "partial replay payload limit"
        );
    }
    expected.event_count = sequence;
    expected.byte_end = add(expected.byte_end, line.len() as u64)?;
    ensure!(
        expected.byte_end <= MAX_STREAM,
        "partial replay stream limit"
    );
    replace_hex(&mut expected.last_event_sha256, &Sha256::digest(line));
    Ok(())
}

pub(crate) fn replay(
    delta: &Delta,
    prior: Option<&CaptureState>,
    binding: &Binding,
    reader: &mut dyn CaptureReader,
    budget: &mut Budget,
) -> Result<CaptureState> {
    let p = prepare(delta, prior, binding, budget)?;
    let mut expected = p.start;
    let mask_bytes = p
        .index
        .entries
        .len()
        .checked_add(64)
        .context("consumer_resource_limit: capture membership")?;
    budget.charge(mask_bytes, "capture referenced membership")?;
    let mut seen = Vec::new();
    seen.try_reserve_exact(p.index.entries.len())?;
    seen.resize(p.index.entries.len(), false);
    // Two simultaneous fixed stack buffers (segment and payload), plus the
    // reusable line capacity. Event-owned trees are charged anew in record().
    let line_capacity = usize::try_from(LINE_MAX)?
        .checked_add(1)
        .context("consumer_resource_limit: capture line capacity")?;
    budget.charge(
        line_capacity
            .checked_add(2 * BUFFER + 1024)
            .context("consumer_resource_limit: capture stream scratch")?,
        "capture reusable stream buffers",
    )?;
    let mut line = Vec::new();
    line.try_reserve_exact(line_capacity)?;
    let mut buf = [0u8; BUFFER];
    for s in &delta.segments {
        let (member, a) = p.index.get(&s.artifact.path)?;
        ensure!(!seen[member], "partial segment referenced twice");
        let mut input = open(reader, a, budget)?;
        let mut consumed = 0u64;
        let mut hash = Sha256::new();
        loop {
            // Both the artifact and the unfinished line get a single sentinel.
            // An infinite suffix is never drained after either bound fails.
            let limit = usize::try_from((a.bytes - consumed + 1).min(BUFFER as u64))?
                .min(line_capacity - line.len());
            let n = read(input.as_mut(), &mut buf[..limit], budget)?;
            if n == 0 {
                break;
            }
            consumed = add(consumed, n as u64)?;
            ensure!(consumed <= a.bytes, "partial segment exceeds declared size");
            hash.update(&buf[..n]);
            let mut remaining = &buf[..n];
            while !remaining.is_empty() {
                let len = remaining
                    .iter()
                    .position(|&b| b == b'\n')
                    .map_or(remaining.len(), |i| i + 1);
                ensure!(
                    line.len()
                        .checked_add(len)
                        .is_some_and(|n| n as u64 <= LINE_MAX),
                    "partial committed event exceeds line limit"
                );
                line.extend_from_slice(&remaining[..len]);
                if line.last() == Some(&b'\n') {
                    record(
                        &line,
                        &p.binding_sha,
                        &mut expected,
                        &p.index,
                        &mut seen,
                        reader,
                        budget,
                    )?;
                    line.clear();
                }
                remaining = &remaining[len..];
            }
        }
        ensure!(
            line.is_empty() && consumed == a.bytes && hash_matches(&hash.finalize(), &a.sha256),
            "partial segment consumed bytes/hash/final LF mismatch"
        );
        ensure!(
            expected.byte_end == s.byte_end
                && expected.event_count == s.last_sequence
                && expected.last_event_sha256 == s.last_event_sha256,
            "partial segment recomputed end mismatch"
        );
        expected.next_chunk = add(expected.next_chunk, 1)?;
        seen[member] = true;
    }
    ensure!(
        seen.iter().all(|seen| *seen) && expected == p.declared.cursor,
        "partial capture replay membership/cursor mismatch"
    );
    Ok(CaptureState {
        cursor: expected,
        stop: p.declared.stop,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, collections::BTreeMap, io::Cursor as Bytes, rc::Rc};
    const BINDING: &[u8] = include_bytes!(
        "../../../../ml_dataset_consumer_partial_20261029/design/vectors/binding-canonical.bin"
    );
    const FIRST: &[u8] = include_bytes!(
        "../../../../ml_dataset_consumer_partial_20261029/design/vectors/event-1.jsonl"
    );
    const SECOND: &[u8] = include_bytes!(
        "../../../../ml_dataset_consumer_partial_20261029/design/vectors/event-2.jsonl"
    );
    const BINDING_HASH: &str = "cf6eb83dd75443e2077e3fac761650963f380209f683b70e565b9e6fb76f8b67";
    const GENESIS_HASH: &str = "ecaa553fa5789d49cb42686de0cc59ec8db26a5af6d76140b5ac89a61abf564a";
    const FIRST_HASH: &str = "c38840a0a8188d791f3d2689d20ab452b8577092f860ebd1e08139c79d83bfca";
    const SECOND_HASH: &str = "9f86063dc854838c04e5a58d7b3eecd31da8d55be614374602e2c4f8f67c23a4";

    struct Memory {
        files: BTreeMap<String, Vec<u8>>,
        opens: usize,
    }
    impl CaptureReader for Memory {
        fn open(&mut self, a: &Artifact) -> Result<Box<dyn Read>> {
            self.opens += 1;
            Ok(Box::new(Bytes::new(
                self.files.get(&a.path).context("fixture missing")?.clone(),
            )))
        }
    }
    fn fixture() -> (Binding, Delta, Memory) {
        let binding: Binding = serde_json::from_slice(BINDING).unwrap();
        let first = Artifact {
            path: "events/000000000000.jsonl".into(),
            role: "committed-solver-event-prefix".into(),
            encoding: "jsonl".into(),
            sha256: FIRST_HASH.into(),
            bytes: 273,
        };
        let second = Artifact {
            path: "events/000000000001.jsonl".into(),
            sha256: SECOND_HASH.into(),
            bytes: 444,
            ..first.clone()
        };
        let payload = Artifact {
            path: "payloads/empty.bin".into(),
            role: "test-payload".into(),
            encoding: "raw".into(),
            sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".into(),
            bytes: 0,
        };
        let cursor = Cursor {
            byte_end: 717,
            event_count: 2,
            last_event_sha256: SECOND_HASH.into(),
            next_chunk: 2,
            payload_count: 1,
            payload_bytes: 0,
        };
        let delta = Delta {
            binding: binding.clone(),
            head: Head {
                schema: HEAD_SCHEMA.into(),
                binding_sha256: BINDING_HASH.into(),
                event_count: 2,
                byte_end: 717,
                last_event_sha256: SECOND_HASH.into(),
                payload_count: 1,
                payload_bytes: 0,
                stop: None,
            },
            segments: vec![
                Segment {
                    artifact: first.clone(),
                    byte_start: 0,
                    byte_end: 273,
                    first_sequence: 1,
                    last_sequence: 1,
                    preceding_event_sha256: GENESIS_HASH.into(),
                    last_event_sha256: FIRST_HASH.into(),
                },
                Segment {
                    artifact: second.clone(),
                    byte_start: 273,
                    byte_end: 717,
                    first_sequence: 2,
                    last_sequence: 2,
                    preceding_event_sha256: FIRST_HASH.into(),
                    last_event_sha256: SECOND_HASH.into(),
                },
            ],
            artifacts: vec![first, second, payload],
            cursor,
        };
        let files = [
            ("events/000000000000.jsonl".into(), FIRST.to_vec()),
            ("events/000000000001.jsonl".into(), SECOND.to_vec()),
            ("payloads/empty.bin".into(), vec![]),
        ]
        .into_iter()
        .collect();
        (binding, delta, Memory { files, opens: 0 })
    }

    #[test]
    fn literal_vectors_replay_consumed_event_and_empty_payload_bytes() {
        let (b, d, mut r) = fixture();
        assert_eq!(genesis(BINDING_HASH).last_event_sha256, GENESIS_HASH);
        let state = replay(&d, None, &b, &mut r, &mut Budget::new()).unwrap();
        assert_eq!(state.cursor, d.cursor);
        assert_eq!(
            (
                state.cursor.byte_end,
                state.cursor.event_count,
                state.cursor.next_chunk
            ),
            (717, 2, 2)
        );
        assert_eq!(
            (
                state.cursor.payload_count,
                state.cursor.payload_bytes,
                r.opens
            ),
            (1, 0, 3)
        );
    }

    #[test]
    fn structural_rejection_precedes_artifact_open_and_allocation_refusal() {
        let (b, mut d, mut r) = fixture();
        d.segments[1].byte_start += 1;
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new()).is_err());
        assert_eq!(r.opens, 0);
        let (_, d, _) = fixture();
        assert!(admit_delta(&d, None, &b, &mut Budget::with_limit(0)).is_err());
    }

    fn replace_second(d: &mut Delta, r: &mut Memory, raw: Vec<u8>) {
        let bytes = raw.len() as u64;
        let hash = crate::common::digest(&raw);
        r.files.insert(d.segments[1].artifact.path.clone(), raw);
        d.segments[1].artifact.bytes = bytes;
        d.segments[1].artifact.sha256 = hash.clone();
        d.artifacts[1] = d.segments[1].artifact.clone();
        d.segments[1].byte_end = 273 + bytes;
        d.segments[1].last_event_sha256 = hash.clone();
        d.cursor.byte_end = 273 + bytes;
        d.cursor.last_event_sha256 = hash.clone();
        d.head.byte_end = 273 + bytes;
        d.head.last_event_sha256 = hash;
    }

    #[test]
    fn rehashed_wrong_sequence_and_missing_lf_are_semantic_rejections() {
        let (b, mut d, mut r) = fixture();
        let mut row: serde_json::Value = serde_json::from_slice(SECOND).unwrap();
        row["sequence"] = 9.into();
        let mut raw = serde_json::to_vec(&row).unwrap();
        raw.push(b'\n');
        replace_second(&mut d, &mut r, raw);
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new())
            .unwrap_err()
            .to_string()
            .contains("sequence"));
        let (b, mut d, mut r) = fixture();
        replace_second(&mut d, &mut r, SECOND[..SECOND.len() - 1].to_vec());
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new()).is_err());
    }

    #[test]
    fn same_delta_payload_membership_cannot_be_replaced_by_equal_empty_bytes() {
        let (b, mut d, mut r) = fixture();
        d.artifacts[2].path = "payloads/other.bin".into();
        r.files.insert("payloads/other.bin".into(), vec![]);
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new())
            .unwrap_err()
            .to_string()
            .contains("same-delta"));
    }

    #[test]
    fn payload_second_read_is_bound_to_its_consumed_bytes() {
        let (b, mut d, mut r) = fixture();
        d.artifacts[2].bytes = 1;
        d.artifacts[2].sha256 = crate::common::digest(b"x");
        let mut event: serde_json::Value = serde_json::from_slice(SECOND).unwrap();
        event["event"]["artifact"] = serde_json::to_value(&d.artifacts[2]).unwrap();
        let mut raw = serde_json::to_vec(&event).unwrap();
        raw.push(b'\n');
        replace_second(&mut d, &mut r, raw);
        d.cursor.payload_bytes = 1;
        d.head.payload_bytes = 1;
        r.files.insert("payloads/empty.bin".into(), b"y".to_vec());
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new())
            .unwrap_err()
            .to_string()
            .contains("payload consumed bytes"));
        r.files.insert("payloads/empty.bin".into(), b"x".to_vec());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new()).is_ok());
    }

    #[test]
    fn rehashed_event_duplicate_key_and_stale_raw_hash_reject() {
        let (b, mut d, mut r) = fixture();
        let raw = std::str::from_utf8(SECOND)
            .unwrap()
            .replacen("\"sequence\":2", "\"sequence\":2,\"sequence\":2", 1)
            .into_bytes();
        replace_second(&mut d, &mut r, raw);
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new())
            .unwrap_err()
            .to_string()
            .contains("duplicate"));
        let (b, mut d, mut r) = fixture();
        let mut raw = SECOND.to_vec();
        raw.insert(1, b' ');
        replace_second(&mut d, &mut r, raw);
        // Outer object SHA is refreshed, but the committed cursor still names
        // the old raw-line hash. This must pass structure and fail byte replay.
        d.segments[1].last_event_sha256 = SECOND_HASH.into();
        d.cursor.last_event_sha256 = SECOND_HASH.into();
        d.head.last_event_sha256 = SECOND_HASH.into();
        assert!(admit_delta(&d, None, &b, &mut Budget::new()).is_ok());
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new())
            .unwrap_err()
            .to_string()
            .contains("recomputed end"));
    }

    #[test]
    fn repeated_admission_spends_one_ledger_without_resetting() {
        let (b, d, _) = fixture();
        let mut measured = Budget::with_limit(1024 * 1024);
        admit_delta(&d, None, &b, &mut measured).unwrap();
        let once = measured.used();
        assert!(once >= GENESIS_ALLOCATION);
        let mut bounded = Budget::with_limit(once);
        admit_delta(&d, None, &b, &mut bounded).unwrap();
        assert!(admit_delta(&d, None, &b, &mut bounded).is_err());
    }

    #[test]
    fn sticky_stop_and_empty_capture_are_distinct_from_no_delta() {
        let (b, mut d, mut r) = fixture();
        d.segments.clear();
        d.artifacts.clear();
        d.cursor = genesis(BINDING_HASH);
        d.head.event_count = 0;
        d.head.byte_end = 0;
        d.head.last_event_sha256 = GENESIS_HASH.into();
        d.head.payload_count = 0;
        d.head.stop = Some(Stop::StorageFailure);
        let stopped = replay(&d, None, &b, &mut r, &mut Budget::new()).unwrap();
        assert_eq!(r.opens, 0);
        assert!(admit_delta(&d, Some(&stopped), &b, &mut Budget::new()).is_ok());
        d.head.stop = None;
        assert!(admit_delta(&d, Some(&stopped), &b, &mut Budget::new()).is_err());
        let (_, advanced, _) = fixture();
        assert!(admit_delta(&advanced, Some(&stopped), &b, &mut Budget::new()).is_err());
    }

    struct Infinite {
        consumed: Rc<Cell<usize>>,
    }
    impl Read for Infinite {
        fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
            bytes.fill(b'x');
            self.consumed.set(self.consumed.get() + bytes.len());
            Ok(bytes.len())
        }
    }
    struct InfiniteReader {
        consumed: Rc<Cell<usize>>,
    }
    impl CaptureReader for InfiniteReader {
        fn open(&mut self, _: &Artifact) -> Result<Box<dyn Read>> {
            Ok(Box::new(Infinite {
                consumed: self.consumed.clone(),
            }))
        }
    }

    #[test]
    fn infinite_payload_suffix_stops_at_exact_declared_plus_one() {
        let (_, d, _) = fixture();
        let consumed = Rc::new(Cell::new(0));
        let mut r = InfiniteReader {
            consumed: consumed.clone(),
        };
        assert!(verify_payload(&d.artifacts[2], &mut r, &mut Budget::new()).is_err());
        assert_eq!(consumed.get(), 1);
    }

    #[test]
    fn infinite_event_suffix_stops_at_line_plus_one() {
        let (b, mut d, _) = fixture();
        d.segments.truncate(1);
        d.artifacts.truncate(1);
        let size = LINE_MAX + 100;
        d.artifacts[0].bytes = size;
        d.segments[0].artifact.bytes = size;
        d.segments[0].byte_end = size;
        d.cursor = Cursor {
            byte_end: size,
            event_count: 1,
            last_event_sha256: FIRST_HASH.into(),
            next_chunk: 1,
            payload_count: 0,
            payload_bytes: 0,
        };
        d.head.byte_end = size;
        d.head.event_count = 1;
        d.head.last_event_sha256 = FIRST_HASH.into();
        d.head.payload_count = 0;
        let consumed = Rc::new(Cell::new(0));
        let mut r = InfiniteReader {
            consumed: consumed.clone(),
        };
        assert!(replay(&d, None, &b, &mut r, &mut Budget::new()).is_err());
        assert_eq!(consumed.get() as u64, LINE_MAX + 1);
    }
}
