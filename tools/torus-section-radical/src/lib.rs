//! First-power support for Fp norm sections of the original distinct-row base.
//! No coefficient root discovery or general algebraic-closure equivalence.
pub mod controls;
pub mod symbolic;
use serde_json::{json, Value};
use torus_quotient_query::arithmetic::{c, encoded, is_zero, P};
use torus_quotient_query::query::Query;
pub type Result<T> = std::result::Result<T, String>;

/// Classical Montgomery XZ doubling, three times, in the existing quotient.
/// A unit denominator excludes support with identity eightfold image.
pub fn eightfold_denominator(q: &Query<'_>) -> Result<P> {
    let r = &q.ring;
    let mut n = r.x()?;
    let mut d = r.constant(c(1));
    let a = torus_s4_tree::Fp2 {
        a: q.curve.a.clone(),
        b: 0u32.into(),
    };
    for _ in 0..3 {
        let nn = r.mul(&n, &n)?;
        let dd = r.mul(&d, &d)?;
        let nd = r.mul(&n, &d)?;
        let difference = r.sub(&nn, &dd);
        let next_n = r.mul(&difference, &difference)?;
        let next_d = r.scale(
            &r.mul(&nd, &r.add(&r.add(&nn, &r.scale(&nd, &a)?), &dd))?,
            &c(4),
        )?;
        n = next_n;
        d = next_d;
    }
    Ok(d)
}

pub fn predicate(q: &Query<'_>) -> Result<Value> {
    let h = q.ring.h_interval(&q.rho, 1, q.spec.k)?;
    let d = eightfold_denominator(q)?;
    let gcd = q.ring.e.gcd(&q.ring.s, &d)?;
    let h_zero = h.iter().all(is_zero);
    let denominator_unit = gcd.len() == 1;
    Ok(json!({"accept":h_zero && denominator_unit,"H_zero":h_zero,
        "eightfold_unit":denominator_unit,"H":encoded(&h),"D3":encoded(&d),
        "gcd":encoded(&gcd),"S":encoded(&q.ring.s),"partial_counts":q.ring.e.stats()}))
}
