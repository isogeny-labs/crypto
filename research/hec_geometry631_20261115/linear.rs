fn monomials(degree: u8) -> Vec<[u8; 8]> {
    fn rec(i: usize, d: u8, e: &mut [u8; 8], out: &mut Vec<[u8; 8]>) {
        if i == 7 {
            e[i] = d;
            out.push(*e);
            return;
        }
        for n in 0..=d {
            e[i] = n;
            rec(i + 1, d - n, e, out)
        }
    }
    let mut out = Vec::new();
    rec(0, degree, &mut [0; 8], &mut out);
    out
}
fn mono(e: &[u8; 8], x: &[u64; 8]) -> u64 {
    let mut out = 1;
    for i in 0..8 {
        for _ in 0..e[i] {
            out = out * x[i] % P
        }
    }
    out
}
fn row_at(ms: &[[u8; 8]], x: &[u64; 8]) -> Vec<u64> {
    ms.iter().map(|e| mono(e, x)).collect()
}
fn derivative_row(ms: &[[u8; 8]], x: &[u64; 8], j: usize) -> Vec<u64> {
    ms.iter()
        .map(|e| {
            if e[j] == 0 {
                return 0;
            }
            let mut ee = *e;
            ee[j] -= 1;
            (e[j] as u64) * mono(&ee, x) % P
        })
        .collect()
}
fn dot_fp(a: &[u64], b: &[u64]) -> u64 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(0, |s, (&x, &y)| (s + x * y) % P)
}
fn nullspace_fp(original: &[Vec<u64>], cols: usize) -> Vec<Vec<u64>> {
    assert!(original.iter().all(|r| r.len() == cols));
    let mut m = original.to_vec();
    let mut pivots = Vec::new();
    let fp = Fp::new(P);
    for j in 0..cols {
        let row = pivots.len();
        if row == m.len() {
            break;
        }
        let Some(i) = (row..m.len()).find(|&i| m[i][j] != 0) else {
            continue;
        };
        m.swap(i, row);
        let inv = fp.inv(m[row][j]);
        for k in j..cols {
            m[row][k] = m[row][k] * inv % P
        }
        for i in 0..m.len() {
            if i != row {
                let t = m[i][j];
                if t != 0 {
                    for k in j..cols {
                        m[i][k] = (m[i][k] + P - t * m[row][k] % P) % P
                    }
                }
            }
        }
        pivots.push(j)
    }
    let mut out = Vec::new();
    for free in 0..cols {
        if pivots.contains(&free) {
            continue;
        }
        let mut v = vec![0; cols];
        v[free] = 1;
        for (i, &p) in pivots.iter().enumerate() {
            v[p] = (P - m[i][free]) % P
        }
        for r in original {
            assert_eq!(dot_fp(r, &v), 0)
        }
        out.push(v)
    }
    out
}
