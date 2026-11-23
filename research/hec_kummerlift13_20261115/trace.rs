include!("trace_eval.rs");
const P: u64 = 127;
type Poly = [u64; 49];
fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn mul(a: u64, b: u64) -> u64 {
    a * b % P
}
fn pow(mut a: u64, mut n: u64) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = mul(r, a)
        }
        a = mul(a, a);
        n >>= 1;
    }
    r
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0);
    pow(a, P - 2)
}
fn monos(d: usize) -> Vec<(usize, usize)> {
    (0..=d)
        .flat_map(|i| (0..=d - i).map(move |j| (i, j)))
        .collect()
}
fn val(a: &Poly, x: u64, y: u64) -> u64 {
    monos(6).into_iter().fold(0, |s, (i, j)| {
        add(
            s,
            mul(a[7 * i + j], mul(pow(x, i as u64), pow(y, j as u64))),
        )
    })
}
fn product(a: &Poly, b: &Poly) -> Poly {
    let mut r = [0; 49];
    for (i, j) in monos(3) {
        for (k, l) in monos(3) {
            r[7 * (i + k) + j + l] = add(r[7 * (i + k) + j + l], mul(a[7 * i + j], b[7 * k + l]));
        }
    }
    r
}
fn reduce(mut a: Poly, f: &Poly) -> Poly {
    assert_eq!(f[28], 1);
    for i in (4..=6).rev() {
        for j in 0..=6 - i {
            let c = a[7 * i + j];
            if c == 0 {
                continue;
            }
            for (k, l) in monos(4) {
                a[7 * (i - 4 + k) + j + l] = sub(a[7 * (i - 4 + k) + j + l], mul(c, f[7 * k + l]));
            }
        }
    }
    a
}
fn kernel(mut a: Vec<Vec<u64>>, n: usize) -> (usize, Vec<Vec<u64>>) {
    assert!(a.iter().all(|r| r.len() == n));
    let mut piv = Vec::new();
    let mut row = 0;
    for c in 0..n {
        let Some(r) = (row..a.len()).find(|r| a[*r][c] != 0) else {
            continue;
        };
        a.swap(row, r);
        let z = inv(a[row][c]);
        for x in &mut a[row] {
            *x = mul(*x, z)
        }
        for r in 0..a.len() {
            if r == row {
                continue;
            }
            let z = a[r][c];
            if z == 0 {
                continue;
            }
            for j in c..n {
                a[r][j] = sub(a[r][j], mul(z, a[row][j]));
            }
        }
        piv.push(c);
        row += 1;
        if row == a.len() {
            break;
        }
    }
    let mut out = Vec::new();
    for free in (0..n).filter(|c| !piv.contains(c)) {
        let mut v = vec![0; n];
        v[free] = 1;
        for (r, c) in piv.iter().enumerate() {
            v[*c] = sub(0, a[r][free]);
        }
        out.push(v);
    }
    (piv.len(), out)
}
fn curve() -> Poly {
    let q = [1, 58, 103, 47, 66, 88, 51, 80, 84, 123, 34, 83, 109, 1, 111];
    let mut f = [0; 49];
    let mut k = 0;
    for i in (0..=4).rev() {
        for j in (0..=4 - i).rev() {
            f[7 * i + j] = q[k];
            k += 1;
        }
    }
    f
}
fn tangent_restrict(f: &Poly, a: u64, b: u64) -> Vec<u64> {
    let mut r = vec![0; 5];
    for (i, j) in monos(4) {
        let mut t = vec![1];
        for _ in 0..j {
            let mut v = vec![0; t.len() + 1];
            for (k, c) in t.iter().enumerate() {
                v[k] = add(v[k], mul(*c, b));
                v[k + 1] = add(v[k + 1], mul(*c, a));
            }
            t = v;
        }
        for (k, c) in t.iter().enumerate() {
            r[i + k] = add(r[i + k], mul(f[7 * i + j], *c));
        }
    }
    r
}
fn div_root(a: &[u64], x: u64) -> Vec<u64> {
    let mut v = a.to_vec();
    let n = v.len() - 1;
    let mut q = vec![0; n];
    for i in (1..=n).rev() {
        q[i - 1] = v[i];
        v[i - 1] = add(v[i - 1], mul(x, v[i]));
    }
    assert_eq!(v[0], 0);
    q
}
fn peval(a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |s, c| add(mul(s, x), *c))
}
fn line_monom(i: usize, j: usize, a: u64, b: u64, r: &[u64]) -> [u64; 2] {
    let mut t = vec![0; i + 1];
    t[i] = 1;
    for _ in 0..j {
        let mut v = vec![0; t.len() + 1];
        for (k, c) in t.iter().enumerate() {
            v[k] = add(v[k], mul(*c, b));
            v[k + 1] = add(v[k + 1], mul(*c, a));
        }
        t = v;
    }
    while t.len() > 2 {
        let k = t.len() - 1;
        let c = mul(t[k], inv(r[2]));
        for l in 0..=2 {
            t[k - 2 + l] = sub(t[k - 2 + l], mul(c, r[l]));
        }
        t.pop();
    }
    [
        t.first().copied().unwrap_or(0),
        t.get(1).copied().unwrap_or(0),
    ]
}
fn rr_basis(f: &Poly) -> Vec<Poly> {
    for (x, y) in [(18, 52), (28, 86), (43, 125), (101, 43)] {
        assert_eq!(val(f, x, y), 0, "archived point mismatch");
        let mut fx = 0;
        let mut fy = 0;
        for (i, j) in monos(4) {
            if i > 0 {
                fx = add(
                    fx,
                    mul(
                        i as u64,
                        mul(f[7 * i + j], mul(pow(x, (i - 1) as u64), pow(y, j as u64))),
                    ),
                );
            }
            if j > 0 {
                fy = add(
                    fy,
                    mul(
                        j as u64,
                        mul(f[7 * i + j], mul(pow(x, i as u64), pow(y, (j - 1) as u64))),
                    ),
                );
            }
        }
        if fy == 0 {
            println!("POINT_REJECT {x} {y} vertical");
            continue;
        }
        let a = mul(sub(0, fx), inv(fy));
        let b = sub(y, mul(a, x));
        let r = div_root(&div_root(&tangent_restrict(f, a, b), x), x);
        if r[2] == 0 || peval(&r, x) == 0 {
            println!("POINT_REJECT {x} {y} residual_chart");
            continue;
        }
        let ms = monos(3);
        let mut mat = vec![vec![0; 10]; 2];
        for (k, (i, j)) in ms.iter().enumerate() {
            let z = line_monom(*i, *j, a, b, &r);
            for t in 0..2 {
                mat[t][k] = z[t];
            }
        }
        let (rank, basis) = kernel(mat, 10);
        assert_eq!(rank, 2);
        assert_eq!(basis.len(), 8);
        println!("ANCHOR O=({x},{y}) tangent_y={a}x+{b} residual={r:?} rr_dimension=8");
        return basis
            .into_iter()
            .map(|v| {
                let mut n = [0; 49];
                for (k, (i, j)) in ms.iter().enumerate() {
                    n[7 * i + j] = v[k];
                }
                n
            })
            .collect();
    }
    panic!("no_registered_anchor_chart")
}
fn residual(m: &[Vec<u64>], v: &[u64]) -> Vec<u64> {
    m.iter()
        .map(|r| r.iter().zip(v).fold(0, |s, (a, b)| add(s, mul(*a, *b))))
        .collect()
}
fn main() {
    let f = curve();
    let v = rr_basis(&f);
    let ps: Vec<_> = monos(6).into_iter().filter(|(i, _)| *i < 4).collect();
    let qs = monos(3);
    assert_eq!(ps.len(), 22);
    let mut products = vec![vec![[0; 49]; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            products[i][j] = reduce(product(&v[i], &v[j]), &f);
        }
    }
    let pairs: Vec<_> = (0..8).flat_map(|i| (i..8).map(move |j| (i, j))).collect();
    let prodmat: Vec<Vec<u64>> = ps
        .iter()
        .map(|(x, y)| {
            pairs
                .iter()
                .map(|(i, j)| products[*i][*j][7 * x + y])
                .collect()
        })
        .collect();
    let (mulrank, _) = kernel(prodmat, 36);
    assert_eq!(mulrank, 18);
    let triples: Vec<_> = (0..8)
        .flat_map(|i| (i..8).flat_map(move |j| (j..8).map(move |k| [i, j, k])))
        .collect();
    assert_eq!(triples.len(), 120);
    let mut m = vec![vec![0; 120]; 220];
    for (c, t) in triples.iter().enumerate() {
        for perm in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let (i, j, k) = (t[perm[0]], t[perm[1]], t[perm[2]]);
            for (p, (x, y)) in ps.iter().enumerate() {
                for (q, (z, w)) in qs.iter().enumerate() {
                    m[10 * p + q][c] = add(
                        m[10 * p + q][c],
                        mul(products[i][j][7 * x + y], v[k][7 * z + w]),
                    );
                }
            }
        }
    }
    let (rank, h) = kernel(m.clone(), 120);
    assert_eq!(rank, 112);
    assert_eq!(h.len(), 8);
    for (i, a) in h.iter().enumerate() {
        assert!(residual(&m, a).iter().all(|x| *x == 0));
        println!("SECTION {i} {a:?}");
    }
    let badcol = (0..120).find(|c| m.iter().any(|r| r[*c] != 0)).unwrap();
    let mut corrupt = h[0].clone();
    corrupt[badcol] = add(corrupt[badcol], 1);
    assert!(residual(&m, &corrupt).iter().any(|x| *x != 0));
    trace_gate(&v, &h, &triples);
    println!("PASS coefficient_identity=true multiplication_rank={mulrank} diagonal_rank={rank} sections={} corrupt_rejected=true timing_claim=false",h.len());
}
