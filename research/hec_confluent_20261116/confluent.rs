use hec_native_ic::{fp::Fp, jac::Curve, poly::Poly};
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub x: u64,
    pub y: u64,
    pub m: usize,
    pub neg: bool,
}
fn smul(a: &[u64], b: &[u64], f: &Fp) -> Vec<u64> {
    let n = a.len();
    let mut c = vec![0; n];
    for i in 0..n {
        for j in 0..n - i {
            c[i + j] = f.add(c[i + j], f.mul(a[i], b[j]));
        }
    }
    c
}
pub fn compose(p: &Poly, x: &[u64], f: &Fp) -> Vec<u64> {
    let mut out = vec![0; x.len()];
    for &a in p.c[..p.n].iter().rev() {
        out = smul(&out, x, f);
        out[0] = f.add(out[0], a);
    }
    out
}
pub fn jets(c: &Curve, node: Node) -> (Vec<u64>, Vec<u64>) {
    let n = node.m;
    let f = &c.fp;
    assert!(n > 0);
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    x[0] = node.x;
    if node.y != 0 {
        if n > 1 {
            x[1] = 1;
        }
        y[0] = if node.neg { f.neg(node.y) } else { node.y };
        let ff = compose(&c.f, &x, f);
        let inv = f.inv(f.add(y[0], y[0]));
        for k in 1..n {
            let z = (1..k).fold(0, |z, j| f.add(z, f.mul(y[j], y[k - j])));
            y[k] = f.mul(f.sub(ff[k], z), inv);
        }
    } else {
        if n > 1 {
            y[1] = 1;
        }
        let inv = f.inv(c.f.deriv(f).eval(node.x, f));
        for k in 1..n {
            let ff = compose(&c.f, &x, f);
            let want = if k == 2 { 1 } else { 0 };
            x[k] = f.mul(f.sub(want, ff[k]), inv);
        }
    }
    assert_eq!(smul(&y, &y, f), compose(&c.f, &x, f), "local_curve_series");
    (x, y)
}
fn bounds(g: usize, d: usize) -> (usize, usize) {
    if d == 0 {
        (g, 0)
    } else {
        (g + d, d - 1)
    }
}
pub fn gaussian(c: &Curve, nodes: &[Node], d: usize) -> (Poly, Poly, usize) {
    let (g, f) = (c.g, &c.fp);
    let (aa, bb) = bounds(g, d);
    let cols = aa + bb + 2;
    assert_eq!(nodes.iter().map(|n| n.m).sum::<usize>() + 1, cols);
    let mut matrix = vec![];
    for &node in nodes {
        let (x, y) = jets(c, node);
        let mut powers = vec![vec![0; node.m]; aa.max(bb) + 1];
        powers[0][0] = 1;
        for j in 1..powers.len() {
            powers[j] = smul(&powers[j - 1], &x, f);
        }
        let by: Vec<_> = (0..=bb).map(|j| smul(&powers[j], &y, f)).collect();
        for k in 0..node.m {
            let mut r: Vec<_> = (0..=aa).map(|j| powers[j][k]).collect();
            r.extend((0..=bb).map(|j| f.neg(by[j][k])));
            matrix.push(r);
        }
    }
    let original = matrix.clone();
    let mut pivots = vec![];
    let mut r = 0;
    for col in 0..cols {
        let Some(q) = (r..matrix.len()).find(|&i| matrix[i][col] != 0) else {
            continue;
        };
        matrix.swap(r, q);
        let inv = f.inv(matrix[r][col]);
        for a in &mut matrix[r][col..] {
            *a = f.mul(*a, inv);
        }
        for i in r + 1..matrix.len() {
            let z = matrix[i][col];
            for j in col..cols {
                matrix[i][j] = f.sub(matrix[i][j], f.mul(z, matrix[r][j]));
            }
        }
        pivots.push(col);
        r += 1;
        if r == matrix.len() {
            break;
        }
    }
    let free = (0..cols).rev().find(|j| !pivots.contains(j)).unwrap();
    let mut sol = vec![0; cols];
    sol[free] = 1;
    for (i, &j) in pivots.iter().enumerate().rev() {
        sol[j] = f.neg((j + 1..cols).fold(0, |z, k| f.add(z, f.mul(matrix[i][k], sol[k]))));
    }
    for row in &original {
        assert_eq!(
            row.iter()
                .zip(&sol)
                .fold(0, |z, (&a, &b)| f.add(z, f.mul(a, b))),
            0,
            "nullspace_equation"
        );
    }
    (
        Poly::from_coeffs(&sol[..=aa]),
        Poly::from_coeffs(&sol[aa + 1..]),
        cols - r,
    )
}
pub fn canonical(a: Poly, b: Poly, f: &Fp) -> (Poly, Poly, Poly) {
    assert!(!a.is_zero() || !b.is_zero());
    let h = Poly::gcd(&a, &b, f);
    let a = a.div_exact(&h, f);
    let b = b.div_exact(&h, f);
    let lead = if !b.is_zero() {
        b.c[b.n - 1]
    } else {
        a.c[a.n - 1]
    };
    let inv = f.inv(lead);
    (a.scale(inv, f), b.scale(inv, f), h)
}
pub fn reference(c: &Curve, nodes: &[Node], d: usize) -> (Poly, Poly) {
    assert!(nodes.iter().all(|n| n.y != 0));
    let f = &c.fp;
    let mut h = Poly::zero();
    let mut a = Poly::one();
    for &node in nodes {
        let (x, y) = jets(c, node);
        for k in 0..node.m {
            let hj = compose(&h, &x, f);
            let aj = compose(&a, &x, f);
            assert_ne!(aj[k], 0);
            let w = f.mul(f.sub(y[k], hj[k]), f.inv(aj[k]));
            h = h.add(&a.scale(w, f), f);
            a = a.mul(&Poly::x_minus(node.x, f), f);
        }
    }
    let (maxa, maxb) = bounds(c.g, d);
    let (mut r0, mut r1) = (a, h);
    let (mut t0, mut t1) = (Poly::zero(), Poly::one());
    while r1.deg() > maxa as isize {
        let (q, r) = r0.divrem(&r1, f);
        let t = t0.sub(&q.mul(&t1, f), f);
        r0 = r1;
        r1 = r;
        t0 = t1;
        t1 = t;
    }
    assert!(t1.deg() <= maxb as isize);
    let (a, b, _) = canonical(r1, t1, f);
    (a, b)
}
pub fn normalize(
    c: &Curve,
    nodes: &[Node],
    a: Poly,
    b: Poly,
) -> (Poly, Poly, Poly, Vec<usize>, usize) {
    let f = &c.fp;
    let norm = a.mul(&a, f).sub(&c.f.mul(&b.mul(&b, f), f), f);
    let mut anchors = Poly::one();
    for n in nodes {
        for _ in 0..n.m {
            anchors = anchors.mul(&Poly::x_minus(n.x, f), f);
        }
    }
    let (_, rem) = norm.divrem(&anchors, f);
    assert!(rem.is_zero(), "original_norm_division");
    let (a, b, h) = canonical(a, b, f);
    let mut kept = vec![];
    let mut divisor = Poly::one();
    for n in nodes {
        let mut tmp = h;
        let mut order = 0;
        while !tmp.is_zero() && tmp.eval(n.x, f) == 0 {
            tmp = tmp.div_exact(&Poly::x_minus(n.x, f), f);
            order += 1;
        }
        let loss = if n.y == 0 { 2 * order } else { order };
        let m = n.m.saturating_sub(loss);
        kept.push(m);
        for _ in 0..m {
            divisor = divisor.mul(&Poly::x_minus(n.x, f), f);
        }
    }
    let norm = a.mul(&a, f).sub(&c.f.mul(&b.mul(&b, f), f), f);
    let (q, r) = norm.divrem(&divisor, f);
    assert!(r.is_zero(), "primitive_norm_division");
    (a, b, q, kept, h.deg() as usize)
}
