//! Original-base adjacent-coset pairs anchored at supplied old columns.
//! Partial loop counters are not a complete cost, yield or attack exponent.
pub mod controls;
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use torus_column_transport::{column, neg, Family, Key};
use torus_coset_circuit::{aggregation::TargetCircuit, index::IndexTable, Spec, WitnessInput};
use torus_reverse_multiplier::{fp2_json, point_json, raw_choices};
use torus_s4::Point;
use torus_s4_tree::Fp2;
pub type Result<T> = std::result::Result<T, String>;

fn key(z: &Fp2) -> Key {
    (z.a.clone(), z.b.clone())
}
#[derive(Default)]
pub struct Counts {
    pub old_admission_checks: usize,
    pub old_visits: u64,
    pub sign_visits: u64,
    pub inverse_calls: u64,
    pub pairs: u64,
    pub q_powers: u64,
    pub coset_probes: u64,
    pub circuit_checks: u64,
    pub target_compilations: u64,
    pub numeric_decode_calls: u64,
}
impl Counts {
    pub fn json(&self) -> Value {
        json!({"old_admission_checks":self.old_admission_checks,
            "old_visits":self.old_visits,"sign_visits":self.sign_visits,
            "inverse_calls":self.inverse_calls,"encoding_pairs":self.pairs,
            "q_power_calls":self.q_powers,"coset_giant_probes":self.coset_probes,
            "original_circuit_checks":self.circuit_checks,
            "original_target_compilations":self.target_compilations,
            "numeric_decode_calls":self.numeric_decode_calls,
            "scope":"partial API/loop counters, not field work, cold total, timing or RAM"})
    }
}
pub struct Context {
    pub family: Family,
    n: u64,
    babies: BTreeMap<Key, u64>,
    omega: Fp2,
    giant: Fp2,
}
impl Context {
    pub fn new(s: &Spec) -> Result<Self> {
        // The inherited constructor enforces the declared original field,
        // A/a/q, order tests and 2K<a; it never substitutes q=1.
        let family = Family::new(s)?;
        let n = s.k.isqrt() + 1;
        if n > 512 {
            return Err("coset setup resource cap; incomplete".into());
        }
        let f = &family.cv.f;
        let omega = s.h.pow(&s.q.into(), f);
        let mut babies = BTreeMap::new();
        let mut cur = Fp2::one();
        for j in 0..n {
            if babies.insert(key(&cur), j).is_some() {
                return Err("coset baby repetition".into());
            }
            cur = cur.mul(&omega, f);
        }
        let giant = omega.pow(&n.into(), f).inverse(f)?;
        Ok(Self {
            family,
            n,
            babies,
            omega,
            giant,
        })
    }
    pub fn stats(&self) -> Value {
        json!({"k":self.family.s.k,"q":self.family.s.q,
            "coset_baby_entries":self.babies.len(),"coset_giant_bound":self.n+1,
            "numeric_t_table_built":false,"ratio_family":"r^q=h^q; no explicit q-element list"})
    }
    fn coset(&self, z: &Fp2, c: &mut Counts) -> Result<Option<u64>> {
        let f = &self.family.cv.f;
        if z.a >= f.p || z.b >= f.p || z.norm(f) != BigUint::one() {
            return Err("noncanonical/non-norm member".into());
        }
        c.q_powers += 1;
        let wanted = z.pow(&self.family.s.q.into(), f);
        let mut cur = wanted.clone();
        for i in 0..=self.n {
            c.coset_probes += 1;
            if let Some(j) = self.babies.get(&key(&cur)) {
                let j = i * self.n + j;
                if j > 0 && j <= self.family.s.k && self.omega.pow(&j.into(), f) == wanted {
                    return Ok(Some(j));
                }
            }
            cur = cur.mul(&self.giant, f);
        }
        Ok(None)
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    First,
    All,
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub old: usize,
    pub pairs: u64,
    pub outputs: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            old: 8,
            pairs: 16384,
            outputs: 2048,
        }
    }
}
#[derive(Clone)]
pub struct Witness {
    pub z: [Fp2; 2],
    pub raw: [Point; 2],
    pub images: [Point; 2],
    pub branches: [usize; 2],
    pub j: [u64; 2],
    pub values: Vec<BigUint>,
    pub closes_on_old: bool,
}
impl Witness {
    pub fn key(&self) -> String {
        json!({"z":[fp2_json(&self.z[0]),fp2_json(&self.z[1])],
            "raw":[point_json(&self.raw[0]),point_json(&self.raw[1])]})
        .to_string()
    }
    pub fn json(&self) -> Value {
        json!({"key":self.key(),"j":self.j,"branches":self.branches,
            "images":self.images.iter().map(point_json).collect::<Vec<_>>(),
            "closes_on_old":self.closes_on_old,
            "original_witness":self.values.iter().map(ToString::to_string).collect::<Vec<_>>()})
    }
    fn inputs(&self, s: &Spec) -> Result<Vec<WitnessInput>> {
        (0..2)
            .map(|i| {
                if self.j[i] == 0 || self.j[i] > s.k {
                    return Err("label bound".into());
                }
                Ok(WitnessInput {
                    z: self.z[i].clone(),
                    selector: self.j[i] - 1,
                    slack: s.k - self.j[i],
                    branch: self.branches[i].into(),
                    raw_v: self.raw[i].as_ref().ok_or("raw identity")?.1.clone(),
                })
            })
            .collect()
    }
}
pub struct Answer {
    pub roots: Vec<Witness>,
    pub counts: Counts,
    pub exhausted: bool,
}
impl Answer {
    pub fn json(&self) -> Value {
        json!({"roots":self.roots.iter().map(Witness::json).collect::<Vec<_>>(),
            "counts":self.counts.json(),"supplied_domain_exhausted":self.exhausted,
            "scope":"original two-member roots only within supplied old-column domain; no scalar recovery"})
    }
}
fn admitted_old(ctx: &Context, old: &[Point], l: Limits) -> Result<BTreeSet<Key>> {
    if l.old == 0
        || l.old > 32
        || l.pairs == 0
        || l.pairs > 65536
        || l.outputs == 0
        || l.outputs > 8192
        || old.is_empty()
        || old.len() > l.old
    {
        return Err("old-list/limit resource admission; incomplete".into());
    }
    let mut keys = BTreeSet::new();
    for p in old {
        ctx.family.check_image(p)?;
        if !keys.insert(column(&ctx.family.cv, p)?.0) {
            return Err("duplicate signed physical old column".into());
        }
    }
    Ok(keys)
}
pub fn verify(ctx: &Context, target: &Point, old: &[Point], w: &Witness) -> Result<()> {
    let f = &ctx.family;
    f.check_image(target)?;
    let oldkeys = admitted_old(
        ctx,
        old,
        Limits {
            old: 32,
            ..Limits::default()
        },
    )?;
    let mut c = Counts::default();
    for i in 0..2 {
        if ctx.coset(&w.z[i], &mut c)? != Some(w.j[i])
            || !raw_choices(&f.cv, &w.z[i])?.contains(&(w.branches[i], w.raw[i].clone()))
            || f.cv.times(&w.raw[i], 8)? != w.images[i]
        {
            return Err("original label/raw/image replay".into());
        }
    }
    let ratio = w.z[1].mul(&w.z[0].inverse(&f.cv.f)?, &f.cv.f);
    if w.j[0].checked_add(1) != Some(w.j[1])
        || w.z[0] == w.z[1]
        || ratio.pow(&f.s.q.into(), &f.cv.f) != ctx.omega
        || !oldkeys.contains(&column(&f.cv, &w.images[0])?.0)
        || f.cv.add(&w.images[0], &w.images[1])? != *target
        || oldkeys.contains(&column(&f.cv, &w.images[1])?.0) != w.closes_on_old
    {
        return Err("ratio/anchor/target/class replay".into());
    }
    let circuit = TargetCircuit::compile(&f.s, 2, target.clone(), true)?;
    if circuit.witness(&w.inputs(&f.s)?)? != w.values {
        return Err("original auxiliary witness replay".into());
    }
    circuit.system.check(&w.values)
}
pub fn query(
    ctx: &Context,
    target: &Point,
    old: &[Point],
    mode: Mode,
    l: Limits,
) -> Result<Answer> {
    let f = &ctx.family;
    f.check_image(target)?;
    let oldkeys = admitted_old(ctx, old, l)?;
    let circuit = TargetCircuit::compile(&f.s, 2, target.clone(), true)?;
    let mut out = Answer {
        roots: vec![],
        counts: Counts::default(),
        exhausted: false,
    };
    out.counts.target_compilations = 1;
    out.counts.old_admission_checks = old.len();
    let mut seen = BTreeSet::new();
    for p in old {
        out.counts.old_visits += 1;
        out.counts.inverse_calls += 1;
        let anchors = f.encodings(p)?;
        for sign in [1, -1] {
            out.counts.sign_visits += 1;
            let p = if sign == 1 { p.clone() } else { neg(&f.cv, p) };
            let complement = f.cv.add(target, &neg(&f.cv, &p))?;
            if complement.is_none() {
                continue;
            }
            out.counts.inverse_calls += 1;
            let other = f.encodings(&complement)?;
            for a in &anchors {
                let Some(j1) = ctx.coset(&a.z, &mut out.counts)? else {
                    continue;
                };
                for b in &other {
                    if out.counts.pairs == l.pairs {
                        return Err("encoding-pair work cap; incomplete".into());
                    }
                    out.counts.pairs += 1;
                    let ratio = b.z.mul(&a.z.inverse(&f.cv.f)?, &f.cv.f);
                    out.counts.q_powers += 1;
                    if ratio.pow(&f.s.q.into(), &f.cv.f) != ctx.omega {
                        continue;
                    }
                    let Some(j2) = ctx.coset(&b.z, &mut out.counts)? else {
                        continue;
                    };
                    if j1.checked_add(1) != Some(j2) {
                        continue;
                    }
                    let mut w = Witness {
                        z: [a.z.clone(), b.z.clone()],
                        raw: [
                            if sign == 1 {
                                a.raw.clone()
                            } else {
                                neg(&f.cv, &a.raw)
                            },
                            b.raw.clone(),
                        ],
                        images: [p.clone(), complement.clone()],
                        branches: [a.branch, b.branch],
                        j: [j1, j2],
                        values: vec![],
                        closes_on_old: oldkeys.contains(&column(&f.cv, &complement)?.0),
                    };
                    if seen.insert(w.key()) {
                        if out.roots.len() == l.outputs {
                            return Err("output cap; incomplete".into());
                        }
                        w.values = circuit.witness(&w.inputs(&f.s)?)?;
                        circuit.system.check(&w.values)?;
                        out.counts.circuit_checks += 1;
                        out.roots.push(w);
                        if mode == Mode::First {
                            return Ok(out);
                        }
                    }
                }
            }
        }
    }
    out.exhausted = true;
    Ok(out)
}
pub fn numeric_replay(
    ctx: &Context,
    target: &Point,
    w: &Witness,
    table: &IndexTable,
) -> Result<Value> {
    let c = TargetCircuit::compile(&ctx.family.s, 2, target.clone(), true)?;
    let rows = c.decode_row(&w.values, table)?;
    Ok(
        json!({"rows":rows.into_iter().map(|r|json!({"j":r.j,"t":r.t,
        "coefficient":r.coefficient,"point":point_json(&r.point)})).collect::<Vec<_>>(),
        "index_table":table.stats(),"scope":"separate original numeric indexing and independent native signed-row replay"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use torus_quotient_query::controls::spec;
    #[test]
    fn coset_window_and_resource_contract() {
        let s = spec(false, 8).unwrap();
        let c = Context::new(&s).unwrap();
        for j in 0..17u64 {
            let z = s.h.pow(&j.into(), &c.family.cv.f);
            assert_eq!(
                c.coset(&z, &mut Counts::default()).unwrap(),
                if (1..=8).contains(&j) { Some(j) } else { None }
            );
        }
        assert!(query(&c, &c.family.g, &[], Mode::All, Limits::default()).is_err());
        assert!(query(
            &c,
            &None,
            std::slice::from_ref(&c.family.g),
            Mode::All,
            Limits::default()
        )
        .is_err());
    }
    #[test]
    fn duplicate_physical_admission() {
        let c = Context::new(&spec(false, 8).unwrap()).unwrap();
        assert!(admitted_old(
            &c,
            &[c.family.g.clone(), neg(&c.family.cv, &c.family.g)],
            Limits::default()
        )
        .is_err());
    }
    #[test]
    fn public_pair_original_witness_and_mutation() {
        let s = spec(false, 8).unwrap();
        let c = Context::new(&s).unwrap();
        let a = raw_choices(&c.family.cv, &s.h).unwrap().remove(0).1;
        let mut b = raw_choices(&c.family.cv, &s.h.pow(&2u32.into(), &c.family.cv.f))
            .unwrap()
            .remove(0)
            .1;
        let p = c.family.cv.times(&a, 8).unwrap();
        let mut t = c
            .family
            .cv
            .times(&c.family.cv.add(&a, &b).unwrap(), 8)
            .unwrap();
        if t.is_none() {
            b = neg(&c.family.cv, &b);
            t = c
                .family
                .cv
                .times(&c.family.cv.add(&a, &b).unwrap(), 8)
                .unwrap();
        }
        let old = vec![p];
        let r = query(&c, &t, &old, Mode::First, Limits::default()).unwrap();
        let w = &r.roots[0];
        verify(&c, &t, &old, w).unwrap();
        assert!(!r.exhausted);
        let mut bad = w.clone();
        bad.values[0] = c.family.cv.f.add(&bad.values[0], &1u32.into());
        assert!(verify(&c, &t, &old, &bad).is_err());
        assert!(r.counts.pairs <= 1024 * 2);
        assert_eq!(r.counts.numeric_decode_calls, 0);
    }
}
