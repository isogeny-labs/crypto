use crate::{
    centered,
    prepared::RootTable,
    table_oracle::{OracleStats, Roots},
};
use hec_native_ic::{fp::Fp, poly::Poly};

pub fn recover(u: &Poly, fp: &Fp, small: &[(u64, u64)], table: &RootTable) -> Option<Roots> {
    if u.n < 4 {
        return centered::recover(u, fp, &mut OracleStats::default(), table);
    }
    assert_eq!(u.n, 4);
    let (a, b) = (u.c[2], u.c[1]);
    let mut value = u.c[0];
    let mut delta = fp.add(1, fp.add(a, b));
    let mut second = fp.add(6, fp.add(a, a));
    let mut x = 0;
    for &(px, _) in small {
        while x < px {
            value = fp.add(value, delta);
            delta = fp.add(delta, second);
            second = fp.add(second, 6);
            x += 1;
        }
        if value != 0 {
            continue;
        }
        let q1 = fp.add(a, x);
        let q0 = fp.add(b, fp.mul(x, q1));
        let quotient = Poly::from_coeffs(&[q0, q1, 1]);
        let mut roots = centered::recover(&quotient, fp, &mut OracleStats::default(), table)?;
        if let Some(entry) = roots.r[..roots.len].iter_mut().find(|e| e.0 == x) {
            entry.1 += 1;
        } else {
            roots.r[roots.len] = (x, 1);
            roots.len += 1;
            roots.r[..roots.len].sort_unstable();
        }
        return Some(roots);
    }
    None
}
