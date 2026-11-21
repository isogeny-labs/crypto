//! Ordinary-residue affine arithmetic and metric reconstruction, independent of
//! the producer's Montgomery arithmetic, root finder, and recovery routines.
use crate::{array, canonical, hash, integer, require, text, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[path = "audit/leads.rs"]
mod leads;

const PHASES: [&str; 6] = [
    "setup",
    "target_query",
    "target_PDP",
    "target_relation_check",
    "target_descent",
    "target_recovery_check",
];
pub const MAX_ORACLE_CANDIDATES: u64 = 1_000_000;

/// The separate scalar-check executable profiles this whole arithmetic body.
/// Parsing the input record and serializing the receipt are outside its frame.
/// This is independent ordinary-residue arithmetic, not producer execution
/// attestation. Invalid/wrong recoveries never become accepted answers.
pub fn scalar_record(r: &Value) -> Result<Value> {
    let c = Curve::from_record(r)?;
    let k = integer(&r["outcome"]["recovered"])?;
    Ok(
        json!({"schema":"aps.scalar-check/v1","record_id":r["record_id"],
        "matches_target":k<c.n && c.scale(Some(c.g),k)==Some(c.q),"execution_attested":false}),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}
#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    n: u64,
    g: Point,
    q: Point,
}
fn prime(n: u64) -> bool {
    n >= 2
        && (2..)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
}
fn pow(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e != 0 {
        if e & 1 != 0 {
            r = mul(r, a, p);
        }
        a = mul(a, a, p);
        e >>= 1;
    }
    r
}
fn hex(v: &Value) -> Result<u64> {
    u64::from_str_radix(text(v)?.trim_start_matches("0x"), 16).map_err(|e| e.to_string())
}
impl Curve {
    fn negate(self, p: Point) -> Point {
        Point {
            x: p.x,
            y: (self.p - p.y) % self.p,
        }
    }
    fn valid(self, p: Point) -> bool {
        p.x < self.p
            && p.y < self.p
            && mul(p.y, p.y, self.p)
                == (mul(mul(p.x, p.x, self.p), p.x, self.p) + mul(self.a, p.x, self.p) + self.b)
                    % self.p
    }
    fn add(self, a: Option<Point>, b: Option<Point>) -> Option<Point> {
        let (a, b) = match (a, b) {
            (None, b) => return b,
            (a, None) => return a,
            (Some(a), Some(b)) => (a, b),
        };
        if a.x == b.x && (a.y + b.y) % self.p == 0 {
            return None;
        }
        let (top, bottom) = if a == b {
            (
                (3 * mul(a.x, a.x, self.p) + self.a) % self.p,
                2 * a.y % self.p,
            )
        } else {
            ((b.y + self.p - a.y) % self.p, (b.x + self.p - a.x) % self.p)
        };
        let slope = mul(top, pow(bottom, self.p - 2, self.p), self.p);
        let x = (mul(slope, slope, self.p) + 2 * self.p - a.x - b.x) % self.p;
        Some(Point {
            x,
            y: (mul(slope, (a.x + self.p - x) % self.p, self.p) + self.p - a.y) % self.p,
        })
    }
    fn scale(self, mut p: Option<Point>, mut n: u64) -> Option<Point> {
        let mut r = None;
        while n != 0 {
            if n & 1 != 0 {
                r = self.add(r, p);
            }
            p = self.add(p, p);
            n >>= 1;
        }
        r
    }
    fn from_record(r: &Value) -> Result<Self> {
        let s = &r["workload"]["curve_spec"];
        require(
            matches!(
                s["kind"].as_str(),
                Some("prime_explicit" | "prime_public_fixture")
            ),
            "arithmetic audit requires a frozen explicit prime curve",
        )?;
        let p = integer(&s["p"])?;
        let n = integer(&s["r"])?;
        require(
            (5..=65521).contains(&p) && prime(p),
            "unsupported audit field",
        )?;
        require(
            n > 3 && n <= p + 1 + 2 * (p as f64).sqrt().ceil() as u64 && prime(n),
            "unsupported audit subgroup",
        )?;
        let g = Point {
            x: integer(&s["gx"])?,
            y: integer(&s["gy"])?,
        };
        let q = Point {
            x: hex(&r["workload"]["target"][0])?,
            y: hex(&r["workload"]["target"][1])?,
        };
        let c = Self {
            p,
            a: integer(&s["a"])?,
            b: integer(&s["b"])?,
            n,
            g,
            q,
        };
        require(
            c.a < p
                && c.b < p
                && !(4 * mul(mul(c.a, c.a, p), c.a, p) + 27 * mul(c.b, c.b, p)).is_multiple_of(p),
            "noncanonical or singular curve",
        )?;
        require(
            c.valid(g)
                && c.valid(q)
                && c.scale(Some(g), n).is_none()
                && c.scale(Some(q), n).is_none(),
            "curve/subgroup witness mismatch",
        )?;
        require(
            integer(&r["workload"]["curve"]["r"])? == n,
            "subgroup facts mismatch",
        )?;
        require(
            hex(&r["workload"]["curve"]["generator"][0])? == g.x
                && hex(&r["workload"]["curve"]["generator"][1])? == g.y,
            "generator facts mismatch",
        )?;
        Ok(c)
    }
}
#[derive(Clone, Copy)]
struct Entry {
    point: Point,
    u: u64,
    v: u64,
}
struct Base {
    entries: BTreeMap<u64, Entry>,
    parts: Vec<Vec<u64>>,
}
impl Base {
    fn certify_orbit(&self, c: Curve, beta: u64, lambda: u64) -> Result<()> {
        require(
            c.a == 0
                && beta > 1
                && beta < c.p
                && pow(beta, 3, c.p) == 1
                && lambda > 1
                && lambda < c.n
                && pow(lambda, 3, c.n) == 1,
            "invalid cubic orbit parameters",
        )?;
        for p in [c.g, c.q] {
            require(
                c.scale(Some(p), lambda)
                    == Some(Point {
                        x: mul(beta, p.x, c.p),
                        y: p.y,
                    }),
                "orbit eigenvalue mismatch",
            )?;
        }
        for part in &self.parts {
            for x in part {
                let e = &self.entries[x];
                let next = mul(beta, *x, c.p);
                require(
                    next != *x && part.contains(&next),
                    "short or unclosed cubic orbit",
                )?;
                let image = &self.entries[&next];
                let flip = if image.point.y == e.point.y {
                    false
                } else if image.point.y == (c.p - e.point.y) % c.p {
                    true
                } else {
                    return Err("orbit ordinate mismatch".into());
                };
                let u = mul(lambda, e.u, c.n);
                let v = mul(lambda, e.v, c.n);
                require(
                    image.u == if flip { (c.n - u) % c.n } else { u }
                        && image.v == if flip { (c.n - v) % c.n } else { v },
                    "orbit label propagation mismatch",
                )?;
            }
        }
        Ok(())
    }
    fn parse(v: &Value, c: Curve) -> Result<Self> {
        Self::parse_partial(v, c, false)
    }
    fn parse_partial(v: &Value, c: Curve, allow_empty: bool) -> Result<Self> {
        let entries = v["points"].as_object().ok_or("missing base points")?;
        require(
            !entries.is_empty() && entries.len() <= 32,
            "unsupported base size",
        )?;
        let mut out = BTreeMap::new();
        for (key, v) in entries {
            let x = integer(&v["x"])?;
            let y = integer(&v["y"])?;
            let u = integer(&v["u"])?;
            let w = integer(&v["v"])?;
            require(
                key == &x.to_string() && u < c.n && w < c.n && c.valid(Point { x, y }),
                "invalid canonical labeled point",
            )?;
            require(
                c.add(c.scale(Some(c.g), u), c.scale(Some(c.q), w)) == Some(Point { x, y }),
                "labeled point arithmetic mismatch",
            )?;
            out.insert(
                x,
                Entry {
                    point: Point { x, y },
                    u,
                    v: w,
                },
            );
        }
        let parts: Vec<Vec<u64>> = array(&v["parts"])?
            .iter()
            .map(|p| array(p)?.iter().map(integer).collect())
            .collect::<Result<_>>()?;
        require(
            matches!(parts.len(), 3 | 4) && (allow_empty || parts.iter().all(|p| !p.is_empty())),
            "unsupported base partitions",
        )?;
        let flat: Vec<_> = parts.iter().flatten().copied().collect();
        require(
            flat.len() == out.len()
                && flat.iter().copied().collect::<BTreeSet<_>>() == out.keys().copied().collect(),
            "partition overlap or missing labeled points",
        )?;
        Ok(Self {
            entries: out,
            parts,
        })
    }
    fn member(&self, xs: &[u64], encoding: &str) -> bool {
        let arity = if matches!(encoding, "partitioned3" | "unordered3" | "line3") {
            3
        } else {
            4
        };
        if xs.len() != arity
            || !xs.windows(2).all(|w| w[0] < w[1])
            || xs.iter().any(|x| !self.entries.contains_key(x))
        {
            return false;
        }
        let counts: Vec<_> = self
            .parts
            .iter()
            .map(|p| xs.iter().filter(|x| p.contains(x)).count())
            .collect();
        match encoding {
            "partitioned3" | "expanded4" | "split4" => {
                self.parts.len() == arity && counts.iter().all(|&n| n == 1)
            }
            "unordered3" | "line3" => self.parts.len() == 3,
            "block_symmetric4" => {
                self.parts.len() == 4 && counts[0] + counts[1] == 2 && counts[2] + counts[3] == 2
            }
            _ => false,
        }
    }
    fn signed(&self, c: Curve, xs: &[u64], signs: &[i64], pruned: bool) -> Option<(u64, u64)> {
        if xs.len() != signs.len() || signs.iter().any(|&s| s != 1 && s != -1) {
            return None;
        }
        let (mut sum, mut a, mut b) = (None, 0, 0);
        let mut ys = BTreeSet::new();
        for (&x, &sign) in xs.iter().zip(signs) {
            let e = self.entries.get(&x)?;
            let p = if sign == 1 {
                e.point
            } else {
                c.negate(e.point)
            };
            sum = c.add(sum, Some(p));
            ys.insert(p.y);
            a = (a + if sign == 1 { e.u } else { (c.n - e.u) % c.n }) % c.n;
            b = (b + if sign == 1 { e.v } else { (c.n - e.v) % c.n }) % c.n;
        }
        (sum.is_none() && (!pruned || ys.len() > 1)).then_some((a, b))
    }
    fn relation(&self, c: Curve, xs: &[u64], pruned: bool) -> bool {
        (0..1usize << (xs.len() - 1)).any(|mask| {
            let signs: Vec<_> = (0..xs.len())
                .map(|i| {
                    if i == 0 || mask & (1 << (i - 1)) == 0 {
                        1
                    } else {
                        -1
                    }
                })
                .collect();
            self.signed(c, xs, &signs, pruned).is_some()
        })
    }
}
fn sets(v: &Value) -> Result<BTreeSet<Vec<u64>>> {
    let rows: Vec<Vec<u64>> = array(v)?
        .iter()
        .map(|xs| array(xs)?.iter().map(integer).collect())
        .collect::<Result<_>>()?;
    require(rows.len() <= 32768, "too many reported coordinate sets")?;
    let out: BTreeSet<_> = rows.iter().cloned().collect();
    require(out.len() == rows.len(), "duplicate coordinate sets")?;
    Ok(out)
}
fn oracle(
    c: Curve,
    b: &Base,
    encoding: &str,
    pruned: bool,
    budget: &mut u64,
) -> Result<BTreeSet<Vec<u64>>> {
    let arity = if matches!(encoding, "partitioned3" | "unordered3" | "line3") {
        3
    } else {
        4
    };
    let keys: Vec<_> = b.entries.keys().copied().collect();
    require(keys.len() >= arity, "base smaller than relation arity")?;
    let mut choose = 1u64;
    for k in 0..arity {
        choose = choose
            .checked_mul((keys.len() - k) as u64)
            .ok_or("oracle bound overflow")?
            / (k as u64 + 1);
    }
    let work = choose
        .checked_mul(1 << (arity - 1))
        .ok_or("oracle work overflow")?;
    *budget = budget
        .checked_sub(work)
        .ok_or("independent full-set oracle exceeds cumulative record work cap")?;
    fn visit(
        c: Curve,
        b: &Base,
        filter: (&str, bool),
        keys: &[u64],
        arity: usize,
        xs: &mut Vec<u64>,
        out: &mut BTreeSet<Vec<u64>>,
    ) {
        if xs.len() == arity {
            if b.member(xs, filter.0) && b.relation(c, xs, filter.1) {
                out.insert(xs.clone());
            }
            return;
        }
        for (i, &x) in keys.iter().enumerate() {
            xs.push(x);
            visit(c, b, filter, &keys[i + 1..], arity, xs, out);
            xs.pop();
        }
    }
    let mut out = BTreeSet::new();
    visit(
        c,
        b,
        (encoding, pruned),
        &keys,
        arity,
        &mut Vec::new(),
        &mut out,
    );
    Ok(out)
}

/// Verify the literal compact JSON seal, without reserializing/reordering it.
pub fn record_line(line: &str) -> Result<Value> {
    let r: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    require(
        r["schema"] == "ecbench.record/v1",
        "unsupported record schema",
    )?;
    let id = text(&r["record_id"])?;
    let needle = format!("\"record_id\":\"{id}\"");
    require(
        line.matches(&needle).count() == 1,
        "record seal field is absent/ambiguous",
    )?;
    let empty = line.replacen(&needle, "\"record_id\":\"\"", 1);
    require(
        id == format!("ECR1h{}", &hash(empty.as_bytes())[..12]),
        "record seal mismatch",
    )?;
    Ok(r)
}

/// Recompute all exclusive clock totals. No field here qualifies an L2 clock.
pub fn metrics(r: &Value) -> Result<Value> {
    let version = leads::version(r)?;
    if version == 0 {
        return Ok(json!({"aps":false}));
    }
    let mut times = BTreeMap::new();
    let (mut total, mut online) = (0u64, 0u64);
    for p in array(&r["phases"])? {
        let name = text(&p["name"])?;
        require(PHASES.contains(&name), "unknown APS clock phase")?;
        let ns = integer(&p["wall_ns"])?;
        require(times.insert(name, ns).is_none(), "duplicate clock phase")?;
        total = total.checked_add(ns).ok_or("clock sum overflow")?;
        if name != "setup" {
            online = online.checked_add(ns).ok_or("online sum overflow")?;
        }
    }
    require(times.len() == 6, "APS clock partition incomplete")?;
    require(
        integer(&r["time"]["solve_wall_ns"])? == total
            && integer(&r["online"]["wall_ns"])? == online,
        "cold/online clock sum mismatch",
    )?;
    let op = r["online"]["phases_ns"]
        .as_object()
        .ok_or("missing online phases")?;
    require(op.len() == 5, "online phase count mismatch")?;
    for name in PHASES.iter().skip(1) {
        require(
            integer(&op[*name])? == times[name],
            "online phase identity mismatch",
        )?;
    }
    if !r["time"]["method_call_wall_ns"].is_null() {
        require(
            integer(&r["time"]["method_call_wall_ns"])? >= total,
            "enclosing method clock is smaller than solve",
        )?;
    }
    require(
        r["detail"]["native_work_accounting_complete"] == false
            && r["detail"]["costs_attested"] == false,
        "unsupported APS cost attestation",
    )?;
    let mut diagnostic = BTreeMap::from([
        ("aps_attempts", 0u64),
        ("aps_failed_attempts", 0),
        ("aps_deadline_attempts", 0),
        ("aps_f4_multiplications_uncharged", 0),
        ("aps_root_multiplications_uncharged", 0),
        ("aps_unreported_algebra_attempts", 0),
        ("aps_legacy_root_count_unknown_attempts", 0),
    ]);
    for a in array(&r["detail"]["attempts"])? {
        *diagnostic.get_mut("aps_attempts").unwrap() += 1;
        *diagnostic.get_mut("aps_failed_attempts").unwrap() += u64::from(!a["error"].is_null());
        *diagnostic.get_mut("aps_deadline_attempts").unwrap() +=
            u64::from(a["deadline_exhausted"] == true);
        if !a["algebra"].is_null() {
            let n = integer(&a["algebra"]["f4_field_multiplications"])?;
            let sum = diagnostic
                .get_mut("aps_f4_multiplications_uncharged")
                .unwrap();
            *sum = sum.checked_add(n).ok_or("F4 diagnostic sum overflow")?;
            if a["algebra"]["roots"].is_null() {
                *diagnostic
                    .get_mut("aps_legacy_root_count_unknown_attempts")
                    .unwrap() += 1;
            } else {
                let n = integer(&a["algebra"]["roots"]["root_field_multiplications"])?;
                let sum = diagnostic
                    .get_mut("aps_root_multiplications_uncharged")
                    .unwrap();
                *sum = sum.checked_add(n).ok_or("root diagnostic sum overflow")?;
            }
        } else if a["error_stage"] == "solver_or_visitor" {
            *diagnostic
                .get_mut("aps_unreported_algebra_attempts")
                .unwrap() += 1;
        }
    }
    for (name, expected) in &diagnostic {
        require(
            integer(&r["counters"][*name])? == *expected,
            "aggregate APS diagnostic differs from retained attempts",
        )?;
    }
    if version == 3 {
        leads::metrics(r)?;
    }
    Ok(
        json!({"aps":true,"cold_ns":total,"online_ns":online,"setup_ns":times["setup"],"source_diagnostics_recomputed":diagnostic,"execution_attested":false,"wall_qualified":false}),
    )
}

fn configuration(params: &Value) -> Result<()> {
    let choice = |name, options: &[&str]| {
        require(
            options.contains(&text(&params[name])?),
            "unsupported APS parameter choice",
        )
    };
    choice(
        "encoding",
        &[
            "partitioned3",
            "unordered3",
            "line3",
            "expanded4",
            "split4",
            "block_symmetric4",
        ],
    )?;
    choice("roots", &["exact", "legacy"])?;
    choice("stop", &["exhaustive", "first_informative"])?;
    choice("labels", &["random", "guaranteed"])?;
    choice("orbit", &["none", "cubic"])?;
    choice("prune", &["false", "true"])?;
    for (name, low, high) in [
        ("subset_size", 1, 4),
        ("attempts", 1, 128),
        ("degree", 1, 24),
        ("budget_ms", 1, 60_000),
        ("sampling_draws", 1, 10_000),
        ("beta", 0, 65_520),
        ("lambda", 0, 1 << 20),
    ] {
        text(&params[name])?;
        require(
            (low..=high).contains(&integer(&params[name])?),
            "APS parameter outside supported bounds",
        )?;
    }
    require(
        params["roots"] != "legacy" || params["stop"] == "exhaustive",
        "legacy roots require exhaustive stopping",
    )?;
    if params["orbit"] == "none" {
        require(
            integer(&params["beta"])? == 0
                && integer(&params["lambda"])? == 0
                && params["prune"] == "false",
            "non-orbit configuration contains orbit parameters",
        )?;
    } else {
        require(
            matches!(
                text(&params["encoding"])?,
                "partitioned3" | "unordered3" | "line3"
            ) && integer(&params["subset_size"])? <= 3,
            "cubic orbit configuration exceeds supported arity or size",
        )?;
    }
    require(
        params["prune"] == "false" || params["encoding"] == "line3",
        "pruning requires line encoding",
    )
}

pub fn mathematical(r: &Value, cap: u64) -> Result<Value> {
    require(
        cap > 0 && cap <= MAX_ORACLE_CANDIDATES,
        "invalid independent oracle work cap",
    )?;
    let version = leads::version(r)?;
    if version == 0 {
        return Ok(json!({"aps":false,"witness_checked":false}));
    }
    let profile = leads::configuration(&r["method"]["params"], version)?;
    if !r["detail"]["target_error"].is_null() {
        require(
            array(&r["detail"]["attempts"])?.is_empty()
                && r["outcome"]["status"] != "verified"
                && integer(&r["counters"]["aps_attempts"])? == 0,
            "target-error execution contains attempts or success",
        )?;
        return Ok(
            json!({"aps":true,"target_error_retained":true,"witnesses_checked":0,"execution_attested":false}),
        );
    }
    let c = Curve::from_record(r)?;
    let params = &r["method"]["params"];
    let raw_encoding = text(&params["encoding"])?;
    let enc = if version == 3 && raw_encoding == "line_chart3" {
        "line3"
    } else {
        raw_encoding
    };
    require(
        matches!(
            enc,
            "partitioned3" | "unordered3" | "line3" | "expanded4" | "split4" | "block_symmetric4"
        ),
        "unsupported APS encoding",
    )?;
    let pruned = params["prune"] == "true";
    let orbit = params["orbit"] == "cubic";
    let rows = array(&r["detail"]["attempts"])?;
    require(
        !pruned || (orbit && enc == "line3"),
        "pruning requires a certified cubic line arm",
    )?;
    require(
        rows.len() <= 128 && rows.len() as u64 <= integer(&params["attempts"])?,
        "too many attempts",
    )?;
    let arity = if matches!(enc, "partitioned3" | "unordered3" | "line3") {
        3
    } else {
        4
    };
    let size = if version == 3 {
        profile[0] as u64
    } else {
        integer(&params["subset_size"])?
    };
    require((1..=4).contains(&size), "unsupported subset size")?;
    let mut complete = 0;
    let mut partial = 0;
    let mut witnesses = 0;
    let mut in_budget = 0;
    let mut reported = 0;
    let mut remaining = cap;
    for (i, a) in rows.iter().enumerate() {
        require(
            a["deadline_exhausted"].is_boolean() && a["independent_witness_verified"].is_boolean(),
            "missing attempt status flags",
        )?;
        require(
            integer(&a["index"])? == i as u64,
            "attempt indices are not consecutive",
        )?;
        require(
            integer(&a["seed"])? == integer(&r["algorithm_seed"])?.wrapping_add(i as u64),
            "attempt seed mismatch",
        )?;
        if a["base"].is_null() {
            require(
                a["recovery"].is_null()
                    && a["system"].is_null()
                    && matches!(a["error_stage"].as_str(), Some("sampling" | "orbit")),
                "missing base without pre-transcript failure",
            )?;
            partial += 1;
            continue;
        }
        let sampling_partial = version == 3
            && a["lead"]["sampling_complete"] == false
            && a["error_stage"] == "sampling";
        let base = Base::parse_partial(&a["base"], c, sampling_partial)?;
        let skipped = if version == 3 {
            leads::attempt(r, i, c, &base, &profile)?
        } else {
            None
        };
        if sampling_partial {
            partial += 1;
            continue;
        }
        let orbit_abort = orbit && a["error_stage"] == "orbit" && !a["error"].is_null();
        require(
            base.parts.len() == arity
                && (version == 3
                    || base.parts.iter().all(|part| {
                        part.len() as u64 == size * if orbit && !orbit_abort { 3 } else { 1 }
                    })),
            "base partition size differs from method",
        )?;
        if orbit && !orbit_abort {
            base.certify_orbit(c, integer(&params["beta"])?, integer(&params["lambda"])?)?;
        }
        if orbit_abort {
            require(
                a["algebra"].is_null()
                    && a["recovery"].is_null()
                    && array(&a["visited_coordinate_sets"])?.is_empty(),
                "orbit abort contains solver output",
            )?;
            partial += 1;
            continue;
        }
        if !a["system"].is_null() {
            require(
                a["system_sha256"] == hash(&canonical(&a["system"])?),
                "actual equation digest mismatch",
            )?;
            require(
                integer(&a["system"]["p"])? == c.p,
                "equation field mismatch",
            )?;
        }
        let visited = sets(&a["visited_coordinate_sets"])?;
        for xs in &visited {
            require(
                base.member(xs, enc) && base.relation(c, xs, pruned),
                "reported coordinate set is not an allowed relation",
            )?;
            require(
                skipped
                    .as_ref()
                    .is_none_or(|old| xs.iter().any(|x| !old.entries.contains_key(x))),
                "old coordinate set re-emitted in a frontier-only plan",
            )?;
        }
        reported += visited.len();
        require(
            reported <= 32768,
            "total reported coordinate-set cap exceeded",
        )?;
        if !a["algebra"].is_null() {
            require(
                a["algebra"]["complete"].is_boolean() && a["algebra"]["timed_out"].is_boolean(),
                "missing algebra completeness/deadline flags",
            )?;
            let emitted = sets(&a["algebra"]["coordinate_sets"])?;
            require(
                emitted == visited,
                "visitor and algebra coordinate sets disagree",
            )?;
            if a["algebra"]["complete"] == true {
                require(
                    a["algebra"]["timed_out"] == false,
                    "timed-out attempt claims completeness",
                )?;
                let mut expected = oracle(c, &base, enc, pruned, &mut remaining)?;
                if let Some(old) = &skipped {
                    for xs in oracle(c, old, enc, pruned, &mut remaining)? {
                        expected.remove(&xs);
                    }
                }
                require(
                    emitted == expected,
                    "complete root set differs from independent point enumeration",
                )?;
                complete += 1;
            } else {
                partial += 1;
            }
        } else {
            partial += 1;
        }
        if !a["recovery"].is_null() {
            let recovery = &a["recovery"];
            let xs: Vec<_> = array(&recovery["coordinates"])?
                .iter()
                .map(integer)
                .collect::<Result<_>>()?;
            require(
                visited.contains(&xs),
                "recovery does not belong to an emitted coordinate set",
            )?;
            let signs: Vec<_> = array(&recovery["signs"])?
                .iter()
                .map(|v| v.as_i64().ok_or_else(|| "noninteger sign".to_string()))
                .collect::<Result<_>>()?;
            let (aa, bb) = base
                .signed(c, &xs, &signs, pruned)
                .ok_or("recovery is not a signed point relation")?;
            let k = integer(&recovery["logarithm"])?;
            require(
                bb != 0
                    && aa == integer(&recovery["a"])?
                    && bb == integer(&recovery["b"])?
                    && k < c.n
                    && (aa + mul(bb, k, c.n)).is_multiple_of(c.n)
                    && c.scale(Some(c.g), k) == Some(c.q),
                "invalid modular/scalar recovery",
            )?;
            require(
                a["independent_witness_verified"] == true,
                "recovery lacks producer witness outcome",
            )?;
            if a["outcome"] == "verified" {
                in_budget += 1;
                require(
                    a["deadline_exhausted"] == false,
                    "late witness counted as a success",
                )?;
                require(
                    integer(&r["outcome"]["recovered"])? == k,
                    "record scalar does not match witness",
                )?;
            } else {
                require(
                    a["outcome"] == "verified_after_deadline" && a["deadline_exhausted"] == true,
                    "witness/outcome mismatch",
                )?;
            }
            witnesses += 1;
            require(
                i + 1 == rows.len(),
                "attempts continued after an accepted or late witness",
            )?;
        } else {
            require(
                a["independent_witness_verified"] == false
                    && a["outcome"] != "verified"
                    && a["outcome"] != "verified_after_deadline",
                "success without witness",
            )?;
        }
    }
    require(
        integer(&r["counters"]["aps_attempts"])? == rows.len() as u64,
        "attempt counter mismatch",
    )?;
    if r["outcome"]["status"] == "verified" {
        require(
            in_budget == 1 && r["detail"]["outcome"] == "verified",
            "record success is not backed by exactly one in-budget witness",
        )?;
        require(
            scalar_record(r)?["matches_target"] == true,
            "record scalar failed the separate arithmetic checker",
        )?;
    }
    Ok(
        json!({"aps":true,"attempts":rows.len(),"coordinate_sets_checked":reported,"complete_attempts_checked":complete,"partial_attempts_retained":partial,"witnesses_checked":witnesses,"in_budget_witnesses_checked":in_budget,"execution_attested":false,"sampling_history_attested":false,"costs_attested":false}),
    )
}
