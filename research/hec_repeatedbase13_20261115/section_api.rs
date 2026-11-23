pub struct Sections {
    v: Vec<Poly>,
    h: Vec<Vec<u64>>,
    ts: Vec<[usize; 3]>,
}
impl Sections {
    pub fn new() -> Self {
        let (v, h, ts) = build();
        Self { v, h, ts }
    }
    pub fn coords(&self, u: [u64; 4], y: [u64; 3]) -> Option<[u64; 8]> {
        self.coords_chart(u, y, false)
    }
    pub fn coords_chart(&self, u: [u64; 4], y: [u64; 3], swap: bool) -> Option<[u64; 8]> {
        let f = swap_form(&curve(), 4, swap);
        let nums: Vec<_> = self.v.iter().map(|n| swap_form(n, 3, swap)).collect();
        if ug(u.to_vec(), ux(&u)).len() != 1 || !eval_u(&f, &y, &u).is_empty() {
            return None;
        }

        let mut fy = [0; 49];
        for (i, j) in monos(4) {
            if j > 0 {
                fy[7 * i + j - 1] = mul(j as u64, f[7 * i + j]);
            }
        }
        if ug(eval_u(&fy, &y, &u), u.to_vec()).len() != 1 {
            return None;
        }
        let (a, _) = corrected_chart(&nums, u, y, &f);
        let ev: Vec<_> = self
            .ts
            .iter()
            .map(|t| polar(a[t[0]], a[t[1]], a[t[2]], u))
            .collect();
        let c = std::array::from_fn(|i| {
            self.h[i]
                .iter()
                .zip(&ev)
                .fold(0, |z, (a, b)| add(z, mul(*a, *b)))
        });
        c.iter().any(|x| *x != 0).then_some(c)
    }
}
pub fn root_degree(u: [u64; 4]) -> usize {
    let mut a = vec![1];
    let mut x = vec![0, 1];
    let mut e = crate::field::p();
    while e > 0 {
        if e & 1 != 0 {
            a = ur(&um(&a, &x), &u)
        }
        x = ur(&um(&x, &x), &u);
        e >>= 1;
    }
    ug(u.to_vec(), ua(&a, &[0, 1], true)).len() - 1
}
pub fn verify_canonical_line() {
    let cfg = crate::config::get();
    let f = curve();
    let o = cfg.o;
    let r = div_root(
        &div_root(&tangent_restrict(&f, cfg.tangent[0], cfg.tangent[1]), o[0]),
        o[0],
    );
    assert_eq!(r.len(), 3);
    let rr: Vec<_> = r.iter().map(|v| mul(*v, inv(r[2]))).collect();
    assert_eq!(rr, cfg.residual);
    assert_ne!(peval(&r, o[0]), 0);
    for p in cfg.canonical {
        assert_eq!(val(&f, p[0], p[1]), 0);
    }
    println!("CANONICAL_CONTROL runtime_tangent_and_line=true");
}

pub fn linear_kernel(m: Vec<Vec<u64>>, n: usize) -> (usize, Vec<Vec<u64>>) {
    kernel(m, n)
}

fn swap_form(f: &Poly, d: usize, swap: bool) -> Poly {
    if !swap {
        return *f;
    }
    let mut g = [0; 49];
    for (i, j) in monos(d) {
        g[7 * (d - i - j) + j] = f[7 * i + j];
    }
    g
}
fn corrected_chart(v: &[Poly], u: [u64; 4], y: R, f: &Poly) -> (Vec<R>, U) {
    assert_eq!(
        ug(u.to_vec(), ux(&u)).len(),
        1,
        "repeated support unsupported"
    );

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
