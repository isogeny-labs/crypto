use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    oracle::{split_reference, OracleStats},
};
pub fn equation(
    c: &Curve,
    source: &Div,
    fb: &[(u64, u64)],
    map: &[(usize, u64)],
    r: usize,
    ell: u64,
) -> Option<(Vec<u64>, Vec<u64>)> {
    let mut stats = OracleStats::default();
    let roots = split_reference(&source.u, &c.fp, &mut stats)?;
    let f = Fp::new(ell);
    let mut full = vec![0; fb.len()];
    let mut fold = vec![0; r];
    let mut sum = c.zero();
    for &(x, multiplicity) in roots.as_slice() {
        let j = fb.iter().position(|v| v.0 == x).unwrap();
        let y = source.v.eval(x, &c.fp);
        let value = if y == fb[j].1 {
            f.from_i64(multiplicity as i64)
        } else {
            assert_eq!(y, c.fp.neg(fb[j].1));
            f.from_i64(-(multiplicity as i64))
        };
        full[j] = f.add(full[j], value);
        let (col, w) = map[j];
        if col != usize::MAX {
            fold[col] = f.add(fold[col], f.mul(value, w))
        }
        for _ in 0..multiplicity {
            sum = c.add_place(&sum, x, y)
        }
    }
    assert_eq!(sum, *source);
    assert!(!c.is_zero(source));
    Some((full, fold))
}
