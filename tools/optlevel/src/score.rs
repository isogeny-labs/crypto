//! Scoring arithmetic of the optimization-level metric (`optlevel.record/v1`).
//!
//! Pure functions over JSON documents.  Definitions and the reasons for them:
//! docs/OPTIMIZATION_LEVEL.md.  Nothing here measures anything; the `calibrate`
//! and `measure` commands produce the documents these functions combine.
//!
//! The arithmetic is kept operation for operation the same as the tool's first
//! (Python) version, so that re-scoring a committed measurement reproduces the
//! committed record bit for bit (tests/golden.rs).
use serde_json::{json, Map, Value};

pub const TOOL_VERSION: &str = "2.0.0";
pub const RECORD_SCHEMA: &str = "optlevel.record/v1";
pub const CALIBRATION_SCHEMA: &str = "optlevel.calibration/v1";
pub const MEASUREMENT_SCHEMA: &str = "optlevel.measurement/v1";
pub const OPS_SCHEMA: &str = "optlevel.ops/v1";

pub const TIERS: [&str; 4] = ["T-pmu", "T-timing", "T-gpu-ncu", "T-gpu-timing"];
/// Published grade rule (docs/OPTIMIZATION_LEVEL.md section 6).  Lower edges.
pub const GRADE_EDGES: [(&str, f64); 5] = [
    ("A", 0.70),
    ("B", 0.40),
    ("C", 0.20),
    ("D", 0.08),
    ("E", 0.0),
];
pub const NEAR_ROOF: f64 = 0.70; // an attainment at or above this is "near the roof"
pub const CEILING_SLACK: f64 = 0.02; // attainment above 1 + slack + noise raises CEILING_EXCEEDED
pub const MIN_KERNEL_SECONDS: f64 = 0.5; // shorter timed regions raise TOO_SHORT
pub const CONTENDED_CORES: f64 = 0.25; // CPU-seconds per second used by everything else on the host
pub const PMU_ONLY: [&str; 9] = [
    "ipc",
    "cycles_per_op",
    "instructions_per_op",
    "branch_miss_rate",
    "llc_misses_per_op",
    "l1d_misses_per_op",
    "dtlb_misses_per_op",
    "topdown",
    "vector_share_sampled",
];
/// Operation classes scored against another class's compute ceiling when the
/// calibration has no entry of their own: a streamed triad performs one
/// multiply-add per element, so its compute roof is the fma_f64 ceiling and
/// its memory roof is bandwidth / declared bytes per operation.
pub const CLASS_ALIASES: [(&str, &str); 1] = [("triad", "fma_f64")];
pub const NOT_A_SPEED_CLAIM: &str =
    "An optimization level describes how close one implementation runs to what its own \
machine can do for the operations it counts. It is not a speed result, not a \
comparison between algorithms, not a benchmark or leaderboard row, and not evidence \
under the validation gate. A perfectly tuned bad algorithm scores high.";
const GRADE_RULE: &str = "sqrt(A_single * A_full); letters A>=0.70 B>=0.40 C>=0.20 D>=0.08 E";

pub type R<T> = Result<T, String>;

// ---- small JSON helpers ------------------------------------------------------
pub fn f(v: &Value, k: &str) -> Option<f64> {
    v.get(k).and_then(Value::as_f64)
}
/// Python truthiness of a JSON value (what `if x:` tests).
pub fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().map(|x| x != 0.0).unwrap_or(true),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(a)) => !a.is_empty(),
        Some(Value::Object(o)) => !o.is_empty(),
    }
}
fn strs(v: Option<&Value>) -> Vec<String> {
    v.and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}
fn obj(v: Value) -> Map<String, Value> {
    match v {
        Value::Object(m) => m,
        _ => Map::new(),
    }
}

/// median/min/max/n of the values; `Null` when there are none.
pub fn stats(xs: &[f64]) -> Value {
    let mut xs: Vec<f64> = xs.to_vec();
    if xs.is_empty() {
        return Value::Null;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = xs.len();
    let med = if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    };
    json!({"median": med, "min": xs[0], "max": xs[n - 1], "n": n})
}
pub fn stats_opt(xs: impl IntoIterator<Item = Option<f64>>) -> Value {
    stats(&xs.into_iter().flatten().collect::<Vec<_>>())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interp {
    pub interpolated: bool,
    pub extrapolated: bool,
}

/// Value at k threads from measured points (threads, value).
///
/// Exact when k was measured; linear between the two neighbours otherwise
/// (interpolated); clamped to the largest measured point beyond it
/// (extrapolated), because nothing was measured there.
pub fn interp_scale(points: &[(i64, f64)], k: i64) -> R<(f64, Interp)> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| a.0.cmp(&b.0));
    if pts.is_empty() {
        return Err("no scaling points".into());
    }
    for &(a, b) in &pts {
        if a == k {
            return Ok((
                b,
                Interp {
                    interpolated: false,
                    extrapolated: false,
                },
            ));
        }
    }
    let (first, last) = (pts[0], pts[pts.len() - 1]);
    if k < first.0 {
        return Ok((
            first.1 * k as f64 / first.0 as f64,
            Interp {
                interpolated: true,
                extrapolated: false,
            },
        ));
    }
    if k > last.0 {
        return Ok((
            last.1,
            Interp {
                interpolated: false,
                extrapolated: true,
            },
        ));
    }
    for w in pts.windows(2) {
        let ((a0, b0), (a1, b1)) = (w[0], w[1]);
        if a0 < k && k < a1 {
            return Ok((
                b0 + (b1 - b0) * (k - a0) as f64 / (a1 - a0) as f64,
                Interp {
                    interpolated: true,
                    extrapolated: false,
                },
            ));
        }
    }
    Err("unreachable".into())
}

/// Classic two-roof roofline in operations per second.
///
/// compute roof = peak rate of the operation class;
/// memory roof  = bandwidth / bytes per operation (= intensity * bandwidth);
/// bound        = the lower one.  With unknown traffic the bound is the compute
/// roof and `binding` says so: the true bound can only be lower, so an
/// attainment against it is a lower bound.
pub fn roofline(peak: f64, bandwidth: f64, bytes_per_op: Option<f64>) -> R<Map<String, Value>> {
    if !(peak > 0.0) {
        return Err("peak rate must be positive".into());
    }
    let mut out = obj(
        json!({"compute_roof_ops_per_s": peak, "memory_roof_ops_per_s": null, "bound_ops_per_s": peak,
                              "binding": "compute (traffic unknown)", "next_roof_ratio": null}),
    );
    let Some(bpo) = bytes_per_op else {
        return Ok(out);
    };
    if bpo <= 0.0 {
        out.insert("binding".into(), json!("compute"));
        return Ok(out);
    }
    let mem = bandwidth / bpo;
    out.insert("memory_roof_ops_per_s".into(), json!(mem));
    out.insert("bound_ops_per_s".into(), json!(peak.min(mem)));
    out.insert(
        "binding".into(),
        json!(if mem < peak { "memory" } else { "compute" }),
    );
    out.insert(
        "next_roof_ratio".into(),
        json!(peak.max(mem) / peak.min(mem)),
    );
    Ok(out)
}

/// achieved / bound, with an interval from the run spread and, when the
/// calibration's own median ceiling is given as bound_low, from the ceiling
/// spread too (ceilings are maxima over repeats; the median is the low end).
pub fn attainment(rate_stats: &Value, bound: f64, bound_low: Option<f64>) -> Value {
    if rate_stats.is_null() || bound == 0.0 {
        return Value::Null;
    }
    let lo_bound = match bound_low {
        Some(x) if x != 0.0 => x,
        _ => bound,
    };
    json!({"value": f(rate_stats, "median").unwrap_or(f64::NAN) / bound,
           "low": f(rate_stats, "min").unwrap_or(f64::NAN) / bound,
           "high": f(rate_stats, "max").unwrap_or(f64::NAN) / lo_bound})
}

/// A/A noise floor of one kernel: the same command in interleaved slots A and
/// B.  aa_ratio_deviation = |median(A)/median(B) - 1|; rel_range =
/// (max - min)/median over all runs.  The larger one is `value`.
pub fn noise_floor(a: &[f64], b: &[f64]) -> Value {
    if a.is_empty() || b.is_empty() {
        return Value::Null;
    }
    let (sa, sb) = (stats(a), stats(b));
    let all = stats(&[a, b].concat());
    let dev = (f(&sa, "median").unwrap() / f(&sb, "median").unwrap() - 1.0).abs();
    let rng = (f(&all, "max").unwrap() - f(&all, "min").unwrap()) / f(&all, "median").unwrap();
    let pairs = a.len().min(b.len());
    json!({"aa_ratio_deviation": dev, "rel_range": rng, "value": dev.max(rng), "pairs": pairs, "sufficient": pairs >= 5})
}

pub fn letter(score: Option<f64>) -> Option<&'static str> {
    let s = score?;
    if s.is_nan() {
        return None;
    }
    for (name, edge) in GRADE_EDGES {
        if s >= edge {
            return Some(name);
        }
    }
    Some("E")
}

/// Letters the score could carry inside its noise floor, e.g. "B" or "B/C".
pub fn letter_range(score: Option<f64>, rel_noise: f64) -> Option<String> {
    let s = score?;
    let (hi, lo) = (
        letter(Some(s * (1.0 + rel_noise))),
        letter(Some(s * (1.0 - rel_noise))),
    );
    match (hi, lo) {
        (Some(h), Some(l)) if h == l => Some(h.to_string()),
        (h, l) => Some(format!("{}/{}", h.unwrap_or("None"), l.unwrap_or("None"))),
    }
}

/// Raw and roofline-normalised parallel efficiency.
///
/// raw        = rate(k) / (k * rate(1))
/// normalised = [rate(k)/rate(1)] / [bound(k)/bound(1)]  = A_full / A_single
/// The second does not charge the kernel for the machine's own saturation
/// (memory bandwidth and all-core frequency do not scale with k).
pub fn parallel_block(
    single_rate: f64,
    full_rate: f64,
    threads: i64,
    bound_single: f64,
    bound_full: f64,
    curve: Value,
) -> Value {
    if single_rate == 0.0 || full_rate == 0.0 || threads == 0 {
        return Value::Null;
    }
    let k = threads as f64;
    json!({"threads": threads, "raw_efficiency": full_rate / (k * single_rate),
           "roofline_normalised_efficiency": (full_rate / single_rate) / (bound_full / bound_single),
           "machine_scaling": bound_full / (k * bound_single),
           "curve": if curve.is_null() { json!([]) } else { curve }})
}

/// Published rule: score = sqrt(A_single * A_full) when both operating points
/// were measured (scope "single+parallel"), else the one measured (scope names
/// it).  A score is never compared across scopes.
pub fn summary_grade(
    a_single: Option<f64>,
    a_full: Option<f64>,
    rel_noise: f64,
    invalid: &[String],
) -> Map<String, Value> {
    if !invalid.is_empty() {
        return obj(
            json!({"score": null, "letter": null, "letter_range": null, "scope": "invalid", "invalid": invalid, "rule": GRADE_RULE}),
        );
    }
    let (s, scope) = match (a_single, a_full) {
        (Some(a), Some(b)) => (Some((a * b).sqrt()), "single+parallel"),
        (Some(a), None) => (Some(a), "single-thread only"),
        (None, Some(b)) => (Some(b), "parallel only"),
        (None, None) => (None, "invalid"),
    };
    obj(
        json!({"score": s, "letter": letter(s), "letter_range": letter_range(s, rel_noise), "scope": scope,
               "invalid": [], "rule": GRADE_RULE}),
    )
}

/// Name the largest non-retiring top-down share, refined by level 2.
pub fn topdown_limit(td: &Value) -> Option<String> {
    let fe = f(td, "frontend_bound")?;
    let bs = f(td, "bad_speculation")?;
    let be = f(td, "backend_bound")?;
    let ret = f(td, "retiring")?;
    // first maximum in the order frontend, bad speculation, backend
    let mut worst = ("frontend_bound", fe);
    for cand in [("bad_speculation", bs), ("backend_bound", be)] {
        if cand.1 > worst.1 {
            worst = cand;
        }
    }
    if ret >= worst.1.max(0.5) {
        return Some("retiring-dominated: the core is busy with useful slots, so the loss is instructions per operation (not vectorized, or overhead around the arithmetic)".into());
    }
    Some(match worst.0 {
        "backend_bound" => match (f(td, "memory_bound"), f(td, "core_bound")) {
            (Some(mem), Some(core)) => {
                if mem >= core {
                    "backend: memory bound (cache or TLB misses inside a compute-bound kernel)"
                        .into()
                } else {
                    "backend: core bound (dependency chains, divider, port contention)".into()
                }
            }
            _ => "backend bound (level 2 unavailable)".into(),
        },
        "frontend_bound" => "frontend bound (decode, instruction-cache or BTB misses)".into(),
        _ => "bad speculation (branch mispredicts, machine clears)".to_string(),
    })
}

/// The published decision rule, in order.  Returns (text, basis).
pub fn limiting_factor(
    tier: &str,
    roof: &Value,
    a_single: Option<f64>,
    par: &Value,
    diag: &Value,
) -> (String, String) {
    if a_single.is_none() && par.is_null() {
        return ("nothing measured".into(), "none".into());
    }
    let norm = f(par, "roofline_normalised_efficiency");
    let binding = roof.get("binding").and_then(Value::as_str).unwrap_or("");
    if a_single.map(|a| a >= NEAR_ROOF).unwrap_or(false)
        && norm.map(|n| n >= NEAR_ROOF).unwrap_or(true)
    {
        if binding == "memory" {
            return ("at the memory-bandwidth roof: only higher arithmetic intensity (blocking, packing, \
                     fewer bytes per operation) or a different algorithm helps".into(), "roofline".into());
        }
        return (
            "none above threshold: near the roofline; what remains is the algorithmic axis".into(),
            "roofline".into(),
        );
    }
    if let Some(n) = norm {
        if a_single.map(|a| n < a).unwrap_or(true) {
            let threads = par.get("threads").and_then(Value::as_i64).unwrap_or(0);
            return (format!("parallel scaling: roofline-normalised efficiency {n:.2} at {threads} threads \
                             (synchronisation, serial sections, false sharing, or a host-side feeder)"), "scaling curve".into());
        }
    }
    if binding == "memory" {
        if let Some(lat) = f(diag, "latency_attainment") {
            if (0.5..=2.0).contains(&lat) {
                return (format!("memory latency bound: the rate matches one outstanding dependent miss at a time \
                                 (latency attainment {lat:.2}); far below the bandwidth roof"), "roofline + latency calibration".into());
            }
        }
        if tier == "T-pmu"
            && f(diag, "dtlb_misses_per_op")
                .map(|x| x > 0.05)
                .unwrap_or(false)
        {
            return (
                "memory bound below the bandwidth roof with TLB misses: page size or access order"
                    .into(),
                "pmu".into(),
            );
        }
        return ("memory bound, below the bandwidth roof: random or dependent access, TLB, or NUMA placement".into(), "roofline".into());
    }
    if tier == "T-pmu" && truthy(diag.get("topdown")) {
        if let Some(t) = topdown_limit(&diag["topdown"]) {
            return (t, "pmu top-down".into());
        }
    }
    let sr = f(diag, "scalar_ceiling_ratio");
    let head = f(diag, "simd_headroom").filter(|h| *h != 0.0);
    let vs = f(diag, "vector_share_static");
    let suffix = if binding == "compute" {
        ""
    } else {
        " (memory traffic unknown: the bound may be lower than assumed)"
    };
    if let (Some(vs), Some(head)) = (vs, head) {
        if vs < 0.2 && head > 1.5 {
            return (format!("core bound / not vectorized (static vector share {vs:.2} in the named symbols; \
                             the class's SIMD ceiling is {head:.1}x its scalar ceiling){suffix}"), "timing + static disassembly".into());
        }
    }
    if let (Some(sr), Some(head)) = (sr, head) {
        if head > 1.5 && sr <= 1.15 {
            return (format!("core bound / not vectorized or a serial dependency: the rate is {sr:.2}x the class's scalar \
                             ceiling and the SIMD ceiling is {head:.1}x higher (inferred from timing; \
                             a PMU top-down and a vector share are needed to split the two){suffix}"), "timing inference".into());
        }
    }
    (format!("core bound below the compute roof: instruction overhead, partial vectorization or dependency \
              chains (timing tier cannot split these){suffix}"), "timing inference".into())
}

/// Error if a record claims a component its tier cannot have measured.
pub fn enforce_tier(record: &Value) -> R<()> {
    let tier = record.get("tier").and_then(Value::as_str).unwrap_or("");
    if !TIERS.contains(&tier) {
        return Err(format!(
            "unknown tier {:?}",
            record.get("tier").unwrap_or(&Value::Null).to_string()
        ));
    }
    let null = Value::Null;
    let diag = record.get("diagnosis").unwrap_or(&null);
    let present = |v: Option<&Value>| v.map(|x| !x.is_null()).unwrap_or(false);
    if tier != "T-pmu" {
        let bad: Vec<&str> = PMU_ONLY
            .iter()
            .copied()
            .filter(|k| present(diag.get(*k)))
            .collect();
        if !bad.is_empty() {
            return Err(format!(
                "tier {tier} must not report PMU components: {}",
                bad.join(", ")
            ));
        }
        let src = record
            .get("intensity")
            .and_then(|i| i.get("source"))
            .and_then(Value::as_str)
            .unwrap_or("");
        if src == "llc-miss" || src == "imc" {
            return Err(format!(
                "tier {tier} cannot have measured memory traffic ({src})"
            ));
        }
    }
    if tier != "T-gpu-ncu" && present(diag.get("gpu").and_then(|g| g.get("ncu"))) {
        return Err(format!("tier {tier} must not report ncu components"));
    }
    if !tier.starts_with("T-gpu") && present(diag.get("gpu")) {
        return Err("a CPU-tier record must not carry GPU components".into());
    }
    if !present(record.get("noise_floor"))
        && present(record.get("grade").and_then(|g| g.get("score")))
    {
        return Err("a graded record must carry its A/A noise floor".into());
    }
    Ok(())
}

// ---- a small JSON-schema subset validator (type, required, properties, enum,
// const, items, minimum, maximum, additionalProperties: false) -----------------
fn type_ok(v: &Value, t: &str) -> bool {
    match t {
        "number" => v.is_number(),
        "integer" => v.is_i64() || v.is_u64(),
        "object" => v.is_object(),
        "array" => v.is_array(),
        "string" => v.is_string(),
        "boolean" => v.is_boolean(),
        "null" => v.is_null(),
        _ => false,
    }
}
fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
/// JSON equality with numbers compared by value (1 == 1.0, as in the schema).
fn json_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        _ => a == b,
    }
}

pub fn validate(doc: &Value, schema: &Value) -> Vec<String> {
    let mut errs = Vec::new();
    validate_at(doc, schema, "$", &mut errs);
    errs
}
fn validate_at(doc: &Value, schema: &Value, path: &str, errs: &mut Vec<String>) {
    if let Some(t) = schema.get("type") {
        let ts: Vec<&str> = match t {
            Value::Array(a) => a.iter().filter_map(Value::as_str).collect(),
            Value::String(s) => vec![s.as_str()],
            _ => vec![],
        };
        if !ts.iter().any(|x| type_ok(doc, x)) {
            errs.push(format!("{path}: expected {t}, got {}", type_name(doc)));
            return;
        }
    }
    if let Some(en) = schema.get("enum").and_then(Value::as_array) {
        if !en.iter().any(|e| json_eq(e, doc)) {
            errs.push(format!("{path}: {doc} not in {}", Value::Array(en.clone())));
        }
    }
    if let Some(c) = schema.get("const") {
        if !json_eq(c, doc) {
            errs.push(format!("{path}: expected {c}"));
        }
    }
    if let Some(x) = doc.as_f64() {
        if let Some(m) = f(schema, "minimum") {
            if x < m {
                errs.push(format!("{path}: {doc} < {m}"));
            }
        }
        if let Some(m) = f(schema, "maximum") {
            if x > m {
                errs.push(format!("{path}: {doc} > {m}"));
            }
        }
    }
    if let Some(o) = doc.as_object() {
        for k in schema
            .get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
        {
            if !o.contains_key(k) {
                errs.push(format!("{path}: missing {k}"));
            }
        }
        let props = schema.get("properties").and_then(Value::as_object);
        for (k, v) in o {
            if let Some(p) = props.and_then(|p| p.get(k)) {
                validate_at(v, p, &format!("{path}.{k}"), errs);
            } else if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
                errs.push(format!("{path}: unexpected {k}"));
            }
        }
    }
    if let (Some(a), Some(items)) = (doc.as_array(), schema.get("items")) {
        for (i, v) in a.iter().enumerate() {
            validate_at(v, items, &format!("{path}[{i}]"), errs);
        }
    }
}

// ---- record assembly ---------------------------------------------------------
fn scale_points(table: &Value, field: &str) -> Vec<(i64, f64)> {
    table
        .as_object()
        .map(|o| {
            o.iter()
                .filter_map(|(k, v)| Some((k.parse().ok()?, f(v, field)?)))
                .collect()
        })
        .unwrap_or_default()
}

/// The calibration entry an operation class is scored against: its own, or its
/// alias's (CLASS_ALIASES).  Returns (entry, name of the class actually used).
pub fn class_entry<'a>(calib: &'a Value, op_class: &str) -> R<(&'a Value, String)> {
    let compute = calib.get("compute").and_then(Value::as_object);
    if let Some(c) = compute
        .and_then(|c| c.get(op_class))
        .filter(|c| truthy(Some(c)))
    {
        return Ok((c, op_class.to_string()));
    }
    for (from, to) in CLASS_ALIASES {
        if from == op_class {
            if let Some(c) = compute.and_then(|c| c.get(to)).filter(|c| truthy(Some(c))) {
                return Ok((c, to.to_string()));
            }
        }
    }
    let mut names: Vec<&String> = compute.map(|c| c.keys().collect()).unwrap_or_default();
    names.sort();
    Err(format!(
        "calibration has no ceiling for operation class {op_class:?}; classes: {names:?}"
    ))
}

/// (peak, peak_median, meta) of an operation class at `threads` cores.
pub fn class_ceiling(calib: &Value, op_class: &str, threads: i64) -> R<(f64, f64, Interp)> {
    let (c, _) = class_entry(calib, op_class)?;
    let (peak, meta) = interp_scale(&scale_points(&c["total_ops_per_s"], "max"), threads)?;
    let (med, _) = interp_scale(&scale_points(&c["total_ops_per_s"], "median"), threads)?;
    Ok((peak, med, meta))
}

pub fn bandwidth(calib: &Value, threads: i64) -> R<(f64, f64, Interp)> {
    let b = &calib["memory"]["bandwidth_bytes_per_s"];
    let (peak, meta) = interp_scale(&scale_points(b, "max"), threads)?;
    let (med, _) = interp_scale(&scale_points(b, "median"), threads)?;
    Ok((peak, med, meta))
}

#[derive(Default)]
pub struct Extras {
    pub algorithmic: Option<Value>,
    pub extra_diag: Map<String, Value>,
    pub kernel: Map<String, Value>,
}

/// Combine one calibration and one measurement into `optlevel.record/v1`.
/// `calib["_sha256"]`, when present, is the SHA-256 of the calibration file.
pub fn build_record(calib: &Value, meas: &Value, extras: Extras) -> R<Value> {
    let mut flags = strs(meas.get("flags"));
    let tier = meas
        .get("tier")
        .and_then(Value::as_str)
        .ok_or("measurement has no tier")?
        .to_string();
    let op_class = meas
        .get("op_class")
        .and_then(Value::as_str)
        .ok_or("measurement has no op_class")?
        .to_string();
    if calib["host"]["fingerprint"] != meas["host"]["fingerprint"] {
        flags.push("CALIBRATION_HOST_MISMATCH".into());
    }
    for fl in strs(calib.get("flags")) {
        flags.push(format!("CALIBRATION_{fl}"));
    }
    let (_, ceiling_class) = class_entry(calib, &op_class)?;
    if ceiling_class != op_class {
        flags.push(format!(
            "CEILING_CLASS_ALIAS_{}",
            ceiling_class.to_uppercase()
        ));
    }

    // memory traffic: measured sources outrank the declared one
    let declared = meas
        .get("declared_bytes_per_op")
        .cloned()
        .unwrap_or(Value::Null);
    let mut intensity = obj(
        json!({"source": "none", "bytes_per_op": null, "ops_per_byte": null,
                                   "declared_bytes_per_op": declared, "measured_bytes_per_op": null, "declared_over_measured": null}),
    );
    let mt_bpo = meas
        .get("memory_traffic")
        .and_then(|m| f(m, "bytes_per_op"));
    if tier == "T-pmu" && mt_bpo.is_some() {
        let b = mt_bpo.unwrap();
        intensity.insert("source".into(), meas["memory_traffic"]["source"].clone());
        intensity.insert("bytes_per_op".into(), json!(b));
        intensity.insert("measured_bytes_per_op".into(), json!(b));
        if let Some(d) = declared.as_f64() {
            if d != 0.0 && b > 0.0 {
                intensity.insert("declared_over_measured".into(), json!(d / b));
            }
        }
    } else if !declared.is_null() {
        intensity.insert("source".into(), json!("declared"));
        intensity.insert("bytes_per_op".into(), declared.clone());
        let ws = meas
            .get("working_set_bytes")
            .cloned()
            .unwrap_or(Value::Null);
        let llc = calib
            .get("memory")
            .and_then(|m| f(m, "llc_bytes"))
            .filter(|x| *x != 0.0);
        intensity.insert("working_set_bytes".into(), ws.clone());
        if let (Some(w), Some(llc)) = (ws.as_f64(), llc) {
            if w <= llc {
                // the bytes stay in cache: nothing is charged to DRAM.  Cache-level
                // bandwidths are not calibrated, so between L2 and the LLC the
                // compute roof is an upper bound on the true roof.
                intensity.insert("bytes_per_op".into(), json!(0.0));
                intensity.insert("in_cache".into(), json!(true));
                let l2 = calib["host"]
                    .get("caches_bytes")
                    .and_then(Value::as_object)
                    .and_then(|c| {
                        c.iter()
                            .filter(|(k, _)| k.starts_with("L2"))
                            .filter_map(|(_, v)| v.as_f64())
                            .reduce(f64::max)
                    });
                if l2.map(|l2| l2 != 0.0 && w > l2).unwrap_or(false) {
                    flags.push("WORKING_SET_IN_LLC".into());
                }
            }
        }
    }
    let bytes_per_op = intensity["bytes_per_op"].as_f64();
    if let Some(b) = bytes_per_op.filter(|b| *b != 0.0) {
        intensity.insert("ops_per_byte".into(), json!(1.0 / b));
    }
    if intensity["source"] == "none" {
        flags.push("INTENSITY_UNKNOWN".into());
    }

    let points = meas
        .get("points")
        .and_then(Value::as_object)
        .ok_or("measurement has no points")?;
    let single = points.get("1").filter(|p| truthy(Some(p)));
    let full_k = points
        .keys()
        .filter_map(|k| k.parse::<i64>().ok())
        .filter(|k| *k > 1)
        .max();
    let full = full_k
        .and_then(|k| points.get(&k.to_string()))
        .filter(|p| truthy(Some(p)));

    let roof_at = |k: i64| -> R<Map<String, Value>> {
        let (peak, peak_med, m1) = class_ceiling(calib, &op_class, k)?;
        let (bw, bw_med, m2) = bandwidth(calib, k)?;
        let mut r = roofline(peak, bw, bytes_per_op)?;
        let lo = roofline(peak_med, bw_med, bytes_per_op)?["bound_ops_per_s"].clone();
        r.insert("threads".into(), json!(k));
        r.insert("bandwidth_bytes_per_s".into(), json!(bw));
        r.insert("bound_low_ops_per_s".into(), lo);
        r.insert(
            "ceiling_interpolated".into(),
            json!(m1.interpolated || m2.interpolated),
        );
        r.insert(
            "ceiling_extrapolated".into(),
            json!(m1.extrapolated || m2.extrapolated),
        );
        Ok(r)
    };
    let att_of = |p: &Value, r: &Map<String, Value>| {
        attainment(
            &p["ops_per_s"],
            r["bound_ops_per_s"].as_f64().unwrap_or(0.0),
            r["bound_low_ops_per_s"].as_f64(),
        )
    };

    let (mut roofs, mut att) = (Map::new(), Map::new());
    if let Some(s) = single {
        let r = roof_at(1)?;
        att.insert("single".into(), att_of(s, &r));
        roofs.insert("single".into(), Value::Object(r));
    }
    if let (Some(fp), Some(k)) = (full, full_k) {
        let r = roof_at(k)?;
        att.insert("full".into(), att_of(fp, &r));
        if r["ceiling_extrapolated"] == true {
            flags.push("CEILING_EXTRAPOLATED".into());
        }
        roofs.insert("full".into(), Value::Object(r));
    }
    let a1 = att.get("single").and_then(|a| f(a, "value"));
    let an = att.get("full").and_then(|a| f(a, "value"));
    // Validity of the full point: k copies need k free cores.  When everything
    // else on the host used more than CONTENDED_CORES during those runs, the
    // full point is reported but neither graded nor used for diagnosis.
    let full_other = full
        .and_then(|p| p.get("other_cpu_cores"))
        .and_then(|o| f(o, "median"));
    let full_usable = full.is_some() && !full_other.map(|o| o > CONTENDED_CORES).unwrap_or(false);
    if full.is_some() && !full_usable {
        flags.push("FULL_POINT_CONTENDED".into());
    }

    let mut par = Value::Null;
    if let (Some(s), Some(fp), Some(k)) = (single, full, full_k) {
        let s_med = f(&s["ops_per_s"], "median").unwrap_or(f64::NAN);
        let mut keys: Vec<i64> = points.keys().filter_map(|k| k.parse().ok()).collect();
        keys.sort();
        let curve: Vec<Value> = keys
            .iter()
            .map(|kk| {
                let m = f(&points[&kk.to_string()]["ops_per_s"], "median").unwrap_or(f64::NAN);
                json!({"threads": kk, "ops_per_s": m, "raw_efficiency": m / (*kk as f64 * s_med)})
            })
            .collect();
        par = parallel_block(
            s_med,
            f(&fp["ops_per_s"], "median").unwrap_or(f64::NAN),
            k,
            f(&roofs["single"], "bound_ops_per_s").unwrap_or(f64::NAN),
            f(&roofs["full"], "bound_ops_per_s").unwrap_or(f64::NAN),
            Value::Array(curve),
        );
    }

    let mut nf = meas.get("noise_floor").cloned().unwrap_or(Value::Null);
    let rel_noise = f(&nf, "value").unwrap_or(0.0);
    if nf.is_null() {
        flags.push("NO_NOISE_FLOOR".into());
    } else if !truthy(nf.get("sufficient")) {
        flags.push("NOISE_FLOOR_UNDER_5_PAIRS".into());
    }

    let mut diag = Map::new();
    for k in PMU_ONLY {
        diag.insert(k.into(), Value::Null);
    }
    for k in [
        "scalar_ceiling_ratio",
        "simd_headroom",
        "vector_share_static",
        "latency_attainment",
        "gpu",
    ] {
        diag.insert(k.into(), Value::Null);
    }
    diag.insert(
        "startup_share".into(),
        meas.get("startup_share").cloned().unwrap_or(Value::Null),
    );
    let mut unavailable = Map::new();
    let cc = class_entry(calib, &op_class)?.0;
    if let Some(s) = single {
        let s_med = f(&s["ops_per_s"], "median").unwrap_or(f64::NAN);
        if let (Some(sc), Some(si)) = (
            f(cc, "scalar_ops_per_s").filter(|x| *x != 0.0),
            f(cc, "simd_ops_per_s").filter(|x| *x != 0.0),
        ) {
            diag.insert("scalar_ceiling_ratio".into(), json!(s_med / sc));
            diag.insert("simd_headroom".into(), json!(si / sc));
        }
        let lat = calib
            .get("memory")
            .and_then(|m| m.get("latency_ns"))
            .filter(|l| truthy(Some(l)));
        if let Some(lat) = lat {
            if roofs["single"]["binding"] == "memory" {
                // dependent misses per second one thread can serve, one at a time
                diag.insert(
                    "latency_attainment".into(),
                    json!(s_med * f(lat, "median").unwrap_or(f64::NAN) * 1e-9),
                );
            }
        }
    }
    if tier == "T-pmu" {
        if let Some(pmu) = meas.get("pmu").and_then(Value::as_object) {
            for (k, v) in pmu {
                if diag.contains_key(k) {
                    diag.insert(k.clone(), v.clone());
                }
            }
        }
    } else {
        let why = meas
            .get("pmu_unavailable_reason")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .unwrap_or("no hardware PMU on the measuring host");
        for k in PMU_ONLY {
            unavailable.insert(k.into(), json!(why));
        }
    }
    diag.insert("unavailable".into(), Value::Object(unavailable));
    for (k, v) in extras.extra_diag {
        diag.insert(k, v);
    }
    if diag["gpu"].is_null() && tier.starts_with("T-gpu") {
        diag.insert(
            "gpu".into(),
            meas.get("gpu").cloned().unwrap_or(Value::Null),
        );
    }

    let mut invalid: Vec<String> = Vec::new();
    for name in ["single", "full"] {
        if name == "full" {
            if !full_usable {
                continue;
            }
            let fs = &full.unwrap()["ops_per_s"];
            let spread = (f(fs, "max").unwrap_or(f64::NAN) - f(fs, "min").unwrap_or(f64::NAN))
                / f(fs, "median").unwrap_or(f64::NAN);
            if let Some(o) = nf.as_object_mut() {
                o.insert("full_rel_range".into(), json!(spread));
            }
        }
        let slack = CEILING_SLACK + rel_noise; // the single-thread A/A floor, for both points
        if let Some(v) = att.get(name).and_then(|a| f(a, "value")) {
            if v > 1.0 + slack {
                flags.push("CEILING_EXCEEDED".into());
                invalid.push(format!("attainment {v:.2} at the {name} point exceeds 1: the calibrated \
                                      ceiling is not a ceiling for this kernel, or its operation counter overcounts"));
                break;
            }
        }
    }
    if flags.iter().any(|x| x == "TOO_SHORT") {
        invalid.push(format!("timed region shorter than {MIN_KERNEL_SECONDS} s"));
    }
    if flags.iter().any(|x| x == "OPS_MISSING") {
        invalid.push("the command reported no operation count".into());
    }

    let diag = Value::Object(diag);
    let mut grade = summary_grade(a1, if full_usable { an } else { None }, rel_noise, &invalid);
    let (text, basis) = if !grade["score"].is_null() && !roofs.is_empty() {
        let roof = roofs.get("single").or(roofs.get("full")).unwrap();
        limiting_factor(
            &tier,
            roof,
            a1,
            if full_usable { &par } else { &Value::Null },
            &diag,
        )
    } else {
        (
            if invalid.is_empty() {
                "nothing measured".to_string()
            } else {
                format!("invalid: {}", invalid.join("; "))
            },
            "validity checks".to_string(),
        )
    };
    grade.insert("limiting_factor".into(), json!(text));
    grade.insert("limiting_basis".into(), json!(basis));

    let mut kernel = obj(
        json!({"op_class": op_class, "command": meas.get("command").cloned().unwrap_or(Value::Null),
                                "ops_source": meas.get("ops_source").cloned().unwrap_or(Value::Null)}),
    );
    for (k, v) in extras.kernel {
        kernel.insert(k, v);
    }
    let achieved: Map<String, Value> = points
        .iter()
        .map(|(k, v)| {
            (
                if k == "1" {
                    "single".to_string()
                } else {
                    format!("threads_{k}")
                },
                v.clone(),
            )
        })
        .collect();
    flags.sort();
    flags.dedup();
    let algorithmic = extras.algorithmic.filter(|a| truthy(Some(a)))
        .unwrap_or_else(|| json!({"counted_ops": null, "best_known_ops": null, "ratio": null, "note": "not supplied; never folded into the score"}));
    let record = json!({
        "schema": RECORD_SCHEMA, "kind": "record", "tool_version": TOOL_VERSION,
        "created_utc": meas.get("created_utc").cloned().unwrap_or(Value::Null),
        "label": meas.get("label").cloned().unwrap_or(json!("")),
        "tier": tier, "device": if tier.starts_with("T-gpu") { "gpu" } else { "cpu" },
        "kernel": kernel, "host": meas["host"],
        "calibration": {"fingerprint": calib["host"]["fingerprint"], "created_utc": calib.get("created_utc").cloned().unwrap_or(Value::Null),
                        "sha256": calib.get("_sha256").cloned().unwrap_or(Value::Null),
                        "conditions": calib.get("conditions").cloned().unwrap_or(Value::Null)},
        "achieved": achieved, "intensity": intensity, "roofline": roofs, "attainment": att, "parallel": par,
        "diagnosis": diag, "algorithmic": algorithmic, "noise_floor": nf,
        "conditions": meas.get("conditions").cloned().unwrap_or(Value::Null),
        "grade": grade, "flags": flags, "not_a_speed_claim": NOT_A_SPEED_CLAIM,
    });
    enforce_tier(&record)?;
    Ok(record)
}

/// The record's values in benchrec's unit vocabulary (ops_per_s, ratio, ipc).
/// These describe an implementation on a host; they carry no reference kind.
pub fn benchrec_metrics(record: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    let lab = record["kernel"]["op_class"].as_str().unwrap_or("");
    // insertion order of the first tool version: single, then threads_<k> ascending
    let mut names: Vec<&String> = record["achieved"]
        .as_object()
        .map(|o| o.keys().collect())
        .unwrap_or_default();
    names.sort_by_key(|n| {
        if n.as_str() == "single" {
            1
        } else {
            n.trim_start_matches("threads_")
                .parse::<i64>()
                .unwrap_or(i64::MAX)
        }
    });
    for name in names {
        let s = &record["achieved"][name]["ops_per_s"];
        out.push(json!({"name": format!("optlevel:{lab}:{name}:ops_per_s"), "value": s["median"], "unit": "ops_per_s",
                        "better": "higher", "n": s["n"], "spread_low": s["min"], "spread_high": s["max"]}));
    }
    for name in ["single", "full"] {
        if let Some(a) = record["attainment"].get(name) {
            out.push(json!({"name": format!("optlevel:{lab}:{name}:attainment"), "value": a["value"], "unit": "ratio",
                            "better": "neutral", "spread_low": a["low"], "spread_high": a["high"]}));
        }
    }
    if !record["diagnosis"]["ipc"].is_null() {
        out.push(json!({"name": format!("optlevel:{lab}:ipc"), "value": record["diagnosis"]["ipc"], "unit": "ipc", "better": "neutral"}));
    }
    out
}
