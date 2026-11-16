type U = Vec<u64>;
fn trim(mut a: U) -> U {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn ua(a: &[u64], b: &[u64], minus: bool) -> U {
    let mut c = vec![0; a.len().max(b.len())];
    for i in 0..c.len() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        c[i] = if minus { sub(x, y) } else { add(x, y) };
    }
    trim(c)
}
fn us(a: &[u64], b: u64) -> U {
    trim(a.iter().map(|x| mul(*x, b)).collect())
}
fn um(a: &[u64], b: &[u64]) -> U {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = add(c[i + j], mul(*x, *y));
        }
    }
    trim(c)
}
fn ud(a: &[u64], b: &[u64]) -> (U, U) {
    assert!(!b.is_empty());
    let mut a = trim(a.to_vec());
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    let z = inv(*b.last().unwrap());
    while a.len() >= b.len() {
        let k = a.len() - b.len();
        let c = mul(*a.last().unwrap(), z);
        q[k] = c;
        for (i, x) in b.iter().enumerate() {
            a[k + i] = sub(a[k + i], mul(c, *x));
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn ur(a: &[u64], m: &[u64]) -> U {
    ud(a, m).1
}
fn ug(mut a: U, mut b: U) -> U {
    while !b.is_empty() {
        let r = ur(&a, &b);
        a = b;
        b = r;
    }
    if a.is_empty() {
        a
    } else {
        us(&a, inv(*a.last().unwrap()))
    }
}
fn ui(a: &[u64], m: &[u64]) -> U {
    let (mut r, mut s) = (m.to_vec(), ur(a, m));
    let (mut x, mut y) = (vec![], vec![1]);
    while !s.is_empty() {
        let (q, t) = ud(&r, &s);
        r = s;
        s = t;
        let z = ua(&x, &um(&q, &y), true);
        x = y;
        y = z;
    }
    assert_eq!(r.len(), 1, "nonunit derivative: chart unsupported");
    ur(&us(&x, inv(r[0])), m)
}
fn ux(a: &[u64]) -> U {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| mul(i as u64, *x))
            .collect(),
    )
}
fn eval_u(f: &Poly, v: &[u64], m: &[u64]) -> U {
    let mut xp = vec![vec![1]];
    let mut yp = xp.clone();
    for i in 1..=6 {
        xp.push(ur(&um(&xp[i - 1], &[0, 1]), m));
        yp.push(ur(&um(&yp[i - 1], v), m));
    }
    let mut a = vec![];
    for (i, j) in monos(6) {
        let t = ur(&um(&xp[i], &yp[j]), m);
        a = ur(&ua(&a, &us(&t, f[7 * i + j]), false), m);
    }
    a
}
fn partial(f: &Poly, x: u64, y: u64, axis: usize) -> u64 {
    monos(6).into_iter().fold(0, |z, (i, j)| {
        let k = if axis == 0 { i } else { j };
        if k == 0 {
            return z;
        }
        add(
            z,
            mul(
                mul(f[7 * i + j], k as u64),
                mul(
                    pow(x, (i - usize::from(axis == 0)) as u64),
                    pow(y, (j - usize::from(axis == 1)) as u64),
                ),
            ),
        )
    })
}
fn corrected(v: &[Poly], u: [u64; 4], y: R) -> (Vec<R>, U) {
    assert_eq!(
        ug(u.to_vec(), ux(&u)).len(),
        1,
        "repeated support unsupported"
    );
    let f = curve();
    assert!(eval_u(&f, &y, &u).is_empty());
    let u2=um(&u,&u);
    let m=um(&u2,&u);
    let mut fy = [0; 49];
    for (i, j) in monos(4) {
        if j > 0 {
            fy[7 * i + j - 1] = mul(j as u64, f[7 * i + j]);
        }
    }
    let mut lift=y.to_vec();
    for _ in 0..2 {
      let fv=eval_u(&f,&lift,&m);
      let iv=ui(&eval_u(&fy,&lift,&m),&m);
      lift=ur(&ua(&lift,&ur(&um(&fv,&iv),&m),true),&m);
    }
    assert!(eval_u(&f,&lift,&m).is_empty(),"second_order_Hensel_residual");
    let mut g=u2;
    for n in v {g=ug(g,eval_u(n,&lift,&m));}
    assert!(g.len()<=3,"full_base_multiplicity_at_most2");
    let a = v
        .iter()
        .map(|n| {
            let top = eval_u(n, &lift, &m);
            let (q, r) = ud(&top, &g);
            assert!(r.is_empty());
            let q = ur(&q, &u);
            std::array::from_fn(|i| q.get(i).copied().unwrap_or(0))
        })
        .collect();
    (a, g)
}
fn point_section(n: &Poly, x: u64, y: u64, g: &[u64]) -> u64 {
    let a = peval(g, x);
    if a != 0 {
        return mul(val(n, x, y), inv(a));
    }
    assert_eq!(val(n, x, y), 0);
    let dy = mul(
        sub(0, partial(&curve(), x, y, 0)),
        inv(partial(&curve(), x, y, 1)),
    );
    let d = add(partial(n, x, y, 0), mul(partial(n, x, y, 1), dy));
    mul(d, inv(peval(&ux(g), x)))
}
