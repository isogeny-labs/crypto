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
                let order = if (i as u64 + rep - 1) % 2 == 0 {
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

fn ratio(num: u64, den: usize) -> Value {
    if den == 0 {
        Value::Null
    } else {
        json!(format!("{:.3}", num as f64 / den as f64))
    }
}

/// Replays receipts text against manifest text. `Ok` carries the canonical
/// output; its `defects` list is empty exactly when every check passed.
fn replay(manifest_text: &str, receipts_text: &str, sealed: Option<&str>) -> Result<Value, String> {
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
    let atts: Vec<&Rec> = recs.iter().filter(|r| r.s("type") == "attempt").collect();
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
    let seqs: Vec<u64> = atts.iter().map(|a| a.u("seq")).collect();
    if seqs != (0..atts.len() as u64).collect::<Vec<_>>() {
        defects.push("seq numbers are not 0..n-1 in order".into());
    }

    #[derive(Default)]
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
    let mut ranks: BTreeMap<(u64, String, String), Rank> = BTreeMap::new();
    let mut sc: BTreeMap<(u64, String, String), Sc> = BTreeMap::new();
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
        if a.u("conflicts") > budget {
            defects.push(format!(
                "{tag}: {} conflicts exceed the budget {budget}",
                a.u("conflicts")
            ));
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
    Ok(json!({
        "schema": "phase_paired_native/replay/v1",
        "manifest_sha256": w.sha,
        "conflict_budget": budget,
        "lambda": w.lam, "r": w.r,
        "receipts": atts.len(),
        "structural_exhaustive": structural,
        "arm_a_decomposable": ca.iter().filter(|c| **c > 0).count(),
        "arm_b_decomposable": cb.iter().filter(|c| **c > 0).count(),
        "scopes": scopes,
        "ratios": ratios,
        "h1_criterion_met_all_forms_all_reps": h1,
        "defects": defects,
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: phase_paired_replay RECEIPTS.jsonl OUT.json [--manifest M] [--sealed S]");
        std::process::exit(64);
    }
    let mut manifest = DEFAULT_MANIFEST.to_string();
    let mut sealed = None;
    let mut i = 2;
    while i + 1 < args.len() {
        match args[i].as_str() {
            "--manifest" => manifest = args[i + 1].clone(),
            "--sealed" => sealed = Some(args[i + 1].clone()),
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
    let out = replay(&mtext, &rtext, stext.as_deref()).unwrap_or_else(|e| {
        eprintln!("replay failed: {e}");
        std::process::exit(3);
    });
    let defects = out["defects"].as_array().map(|d| d.len()).unwrap_or(1);
    fs::write(&args[1], serde_json::to_string_pretty(&out).unwrap() + "\n").expect("write");
    println!(
        "{}: {} receipts, {} defects",
        args[1], out["receipts"], defects
    );
    for d in out["defects"].as_array().unwrap() {
        eprintln!("DEFECT {d}");
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
}
