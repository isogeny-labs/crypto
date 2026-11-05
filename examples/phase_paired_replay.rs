//! Independent replay of the native paired fixed-budget receipts.
//!
//! Protocol: research/phase_paired_native_20261026/PROTOCOL.md
//!
//! It shares no encoder or solver code with `phase_paired_fixed_budget.rs`.
//! From the hash-pinned frozen manifest alone it rebuilds the curve (the crate
//! group law and field, `F2mElement`, with no `u64` field code of its own),
//! the discrete-log table of the odd subgroup, the Frobenius orbits of the
//! representatives and, by exhaustive pair matching on discrete logarithms,
//! which targets are decomposable in each arm's search space. Then it
//! re-verifies every receipt:
//!
//! * `verified`: four points on the curve, distinct abscissae, in the odd
//!   subgroup, in the arm's pool (arm A: representatives at phase 0), signed
//!   sum equal to the target by the group law, coefficients `+-lambda^k`
//!   recomputed, `log R = sum c_j log V_j (mod r)`, solver summands consistent.
//! * `unsat`: the target is not decomposable in that arm's pool.
//! * `budget`: the conflict count reached the preregistered budget.
//! * the receipt order is the preregistered ABBA schedule.
//!
//! Rank is recomputed by dense Gaussian elimination mod r written here, not by
//! the crate's `U64RankTracker`. The output is canonical JSON of the receipts'
//! contents and contains no value of this host, so `cmp` against the committed
//! replay output is a regression test.
//!
//! Receipts of the CryptoMiniSat follow-up (schema `phase_paired_cms/receipt/v1`,
//! PROTOCOL_ADDENDUM_2.md) are replayed by the same checks plus the pinned binary,
//! the CPU cap, the stage rule, the controls and the decision rule; `--native`
//! names the native receipts for the side-by-side tables.
//!
//! usage: phase_paired_replay RECEIPTS.jsonl OUT.json [--manifest M] [--sealed S]

use crypto_lib::binary_ecc::curve::{point_add, point_neg, BinaryCurve, BinaryPoint};
use crypto_lib::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crypto_lib::hash::sha256::sha256;
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

const FROZEN_MANIFEST_SHA: &str =
    "8cb7f7989e5ea35656b34e7ba8bf7104e0c081277b8dbbd6b0f7507369b01231";
const DEFAULT_MANIFEST: &str = "research/phase_paired_native_20261026/fixtures/manifest.json";

fn hexs(b: &[u8]) -> String {
    hex::encode(b)
}

fn unhex(s: &str) -> Result<u64, String> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map_err(|e| format!("hex {s}: {e}"))
}

type P = Option<(u64, u64)>;

/// The frozen instance, rebuilt with the crate's field and group law only.
struct World {
    n: u32,
    r: u64,
    curve: BinaryCurve,
    log: HashMap<(u64, u64), u64>,
    lam: u64,
    reps: Vec<(u64, u64)>,
    rep_logs: Vec<u64>,
    orbit: HashMap<u64, (usize, u32)>,
    targets: Vec<(String, String, (u64, u64))>,
    sha: String,
}

impl World {
    fn fe(&self, x: u64) -> F2mElement {
        F2mElement::from_words(&[x], self.n)
    }
    fn bp(&self, p: P) -> BinaryPoint {
        match p {
            None => BinaryPoint::Infinity,
            Some((x, y)) => BinaryPoint::Affine {
                x: self.fe(x),
                y: self.fe(y),
            },
        }
    }
    fn back(b: &BinaryPoint) -> P {
        match b {
            BinaryPoint::Infinity => None,
            BinaryPoint::Affine { x, y } => Some((x.raw_bits()[0], y.raw_bits()[0])),
        }
    }
    fn add(&self, p: P, q: P) -> P {
        Self::back(&point_add(&self.curve, &self.bp(p), &self.bp(q)))
    }
    fn neg(&self, p: P) -> P {
        Self::back(&point_neg(&self.bp(p)))
    }
    fn on(&self, p: P) -> bool {
        self.curve.is_on_curve(&self.bp(p))
    }
    fn sqr(&self, x: u64) -> u64 {
        self.fe(x).square(&self.curve.irreducible).raw_bits()[0]
    }
    fn frob(&self, p: (u64, u64)) -> (u64, u64) {
        (self.sqr(p.0), self.sqr(p.1))
    }
    fn phase_point(&self, rep: usize, k: u32) -> (u64, u64) {
        let mut p = self.reps[rep];
        for _ in 0..k {
            p = self.frob(p);
        }
        p
    }
    fn lam_pow(&self, k: u32) -> u64 {
        let mut v = 1u128;
        for _ in 0..k {
            v = v * self.lam as u128 % self.r as u128;
        }
        v as u64
    }

    fn from_manifest(text: &str) -> Result<World, String> {
        let m: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let c = &m["curve"];
        let n = c["n"].as_u64().ok_or("n")? as u32;
        let modulus = unhex(c["modulus"].as_str().ok_or("modulus")?)?;
        let r = c["odd_subgroup_order"].as_u64().ok_or("r")?;
        let low_terms: Vec<u32> = (0..n).filter(|b| (modulus >> b) & 1 == 1).collect();
        let curve = BinaryCurve {
            m: n,
            irreducible: IrreduciblePoly {
                degree: n,
                low_terms,
            },
            a: F2mElement::zero(n),
            b: F2mElement::one(n),
            generator: BinaryPoint::Infinity,
            order: BigUint::from(r),
            cofactor: BigUint::from(4u32),
        };
        let mut w = World {
            n,
            r,
            curve,
            log: HashMap::new(),
            lam: 0,
            reps: Vec::new(),
            rep_logs: Vec::new(),
            orbit: HashMap::new(),
            targets: Vec::new(),
            sha: hexs(&sha256(text.as_bytes())),
        };
        let g = (
            unhex(c["generator"][0].as_str().ok_or("gx")?)?,
            unhex(c["generator"][1].as_str().ok_or("gy")?)?,
        );
        if !w.on(Some(g)) {
            return Err("generator off curve".into());
        }
        let mut p: P = None;
        for k in 0..r {
            if let Some(q) = p {
                if w.log.insert(q, k).is_some() {
                    return Err("generator has order below r".into());
                }
            }
            p = w.add(p, Some(g));
        }
        if p.is_some() {
            return Err("r*g is not the identity".into());
        }
        w.lam = *w.log.get(&w.frob(g)).ok_or("pi(g) not in <g>")?;
        let t = c["trace_t"].as_i64().ok_or("trace")? as i128;
        if (w.lam as i128 * w.lam as i128 - t * w.lam as i128 + 2).rem_euclid(r as i128) != 0 {
            return Err("lambda equation fails".into());
        }
        for rep in m["factor_space"]["representatives"]
            .as_array()
            .ok_or("reps")?
        {
            let pt = (
                unhex(rep["point"][0].as_str().ok_or("rx")?)?,
                unhex(rep["point"][1].as_str().ok_or("ry")?)?,
            );
            if !w.on(Some(pt)) || !w.log.contains_key(&pt) {
                return Err("representative is not a point of H".into());
            }
            w.reps.push(pt);
        }
        w.rep_logs = w.reps.iter().map(|p| w.log[p]).collect();
        for j in 0..w.reps.len() {
            let mut x = w.reps[j].0;
            for k in 0..n {
                if w.orbit.insert(x, (j, k)).is_some() {
                    return Err("orbits overlap".into());
                }
                x = w.sqr(x);
            }
        }
        for key in ["a", "b", "c"] {
            for t in m["target_sets"][key]["targets"]
                .as_array()
                .ok_or("targets")?
            {
                let rp = (
                    unhex(t["R"][0].as_str().ok_or("Rx")?)?,
                    unhex(t["R"][1].as_str().ok_or("Ry")?)?,
                );
                if !w.log.contains_key(&rp) {
                    return Err("target not in H".into());
                }
                w.targets.push((
                    t["id"].as_str().ok_or("id")?.to_string(),
                    key.to_string(),
                    rp,
                ));
            }
        }
        Ok(w)
    }

    /// The pool of one point per x-class: representatives at phase 0
    /// (`phases = false`) or all Frobenius images.
    fn pool(&self, phases: bool) -> Vec<(u64, u64)> {
        let ph = if phases { self.n } else { 1 };
        let mut pts = Vec::new();
        for j in 0..self.reps.len() {
            for k in 0..ph {
                pts.push(self.phase_point(j, k));
            }
        }
        pts
    }

    fn decomposition_counts(&self, phases: bool) -> Vec<usize> {
        self.decompositions(phases)
            .iter()
            .map(|q| q.len())
            .collect()
    }

    /// Exhaustive pair matching on discrete logarithms. Returns, per target
    /// (in manifest order), the set of x-class quadruples (indices into
    /// `pool`) with a signed sum equal to the target.
    fn decompositions(&self, phases: bool) -> Vec<BTreeSet<[usize; 4]>> {
        let logs: Vec<u64> = self.pool(phases).iter().map(|p| self.log[p]).collect();
        let r = self.r;
        let mut pair: HashMap<u64, Vec<(usize, usize)>> = HashMap::new();
        for a in 0..logs.len() {
            for b in (a + 1)..logs.len() {
                for sa in [1i64, -1] {
                    for sb in [1i64, -1] {
                        let v = (sa * logs[a] as i64 + sb * logs[b] as i64).rem_euclid(r as i64);
                        pair.entry(v as u64).or_default().push((a, b));
                    }
                }
            }
        }
        self.targets
            .iter()
            .map(|(_, _, rp)| {
                let l = self.log[rp];
                let mut quads: BTreeSet<[usize; 4]> = BTreeSet::new();
                for (v1, ps1) in &pair {
                    let v2 = (l + r - v1) % r;
                    if let Some(ps2) = pair.get(&v2) {
                        for &(a, b) in ps1 {
                            for &(c, d) in ps2 {
                                let mut q = [a, b, c, d];
                                q.sort_unstable();
                                if q[0] != q[1] && q[1] != q[2] && q[2] != q[3] {
                                    quads.insert(q);
                                }
                            }
                        }
                    }
                }
                quads
            })
            .collect()
    }
}

/// Dense Gaussian elimination mod a prime, one pass per inserted row.
struct Rank {
    r: u64,
    rows: Vec<(usize, Vec<u64>)>,
}

impl Rank {
    fn mulmod(&self, a: u64, b: u64) -> u64 {
        (a as u128 * b as u128 % self.r as u128) as u64
    }
    fn inv(&self, a: u64) -> u64 {
        let (mut r, mut b, mut e) = (1u64, a % self.r, self.r - 2);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulmod(r, b);
            }
            b = self.mulmod(b, b);
            e >>= 1;
        }
        r
    }
    /// Returns true if the row is independent of those inserted so far.
    fn insert(&mut self, row: &[u64]) -> bool {
        let mut row: Vec<u64> = row.iter().map(|v| v % self.r).collect();
        for (piv, base) in &self.rows {
            let f = row[*piv];
            if f != 0 {
                for (x, y) in row.iter_mut().zip(base) {
                    *x = (*x + self.r - self.mulmod(f, *y)) % self.r;
                }
            }
        }
        if let Some(piv) = row.iter().position(|v| *v != 0) {
            let inv = self.inv(row[piv]);
            let row: Vec<u64> = row.iter().map(|v| self.mulmod(*v, inv)).collect();
            self.rows.push((piv, row));
            true
        } else {
            false
        }
    }
}

struct Rec {
    v: Value,
}

impl Rec {
    fn s(&self, k: &str) -> &str {
        self.v[k].as_str().unwrap_or("")
    }
    fn u(&self, k: &str) -> u64 {
        self.v[k].as_u64().unwrap_or(0)
    }
}

/// Check one verified receipt. Returns the coefficient row.
fn check_verified(
    w: &World,
    rec: &Rec,
    arm_b: bool,
    target: (u64, u64),
) -> Result<Vec<u64>, String> {
    let pts: Vec<(u64, u64)> = rec.v["found_points"]
        .as_array()
        .ok_or("verified receipt without found_points")?
        .iter()
        .map(|p| {
            Ok((
                unhex(p[0].as_str().ok_or("x")?)?,
                unhex(p[1].as_str().ok_or("y")?)?,
            ))
        })
        .collect::<Result<_, String>>()?;
    if pts.len() != 4 {
        return Err(format!("{} points, expected 4", pts.len()));
    }
    let xs: BTreeSet<u64> = pts.iter().map(|p| p.0).collect();
    if xs.len() != 4 {
        return Err("abscissae are not distinct".into());
    }
    let mut sum: P = None;
    let mut row = vec![0u64; w.reps.len()];
    let mut seen_phase: BTreeMap<u64, (usize, u32)> = BTreeMap::new();
    for p in &pts {
        if !w.on(Some(*p)) {
            return Err(format!("point x={:#x} is off the curve", p.0));
        }
        if !w.log.contains_key(p) {
            return Err(format!("point x={:#x} is outside the odd subgroup", p.0));
        }
        let &(j, k) = w
            .orbit
            .get(&p.0)
            .ok_or_else(|| format!("x={:#x} is in no representative orbit", p.0))?;
        if !arm_b && k != 0 {
            return Err(format!("arm A receipt uses phase {k}"));
        }
        let base = w.phase_point(j, k);
        let sign: i64 = if *p == base {
            1
        } else if Some(*p) == w.neg(Some(base)) {
            -1
        } else {
            return Err(format!("point x={:#x} is not +-pi^{k}(V_{j})", p.0));
        };
        sum = w.add(sum, Some(*p));
        let c = w.lam_pow(k);
        let c = if sign == 1 { c } else { w.r - c };
        row[j] = (row[j] + c) % w.r;
        seen_phase.insert(p.0, (j, k));
    }
    if sum != Some(target) {
        return Err("signed sum is not the target".into());
    }
    let lhs: u128 = row
        .iter()
        .zip(&w.rep_logs)
        .map(|(c, l)| *c as u128 * *l as u128)
        .sum::<u128>()
        % w.r as u128;
    if lhs != w.log[&target] as u128 {
        return Err("log R != sum c_j log V_j (mod r)".into());
    }
    // The solver's own account of the summands must agree with the points.
    if let Some(sm) = rec.v["solver_summands"].as_array() {
        let mut got = BTreeSet::new();
        for s in sm {
            let v = unhex(s["v"].as_str().ok_or("v")?)?;
            let k = s["k"].as_u64().ok_or("k")? as u32;
            let &(j, _) = w.orbit.get(&v).ok_or("summand v is not a representative")?;
            if w.reps[j].0 != v {
                return Err("summand v is not a representative".into());
            }
            let mut x = v;
            for _ in 0..k {
                x = w.sqr(x);
            }
            got.insert(x);
        }
        if got != xs {
            return Err("solver summands disagree with the found points".into());
        }
    } else {
        return Err("verified receipt without solver_summands".into());
    }
    let _ = seen_phase;
    Ok(row)
}

fn schedule(
    w: &World,
    forms: &[String],
    reps: u64,
) -> Vec<(u64, String, usize, usize, String, String)> {
    let mut out = Vec::new();
    for rep in 1..=reps {
        for f in forms {
            for (i, t) in w.targets.iter().enumerate() {
                let order = if (i as u64 + rep - 1).is_multiple_of(2) {
                    ["A", "B"]
                } else {
                    ["B", "A"]
                };
                for (pos, arm) in order.iter().enumerate() {
                    out.push((rep, f.clone(), i, pos, arm.to_string(), t.0.clone()));
                }
            }
        }
    }
    out
}

/// Per (repetition, formulation, arm) totals, every attempt charged.
#[derive(Default, Clone, Debug)]
struct Sc {
    attempts: u64,
    verified: u64,
    unsat: u64,
    budget: u64,
    deadline: u64,
    skipped: u64,
    rejected: u64,
    dependent: u64,
    conflicts: u64,
    decisions: u64,
    propagations: u64,
    word_ops: u64,
    cpu_ns: u64,
    wall_ns: u64,
    rank: usize,
}

/// One charged attempt, kept for the tables.
struct AttRow {
    rep: u64,
    form: String,
    arm: String,
    target: usize,
    status: String,
    conflicts: u64,
    cpu_ns: u64,
}

type Key = (u64, String, String);

/// Everything `replay` establishes, for the canonical JSON and the tables.
struct Replayed {
    json: Value,
    scopes: BTreeMap<Key, Sc>,
    rows: Vec<AttRow>,
    first_hit: BTreeMap<Key, BTreeMap<usize, (u64, u64, u64)>>,
    targets: Vec<(String, String)>,
    /// Per target (manifest order): decomposable in arm A's search space.
    a_decomposable: Vec<bool>,
    world: World,
}

fn ratio(num: u64, den: usize) -> Value {
    if den == 0 {
        Value::Null
    } else {
        json!(format!("{:.3}", num as f64 / den as f64))
    }
}

/// Replays receipts text against manifest text. `Ok` carries the canonical
/// output; its `defects` list is empty exactly when every check passed.
#[cfg(test)]
fn replay(manifest_text: &str, receipts_text: &str, sealed: Option<&str>) -> Result<Value, String> {
    replay_full(manifest_text, receipts_text, sealed).map(|r| r.json)
}

fn replay_full(
    manifest_text: &str,
    receipts_text: &str,
    sealed: Option<&str>,
) -> Result<Replayed, String> {
    let w = World::from_manifest(manifest_text)?;
    let recs: Vec<Rec> = receipts_text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            serde_json::from_str(l)
                .map(|v| Rec { v })
                .map_err(|e| e.to_string())
        })
        .collect::<Result<_, _>>()?;
    let header = recs
        .iter()
        .find(|r| r.s("type") == "header")
        .ok_or("no header")?;
    let mut defects: Vec<String> = Vec::new();
    if header.s("manifest_sha256") != w.sha {
        defects.push(format!(
            "header manifest sha {} != recomputed {}",
            header.s("manifest_sha256"),
            w.sha
        ));
    }
    let budget = header.u("conflict_budget");
    let cms = header.s("schema") == CMS_SCHEMA;
    // CryptoMiniSat tests its conflict limit at restart granularity (it reported
    // 5,002 conflicts under a limit of 5,000): one part in a thousand is allowed.
    let slack = if cms { budget / 1000 } else { 0 };
    let maxtime_cpu = header.v["maxtime_cpu_s"].as_f64().unwrap_or(0.0);
    let forms: Vec<String> = header.v["forms"]
        .as_array()
        .map(|a| {
            a.iter()
                .map(|f| f.as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();

    // Structural ground truth.
    let ca = w.decomposition_counts(false);
    let cb = w.decomposition_counts(true);
    let by_id: HashMap<&str, usize> = w
        .targets
        .iter()
        .enumerate()
        .map(|(i, t)| (t.0.as_str(), i))
        .collect();
    if let Some(s) = sealed {
        let sv: Value = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if sv["manifest_sha256"] != w.sha {
            defects.push("sealed file names a different manifest".into());
        }
        for (i, t) in w.targets.iter().enumerate() {
            let bf = &sv["targets"][&t.0]["brute_force"];
            if bf["decomposable_reps_only"].as_bool() != Some(ca[i] > 0)
                || bf["decomposable_phases_free"].as_bool() != Some(cb[i] > 0)
            {
                defects.push(format!(
                    "sealed brute force disagrees with exhaustive replay on {}",
                    t.0
                ));
            }
        }
    }

    // Per-attempt checks.
    let all_atts: Vec<&Rec> = recs.iter().filter(|r| r.s("type") == "attempt").collect();
    // A `skipped` receipt is superseded by a later receipt of the same schedule
    // slot (a capped run resumed with a larger cap); it stays in the file.
    let slot = |a: &Rec| {
        (
            a.u("rep"),
            a.s("form").to_string(),
            a.u("pair"),
            a.u("position"),
            a.s("arm").to_string(),
            a.s("target_id").to_string(),
        )
    };
    let mut later_real = std::collections::HashSet::new();
    let mut keep = vec![true; all_atts.len()];
    for (i, a) in all_atts.iter().enumerate().rev() {
        if a.s("status") == "skipped" {
            keep[i] = !later_real.contains(&slot(a));
        } else {
            later_real.insert(slot(a));
        }
    }
    let superseded = keep.iter().filter(|k| !**k).count();
    let atts: Vec<&Rec> = all_atts
        .iter()
        .zip(&keep)
        .filter(|(_, k)| **k)
        .map(|(a, _)| *a)
        .collect();
    let max_rep = atts.iter().map(|a| a.u("rep")).max().unwrap_or(0);
    let expect = schedule(&w, &forms, max_rep);
    let got: Vec<_> = atts
        .iter()
        .map(|a| {
            (
                a.u("rep"),
                a.s("form").to_string(),
                a.u("pair") as usize,
                a.u("position") as usize,
                a.s("arm").to_string(),
                a.s("target_id").to_string(),
            )
        })
        .collect();
    if got != expect {
        defects.push(format!(
            "receipt order is not the preregistered schedule ({} receipts, {} expected)",
            got.len(),
            expect.len()
        ));
    }
    let seqs: Vec<u64> = all_atts.iter().map(|a| a.u("seq")).collect();
    if seqs != (0..all_atts.len() as u64).collect::<Vec<_>>() {
        defects.push("seq numbers are not 0..n-1 in order".into());
    }

    let mut ranks: BTreeMap<(u64, String, String), Rank> = BTreeMap::new();
    let mut sc: BTreeMap<(u64, String, String), Sc> = BTreeMap::new();
    let mut rows: Vec<AttRow> = Vec::new();
    let mut first_hit: BTreeMap<Key, BTreeMap<usize, (u64, u64, u64)>> = BTreeMap::new();
    for a in &atts {
        let tid = a.s("target_id");
        let Some(&ti) = by_id.get(tid) else {
            defects.push(format!("seq {}: unknown target {tid}", a.u("seq")));
            continue;
        };
        let target = w.targets[ti].2;
        let tag = format!(
            "seq {} rep {} {} {} {}",
            a.u("seq"),
            a.u("rep"),
            a.s("form"),
            a.s("arm"),
            tid
        );
        let rt = (
            unhex(a.v["target"][0].as_str().unwrap_or("0x0")).unwrap_or(0),
            unhex(a.v["target"][1].as_str().unwrap_or("0x0")).unwrap_or(0),
        );
        if rt != target {
            defects.push(format!(
                "{tag}: target coordinates differ from the manifest"
            ));
        }
        if a.s("set") != w.targets[ti].1 {
            defects.push(format!("{tag}: set label differs from the manifest"));
        }
        let arm_b = match a.s("arm") {
            "A" => false,
            "B" => true,
            other => {
                defects.push(format!("{tag}: arm {other}"));
                continue;
            }
        };
        if a.v["phases"].as_bool() != Some(arm_b) {
            defects.push(format!("{tag}: phases flag inconsistent with arm"));
        }
        let key = (a.u("rep"), a.s("form").to_string(), a.s("arm").to_string());
        let s = sc.entry(key.clone()).or_default();
        let st = a.s("status");
        if st == "skipped" {
            s.skipped += 1;
            continue;
        }
        s.attempts += 1;
        s.conflicts += a.u("conflicts");
        s.decisions += a.u("decisions");
        s.propagations += a.u("propagations");
        s.word_ops += a.u("word_ops");
        s.cpu_ns += a.u("total_cpu_ns");
        s.wall_ns += a.u("total_wall_ns");
        s.rejected += a.u("rejected_tuples");
        rows.push(AttRow {
            rep: a.u("rep"),
            form: a.s("form").to_string(),
            arm: a.s("arm").to_string(),
            target: ti,
            status: st.to_string(),
            conflicts: a.u("conflicts"),
            cpu_ns: a.u("total_cpu_ns"),
        });
        if a.u("conflicts") > budget + slack {
            defects.push(format!(
                "{tag}: {} conflicts exceed the budget {budget}",
                a.u("conflicts")
            ));
        }
        if cms {
            cms_attempt_checks(a, st, maxtime_cpu, &tag, &mut defects);
        }
        let decomposable = if arm_b { cb[ti] > 0 } else { ca[ti] > 0 };
        match st {
            "verified" => {
                s.verified += 1;
                if !decomposable {
                    defects.push(format!(
                        "{tag}: verified, but the exhaustive check says not decomposable"
                    ));
                }
                match check_verified(&w, a, arm_b, target) {
                    Err(e) => defects.push(format!("{tag}: {e}")),
                    Ok(row) => {
                        let rk = ranks.entry(key.clone()).or_insert_with(|| Rank {
                            r: w.r,
                            rows: Vec::new(),
                        });
                        if !rk.insert(&row) {
                            s.dependent += 1;
                        } else {
                            first_hit
                                .entry(key.clone())
                                .or_default()
                                .insert(rk.rows.len(), (s.attempts, s.conflicts, s.cpu_ns));
                        }
                        s.rank = rk.rows.len();
                    }
                }
            }
            "unsat" => {
                s.unsat += 1;
                if decomposable {
                    defects.push(format!(
                        "{tag}: UNSAT, but the target is decomposable in this arm"
                    ));
                }
            }
            "budget" => {
                s.budget += 1;
                if a.u("conflicts") < budget {
                    defects.push(format!("{tag}: budget status below the budget"));
                }
            }
            "deadline" => s.deadline += 1,
            other => defects.push(format!("{tag}: unknown status {other}")),
        }
    }

    // Report.
    let structural: Vec<Value> = w
        .targets
        .iter()
        .enumerate()
        .map(|(i, t)| {
            json!({"target": t.0, "set": t.1, "arm_a_quadruples": ca[i], "arm_b_quadruples": cb[i]})
        })
        .collect();
    let scopes: Vec<Value> = sc
        .iter()
        .map(|((rep, form, arm), s)| {
            json!({"rep": rep, "form": form, "arm": arm, "attempts": s.attempts,
                   "verified": s.verified, "unsat": s.unsat, "budget": s.budget,
                   "deadline": s.deadline, "skipped": s.skipped,
                   "rejected_tuples": s.rejected, "dependent_verified": s.dependent,
                   "rank": s.rank, "conflicts": s.conflicts, "decisions": s.decisions,
                   "propagations": s.propagations, "word_ops": s.word_ops,
                   "cpu_s": format!("{:.3}", s.cpu_ns as f64 / 1e9),
                   "wall_s": format!("{:.3}", s.wall_ns as f64 / 1e9),
                   "conflicts_per_new_independent": ratio(s.conflicts, s.rank),
                   "cpu_s_per_new_independent": if s.rank == 0 { Value::Null } else { json!(format!("{:.4}", s.cpu_ns as f64 / 1e9 / s.rank as f64)) }})
        })
        .collect();
    let mut ratios: Vec<Value> = Vec::new();
    let mut per_form: BTreeMap<String, Vec<(bool, bool)>> = BTreeMap::new();
    for ((rep, form, arm), a) in &sc {
        if arm != "A" {
            continue;
        }
        let Some(b) = sc.get(&(*rep, form.clone(), "B".to_string())) else {
            continue;
        };
        if a.rank == 0 || b.rank == 0 {
            continue;
        }
        let rc = (b.conflicts as f64 / b.rank as f64) / (a.conflicts as f64 / a.rank as f64);
        let rt = (b.cpu_ns as f64 / b.rank as f64) / (a.cpu_ns as f64 / a.rank as f64);
        per_form
            .entry(form.clone())
            .or_default()
            .push((rc < 1.0, rt < 1.0));
        ratios.push(
            json!({"rep": rep, "form": form, "conflicts_ratio_b_over_a": format!("{rc:.3}"),
                           "cpu_ratio_b_over_a": format!("{rt:.3}")}),
        );
    }
    let h1 = !per_form.is_empty()
        && per_form.values().all(|v| v.iter().all(|(c, t)| *c && *t))
        && sc.values().all(|s| s.rejected == 0);
    let json = json!({
        "schema": "phase_paired_native/replay/v1",
        "manifest_sha256": w.sha,
        "conflict_budget": budget,
        "lambda": w.lam, "r": w.r,
        "receipts": atts.len(),
        "superseded_skipped": superseded,
        "structural_exhaustive": structural,
        "arm_a_decomposable": ca.iter().filter(|c| **c > 0).count(),
        "arm_b_decomposable": cb.iter().filter(|c| **c > 0).count(),
        "scopes": scopes,
        "ratios": ratios,
        "h1_criterion_met_all_forms_all_reps": h1,
        "defects": defects,
    });
    Ok(Replayed {
        json,
        scopes: sc,
        rows,
        first_hit,
        targets: w
            .targets
            .iter()
            .map(|t| (t.0.clone(), t.1.clone()))
            .collect(),
        a_decomposable: ca.iter().map(|c| *c > 0).collect(),
        world: w,
    })
}

// ---------------------------------------------------------------------------
// Tables and chart, rendered from verified receipts only
// ---------------------------------------------------------------------------

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        f64::NAN
    } else if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn per_new(total: f64, rank: usize) -> f64 {
    if rank == 0 {
        f64::NAN
    } else {
        total / rank as f64
    }
}

/// Cost per newly independent relation of one scope: (conflicts, CPU s).
fn costs(s: &Sc) -> (f64, f64) {
    (
        per_new(s.conflicts as f64, s.rank),
        per_new(s.cpu_ns as f64 / 1e9, s.rank),
    )
}

fn short(v: f64) -> String {
    if v >= 1e6 {
        format!("{:.2}M", v / 1e6)
    } else if v >= 1e4 {
        format!("{:.0}k", v / 1e3)
    } else {
        format!("{v:.1}")
    }
}

fn tables_md(rp: &Replayed, ncols: usize) -> String {
    let mut md = String::new();
    let forms: Vec<String> = {
        let mut f: Vec<String> = rp.scopes.keys().map(|k| k.1.clone()).collect();
        f.sort();
        f.dedup();
        f
    };
    let reps: Vec<u64> = {
        let mut r: Vec<u64> = rp.scopes.keys().map(|k| k.0).collect();
        r.sort();
        r.dedup();
        r
    };
    md.push_str("### Per repetition and arm, every attempt charged\n\n");
    md.push_str("| rep | form | arm | attempts | verified | UNSAT | budget or deadline | rank | conflicts | CPU s | conflicts per new independent | CPU s per new independent | dependent verified |\n");
    md.push_str("|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for ((rep, form, arm), s) in &rp.scopes {
        let (c, t) = costs(s);
        md.push_str(&format!(
            "| {rep} | {form} | {arm} | {} | {} | {} | {} | {}/{ncols} | {} | {:.1} | {:.0} | {:.1} | {} |\n",
            s.attempts,
            s.verified,
            s.unsat,
            s.budget + s.deadline,
            s.rank,
            s.conflicts,
            s.cpu_ns as f64 / 1e9,
            c,
            t,
            s.dependent
        ));
    }

    md.push_str("\n### Ratio B / A of cost per newly independent verified relation\n\n");
    md.push_str("| rep | form | conflicts | CPU | decisions (exploratory) | B below A on conflicts | B below A on CPU |\n|---|---|---|---|---|---|---|\n");
    let mut by_form: BTreeMap<String, Vec<(f64, f64)>> = BTreeMap::new();
    for form in &forms {
        for rep in &reps {
            let (Some(a), Some(b)) = (
                rp.scopes.get(&(*rep, form.clone(), "A".to_string())),
                rp.scopes.get(&(*rep, form.clone(), "B".to_string())),
            ) else {
                continue;
            };
            if a.rank == 0 || b.rank == 0 {
                continue;
            }
            let (ca, ta) = costs(a);
            let (cb, tb) = costs(b);
            let rd = per_new(b.decisions as f64, b.rank) / per_new(a.decisions as f64, a.rank);
            by_form
                .entry(form.clone())
                .or_default()
                .push((cb / ca, tb / ta));
            md.push_str(&format!(
                "| {rep} | {form} | {:.3} | {:.3} | {:.3} | {} | {} |\n",
                cb / ca,
                tb / ta,
                rd,
                cb / ca < 1.0,
                tb / ta < 1.0
            ));
        }
        if let Some(v) = by_form.get(form) {
            let n = v.len() as f64;
            let (mc, mt) = (
                v.iter().map(|x| x.0).sum::<f64>() / n,
                v.iter().map(|x| x.1).sum::<f64>() / n,
            );
            let rng = |f: fn(&(f64, f64)) -> f64| {
                let lo = v.iter().map(f).fold(f64::INFINITY, f64::min);
                let hi = v.iter().map(f).fold(f64::NEG_INFINITY, f64::max);
                format!("{lo:.3} to {hi:.3}")
            };
            md.push_str(&format!(
                "| mean | {form} | {mc:.3} (range {}) | {mt:.3} (range {}) | | | |\n",
                rng(|x| x.0),
                rng(|x| x.1)
            ));
        }
    }

    md.push_str("\n### Decision rule (PROTOCOL.md)\n\n");
    let all_clean = rp.scopes.values().all(|s| s.rejected == 0);
    let mut supported = !by_form.is_empty() && all_clean;
    for (form, v) in &by_form {
        let both = v.iter().filter(|x| x.0 < 1.0 && x.1 < 1.0).count();
        let neither = v.iter().filter(|x| x.0 >= 1.0 && x.1 >= 1.0).count();
        let verdict = if both == v.len() {
            "H1 criterion met"
        } else if neither == v.len() {
            "H1 not supported (B at or above A on both clocks in every repetition)"
        } else {
            "mixed (the clocks or repetitions disagree), counts as not supported"
        };
        supported &= both == v.len();
        md.push_str(&format!(
            "- {form}: B below A on both clocks in {both} of {} repetitions; at or above on both in {neither}; verdict: {verdict}\n",
            v.len()
        ));
    }
    md.push_str(&format!(
        "- every relation passed every check, no rejected tuple: {all_clean}\n- **overall: {}**\n",
        if supported {
            "H1 supported"
        } else {
            "H1 not supported"
        }
    ));

    md.push_str("\n### Where the cost goes, by outcome (conflicts; CPU s)\n\n");
    md.push_str("| rep | arm | verified (n, conflicts, CPU s) | UNSAT (n, conflicts, CPU s) | budget or deadline (n, conflicts, CPU s) |\n|---|---|---|---|---|\n");
    for (rep, _form, arm) in rp.scopes.keys() {
        let sel = |st: &[&str]| -> (u64, u64, f64) {
            rp.rows
                .iter()
                .filter(|r| r.rep == *rep && &r.arm == arm && st.contains(&r.status.as_str()))
                .fold((0, 0, 0.0), |a, r| {
                    (a.0 + 1, a.1 + r.conflicts, a.2 + r.cpu_ns as f64 / 1e9)
                })
        };
        let (v, u, b) = (
            sel(&["verified"]),
            sel(&["unsat"]),
            sel(&["budget", "deadline"]),
        );
        md.push_str(&format!(
            "| {rep} | {arm} | {}, {}, {:.1} | {}, {}, {:.1} | {}, {}, {:.1} |\n",
            v.0, v.1, v.2, u.0, u.1, u.2, b.0, b.1, b.2
        ));
    }

    md.push_str("\n### Like for like: the four targets both arms decompose (b0 to b3)\n\n");
    md.push_str("| rep | A conflicts | B conflicts | B/A conflicts | A CPU s | B CPU s | B/A CPU |\n|---|---|---|---|---|---|---|\n");
    for rep in &reps {
        let sum = |arm: &str| -> (f64, f64) {
            rp.rows
                .iter()
                .filter(|r| r.rep == *rep && r.arm == arm && rp.targets[r.target].1 == "b")
                .fold((0.0, 0.0), |a, r| {
                    (a.0 + r.conflicts as f64, a.1 + r.cpu_ns as f64 / 1e9)
                })
        };
        let (a, b) = (sum("A"), sum("B"));
        md.push_str(&format!(
            "| {rep} | {:.0} | {:.0} | {:.2} | {:.1} | {:.1} | {:.2} |\n",
            a.0,
            b.0,
            b.0 / a.0,
            a.1,
            b.1,
            b.1 / a.1
        ));
    }

    md.push_str("\n### Per target, median over repetitions\n\n");
    md.push_str("| form | target | A status | A conflicts | A CPU s | B status | B conflicts | B CPU s | B/A conflicts | B/A CPU |\n|---|---|---|---|---|---|---|---|---|---|\n");
    for form in &forms {
        for (ti, (tid, _)) in rp.targets.iter().enumerate() {
            let pick = |arm: &str| -> Vec<&AttRow> {
                rp.rows
                    .iter()
                    .filter(|r| &r.form == form && r.arm == arm && r.target == ti)
                    .collect()
            };
            let (pa, pb) = (pick("A"), pick("B"));
            if pa.is_empty() || pb.is_empty() {
                continue;
            }
            let st = |v: &Vec<&AttRow>| {
                let mut s: Vec<String> = v.iter().map(|r| r.status.clone()).collect();
                s.sort();
                s.dedup();
                s.join("/")
            };
            let mc = |v: &Vec<&AttRow>| {
                median(&mut v.iter().map(|r| r.conflicts as f64).collect::<Vec<_>>())
            };
            let mt = |v: &Vec<&AttRow>| {
                median(&mut v.iter().map(|r| r.cpu_ns as f64 / 1e9).collect::<Vec<_>>())
            };
            let both = pa.iter().all(|r| r.status == "verified")
                && pb.iter().all(|r| r.status == "verified");
            md.push_str(&format!(
                "| {form} | {tid} | {} | {:.0} | {:.1} | {} | {:.0} | {:.1} | {} | {} |\n",
                st(&pa),
                mc(&pa),
                mt(&pa),
                st(&pb),
                mc(&pb),
                mt(&pb),
                if both {
                    format!("{:.2}", mc(&pb) / mc(&pa))
                } else {
                    "-".into()
                },
                if both {
                    format!("{:.2}", mt(&pb) / mt(&pa))
                } else {
                    "-".into()
                },
            ));
        }
    }

    md.push_str("\n### Cost charged when each rank was first reached (attempts, conflicts, CPU s); depends on the schedule's target order\n\n");
    md.push_str("| rep | form | arm | rank 1 | rank 2 | rank 3 | rank 4 | rank 5 |\n|---|---|---|---|---|---|---|---|\n");
    for key in rp.scopes.keys() {
        let cells: Vec<String> = (1..=ncols)
            .map(|k| match rp.first_hit.get(key).and_then(|m| m.get(&k)) {
                Some((a, c, t)) => format!("{a}, {c}, {:.1}", *t as f64 / 1e9),
                None => "-".into(),
            })
            .collect();
        md.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            key.0,
            key.1,
            key.2,
            cells.join(" | ")
        ));
    }
    md
}

/// Hand-written SVG, light and dark through CSS variables. Bars: cost per
/// newly independent relation for arm A and arm B in each repetition, and the
/// mean. Colours are the first two slots of the validated categorical palette.
fn svg_chart(rp: &Replayed) -> String {
    let forms: Vec<String> = {
        let mut f: Vec<String> = rp.scopes.keys().map(|k| k.1.clone()).collect();
        f.sort();
        f.dedup();
        f
    };
    // groups: (label, (conflicts A, B), (CPU A, B))
    let mut groups: Vec<(String, (f64, f64), (f64, f64))> = Vec::new();
    for form in &forms {
        let reps: Vec<u64> = rp
            .scopes
            .keys()
            .filter(|k| &k.1 == form)
            .map(|k| k.0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut acc = (0.0, 0.0, 0.0, 0.0);
        let mut n = 0.0;
        for rep in &reps {
            let (Some(a), Some(b)) = (
                rp.scopes.get(&(*rep, form.clone(), "A".to_string())),
                rp.scopes.get(&(*rep, form.clone(), "B".to_string())),
            ) else {
                continue;
            };
            let (ca, ta) = costs(a);
            let (cb, tb) = costs(b);
            groups.push((format!("rep {rep}"), (ca, cb), (ta, tb)));
            acc = (acc.0 + ca, acc.1 + cb, acc.2 + ta, acc.3 + tb);
            n += 1.0;
        }
        if n > 0.0 {
            groups.push((
                "mean".into(),
                (acc.0 / n, acc.1 / n),
                (acc.2 / n, acc.3 / n),
            ));
        }
    }
    let (w, h) = (900.0f64, 470.0f64);
    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" aria-labelledby=\"t d\" font-family=\"sans-serif\" font-size=\"12\">\n"
    ));
    s.push_str("<title id=\"t\">Cost per newly independent verified relation, arm A against arm B</title>\n");
    s.push_str("<desc id=\"d\">Grouped bars per repetition and their mean, conflicts and CPU seconds, native sat::Solver, n = 13 stage diagnostic. The same numbers are in tables.md.</desc>\n");
    s.push_str("<style>\nsvg{--surface:#fcfcfb;--ink:#0b0b0b;--ink2:#52514e;--grid:#e3e2de;--a:#2a78d6;--b:#eb6834}\n@media (prefers-color-scheme: dark){svg{--surface:#1a1a19;--ink:#f0efec;--ink2:#c3c2b7;--grid:#3a3935;--a:#3987e5;--b:#d95926}}\n.bg{fill:var(--surface)}.ink{fill:var(--ink)}.ink2{fill:var(--ink2)}.gl{stroke:var(--grid);stroke-width:1}.ax{stroke:var(--ink2);stroke-width:1}.ba{fill:var(--a)}.bb{fill:var(--b)}\n</style>\n");
    s.push_str(&format!(
        "<rect class=\"bg\" width=\"{w}\" height=\"{h}\"/>\n"
    ));
    s.push_str("<text class=\"ink\" x=\"24\" y=\"28\" font-size=\"16\" font-weight=\"bold\">Cost per newly independent verified relation, every attempt charged</text>\n");
    s.push_str("<text class=\"ink2\" x=\"24\" y=\"48\">Native sat::Solver (not CryptoMiniSat), n = 13, subgroup_xor, 3 repetitions. Lower is cheaper. Rank cap 5; arm A can reach 4.</text>\n");
    // legend
    s.push_str("<rect class=\"ba\" x=\"24\" y=\"64\" width=\"12\" height=\"12\" rx=\"2\"/><text class=\"ink\" x=\"42\" y=\"75\">Arm A: representatives only</text>\n");
    s.push_str("<rect class=\"bb\" x=\"236\" y=\"64\" width=\"12\" height=\"12\" rx=\"2\"/><text class=\"ink\" x=\"254\" y=\"75\">Arm B: Frobenius phases as solver unknowns</text>\n");
    let panels = [
        ("Solver conflicts per new relation", 0usize),
        ("CPU seconds per new relation", 1usize),
    ];
    for (pi, (title, metric)) in panels.iter().enumerate() {
        let x0 = 70.0 + pi as f64 * 430.0;
        let (top, bot, width) = (130.0f64, 380.0f64, 370.0f64);
        let val = |g: &(String, (f64, f64), (f64, f64))| if *metric == 0 { g.1 } else { g.2 };
        let maxv = groups
            .iter()
            .map(|g| val(g).0.max(val(g).1))
            .fold(0.0f64, f64::max)
            * 1.12;
        let maxv = if maxv > 0.0 { maxv } else { 1.0 };
        s.push_str(&format!(
            "<text class=\"ink\" x=\"{x0}\" y=\"108\" font-weight=\"bold\">{title}</text>\n"
        ));
        for tick in 0..=4 {
            let v = maxv * tick as f64 / 4.0;
            let y = bot - (bot - top) * tick as f64 / 4.0;
            s.push_str(&format!(
                "<line class=\"gl\" x1=\"{x0}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\"/>\n",
                x0 + width
            ));
            s.push_str(&format!(
                "<text class=\"ink2\" x=\"{}\" y=\"{}\" text-anchor=\"end\">{}</text>\n",
                x0 - 6.0,
                y + 4.0,
                short(v)
            ));
        }
        s.push_str(&format!(
            "<line class=\"ax\" x1=\"{x0}\" y1=\"{bot}\" x2=\"{}\" y2=\"{bot}\"/>\n",
            x0 + width
        ));
        let gw = width / groups.len().max(1) as f64;
        for (gi, g) in groups.iter().enumerate() {
            let (va, vb) = val(g);
            let gx = x0 + gi as f64 * gw;
            let bw = (gw * 0.34).min(40.0);
            for (k, (v, cls)) in [(va, "ba"), (vb, "bb")].iter().enumerate() {
                let bx = gx + gw / 2.0 - bw - 1.0 + k as f64 * (bw + 2.0);
                let hgt = (bot - top) * v / maxv;
                s.push_str(&format!(
                    "<path class=\"{cls}\" d=\"M{bx},{bot} V{} a4,4 0 0 1 4,-4 h{} a4,4 0 0 1 4,4 V{bot} z\"/>\n",
                    bot - hgt + 4.0,
                    bw - 8.0
                ));
                s.push_str(&format!(
                    "<text class=\"ink\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"11\">{}</text>\n",
                    bx + bw / 2.0,
                    bot - hgt - 5.0,
                    short(*v)
                ));
            }
            s.push_str(&format!(
                "<text class=\"ink2\" x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text>\n",
                gx + gw / 2.0,
                bot + 18.0,
                g.0
            ));
        }
    }
    s.push_str("<text class=\"ink2\" x=\"24\" y=\"428\">Historical, not comparable in absolute terms: the legacy Python run with CryptoMiniSat (one trial) read B/A = 1.61 on CPU.</text>\n");
    s.push_str("<text class=\"ink2\" x=\"24\" y=\"446\">Scope: stage diagnostic on a toy field. No S, no ratio to rho, no ECDLP speedup.</text>\n");
    s.push_str("</svg>\n");
    s
}

// ---------------------------------------------------------------------------
// CryptoMiniSat follow-up (PROTOCOL_ADDENDUM_2.md): receipts of the pinned
// external solver, the stage rule, the cap, the controls and the decision rule,
// all recomputed from the files alone.
// ---------------------------------------------------------------------------

const CMS_SCHEMA: &str = "phase_paired_cms/receipt/v1";
const CMS_CONTROLS_SCHEMA: &str = "phase_paired_cms/controls/v1";
const CMS_BINARY_SHA: &str = "8e1e539976b1fa8bff2b1e30df5c200c9a03ee343c634eadc0152e1b6cecd8f7";
/// Decision rule thresholds (seconds of CPU, median over the eight arm A
/// refutations of one repetition), fixed before any measurement.
const RULE_SOLVER_MAX_S: f64 = 45.0;
const RULE_ENCODING_MIN_S: f64 = 80.0;
const RULE_ONE_REP_SOLVER_MAX_S: f64 = 30.0;
const RULE_ONE_REP_ENCODING_MIN_S: f64 = 120.0;
/// Legacy (CryptoMiniSat 5.16.0 through pycryptosat, one trial): arm A UNSAT
/// CPU seconds, `subgroup_xor`, from `paired_n13.jsonl` of crypto PR #2583.
const LEGACY_A_REFUTATION_S: [(&str, f64); 8] = [
    ("a0", 8.3442),
    ("a1", 23.4089),
    ("a2", 20.5688),
    ("a3", 14.9519),
    ("c0", 44.0095),
    ("c1", 29.2503),
    ("c2", 28.0098),
    ("c3", 24.2291),
];
const LEGACY_B_OVER_A_CPU: f64 = 1.61;

/// The stage rule of the producer (`rep_mode`), restated here.
fn rep_mode(
    rep: u64,
    remaining: f64,
    guard: f64,
    full_cost: Option<f64>,
    a_cost: Option<f64>,
) -> &'static str {
    if rep == 1 {
        return "full";
    }
    match (full_cost, a_cost) {
        (Some(f), _) if remaining >= guard * f => "full",
        (_, Some(a)) if remaining >= guard * a => "armA",
        _ => "none",
    }
}

fn cms_attempt_checks(a: &Rec, st: &str, maxtime: f64, tag: &str, defects: &mut Vec<String>) {
    if a.s("solver") != "cryptominisat5" {
        defects.push(format!("{tag}: solver is not cryptominisat5"));
    }
    let c = &a.v["cms"];
    let calls = c["calls"].as_array().cloned().unwrap_or_default();
    if calls.is_empty() {
        defects.push(format!("{tag}: no child process call recorded"));
        return;
    }
    if !c["error"].is_null() {
        defects.push(format!("{tag}: an instrument error is recorded"));
    }
    if a.u("solver_calls") != calls.len() as u64 {
        defects.push(format!(
            "{tag}: solver_calls differs from the recorded calls"
        ));
    }
    let child = c["child_cpu_ns"].as_u64().unwrap_or(0);
    let sum: u64 = calls
        .iter()
        .map(|x| x["child_cpu_ns"].as_u64().unwrap_or(0))
        .sum();
    if child != sum || child > a.u("total_cpu_ns") || child != a.u("solve_cpu_ns") {
        defects.push(format!("{tag}: child CPU does not add up"));
    }
    if calls
        .iter()
        .any(|x| x["killed_at_wall_backstop"] == true || !x["signal"].is_null())
    {
        defects.push(format!("{tag}: a child was killed or died on a signal"));
    }
    let last = calls.last().unwrap();
    let want = match st {
        "verified" => Some((10, "SATISFIABLE")),
        "unsat" => Some((20, "UNSATISFIABLE")),
        "budget" | "deadline" => Some((15, "INDETERMINATE")),
        _ => None,
    };
    if let Some((exit, line)) = want {
        if last["exit_code"].as_i64() != Some(exit) || last["status_line"].as_str() != Some(line) {
            defects.push(format!(
                "{tag}: status {st} but the last child ended with exit {} and `{}`",
                last["exit_code"], last["status_line"]
            ));
        }
    }
    if st == "deadline" && (child as f64 / 1e9) < 0.95 * maxtime {
        defects.push(format!(
            "{tag}: deadline status with {:.1} s of child CPU, below the {maxtime} s backstop",
            child as f64 / 1e9
        ));
    }
}

/// The checks that only make sense for CryptoMiniSat receipts, and the
/// decision rule. Extends `rp.json` and its defect list.
fn cms_audit(rp: &mut Replayed, receipts_text: &str, controls_text: Option<&str>) {
    let recs: Vec<Value> = receipts_text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap_or(Value::Null))
        .collect();
    let mut defects: Vec<String> = Vec::new();
    let header = recs.iter().find(|r| r["type"] == "header").unwrap().clone();
    if header["cms_binary_sha256"] != CMS_BINARY_SHA {
        defects.push("header binary hash is not the pinned one".into());
    }
    let (cap, prior, controls_hdr) = (
        header["cpu_cap_s"].as_f64().unwrap_or(0.0),
        header["cpu_prior_s"].as_f64().unwrap_or(0.0),
        header["controls_cpu_s"].as_f64().unwrap_or(0.0),
    );
    let guard = header["stage_guard"].as_f64().unwrap_or(1.1);
    let maxtime = header["maxtime_cpu_s"].as_f64().unwrap_or(0.0);
    let secs = |r: &Value| -> f64 {
        if r["type"] != "attempt" || r["status"] == "skipped" {
            0.0
        } else {
            r["total_cpu_ns"].as_u64().unwrap_or(0) as f64 / 1e9
        }
    };

    // Controls file.
    let mut controls = json!(null);
    if let Some(text) = controls_text {
        let crecs: Vec<Value> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap_or(Value::Null))
            .collect();
        let count = |t: &str, pass: bool| {
            crecs
                .iter()
                .filter(|r| r["type"] == t && (r["pass"] == true) == pass)
                .count()
        };
        let controls_cpu: f64 = crecs
            .iter()
            .map(|r| {
                r["total_cpu_ns"]
                    .as_u64()
                    .or_else(|| r["cpu_ns"].as_u64())
                    .unwrap_or(0) as f64
                    / 1e9
            })
            .sum();
        for h in crecs.iter().filter(|r| r["type"] == "header") {
            if h["schema"] != CMS_CONTROLS_SCHEMA || h["cms_binary_sha256"] != CMS_BINARY_SHA {
                defects.push("controls header: wrong schema or binary hash".into());
            }
        }
        let (st_ok, st_bad) = (count("selftest", true), count("selftest", false));
        let (fi_ok, fi_bad) = (count("fidelity", true), count("fidelity", false));
        if st_bad > 0 || fi_bad > 0 || st_ok == 0 || fi_ok == 0 {
            defects.push(format!(
                "controls: selftest {st_ok} pass / {st_bad} fail, fidelity {fi_ok} pass / {fi_bad} fail"
            ));
        }
        let mut positive = 0;
        let mut deterministic = None;
        for p in crecs.iter().filter(|r| r["type"] == "positive_control") {
            let rec = Rec { v: p.clone() };
            let tid = rec.s("target_id").to_string();
            let Some(t) = rp.world.targets.iter().find(|t| t.0 == tid) else {
                defects.push(format!("positive control: unknown target {tid}"));
                continue;
            };
            if rec.s("status") != "verified" {
                defects.push(format!(
                    "positive control {tid}: status {}",
                    rec.s("status")
                ));
                continue;
            }
            match check_verified(&rp.world, &rec, rec.s("arm") == "B", t.2) {
                Ok(_) => positive += 1,
                Err(e) => defects.push(format!("positive control {tid}: {e}")),
            }
            if let Some(d) = p["same_seed_rerun_identical"].as_bool() {
                deterministic = Some(d);
            }
        }
        if positive == 0 {
            defects.push("controls: no verified positive control".into());
        }
        if deterministic != Some(true) {
            defects.push("controls: the same-seed rerun was not shown to be identical".into());
        }
        if (controls_cpu - controls_hdr).abs() > 0.01 {
            defects.push(format!(
                "controls CPU {controls_cpu:.3} s differs from the header's {controls_hdr:.3} s"
            ));
        }
        controls = json!({"selftest_pass": st_ok, "selftest_fail": st_bad,
            "fidelity_pass": fi_ok, "fidelity_fail": fi_bad,
            "positive_controls_verified": positive, "same_seed_rerun_identical": deterministic,
            "cpu_s": format!("{controls_cpu:.3}")});
    }

    // Cap accounting and the stage rule, in receipt order.
    let attempts_cpu: f64 = recs.iter().map(&secs).sum();
    let total = prior + controls_hdr + attempts_cpu;
    if total > cap {
        defects.push(format!(
            "total solver CPU {total:.1} s exceeds the cap {cap} s"
        ));
    }
    let mut spent = prior + controls_hdr;
    let mut so_far: Vec<&Value> = Vec::new();
    let mut stages: Vec<Value> = Vec::new();
    let mut modes: BTreeMap<u64, String> = BTreeMap::new();
    let mut first_cap_skip_checked = false;
    let mut cap_latched = false;
    for r in &recs {
        match r["type"].as_str() {
            Some("stage") => {
                let rep = r["rep"].as_u64().unwrap_or(0);
                let cost = |rr: u64, arm: Option<&str>| -> (f64, usize) {
                    let rows: Vec<&&Value> = so_far
                        .iter()
                        .filter(|x| {
                            x["type"] == "attempt"
                                && x["status"] != "skipped"
                                && x["rep"] == rr
                                && arm.is_none_or(|a| x["arm"] == a)
                        })
                        .collect();
                    (rows.iter().map(|x| secs(x)).sum(), rows.len())
                };
                let nt = rp.targets.len();
                let mx = |v: Vec<f64>| {
                    v.into_iter()
                        .fold(None::<f64>, |m, x| Some(m.map_or(x, |y| y.max(x))))
                };
                let fc = mx((1..rep)
                    .filter(|k| cost(*k, None).1 == 2 * nt)
                    .map(|k| cost(k, None).0)
                    .collect());
                let ac = mx((1..rep)
                    .filter(|k| cost(*k, Some("A")).1 == nt)
                    .map(|k| cost(k, Some("A")).0)
                    .collect());
                let remaining = cap - spent;
                let want = rep_mode(rep, remaining, guard, fc, ac);
                // After a cap stop every later repetition is `none` by the latch.
                if r["mode"] != want && !(cap_latched && r["mode"] == "none") {
                    defects.push(format!(
                        "stage rule: repetition {rep} ran as `{}` but the rule gives `{want}` (remaining {remaining:.1} s)",
                        r["mode"]
                    ));
                }
                if (r["remaining_cpu_s"].as_f64().unwrap_or(f64::NAN) - remaining).abs() > 0.01 {
                    defects.push(format!(
                        "stage rule: repetition {rep} recorded remaining {} s, receipts give {remaining:.3} s",
                        r["remaining_cpu_s"]
                    ));
                }
                modes.insert(rep, r["mode"].as_str().unwrap_or("").to_string());
                stages.push(json!({"rep": rep, "mode": r["mode"], "remaining_cpu_s": format!("{remaining:.1}")}));
            }
            Some("attempt") => {
                let rep = r["rep"].as_u64().unwrap_or(0);
                let mode = modes.get(&rep).map(String::as_str).unwrap_or("");
                if mode.is_empty() {
                    defects.push(format!("repetition {rep}: attempt before any stage record"));
                }
                if r["status"] == "skipped" {
                    let reason = r["reason"].as_str().unwrap_or("");
                    if reason.starts_with("cpu cap") {
                        cap_latched = true;
                        if !first_cap_skip_checked {
                            let arms = if mode == "armA" { 1.0 } else { 2.0 };
                            let worst = arms * (maxtime * 1.05 + 3.0);
                            if spent + worst <= cap {
                                defects.push(format!(
                                    "cap stop at {spent:.1} s was not needed ({worst:.0} s fits under {cap} s)"
                                ));
                            }
                            first_cap_skip_checked = true;
                        }
                    } else if reason.starts_with("stage rule: arm A only") {
                        if mode != "armA" || r["arm"] != "B" {
                            defects.push(format!(
                                "repetition {rep}: skip reason does not match mode {mode}"
                            ));
                        }
                    } else if reason.starts_with("stage rule") {
                        if mode != "none" {
                            defects.push(format!("repetition {rep}: skipped in mode {mode}"));
                        }
                    } else {
                        defects.push(format!("repetition {rep}: unexplained skip `{reason}`"));
                    }
                } else if mode == "none" || (mode == "armA" && r["arm"] == "B") {
                    defects.push(format!(
                        "repetition {rep}: ran an attempt the stage rule excludes"
                    ));
                }
                spent += secs(r);
            }
            _ => {}
        }
        so_far.push(r);
    }

    let rule = cms_rule(rp);
    rp.json["schema"] = json!("phase_paired_cms/replay/v1");
    rp.json["cms"] = json!({
        "binary_sha256": header["cms_binary_sha256"], "tag": header["cms_tag"],
        "commit": header["cms_commit"], "cadical_commit": header["cadical_commit"],
        "cadiback_commit": header["cadiback_commit"], "conflict_budget": header["conflict_budget"],
        "maxtime_cpu_s": header["maxtime_cpu_s"],
        "cap": {"cap_s": cap, "prior_s": format!("{prior:.3}"), "controls_s": format!("{controls_hdr:.3}"),
                "attempts_s": format!("{attempts_cpu:.3}"), "total_s": format!("{total:.3}")},
        "stages": stages, "controls": controls, "decision_rule": rule,
    });
    if let Some(d) = rp.json["defects"].as_array_mut() {
        d.extend(defects.into_iter().map(Value::from));
    }
}

/// The preregistered decision rule on the arm A refutations.
fn cms_rule(rp: &Replayed) -> Value {
    let nt = rp.a_decomposable.iter().filter(|d| !**d).count();
    let reps: BTreeSet<u64> = rp.rows.iter().map(|r| r.rep).collect();
    let mut per_rep = Vec::new();
    let mut classes: Vec<(u64, &'static str, f64)> = Vec::new();
    for rep in reps {
        let rows: Vec<&AttRow> = rp
            .rows
            .iter()
            .filter(|r| r.rep == rep && r.arm == "A" && !rp.a_decomposable[r.target])
            .collect();
        if rows.is_empty() {
            continue;
        }
        let t: Vec<f64> = rows.iter().map(|r| r.cpu_ns as f64 / 1e9).collect();
        let med = median(&mut t.clone());
        let unknown = rows.iter().filter(|r| r.status != "unsat").count();
        let complete = rows.len() == nt;
        let class = if !complete {
            "incomplete"
        } else if med <= RULE_SOLVER_MAX_S && unknown == 0 {
            "S"
        } else if med >= RULE_ENCODING_MIN_S {
            "E"
        } else {
            "M"
        };
        if complete {
            classes.push((rep, class, med));
        }
        per_rep.push(json!({
            "rep": rep, "attempts": rows.len(), "complete": complete,
            "median_cpu_s": format!("{med:.2}"),
            "min_cpu_s": format!("{:.2}", t.iter().cloned().fold(f64::INFINITY, f64::min)),
            "max_cpu_s": format!("{:.2}", t.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
            "mean_cpu_s": format!("{:.2}", t.iter().sum::<f64>() / t.len() as f64),
            "not_unsat": unknown, "class": class,
        }));
    }
    let verdict = match classes.len() {
        0 => "NO VERDICT: no repetition completed its eight arm A refutations".to_string(),
        1 => {
            let (_, c, m) = classes[0];
            if c == "S" && m <= RULE_ONE_REP_SOLVER_MAX_S {
                "SOLVER (provisional: one complete repetition, no noise floor)".into()
            } else if c == "E" && m >= RULE_ONE_REP_ENCODING_MIN_S {
                "ENCODING (provisional: one complete repetition, no noise floor)".into()
            } else {
                "INCONCLUSIVE (one complete repetition, no noise floor, outside the single-repetition margins)".into()
            }
        }
        _ => {
            let s = classes.iter().filter(|c| c.1 == "S").count();
            let e = classes.iter().filter(|c| c.1 == "E").count();
            if s == classes.len() {
                "SOLVER".into()
            } else if e == classes.len() {
                "ENCODING".into()
            } else if s > 0 && e > 0 {
                "INCONCLUSIVE (repetitions fall on both sides of the band: noise dominates)".into()
            } else {
                "INTERMEDIATE (neither anchor: the design cannot separate solver from encoding)"
                    .into()
            }
        }
    };
    json!({"thresholds_s": {"solver_max": RULE_SOLVER_MAX_S, "encoding_min": RULE_ENCODING_MIN_S,
                            "one_rep_solver_max": RULE_ONE_REP_SOLVER_MAX_S,
                            "one_rep_encoding_min": RULE_ONE_REP_ENCODING_MIN_S},
           "per_repetition": per_rep, "complete_repetitions": classes.len(), "verdict": verdict})
}

fn replay_any(
    manifest_text: &str,
    receipts_text: &str,
    sealed: Option<&str>,
    controls: Option<&str>,
) -> Result<Replayed, String> {
    let mut rp = replay_full(manifest_text, receipts_text, sealed)?;
    let is_cms = receipts_text
        .lines()
        .next()
        .is_some_and(|l| serde_json::from_str::<Value>(l).is_ok_and(|v| v["schema"] == CMS_SCHEMA));
    if is_cms {
        cms_audit(&mut rp, receipts_text, controls);
    }
    Ok(rp)
}

fn median_of<'a>(rows: impl Iterator<Item = &'a AttRow>) -> Option<f64> {
    let mut v: Vec<f64> = rows.map(|r| r.cpu_ns as f64 / 1e9).collect();
    (!v.is_empty()).then(|| median(&mut v))
}

/// Tables for the CryptoMiniSat study, next to the native study's numbers and
/// the legacy anchors. Everything comes from replayed receipts.
fn cms_tables_md(cms: &Replayed, native: Option<&Replayed>) -> String {
    let rule = &cms.json["cms"]["decision_rule"];
    let mut md = String::new();
    md.push_str("### Verdict under the preregistered rule (PROTOCOL_ADDENDUM_2.md)\n\n");
    md.push_str(&format!(
        "Thresholds on the per-repetition median CPU of the eight arm A refutations: solver if at or below {RULE_SOLVER_MAX_S} s with all eight UNSAT, encoding if at or above {RULE_ENCODING_MIN_S} s, intermediate between.\n\n"
    ));
    md.push_str("| rep | complete | median CPU s | min | max | mean | not UNSAT (of 8) | class |\n|---|---|---|---|---|---|---|---|\n");
    for r in rule["per_repetition"].as_array().unwrap() {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            r["rep"],
            r["complete"],
            r["median_cpu_s"].as_str().unwrap(),
            r["min_cpu_s"].as_str().unwrap(),
            r["max_cpu_s"].as_str().unwrap(),
            r["mean_cpu_s"].as_str().unwrap(),
            r["not_unsat"],
            r["class"].as_str().unwrap()
        ));
    }
    md.push_str(&format!(
        "\n**Verdict: {}** ({} complete repetition(s)).\n\n",
        rule["verdict"].as_str().unwrap(),
        rule["complete_repetitions"]
    ));

    md.push_str("### Arm A refutations, CPU seconds per target\n\n");
    let lm = {
        let mut v: Vec<f64> = LEGACY_A_REFUTATION_S.iter().map(|x| x.1).collect();
        median(&mut v)
    };
    md.push_str("Legacy: CryptoMiniSat 5.16.0 via pycryptosat on the legacy Python encoding, one trial. Native: the crate's `sat::Solver` on this study's encoding. CMS: the pinned binary on this study's encoding exported with `to_dimacs_xor`. Status other than `unsat` is marked; a budget or deadline value is a lower bound.\n\n");
    md.push_str("| target | legacy | native rep 1 | native rep 2 | native rep 3 | CMS rep 1 | CMS rep 2 | CMS rep 3 |\n|---|---|---|---|---|---|---|---|\n");
    let cell = |rp: Option<&Replayed>, rep: u64, ti: usize| -> String {
        let Some(rp) = rp else { return "-".into() };
        match rp
            .rows
            .iter()
            .find(|r| r.rep == rep && r.arm == "A" && r.target == ti)
        {
            None => "-".into(),
            Some(r) => {
                let mark = if r.status == "unsat" {
                    String::new()
                } else {
                    format!(" ({})", r.status)
                };
                format!("{:.1}{mark}", r.cpu_ns as f64 / 1e9)
            }
        }
    };
    for (ti, (tid, _)) in cms.targets.iter().enumerate() {
        if cms.a_decomposable[ti] {
            continue;
        }
        let legacy = LEGACY_A_REFUTATION_S
            .iter()
            .find(|x| x.0 == tid)
            .map_or("-".into(), |x| format!("{:.1}", x.1));
        md.push_str(&format!(
            "| {tid} | {legacy} | {} | {} | {} | {} | {} | {} |\n",
            cell(native, 1, ti),
            cell(native, 2, ti),
            cell(native, 3, ti),
            cell(Some(cms), 1, ti),
            cell(Some(cms), 2, ti),
            cell(Some(cms), 3, ti)
        ));
    }
    let med = |rp: Option<&Replayed>, rep: Option<u64>| -> String {
        rp.and_then(|rp| {
            median_of(rp.rows.iter().filter(|r| {
                r.arm == "A" && !rp.a_decomposable[r.target] && rep.is_none_or(|x| r.rep == x)
            }))
        })
        .map_or("-".into(), |v| format!("{v:.1}"))
    };
    md.push_str(&format!(
        "| median | {lm:.1} | {} | {} | {} | {} | {} | {} |\n",
        med(native, Some(1)),
        med(native, Some(2)),
        med(native, Some(3)),
        med(Some(cms), Some(1)),
        med(Some(cms), Some(2)),
        med(Some(cms), Some(3))
    ));

    md.push_str("\n### Paired comparison per repetition, every attempt charged\n\n");
    md.push_str("Cost per newly independent verified relation, B over A. Legacy B/A CPU: 1.61 (one trial).\n\n");
    md.push_str("| rep | CMS conflicts B/A | CMS CPU B/A | native conflicts B/A | native CPU B/A | CMS arm A CPU s | CMS arm B CPU s | CMS unknown (budget/deadline) A, B |\n|---|---|---|---|---|---|---|---|\n");
    let ratio_of = |rp: &Replayed, rep: u64| -> Option<(f64, f64)> {
        let a = rp
            .scopes
            .get(&(rep, "subgroup_xor".to_string(), "A".to_string()))?;
        let b = rp
            .scopes
            .get(&(rep, "subgroup_xor".to_string(), "B".to_string()))?;
        if a.rank == 0 || b.rank == 0 {
            return None;
        }
        let (ca, ta) = costs(a);
        let (cb, tb) = costs(b);
        Some((cb / ca, tb / ta))
    };
    let reps: BTreeSet<u64> = cms.scopes.keys().map(|k| k.0).collect();
    for rep in reps {
        let get = |arm: &str| {
            cms.scopes
                .get(&(rep, "subgroup_xor".to_string(), arm.to_string()))
        };
        let f = |o: Option<(f64, f64)>, i: usize| {
            o.map_or("-".to_string(), |x| {
                format!("{:.3}", if i == 0 { x.0 } else { x.1 })
            })
        };
        let (c, n) = (ratio_of(cms, rep), native.and_then(|n| ratio_of(n, rep)));
        let secs =
            |s: Option<&Sc>| s.map_or("-".into(), |s| format!("{:.1}", s.cpu_ns as f64 / 1e9));
        let unk = |s: Option<&Sc>| s.map_or(0, |s| s.budget + s.deadline);
        md.push_str(&format!(
            "| {rep} | {} | {} | {} | {} | {} | {} | {}, {} |\n",
            f(c, 0),
            f(c, 1),
            f(n, 0),
            f(n, 1),
            secs(get("A")),
            secs(get("B")),
            unk(get("A")),
            unk(get("B"))
        ));
    }
    md.push_str(&format!(
        "\nDirection on CPU per new relation (B/A above 1 is the legacy direction: {LEGACY_B_OVER_A_CPU}): read the CMS column above; a repetition with arm B skipped has no ratio.\n"
    ));

    md.push_str("\n### Full tables, same renderer as the native study\n\nThe block headed \"Decision rule (PROTOCOL.md)\" below applies the native study's H1 rule to this solver's data. It is exploratory here: the preregistered rule of this follow-up is the verdict at the top.\n\n");
    md.push_str(&tables_md(cms, 5));
    md
}

/// Strip plot, log scale: arm A refutation CPU seconds per target for the
/// legacy study, the native solver and CryptoMiniSat, with the two thresholds.
fn cms_svg(cms: &Replayed, native: Option<&Replayed>) -> String {
    let (w, h) = (900.0f64, 330.0f64);
    let (x0, x1) = (150.0f64, 860.0f64);
    let (lo, hi) = (3.0f64.ln(), 400.0f64.ln());
    let px = |v: f64| x0 + (v.clamp(3.0, 400.0).ln() - lo) / (hi - lo) * (x1 - x0);
    let mut s = String::new();
    s.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" role=\"img\" aria-labelledby=\"t d\" font-family=\"sans-serif\" font-size=\"12\">\n"));
    s.push_str(
        "<title id=\"t\">Arm A refutation CPU per target: legacy, native, CryptoMiniSat</title>\n",
    );
    s.push_str("<desc id=\"d\">One dot per arm A UNSAT attempt on a log CPU-seconds axis; vertical lines mark the preregistered 45 s and 80 s thresholds. n = 13 stage diagnostic.</desc>\n");
    s.push_str("<style>\nsvg{--surface:#fcfcfb;--ink:#0b0b0b;--ink2:#52514e;--grid:#e3e2de;--a:#2a78d6;--b:#eb6834;--c:#3a8f5b}\n@media (prefers-color-scheme: dark){svg{--surface:#1a1a19;--ink:#f0efec;--ink2:#c3c2b7;--grid:#3a3935;--a:#3987e5;--b:#d95926;--c:#4fb07a}}\n.bg{fill:var(--surface)}.ink{fill:var(--ink)}.ink2{fill:var(--ink2)}.gl{stroke:var(--grid);stroke-width:1}.th{stroke:var(--ink2);stroke-width:1;stroke-dasharray:4 3}.da{fill:var(--a)}.db{fill:var(--b)}.dc{fill:var(--c)}\n</style>\n");
    s.push_str(&format!(
        "<rect class=\"bg\" width=\"{w}\" height=\"{h}\"/>\n"
    ));
    s.push_str("<text class=\"ink\" x=\"24\" y=\"28\" font-size=\"16\" font-weight=\"bold\">Arm A refutations: CPU seconds per UNSAT attempt</text>\n");
    s.push_str("<text class=\"ink2\" x=\"24\" y=\"48\">Same eight targets (a0 to a3, c0 to c3). Log axis. Unfinished attempts (budget or deadline) are drawn at the CPU they used.</text>\n");
    for t in [5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 400.0] {
        let x = px(t);
        s.push_str(&format!("<line class=\"gl\" x1=\"{x:.1}\" y1=\"70\" x2=\"{x:.1}\" y2=\"250\"/><text class=\"ink2\" x=\"{x:.1}\" y=\"268\" text-anchor=\"middle\">{t:.0}</text>\n"));
    }
    for (v, lab) in [(RULE_SOLVER_MAX_S, "45 s"), (RULE_ENCODING_MIN_S, "80 s")] {
        let x = px(v);
        s.push_str(&format!("<line class=\"th\" x1=\"{x:.1}\" y1=\"70\" x2=\"{x:.1}\" y2=\"250\"/><text class=\"ink2\" x=\"{x:.1}\" y=\"66\" text-anchor=\"middle\">{lab}</text>\n"));
    }
    s.push_str("<text class=\"ink2\" x=\"505\" y=\"290\" text-anchor=\"middle\">CPU seconds per attempt (log scale)</text>\n");
    let ys = [105.0f64, 160.0, 215.0];
    let mut rows: Vec<(&str, &str, Vec<f64>)> = Vec::new();
    rows.push((
        "Legacy (CMS via Python)",
        "da",
        LEGACY_A_REFUTATION_S.iter().map(|x| x.1).collect(),
    ));
    if let Some(n) = native {
        rows.push((
            "Native sat::Solver",
            "db",
            n.rows
                .iter()
                .filter(|r| r.arm == "A" && !n.a_decomposable[r.target])
                .map(|r| r.cpu_ns as f64 / 1e9)
                .collect(),
        ));
    }
    rows.push((
        "CryptoMiniSat 5.16.0",
        "dc",
        cms.rows
            .iter()
            .filter(|r| r.arm == "A" && !cms.a_decomposable[r.target])
            .map(|r| r.cpu_ns as f64 / 1e9)
            .collect(),
    ));
    for (i, (label, class, vals)) in rows.iter().enumerate() {
        let y = ys[i];
        s.push_str(&format!(
            "<text class=\"ink\" x=\"24\" y=\"{:.0}\">{label}</text>\n",
            y + 4.0
        ));
        for (k, v) in vals.iter().enumerate() {
            let jitter = (k % 3) as f64 * 7.0 - 7.0;
            s.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\" fill-opacity=\"0.8\"/>\n", px(*v), y + jitter));
        }
    }
    s.push_str("<text class=\"ink2\" x=\"24\" y=\"314\">Scope: stage diagnostic on a toy field, one solver build, 8 refutations per repetition. No S, no ratio to rho, no ECDLP speedup.</text>\n");
    s.push_str("</svg>\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: phase_paired_replay RECEIPTS.jsonl OUT.json [--manifest M] [--sealed S] [--tables T.md] [--svg F.svg]\n       CryptoMiniSat receipts also take [--controls C.jsonl] [--native NATIVE_RECEIPTS.jsonl]");
        std::process::exit(64);
    }
    let mut manifest = DEFAULT_MANIFEST.to_string();
    let mut sealed = None;
    let mut tables = None;
    let mut svg = None;
    let mut controls = None;
    let mut native = None;
    let mut i = 2;
    while i + 1 < args.len() {
        match args[i].as_str() {
            "--manifest" => manifest = args[i + 1].clone(),
            "--sealed" => sealed = Some(args[i + 1].clone()),
            "--tables" => tables = Some(args[i + 1].clone()),
            "--svg" => svg = Some(args[i + 1].clone()),
            "--controls" => controls = Some(args[i + 1].clone()),
            "--native" => native = Some(args[i + 1].clone()),
            _ => {}
        }
        i += 2;
    }
    let mtext = fs::read_to_string(&manifest).expect("manifest");
    if manifest == DEFAULT_MANIFEST && hexs(&sha256(mtext.as_bytes())) != FROZEN_MANIFEST_SHA {
        eprintln!("frozen manifest changed");
        std::process::exit(3);
    }
    let rtext = fs::read_to_string(&args[0]).expect("receipts");
    let stext = sealed.map(|p| fs::read_to_string(p).expect("sealed"));
    let ctext = controls.map(|p| fs::read_to_string(p).expect("controls"));
    let rp = replay_any(&mtext, &rtext, stext.as_deref(), ctext.as_deref()).unwrap_or_else(|e| {
        eprintln!("replay failed: {e}");
        std::process::exit(3);
    });
    let is_cms = rp.json["schema"] == "phase_paired_cms/replay/v1";
    let native_rp = native.map(|p| {
        let nt = fs::read_to_string(p).expect("native receipts");
        replay_full(&mtext, &nt, stext.as_deref()).unwrap_or_else(|e| {
            eprintln!("native replay failed: {e}");
            std::process::exit(3);
        })
    });
    let out = &rp.json;
    let defects = out["defects"].as_array().map(|d| d.len()).unwrap_or(1);
    fs::write(&args[1], serde_json::to_string_pretty(&out).unwrap() + "\n").expect("write");
    println!(
        "{}: {} receipts, {} defects",
        args[1], out["receipts"], defects
    );
    for d in out["defects"].as_array().unwrap() {
        eprintln!("DEFECT {d}");
    }
    if defects == 0 {
        // Tables and chart are rendered from verified receipts only.
        if let Some(p) = tables {
            let md = if is_cms {
                cms_tables_md(&rp, native_rp.as_ref())
            } else {
                tables_md(&rp, 5)
            };
            fs::write(p, md).expect("write tables");
        }
        if let Some(p) = svg {
            let s = if is_cms {
                cms_svg(&rp, native_rp.as_ref())
            } else {
                svg_chart(&rp)
            };
            fs::write(p, s).expect("write svg");
        }
    }
    std::process::exit(if defects == 0 { 0 } else { 2 });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest_text() -> String {
        fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/research/phase_paired_native_20261026/fixtures/manifest.json"
        ))
        .expect("fixture manifest")
    }

    /// Four signed points with abscissae in `pool` (index quadruple) summing to
    /// the target, found by trying the 16 sign vectors with the group law.
    fn planted(w: &World, target: usize, phases: bool) -> Option<Vec<(u64, u64)>> {
        let pts = w.pool(phases);
        let rp = w.targets[target].2;
        let quad = *w.decompositions(phases)[target].iter().next()?;
        let q: Vec<(u64, u64)> = quad.iter().map(|i| pts[*i]).collect();
        for s in 0u32..16 {
            let mut sum: P = None;
            let mut signed = Vec::new();
            for (i, p) in q.iter().enumerate() {
                let t = if (s >> i) & 1 == 1 {
                    w.neg(Some(*p))
                } else {
                    Some(*p)
                };
                signed.push(t.unwrap());
                sum = w.add(sum, t);
            }
            if sum == Some(rp) {
                return Some(signed);
            }
        }
        None
    }

    /// A receipts file with one full ABBA repetition of one formulation whose
    /// statuses are the exhaustive truth (no solver involved).
    fn synthetic(w: &World) -> String {
        let ca = w.decomposition_counts(false);
        let cb = w.decomposition_counts(true);
        let mut lines = vec![json!({"type": "header", "manifest_sha256": w.sha,
                                    "conflict_budget": 1000u64, "forms": ["subgroup_xor"]})];
        for (n, (rep, form, pair, pos, arm, tid)) in schedule(w, &["subgroup_xor".to_string()], 1)
            .into_iter()
            .enumerate()
        {
            let ti = pair;
            let arm_b = arm == "B";
            let dec = if arm_b { cb[ti] > 0 } else { ca[ti] > 0 };
            let t = w.targets[ti].2;
            let mut v = json!({"type": "attempt", "seq": n, "rep": rep, "form": form,
                "pair": pair, "position": pos, "arm": arm, "phases": arm_b, "set": w.targets[ti].1,
                "target_id": tid, "target": [format!("{:#x}", t.0), format!("{:#x}", t.1)],
                "conflicts": 10, "decisions": 20, "propagations": 30, "word_ops": 40,
                "rejected_tuples": 0, "total_cpu_ns": 1_000_000, "total_wall_ns": 1_100_000});
            if dec {
                let pts = planted(w, ti, arm_b).expect("exhaustive says decomposable");
                v["status"] = json!("verified");
                v["found_points"] = json!(pts
                    .iter()
                    .map(|p| [format!("{:#x}", p.0), format!("{:#x}", p.1)])
                    .collect::<Vec<_>>());
                v["solver_summands"] = json!(pts
                    .iter()
                    .map(|p| {
                        let (j, k) = w.orbit[&p.0];
                        json!({"v": format!("{:#x}", w.reps[j].0), "k": k})
                    })
                    .collect::<Vec<_>>());
            } else {
                v["status"] = json!("unsat");
            }
            lines.push(v);
        }
        lines
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    #[test]
    fn frozen_structure_matches_legacy() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        assert_eq!(w.sha, FROZEN_MANIFEST_SHA);
        assert_eq!(w.lam, 89, "legacy lambda");
        assert_eq!(w.r, 2003);
        let ca = w.decomposition_counts(false);
        let cb = w.decomposition_counts(true);
        let a: Vec<&str> = w
            .targets
            .iter()
            .zip(&ca)
            .filter(|(_, c)| **c > 0)
            .map(|(t, _)| t.0.as_str())
            .collect();
        assert_eq!(
            a,
            ["b0", "b1", "b2", "b3"],
            "arm A decomposes only the b targets"
        );
        assert!(cb.iter().all(|c| *c > 0), "arm B decomposes all 12");
    }

    #[test]
    fn clean_synthetic_receipts_replay_without_defects() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let out = replay(&manifest_text(), &synthetic(&w), None).unwrap();
        assert_eq!(
            out["defects"].as_array().unwrap().len(),
            0,
            "{}",
            out["defects"]
        );
        let ranks: Vec<(String, u64)> = out["scopes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| {
                (
                    s["arm"].as_str().unwrap().to_string(),
                    s["rank"].as_u64().unwrap(),
                )
            })
            .collect();
        assert!(ranks.contains(&("A".to_string(), 4)), "{ranks:?}");
        assert!(ranks.contains(&("B".to_string(), 5)), "{ranks:?}");
    }

    fn tampered(text: &str, f: impl Fn(&mut Value, &World)) -> Value {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let mut done = false;
        let lines: Vec<String> = text
            .lines()
            .map(|l| {
                let mut v: Value = serde_json::from_str(l).unwrap();
                if !done && v["type"] == "attempt" && v["status"] == "verified" && v["arm"] == "B" {
                    f(&mut v, &w);
                    done = true;
                }
                v.to_string()
            })
            .collect();
        replay(&manifest_text(), &(lines.join("\n") + "\n"), None).unwrap()
    }

    #[test]
    fn replay_rejects_tampered_receipts() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let text = synthetic(&w);
        // 1. a different ordinate
        let out = tampered(&text, |v, _| v["found_points"][0][1] = json!("0x1"));
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "wrong y accepted"
        );
        // 2. a point replaced by its negation is still a valid point but changes the sum
        let out = tampered(&text, |v, w| {
            let x = unhex(v["found_points"][0][0].as_str().unwrap()).unwrap();
            let y = unhex(v["found_points"][0][1].as_str().unwrap()).unwrap();
            let n = w.neg(Some((x, y))).unwrap();
            v["found_points"][0] = json!([format!("{:#x}", n.0), format!("{:#x}", n.1)]);
        });
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "flipped sign accepted"
        );
        // 3. a verified receipt relabelled with another target's coordinates
        let out = tampered(&text, |v, _| v["target_id"] = json!("c3"));
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "wrong target accepted"
        );
        // 4. a false UNSAT on a decomposable target
        let out = tampered(&text, |v, _| {
            v["status"] = json!("unsat");
            v.as_object_mut().unwrap().remove("found_points");
        });
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "false UNSAT accepted"
        );
        // 5. a status dressed as budget exhaustion below the budget
        let out = tampered(&text, |v, _| v["status"] = json!("budget"));
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "early budget accepted"
        );
        // 6. solver summands that do not describe the found points
        let out = tampered(&text, |v, _| v["solver_summands"][0]["k"] = json!(12));
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "wrong summands accepted"
        );
        // 7. a different manifest hash in the header
        let altered = text.replacen(&w.sha, &"0".repeat(64), 1);
        let out = replay(&manifest_text(), &altered, None).unwrap();
        assert!(
            !out["defects"].as_array().unwrap().is_empty(),
            "foreign manifest accepted"
        );
    }

    #[test]
    fn arm_a_receipt_with_a_phase_shift_is_rejected() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let text = synthetic(&w);
        // Put an arm B (phase-shifted) decomposition of a0 into an arm A receipt.
        let b_pts = planted(&w, 0, true).unwrap();
        let lines: Vec<String> = text
            .lines()
            .map(|l| {
                let mut v: Value = serde_json::from_str(l).unwrap();
                if v["type"] == "attempt" && v["arm"] == "A" && v["target_id"] == "a0" {
                    v["status"] = json!("verified");
                    v["found_points"] = json!(b_pts
                        .iter()
                        .map(|p| [format!("{:#x}", p.0), format!("{:#x}", p.1)])
                        .collect::<Vec<_>>());
                    v["solver_summands"] = json!(b_pts
                        .iter()
                        .map(|p| {
                            let (j, k) = w.orbit[&p.0];
                            json!({"v": format!("{:#x}", w.reps[j].0), "k": k})
                        })
                        .collect::<Vec<_>>());
                }
                v.to_string()
            })
            .collect();
        let out = replay(&manifest_text(), &(lines.join("\n") + "\n"), None).unwrap();
        let d = out["defects"].as_array().unwrap();
        assert!(
            d.iter()
                .any(|x| x.as_str().unwrap().contains("arm A receipt uses phase")),
            "{d:?}"
        );
    }

    #[test]
    fn superseded_skipped_receipts_are_accepted() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let text = synthetic(&w);
        let mut lines: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let n_att = lines.len() - 1; // header + attempts
        let real: Vec<Value> = lines[lines.len() - 4..].to_vec();
        for l in lines.iter_mut().rev().take(4) {
            l["status"] = json!("skipped");
            let o = l.as_object_mut().unwrap();
            o.remove("found_points");
            o.remove("solver_summands");
        }
        let orphan_only = lines.clone();
        for (k, mut r) in real.into_iter().enumerate() {
            r["seq"] = json!(n_att + k);
            lines.push(r);
        }
        let join = |v: &[Value]| {
            v.iter()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        };
        let out = replay(&manifest_text(), &join(&lines), None).unwrap();
        assert_eq!(
            out["defects"].as_array().unwrap().len(),
            0,
            "{}",
            out["defects"]
        );
        assert_eq!(out["superseded_skipped"], 4);
        assert_eq!(out["receipts"], n_att);
        // Skipped receipts with nothing after them are not superseded and still replay.
        let out = replay(&manifest_text(), &join(&orphan_only), None).unwrap();
        assert_eq!(
            out["defects"].as_array().unwrap().len(),
            0,
            "{}",
            out["defects"]
        );
        assert_eq!(out["superseded_skipped"], 0);
        // A real receipt appended out of the schedule order is a defect.
        let mut bad = lines.clone();
        let last = bad.pop().unwrap();
        bad.insert(1, last);
        let out = replay(&manifest_text(), &join(&bad), None).unwrap();
        assert!(!out["defects"].as_array().unwrap().is_empty());
    }

    #[test]
    fn rank_elimination_matches_hand_cases() {
        let mut rk = Rank {
            r: 2003,
            rows: Vec::new(),
        };
        assert!(rk.insert(&[1, 2, 3, 4, 5]));
        assert!(rk.insert(&[0, 1, 0, 0, 0]));
        assert!(!rk.insert(&[2, 5, 6, 8, 10]), "2*row1 + row2 is dependent");
        assert!(rk.insert(&[0, 0, 1, 0, 0]));
        assert_eq!(rk.rows.len(), 3);
    }

    // ---- CryptoMiniSat follow-up (PROTOCOL_ADDENDUM_2.md) ----

    /// CryptoMiniSat receipts whose statuses are the exhaustive truth: arm A
    /// refutations cost `a_secs[rep - 1]` seconds of child CPU, every other
    /// attempt one second. Stage records follow the preregistered rule.
    fn cms_synthetic(w: &World, a_secs: &[f64]) -> Vec<Value> {
        let ca = w.decomposition_counts(false);
        let cb = w.decomposition_counts(true);
        let cap = 3600.0;
        let mut lines = vec![json!({"type": "header", "schema": CMS_SCHEMA,
            "manifest_sha256": w.sha, "conflict_budget": 3_000_000u64, "forms": ["subgroup_xor"],
            "maxtime_cpu_s": 120.0, "cpu_cap_s": cap, "cpu_prior_s": 0.0, "controls_cpu_s": 0.0,
            "stage_guard": 1.1, "cms_binary_sha256": CMS_BINARY_SHA, "cms_tag": "release/v5.16.0",
            "cms_commit": "x", "cadical_commit": "y", "cadiback_commit": "z"})];
        let sched = schedule(w, &["subgroup_xor".to_string()], a_secs.len() as u64);
        let mut spent = 0.0;
        let mut seq = 0;
        let mut rep_cost: Vec<f64> = Vec::new();
        for (rep, form, pair, pos, arm, tid) in sched {
            if pair == 0 && pos == 0 {
                let remaining: f64 = cap - spent;
                let full = rep_cost
                    .iter()
                    .cloned()
                    .fold(None::<f64>, |m, x| Some(m.map_or(x, |y| y.max(x))));
                let a_of = |r: u64| -> Option<f64> {
                    // arm A cost of repetition r: 8 refutations plus 4 one-second relations
                    (r < rep).then(|| 8.0 * a_secs[r as usize - 1] + 4.0)
                };
                let ac = (1..rep)
                    .filter_map(a_of)
                    .fold(None::<f64>, |m, x| Some(m.map_or(x, |y| y.max(x))));
                let mode = rep_mode(rep, remaining, 1.1, full, ac);
                lines.push(json!({"type": "stage", "rep": rep, "mode": mode,
                                  "remaining_cpu_s": (remaining * 1000.0).round() / 1000.0}));
                assert_eq!(mode, "full", "synthetic receipts assume roomy repetitions");
            }
            let ti = pair;
            let arm_b = arm == "B";
            let dec = if arm_b { cb[ti] > 0 } else { ca[ti] > 0 };
            let t = w.targets[ti].2;
            let secs = if !arm_b && !dec {
                a_secs[rep as usize - 1]
            } else {
                1.0
            };
            let child = (secs * 1e9) as u64;
            let (status, exit, line) = if dec {
                ("verified", 10, "SATISFIABLE")
            } else {
                ("unsat", 20, "UNSATISFIABLE")
            };
            let mut v = json!({"type": "attempt", "seq": seq, "rep": rep, "form": form,
                "pair": pair, "position": pos, "arm": arm, "phases": arm_b, "set": w.targets[ti].1,
                "target_id": tid, "target": [format!("{:#x}", t.0), format!("{:#x}", t.1)],
                "status": status, "solver": "cryptominisat5", "solver_calls": 1,
                "conflicts": 10, "decisions": 20, "propagations": 30, "word_ops": 0,
                "rejected_tuples": 0, "solve_cpu_ns": child, "total_cpu_ns": child + 1_000_000,
                "total_wall_ns": child + 2_000_000,
                "cms": {"child_cpu_ns": child, "error": null, "calls": [
                    {"exit_code": exit, "status_line": line, "child_cpu_ns": child,
                     "killed_at_wall_backstop": false, "signal": null}]}});
            if dec {
                let pts = planted(w, ti, arm_b).expect("exhaustive says decomposable");
                v["found_points"] = json!(pts
                    .iter()
                    .map(|p| [format!("{:#x}", p.0), format!("{:#x}", p.1)])
                    .collect::<Vec<_>>());
                v["solver_summands"] = json!(pts
                    .iter()
                    .map(|p| {
                        let (j, k) = w.orbit[&p.0];
                        json!({"v": format!("{:#x}", w.reps[j].0), "k": k})
                    })
                    .collect::<Vec<_>>());
            }
            spent += secs + 0.001;
            seq += 1;
            lines.push(v);
            if pair == w.targets.len() - 1 && pos == 1 {
                rep_cost.push(8.0 * a_secs[rep as usize - 1] + 4.0 + 12.0);
            }
        }
        lines
    }

    fn join(v: &[Value]) -> String {
        v.iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    fn cms_verdict_of(a_secs: &[f64]) -> (String, usize) {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let lines = cms_synthetic(&w, a_secs);
        let rp = replay_any(&manifest_text(), &join(&lines), None, None).unwrap();
        (
            rp.json["cms"]["decision_rule"]["verdict"]
                .as_str()
                .unwrap()
                .to_string(),
            rp.json["defects"].as_array().unwrap().len(),
        )
    }

    #[test]
    fn cms_decision_rule_classifies_the_preregistered_cases() {
        let (v, d) = cms_verdict_of(&[20.0]);
        assert_eq!(d, 0);
        assert!(v.starts_with("SOLVER (provisional"), "{v}");
        assert!(cms_verdict_of(&[40.0])
            .0
            .starts_with("INCONCLUSIVE (one complete"));
        assert!(cms_verdict_of(&[130.0])
            .0
            .starts_with("ENCODING (provisional"));
        assert!(cms_verdict_of(&[100.0])
            .0
            .starts_with("INCONCLUSIVE (one complete"));
        assert_eq!(cms_verdict_of(&[20.0, 44.0]), ("SOLVER".to_string(), 0));
        assert_eq!(cms_verdict_of(&[90.0, 200.0]), ("ENCODING".to_string(), 0));
        assert!(cms_verdict_of(&[60.0, 70.0]).0.starts_with("INTERMEDIATE"));
        assert!(cms_verdict_of(&[20.0, 100.0])
            .0
            .starts_with("INCONCLUSIVE (repetitions"));
        // Just under 45 s (the receipts add a millisecond of parent CPU) is solver, 45.2 s is not.
        assert_eq!(cms_verdict_of(&[44.99, 44.99]).0, "SOLVER");
        assert!(cms_verdict_of(&[45.2, 45.2]).0.starts_with("INTERMEDIATE"));
        assert_eq!(cms_verdict_of(&[80.0, 80.0]).0, "ENCODING");
    }

    #[test]
    fn cms_unfinished_refutation_blocks_the_solver_verdict() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let mut lines = cms_synthetic(&w, &[20.0, 20.0]);
        // One arm A refutation in repetition 2 ends at the deadline: it is unknown.
        let mut done = false;
        for l in lines.iter_mut() {
            if !done
                && l["type"] == "attempt"
                && l["rep"] == 2
                && l["arm"] == "A"
                && l["status"] == "unsat"
            {
                l["status"] = json!("deadline");
                l["cms"]["calls"][0]["exit_code"] = json!(15);
                l["cms"]["calls"][0]["status_line"] = json!("INDETERMINATE");
                let child = 120_000_000_000u64;
                l["cms"]["child_cpu_ns"] = json!(child);
                l["cms"]["calls"][0]["child_cpu_ns"] = json!(child);
                l["solve_cpu_ns"] = json!(child);
                l["total_cpu_ns"] = json!(child + 1_000_000);
                done = true;
            }
        }
        let rp = replay_any(&manifest_text(), &join(&lines), None, None).unwrap();
        assert_eq!(
            rp.json["defects"].as_array().unwrap().len(),
            0,
            "{}",
            rp.json["defects"]
        );
        let v = rp.json["cms"]["decision_rule"]["verdict"].as_str().unwrap();
        assert!(
            v.starts_with("INTERMEDIATE") || v.starts_with("INCONCLUSIVE"),
            "{v}"
        );
    }

    #[test]
    fn cms_audit_rejects_what_the_protocol_forbids() {
        let w = World::from_manifest(&manifest_text()).unwrap();
        let base = cms_synthetic(&w, &[20.0, 20.0]);
        let defects = |f: &dyn Fn(&mut Vec<Value>)| -> Vec<String> {
            let mut l = base.clone();
            f(&mut l);
            let rp = replay_any(&manifest_text(), &join(&l), None, None).unwrap();
            rp.json["defects"]
                .as_array()
                .unwrap()
                .iter()
                .map(|d| d.to_string())
                .collect()
        };
        assert!(defects(&|_| {}).is_empty());
        // 1. a different binary
        assert!(!defects(&|l| l[0]["cms_binary_sha256"] = json!("0".repeat(64))).is_empty());
        // 2. the cap exceeded
        assert!(!defects(&|l| l[0]["cpu_cap_s"] = json!(100.0)).is_empty());
        // 3. a stage record that does not follow the rule
        assert!(!defects(&|l| {
            let s = l
                .iter_mut()
                .filter(|x| x["type"] == "stage")
                .nth(1)
                .unwrap();
            s["mode"] = json!("none");
        })
        .is_empty());
        // 4. a recorded remaining allowance that the receipts do not give
        assert!(!defects(&|l| {
            let s = l
                .iter_mut()
                .filter(|x| x["type"] == "stage")
                .nth(1)
                .unwrap();
            s["remaining_cpu_s"] = json!(5.0);
        })
        .is_empty());
        // 5. an UNSAT that carries the SAT exit code
        assert!(!defects(&|l| {
            let a = l.iter_mut().find(|x| x["status"] == "unsat").unwrap();
            a["cms"]["calls"][0]["exit_code"] = json!(10);
        })
        .is_empty());
        // 6. a deadline below the CPU backstop
        assert!(!defects(&|l| {
            let a = l.iter_mut().find(|x| x["status"] == "unsat").unwrap();
            a["status"] = json!("deadline");
            a["cms"]["calls"][0]["exit_code"] = json!(15);
            a["cms"]["calls"][0]["status_line"] = json!("INDETERMINATE");
        })
        .is_empty());
        // 7. child CPU that does not add up
        assert!(!defects(&|l| {
            let a = l.iter_mut().find(|x| x["type"] == "attempt").unwrap();
            a["cms"]["child_cpu_ns"] = json!(5u64);
        })
        .is_empty());
        // 8. an instrument error recorded
        assert!(!defects(&|l| {
            let a = l.iter_mut().find(|x| x["type"] == "attempt").unwrap();
            a["cms"]["error"] = json!("model does not satisfy the exported instance");
        })
        .is_empty());
        // 9. a false UNSAT on a decomposable target (the exhaustive check wins)
        assert!(!defects(&|l| {
            let a = l
                .iter_mut()
                .find(|x| x["status"] == "verified" && x["arm"] == "B")
                .unwrap();
            a["status"] = json!("unsat");
            a["cms"]["calls"][0]["exit_code"] = json!(20);
            a["cms"]["calls"][0]["status_line"] = json!("UNSATISFIABLE");
        })
        .is_empty());
    }
}
