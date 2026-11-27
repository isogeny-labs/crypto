//! Replay the frozen ten-query M31 B677 source into the existing v1 capture.
//! No collector, pair enumeration, rank computation, or scalar recovery runs here.
//! Protocol: research/lp_graph_algorithms_20261123/M31_CAPTURE_REPLAY_20261124.md.
use prime31_algebraic_ic::{Arithmetic, Base, Counts, PRIME31};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[allow(dead_code)]
#[path = "../../../prime31_four_summand_20261122/native/b677_fixture.rs"]
mod b677_fixture;

const MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_ROWS: usize = 4096;
const MAX_QUERIES: usize = 128;
const SOURCE_QUERIES: usize = 10;
const SOURCE_ROWS: usize = 645;
const KEPT_ROWS: usize = 613;
const SOURCE_SCOPE: &str =
    "single-target generic four-sum graph-control; pair enumeration fully charged";
const SOURCE_SEED: &str = "0x7033315f6d340101";
const PUBLIC_TARGET: (u64, u64) = (976_979_610, 1_228_979_761);
const EXPECTED_CLASSES: [usize; 5] = [2, 30, 138, 276, 199];
const EXPECTED_KEPT_BY_QUERY: [usize; SOURCE_QUERIES] =
    [50, 51, 70, 61, 68, 68, 62, 54, 66, 63];

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurveHeader {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    g: (u64, u64),
}

// Deliberate whitelist: producer rank, recovered-scalar, recovery-result,
// timing and charged-work fields are neither deserialized nor consulted.
#[derive(Deserialize)]
struct Source {
    scope: String,
    seed: String,
    curve: CurveHeader,
    public_target: (u64, u64),
    base_size: usize,
    lp_pool_size: usize,
    pair_entries: usize,
    distinct_pair_sums: usize,
    probe_additions: usize,
    fixture_certificate: Value,
    classes_by_base_terms: [usize; 5],
    queries: Vec<Query>,
    relation_rows: Vec<SignedRow>,
}

// The source's per-query rank and timing fields are likewise ignored.
#[derive(Deserialize)]
struct Query {
    label: usize,
    a: u64,
    b: u64,
    query: (u64, u64),
    classes_by_base_terms: [usize; 5],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SignedRow {
    label: usize,
    terms: Vec<(usize, i8)>,
}

#[derive(Serialize)]
struct CaptureRow {
    id: u64,
    small: Vec<(u32, u64)>,
    large: Vec<(u64, u64)>,
    a: u64,
    b: u64,
}

#[derive(Serialize)]
struct Capture {
    schema: &'static str,
    target: (u64, u64),
    small_points: Vec<(u32, (u64, u64))>,
    large_points: Vec<(u64, (u64, u64))>,
    rows: Vec<CaptureRow>,
}

#[derive(Serialize)]
struct QueryJoin {
    label: usize,
    a: u64,
    b: u64,
    query: (u64, u64),
    query_replayed: bool,
    first_source_row_id: u64,
    last_source_row_id: u64,
    source_rows: usize,
    kept_rows: usize,
    excluded_rows: usize,
    classes_by_base_terms: [usize; 5],
}

#[derive(Serialize)]
struct ExcludedRow {
    id: u64,
    label: usize,
    a: u64,
    b: u64,
    reason: &'static str,
    base_terms: usize,
    large_terms: usize,
    terms: Vec<(usize, i8)>,
}

#[derive(Serialize)]
struct ReplayCounts {
    queries: usize,
    source_rows: usize,
    replayed_rows: usize,
    kept_rows: usize,
    excluded_rows: usize,
    classes_by_base_terms: [usize; 5],
    kept_by_lp_arity: [usize; 3],
    excluded_by_lp_arity: [usize; 5],
    source_points: usize,
    small_points: usize,
    large_points: usize,
}

#[derive(Serialize)]
struct ReplayArithmetic {
    fixture_setup: Counts,
    base_classification: Counts,
    group_replay: Counts,
    counters_are_complete_cold_cost: bool,
}

#[derive(Serialize)]
struct PointTables {
    identity: &'static str,
    hybrid_points: Vec<(u64, (u64, u64))>,
    small_points: Vec<(u32, (u64, u64))>,
    large_points: Vec<(u64, (u64, u64))>,
    dickson_terminal: u64,
    fixture_certificate: Value,
    source_identity: Value,
}

#[derive(Serialize)]
struct Audit {
    schema: &'static str,
    status: &'static str,
    source: Value,
    instance: Value,
    counts: ReplayCounts,
    query_joins: Vec<QueryJoin>,
    excluded_rows: Vec<ExcludedRow>,
    point_tables: PointTables,
    arithmetic: ReplayArithmetic,
    custody_limits: [&'static str; 5],
}

fn convert(source: Source, input_bytes: usize) -> Result<(Capture, Audit), String> {
    if source.scope != SOURCE_SCOPE || source.seed != SOURCE_SEED {
        return Err("fixed source scope or seed mismatch".into());
    }
    let curve = source.curve;
    if (curve.p, curve.a, curve.b, curve.n, curve.g)
        != (PRIME31.p, PRIME31.a, PRIME31.b, PRIME31.n, PRIME31.g)
        || source.public_target != PUBLIC_TARGET
    {
        return Err("fixed M31 curve, group, generator or signed target mismatch".into());
    }
    if source.queries.len() > MAX_QUERIES
        || source.queries.len() != SOURCE_QUERIES
        || source.relation_rows.len() > MAX_ROWS
        || source.relation_rows.len() != SOURCE_ROWS
        || source.base_size != 509
        || source.lp_pool_size != 168
        || source.pair_entries != 915_304
        || source.distinct_pair_sums != 915_116
        || source.probe_additions != 9_153_040
        || source.classes_by_base_terms != EXPECTED_CLASSES
    {
        return Err("fixed ten-query source dimensions or class header mismatch".into());
    }

    // Hybrid677 performs the original fixture checks, including check_curve.
    let fixture = b677_fixture::Hybrid677::build()?;
    let reconstructed_certificate =
        serde_json::to_value(fixture.certificate()).map_err(|error| error.to_string())?;
    if source.fixture_certificate != reconstructed_certificate {
        return Err("source fixture certificate differs from native reconstruction".into());
    }
    let points = fixture.points();
    if points.len() != 677 || !points.windows(2).all(|w| w[0].0 < w[1].0) {
        return Err("reconstructed hybrid point ordering or size mismatch".into());
    }

    // Membership comes from the original builder, not a second Dickson formula.
    // Charge this deliberately separate reconstruction in its own counter set.
    let mut classification_ar = Arithmetic::default();
    let base = Base::build(PRIME31, 10, &mut classification_ar)?;
    let base_set: BTreeSet<_> = base.points.iter().copied().collect();
    if base.depth != 10
        || base.values.len() != 1024
        || base.points.len() != 509
        || base_set.len() != 509
        || base.terminal != fixture.terminal()
    {
        return Err("original base reconstruction differs from the fixed B509".into());
    }
    let mut is_small = Vec::with_capacity(points.len());
    let mut small_points = Vec::with_capacity(509);
    let mut large_points = Vec::with_capacity(168);
    for (index, &point) in points.iter().enumerate() {
        let member = base_set.contains(&point);
        is_small.push(member);
        if member {
            small_points.push((index as u32, point));
        } else {
            large_points.push((index as u64, point));
        }
    }
    if small_points.len() != 509 || large_points.len() != 168 {
        return Err("native B509/Y7 partition does not contain 509/168 points".into());
    }

    let mut replay_ar = Arithmetic::default();
    if !replay_ar.on_curve(PRIME31, Some(PUBLIC_TARGET)) {
        return Err("fixed target is not an affine point of the original curve".into());
    }
    let mut query_joins = Vec::with_capacity(SOURCE_QUERIES);
    for (position, query) in source.queries.iter().enumerate() {
        // The frozen producer emits each label once in the range 0..10.
        if query.label != position || query.a >= PRIME31.n || query.b >= PRIME31.n {
            return Err(format!("query {position}: label order or scalar residue mismatch"));
        }
        if !replay_ar.on_curve(PRIME31, Some(query.query)) {
            return Err(format!("query {position}: invalid recorded point"));
        }
        let ag = replay_ar.scalar(PRIME31, query.a, Some(PRIME31.g));
        let bq = replay_ar.scalar(PRIME31, query.b, Some(PUBLIC_TARGET));
        if replay_ar.add(PRIME31, ag, bq) != Some(query.query) {
            return Err(format!("query {position}: recorded point differs from aG+bQ"));
        }
        query_joins.push(QueryJoin {
            label: query.label,
            a: query.a,
            b: query.b,
            query: query.query,
            query_replayed: true,
            first_source_row_id: 0,
            last_source_row_id: 0,
            source_rows: 0,
            kept_rows: 0,
            excluded_rows: 0,
            classes_by_base_terms: [0; 5],
        });
    }

    let mut rows = Vec::with_capacity(KEPT_ROWS);
    let mut excluded_rows = Vec::with_capacity(SOURCE_ROWS - KEPT_ROWS);
    let mut classes = [0usize; 5];
    let mut kept_by_lp_arity = [0usize; 3];
    let mut excluded_by_lp_arity = [0usize; 5];
    let mut replayed_rows = 0;
    let mut previous: Option<(usize, Vec<(usize, i8)>)> = None;
    for (ordinal, row) in source.relation_rows.into_iter().enumerate() {
        if row.label >= SOURCE_QUERIES || row.terms.len() != 4 {
            return Err(format!("row {ordinal}: query label or four-term arity mismatch"));
        }
        if !row
            .terms
            .iter()
            .all(|&(index, sign)| index < points.len() && (sign == 1 || sign == -1))
            || !row.terms.windows(2).all(|w| w[0].0 < w[1].0)
        {
            return Err(format!("row {ordinal}: invalid sign, point index or repeated/unsorted index"));
        }
        // The producer sorts its deduplicated (index,sign) vectors per label.
        // Reject a changed ordering or duplicate; never silently normalize it.
        let key = (row.label, row.terms.clone());
        if previous.as_ref().is_some_and(|prior| prior >= &key) {
            return Err(format!("row {ordinal}: duplicate row or changed producer ordering"));
        }
        previous = Some(key);

        let query = &source.queries[row.label];
        let mut sum = None;
        let mut small = Vec::with_capacity(4);
        let mut large = Vec::with_capacity(4);
        for &(index, sign) in &row.terms {
            let point = Some(points[index]);
            let signed_point = if sign == 1 {
                point
            } else {
                replay_ar.neg(PRIME31, point)
            };
            sum = replay_ar.add(PRIME31, sum, signed_point);
            let coefficient = if sign == 1 { 1 } else { PRIME31.n - 1 };
            if is_small[index] {
                small.push((index as u32, coefficient));
            } else {
                large.push((index as u64, coefficient));
            }
        }
        // Excluded >2-LP rows must pass the same original group replay.
        if sum != Some(query.query) {
            return Err(format!("row {ordinal}: signed four-point group residual is nonzero"));
        }
        replayed_rows += 1;
        let base_terms = small.len();
        let large_terms = large.len();
        classes[base_terms] += 1;
        let join = &mut query_joins[row.label];
        if join.source_rows == 0 {
            join.first_source_row_id = ordinal as u64;
        }
        join.last_source_row_id = ordinal as u64;
        join.source_rows += 1;
        join.classes_by_base_terms[base_terms] += 1;
        if large_terms <= 2 {
            kept_by_lp_arity[large_terms] += 1;
            join.kept_rows += 1;
            rows.push(CaptureRow {
                id: ordinal as u64,
                small,
                large,
                a: query.a,
                b: query.b,
            });
        } else {
            excluded_by_lp_arity[large_terms] += 1;
            join.excluded_rows += 1;
            excluded_rows.push(ExcludedRow {
                id: ordinal as u64,
                label: row.label,
                a: query.a,
                b: query.b,
                reason: "large-prime-arity-exceeds-two",
                base_terms,
                large_terms,
                terms: row.terms,
            });
        }
    }

    if replayed_rows != SOURCE_ROWS
        || classes != source.classes_by_base_terms
        || classes != EXPECTED_CLASSES
        || rows.len() != KEPT_ROWS
        || excluded_rows.len() != 32
        || kept_by_lp_arity != [199, 276, 138]
        || excluded_by_lp_arity != [0, 0, 0, 30, 2]
    {
        return Err("native replay/classification differs from the frozen 645-to-613 projection".into());
    }
    for (position, join) in query_joins.iter().enumerate() {
        if join.source_rows == 0
            || join.classes_by_base_terms != source.queries[position].classes_by_base_terms
            || join.kept_rows != EXPECTED_KEPT_BY_QUERY[position]
        {
            return Err(format!("query {position}: native class or kept-row count differs from source"));
        }
    }

    let counts = ReplayCounts {
        queries: source.queries.len(),
        source_rows: SOURCE_ROWS,
        replayed_rows,
        kept_rows: rows.len(),
        excluded_rows: excluded_rows.len(),
        classes_by_base_terms: classes,
        kept_by_lp_arity,
        excluded_by_lp_arity,
        source_points: points.len(),
        small_points: small_points.len(),
        large_points: large_points.len(),
    };
    let audit = Audit {
        schema: "prime31-b677-capture-audit/v1",
        status: "verified-fixed-input-replay",
        source: json!({
            "repository": "isogeny-labs/crypto",
            "commit": "577d6c2c1c4a527604d965e6fb62c44df7902f36",
            "path": "research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json",
            "git_blob": "7fd27b9888058399d8814cef373d373243e6a1e5",
            "input_sha256_expected": "e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab",
            "input_bytes_expected": 165511,
            "input_bytes_read": input_bytes,
            "scope": source.scope,
            "seed": source.seed,
            "producer_path": "research/prime31_graph_ecdlp_20261123/native/mitm_four.rs",
            "producer_sha256_expected": "6b97389197bcebc85ace5b47096dbc590475a1fd1021590715e66aa88f13fc12",
            "hash_validation": "The replay wrapper binds input, source and output bytes with SHA-256; this converter checks structure, fixture and group equations.",
            "input_kind": "retained generic exhaustive-pair search on a known-answer public target; not planted decompositions or blinded algebraic extraction"
        }),
        instance: json!({ "curve": curve, "public_target": PUBLIC_TARGET }),
        counts,
        query_joins,
        excluded_rows,
        point_tables: PointTables {
            identity: "Original Hybrid677 sorted union, original Base::build(PRIME31,10) membership; original hybrid index retained in both namespaces",
            hybrid_points: points
                .iter()
                .enumerate()
                .map(|(id, &point)| (id as u64, point))
                .collect(),
            small_points: small_points.clone(),
            large_points: large_points.clone(),
            dickson_terminal: base.terminal,
            fixture_certificate: reconstructed_certificate,
            source_identity: json!({
                "fixture_path": "research/prime31_four_summand_20261122/native/b677_fixture.rs",
                "fixture_sha256_expected": "3f491defbc7aa1643192db4d0c7144afcec1f84a41f03489355a866459329624",
                "extra_csv_path": "research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv",
                "extra_csv_sha256_expected": b677_fixture::Y7_CSV_SHA256,
                "arithmetic_path": "research/prime31_ic_20261115/native/lib.rs",
                "arithmetic_sha256_expected": "52b30e41478b0306603db7d09bcb17377cae64efcfd78d4c34db7bf91325a867",
                "derived_table_binding": "The external replay wrapper hashes this audit, its exact extracted point_tables and the capture output."
            }),
        },
        arithmetic: ReplayArithmetic {
            fixture_setup: fixture.counts(),
            base_classification: classification_ar.counts,
            group_replay: replay_ar.counts,
            counters_are_complete_cold_cost: false,
        },
        custody_limits: [
            "The source does not supply a complete per-probe rejected-attempt trace.",
            "The original collector binary/toolchain attestation is not supplied by this input.",
            "The producer arithmetic counter omits its separate fixture setup and bespoke graph arithmetic; it is not a full cold-cost receipt.",
            "Fixture setup, separate Base classification and group replay counters here describe this consumer only; no timing or speedup inference is made.",
            "Producer rank, recovered-scalar, recovery-result and timing fields are ignored. Native replay is not independently signed PCC validation.",
        ],
    };
    let capture = Capture {
        schema: "prime31-lp-capture/v1",
        target: PUBLIC_TARGET,
        small_points,
        large_points,
        rows,
    };
    Ok((capture, audit))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let input_path = args
        .next()
        .ok_or("usage: prime31_b677_capture INPUT.json --receipt AUDIT.json")?;
    if args.next().as_deref() != Some(OsStr::new("--receipt")) {
        return Err("expected --receipt AUDIT.json after the input path".into());
    }
    let receipt_path = args.next().ok_or("missing new audit receipt path")?;
    if args.next().is_some() {
        return Err("unexpected additional argument".into());
    }
    let mut input = Vec::new();
    File::open(input_path)?
        .take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut input)?;
    if input.len() > MAX_BYTES {
        return Err("input exceeds the 8 MiB bound".into());
    }
    let source: Source = serde_json::from_slice(&input)?;
    let (capture, audit) = convert(source, input.len())?;
    // Fully stage both documents. A semantic error never creates a receipt or
    // emits a capture prefix, and an existing receipt is never overwritten.
    let mut capture_bytes = serde_json::to_vec_pretty(&capture)?;
    capture_bytes.push(b'\n');
    let mut audit_bytes = serde_json::to_vec_pretty(&audit)?;
    audit_bytes.push(b'\n');
    if capture_bytes.len() > MAX_BYTES || audit_bytes.len() > MAX_BYTES {
        return Err("serialized output exceeds the 8 MiB bound".into());
    }
    let mut receipt = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(receipt_path)?;
    receipt.write_all(&audit_bytes)?;
    receipt.sync_all()?;
    drop(receipt);
    std::io::stdout().lock().write_all(&capture_bytes)?;
    eprintln!(
        "fixed M31 replay: {} rows verified, {} retained, {} excluded after group replay",
        audit.counts.replayed_rows, audit.counts.kept_rows, audit.counts.excluded_rows
    );
    Ok(())
}
