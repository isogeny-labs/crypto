//! The 2-torsion translation iota: P -> P + T, T = (0, sqrt b); on x it is x -> sqrt(b)/x.
//! Exact audit (PROTOCOL section 7): for k iota-stable slots S = {+-P, +-(P+T)} against the plain slots {+-P},
//!   H_iota[R] = 2^(k-1) (H_plain[R] + H_plain[R+T])     and     H_iota[R] = H_iota[R+T].
//! So an iota-stable factor base returns the plain relations of the T-coset of the target at 2^k times the enumeration.

use crate::census;
use crate::curve::{Curve, Pt};
use crate::rng::Rng;

pub struct IotaReport {
    pub n: u32,
    pub m: usize,
    pub k: usize,
    pub translate_invariant: bool,
    pub multiple_of_2k1: bool,
    pub quotient_identity: bool,
    pub tuples_iota: u128,
    pub tuples_plain: u128,
    pub query_iota: usize,
    pub query_plain: usize,
}

pub fn iota_audit(c: &Curve, m: usize, k: usize, seed: u64) -> Result<IotaReport, String> {
    assert!(k >= 1 && k <= m);
    let f = c.f;
    let t = c.t2();
    let xfix = f.sqrt(f.sqrt(c.b)); // iota fixes x only at x^2 = sqrt b
    let mut rng = Rng::new(seed);
    let mut used = std::collections::HashSet::new();
    let (mut iota_slots, mut plain_slots): (Vec<Vec<Pt>>, Vec<Vec<Pt>>) = (vec![], vec![]);
    let mut guard = 0;
    while iota_slots.len() < m {
        guard += 1;
        if guard > 100000 {
            return Err("not enough x values".into());
        }
        let x = 1 + rng.below(f.q as u64 - 1) as u32;
        let y = match c.lift_x(x) {
            Some(y) => y,
            None => continue,
        };
        let xi = c.iota_x(x);
        if x == xfix || xi == xfix || used.contains(&x) || used.contains(&xi) {
            continue;
        }
        let p = Pt::Aff(x, y);
        let pt = c.add(p, t);
        match pt {
            Pt::Aff(px, _) if px == xi => {}
            _ => return Err("x(P+T) != sqrt(b)/x".into()),
        }
        used.insert(x);
        used.insert(xi);
        let plain = vec![p, c.neg(p)];
        if iota_slots.len() < k {
            iota_slots.push(vec![p, c.neg(p), pt, c.neg(pt)]);
        } else {
            iota_slots.push(plain.clone());
        }
        plain_slots.push(plain);
    }
    let mut hi = Vec::new();
    let mut hp = Vec::new();
    let ci = census::histogram_into(c, &iota_slots, &mut hi);
    let cp = census::histogram_into(c, &plain_slots, &mut hp);
    let mult = 1u32 << (k - 1);
    let (mut tr, mut mo, mut qi) = (true, true, true);
    let mut check = |p: Pt| {
        let r = c.key(p);
        let rt = c.key(c.add(p, t));
        if hi[r] != hi[rt] {
            tr = false;
        }
        if hi[r] % mult != 0 {
            mo = false;
        }
        if hi[r] != mult * (hp[r] + hp[rt]) {
            qi = false;
        }
    };
    check(Pt::Inf);
    for x in 0..f.q {
        if let Some(y) = c.lift_x(x) {
            check(Pt::Aff(x, y));
            if x != 0 {
                check(Pt::Aff(x, x ^ y));
            }
        }
    }
    Ok(IotaReport {
        n: f.n,
        m,
        k,
        translate_invariant: tr,
        multiple_of_2k1: mo,
        quotient_identity: qi,
        tuples_iota: ci.t,
        tuples_plain: cp.t,
        query_iota: ci.a_len.min(ci.b_len),
        query_plain: cp.a_len.min(cp.b_len),
    })
}
