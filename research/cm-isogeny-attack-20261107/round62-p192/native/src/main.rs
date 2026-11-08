#![allow(dead_code, unused_imports)]
include!("kernel.rs");
type K = F<211, 210>;
type Point = Pt<211, 210>;
fn rat(p: Point) -> bool {
    p.map(|(x, y)| x.1 == 0 && y.1 == 0).unwrap_or(true)
}
fn on(p: Point, b: i64) -> bool {
    p.map(|(x, y)| y * y == x * x * x + K::c(b)).unwrap_or(true)
}
fn fx(x: K, b: i64) -> K {
    x + K::c(4 * b) / (x * x)
}
fn dx(x: K, b: i64) -> K {
    K::c(1) - K::c(8 * b) / (x * x * x)
}
fn iso(p: Point, b: i64) -> Point {
    p.and_then(|(x, y)| {
        if x == K::c(0) {
            None
        } else {
            Some((fx(x, b), y * dx(x, b)))
        }
    })
}
fn dual(p: Point) -> Point {
    iso(p, 22).map(|(x, y)| (x / K::c(9), y / K::c(27)))
}
fn hd(x: K, y: K) -> K {
    if x == y {
        dx(x, 7)
    } else {
        (fx(x, 7) - fx(y, 7)) / (x - y)
    }
}
fn constant(gg: &[Point], qq: &[Point]) -> K {
    gg.iter()
        .zip(qq)
        .skip(1)
        .fold(K::c(1), |z, (g, q)| z * (g.unwrap().0 - q.unwrap().0))
}
fn arrays(gg: &[Point], qq: &[Point], p: Point, rr: Point) -> Option<(Vec<K>, Vec<K>)> {
    let mut ds = vec![];
    let mut xs = vec![];
    for (&g, &q) in gg.iter().zip(qq) {
        let x = add(p, g, 0)?.0;
        let y = add(rr, q, 0)?.0;
        ds.push(x - y);
        xs.push(x)
    }
    Some((ds, xs))
}
fn contract(ds: &[K], wb: &[K], wc: &[K], reference: bool) -> [K; 3] {
    if reference {
        let n = ds.len();
        let mut pre = vec![K::c(1); n + 1];
        let mut post = pre.clone();
        for i in 0..n {
            pre[i + 1] = pre[i] * ds[i]
        }
        for i in (0..n).rev() {
            post[i] = post[i + 1] * ds[i]
        }
        let mut b = K::c(0);
        let mut c = b;
        for i in 0..n {
            let v = pre[i] * post[i + 1];
            b = b + v * wb[i];
            c = c + v * wc[i]
        }
        [pre[n], b, c]
    } else {
        let mut v = [K::c(1), K::c(0), K::c(0)];
        for i in 0..ds.len() {
            v = [
                v[0] * ds[i],
                v[1] * ds[i] + v[0] * wb[i],
                v[2] * ds[i] + v[0] * wc[i],
            ]
        }
        v
    }
}
fn code(v: [K; 3]) -> Value {
    json!(v.map(|x| x.code()))
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let reference = args.iter().any(|x| x == "--cofactor-reference");
    let g = Some((K::c(3), K::c(33)));
    let gg = graph(g, 0, 199);
    let gp = iso(g, 7);
    let ggp = graph(gp, 0, 199);
    let inv3 = 133;
    let maprows: Vec<_> = gg
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            assert!(on(p, 7));
            let q = iso(p, 7);
            assert!(on(q, 22));
            assert_eq!(q, ggp[i]);
            assert_eq!(dual(q), mul(3, p, 0));
            assert_eq!(mul(inv3, dual(q), 0), p);
            json!({"i":i,"source":pc(p),"image":pc(q),"retracted":true})
        })
        .collect();
    let mut chosen = None;
    let mut attempts = vec![];
    'outer: for x in 0..211 {
        for y in 1..211 {
            let p = Some((K::c(x), K::new(0, y)));
            if !on(p, 7) {
                continue;
            }
            let ip = iso(p, 7);
            let eligible = !rat(mul(2, ip, 0));
            attempts.push(json!({"P":pc(p),"image":pc(ip),"eligible":eligible}));
            if eligible {
                chosen = Some(p);
                break 'outer;
            }
        }
    }
    let p = chosen.expect("frozen shift scan found no eligible point");
    let ip = iso(p, 7);
    assert!(!rat(mul(2, p, 0)));
    assert!(!rat(mul(2, ip, 0)));
    let psi_p = phi(p, 0).unwrap();
    assert_ne!(psi_p.0, K::c(0));
    let base_derivative = dx(psi_p.0, 7);
    assert_ne!(base_derivative, K::c(0));
    let mut rows = vec![];
    let mut probes = vec![];
    let mut norms = 0;
    let mut factor_terms = 0;
    let mut weighted = 0;
    let mut naive_rejections = 0;
    let mut sign_checks = 0;
    for k in [2, 3, 5, 7, 11, 17, 197] {
        let q = mul(k, g, 0);
        let qq = graph(q, 0, 199);
        let qp = iso(q, 7);
        let qqp = graph(qp, 0, 199);
        let c = constant(&gg, &qq);
        let cp = constant(&ggp, &qqp);
        let cr = cp / c;
        let mut probe_count = 0;
        'probes: for i in 1..=12 {
            for j in 1..=12 {
                if i == j {
                    continue;
                }
                let pp = mul(i, p, 0);
                let rr = mul(j, p, 0);
                let Some((ds, xs)) = arrays(&gg, &qq, pp, rr) else {
                    continue;
                };
                let Some((dsp, xsp)) = arrays(&ggp, &qqp, iso(pp, 7), iso(rr, 7)) else {
                    continue;
                };
                if xs.contains(&K::c(0)) {
                    continue;
                }
                let ys: Vec<_> = xs.iter().zip(&ds).map(|(&a, &d)| a - d).collect();
                if ys.contains(&K::c(0)) {
                    continue;
                }
                let hs: Vec<_> = xs.iter().zip(&ys).map(|(&a, &b)| hd(a, b)).collect();
                if hs.contains(&K::c(0)) {
                    continue;
                }
                let (Some(ps), Some(rs)) = (phi(pp, 0), phi(rr, 0)) else {
                    continue;
                };
                if ps.0 == rs.0 || ps.0 == K::c(0) || rs.0 == K::c(0) {
                    continue;
                }
                let hprod = hs.iter().copied().fold(K::c(1), |a, b| a * b);
                let compact = cr * hd(ps.0, rs.0);
                assert_eq!(hprod, compact);
                let ones = vec![K::c(1); 199];
                let v = contract(&ds, &ones, &xs, reference);
                let vp = contract(&dsp, &ones, &xsp, reference);
                let wb: Vec<_> = hs.iter().map(|h| h.inv()).collect();
                let wc: Vec<_> = xs.iter().zip(&wb).map(|(&x, &w)| fx(x, 7) * w).collect();
                let w = contract(&ds, &wb, &wc, reference);
                assert_eq!(vp, w.map(|x| x * hprod));
                assert_eq!(vp[0], v[0] * hprod);
                norms += 2;
                weighted += 1;
                factor_terms += 3 * 199;
                probes.push(json!({"k_fixture":k,"i":i,"j":j,"source":code(v),"destination":code(vp),"weighted_source":code(w),"multiplier":hprod.code(),"compact_multiplier":compact.code()}));
                probe_count += 1;
                if probe_count == 4 {
                    break 'probes;
                }
            }
        }
        assert_eq!(probe_count, 4);
        for n in [0, 1, 2, 3, 5, 17, 198] {
            let u = mul(n, g, 0);
            let rr = add(p, u, 0);
            let (ds, xs) = arrays(&gg, &qq, p, rr).unwrap();
            let (dsp, xsp) = arrays(&ggp, &qqp, ip, iso(rr, 7)).unwrap();
            let ys: Vec<_> = xs.iter().zip(&ds).map(|(&a, &d)| a - d).collect();
            let hs: Vec<_> = xs.iter().zip(&ys).map(|(&a, &b)| hd(a, b)).collect();
            assert!(!hs.contains(&K::c(0)));
            let hprod = hs.iter().copied().fold(K::c(1), |a, b| a * b);
            assert_eq!(hprod, cr * base_derivative);
            let ones = vec![K::c(1); 199];
            let v = contract(&ds, &ones, &xs, reference);
            let vp = contract(&dsp, &ones, &xsp, reference);
            assert_eq!(v[0], K::c(0));
            assert_ne!(v[1], K::c(0));
            let hidden_x = v[2] / v[1];
            let h0 = dx(hidden_x, 7);
            assert_ne!(h0, K::c(0));
            assert_eq!(vp[1], hprod * v[1] / h0);
            assert_eq!(vp[2] / vp[1], fx(hidden_x, 7));
            let wb: Vec<_> = hs.iter().map(|h| h.inv()).collect();
            let wc: Vec<_> = xs.iter().zip(&wb).map(|(&x, &w)| fx(x, 7) * w).collect();
            let w = contract(&ds, &wb, &wc, reference);
            assert_eq!(vp, w.map(|x| x * hprod));
            let src = decode(hidden_x, p, 0, 7, 199);
            let dst = decode(vp[2] / vp[1], ip, 0, 22, 199);
            assert_eq!(dst, iso(src, 7));
            let pulled = mul(inv3, dual(dst), 0);
            assert_eq!(pulled, src);
            let inv = (1..199).find(|x| ((200 - k) % 199) * x % 199 == 1).unwrap();
            assert_eq!(src, mul(inv, u, 0));
            let wrong = vp[1] != hprod * v[1];
            if wrong {
                naive_rejections += 1
            }
            if k == 2 || k == 197 {
                let other = (1..199).find(|x| (1 + k) * x % 199 == 1).unwrap();
                if n != 0 {
                    assert_ne!(src, mul(other, u, 0));
                    sign_checks += 1
                }
            }
            rows.push(json!({"k_fixture":k,"U_fixture":n,"source_coefficients":code(v),"destination_coefficients":code(vp),"weighted_coefficients":code(w),"multiplier":hprod.code(),"hidden_x":hidden_x.code(),"diagonal_derivative":h0.code(),"output":pc(src),"destination_output":pc(dst),"pulled_back":pc(pulled),"naive_unweighted_B_rejected":wrong}));
            norms += 2;
            weighted += 1;
            factor_terms += 3 * 199;
        }
    }
    assert!(naive_rejections > 0);
    let out = json!({"all_checks_pass":true,"p":211,"r":199,"source":[0,7],"destination":[0,22],"degree":3,"shift_selection":attempts,"shift":pc(p),"image_shift":pc(ip),"map_retractions":maprows,"generic_probes":probes,"marked_rows":rows,"naive_unweighted_B_rejections":naive_rejections,"sign_controls":sign_checks,"norm_calls":norms,"weighted_contracts":weighted,"total_factor_terms":factor_terms,"G_graph_points":398,"changing_target_graph_points":7*398,"scope":"complete degree3 control; exact coefficient/action transport; no cheap weighted norm inferred"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let v: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, v, "changed transport certificate")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
