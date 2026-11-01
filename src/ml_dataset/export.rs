//! A derived, explicitly projected training view. Raw artifacts remain canonical.
//!
//! Feature records never copy a whole source JSON object. Labels and provenance
//! live in separate restricted files. A bounded Rust external merge deduplicates
//! feature identities without keeping the corpus or a corpus-sized hash set in
//! memory. Export takes the source lease to obtain a stable local snapshot.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::storage;
use super::types::Artifact;

#[path = "algebraic_targets.rs"]
mod algebraic_targets;
#[path = "prefix_algebra.rs"]
mod prefix_algebra;
#[path = "prime_chain_graph.rs"]
mod prime_chain_graph;
#[path = "prime_chain_prefix.rs"]
mod prime_chain_prefix;

const CHUNK_BYTES: usize = 16 * 1024 * 1024;
const MAX_RECORD_BYTES: usize = 64 * 1024 * 1024;
const MERGE_FAN_IN: usize = 8;
const SPLIT_DOMAIN: &str = "ml-dataset-whole-field-size-split-v1";
const STANDARD_PDP_DOMAIN: &str = "cyclic-subgroup-factor-base";
const KUMMER_PDP_DOMAIN: &str = "prime-kummer-full-curve";
const FROBENIUS_LARGE_PRIME_DOMAIN: &str = "cyclic-subgroup-signed-frobenius-large-prime";
const GENERIC_LARGE_PRIME_DOMAIN: &str = "cyclic-subgroup-identity-signed-large-prime";
const LARGE_PRIME_DOMAIN_ROLE: &str = "exact-large-prime-discovery-domains";
const LARGE_PRIME_TRANSCRIPT_ROLE: &str = "large-prime-elimination-transcript";

fn text(value: &Value, name: &str) -> Result<String, String> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("missing string {name}"))
}

fn integer(value: &Value, name: &str) -> Result<u64, String> {
    value
        .as_u64()
        .ok_or_else(|| format!("missing nonnegative integer {name}"))
}

fn boolean(value: &Value, name: &str) -> Result<bool, String> {
    value
        .as_bool()
        .ok_or_else(|| format!("missing boolean {name}"))
}

fn decimal(value: &Value, name: &str) -> Result<String, String> {
    let s = text(value, name)?;
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("{name} is not a decimal integer string"));
    }
    Ok(num_bigint::BigUint::parse_bytes(s.as_bytes(), 10)
        .ok_or_else(|| format!("invalid integer {name}"))?
        .to_string())
}

fn point(value: &Value) -> Result<Value, String> {
    if value.is_null() {
        return Ok(Value::Null);
    }
    Ok(json!({"x":decimal(&value["x"],"point.x")?,"y":decimal(&value["y"],"point.y")?}))
}

fn points(value: &Value, name: &str) -> Result<Value, String> {
    let rows = value
        .as_array()
        .ok_or_else(|| format!("missing array {name}"))?;
    Ok(Value::Array(
        rows.iter().map(point).collect::<Result<_, _>>()?,
    ))
}

fn action(value: &Value, name: &str) -> Result<Value, String> {
    let rows = value
        .as_array()
        .ok_or_else(|| format!("missing action {name}"))?;
    rows.iter()
        .map(|v| {
            if v.is_null() {
                Ok(Value::Null)
            } else {
                integer(v, name).map(Value::from)
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Value::Array)
}

/// This is an allowlist by structure and scalar type, not a recursive blacklist.
/// Future fields on curves, points, certificates, candidates, or orbit records
/// cannot silently become model features.
fn feature_projection(public: &Value, base: &Value) -> Result<Value, String> {
    if public["schema"] != "ml-ecdlp-instance/v1" || base["schema"] != "ml-ecdlp-factor-base/v1" {
        return Err("unsupported source feature schema".into());
    }
    if base["rule"] == "planted-known-log-subgroup/v1" {
        return Err("planted control requires normalized v7 feature projection".into());
    }
    let source_field = &public["field"];
    let mut field = json!({
        "characteristic":decimal(&source_field["characteristic"],"field.characteristic")?,
        "degree":integer(&source_field["degree"],"field.degree")?,
        "basis":text(&source_field["basis"],"field.basis")?,
    });
    if source_field.get("modulus").is_some() {
        field["modulus"] = json!(decimal(&source_field["modulus"], "field.modulus")?);
    }
    if let Some(terms) = source_field.get("irreducible_low_terms") {
        field["irreducible_low_terms"] = Value::Array(
            terms
                .as_array()
                .ok_or("irreducible low terms must be an array")?
                .iter()
                .map(|v| integer(v, "irreducible term").map(Value::from))
                .collect::<Result<_, _>>()?,
        );
    }
    let c = &public["curve"];
    let curve = json!({
        "model":text(&c["model"],"curve.model")?,
        "a":decimal(&c["a"],"curve.a")?,"b":decimal(&c["b"],"curve.b")?,
        "group_order":decimal(&c["group_order"],"curve.group_order")?,
        "subgroup_order":decimal(&c["subgroup_order"],"curve.subgroup_order")?,
        "cofactor":decimal(&c["cofactor"],"curve.cofactor")?,
    });
    let symmetry = &public["symmetry"];
    let orbits = base["orbits"]
        .as_array()
        .ok_or("missing factor-base orbits")?
        .iter()
        .map(|orbit| {
            Ok(json!({"points":points(&orbit["points"],"orbit.points")?,
            "closed":boolean(&orbit["closed"],"orbit.closed")?,
            "stabilizer_size":integer(&orbit["stabilizer_size"],"orbit.stabilizer_size")?}))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({
        "pdp_domain":STANDARD_PDP_DOMAIN,
        "family":text(&public["family"],"family")?,"n":integer(&public["n"],"n")?,
        "m":integer(&public["m"],"m")?,"twist":boolean(&public["twist"],"twist")?,
        "field":field,"curve":curve,"generator":point(&public["generator"])?,
        "target":point(&public["target"])?,
        "symmetry":{"coefficient_field_degree":integer(&symmetry["coefficient_field_degree"],"coefficient_field_degree")?,
            "frobenius_power":integer(&symmetry["frobenius_power"],"frobenius_power")?,
            "frobenius_nontrivial_on_model":boolean(&symmetry["frobenius_nontrivial_on_model"],"frobenius_nontrivial_on_model")?,
            "negation":boolean(&symmetry["negation"],"negation")?},
        "factor_base":{"rule":text(&base["rule"],"factor_base.rule")?,
            "points":points(&base["points"],"factor_base.points")?,
            "negation_action":action(&base["negation_action"],"negation_action")?,
            "frobenius_action":action(&base["frobenius_action"],"frobenius_action")?,
            "frobenius_power":integer(&base["frobenius_power"],"factor_base.frobenius_power")?,"orbits":orbits},
    }))
}

/// Causal projection prepared for the planted-control integration. Normalize
/// policy provenance for both populations; coordinates and mathematical
/// actions remain visible, while the constructed-positive flag stays private.
fn feature_projection_v7(public: &Value, base: &Value) -> Result<Value, String> {
    let mut normalized = base.clone();
    if base["rule"] != "planted-known-log-subgroup/v1"
        && base["rule"] != "small-x-prime-primary-projection-with-cyclic-membership"
    {
        return Err("unknown factor-base policy for v7 projection".into());
    }
    normalized["rule"] = json!("explicit-verified-point-set/v1");
    feature_projection(public, &normalized)
}

/// Only the public mathematical definition enters the identity. Cost, logs,
/// seeds, construction history, and future fields on the artifact stay out.
fn kummer_definition_projection(definition: &Value) -> Result<Value, String> {
    if definition["kind"] != "prime-kummer" || definition["subgroup_filtered"] != false {
        return Err("unsupported Kummer feature domain".into());
    }
    Ok(json!({
        "rule":"prime-kummer",
        "t":integer(&definition["t"],"Kummer height")?,
        "g":decimal(&definition["g"],"Kummer coset")?,
        "points":points(&definition["points"],"Kummer points")?,
        "subgroup_filtered":false,
    }))
}

fn pkm_feature_projection(public: &Value, base: &Value, pkm: &Value) -> Result<Value, String> {
    super::solvers::verify_pkm_factor_base(public, base, pkm)?;
    let mut projected = feature_projection(public, base)?;
    projected["pdp_domain"] = json!(KUMMER_PDP_DOMAIN);
    projected["factor_base"] = kummer_definition_projection(&pkm["definition"])?;
    Ok(projected)
}

fn signed_domain_projection(domain: &Value) -> Result<Value, String> {
    let signed = domain["signed"]
        .as_array()
        .ok_or("missing signed-column map")?
        .iter()
        .map(|row| {
            Ok(json!({"column":integer(&row["column"],"signed column")?,
                "coefficient":integer(&row["coefficient"],"signed coefficient")?}))
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(
        json!({"points":points(&domain["points"],"signed-domain points")?,
        "columns":points(&domain["columns"],"signed-domain columns")?,"signed":signed}),
    )
}

/// Keep only public domain structure, never pool-construction receipts, logs,
/// sampled queries or elimination results. The transcript is replayed before
/// its projected definition is compared with the separately saved domain.
fn large_prime_feature_projection(
    public: &Value,
    base: &Value,
    domain: &Value,
    transcript: &Value,
) -> Result<Value, String> {
    super::relations::verify_transcript(public, base, transcript)?;
    let mut projected = feature_projection(public, base)?;
    let m = integer(&public["m"], "relation arity")?;
    let small_summands = m.checked_sub(2).ok_or("invalid large-prime arity")?;
    match transcript["schema"].as_str() {
        Some("ml-ecdlp-large-prime-transcript/v1") => {
            let actual = json!({
                "small_points":points(&domain["small_points"],"small points")?,
                "residual_points":points(&domain["residual_points"],"residual points")?,
                "lambda":decimal(&domain["lambda"],"Frobenius eigenvalue")?,
                "frobenius_power":integer(&domain["frobenius_power"],"Frobenius power")?,
            });
            let expected = json!({
                "small_points":points(&transcript["small_base"]["points"],"verified small points")?,
                "residual_points":points(&transcript["residual_pool"]["points"],"verified residual points")?,
                "lambda":decimal(&transcript["model"]["lambda"],"verified Frobenius eigenvalue")?,
                "frobenius_power":1,
            });
            if actual != expected || transcript["model"]["frobenius_power"] != 1 {
                return Err(
                    "saved Frobenius large-prime domain differs from verified transcript".into(),
                );
            }
            let orbits = transcript["small_base"]["signed_orbits"]
                .as_array()
                .ok_or("missing signed Frobenius orbits")?
                .iter()
                .map(|row| {
                    row.as_array()
                        .ok_or("invalid signed Frobenius orbit")?
                        .iter()
                        .map(|v| integer(v, "orbit point index").map(Value::from))
                        .collect::<Result<Vec<_>, _>>()
                        .map(Value::Array)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let signed = transcript["small_base"]["signed_orbit_of"]
                .as_array()
                .ok_or("missing signed Frobenius column map")?
                .iter()
                .map(|row| {
                    let row = row
                        .as_array()
                        .filter(|r| r.len() == 3)
                        .ok_or("invalid signed Frobenius column map")?;
                    Ok(json!({"column":integer(&row[0],"orbit column")?,
                        "frobenius_exponent":integer(&row[1],"orbit exponent")?,
                        "negated":boolean(&row[2],"orbit sign")?}))
                })
                .collect::<Result<Vec<Value>, String>>()?;
            projected["pdp_domain"] = json!(FROBENIUS_LARGE_PRIME_DOMAIN);
            projected["factor_base"] = json!({"rule":"signed-frobenius-closure",
                "points":actual["small_points"],"signed_orbits":orbits,"signed":signed});
            projected["residual_domain"] = json!({"points":actual["residual_points"]});
            projected["large_prime_action"] = json!({"kind":"frobenius-with-signed-negation",
                "lambda":actual["lambda"],"frobenius_power":1});
        }
        Some("ml-ecdlp-generic-large-prime-transcript/v1") => {
            if domain["orbit_action"] != "identity-with-signed-negation" {
                return Err("invalid generic large-prime orbit action".into());
            }
            let small = signed_domain_projection(&domain["small"])?;
            let residual = signed_domain_projection(&domain["residual"]["domain"])?;
            if small != signed_domain_projection(&transcript["small"])?
                || residual != signed_domain_projection(&transcript["residual"]["domain"])?
            {
                return Err(
                    "saved generic large-prime domain differs from verified transcript".into(),
                );
            }
            projected["pdp_domain"] = json!(GENERIC_LARGE_PRIME_DOMAIN);
            projected["factor_base"] = small;
            projected["factor_base"]["rule"] = json!("identity-with-signed-negation");
            projected["residual_domain"] = residual;
            projected["large_prime_action"] = json!({"kind":"identity-with-signed-negation"});
        }
        _ => return Err("unsupported large-prime transcript schema".into()),
    }
    projected["relation_query"] = json!({"target_form":"a*generator+b*target",
        "small_summands":small_summands,"residual_summands":2,
        "identity_residual_pairs_admitted":false});
    Ok(projected)
}

fn large_prime_artifact<'a>(
    artifacts: &'a [Artifact],
    role: &str,
    paths: &[&str],
) -> Result<Option<&'a Artifact>, String> {
    let mut found = None;
    for artifact in artifacts
        .iter()
        .filter(|a| a.role == role || paths.contains(&a.path.as_str()))
    {
        if artifact.role != role
            || artifact.encoding != "json"
            || !paths.contains(&artifact.path.as_str())
            || found.replace(artifact).is_some()
        {
            return Err(format!("ambiguous or invalid {role} artifact"));
        }
    }
    Ok(found)
}

fn large_prime_cell_projection(
    directory: &Path,
    artifacts: &[Artifact],
    public: &Value,
    base: &Value,
) -> Result<Option<Value>, String> {
    let domain = large_prime_artifact(
        artifacts,
        LARGE_PRIME_DOMAIN_ROLE,
        &[
            "large-prime-domains.json",
            "generic-large-prime-domains.json",
        ],
    )?;
    let transcript = large_prime_artifact(
        artifacts,
        LARGE_PRIME_TRANSCRIPT_ROLE,
        &[
            "large-prime-transcript.json",
            "generic-large-prime-transcript.json",
        ],
    )?;
    let (Some(domain), Some(transcript)) = (domain, transcript) else {
        // Partial live payloads remain in restricted provenance. Without the
        // replayable transcript this view cannot certify the exact domain.
        return Ok(None);
    };
    let generic = transcript.path == "generic-large-prime-transcript.json";
    if (domain.path == "generic-large-prime-domains.json") != generic {
        return Err("large-prime domain/transcript route mismatch".into());
    }
    let domain: Value = super::read_json(&directory.join(&domain.path))?;
    let transcript: Value = super::read_json(&directory.join(&transcript.path))?;
    if generic != (transcript["schema"] == "ml-ecdlp-generic-large-prime-transcript/v1") {
        return Err("large-prime transcript path/schema mismatch".into());
    }
    let inputs = large_prime_feature_projection(public, base, &domain, &transcript)?;
    let recovered = if transcript["recovered_scalar"].is_null() {
        None
    } else if generic {
        Some(integer(&transcript["recovered_scalar"], "recovered scalar")?.to_string())
    } else {
        Some(decimal(
            &transcript["recovered_scalar"],
            "recovered scalar",
        )?)
    };
    for certificate in artifacts
        .iter()
        .filter(|a| a.role == "restricted-independent-scalar-label")
    {
        let label: Value = super::read_json(&directory.join(&certificate.path))?;
        if Some(decimal(&label["scalar"], "independent scalar")?) != recovered
            || point(&label["generator"])? != inputs["generator"]
            || point(&label["target"])? != inputs["target"]
        {
            return Err("large-prime scalar certificate differs from verified transcript".into());
        }
    }
    Ok(Some(inputs))
}

/// A verified success certificate must refer to the same domain as its row.
/// Hash consistency alone does not bind two independently stored definitions.
fn verify_projected_pdp_domain(
    directory: &Path,
    artifacts: &[Artifact],
    inputs: &Value,
) -> Result<(), String> {
    for certificate in artifacts
        .iter()
        .filter(|a| a.role == "independently-verified-pdp")
    {
        let label: Value = super::read_json(&directory.join(&certificate.path))?;
        if [FROBENIUS_LARGE_PRIME_DOMAIN, GENERIC_LARGE_PRIME_DOMAIN]
            .iter()
            .any(|domain| inputs["pdp_domain"] == *domain)
        {
            return Err(
                "target PDP certificate cannot label a large-prime relation-query domain".into(),
            );
        }
        if inputs["pdp_domain"] == KUMMER_PDP_DOMAIN {
            if kummer_definition_projection(&label["factor_base_definition"])?
                != inputs["factor_base"]
            {
                return Err("PDP certificate differs from exported Kummer factor base".into());
            }
        } else if label.get("factor_base_definition").is_some() {
            return Err("PDP certificate overrides the exported standard factor base".into());
        }
    }
    Ok(())
}

fn split_for_size(n: u64) -> (String, &'static str) {
    let component = format!("field-size-n{n}");
    let hash = super::digest(format!("{SPLIT_DOMAIN}\n{component}").as_bytes());
    let bucket = u64::from_str_radix(&hash[..16], 16).expect("hex digest") % 100;
    (
        component,
        if bucket < 80 {
            "train"
        } else if bucket < 90 {
            "validation"
        } else {
            "test"
        },
    )
}

#[derive(Eq, PartialEq)]
struct Record {
    key: String,
    json: String,
}

struct CappedWriter {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for CappedWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "export row exceeds {}-byte limit; split the source artifact",
                    self.limit
                ),
            ));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn bounded_json(value: &Value, limit: usize) -> Result<String, String> {
    let mut writer = CappedWriter {
        bytes: Vec::new(),
        limit,
    };
    serde_json::to_writer(&mut writer, value).map_err(|e| e.to_string())?;
    String::from_utf8(writer.bytes).map_err(|e| e.to_string())
}

impl Record {
    fn new(key: &str, value: &Value) -> Result<Self, String> {
        crate::store::index::validate_sha256(key)?;
        let json = bounded_json(value, MAX_RECORD_BYTES)?;
        Ok(Self {
            key: key.into(),
            json,
        })
    }
    fn write(&self, writer: &mut impl Write) -> Result<(), String> {
        writeln!(writer, "{}\t{}", self.key, self.json).map_err(|e| e.to_string())
    }
}

struct HeapEntry {
    record: Record,
    run: usize,
}
impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.record.key == other.record.key && self.run == other.run
    }
}
impl Eq for HeapEntry {}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .record
            .key
            .cmp(&self.record.key)
            .then_with(|| other.run.cmp(&self.run))
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn new_file(path: &Path) -> Result<File, String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))
}

fn bounded_line(reader: &mut impl BufRead, limit: usize) -> Result<Vec<u8>, String> {
    let mut line = Vec::new();
    loop {
        let available = reader.fill_buf().map_err(|e| e.to_string())?;
        if available.is_empty() {
            return Ok(line);
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let take = newline.map_or(available.len(), |position| position + 1);
        if take > limit.saturating_sub(line.len()) {
            return Err("external-sort record exceeds bounded line limit".into());
        }
        line.extend_from_slice(&available[..take]);
        reader.consume(take);
        if newline.is_some() {
            return Ok(line);
        }
    }
}

fn next_record(reader: &mut BufReader<File>) -> Result<Option<Record>, String> {
    // 64 hex key bytes, tab and newline are the only non-JSON bytes. Reject
    // before extending the buffer, including a malformed unterminated row.
    let line = bounded_line(reader, MAX_RECORD_BYTES + 66)?;
    if line.is_empty() {
        return Ok(None);
    }
    if !line.ends_with(b"\n") {
        return Err("invalid external-sort record".into());
    }
    let mut line = String::from_utf8(line).map_err(|e| e.to_string())?;
    line.pop();
    let (key, json) = line
        .split_once('\t')
        .ok_or("external-sort record has no key")?;
    crate::store::index::validate_sha256(key)?;
    Ok(Some(Record {
        key: key.into(),
        json: json.into(),
    }))
}

/// Merge at most eight runs, retaining at most one row per run. In the final
/// pass, equal IDs must have identical payloads or export fails closed.
fn merge(runs: &[PathBuf], destination: &Path, final_jsonl: bool) -> Result<u64, String> {
    if runs.len() > MERGE_FAN_IN {
        return Err("merge fan-in limit exceeded".into());
    }
    let mut readers = runs
        .iter()
        .map(|path| {
            File::open(path)
                .map(BufReader::new)
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut heap = BinaryHeap::new();
    for (run, reader) in readers.iter_mut().enumerate() {
        if let Some(record) = next_record(reader)? {
            heap.push(HeapEntry { record, run });
        }
    }
    let mut writer = BufWriter::new(new_file(destination)?);
    let mut previous: Option<Record> = None;
    let mut count = 0u64;
    while let Some(entry) = heap.pop() {
        if final_jsonl {
            let duplicate = previous
                .as_ref()
                .is_some_and(|old| old.key == entry.record.key);
            if duplicate {
                if previous
                    .as_ref()
                    .is_some_and(|old| old.json != entry.record.json)
                {
                    return Err(format!(
                        "export identity collision for {}",
                        entry.record.key
                    ));
                }
            } else {
                writeln!(writer, "{}", entry.record.json).map_err(|e| e.to_string())?;
                count += 1;
            }
            previous = Some(entry.record);
        } else {
            entry.record.write(&mut writer)?;
            count += 1;
        }
        if let Some(record) = next_record(&mut readers[entry.run])? {
            heap.push(HeapEntry {
                record,
                run: entry.run,
            });
        }
    }
    writer.flush().map_err(|e| e.to_string())?;
    writer.get_ref().sync_all().map_err(|e| e.to_string())?;
    Ok(count)
}

struct Sorter {
    directory: PathBuf,
    prefix: &'static str,
    pending: Vec<Record>,
    pending_bytes: usize,
    runs: Vec<PathBuf>,
    sequence: usize,
    chunk_bytes: usize,
}
impl Sorter {
    fn new(directory: &Path, prefix: &'static str) -> Self {
        Self {
            directory: directory.into(),
            prefix,
            pending: Vec::new(),
            pending_bytes: 0,
            runs: Vec::new(),
            sequence: 0,
            chunk_bytes: CHUNK_BYTES,
        }
    }
    fn path(&mut self) -> PathBuf {
        let path = self
            .directory
            .join(format!("{}-{:012}.run", self.prefix, self.sequence));
        self.sequence += 1;
        path
    }
    fn push(&mut self, key: &str, value: &Value) -> Result<(), String> {
        let record = Record::new(key, value)?;
        self.pending_bytes += record.key.len() + record.json.len() + 2;
        self.pending.push(record);
        if self.pending_bytes >= self.chunk_bytes {
            self.flush()?;
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), String> {
        if self.pending.is_empty() {
            return Ok(());
        }
        self.pending
            .sort_by(|a, b| a.key.cmp(&b.key).then_with(|| a.json.cmp(&b.json)));
        let path = self.path();
        let mut writer = BufWriter::new(new_file(&path)?);
        for record in &self.pending {
            record.write(&mut writer)?;
        }
        writer.flush().map_err(|e| e.to_string())?;
        writer.get_ref().sync_all().map_err(|e| e.to_string())?;
        self.runs.push(path);
        self.pending.clear();
        self.pending_bytes = 0;
        Ok(())
    }
    fn finish(mut self, destination: &Path) -> Result<u64, String> {
        self.flush()?;
        while self.runs.len() > MERGE_FAN_IN {
            let old = std::mem::take(&mut self.runs);
            for group in old.chunks(MERGE_FAN_IN) {
                let next = self.path();
                merge(group, &next, false)?;
                self.runs.push(next);
                for path in group {
                    fs::remove_file(path).map_err(|e| e.to_string())?;
                }
            }
        }
        merge(&self.runs, destination, true)
    }
}

struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn install(out: &Path, staged: &Path, relative: &str, role: &str) -> Result<Artifact, String> {
    let (sha256, bytes) = crate::store::hash_file(staged)?;
    let artifact = Artifact {
        role: role.into(),
        path: relative.into(),
        sha256,
        bytes,
        encoding: "jsonl".into(),
    };
    let destination = out.join(relative);
    match fs::hard_link(staged, &destination) {
        Ok(()) => super::sync_dir(destination.parent().expect("export file parent"))?,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            storage::verify_manifest(out, std::slice::from_ref(&artifact))?
        }
        Err(e) => return Err(format!("install {}: {e}", destination.display())),
    }
    Ok(artifact)
}

fn output_directory(root: &Path, requested: &Path) -> Result<PathBuf, String> {
    let absolute = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(requested)
    };
    if absolute
        .components()
        .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err("parent traversal in export destination".into());
    }
    let mut ancestor = absolute.as_path();
    loop {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => break,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                ancestor = ancestor
                    .parent()
                    .ok_or("export destination has no existing ancestor")?;
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    let parent = super::checked_directory(ancestor, false)?;
    let candidate = parent.join(absolute.strip_prefix(ancestor).map_err(|e| e.to_string())?);
    if root.starts_with(&candidate) || candidate.starts_with(root.join("cells")) {
        return Err("export destination overlaps source cell storage".into());
    }
    super::checked_directory(&candidate, true)
}

/// Export local canonical records into a projection safe from known-answer and
/// solver-outcome leakage. This does not download remote sources or train a model.
pub fn export(root: &Path, out: &Path) -> Result<Value, String> {
    export_with_graph_projector(root, out, prime_chain_graph::project)
}

type GraphProjector = fn(&Value, &Value, &Value) -> Result<Option<Value>, String>;

fn export_with_graph_projector(
    root: &Path,
    out: &Path,
    graph_projector: GraphProjector,
) -> Result<Value, String> {
    let root = super::checked_directory(root, false)?;
    let _source_lease = super::Lease::acquire(&root)?;
    let (plan, binding, plan_sha) = super::bound_run(&root)?;
    let planted = plan.planted();
    let out = output_directory(&root, out)?;
    let _out_lease = super::Lease::acquire(&out)?;
    let restricted = super::checked_directory(&out.join("restricted"), true)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&restricted, fs::Permissions::from_mode(0o700))
            .map_err(|e| e.to_string())?;
    }
    let scratch = Scratch(out.join(format!(
        ".export-staging-{}-{}",
        std::process::id(),
        super::unix_nanos()?
    )));
    fs::create_dir(&scratch.0).map_err(|e| e.to_string())?;
    let mut features = Sorter::new(&scratch.0, "feature");
    let mut targets = Sorter::new(&scratch.0, "target");
    let mut index = Sorter::new(&scratch.0, "index");
    let mut prefix_features = Sorter::new(&scratch.0, "prefix-feature");
    let mut prefix_records = Sorter::new(&scratch.0, "prefix-record");
    let mut algebraic_features = Sorter::new(&scratch.0, "algebraic-feature");
    let mut algebraic_records = Sorter::new(&scratch.0, "algebraic-record");
    let mut algebraic_occurrences = 0u64;
    let mut algebraic_verified_nonzero = 0u64;
    let mut algebraic_verified_zero = 0u64;
    let mut algebraic_unknown_suffixes = 0u64;
    let mut algebraic_degree_reductions = 0u64;
    let mut prefix_occurrences = 0u64;
    let mut feature_occurrences = 0u64;
    let mut sealed = 0u64;
    let mut unsealed = 0u64;
    let mut unavailable = 0u64;
    let mut missing_alternative_base = 0u64;
    let mut missing_large_prime_domain = 0u64;
    let mut missing_prime_chain_graph = 0u64;
    let mut oversized_prime_chain_graph = 0u64;
    let mut prime_chain_graph_occurrences = 0u64;
    let mut domain_counts = std::collections::BTreeMap::<String, u64>::new();
    let mut split_counts = std::collections::BTreeMap::<String, u64>::new();
    let shard_count = integer(&binding["shard_count"], "shard_count")?;
    let shard_index = integer(&binding["shard_index"], "shard_index")?;
    if shard_count == 0 || shard_index >= shard_count {
        return Err("invalid bound source shard".into());
    }
    let source_run_sha = crate::store::hash_file(&root.join("run.json"))?.0;
    let cells = root.join("cells");
    if cells.exists() {
        let cells = super::checked_directory(&cells, false)?;
        for entry in fs::read_dir(cells).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                return Err("unexpected source cell entry".into());
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| "non-UTF8 source cell")?;
            let cell_index: u64 = name.parse().map_err(|_| "nonnumeric source cell")?;
            if name != format!("{cell_index:012}") || cell_index % shard_count != shard_index {
                return Err("source cell directory does not belong to the bound shard".into());
            }
            let expected = plan.cell(cell_index)?;
            let dir = entry.path();
            let relative = format!("cells/{name}");
            if !dir.join("manifest.json").exists() {
                unsealed += 1;
                let id =
                    super::digest(format!("{source_run_sha}\n{relative}\nunsealed").as_bytes());
                index.push(&id,&json!({"schema":"ml-dataset.export-index/v1","record_id":id,"cell":relative,
                    "status":"unsealed_attempt_excluded","feature_id":null,"source_run_sha256":source_run_sha}))?;
                continue;
            }
            let manifest = super::verify_cell(&dir, true)?;
            if manifest.cell_index != cell_index
                || manifest.plan_sha256 != plan_sha
                || manifest.config != expected
                || super::read_json::<Value>(&dir.join("run.json"))? != binding
            {
                return Err("source cell is inconsistent with export run binding".into());
            }
            let manifest_sha = crate::store::hash_file(&dir.join("manifest.json"))?.0;
            let target_id =
                super::digest(format!("{source_run_sha}\n{manifest_sha}\n{cell_index}").as_bytes());
            let has = |path: &str| manifest.artifacts.iter().any(|a| a.path == path);
            let mut feature_id = None;
            let mut component = None;
            let mut split = None;
            let mut feature_status = "missing_verified_public_inputs";
            let mut pdp_domain = None;
            if has("public.json") && has("factor-base.json") && has("witness.json") {
                let public: Value = super::read_json(&dir.join("public.json"))?;
                let base: Value = super::read_json(&dir.join("factor-base.json"))?;
                let alternative = manifest
                    .artifacts
                    .iter()
                    .find(|a| a.path == "pkm-factor-base.json");
                if manifest.config.solver != "double-large-prime"
                    && manifest.artifacts.iter().any(|a| {
                        [LARGE_PRIME_DOMAIN_ROLE, LARGE_PRIME_TRANSCRIPT_ROLE]
                            .contains(&a.role.as_str())
                    })
                {
                    return Err("large-prime domain on a different solver source cell".into());
                }
                let inputs = if planted
                    && (matches!(
                        manifest.config.solver.as_str(),
                        "pkm" | "double-large-prime"
                    ) || (manifest.config.family == "prime"
                        && manifest.config.solver == "f4"
                        && (3..=15).contains(&manifest.config.m)))
                {
                    feature_status = "planted_specialized_domain_projection_not_admitted";
                    None
                } else if manifest.config.solver == "pkm" {
                    if let Some(artifact) = alternative {
                        if artifact.role != "explicit-alternative-kummer-factor-base"
                            || artifact.encoding != "json"
                        {
                            return Err("invalid PKM factor-base artifact role or encoding".into());
                        }
                        let pkm: Value = super::read_json(&dir.join(&artifact.path))?;
                        Some(pkm_feature_projection(&public, &base, &pkm)?)
                    } else {
                        missing_alternative_base += 1;
                        feature_status = "missing_verified_alternative_factor_base";
                        None
                    }
                } else {
                    if alternative.is_some()
                        || manifest.outcome["factor_base_variant"] == "prime-kummer"
                    {
                        return Err("alternative factor base on a non-PKM source cell".into());
                    }
                    if manifest.config.solver == "double-large-prime" {
                        let inputs =
                            large_prime_cell_projection(&dir, &manifest.artifacts, &public, &base)?;
                        if inputs.is_none() {
                            missing_large_prime_domain += 1;
                            feature_status = "missing_verified_large_prime_domain_or_transcript";
                        }
                        inputs
                    } else if manifest.config.family == "prime"
                        && manifest.config.solver == "f4"
                        && (3..=15).contains(&manifest.config.m)
                    {
                        if let Some(artifact) = manifest
                            .artifacts
                            .iter()
                            .find(|a| a.path == "prime-f4-input.json")
                        {
                            if artifact.role != "complete-prime-field-polynomial-system"
                                || artifact.encoding != "json"
                            {
                                return Err(
                                    "invalid prime-chain input artifact role or encoding".into()
                                );
                            }
                            let input: Value = super::read_json(&dir.join(&artifact.path))?;
                            match graph_projector(&public, &base, &input)? {
                                Some(graph) => {
                                    let mut projected = feature_projection(&public, &base)?;
                                    projected["prime_chain_graph"] = graph;
                                    prime_chain_graph_occurrences += 1;
                                    Some(projected)
                                }
                                None => {
                                    oversized_prime_chain_graph += 1;
                                    feature_status = "prime_chain_graph_size_limit";
                                    None
                                }
                            }
                        } else {
                            missing_prime_chain_graph += 1;
                            feature_status = "missing_verified_prime_chain_input";
                            None
                        }
                    } else {
                        Some(if planted {
                            feature_projection_v7(&public, &base)?
                        } else {
                            feature_projection(&public, &base)?
                        })
                    }
                };
                if let Some(inputs) = inputs {
                    verify_projected_pdp_domain(&dir, &manifest.artifacts, &inputs)?;
                    let domain = text(&inputs["pdp_domain"], "PDP domain")?;
                    let id = super::digest(bounded_json(&inputs, MAX_RECORD_BYTES)?.as_bytes());
                    let (group, partition) = split_for_size(integer(&inputs["n"], "n")?);
                    if inputs.get("prime_chain_graph").is_some() {
                        let event_artifact = manifest
                            .artifacts
                            .iter()
                            .find(|a| a.path == "prime-f4-chain-events.jsonl");
                        let replay = if let Some(artifact) = event_artifact {
                            if artifact.role
                                != "restricted-native-prime-f4-recursion-and-lift-events"
                                || artifact.encoding != "jsonl"
                            {
                                return Err(
                                    "invalid prime-chain event artifact role or encoding".into()
                                );
                            }
                            let input: Value = super::read_json(&dir.join("prime-f4-input.json"))?;
                            let reader = BufReader::new(
                                File::open(dir.join(&artifact.path)).map_err(|e| e.to_string())?,
                            );
                            let mut occurrence = 0u64;
                            let mut algebraic_record_occurrence = 0u64;
                            prime_chain_prefix::replay(
                                &input,
                                &inputs,
                                reader,
                                |feature, record| {
                                    if record["kind"] == "polynomial_consequence" {
                                        algebraic_record_occurrence += 1;
                                        let record_id = super::digest(
                                            format!("{target_id}\nalgebraic-record\n{algebraic_record_occurrence}").as_bytes(),
                                        );
                                        match record["admission_status"].as_str() {
                                            Some("verified_nonzero") => {
                                                algebraic_verified_nonzero += 1
                                            }
                                            Some("verified_zero") => algebraic_verified_zero += 1,
                                            Some("bounded_unknown_suffix") => {
                                                algebraic_unknown_suffixes += 1
                                            }
                                            _ => {
                                                return Err(
                                                    "unknown algebraic admission status".into()
                                                )
                                            }
                                        }
                                        if record["witnessed_degree_drop"] == true {
                                            algebraic_degree_reductions += 1;
                                        }
                                        let feature_id = if let Some(feature) = feature {
                                            if record["admission_status"] != "verified_nonzero" {
                                                return Err("unverified or zero algebraic row supplied public features".into());
                                            }
                                            let id = super::digest(
                                                bounded_json(&feature, MAX_RECORD_BYTES)?
                                                    .as_bytes(),
                                            );
                                            algebraic_features.push(&id, &json!({"schema":"ml-dataset.algebraic-features/v1",
                                                "feature_id":id,"split_component":group,"split":partition,"inputs":feature}))?;
                                            algebraic_occurrences += 1;
                                            Some(id)
                                        } else {
                                            None
                                        };
                                        return algebraic_records.push(&record_id, &json!({"schema":"ml-dataset.restricted-algebraic-record/v1",
                                            "record_id":record_id,"source_target_id":target_id,"source_event_artifact":artifact,
                                            "feature_id":feature_id,"split_component":group,"split":partition,"record":record}));
                                    }
                                    occurrence += 1;
                                    let record_id = super::digest(
                                        format!("{target_id}\nprefix-record\n{occurrence}")
                                            .as_bytes(),
                                    );
                                    let prefix_id = if let Some(feature) = feature {
                                        let prefix_id = super::digest(
                                            bounded_json(&feature, MAX_RECORD_BYTES)?.as_bytes(),
                                        );
                                        prefix_features.push(&prefix_id, &json!({"schema":"ml-dataset.prefix-features/v1",
                                        "feature_id":prefix_id,"split_component":group,"split":partition,"inputs":feature}))?;
                                        prefix_occurrences += 1;
                                        Some(prefix_id)
                                    } else {
                                        None
                                    };
                                    prefix_records.push(&record_id, &json!({"schema":"ml-dataset.restricted-prefix-record/v1",
                                    "record_id":record_id,"source_target_id":target_id,"source_event_artifact":artifact,
                                    "feature_id":prefix_id,"split_component":group,"split":partition,"record":record}))
                                },
                            )?
                        } else {
                            json!({"schema":"ml-dataset.prime-chain-prefix-replay/v1","admission_status":"no_retained_native_events",
                                "verified_branch_examples":0,"source_completion_claimed":false})
                        };
                        let record_id =
                            super::digest(format!("{target_id}\nprefix-replay-summary").as_bytes());
                        prefix_records.push(
                            &record_id,
                            &json!({"schema":"ml-dataset.restricted-prefix-record/v1",
                            "record_id":record_id,"source_target_id":target_id,"feature_id":null,
                            "split_component":group,"split":partition,"record":replay}),
                        )?;
                    }
                    let row = json!({"schema":"ml-dataset.features/v1","feature_id":id,"split_component":group,"split":partition,"inputs":inputs});
                    features.push(&id, &row)?;
                    feature_occurrences += 1;
                    *split_counts.entry(partition.into()).or_default() += 1;
                    *domain_counts.entry(domain.clone()).or_default() += 1;
                    feature_id = Some(id);
                    component = Some(group);
                    split = Some(partition);
                    pdp_domain = Some(domain);
                    feature_status = "available";
                }
            }
            if feature_id.is_none() {
                unavailable += 1;
            }
            let witness_ref = manifest.artifacts.iter().find(|a| a.path == "witness.json");
            let known_scalar = if let Some(artifact) = witness_ref {
                let witness: Value = super::read_json(&dir.join(&artifact.path))?;
                if feature_id.is_some() {
                    Some(decimal(&witness["scalar"], "known answer scalar")?)
                } else {
                    None
                }
            } else {
                None
            };
            let certificate_refs: Vec<_> = manifest
                .artifacts
                .iter()
                .filter(|a| {
                    [
                        "independently-verified-pdp",
                        "restricted-independent-scalar-label",
                    ]
                    .contains(&a.role.as_str())
                })
                .cloned()
                .collect();
            let mut independent_scalar = None;
            for certificate in &certificate_refs {
                if certificate.role == "restricted-independent-scalar-label" {
                    let label: Value = super::read_json(&dir.join(&certificate.path))?;
                    independent_scalar = Some(decimal(&label["scalar"], "independent scalar")?);
                }
            }
            targets.push(&target_id,&json!({"schema":"ml-dataset.restricted-targets/v1","target_id":target_id,
                "feature_id":feature_id,"pdp_domain":pdp_domain,"split_component":component,"split":split,"solver":manifest.config.solver,
                "labels":manifest.outcome,"known_answer_scalar":known_scalar,"independently_recovered_scalar":independent_scalar,
                "witness_artifact":witness_ref,"solution_certificates":certificate_refs,
                "feature_status":feature_status}))?;
            index.push(&target_id,&json!({"schema":"ml-dataset.export-index/v1","record_id":target_id,"target_id":target_id,
                "feature_id":feature_id,"cell":relative,"source_manifest_sha256":manifest_sha,
                "source_run_sha256":source_run_sha,"source_artifacts":manifest.artifacts,"status":"verified_sealed_cell"}))?;
            sealed += 1;
        }
    }
    let features_staged = scratch.0.join("features.jsonl");
    let targets_staged = scratch.0.join("targets.jsonl");
    let index_staged = scratch.0.join("index.jsonl");
    let unique_features = features.finish(&features_staged)?;
    let target_rows = targets.finish(&targets_staged)?;
    let index_rows = index.finish(&index_staged)?;
    let prefix_features_staged = scratch.0.join("prefix-features.jsonl");
    let prefix_records_staged = scratch.0.join("prefix-records.jsonl");
    let unique_prefix_features = prefix_features.finish(&prefix_features_staged)?;
    let prefix_record_rows = prefix_records.finish(&prefix_records_staged)?;
    let algebraic_features_staged = scratch.0.join("algebraic-features.jsonl");
    let algebraic_records_staged = scratch.0.join("algebraic-records.jsonl");
    let unique_algebraic_features = algebraic_features.finish(&algebraic_features_staged)?;
    let algebraic_record_rows = algebraic_records.finish(&algebraic_records_staged)?;
    if algebraic_occurrences != algebraic_verified_nonzero
        || algebraic_record_rows
            != algebraic_verified_nonzero + algebraic_verified_zero + algebraic_unknown_suffixes
    {
        return Err("export lost algebraic feature or record joins".into());
    }
    if target_rows != sealed || index_rows != sealed + unsealed {
        return Err("export lost or duplicated source identities".into());
    }
    let artifacts = vec![
        install(
            &out,
            &features_staged,
            "features.jsonl",
            "allowlisted-model-inputs",
        )?,
        install(
            &out,
            &targets_staged,
            "restricted/targets.jsonl",
            "restricted-supervised-labels",
        )?,
        install(
            &out,
            &index_staged,
            "restricted/index.jsonl",
            "restricted-source-provenance",
        )?,
        install(
            &out,
            &prefix_features_staged,
            "prefix-features.jsonl",
            "allowlisted-causal-model-inputs",
        )?,
        install(
            &out,
            &prefix_records_staged,
            "restricted/prefix-records.jsonl",
            "restricted-prefix-labels-certificates-and-provenance",
        )?,
        install(
            &out,
            &algebraic_features_staged,
            "algebraic-features.jsonl",
            "allowlisted-polynomial-consequence-inputs",
        )?,
        install(
            &out,
            &algebraic_records_staged,
            "restricted/algebraic-records.jsonl",
            "restricted-checked-polynomial-consequences-certificates-and-provenance",
        )?,
    ];
    // Keep each object expansion bounded without changing manifest keys or order.
    let mut manifest = json!({"schema":"ml-dataset.training-export/v1","source_run_sha256":source_run_sha,"plan_sha256":plan_sha,
        "source_commit":binding["source_commit"],"binary_sha256":binding["binary_sha256"],"artifacts":artifacts,
        "sealed_source_cells":sealed,"unsealed_attempts_excluded":unsealed,"cells_without_verified_features":unavailable,
        "cells_without_verified_alternative_factor_base":missing_alternative_base,"feature_occurrences_by_pdp_domain":domain_counts,
        "cells_without_verified_large_prime_domain":missing_large_prime_domain,
        "cells_without_verified_prime_chain_graph":missing_prime_chain_graph,
        "cells_excluded_by_prime_chain_graph_size_limit":oversized_prime_chain_graph,
        "prime_chain_graph_occurrences":prime_chain_graph_occurrences,
        "prefix_feature_occurrences":prefix_occurrences,"unique_prefix_feature_rows":unique_prefix_features,
        "restricted_prefix_record_rows":prefix_record_rows});
    let manifest_tail = json!({
        "algebraic_feature_occurrences":algebraic_occurrences,"unique_algebraic_feature_rows":unique_algebraic_features,
        "restricted_algebraic_record_rows":algebraic_record_rows,
        "algebraic_outcomes":{"verified_nonzero_target_rows":algebraic_verified_nonzero,"verified_zero_target_rows":algebraic_verified_zero,
            "witnessed_degree_reduction_rows":algebraic_degree_reductions,"bounded_unknown_suffix_records":algebraic_unknown_suffixes,
            "count_scope":"non-deduplicated target occurrences and one terminal unknown record per capped consequence channel; not exact degree invariants or solved instances"},
        "feature_occurrences":feature_occurrences,"unique_feature_rows":unique_features,"restricted_target_rows":target_rows,
        "feature_identity":"SHA256 of canonical strictly projected public inputs, optional exact equation graph and mathematical encoding, actual small/residual factor bases, column actions and PDP/relation-query domain; solver and source provenance excluded",
        "split_policy":{"domain":SPLIT_DOMAIN,"component":"field-size-nN","assignment":"first 64 hash bits modulo 100: train 0..79, validation 80..89, test 90..99",
            "occurrences_by_partition":split_counts,"all_same_size_families_twists_generators_keys_stay_together":true,
            "cross_size_algebraic_links_merged":false,"row_random_split_permitted":false},
        "feature_policy":{"version":if planted {"strict-structural-allowlist-v7-planted"} else {"strict-structural-allowlist-v6"},
            "cross_population_pooling_permitted":!planted,
            "only_features_jsonl_is_model_input":false,
            "model_input_files":["features.jsonl","prefix-features.jsonl","algebraic-features.jsonl"],
            "algebraic_task":"prime-polynomial-consequence/v1; pre-native equations only; independently checked identities for retained native basis rows, with separately searched witnesses",
            "algebraic_claim_limits":["recursive prime m3..15 and p<2^32 only","admission requires a previously certified equality node","separate cumulative consequence-channel caps preserve existing branch budget","witness-dependent cancellation is not exact first/last fall degree","no native-operation or native-instruction witness"],
            "prefix_task":"prime-chain-next-branch-attempt/v1; replayed recursive state and independently derived iterator; observed imitation labels only, no optimality or counterfactual cost",
            "prefix_claim_limits":["bounded ideal-certificate search can exclude a suffix","no complete native execution or instruction proof","no exact first/last fall degree","no complete Groebner basis claim"],
            "prime_chain_graph":"prime-chain-graph/v1 static equations/domains and signed-point incidence, reconstructed before publication; no solver-prefix/action features; missing or oversized graphs excluded with restricted provenance",
            "prime_chain_graph_capabilities":prime_chain_graph::capabilities(),
            "pkm_domain":"verified exact alternative Kummer base; missing base excluded from matched routing features",
            "large_prime_domain":"exact saved small/residual domains bound to independently replayed transcript; signed Frobenius and identity/sign routes have distinct identities; missing domain or transcript excluded with provenance retained",
            "cross_domain_routing_labels_matched":false,
            "excluded":["private witness","sampling seeds","curve/generator/key indices","configuration","order certificates","construction candidates","construction trace","solver outcomes","solver event streams","labels","source paths"]},
        "sort":{"implementation":"bounded Rust external merge","chunk_bytes":CHUNK_BYTES,"merge_fan_in":MERGE_FAN_IN,"max_row_bytes":MAX_RECORD_BYTES,
            "source_cell_values_materialized":true,"total_process_rss_bound_claimed":false},
        "source_artifacts_modified":false,"remote_download_performed":false});
    let Value::Object(tail_fields) = manifest_tail else {
        unreachable!("manifest tail is an object literal");
    };
    manifest
        .as_object_mut()
        .expect("manifest is an object literal")
        .extend(tail_fields);
    let manifest_artifact =
        storage::write_json(&out, "manifest.json", "training-export-manifest", &manifest)?;
    storage::verify_manifest(&out, &artifacts)?;
    Ok(json!({"manifest":manifest_artifact,"export":manifest}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{curves, CellManifest, DatasetPlan};

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "crypto-export-test-{}-{}",
                std::process::id(),
                crate::ml_dataset::unix_nanos().unwrap()
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
        fn source(&self) -> PathBuf {
            self.source_with_backends(false, false)
        }
        fn source_with_backends(&self, pkm: bool, include_pkm_base: bool) -> PathBuf {
            self.source_with_method("prime", if pkm { "pkm" } else { "bsgs" }, include_pkm_base)
        }
        fn source_with_method(&self, family: &str, method: &str, run_solver: bool) -> PathBuf {
            self.source_with_parameters(family, method, run_solver, 11, 2, 8)
        }
        fn source_with_parameters(
            &self,
            family: &str,
            method: &str,
            run_solver: bool,
            n: u32,
            m: usize,
            factor_base_size: usize,
        ) -> PathBuf {
            let source = self.0.join("source");
            fs::create_dir(&source).unwrap();
            let mut protocol = json!({"schema":"experiment.protocol/v1","purpose":"native export correctness fixture"});
            let freeze_hash = crate::ml_dataset::digest(
                &crate::ml_dataset::protocol_canonical_bytes(&protocol).unwrap(),
            );
            protocol["freeze"] = json!({"status":"frozen","protocol_sha256":freeze_hash,
                "filed_before_affected_data":true,"frozen_at":"2026-10-28T00:00:00Z"});
            let protocol = serde_json::to_vec(&protocol).unwrap();
            let plan = DatasetPlan {
                schema: "crypto.ml-dataset.plan/v1".into(),
                field_sizes: vec![n],
                arities: vec![m],
                families: vec![family.into()],
                solvers: vec!["enumerate".into(), method.into()],
                curve_seeds: 8,
                generators: 2,
                keys: 2,
                seed: 42,
                factor_base_size,
                degree_bound: if method == "f4" && m >= 3 { 1 } else { 4 },
                work_limit: if method == "f4" && m >= 3 { 1 } else { 1000 },
                twists: true,
                protocol_sha256: Some(crate::ml_dataset::digest(&protocol)),
                split_rule: "curve-twist-component-sha256-v1".into(),
            };
            let plan_sha = crate::ml_dataset::digest(&serde_json::to_vec(&plan).unwrap());
            let binding = json!({"schema":"crypto.ml-dataset.run/v1","plan_sha256":plan_sha,
                "source_commit":"a".repeat(40),"binary_sha256":"b".repeat(64),"shard_index":0,"shard_count":1});
            storage::write_json(&source, "plan.json", "plan", &plan).unwrap();
            storage::write_json(&source, "run.json", "run", &binding).unwrap();
            storage::write_bytes(&source, "protocol.json", "protocol", "json", &protocol).unwrap();
            let first_case = curves::generate(&plan.cell(124).unwrap()).unwrap();
            let cell_indices: &[u64] = if method == "f4" && m >= 3 {
                &[124, 125, 126, 127]
            } else {
                &[124, 125]
            };
            for &cell_index in cell_indices {
                let dir = source.join("cells").join(format!("{cell_index:012}"));
                fs::create_dir_all(&dir).unwrap();
                let config = plan.cell(cell_index).unwrap();
                let case = if config.twist {
                    curves::generate(&config).unwrap()
                } else {
                    first_case.clone()
                };
                let mut artifacts = vec![
                    storage::write_json(&dir, "plan.json", "plan", &plan).unwrap(),
                    storage::write_json(&dir, "run.json", "run", &binding).unwrap(),
                    storage::write_bytes(&dir, "protocol.json", "protocol", "json", &protocol)
                        .unwrap(),
                    storage::write_json(&dir, "config.json", "restricted-config", &config).unwrap(),
                    storage::write_json(
                        &dir,
                        "public.json",
                        "public-instance",
                        &case.public_instance,
                    )
                    .unwrap(),
                    storage::write_json(&dir, "factor-base.json", "factor-base", &case.factor_base)
                        .unwrap(),
                    storage::write_json(
                        &dir,
                        "witness.json",
                        "restricted-known-answer",
                        &case.private_witness,
                    )
                    .unwrap(),
                ];
                for path in ["attempt.json", "worker-process.json"] {
                    artifacts
                        .push(storage::write_json(&dir, path, "provenance", &json!({})).unwrap());
                }
                for path in ["stdout.log", "stderr.log"] {
                    artifacts.push(storage::write_bytes(&dir, path, "log", "utf8", b"").unwrap());
                }
                let mut outcome = json!({"status":"not_attempted","known_answer_verified":true,"factor_base_verified":true,
                    "solver_completed":false,"scalar_recovered_without_witness":false,"scalar_replay_verified":false});
                if config.solver == method && run_solver {
                    let mut solver_case = case.clone();
                    solver_case.private_witness = Value::Null;
                    let result = crate::ml_dataset::solvers::solve(&config, &solver_case).unwrap();
                    let required_role = if method == "pkm" {
                        "explicit-alternative-kummer-factor-base"
                    } else if method == "f4" {
                        "complete-prime-field-polynomial-system"
                    } else {
                        LARGE_PRIME_DOMAIN_ROLE
                    };
                    assert!(
                        result.artifacts.iter().any(|a| a.role == required_role),
                        "{}",
                        result.outcome
                    );
                    for payload in result.artifacts {
                        artifacts.push(
                            storage::write_bytes(
                                &dir,
                                &payload.path,
                                &payload.role,
                                &payload.encoding,
                                &payload.bytes,
                            )
                            .unwrap(),
                        );
                    }
                    outcome = result.outcome;
                    outcome["known_answer_verified"] = json!(true);
                    outcome["factor_base_verified"] = json!(true);
                }
                storage::write_json(
                    &dir,
                    "manifest.json",
                    "manifest",
                    &CellManifest {
                        schema: "crypto.ml-dataset.cell/v1".into(),
                        cell_index,
                        plan_sha256: plan_sha.clone(),
                        config,
                        outcome,
                        artifacts,
                    },
                )
                .unwrap();
            }
            source
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn projection_ignores_unknown_secret_and_future_state_at_every_nested_level() {
        let marker = "FORBIDDEN_FUTURE_OR_PRIVATE_VALUE";
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":11,"m":2,"twist":false,
            "field":{"characteristic":"2017","modulus":"2017","degree":1,"basis":"canonical-residue","secret":marker},
            "curve":{"model":"short-weierstrass","a":"1","b":"1","group_order":"2011","subgroup_order":"2011","cofactor":"1",
                "order_certificate":{"private":marker},"rejected_curve_samples":marker},
            "generator":{"x":"1","y":"2","scalar":marker},"target":{"x":"3","y":"4","key_index":marker},
            "symmetry":{"coefficient_field_degree":1,"frobenius_power":1,"frobenius_nontrivial_on_model":false,"negation":true,"future":marker},
            "seed":marker,"private_witness":marker,"known_answer_verified":marker,"construction_trace":marker,"sampling":marker});
        let base = json!({"schema":"ml-ecdlp-factor-base/v1","rule":"small-x-cofactor-projection",
            "points":[{"x":"1","y":"2","scalar":marker}],"negation_action":[null],"frobenius_action":[0],"frobenius_power":1,
            "orbits":[{"points":[{"x":"1","y":"2","seed":marker}],"closed":true,"stabilizer_size":1,"future":marker}],
            "construction_candidates":[{"private":marker}],"solver_outcome":marker,"generator_index":marker});
        let projected = feature_projection(&public, &base).unwrap();
        let encoded = serde_json::to_string(&projected).unwrap();
        assert!(!encoded.contains(marker));
        assert!(!encoded.contains("scalar"));
        assert!(!encoded.contains("seed"));
        assert!(!encoded.contains("construction"));
        assert_eq!(projected["target"], json!({"x":"3","y":"4"}));
        let mut poisoned = public;
        poisoned["field"]["basis"] = json!({"private":marker});
        assert!(feature_projection(&poisoned, &base).is_err());
    }

    #[test]
    fn planted_v7_projection_hides_population_and_construction_provenance() {
        let public = json!({"schema":"ml-ecdlp-instance/v1","family":"prime","n":11,"m":3,"twist":false,
            "field":{"characteristic":"2017","modulus":"2017","degree":1,"basis":"canonical-residue"},
            "curve":{"model":"short-weierstrass","a":"1","b":"1","group_order":"2011","subgroup_order":"2011","cofactor":"1"},
            "generator":{"x":"1","y":"2"},"target":{"x":"3","y":"4"},
            "symmetry":{"coefficient_field_degree":1,"frobenius_power":1,"frobenius_nontrivial_on_model":false,"negation":true},
            "population":"planted-pdp-v1","planted_construction":{"scalar":"123","selected_indices":[0,0,0]}});
        let base = json!({"schema":"ml-ecdlp-factor-base/v1","rule":"planted-known-log-subgroup/v1",
            "points":[{"x":"5","y":"6"}],"negation_action":[null],"frobenius_action":[0],
            "frobenius_power":1,"orbits":[{"points":[{"x":"5","y":"6"}],"closed":true,"stabilizer_size":1}],
            "base_logs":["123"],"construction_attempts":17});
        assert!(feature_projection(&public, &base).is_err());
        let planted = feature_projection_v7(&public, &base).unwrap();
        let mut natural_public = public.clone();
        natural_public["population"] = json!("unconditioned-uniform-nonzero-subgroup-scalar");
        natural_public
            .as_object_mut()
            .unwrap()
            .remove("planted_construction");
        let mut natural_base = base.clone();
        natural_base["rule"] = json!("small-x-prime-primary-projection-with-cyclic-membership");
        natural_base.as_object_mut().unwrap().remove("base_logs");
        natural_base
            .as_object_mut()
            .unwrap()
            .remove("construction_attempts");
        assert_eq!(
            planted,
            feature_projection_v7(&natural_public, &natural_base).unwrap()
        );
        let text = serde_json::to_string(&planted).unwrap();
        for forbidden in [
            "planted",
            "population",
            "scalar",
            "selected_indices",
            "base_logs",
            "construction_attempts",
        ] {
            assert!(!text.contains(forbidden), "leaked {forbidden}");
        }
    }

    #[test]
    fn malformed_scratch_lines_and_serialization_stop_at_their_limits() {
        let mut reader = BufReader::with_capacity(16, io::Cursor::new(vec![b'x'; 4096]));
        let error = bounded_line(&mut reader, 32).unwrap_err();
        assert!(error.contains("bounded line limit"));
        // At most one fixed reader buffer can be prefetched beyond the cap;
        // the rest of the malformed, unterminated row is never read.
        assert!(reader.get_ref().position() <= 48);

        let mut reader = BufReader::with_capacity(3, io::Cursor::new(b"abc\nnext\n"));
        assert_eq!(bounded_line(&mut reader, 4).unwrap(), b"abc\n");
        assert_eq!(bounded_line(&mut reader, 5).unwrap(), b"next\n");
        assert!(bounded_line(&mut reader, 5).unwrap().is_empty());

        let value = json!({"escaped":"\n\n\n\n\n\n\n\n"});
        assert!(bounded_json(&value, 16)
            .unwrap_err()
            .contains("exceeds 16-byte limit"));
        let expected = serde_json::to_string(&value).unwrap();
        assert_eq!(bounded_json(&value, expected.len()).unwrap(), expected);
        let mut writer = CappedWriter {
            bytes: Vec::new(),
            limit: 8,
        };
        writer.write_all(b"1234").unwrap();
        assert!(writer.write_all(b"12345").is_err());
        assert_eq!(writer.bytes, b"1234");
    }

    #[test]
    fn external_merge_is_deterministic_deduplicated_and_checks_collisions() {
        let f = Fixture::new();
        let mut sorter = Sorter::new(&f.0, "rows");
        sorter.chunk_bytes = 80;
        for i in (0..40).rev() {
            let key = crate::ml_dataset::digest(format!("row-{i}").as_bytes());
            for _ in 0..2 {
                sorter
                    .push(&key, &json!({"feature_id":key,"value":i}))
                    .unwrap();
            }
        }
        let output = f.0.join("rows.jsonl");
        assert_eq!(sorter.finish(&output).unwrap(), 40);
        let rows: Vec<Value> = BufReader::new(File::open(output).unwrap())
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
            .collect();
        assert_eq!(rows.len(), 40);
        assert!(rows
            .windows(2)
            .all(|pair| pair[0]["feature_id"].as_str() < pair[1]["feature_id"].as_str()));
        let key = "a".repeat(64);
        let mut collision = Sorter::new(&f.0, "collision");
        collision.push(&key, &json!({"value":1})).unwrap();
        collision.push(&key, &json!({"value":2})).unwrap();
        assert!(collision
            .finish(&f.0.join("collision.jsonl"))
            .unwrap_err()
            .contains("identity collision"));
    }

    #[test]
    fn source_export_separates_labels_deduplicates_backends_and_retries_immutably() {
        let f = Fixture::new();
        let source = f.source();
        let out = f.0.join("training");
        let first = export(&source, &out).unwrap();
        assert_eq!(first["export"]["sealed_source_cells"], 2);
        assert_eq!(first["export"]["feature_occurrences"], 2);
        assert_eq!(first["export"]["unique_feature_rows"], 1);
        assert_eq!(first["export"]["restricted_target_rows"], 2);
        let features = fs::read_to_string(out.join("features.jsonl")).unwrap();
        for forbidden in [
            "private_witness",
            "scalar",
            "key_index",
            "generator_index",
            "curve_seed",
            "source_commit",
            "solver_completed",
        ] {
            assert!(!features.contains(forbidden), "{forbidden}");
        }
        let targets = fs::read_to_string(out.join("restricted/targets.jsonl")).unwrap();
        assert!(targets.contains("known_answer_scalar"));
        let second = export(&source, &out).unwrap();
        assert_eq!(first["manifest"], second["manifest"]);
        fs::write(source.join("cells/000000000124/witness.json"), b"corrupt").unwrap();
        assert!(export(&source, &f.0.join("bad-export")).is_err());
        assert!(!f.0.join("bad-export/manifest.json").exists());
    }

    #[test]
    fn legacy_prime_f4_m2_export_keeps_the_shared_generic_identity() {
        let f = Fixture::new();
        let source = f.source_with_parameters("prime", "f4", true, 7, 2, 2);
        let out = f.0.join("training");
        let result = export(&source, &out).unwrap();
        assert_eq!(result["export"]["prime_chain_graph_occurrences"], 0);
        assert_eq!(result["export"]["unique_feature_rows"], 1);
        assert_eq!(result["export"]["feature_occurrences"], 2);
        let row: Value = serde_json::from_str(
            fs::read_to_string(out.join("features.jsonl"))
                .unwrap()
                .trim(),
        )
        .unwrap();
        assert!(row["inputs"].get("prime_chain_graph").is_none());
    }

    #[test]
    fn graph_resource_refusal_retains_labels_without_generic_feature_fallback() {
        let f = Fixture::new();
        let source = f.source_with_parameters("prime", "f4", true, 7, 3, 2);
        let out = f.0.join("training");
        // Inject only a projector resource refusal after real sealed-cell replay.
        // This checks exporter behavior independently of the codec's cap tests.
        let result = export_with_graph_projector(&source, &out, |_, _, _| Ok(None)).unwrap();
        assert_eq!(
            result["export"]["cells_excluded_by_prime_chain_graph_size_limit"],
            2
        );
        assert_eq!(result["export"]["prime_chain_graph_occurrences"], 0);
        assert_eq!(result["export"]["unique_feature_rows"], 2);
        let rows: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        for row in rows.iter().filter(|r| r["solver"] == "f4") {
            assert!(row["feature_id"].is_null());
            assert_eq!(row["feature_status"], "prime_chain_graph_size_limit");
            assert!(row["known_answer_scalar"].is_null());
            assert!(row["witness_artifact"].is_object());
        }
    }

    #[test]
    fn prime_chain_export_preserves_both_twists_and_unsupported_wide_equations() {
        for n in [7, 61] {
            let f = Fixture::new();
            let source = f.source_with_parameters("prime", "f4", true, n, 3, 2);
            let out = f.0.join("training");
            let result = export(&source, &out).unwrap();
            assert_eq!(result["export"]["prime_chain_graph_occurrences"], 2);
            assert_eq!(result["export"]["unique_feature_rows"], 4);
            assert_eq!(result["export"]["restricted_target_rows"], 4);
            assert_eq!(
                result["export"]["feature_policy"]["version"],
                "strict-structural-allowlist-v6"
            );
            let bytes = fs::read_to_string(out.join("features.jsonl")).unwrap();
            let rows: Vec<Value> = bytes
                .lines()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            let graphs: Vec<_> = rows
                .iter()
                .filter(|r| r["inputs"].get("prime_chain_graph").is_some())
                .collect();
            assert_eq!(graphs.len(), 2);
            assert!(graphs.iter().any(|r| r["inputs"]["twist"] == false));
            assert!(graphs.iter().any(|r| r["inputs"]["twist"] == true));
            assert!(rows
                .iter()
                .all(|r| r["split_component"] == format!("field-size-n{n}")));
            for row in graphs {
                assert_eq!(
                    row["inputs"]["prime_chain_graph"]["schema"],
                    "prime-chain-graph/v1"
                );
                let expected = super::super::digest(
                    bounded_json(&row["inputs"], MAX_RECORD_BYTES)
                        .unwrap()
                        .as_bytes(),
                );
                assert_eq!(row["feature_id"], expected);
            }
            for forbidden in [
                "known_answer_scalar",
                "native_backend_invoked",
                "solver_completed",
                "source_commit",
                "curve_seed",
                "private_witness",
                "elapsed_ms",
            ] {
                assert!(!bytes.contains(forbidden), "leaked {forbidden}");
            }
            if n == 61 {
                assert!(bytes.contains("\"2305843009213693951\""));
                let labels: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
                    .unwrap()
                    .lines()
                    .map(|s| serde_json::from_str(s).unwrap())
                    .collect();
                for row in labels.iter().filter(|r| r["solver"] == "f4") {
                    assert_eq!(row["labels"]["native_backend_invoked"], false);
                    assert_eq!(row["labels"]["status"], "unsupported");
                    assert!(row["feature_id"].is_string());
                }
            }
            assert_eq!(
                result["manifest"],
                export(&source, &out).unwrap()["manifest"]
            );
        }
    }

    fn reseal_export_cell(dir: &Path, manifest: &mut CellManifest) {
        for artifact in &mut manifest.artifacts {
            (artifact.sha256, artifact.bytes) =
                crate::store::hash_file(&dir.join(&artifact.path)).unwrap();
        }
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_vec(manifest).unwrap(),
        )
        .unwrap();
        storage::verify_manifest(dir, &manifest.artifacts).unwrap();
    }

    #[test]
    fn prime_chain_export_ignores_private_metadata_and_rejects_rehashed_algebra() {
        let f = Fixture::new();
        let source = f.source_with_parameters("prime", "f4", true, 7, 3, 2);
        let clean_out = f.0.join("clean");
        export(&source, &clean_out).unwrap();
        let clean_features = fs::read(clean_out.join("features.jsonl")).unwrap();
        let dir = source.join("cells/000000000125");
        let mut manifest: CellManifest =
            super::super::read_json(&dir.join("manifest.json")).unwrap();
        let marker = "PRIVATE_FUTURE_GRAPH_EXPORT_SENTINEL";
        for name in [
            "public.json",
            "factor-base.json",
            "prime-f4-input.json",
            "witness.json",
        ] {
            let path = dir.join(name);
            let mut value: Value = super::super::read_json(&path).unwrap();
            value["unknown_private_metadata"] =
                json!({"seed":marker,"scalar":marker,"future_outcome":marker});
            fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
        }
        manifest.outcome["unknown_future_cost"] = json!(marker);
        reseal_export_cell(&dir, &mut manifest);
        let injected_out = f.0.join("injected");
        export(&source, &injected_out).unwrap();
        assert_eq!(
            clean_features,
            fs::read(injected_out.join("features.jsonl")).unwrap()
        );
        assert!(
            fs::read_to_string(injected_out.join("restricted/targets.jsonl"))
                .unwrap()
                .contains(marker)
        );

        let input_path = dir.join("prime-f4-input.json");
        let original: Value = super::super::read_json(&input_path).unwrap();
        for mutation in 0..4 {
            let mut input = original.clone();
            let mut bad_manifest: CellManifest =
                serde_json::from_value(serde_json::to_value(&manifest).unwrap()).unwrap();
            match mutation {
                0 => input["polynomials"][0][0]["coefficient"] = json!("0"),
                1 => input["domains"][3] = input["domains"][0].clone(),
                2 => input["schema"] = json!("legacy-filename-is-not-admission"),
                _ => {
                    bad_manifest
                        .artifacts
                        .iter_mut()
                        .find(|a| a.path == "prime-f4-input.json")
                        .unwrap()
                        .role = "unverified-input".into()
                }
            }
            fs::write(&input_path, serde_json::to_vec(&input).unwrap()).unwrap();
            reseal_export_cell(&dir, &mut bad_manifest);
            let rejected = f.0.join(format!("rejected-{mutation}"));
            assert!(export(&source, &rejected).is_err(), "mutation {mutation}");
            assert!(!rejected.join("manifest.json").exists());
        }
    }

    #[test]
    fn missing_prime_chain_input_retains_restricted_exclusion_without_generic_fallback() {
        let f = Fixture::new();
        let source = f.source_with_parameters("prime", "f4", false, 7, 3, 2);
        let out = f.0.join("training");
        let result = export(&source, &out).unwrap();
        assert_eq!(
            result["export"]["cells_without_verified_prime_chain_graph"],
            2
        );
        assert_eq!(result["export"]["prime_chain_graph_occurrences"], 0);
        assert_eq!(result["export"]["unique_feature_rows"], 2);
        let rows: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        for row in rows.iter().filter(|r| r["solver"] == "f4") {
            assert!(row["feature_id"].is_null());
            assert_eq!(row["feature_status"], "missing_verified_prime_chain_input");
        }
    }

    #[test]
    fn pkm_uses_its_verified_actual_base_and_distinct_routing_identity() {
        let f = Fixture::new();
        let source = f.source_with_backends(true, true);
        let out = f.0.join("training");
        let exported = export(&source, &out).unwrap();
        assert_eq!(exported["export"]["unique_feature_rows"], 2);
        assert_eq!(
            exported["export"]["feature_occurrences_by_pdp_domain"][STANDARD_PDP_DOMAIN],
            1
        );
        assert_eq!(
            exported["export"]["feature_occurrences_by_pdp_domain"][KUMMER_PDP_DOMAIN],
            1
        );
        let features: Vec<Value> = fs::read_to_string(out.join("features.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let standard = features
            .iter()
            .find(|v| v["inputs"]["pdp_domain"] == STANDARD_PDP_DOMAIN)
            .unwrap();
        let kummer = features
            .iter()
            .find(|v| v["inputs"]["pdp_domain"] == KUMMER_PDP_DOMAIN)
            .unwrap();
        assert_ne!(standard["feature_id"], kummer["feature_id"]);
        assert_eq!(standard["inputs"]["target"], kummer["inputs"]["target"]);
        assert_eq!(standard["split_component"], kummer["split_component"]);
        let dir = source.join("cells/000000000125");
        let public: Value = crate::ml_dataset::read_json(&dir.join("public.json")).unwrap();
        let base: Value = crate::ml_dataset::read_json(&dir.join("factor-base.json")).unwrap();
        let mut pkm: Value =
            crate::ml_dataset::read_json(&dir.join("pkm-factor-base.json")).unwrap();
        assert_eq!(
            kummer["inputs"]["factor_base"]["points"],
            pkm["definition"]["points"]
        );
        assert_eq!(kummer["inputs"]["factor_base"]["t"], pkm["definition"]["t"]);
        assert_eq!(kummer["inputs"]["factor_base"]["g"], pkm["definition"]["g"]);

        let marker = "FORBIDDEN_PKM_LOG_OR_FUTURE_VALUE";
        pkm["construction_cost"] = json!({"secret":marker});
        pkm["seed"] = json!(marker);
        pkm["private_witness"] = json!(marker);
        pkm["definition"]["hidden_logs"] = json!(marker);
        pkm["definition"]["variant"] = json!(marker);
        for row in pkm["definition"]["points"].as_array_mut().unwrap() {
            row["scalar"] = json!(marker);
        }
        let projected = pkm_feature_projection(&public, &base, &pkm).unwrap();
        assert_eq!(projected, kummer["inputs"]);
        let encoded = serde_json::to_string(&projected).unwrap();
        for forbidden in [
            marker,
            "scalar",
            "seed",
            "construction",
            "hidden_logs",
            "private_witness",
        ] {
            assert!(!encoded.contains(forbidden), "{forbidden}");
        }
        pkm["definition"]["g"] = json!("0");
        assert!(pkm_feature_projection(&public, &base, &pkm).is_err());

        // Certificate definitions must match the saved feature domain even
        // when the certificate is stored with its own consistent digest.
        let certificate_dir = f.0.join("certificate");
        fs::create_dir(&certificate_dir).unwrap();
        let mut other_definition =
            crate::ml_dataset::read_json::<Value>(&dir.join("pkm-factor-base.json")).unwrap()
                ["definition"]
                .clone();
        other_definition["g"] = json!("0");
        let certificate = storage::write_json(
            &certificate_dir,
            "pdp.json",
            "independently-verified-pdp",
            &json!({"factor_base_definition":other_definition}),
        )
        .unwrap();
        assert!(
            verify_projected_pdp_domain(&certificate_dir, &[certificate], &kummer["inputs"])
                .unwrap_err()
                .contains("differs from exported Kummer")
        );
    }

    #[test]
    fn pkm_without_actual_base_retains_provenance_but_has_no_matched_feature() {
        let f = Fixture::new();
        let source = f.source_with_backends(true, false);
        let out = f.0.join("training");
        let exported = export(&source, &out).unwrap();
        assert_eq!(exported["export"]["sealed_source_cells"], 2);
        assert_eq!(exported["export"]["unique_feature_rows"], 1);
        assert_eq!(exported["export"]["restricted_target_rows"], 2);
        assert_eq!(
            exported["export"]["cells_without_verified_alternative_factor_base"],
            1
        );
        let targets: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let pkm = targets.iter().find(|row| row["solver"] == "pkm").unwrap();
        assert!(pkm["feature_id"].is_null());
        assert!(pkm["pdp_domain"].is_null());
        assert_eq!(
            pkm["feature_status"],
            "missing_verified_alternative_factor_base"
        );
        let index = fs::read_to_string(out.join("restricted/index.jsonl")).unwrap();
        assert!(index.contains("cells/000000000125"));
        assert!(index.contains("witness.json"));
    }

    #[test]
    fn large_prime_features_bind_actual_domains_without_construction_or_label_leakage() {
        for (family, expected_domain) in [
            ("koblitz", FROBENIUS_LARGE_PRIME_DOMAIN),
            ("prime", GENERIC_LARGE_PRIME_DOMAIN),
        ] {
            let f = Fixture::new();
            let source = f.source_with_method(family, "double-large-prime", true);
            let out = f.0.join("training");
            let exported = export(&source, &out).unwrap();
            assert_eq!(exported["export"]["unique_feature_rows"], 2);
            assert_eq!(
                exported["export"]["feature_occurrences_by_pdp_domain"][expected_domain],
                1
            );
            let features: Vec<Value> = fs::read_to_string(out.join("features.jsonl"))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            let ordinary = features
                .iter()
                .find(|v| v["inputs"]["pdp_domain"] == STANDARD_PDP_DOMAIN)
                .unwrap();
            let relation = features
                .iter()
                .find(|v| v["inputs"]["pdp_domain"] == expected_domain)
                .unwrap();
            assert_ne!(ordinary["feature_id"], relation["feature_id"]);
            assert_eq!(ordinary["inputs"]["target"], relation["inputs"]["target"]);
            assert_eq!(ordinary["split_component"], relation["split_component"]);
            assert_eq!(relation["inputs"]["relation_query"]["small_summands"], 0);
            assert_eq!(relation["inputs"]["relation_query"]["residual_summands"], 2);
            let targets: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            let label = targets
                .iter()
                .find(|v| v["solver"] == "double-large-prime")
                .unwrap();
            assert_eq!(label["feature_id"], relation["feature_id"]);
            assert_eq!(label["pdp_domain"], expected_domain);

            let dir = source.join("cells/000000000125");
            let manifest: CellManifest =
                crate::ml_dataset::read_json(&dir.join("manifest.json")).unwrap();
            let public: Value = crate::ml_dataset::read_json(&dir.join("public.json")).unwrap();
            let base: Value = crate::ml_dataset::read_json(&dir.join("factor-base.json")).unwrap();
            let domain_artifact = manifest
                .artifacts
                .iter()
                .find(|a| a.role == LARGE_PRIME_DOMAIN_ROLE)
                .unwrap();
            let transcript_artifact = manifest
                .artifacts
                .iter()
                .find(|a| a.role == LARGE_PRIME_TRANSCRIPT_ROLE)
                .unwrap();
            let mut domain: Value =
                crate::ml_dataset::read_json(&dir.join(&domain_artifact.path)).unwrap();
            let transcript: Value =
                crate::ml_dataset::read_json(&dir.join(&transcript_artifact.path)).unwrap();
            let marker = "FORBIDDEN_LARGE_PRIME_LOG_OR_FUTURE_VALUE";
            domain["seed"] = json!(marker);
            domain["hidden_logs"] = json!(marker);
            domain["recovered_scalar"] = json!(marker);
            if family == "koblitz" {
                assert_eq!(
                    relation["inputs"]["factor_base"]["points"],
                    domain["small_points"]
                );
                assert_eq!(
                    relation["inputs"]["residual_domain"]["points"],
                    domain["residual_points"]
                );
                assert_ne!(
                    relation["inputs"]["factor_base"]["points"],
                    ordinary["inputs"]["factor_base"]["points"]
                );
                for row in domain["small_points"].as_array_mut().unwrap() {
                    row["scalar"] = json!(marker);
                }
            } else {
                assert_eq!(
                    relation["inputs"]["factor_base"]["points"],
                    domain["small"]["points"]
                );
                assert_eq!(
                    relation["inputs"]["residual_domain"]["points"],
                    domain["residual"]["domain"]["points"]
                );
                domain["residual"]["construction"] = json!({"private":marker});
                domain["residual"]["projection_metrics"] = json!({"private":marker});
                for key in ["points", "columns"] {
                    for row in domain["small"][key].as_array_mut().unwrap() {
                        row["scalar"] = json!(marker);
                    }
                    for row in domain["residual"]["domain"][key].as_array_mut().unwrap() {
                        row["scalar"] = json!(marker);
                    }
                }
                for row in domain["small"]["signed"].as_array_mut().unwrap() {
                    row["future_pivot"] = json!(marker);
                }
            }
            let projected =
                large_prime_feature_projection(&public, &base, &domain, &transcript).unwrap();
            assert_eq!(projected, relation["inputs"]);
            let encoded = serde_json::to_string(&projected).unwrap();
            for forbidden in [
                marker,
                "scalar",
                "seed",
                "construction",
                "future_pivot",
                "projection_metrics",
                "recovered_column_logs",
            ] {
                assert!(!encoded.contains(forbidden), "{forbidden}");
            }

            // A scalar certificate can be individually group-valid yet not be
            // a recovery of this relation transcript. Bind the two artifacts.
            let mut certificates = manifest.artifacts.clone();
            certificates.retain(|a| a.role != "restricted-independent-scalar-label");
            let witness: Value = crate::ml_dataset::read_json(&dir.join("witness.json")).unwrap();
            let scalar = if transcript["recovered_scalar"].is_null() {
                witness["scalar"].clone()
            } else {
                json!("0")
            };
            let certificate = json!({"scalar":scalar,
                "generator":public["generator"],"target":public["target"]});
            if transcript["recovered_scalar"].is_null() {
                crate::ml_dataset::solvers::verify_solution_artifact(
                    &public,
                    &base,
                    "restricted-independent-scalar-label",
                    &certificate,
                )
                .unwrap();
            }
            certificates.push(
                storage::write_json(
                    &dir,
                    "mismatched-scalar.json",
                    "restricted-independent-scalar-label",
                    &certificate,
                )
                .unwrap(),
            );
            assert!(
                large_prime_cell_projection(&dir, &certificates, &public, &base)
                    .unwrap_err()
                    .contains("scalar certificate differs")
            );

            let mut duplicate = manifest.artifacts.clone();
            duplicate.push(domain_artifact.clone());
            assert!(
                large_prime_cell_projection(&dir, &duplicate, &public, &base)
                    .unwrap_err()
                    .contains("ambiguous")
            );

            let certificate = storage::write_json(
                &dir,
                "mismatched-pdp.json",
                "independently-verified-pdp",
                &json!({}),
            )
            .unwrap();
            assert!(
                verify_projected_pdp_domain(&dir, &[certificate], &projected)
                    .unwrap_err()
                    .contains("cannot label a large-prime")
            );
            let mut wrong_residual = domain.clone();
            let residual = if family == "koblitz" {
                &mut wrong_residual["residual_points"]
            } else {
                &mut wrong_residual["residual"]["domain"]["points"]
            };
            residual
                .as_array_mut()
                .unwrap()
                .push(public["generator"].clone());
            assert!(
                large_prime_feature_projection(&public, &base, &wrong_residual, &transcript)
                    .unwrap_err()
                    .contains("domain differs from verified transcript")
            );

            // A consistent new SHA does not certify a substituted discovery
            // domain. Exact replay and domain comparison must still reject it.
            let key = if family == "koblitz" {
                "small_points"
            } else {
                "small"
            };
            if family == "koblitz" {
                domain[key].as_array_mut().unwrap().pop();
            } else {
                domain[key]["signed"][0]["coefficient"] = json!(0);
            }
            fs::write(
                dir.join(&domain_artifact.path),
                serde_json::to_vec(&domain).unwrap(),
            )
            .unwrap();
            let mut forged_manifest = manifest;
            let artifact = forged_manifest
                .artifacts
                .iter_mut()
                .find(|a| a.role == LARGE_PRIME_DOMAIN_ROLE)
                .unwrap();
            (artifact.sha256, artifact.bytes) =
                crate::store::hash_file(&dir.join(&artifact.path)).unwrap();
            fs::write(
                dir.join("manifest.json"),
                serde_json::to_vec(&forged_manifest).unwrap(),
            )
            .unwrap();
            assert!(export(&source, &f.0.join("forged-domain-export"))
                .unwrap_err()
                .contains("domain differs from verified transcript"));
            assert!(!f.0.join("forged-domain-export/manifest.json").exists());
        }
    }

    #[test]
    fn missing_large_prime_domain_or_transcript_excludes_features_and_retains_provenance() {
        for missing_role in [LARGE_PRIME_DOMAIN_ROLE, LARGE_PRIME_TRANSCRIPT_ROLE] {
            let f = Fixture::new();
            let source = f.source_with_method("prime", "double-large-prime", true);
            let dir = source.join("cells/000000000125");
            let mut manifest: CellManifest =
                crate::ml_dataset::read_json(&dir.join("manifest.json")).unwrap();
            let missing = manifest
                .artifacts
                .iter()
                .find(|a| a.role == missing_role)
                .unwrap()
                .path
                .clone();
            manifest.artifacts.retain(|a| a.role != missing_role);
            fs::remove_file(dir.join(missing)).unwrap();
            fs::write(
                dir.join("manifest.json"),
                serde_json::to_vec(&manifest).unwrap(),
            )
            .unwrap();
            let out = f.0.join("training");
            let exported = export(&source, &out).unwrap();
            assert_eq!(exported["export"]["sealed_source_cells"], 2);
            assert_eq!(exported["export"]["unique_feature_rows"], 1);
            assert_eq!(
                exported["export"]["cells_without_verified_large_prime_domain"],
                1
            );
            let targets: Vec<Value> = fs::read_to_string(out.join("restricted/targets.jsonl"))
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str(line).unwrap())
                .collect();
            let row = targets
                .iter()
                .find(|v| v["solver"] == "double-large-prime")
                .unwrap();
            assert!(row["feature_id"].is_null());
            assert!(row["pdp_domain"].is_null());
            assert_eq!(
                row["feature_status"],
                "missing_verified_large_prime_domain_or_transcript"
            );
            assert!(fs::read_to_string(out.join("restricted/index.jsonl"))
                .unwrap()
                .contains("cells/000000000125"));
        }
    }

    #[test]
    fn rejected_overlap_leaves_source_directories_and_manifests_intact() {
        let f = Fixture::new();
        let source = f.source();
        let original = fs::read(source.join("cells/000000000124/manifest.json")).unwrap();
        for destination in [
            source.join("cells/report"),
            source.join("cells/000000000124/derived"),
        ] {
            assert!(export(&source, &destination)
                .unwrap_err()
                .contains("overlaps"));
            assert!(!destination.exists());
        }
        assert_eq!(fs::read_dir(source.join("cells")).unwrap().count(), 2);
        assert_eq!(
            fs::read(source.join("cells/000000000124/manifest.json")).unwrap(),
            original
        );
        assert_eq!(
            crate::ml_dataset::verify_run(&source, true).unwrap()["completed_receipts"],
            2
        );
        #[cfg(unix)]
        {
            let alias = f.0.join("source-alias");
            std::os::unix::fs::symlink(&source, &alias).unwrap();
            assert!(export(&source, &alias.join("cells/report")).is_err());
            assert!(!source.join("cells/report").exists());
        }
    }

    #[test]
    fn splits_group_all_instances_of_the_same_size() {
        let first = split_for_size(23);
        assert_eq!(first, split_for_size(23));
        assert_eq!(first.0, "field-size-n23");
        assert_ne!(first.0, split_for_size(24).0);
    }
}
