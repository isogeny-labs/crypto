#[derive(Clone, Debug, PartialEq, Eq)]
struct H {
    a: Pol,
    b: Pol,
}
fn pow(a: &Pol, n: usize, f: &Field) -> Pol {
    (0..n).fold(Pol::one(), |s, _| s.mul(a, f))
}
fn inverse(a: &Pol, m: &Pol, f: &Field) -> Option<Pol> {
    let (g, s, _) = Pol::xgcd(a, m, f);
    (g == Pol::one()).then(|| s.rem(m, f))
}
fn derivative(a: &Pol, f: &Field) -> Pol {
    Pol::new(
        (1..a.0.len())
            .map(|i| f.mul(base(i as u64), a.coeff(i)))
            .collect(),
    )
}
fn squarefree(a: &Pol, f: &Field) -> bool {
    Pol::xgcd(a, &derivative(a, f), f).0 == Pol::one()
}
fn nullvector(mut m: Vec<Vec<E>>, f: &Field) -> Option<Vec<E>> {
    let cols = m[0].len();
    let mut pivots = Vec::new();
    for j in 0..cols {
        let row = pivots.len();
        let Some(i) = (row..m.len()).find(|&i| m[i][j] != Z) else {
            continue;
        };
        m.swap(i, row);
        let inv = f.inv(m[row][j]);
        for k in j..cols {
            m[row][k] = f.mul(m[row][k], inv)
        }
        for i in 0..m.len() {
            if i != row {
                let t = m[i][j];
                for k in j..cols {
                    m[i][k] = f.sub(m[i][k], f.mul(t, m[row][k]))
                }
            }
        }
        pivots.push(j);
    }
    if pivots.len() + 1 != cols {
        return None;
    }
    let free = (0..cols).find(|j| !pivots.contains(j)).unwrap();
    let mut v = vec![Z; cols];
    v[free] = O;
    for (i, &j) in pivots.iter().enumerate() {
        v[j] = f.neg(m[i][free])
    }
    let lead = f.inv(*v.iter().find(|&&x| x != Z).unwrap());
    for x in &mut v {
        *x = f.mul(*x, lead)
    }
    Some(v)
}
fn valid_h(c: &Curve, d: &Div, h: &H) -> bool {
    let f = &c.field;
    let u3 = pow(&d.u, 3, f);
    if h.a.degree() > 4 || h.b.degree() != 1 {
        return false;
    }
    let norm = h.a.mul(&h.a, f).sub(&c.f.mul(&h.b.mul(&h.b, f), f), f);
    let (q, r) = norm.divrem(&u3, f);
    if !r.0.is_empty() || q.degree() != 0 || q.coeff(0) == Z {
        return false;
    }
    let Some(mut w) = lift(c, d) else {
        return false;
    };
    w = h.a.add(&h.b.mul(&w, f), f).rem(&u3, f);
    w.0.is_empty()
}
fn lift_with_half(c: &Curve, d: &Div, half:u64) -> Option<Pol> {
    let f = &c.field;
    if !c.valid(d) || d.u.degree() != 3 || !squarefree(&d.u, f) {
        return None;
    }
    let m = pow(&d.u, 3, f);
    let mut w = d.v.clone();
    for _ in 0..2 {
        let wi = inverse(&w, &m, f)?;
        w = w.add(&c.f.mul(&wi, f), f).scale(base(half), f).rem(&m, f)
    }
    if !w.mul(&w, f).sub(&c.f, f).rem(&m, f).0.is_empty() {
        return None;
    }
    Some(w)
}
fn construct(c: &Curve, d: &Div) -> Option<H> {
    if c.is_zero(d) {
        return Some(H {
            a: Pol::one(),
            b: Pol::zero(),
        });
    }
    let f = &c.field;
    let w = lift(c, d)?;
    let m = pow(&d.u, 3, f);
    let cols: Vec<_> = (0..5)
        .map(|i| {
            let mut c = vec![Z; i + 1];
            c[i] = O;
            Pol::new(c)
        })
        .chain([w.clone(), Pol::new(vec![Z, O]).mul(&w, f).rem(&m, f)])
        .collect();
    let matrix = (0..9)
        .map(|i| cols.iter().map(|p| p.coeff(i)).collect())
        .collect();
    let v = nullvector(matrix, f)?;
    let h = H {
        a: Pol::new(v[..5].to_vec()),
        b: Pol::new(v[5..].to_vec()),
    };
    valid_h(c, d, &h).then_some(h)
}
fn det(mut a: Vec<Vec<E>>, f: &Field) -> E {
    let n = a.len();
    let mut out = O;
    for j in 0..n {
        let Some(i) = (j..n).find(|&i| a[i][j] != Z) else {
            return Z;
        };
        if i != j {
            a.swap(i, j);
            out = f.neg(out)
        }
        let p = a[j][j];
        out = f.mul(out, p);
        let inv = f.inv(p);
        for i in j + 1..n {
            let t = f.mul(a[i][j], inv);
            for k in j + 1..n {
                a[i][k] = f.sub(a[i][k], f.mul(t, a[j][k]))
            }
        }
    }
    out
}
fn norm(a: &Pol, u: &Pol, f: &Field) -> E {
    let n = u.degree() as usize;
    let mut cols = Vec::new();
    let mut v = a.rem(u, f);
    for _ in 0..n {
        cols.push(v.clone());
        v = Pol::new(vec![Z, O]).mul(&v, f).rem(u, f)
    }
    det(
        (0..n)
            .map(|i| cols.iter().map(|v| v.coeff(i)).collect())
            .collect(),
        f,
    )
}
fn resultant(a: &Pol, b: &Pol, f: &Field) -> E {
    if b.0.is_empty() {
        return Z;
    }
    if b.degree() == 0 {
        return f.pow(b.coeff(0), a.degree() as u128);
    }
    let m = a.degree();
    let n = b.degree();
    let r = a.rem(b, f);
    if r.0.is_empty() {
        return Z;
    }
    let mut v = f.mul(
        f.pow(*b.0.last().unwrap(), (m - r.degree()) as u128),
        resultant(b, &r, f),
    );
    if m * n % 2 != 0 {
        v = f.neg(v)
    }
    v
}
fn evaluate(c: &Curve, p: &Div, h: &H, x: &Div) -> Option<E> {
    if c.is_zero(p) {
        return Some(O);
    }
    let f = &c.field;
    if !c.valid(x) || x.u.degree() != 3 || !squarefree(&x.u, f) {
        return None;
    }
    let inv = inverse(&p.u, &x.u, f)?;
    let r = x.v.add(&p.v, f).mul(&inv, f).rem(&x.u, f);
    let hv = h.a.add(&h.b.mul(&x.v, f), f);
    Some(f.mul(f.pow(r.coeff(2), 3), norm(&hv, &x.u, f)))
}
fn direct(c: &Curve, p: &Div, h: &H, pts: &[(E, E)]) -> Option<E> {
    if c.is_zero(p) {
        return Some(O);
    }
    let f = &c.field;
    let mut n = vec![Vec::new(); 3];
    let mut d = vec![Vec::new(); 3];
    let mut product = O;
    for &(x, y) in pts {
        let u = p.u.eval(x, f);
        if u == Z {
            return None;
        }
        n[0].push(O);
        n[1].push(x);
        n[2].push(f.mul(f.add(y, p.v.eval(x, f)), f.inv(u)));
        d[0].push(O);
        d[1].push(x);
        d[2].push(f.mul(x, x));
        product = f.mul(product, f.add(h.a.eval(x, f), f.mul(h.b.eval(x, f), y)));
    }
    let den = det(d, f);
    if den == Z {
        return None;
    }
    Some(f.mul(f.pow(f.mul(det(n, f), f.inv(den)), 3), product))
}
fn frobh(h: &H, f: &Field) -> H {
    let fr = |p: &Pol| Pol::new(p.0.iter().map(|&a| f.frob(a)).collect());
    H {
        a: fr(&h.a),
        b: fr(&h.b),
    }
}

fn lift(c:&Curve,d:&Div)->Option<Pol>{lift_with_half(c,d,(P+1)/2)}
