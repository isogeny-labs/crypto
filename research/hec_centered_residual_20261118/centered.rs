use crate::prepared::RootTable;
use crate::table_oracle::{split, OracleStats, Roots};
use hec_native_ic::{fp::Fp, poly::Poly};

pub fn recover(u: &Poly, fp: &Fp, stats: &mut OracleStats, table: &RootTable) -> Option<Roots> {
    if u.n != 3 {
        return split(u, fp, stats, table);
    }
    debug_assert_eq!(u.c[2], 1);
    stats.calls += 1;
    let negative_b = fp.neg(u.c[1]);
    // The native field bound p < 2^62 leaves space for the parity correction.
    let center = (negative_b + (negative_b & 1) * fp.p) >> 1;
    let square = fp.sub(fp.mul(center, center), u.c[0]);
    let mut roots = Roots::default();
    if square == 0 {
        roots.r[0] = (center, 2);
        roots.len = 1;
    } else {
        let offset = table.sqrt(square, fp)?;
        let first = fp.add(center, offset);
        let second = fp.sub(center, offset);
        roots.r[0] = (first.min(second), 1);
        roots.r[1] = (first.max(second), 1);
        roots.len = 2;
    }
    stats.smooth += 1;
    Some(roots)
}
