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
        let _phase=crate::phase::Span::new("section_coords");
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
fn corrected_chart_reference(v: &[Poly], u: [u64; 4], y: R, f: &Poly) -> (Vec<R>, U) {
    assert_eq!(
        ug(u.to_vec(), ux(&u)).len(),
        1,
        "repeated support unsupported"
    );

    assert!(eval_u(&f, &y, &u).is_empty());
    let mut g = u.to_vec();
    for n in v {
        g = ug(g, eval_u(n, &y, &u));
    }
    assert!(g.len() <= 3, "base divisor degree too large");
    let m = um(&u, &u);
    let mut fy = [0; 49];
    for (i, j) in monos(4) {
        if j > 0 {
            fy[7 * i + j - 1] = mul(j as u64, f[7 * i + j]);
        }
    }
    let fv = eval_u(&f, &y, &m);
    let iv = ui(&eval_u(&fy, &y, &m), &m);
    let lift = ur(&ua(&y, &ur(&um(&fv, &iv), &m), true), &m);
    assert!(eval_u(&f, &lift, &m).is_empty(), "Hensel curve residual");
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
static SHORT:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
static FALL:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);
fn corrected_chart(v:&[Poly],u:[u64;4],y:R,f:&Poly)->(Vec<R>,U){assert_eq!(ug(u.to_vec(),ux(&u)).len(),1);assert!(eval_u(f,&y,&u).is_empty());let vals:Vec<U>=v.iter().map(|n|eval_u(n,&y,&u)).collect();let mut g=u.to_vec();for n in &vals{g=ug(g,n.clone());}let fast=if g==vec![1]{SHORT.fetch_add(1,std::sync::atomic::Ordering::Relaxed);(vals.into_iter().map(|q|std::array::from_fn(|i|q.get(i).copied().unwrap_or(0))).collect(),g)}else{FALL.fetch_add(1,std::sync::atomic::Ordering::Relaxed);corrected_chart_reference(v,u,y,f)};fast}
pub fn special_control(){let cfg=crate::config::get();let r=cfg.residual;let o=cfg.o[0];let u=[sub(0,mul(o,r[0])),sub(r[0],mul(o,r[1])),sub(r[1],o),1];let [a,b]=cfg.tangent;let y=[b,a,0];let(v,_,_)=build();let(_,g)=corrected_chart(&v,u,y,&curve());assert!(g.len()>1,"base_control_not_exceptional");println!("SPECIAL_CONTROL base_factor_degree={} exact_reference_match=true",g.len()-1);}
pub fn counts(){let a=SHORT.load(std::sync::atomic::Ordering::Relaxed);let b=FALL.load(std::sync::atomic::Ordering::Relaxed);assert!(a>0&&b>0);println!("PASS jetfree_numerator_checks={a} base_cancellation_fallback_checks={b} no_base_reference_calls=false exceptional_reference_retained=true timing_claim=false");}
