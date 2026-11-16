//! Offline signed-stream replay. Does not certify the walk's group-label invariant,
//! PRNG history, source custody, or source scalar verification. Never meter online.
use five_leads_study::three_leads::{Divisor, Ring, SignedGraph};
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

// Reject duplicate keys at every nesting depth rather than accepting last-wins JSON.
struct Unique(Value);
impl<'de> Deserialize<'de> for Unique {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Unique;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("integer-only JSON with unique object keys")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<Unique, E> {
                Ok(Unique(json!(v)))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut v = vec![];
                while let Some(x) = a.next_element::<Unique>()? {
                    v.push(x.0);
                }
                Ok(Unique(Value::Array(v)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Unique, A::Error> {
                let mut v = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if v.contains_key(&k) {
                        return Err(de::Error::custom("duplicate JSON key"));
                    }
                    v.insert(k, a.next_value::<Unique>()?.0);
                }
                Ok(Unique(Value::Object(v)))
            }
        }
        d.deserialize_any(V)
    }
}
/// Parse the independently supplied header using the same strict JSON key policy.
pub fn parse_expected_header(input: &[u8]) -> Result<Value, String> {
    require(input.len() <= 65536, "header byte limit")?;
    serde_json::from_slice::<Unique>(input)
        .map(|v| v.0)
        .map_err(|e| e.to_string())
}

#[derive(Clone, Copy)]
pub struct Limits {
    pub bytes: usize,
    pub events: usize,
    pub nodes: usize,
    pub prefix_cells: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            bytes: 8_000_000,
            events: 16_384,
            nodes: 2048,
            prefix_cells: 20_000_000,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Prefix {
    pub sequence: usize,
    pub attempt: u64,
    pub rows: usize,
    pub nuisance_columns: usize,
    pub nuisance_rank: usize,
    pub nuisance_and_target_rank: usize,
    pub full_rank: usize,
    pub consistent: bool,
    pub target_determined: bool,
}
#[derive(Debug, Serialize)]
pub struct Replay {
    pub prefixes: Vec<Prefix>,
    pub candidates: u64,
    pub duplicates: u64,
    pub recovered: Option<u64>,
    pub complete: bool,
    pub geometric_label_invariant_verified: bool,
}
fn require(b: bool, msg: &str) -> Result<(), String> {
    if b {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn keys(v: &Value, ks: &[&str]) -> Result<(), String> {
    let o = v.as_object().ok_or("object required")?;
    require(
        o.len() == ks.len() && ks.iter().all(|k| o.contains_key(*k)),
        "unexpected or missing fields",
    )
}
fn num(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64()
        .ok_or_else(|| format!("unsigned {k} required"))
}
fn boolean(v: &Value, k: &str) -> Result<bool, String> {
    v[k].as_bool()
        .ok_or_else(|| format!("boolean {k} required"))
}
fn opt(v: &Value, k: &str, r: u64) -> Result<Option<u64>, String> {
    if v[k].is_null() {
        Ok(None)
    } else {
        let x = num(v, k)?;
        require(x < r, "scalar outside field")?;
        Ok(Some(x))
    }
}
fn neg(x: u64, p: u64) -> u64 {
    if x == 0 {
        0
    } else {
        p - x
    }
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn eval(c: &[u64], x: u64, p: u64) -> u64 {
    c.iter().rev().fold(0, |a, b| (mul(a, x, p) + b) % p)
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n > 0 {
        if n & 1 == 1 {
            b = mul(a, b, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    b
}

fn divisor(v: &Value, p: u64) -> Result<Divisor, String> {
    keys(v, &["degree", "u", "v"])?;
    let d: Divisor = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    require(
        d.degree <= 2 && d.u.iter().chain(&d.v).all(|x| *x < p),
        "divisor domain",
    )?;
    require(
        d.u[d.degree] == 1
            && d.u[d.degree + 1..].iter().all(|x| *x == 0)
            && d.v[d.degree..].iter().all(|x| *x == 0),
        "noncanonical divisor",
    )?;
    Ok(d)
}
fn canon(d: Divisor, p: u64) -> Divisor {
    let mut e = d;
    for x in &mut e.v {
        *x = neg(*x, p);
    }
    d.min(e)
}

/// `expected_header` must come from independently pinned input/custody, not the
/// first row of the same untrusted stream. Limits bound input and diagnostic cells,
/// not total allocator/RSS overhead. Returns no partial success on any failure.
pub fn replay_jsonl(
    input: &[u8],
    expected_header: &Value,
    limits: Limits,
) -> Result<Replay, String> {
    require(
        input.len() <= limits.bytes && limits.events > 0 && limits.nodes > 0,
        "input limit",
    )?;
    let text = std::str::from_utf8(input).map_err(|e| e.to_string())?;
    let mut records = vec![];
    for line in text.lines() {
        require(
            records.len() < limits.events && !line.trim().is_empty(),
            "event limit or blank record",
        )?;
        let v = serde_json::from_str::<Unique>(line)
            .map_err(|e| e.to_string())?
            .0;
        keys(&v, &["sequence", "event"])?;
        require(
            num(&v, "sequence")? == records.len() as u64,
            "sequence mismatch",
        )?;
        records.push(v["event"].clone());
    }
    require(
        records.len() >= 2 && records.first() == Some(expected_header),
        "header identity mismatch or truncated stream",
    )?;
    let h = expected_header;
    keys(
        h,
        &[
            "kind",
            "schema",
            "convention",
            "zero_node",
            "p",
            "r",
            "order",
            "cofactor",
            "source_field",
            "source_binary",
            "source_coefficients",
            "generator",
            "target",
            "transferred_generator",
            "transferred_target",
            "polynomial",
            "seed",
            "candidate_cap",
            "projective",
            "rank_policy",
            "geometric_relations_independently_verified",
        ],
    )?;
    require(
        h["kind"] == "header"
            && h["schema"] == "signed-collector-events/v1"
            && h["convention"] == "su_xu_plus_sv_xv_eq_a_plus_bk"
            && h["zero_node"] == 0
            && h["geometric_relations_independently_verified"] == false,
        "header convention",
    )?;
    let p = num(h, "p")?;
    let r = num(h, "r")?;
    let cap = num(h, "candidate_cap")?;
    require(
        p > 2 && r > 2 && p <= u64::MAX / 2 && r <= u64::MAX / 2,
        "native arithmetic no-overflow domain",
    )?;
    pencil_target_information::information(r, 0, &[])?; // primality before Ring's assertion
    pencil_target_information::information(p, 0, &[])?;
    let rank = boolean(h, "rank_policy")?;
    let polynomial: Vec<u64> =
        serde_json::from_value(h["polynomial"].clone()).map_err(|e| e.to_string())?;
    require(
        polynomial.len() == 6 && polynomial.iter().all(|x| *x < p),
        "curve polynomial domain",
    )?;
    let mut graph = SignedGraph::new(Ring::new(r));
    let mut xs = BTreeMap::<u64, usize>::new();
    let mut node_x = vec![0u64];
    let mut allocated = vec![];
    let mut first = BTreeMap::<Divisor, u64>::new();
    let mut rows: Vec<Vec<u64>> = vec![];
    let mut prefixes = vec![];
    let mut cells = 0usize;
    let mut candidate: Option<(u64, u64, u64, Divisor)> = None;
    let (mut attempts, mut duplicates, mut continuation) = (0u64, 0u64, 0u64);
    let mut group = BTreeMap::<u64, bool>::new();
    let mut group_closed = true;
    let mut must_restart = false;
    let mut walk_restart_pending = false;
    let mut jump_indices = BTreeSet::new();
    let mut recovered = None;
    let mut terminal = false;
    for (sequence, e) in records.iter().enumerate().skip(1) {
        require(!terminal, "post-terminal record")?;
        let kind = e["kind"].as_str().ok_or("event kind")?;
        require(
            !must_restart || (kind == "restart" && e["reason"] == "chosen_duplicate"),
            "missing required chosen-duplicate restart",
        )?;
        require(
            !walk_restart_pending || kind == "candidate",
            "walk restart must precede a candidate",
        )?;
        require(
            recovered.is_none() || kind == "terminal",
            "post-recovery record",
        )?;
        match kind {
            "candidate" => {
                keys(
                    e,
                    &[
                        "kind",
                        "attempt",
                        "continuation",
                        "jump_index",
                        "divisor",
                        "a",
                        "b",
                    ],
                )?;
                require(candidate.is_none(), "candidate missing disposition")?;
                let next = num(e, "continuation")?;
                if next == continuation + 1 {
                    require(group_closed, "missing choice")?;
                    group.clear();
                    jump_indices.clear();
                    walk_restart_pending = false;
                    group_closed = false;
                    continuation = next;
                } else {
                    require(next == continuation && !group_closed, "continuation order")?;
                }
                require(group.len() < 2, "more than two candidates")?;
                attempts = attempts.checked_add(1).ok_or("attempt overflow")?;
                require(
                    num(e, "attempt")? == attempts && attempts <= cap && num(e, "jump_index")? < 16,
                    "candidate identity or cap",
                )?;
                require(
                    jump_indices.insert(num(e, "jump_index")?),
                    "repeated jump index in continuation",
                )?;
                let a = num(e, "a")?;
                let b = num(e, "b")?;
                require(a < r && b < r, "candidate label domain")?;
                candidate = Some((attempts, a, b, divisor(&e["divisor"], p)?));
            }
            "node" => {
                keys(e, &["kind", "attempt", "x", "node"])?;
                require(
                    candidate.is_some() && num(e, "attempt")? == attempts,
                    "node without active candidate",
                )?;
                let x = num(e, "x")?;
                let d = candidate.as_ref().ok_or("node candidate")?.3;
                require(
                    x < p && allocated.len() < d.degree && eval(&d.u[..=d.degree], x, p) == 0,
                    "allocated node is not a divisor root",
                )?;
                require(
                    x < p && !xs.contains_key(&x) && graph.nodes() < limits.nodes,
                    "node identity or limit",
                )?;
                require(num(e, "node")? == graph.nodes() as u64, "node order")?;
                let n = graph.node();
                xs.insert(x, n);
                node_x.push(x);
                allocated.push(n);
                let width = graph.nodes() + 1;
                require(
                    rows.len().checked_mul(width).ok_or("cell overflow")? <= limits.prefix_cells,
                    "retained cell limit",
                )?;
                for row in &mut rows {
                    row.insert(row.len() - 2, 0);
                }
            }
            "edge" => {
                keys(
                    e,
                    &[
                        "kind",
                        "attempt",
                        "nodes",
                        "signs",
                        "a",
                        "b",
                        "canonical",
                        "node_count",
                        "rank_score",
                        "recovered",
                        "cycles",
                        "negative_cycles",
                        "zero_coefficient",
                    ],
                )?;
                let (attempt, a, b, d) = candidate.take().ok_or("edge without candidate")?;
                require(
                    num(e, "attempt")? == attempt
                        && num(e, "a")? == a
                        && num(e, "b")? == b
                        && d.degree > 0,
                    "edge labels",
                )?;
                let canonical = divisor(&e["canonical"], p)?;
                require(
                    canonical == canon(d, p) && !first.contains_key(&canonical),
                    "canonical edge identity or duplicate",
                )?;
                let ns: Vec<usize> =
                    serde_json::from_value(e["nodes"].clone()).map_err(|x| x.to_string())?;
                let signs: Vec<i8> =
                    serde_json::from_value(e["signs"].clone()).map_err(|x| x.to_string())?;
                require(
                    ns.len() == 2
                        && signs.len() == 2
                        && ns.iter().all(|x| *x < graph.nodes())
                        && signs.iter().all(|s| *s == 1 || *s == -1),
                    "edge node/sign domain",
                )?;
                require(num(e, "node_count")? == graph.nodes() as u64, "node count")?;
                require(
                    allocated.iter().all(|n| ns.contains(n)),
                    "orphan allocated node",
                )?;
                if d.degree == 1 {
                    require(
                        ns[0] != 0
                            && ns[1] == 0
                            && signs[1] == 1
                            && node_x[ns[0]] == neg(d.u[0], p),
                        "degree-one edge roots",
                    )?;
                } else {
                    require(ns[0] != 0 && ns[1] != 0, "degree-two zero-node padding")?;
                    let x = node_x[ns[0]];
                    let y = node_x[ns[1]];
                    require(
                        (x + y) % p == neg(d.u[1], p) && mul(x, y, p) == d.u[0],
                        "degree-two edge roots",
                    )?;
                }
                for i in 0..d.degree {
                    let x = node_x[ns[i]];
                    let y = eval(&d.v[..d.degree], x, p);
                    require(
                        signs[i] == if y <= p / 2 { 1 } else { -1 },
                        "point sign correspondence",
                    )?;
                    require(
                        mul(y, y, p) == eval(&polynomial, x, p),
                        "point not on destination curve",
                    )?;
                }
                allocated.clear();
                if rank {
                    require(
                        num(e, "rank_score")?
                            == graph.target_rank_score(ns[0], signs[0], ns[1], signs[1], b) as u64,
                        "rank score",
                    )?;
                } else {
                    require(e["rank_score"].is_null(), "unexpected rank score")?;
                }
                recovered = graph.edge(ns[0], signs[0], ns[1], signs[1], a, b);
                require(
                    opt(e, "recovered", r)? == recovered
                        && num(e, "cycles")? == graph.cycles
                        && num(e, "negative_cycles")? == graph.negative_cycles
                        && num(e, "zero_coefficient")? == graph.zero_coefficient,
                    "graph result or counters",
                )?;
                let n = graph.nodes() - 1;
                let width = n + 2;
                cells = cells
                    .checked_add((rows.len() + 1).checked_mul(width).ok_or("cell overflow")?)
                    .ok_or("cell overflow")?;
                require(cells <= limits.prefix_cells, "prefix cell limit")?;
                let mut row = vec![0; width];
                for (node, sign) in ns.into_iter().zip(signs) {
                    if node != 0 {
                        row[node - 1] = (row[node - 1] + if sign == 1 { 1 } else { r - 1 }) % r;
                    }
                }
                row[n] = neg(b, r);
                row[n + 1] = neg(a, r);
                rows.push(row);
                let info = pencil_target_information::information(r, n, &rows)?;
                require(
                    info.consistent && info.target_determined == recovered.is_some(),
                    "prefix/native information disagreement",
                )?;
                prefixes.push(Prefix {
                    sequence,
                    attempt,
                    rows: rows.len(),
                    nuisance_columns: n,
                    nuisance_rank: info.nuisance_rank,
                    nuisance_and_target_rank: info.nuisance_and_target_rank,
                    full_rank: info.full_rank,
                    consistent: info.consistent,
                    target_determined: info.target_determined,
                });
                first.insert(canonical, attempt);
                group.insert(attempt, false);
            }
            "duplicate" | "identity" | "nonsplit" => {
                let (attempt, a, b, d) = candidate.take().ok_or("disposition without candidate")?;
                require(allocated.is_empty(), "unused node allocation")?;
                require(num(e, "attempt")? == attempt, "disposition attempt")?;
                if kind == "duplicate" {
                    keys(e, &["kind", "attempt", "canonical", "first_attempt"])?;
                    let c = divisor(&e["canonical"], p)?;
                    require(
                        c == canon(d, p) && first.get(&c) == Some(&num(e, "first_attempt")?),
                        "duplicate reference",
                    )?;
                    duplicates += 1;
                } else if kind == "identity" {
                    keys(e, &["kind", "attempt", "recovered"])?;
                    require(d == Divisor::O, "nonidentity disposition")?;
                    let ring = Ring::new(r);
                    recovered = if b == 0 {
                        require(a == 0, "inconsistent identity labels")?;
                        None
                    } else {
                        Some(ring.mul(ring.neg(a), ring.inv(b)))
                    };
                    require(opt(e, "recovered", r)? == recovered, "identity result")?;
                } else {
                    keys(e, &["kind", "attempt"])?;
                    require(d.degree == 2, "nonsplit degree")?;
                    let disc = (mul(d.u[1], d.u[1], p) + neg(mul(4, d.u[0], p), p)) % p;
                    require(
                        pow(disc, (p - 1) / 2, p) == p - 1,
                        "false nonsplit disposition",
                    )?;
                }
                group.insert(attempt, kind == "duplicate");
            }
            "choice" => {
                keys(e, &["kind", "continuation", "selected_attempt", "repeated"])?;
                require(
                    candidate.is_none()
                        && !group_closed
                        && !group.is_empty()
                        && num(e, "continuation")? == continuation,
                    "choice order",
                )?;
                require(
                    group.len() == 2 || attempts == cap,
                    "choice before both candidates or cap",
                )?;
                let selected = num(e, "selected_attempt")?;
                require(
                    group.get(&selected) == Some(&boolean(e, "repeated")?),
                    "choice reference",
                )?;
                group_closed = true;
                must_restart = boolean(e, "repeated")?;
            }
            "restart" => {
                keys(
                    e,
                    &[
                        "kind",
                        "reason",
                        "continuation",
                        "after_attempt",
                        "divisor",
                        "a",
                        "b",
                    ],
                )?;
                require(
                    candidate.is_none() && group_closed && num(e, "after_attempt")? == attempts,
                    "restart order",
                )?;
                let c = num(e, "continuation")?;
                require(
                    (e["reason"] == "walk_duplicate" && c == continuation + 1)
                        || (e["reason"] == "chosen_duplicate" && c == continuation),
                    "restart reason or continuation",
                )?;
                if e["reason"] == "chosen_duplicate" {
                    require(must_restart, "unsolicited chosen-duplicate restart")?;
                    must_restart = false;
                } else {
                    require(
                        !must_restart && attempts < cap,
                        "walk restart outside active continuation",
                    )?;
                    walk_restart_pending = true;
                }
                require(num(e, "a")? < r && num(e, "b")? < r, "restart label domain")?;
                divisor(&e["divisor"], p)?;
            }
            "terminal" => {
                keys(
                    e,
                    &[
                        "kind",
                        "attempts",
                        "continuations",
                        "split_relations",
                        "duplicates",
                        "node_count",
                        "recovered",
                        "exhausted",
                        "source_verified",
                    ],
                )?;
                require(
                    candidate.is_none() && (group_closed || recovered.is_some()),
                    "incomplete candidate group",
                )?;
                require(
                    num(e, "attempts")? == attempts
                        && num(e, "continuations")? == continuation
                        && num(e, "split_relations")? == rows.len() as u64
                        && num(e, "duplicates")? == duplicates
                        && num(e, "node_count")? == graph.nodes() as u64,
                    "terminal counters",
                )?;
                require(
                    opt(e, "recovered", r)? == recovered
                        && boolean(e, "exhausted")? == recovered.is_none()
                        && e["source_verified"] == false,
                    "terminal result",
                )?;
                require(recovered.is_some() || attempts == cap, "early terminal")?;
                terminal = true;
            }
            _ => return Err("unknown event".into()),
        }
    }
    require(terminal, "missing terminal")?;
    Ok(Replay {
        prefixes,
        candidates: attempts,
        duplicates,
        recovered,
        complete: true,
        geometric_label_invariant_verified: false,
    })
}
