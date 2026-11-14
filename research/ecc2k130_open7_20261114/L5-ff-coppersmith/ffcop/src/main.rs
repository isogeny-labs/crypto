mod exist;
mod extract;
mod field;
mod gf2x;
mod lattice;
mod popov;

use field::{Fld, Pt};
use gf2x::{derive_seed, Rng, F64, P};
use lattice::{bi_eval, bi_pow, bi_mul, bi_scale, bi_shift, build, reduce, Bi, Form, Lat};

const PRIMES: [u32; 13] = [17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 61];

fn small_field_for(bound: usize) -> Option<F64> {
    // need D > bound (candidates have deg <= bound)
    PRIMES.iter().find(|&&p| p as usize > bound + 1).map(|&p| F64::new(p))
}

fn args() -> Vec<String> { std::env::args().collect() }
fn arg_u(a: &[String], i: usize) -> usize { a[i].parse().unwrap() }
fn parse_list(s: &str) -> Vec<usize> {
    if let Some((lo, hi)) = s.split_once(':') { (lo.parse().unwrap()..=hi.parse().unwrap()).collect() } else { s.split(',').map(|x| x.parse().unwrap()).collect() }
}

fn s3_poly(fld: &Fld, c: &P) -> Bi {
    let mut g = Bi::new();
    let c2 = fld.sqr(c);
    g.insert((2, 2), P::one());
    if !c2.is_zero() { g.insert((2, 0), c2.clone()); g.insert((0, 2), c2); }
    if !c.is_zero() { g.insert((1, 1), c.clone()); }
    g.insert((0, 0), P::one());
    g
}
fn e3_poly(fld: &Fld, c: &P) -> Bi {
    let mut g = Bi::new();
    let c2 = fld.sqr(c);
    g.insert((0, 2), P::one());
    if !c2.is_zero() { g.insert((2, 0), c2); }
    if !c.is_zero() { g.insert((0, 1), c.clone()); }
    g.insert((0, 0), P::one());
    g
}
fn generic_poly(r: &[P; 4]) -> Bi {
    let mut g = Bi::new();
    g.insert((2, 2), P::one());
    for (m, x) in [((2, 0), &r[0]), ((0, 2), &r[1]), ((1, 1), &r[2]), ((0, 0), &r[3])] { if !x.is_zero() { g.insert(m, x.clone()); } }
    g
}
fn eval_mod(fld: &Fld, g: &Bi, a: &P, b: &P) -> P { fld.red(&bi_eval(g, a, b)) }

struct Planted { p1: Pt, p2: Pt, r: Pt }
fn plant(fld: &Fld, rng: &mut Rng, l: usize) -> Planted {
    loop {
        let p1 = fld.rand_point(rng, l);
        let p2 = fld.rand_point(rng, l);
        if p1.x == p2.x { continue; }
        let r = fld.add(&p1, &p2);
        if r.inf { continue; }
        return Planted { p1, p2, r };
    }
}

#[allow(clippy::too_many_arguments)]
fn run_arm(fld: &Fld, arm: &str, form: Form, g: &Bi, wts: (usize, usize), k: usize, l: usize, planted: Option<(P, P)>, bounds: (usize, usize), rng: &mut Rng, extra: &str) -> String {
    let n = fld.n;
    let lb = l - 1;
    let lat: Lat = build(&form, g, &fld.f, k, wts);
    let dim = lat.dim();
    let det = lat.det_deg(lb);
    let rows = lat.rows(&lat.gens, lb);
    let red = reduce(rows);
    let sumdeg: i64 = red.degs.iter().sum();
    let kn = (k * n) as i64;
    let short: Vec<usize> = (0..red.rows.len()).filter(|&i| red.degs[i] < kn).collect();
    let pred1 = det / dim as i64 < kn;
    let wp = popov::is_weak_popov(&red.rows);
    // vanishing diagnostics (uses planted root)
    let mut vanish = -1i64;
    let mut short_vanish_ok = true;
    let hs: Vec<Bi> = red.rows.iter().map(|r| lat.unscale(r, lb)).collect();
    if let Some((a, b)) = &planted {
        vanish = hs.iter().filter(|h| bi_eval(h, a, b).is_zero()).count() as i64;
        for &i in &short { if !bi_eval(&hs[i], a, b).is_zero() { short_vanish_ok = false; } }
    }
    // extraction (does not use the planted root)
    let (mut pairs, mut rz, mut ncand, mut ngen, mut recovered, mut status) = (0u32, 0u32, 0usize, 0usize, false, "none");
    let mut genuine_list = String::new();
    if short.len() >= 2 {
        match small_field_for(bounds.0.max(bounds.1)) {
            None => status = "skipped_D",
            Some(kf) => {
                let sh: Vec<Bi> = short.iter().map(|&i| hs[i].clone()).collect();
                let ex = extract::extract(&kf, &sh, bounds.0, bounds.1, rng, 8);
                pairs = ex.pairs_tried; rz = ex.res_zero; ncand = ex.candidates.len();
                status = if ex.resolved { "resolved" } else { "all_res_zero" };
                let mut gl = vec![];
                for (a, b) in &ex.candidates {
                    if eval_mod(fld, g, a, b).is_zero() {
                        ngen += 1;
                        gl.push(format!("[\"{}\",\"{}\"]", a.hex(), b.hex()));
                        if let Some((pa, pb)) = &planted {
                            let sym = matches!(form, Form::XBox) && wts.0 == wts.1;
                            if (a == pa && b == pb) || (sym && a == pb && b == pa) { recovered = true; }
                        }
                    }
                }
                genuine_list = gl.join(",");
            }
        }
    }
    let degs: Vec<String> = red.degs.iter().map(|d| d.to_string()).collect();
    format!(
        "{{\"n\":{},\"k\":{},\"l\":{},\"arm\":\"{}\",{}\"dim\":{},\"det_deg\":{},\"sum_row_deg\":{},\"det_identity\":{},\"weak_popov\":{},\"kn\":{},\"pred_ge1_short\":{},\"short_rows\":{},\"deg_min\":{},\"deg_2nd\":{},\"deg_max\":{},\"transforms\":{},\"word_xors\":{},\"rows_vanish\":{},\"short_rows_vanish_at_planted\":{},\"extract\":\"{}\",\"pairs_tried\":{},\"res_zero\":{},\"candidates\":{},\"genuine\":{},\"genuine_pairs\":[{}],\"recovered\":{},\"row_degs\":[{}]}}",
        n, k, l, arm, extra, dim, det, sumdeg, sumdeg == det, wp, kn, pred1, short.len(), red.degs[0], red.degs.get(1).copied().unwrap_or(-1), red.degs.last().unwrap(), red.st.transforms, red.st.word_xors, vanish, short_vanish_ok, status, pairs, rz, ncand, ngen, genuine_list, recovered, degs.join(",")
    )
}

fn cmd_lattice(a: &[String]) {
    // lattice <n> <k-list> <l-list> <trials> <seed> <arms comma>
    let n = arg_u(a, 2);
    let ks = parse_list(&a[3]);
    let ls = parse_list(&a[4]);
    let trials = arg_u(a, 5);
    let seed: u64 = a[6].parse().unwrap();
    let arms: Vec<&str> = a[7].split(',').collect();
    let fld = Fld::new(n);
    for &k in &ks { for &l in &ls { for t in 0..trials {
        let mut rng = Rng::new(derive_seed(&[seed, n as u64, k as u64, l as u64, t as u64]));
        let pl = plant(&fld, &mut rng, l);
        let (x1, x2, c) = (pl.p1.x.clone(), pl.p2.x.clone(), pl.r.x.clone());
        assert!(fld.s3(&x1, &x2, &c).is_zero());
        let cu = fld.rand_point(&mut rng, n).x;
        let r: [P; 3] = [fld.rand(&mut rng), fld.rand(&mut rng), fld.rand(&mut rng)];
        let mut gp = generic_poly(&[r[0].clone(), r[1].clone(), r[2].clone(), P::zero()]);
        let r4 = eval_mod(&fld, &gp, &x1, &x2);
        if !r4.is_zero() { gp.insert((0, 0), r4); }
        let gu = generic_poly(&[r[0].clone(), r[1].clone(), r[2].clone(), fld.rand(&mut rng)]);
        // K2: P2' = phi(Q2), deg x(Q2) < l
        let q2 = loop { let q = fld.rand_point(&mut rng, l); if q.x != x1 { break q; } };
        let p2f = fld.frob(&q2);
        let rk = fld.add(&pl.p1, &p2f);
        let lb = l - 1;
        let extra = format!("\"trial\":{},\"seed\":{},", t, seed);
        for arm in &arms {
            let mut arng = Rng::new(derive_seed(&[seed, n as u64, k as u64, l as u64, t as u64, 77]));
            let line = match *arm {
                "s3p" => run_arm(&fld, arm, Form::XBox, &s3_poly(&fld, &c), (1, 1), k, l, Some((x1.clone(), x2.clone())), (lb, lb), &mut arng, &extra),
                "s3u" => run_arm(&fld, arm, Form::XBox, &s3_poly(&fld, &cu), (1, 1), k, l, None, (lb, lb), &mut arng, &extra),
                "genp" => run_arm(&fld, arm, Form::XBox, &gp, (1, 1), k, l, Some((x1.clone(), x2.clone())), (lb, lb), &mut arng, &extra),
                "genu" => run_arm(&fld, arm, Form::XBox, &gu, (1, 1), k, l, None, (lb, lb), &mut arng, &extra),
                "e3p" => run_arm(&fld, arm, Form::EForm, &e3_poly(&fld, &c), (1, 2), k, l, Some((x1.add(&x2), x1.mul(&x2))), (lb, 2 * lb), &mut arng, &extra),
                "e3u" => run_arm(&fld, arm, Form::EForm, &e3_poly(&fld, &cu), (1, 2), k, l, None, (lb, 2 * lb), &mut arng, &extra),
                "k2p" => {
                    if rk.inf { continue; }
                    assert!(fld.s3(&x1, &p2f.x, &rk.x).is_zero());
                    run_arm(&fld, arm, Form::XBox, &s3_poly(&fld, &rk.x), (1, 2), k, l, Some((x1.clone(), p2f.x.clone())), (lb, 2 * lb), &mut arng, &extra)
                }
                _ => panic!("arm"),
            };
            println!("{}", line);
        }
    }}}
}

fn cmd_uni(a: &[String]) {
    // uni <n> <d> <k-list> <l-list> <trials> <seed>
    let n = arg_u(a, 2);
    let d = arg_u(a, 3);
    let ks = parse_list(&a[4]);
    let ls = parse_list(&a[5]);
    let trials = arg_u(a, 6);
    let seed: u64 = a[7].parse().unwrap();
    let fld = Fld::new(n);
    for &k in &ks { for &l in &ls { for t in 0..trials {
        let mut rng = Rng::new(derive_seed(&[seed, 9, n as u64, d as u64, k as u64, l as u64, t as u64]));
        let x0 = loop { let x = P::random(&mut rng, l); if !x.is_zero() { break x; } };
        let mut p = Bi::new();
        p.insert((d, 0), P::one());
        for i in 1..d { let r = fld.rand(&mut rng); if !r.is_zero() { p.insert((i, 0), r); } }
        let r0 = eval_mod(&fld, &p, &x0, &P::zero());
        if !r0.is_zero() { p.insert((0, 0), r0); }
        let lb = l - 1;
        let mons: Vec<(usize, usize)> = (0..=d * k).map(|m| (m, 0)).collect();
        let idx = mons.iter().enumerate().map(|(i, m)| (*m, i)).collect();
        let mut gens = vec![]; let mut diag = vec![];
        for i in 0..k { for j in 0..d {
            gens.push(bi_scale(&bi_shift(&bi_pow(&p, i), (j, 0)), &fld.f.pow(k - i)));
            diag.push(((k - i) * n) as i64);
        }}
        gens.push(bi_pow(&p, k)); diag.push(0);
        let lat = Lat { mons, idx, wts: (1, 0), gens, diag_deg: diag };
        let det = lat.det_deg(lb);
        let red = reduce(lat.rows(&lat.gens, lb));
        let kn = (k * n) as i64;
        let short: Vec<usize> = (0..red.rows.len()).filter(|&i| red.degs[i] < kn).collect();
        let mut recovered = false; let mut ngen = 0;
        let kf = small_field_for(lb).unwrap();
        for &i in &short {
            let h = lat.unscale(&red.rows[i], lb);
            assert!(bi_eval(&h, &x0, &P::zero()).is_zero());
            let bf = extract::to_bif(&kf, &h);
            let u: Vec<u64> = (0..=bf.da).map(|a| bf.c[a][0]).collect();
            for r in extract::roots(&kf, &u, &mut rng) {
                if 64 - r.leading_zeros() as usize > l { continue; }
                let rp = P::from_u64(r);
                if eval_mod(&fld, &p, &rp, &P::zero()).is_zero() { ngen += 1; if rp == x0 { recovered = true; } }
            }
            if recovered { break; }
        }
        let sumdeg: i64 = red.degs.iter().sum();
        println!("{{\"n\":{},\"d\":{},\"k\":{},\"l\":{},\"arm\":\"uni\",\"trial\":{},\"dim\":{},\"det_deg\":{},\"det_identity\":{},\"kn\":{},\"pred_ge1_short\":{},\"short_rows\":{},\"deg_min\":{},\"transforms\":{},\"word_xors\":{},\"genuine\":{},\"recovered\":{}}}",
            n, d, k, l, t, lat.dim(), det, sumdeg == det, kn, det / lat.dim() as i64 < kn, short.len(), red.degs[0], red.st.transforms, red.st.word_xors, ngen, recovered);
    }}}
}

fn cmd_kobaug(a: &[String]) {
    // kobaug <n> <k-list> <l-list> <trials> <seed>
    let n = arg_u(a, 2);
    let ks = parse_list(&a[3]);
    let ls = parse_list(&a[4]);
    let trials = arg_u(a, 5);
    let seed: u64 = a[6].parse().unwrap();
    let fld = Fld::new(n);
    for &k in &ks { for &l in &ls { for t in 0..trials {
        let mut rng = Rng::new(derive_seed(&[seed, 11, n as u64, k as u64, l as u64, t as u64]));
        let pl = plant(&fld, &mut rng, l);
        let (x1, x2, c) = (pl.p1.x.clone(), pl.p2.x.clone(), pl.r.x.clone());
        let g = s3_poly(&fld, &c);
        let cp = fld.sqr(&c); // x(phi R)
        // H(X1,X2) = S3(X1^2, X2^2, c') : conjugate relation in the original small unknowns
        let mut h = Bi::new();
        h.insert((4, 4), P::one());
        let cp2 = fld.sqr(&cp);
        h.insert((4, 0), cp2.clone()); h.insert((0, 4), cp2);
        h.insert((2, 2), cp.clone());
        h.insert((0, 0), P::one());
        assert!(eval_mod(&fld, &h, &x1, &x2).is_zero());
        let g2 = bi_mul(&g, &g);
        let h_minus_g2_div_f = h.keys().chain(g2.keys()).all(|m| {
            let d = h.get(m).cloned().unwrap_or_default().add(&g2.get(m).cloned().unwrap_or_default());
            d.rem(&fld.f).is_zero()
        });
        let lb = l - 1;
        let lat = build(&Form::XBox, &g, &fld.f, k, (1, 1));
        let plain = reduce(lat.rows(&lat.gens, lb));
        let mut gens = lat.gens.clone();
        let mut added = 0;
        for j in 1..=k { for i in 0..=(k - j) {
            let base = bi_scale(&bi_mul(&bi_pow(&g, i), &bi_pow(&h, j)), &fld.f.pow(k - i - j));
            let span = 2 * i + 4 * j;
            if span > 2 * k { continue; }
            for sa in 0..=(2 * k - span) { for sb in 0..=(2 * k - span) { gens.push(bi_shift(&base, (sa, sb))); added += 1; } }
        }}
        let aug = reduce(lat.rows(&gens, lb));
        let kn = (k * n) as i64;
        let sp: i64 = plain.degs.iter().sum();
        let sa: i64 = aug.degs.iter().sum();
        println!("{{\"n\":{},\"k\":{},\"l\":{},\"arm\":\"kobaug\",\"trial\":{},\"H_eq_g2_mod_f\":{},\"added_rows\":{},\"rank_plain\":{},\"rank_aug\":{},\"sumdeg_plain\":{},\"sumdeg_aug\":{},\"short_plain\":{},\"short_aug\":{},\"profiles_equal\":{},\"transforms_aug\":{}}}",
            n, k, l, t, h_minus_g2_div_f, added, plain.degs.len(), aug.degs.len(), sp, sa,
            plain.degs.iter().filter(|&&d| d < kn).count(), aug.degs.iter().filter(|&&d| d < kn).count(), plain.degs == aug.degs, aug.st.transforms);
    }}}
}

fn cmd_exist(a: &[String]) {
    // exist <n> <lmax> <trials> <seed>
    let n = arg_u(a, 2);
    let lmax = arg_u(a, 3) as u32;
    let trials = arg_u(a, 4);
    let seed: u64 = a[5].parse().unwrap();
    let f = exist::F::new(n);
    for t in 0..trials {
        let mut rng = Rng::new(derive_seed(&[seed, 3, n as u64, t as u64]));
        let x3 = exist::rand_x(&f, &mut rng);
        let o = exist::search(&f, x3, lmax);
        let h: Vec<String> = o.hist.iter().map(|x| x.to_string()).collect();
        println!("{{\"n\":{},\"trial\":{},\"seed\":{},\"lmax\":{},\"x3\":\"{:x}\",\"l_min\":{},\"x1_evals\":{},\"inversions\":{},\"hist_ordered\":[{}]}}", n, t, seed, lmax, o.x3, o.l_min, o.x1_evals, o.inversions, h.join(","));
    }
}

fn cmd_theory(a: &[String]) {
    // theory <n-list> <k-list> : real-valued L threshold from floor(det/N) < kn, per form
    let ns = parse_list(&a[2]);
    let ks = parse_list(&a[3]);
    for &n in &ns { for &k in &ks {
        let f = P::mono(n).add(&P::one());
        for (name, form, w) in [("xbox", Form::XBox, (1usize, 1usize)), ("eform", Form::EForm, (1, 2)), ("k2", Form::XBox, (1, 2))] {
            let g = if matches!(form, Form::XBox) { let mut g = Bi::new(); g.insert((2, 2), P::one()); g.insert((2, 0), P::one()); g.insert((0, 2), P::one()); g.insert((1, 1), P::one()); g.insert((0, 0), P::one()); g } else { let mut g = Bi::new(); g.insert((0, 2), P::one()); g.insert((2, 0), P::one()); g.insert((0, 1), P::one()); g.insert((0, 0), P::one()); g };
            let lat = build(&form, &g, &f, k, w);
            let nn = lat.dim() as i64;
            let sd: i64 = lat.diag_deg.iter().sum();
            let sw: i64 = lat.mons.iter().map(|m| lat.w(*m) as i64).sum();
            // largest integer L with floor((sd + L sw)/N) < kn  <=>  sd + L sw < kn N
            let knn = (k * n) as i64 * nn;
            let lmax_l = if sd >= knn { -1 } else { (knn - sd - 1) / sw };
            println!("{{\"n\":{},\"k\":{},\"form\":\"{}\",\"dim\":{},\"sum_diag\":{},\"sum_w\":{},\"beta_det\":{:.6},\"L_det\":{},\"l_det\":{}}}",
                n, k, name, nn, sd, sw, (knn - sd) as f64 / (sw as f64 * n as f64), lmax_l, lmax_l + 1);
        }
    }}
}

fn cmd_selftest() {
    for n in [13usize, 19, 29, 37, 53, 61, 131] {
        let fld = Fld::new(n);
        let mut rng = Rng::new(1);
        for _ in 0..20 {
            let a = fld.rand(&mut rng);
            if a.is_zero() { continue; }
            assert!(fld.mul(&a, &fld.inv(&a)).is_one());
            let p1 = fld.rand_point(&mut rng, n);
            let p2 = fld.rand_point(&mut rng, n);
            assert!(fld.on_curve(&p1) && fld.on_curve(&p2));
            let r = fld.add(&p1, &p2);
            assert!(fld.on_curve(&r));
            assert!(fld.on_curve(&fld.frob(&p1)));
            assert!(fld.s3(&p1.x, &p2.x, &r.x).is_zero());
            let d = fld.add(&p1, &p1);
            assert!(fld.on_curve(&d));
        }
        if n <= 63 {
            let f = exist::F::new(n);
            let mut r2 = Rng::new(2);
            for _ in 0..1000 {
                let a = r2.next() & f.mask; let b = r2.next() & f.mask;
                let pa = P::from_u64(a); let pb = P::from_u64(b);
                assert_eq!(f.mul(a, b), fld.mul(&pa, &pb).to_u64());
                if a != 0 { assert_eq!(f.mul(a, f.inv(a)), 1); }
                assert_eq!(f.tr(a), fld.tr(&pa));
                if f.tr(a) == 0 { let z = f.htr(a); assert_eq!(f.sqr(z) ^ z, a); }
            }
        }
        println!("selftest n={} f={} ok", n, fld.f.hex());
    }
    // small field + root finding
    let k = F64::new(17);
    let mut rng = Rng::new(5);
    let rts = [3u64, 77, 1000, 5];
    let mut u = vec![1u64];
    for &r in &rts { u = extract::umul(&k, &u, &vec![r, 1]); }
    let mut got = extract::roots(&k, &u, &mut rng);
    got.sort();
    let mut want = rts.to_vec(); want.sort();
    assert_eq!(got, want);
    println!("selftest roots ok");
}

fn main() {
    let a = args();
    match a.get(1).map(|s| s.as_str()) {
        Some("selftest") => cmd_selftest(),
        Some("lattice") => cmd_lattice(&a),
        Some("uni") => cmd_uni(&a),
        Some("kobaug") => cmd_kobaug(&a),
        Some("exist") => cmd_exist(&a),
        Some("theory") => cmd_theory(&a),
        _ => eprintln!("usage: ffcop selftest|lattice|uni|kobaug|exist|theory ..."),
    }
}
