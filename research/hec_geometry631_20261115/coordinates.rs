fn digits(mut i: usize) -> [usize; 3] {
    let mut a = [0; 3];
    for x in a.iter_mut().rev() {
        *x = i % 3;
        i /= 3
    }
    assert_eq!(i, 0);
    a
}
fn negchar(a: usize) -> usize {
    let a = digits(a);
    ((3 - a[0]) % 3) * 9 + ((3 - a[1]) % 3) * 3 + (3 - a[2]) % 3
}
fn sumchar(a: usize, b: usize) -> usize {
    let a = digits(a);
    let b = digits(b);
    ((a[0] + b[0]) % 3) * 9 + ((a[1] + b[1]) % 3) * 3 + (a[2] + b[2]) % 3
}
fn projections(v: &[E], alpha: &[E], characters: &[Vec<E>], f: &Field) -> Vec<E> {
    characters
        .iter()
        .map(|row| {
            row.iter()
                .zip(v)
                .zip(alpha)
                .fold(Z, |o, ((&w, &v), &a)| f.add(o, f.mul(w, f.mul(v, a))))
        })
        .collect()
}
fn products(phi: &[E], representatives: &[usize], f: &Field) -> Vec<E> {
    assert_ne!(phi[0], Z);
    let inv = f.inv(f.mul(phi[0], phi[0]));
    representatives
        .iter()
        .map(|&a| f.mul(f.mul(phi[a], phi[negchar(a)]), inv))
        .collect()
}
fn trace_candidates(g: &[E], sigma: &[usize], f: &Field) -> Vec<E> {
    let z = {
        let mut a = Z;
        a[1] = 1;
        a
    };
    let mut result = Vec::new();
    for a in 0..g.len() {
        for j in 0..13 {
            let mut w = f.pow(z, j);
            let mut index = a;
            let mut value = Z;
            for _ in 0..13 {
                value = f.add(value, f.mul(w, g[index]));
                w = f.frob(w);
                index = sigma[index]
            }
            assert_eq!(index, a);
            result.push(value)
        }
    }
    result
}
fn field_trace(a: E, f: &Field) -> E {
    let mut v = a;
    let mut out = Z;
    for _ in 0..13 {
        out = f.add(out, v);
        v = f.frob(v)
    }
    assert_eq!(v, a);
    out
}
fn pivots_fp(mut rows: Vec<Vec<u64>>) -> Vec<usize> {
    let n = rows.len();
    let cols = rows[0].len();
    let fp = Fp::new(P);
    let mut out = Vec::new();
    for j in 0..cols {
        let r = out.len();
        if r == n {
            break;
        }
        let Some(i) = (r..n).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(i, r);
        let inv = fp.inv(rows[r][j]);
        for k in j..cols {
            rows[r][k] = fp.mul(rows[r][k], inv)
        }
        for i in 0..n {
            if i != r {
                let v = rows[i][j];
                for k in j..cols {
                    rows[i][k] = fp.sub(rows[i][k], fp.mul(v, rows[r][k]))
                }
            }
        }
        out.push(j)
    }
    out
}
