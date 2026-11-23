use hec_native_ic::fp::Fp;
pub fn witness(rows: &[Vec<u64>], want: &[u64], f: &Fp) -> Option<Vec<u64>> {
    let n = want.len();
    let mut basis: Vec<Option<(Vec<u64>, Vec<u64>)>> = vec![None; n];
    for (i, row) in rows.iter().enumerate() {
        let mut a = row.clone();
        let mut w = vec![0; rows.len()];
        w[i] = 1;
        for j in 0..n {
            if a[j] == 0 {
                continue;
            }
            if let Some((p, q)) = &basis[j] {
                let t = a[j];
                for k in j..n {
                    a[k] = f.sub(a[k], f.mul(t, p[k]));
                }
                for k in 0..w.len() {
                    w[k] = f.sub(w[k], f.mul(t, q[k]));
                }
            } else {
                let inv = f.inv(a[j]);
                for x in &mut a {
                    *x = f.mul(*x, inv);
                }
                for x in &mut w {
                    *x = f.mul(*x, inv);
                }
                basis[j] = Some((a, w));
                break;
            }
        }
    }
    let mut rem = want.to_vec();
    let mut out = vec![0; rows.len()];
    for j in 0..n {
        if rem[j] == 0 {
            continue;
        }
        let (p, q) = basis[j].as_ref()?;
        let t = rem[j];
        for k in j..n {
            rem[k] = f.sub(rem[k], f.mul(t, p[k]));
        }
        for k in 0..out.len() {
            out[k] = f.add(out[k], f.mul(t, q[k]));
        }
    }
    if rem.iter().all(|x| *x == 0) {
        Some(out)
    } else {
        None
    }
}
pub fn verify(rows: &[Vec<u64>], want: &[u64], w: &[u64], f: &Fp) -> bool {
    if w.len() != rows.len() {
        return false;
    }
    let mut out = vec![0; want.len()];
    for (row, &a) in rows.iter().zip(w) {
        for j in 0..out.len() {
            out[j] = f.add(out[j], f.mul(a, row[j]));
        }
    }
    out == want
}
