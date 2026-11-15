//! Saved-stream deterministic scheduler reconstruction. No collector is invoked.
use five_leads_study::three_leads::{Divisor, Scholten};
use serde_json::{json, Value};
use signed_collector_replay::Limits;
use std::collections::{BTreeMap, BTreeSet};
fn need(ok: bool, msg: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn n(v: &Value, k: &str) -> u64 {
    v[k].as_u64().expect("strict replay checked integer")
}
fn divisor(v: &Value) -> Divisor {
    serde_json::from_value(v.clone()).expect("label checker checked divisor")
}
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
struct Random {
    state: u64,
    draws: u64,
}
impl Random {
    fn next(&mut self) -> u64 {
        self.draws += 1;
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        mix(self.state)
    }
    fn uniform(&mut self, r: u64) -> u64 {
        let bound = u64::MAX - u64::MAX % r;
        loop {
            let x = self.next();
            if x < bound {
                return x % r;
            }
        }
    }
}
/// Uses affine Cantor for transition arithmetic, including projective-labelled streams.
/// Connectivity is reconstructed by relabelling all members, not native union-find.
/// Rank priorities rely on the prior strict graph replay's checked rank_score.
pub fn verify(input: &[u8], header: &Value, limits: Limits) -> Result<Value, String> {
    let checked = signed_collector_label_check::verify(input, header, limits)?;
    need(
        usize::BITS == 64,
        "scheduler requires original 64-bit usize semantics",
    )?;
    let s = Scholten::new(n(header, "p"));
    let r = n(header, "r");
    let pg = divisor(&header["transferred_generator"]);
    let tq = divisor(&header["transferred_target"]);
    let mut rng = Random {
        state: n(header, "seed"),
        draws: 0,
    };
    let combine = |a, b| s.j.add(s.j.mul(pg, a), s.j.mul(tq, b));
    let jumps: Vec<_> = (0..16)
        .map(|_| {
            let a = rng.uniform(r.min(4096));
            let b = rng.uniform(r.min(4096));
            (combine(a, b), a, b)
        })
        .collect();
    let (mut a, mut b) = (rng.uniform(r), rng.uniform(r));
    let mut d = combine(a, b);
    let mut seen = BTreeSet::new();
    let mut group = 0;
    let mut indices = [0usize; 2];
    let mut offset = 0;
    let mut choices = 0;
    let mut restart_count = 0;
    let mut candidates = 0;
    let mut component = vec![0usize];
    let mut states = BTreeMap::new();
    let mut best: Option<((bool, usize), u64)> = None;
    let mut rows = vec![];
    for line in std::str::from_utf8(input)
        .map_err(|e| e.to_string())?
        .lines()
    {
        let rec: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let e = &rec["event"];
        let kind = e["kind"].as_str().ok_or("kind")?;
        let walk_restart = kind == "restart" && e["reason"] == "walk_duplicate";
        if (kind == "candidate" || walk_restart) && n(e, "continuation") != group {
            group = n(e, "continuation");
            offset = 0;
            states.clear();
            best = None;
            let repeated = !seen.insert(d);
            need(repeated == walk_restart, "walk membership/restart mismatch")?;
            if walk_restart {
                seen.clear();
                a = rng.uniform(r);
                b = rng.uniform(r);
                d = combine(a, b);
            }
            let key = mix(d.u[0]
                ^ d.u[1].rotate_left(19)
                ^ d.v[0].rotate_left(37)
                ^ d.v[1].rotate_left(7));
            indices = [
                key as usize % 16,
                (key as usize % 16 + 1 + rng.next() as usize % 15) % 16,
            ];
        }
        match kind {
            "candidate" => {
                need(offset < 2, "too many candidates")?;
                let index = indices[offset];
                offset += 1;
                need(
                    n(e, "jump_index") as usize == index,
                    "deterministic jump index mismatch",
                )?;
                let jump = jumps[index];
                let na = (a + jump.1) % r;
                let nb = (b + jump.2) % r;
                let nd = s.j.add(d, jump.0);
                need(
                    na == n(e, "a") && nb == n(e, "b") && nd == divisor(&e["divisor"]),
                    "candidate transition mismatch",
                )?;
                states.insert(n(e, "attempt"), (nd, na, nb));
                candidates += 1;
            }
            "node" => {
                let id = n(e, "node") as usize;
                need(id == component.len(), "component node order")?;
                component.push(id);
            }
            "edge" | "identity" | "nonsplit" | "duplicate" => {
                let mut score = (false, 0usize);
                if kind == "edge" {
                    let u = e["nodes"][0].as_u64().ok_or("node")? as usize;
                    let v = e["nodes"][1].as_u64().ok_or("node")? as usize;
                    if header["rank_policy"] == true {
                        let rank = n(e, "rank_score") as usize;
                        score = (rank > 0, rank);
                    } else {
                        let x = component[u];
                        let y = component[v];
                        let size = component.iter().filter(|&&c| c == x || c == y).count();
                        score = (x == y, size);
                        for c in &mut component {
                            if *c == y {
                                *c = x;
                            }
                        }
                    }
                }
                let attempt = n(e, "attempt");
                if best.is_none_or(|(old, _)| score > old) {
                    best = Some((score, attempt));
                }
            }
            "choice" => {
                let selected = n(e, "selected_attempt");
                need(
                    best.map(|(_, i)| i) == Some(selected),
                    "first-maximal choice mismatch",
                )?;
                let next = states.get(&selected).ok_or("selected state")?;
                d = next.0;
                a = next.1;
                b = next.2;
                choices += 1;
                rows.push(json!({"continuation":group,"selected_attempt":selected,"priority":best.unwrap().0,"matches":true}));
            }
            "restart" => {
                if !walk_restart {
                    seen.clear();
                    a = rng.uniform(r);
                    b = rng.uniform(r);
                    d = combine(a, b);
                }
                need(
                    a == n(e, "a") && b == n(e, "b") && d == divisor(&e["divisor"]),
                    "restart draw/state mismatch",
                )?;
                restart_count += 1;
            }
            _ => {}
        }
    }
    need(
        candidates == checked.candidate_equations,
        "candidate count disagreement",
    )?;
    Ok(
        json!({"schema":"signed.scheduler.check/v1","candidates":candidates,"choices":choices,"restarts":restart_count,"random_draws":rng.draws,"choice_rows":rows,"source_scalar_verified":checked.source_scalar_verified,"collector_invoked":false,"independent_arithmetic":false,"independent_review":"pending"}),
    )
}
