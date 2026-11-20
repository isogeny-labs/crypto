//! Residue/character leads on Q = kG: R1 (homomorphic residues), R2 (non-homomorphic bias),
//! R3 (Semaev pre-filter), R4 (rho equivalence classes). Protocol: ../PROTOCOL.md (frozen).
mod curves;
mod util;
use curves::*;
use std::collections::HashSet;
use std::fmt::Write as _;
use util::*;

const ALPHA: f64 = 0.01;
const K_AA: usize = 64;
const RES_MODS: [u64; 7] = [2, 3, 4, 5, 7, 8, 16];

/// An instance with <G> enumerated and predicate table.
struct Inst {
    label: String,
    field: &'static str,
    q_bits: f64, // log2 q
    n: u64,
    xs: Vec<u32>,
    ys: Vec<u32>,
    preds: Vec<(String, Vec<i8>)>, // +-1 per k (k=0 -> +1)
}

fn prime_preds(c: &PCurve, xs: &[u32], ys: &[u32], bits: u32) -> Vec<(String, Vec<i8>)> {
    let p = c.p;
    let qr = qr_table(p);
    let sgn = |v: i8| if v >= 0 { 1i8 } else { -1i8 };
    let n = xs.len();
    let mk = |f: &dyn Fn(u64, u64) -> i8| -> Vec<i8> {
        (0..n).map(|k| if xs[k] == INF { 1 } else { f(xs[k] as u64, ys[k] as u64) }).collect()
    };
    vec![
        ("chi(x)".into(), mk(&|x, _| sgn(qr[x as usize]))),
        ("chi(y)".into(), mk(&|_, y| sgn(qr[y as usize]))),
        ("chi(x+1)".into(), mk(&|x, _| sgn(qr[((x + 1) % p) as usize]))),
        ("lsb(x)".into(), mk(&|x, _| if x & 1 == 0 { 1 } else { -1 })),
        ("lsb(y)".into(), mk(&|_, y| if y & 1 == 0 { 1 } else { -1 })),
        (format!("bit{}(x)", bits / 2), mk(&|x, _| if (x >> (bits / 2)) & 1 == 0 { 1 } else { -1 })),
    ]
}
fn binary_preds(c: &BCurve, xs: &[u32], ys: &[u32]) -> Vec<(String, Vec<i8>)> {
    let k = &c.k;
    let n = xs.len();
    let s = |b: u32| if b == 0 { 1i8 } else { -1i8 };
    let mk = |f: &dyn Fn(u32, u32) -> i8| -> Vec<i8> {
        (0..n).map(|i| if xs[i] == INF { 1 } else { f(xs[i], ys[i]) }).collect()
    };
    vec![
        ("Tr(t*x)".into(), mk(&|x, _| s(k.tr(k.mul(2, x))))),
        ("Tr(y)".into(), mk(&|_, y| s(k.tr(y)))),
        ("Tr(y/x)".into(), mk(&|x, y| if x == 0 { 1 } else { s(k.tr(k.mul(y, k.inv(x)))) })),
        ("x0".into(), mk(&|x, _| s(x & 1))),
        ("y0".into(), mk(&|_, y| s(y & 1))),
        ("Tr(x)[R1:constant]".into(), mk(&|x, _| s(k.tr(x)))),
    ]
}

fn build_instances() -> Vec<Inst> {
    let mut v = vec![];
    for bits in [16u32, 18, 20] {
        for c in family_p(bits, 2) {
            let (xs, ys) = enum_prime(&c);
            let preds = prime_preds(&c, &xs, &ys, bits);
            v.push(Inst {
                label: format!("P{bits}_p{}_A{}_B{}", c.p, c.a, c.b),
                field: "prime",
                q_bits: (c.p as f64).log2(),
                n: c.n,
                xs,
                ys,
                preds,
            });
            eprintln!("built {}", v.last().unwrap().label);
        }
    }
    for m in [11u32, 13, 17, 19] {
        for c in family_b(m) {
            let (xs, ys) = enum_binary(&c);
            let preds = binary_preds(&c, &xs, &ys);
            v.push(Inst { label: format!("{}_n{}", c.label, c.n), field: "binary", q_bits: m as f64, n: c.n, xs, ys, preds });
            eprintln!("built {}", v.last().unwrap().label);
        }
    }
    v
}

// ============================ R1 ============================
fn r1(out: &mut String) {
    let mut rows = vec![];
    // R1-bin: Tr(x(P)) = Tr(a) on <G>; P in 2E <=> Tr(x)=Tr(a) on E (x != 0)
    for m in [11u32, 13, 17, 19] {
        for c in family_b(m) {
            let k = &c.k;
            let (xs, _) = enum_binary(&c);
            let tra = k.tr(c.a);
            let viol_g = xs.iter().filter(|&&x| x != INF && k.tr(x) != tra).count();
            // full group and 2E
            let mut pts = vec![];
            pts.push((0u32, k.pow(c.b, 1u64 << (m - 1))));
            for x in 1..(1u32 << m) {
                if let Some((y0, y1)) = bpoints_at(k, c.a, c.b, x) {
                    pts.push((x, y0));
                    pts.push((x, y1));
                }
            }
            assert_eq!(pts.len() as u64 + 1, c.order);
            let twoe: HashSet<(u32, u32)> = pts.iter().filter_map(|&p| badd(&c, Some(p), Some(p))).collect();
            let mut mismatch = 0;
            let mut in2e = 0;
            let mut tr_cond = 0;
            for &(x, y) in &pts {
                if x == 0 {
                    continue;
                }
                let a_ = twoe.contains(&(x, y));
                let b_ = k.tr(x) == tra;
                in2e += a_ as u64;
                tr_cond += b_ as u64;
                if a_ != b_ {
                    mismatch += 1;
                }
            }
            rows.push(format!(
                "{{\"claim\":\"R1-bin\",\"curve\":\"{}\",\"m\":{m},\"order\":{},\"h\":{},\"n\":{},\"f\":{},\"violations_on_G\":{viol_g},\"E_affine_x_nonzero\":{},\"in_2E\":{in2e},\"tr_condition\":{tr_cond},\"mismatch_2E_vs_trace\":{mismatch}}}",
                c.label, c.order, c.h, c.n, c.k.f, pts.len() - 1
            ));
            eprintln!("R1-bin {} viol {viol_g} mismatch {mismatch}", c.label);
        }
    }
    // R1-2desc on P4
    for bits in [16u32, 18, 20] {
        let pc = family_p4(bits);
        let c = &pc.c;
        let p = c.p;
        let e = pc.e;
        let delta = |pt: (u64, u64)| -> (i64, i64) {
            let f = |i: usize| -> i64 {
                let (x, _) = pt;
                if x == e[i] {
                    let j = (i + 1) % 3;
                    let l = (i + 2) % 3;
                    legendre((e[i] + p - e[j]) % p * ((e[i] + p - e[l]) % p) % p, p)
                } else {
                    legendre((x + p - e[i]) % p, p)
                }
            };
            (f(0), f(1))
        };
        // <G>
        let (xs, ys) = enum_prime(c);
        let mut nonconst_g = 0;
        for k in 1..xs.len() {
            if delta((xs[k] as u64, ys[k] as u64)) != (1, 1) {
                nonconst_g += 1;
            }
        }
        // full E: image of delta, and homomorphism check on random pairs
        let mut img = HashSet::new();
        let qr = qr_table(p);
        let mut epts = vec![];
        for x in 0..p {
            let r = (x * x % p * x + c.a * x + c.b) % p;
            if qr[r as usize] >= 0 {
                let y = sqrt_mod(r, p).unwrap();
                epts.push((x, y));
                if y != 0 {
                    epts.push((x, p - y));
                }
            }
        }
        for &pt in &epts {
            img.insert(delta(pt));
        }
        let mut rng = SplitMix::from_label(&format!("R1-2desc-hom-{bits}"));
        let mut hom_fail = 0;
        for _ in 0..10000 {
            let a = epts[rng.below(epts.len() as u64) as usize];
            let b = epts[rng.below(epts.len() as u64) as usize];
            if let Some(s) = padd(c, Some(a), Some(b)) {
                let (da, db, ds) = (delta(a), delta(b), delta(s));
                if (da.0 * db.0, da.1 * db.1) != ds {
                    hom_fail += 1;
                }
            }
        }
        rows.push(format!(
            "{{\"claim\":\"R1-2desc\",\"p\":{p},\"A\":{},\"B\":{},\"order\":{},\"n\":{},\"nonconstant_on_G\":{nonconst_g},\"image_size_on_E\":{},\"hom_failures_10000\":{hom_fail}}}",
            c.a, c.b, c.order, c.n, img.len()
        ));
        eprintln!("R1-2desc p={p} nonconst {nonconst_g} img {} homfail {hom_fail}", img.len());
    }
    let _ = write!(out, "[\n{}\n]\n", rows.join(",\n"));
}

// ============================ R2 ============================
struct SpecStat {
    m_max: f64,
    argmax: usize,
    sigma2: f64,
    p_upper: f64,
    p_lower: f64,
}
fn spectral(bl: &Bluestein, f: &[i8]) -> SpecStat {
    let n = f.len();
    let x: Vec<f64> = f.iter().map(|&v| v as f64).collect();
    let mu = x.iter().sum::<f64>() / n as f64;
    let xf = bl.dft_real(&x);
    let (mut m, mut am) = (0.0f64, 0usize);
    for a in 1..=(n - 1) / 2 {
        let v = xf[a].abs();
        if v > m {
            m = v;
            am = a;
        }
    }
    let sigma2 = n as f64 * (1.0 - mu * mu);
    let t = (-(m * m) / sigma2).exp();
    let nn = ((n - 1) / 2) as f64;
    SpecStat { m_max: m, argmax: am, sigma2, p_upper: max_upper(t, nn), p_lower: max_lower(t, nn) }
}
fn residue_chi2(f: &[i8], md: u64) -> (f64, f64, f64) {
    // contingency 2 x md over k = 1..n-1 ; returns (chi2, p, max |class mean - overall|)
    let mut cnt = vec![[0f64; 2]; md as usize];
    for k in 1..f.len() {
        let r = (k as u64 % md) as usize;
        cnt[r][(f[k] < 0) as usize] += 1.0;
    }
    let tot: f64 = cnt.iter().map(|c| c[0] + c[1]).sum();
    let col = [cnt.iter().map(|c| c[0]).sum::<f64>(), cnt.iter().map(|c| c[1]).sum::<f64>()];
    let mut chi = 0.0;
    let mut maxdev: f64 = 0.0;
    let overall = (col[0] - col[1]) / tot;
    for c in &cnt {
        let rt = c[0] + c[1];
        for j in 0..2 {
            let e = rt * col[j] / tot;
            if e > 0.0 {
                chi += (c[j] - e).powi(2) / e;
            }
        }
        maxdev = maxdev.max(((c[0] - c[1]) / rt - overall).abs());
    }
    (chi, chi2_sf(chi, (md - 1) as f64), maxdev)
}
fn r2(insts: &[Inst], out: &mut String, aa_out: &mut String) {
    let mut rows = vec![];
    let mut aa_rows = vec![];
    let n_tests: usize = insts.iter().map(|i| i.preds.iter().filter(|p| !p.0.contains("R1:")).count() * (2 + RES_MODS.len())).sum();
    let thr = ALPHA / n_tests as f64;
    eprintln!("R2 tests {n_tests} threshold {thr:e}");
    for inst in insts {
        let n = inst.n as usize;
        let bl = Bluestein::new(n);
        let sqrt_q = (2f64).powf(inst.q_bits / 2.0);
        let rn = (n as f64 * (n as f64).ln()).sqrt();
        let mut emit = |name: &str, f: &[i8], role: &str, rows: &mut Vec<String>| {
            let s = spectral(&bl, f);
            let mut r = format!(
                "{{\"lead\":\"R2\",\"instance\":\"{}\",\"field\":\"{}\",\"n\":{n},\"log2q\":{:.4},\"predicate\":\"{name}\",\"role\":\"{role}\",\"M\":{},\"argmax_a\":{},\"M_over_sqrt_q\":{},\"M_over_sqrt_nlnn\":{},\"sigma2\":{},\"p_upper\":{},\"p_lower\":{}",
                inst.label, inst.field, inst.q_bits, jnum(s.m_max), s.argmax, jnum(s.m_max / sqrt_q), jnum(s.m_max / rn), jnum(s.sigma2), jnum(s.p_upper), jnum(s.p_lower)
            );
            let mut minp_res: f64 = 1.0;
            r.push_str(",\"residues\":[");
            let mut parts = vec![];
            for &md in RES_MODS.iter() {
                let (chi, p, dev) = residue_chi2(f, md);
                minp_res = minp_res.min(p);
                parts.push(format!("{{\"m\":{md},\"chi2\":{},\"p\":{},\"max_class_dev\":{}}}", jnum(chi), jnum(p), jnum(dev)));
            }
            r.push_str(&parts.join(","));
            let _ = write!(
                r,
                "],\"verdict_correlated_spectral\":{},\"verdict_more_uniform\":{},\"verdict_correlated_residue\":{}}}",
                s.p_upper < thr,
                s.p_lower < thr,
                minp_res < thr
            );
            rows.push(r);
            (s, minp_res)
        };
        for (name, f) in &inst.preds {
            let role = if name.contains("R1:") { "excluded_constant" } else { "test" };
            let (s, mp) = emit(name, f, role, &mut rows);
            eprintln!("R2 {} {name}: M/sqrt(q)={:.3} M/sqrt(nlnn)={:.3} pU={:.2e} pL={:.2e} minp_res={:.2e}", inst.label, s.m_max / sqrt_q, s.m_max / rn, s.p_upper, s.p_lower, mp);
        }
        // positive controls
        let mut rng = SplitMix::from_label(&format!("R2-pc-{}", inst.label));
        let a0 = 1 + rng.below((n as u64 - 1) / 2) as usize;
        let eps = (8.0 * rn * std::f64::consts::PI / (4.0 * n as f64)).min(0.5);
        let fpc: Vec<i8> = (0..n)
            .map(|k| {
                let base = if (2.0 * std::f64::consts::PI * ((a0 * k) % n) as f64 / n as f64).cos() >= 0.0 { 1i8 } else { -1 };
                if rng.unif() < 0.5 - eps { -base } else { base }
            })
            .collect();
        let (s, _) = emit("planted_spectral", &fpc, "positive_control", &mut rows);
        eprintln!("R2 PC spectral {}: a0={a0} found={} pU={:.2e}", inst.label, s.argmax, s.p_upper);
        let eps3 = (900.0 / (32.0 * n as f64)).sqrt().min(0.5);
        let fpc3: Vec<i8> = (0..n)
            .map(|k| {
                let base = if k % 3 == 0 { 1i8 } else { -1 };
                if rng.unif() < 0.5 - eps3 { -base } else { base }
            })
            .collect();
        let (_, mp) = emit("planted_k_mod_3", &fpc3, "positive_control", &mut rows);
        eprintln!("R2 PC mod3 {}: minp={mp:.2e}", inst.label);
        // A/A: random +-1 functions through the same code
        let mut pu = vec![];
        let mut pl = vec![];
        let mut p3 = vec![];
        let mut rng = SplitMix::from_label(&format!("R2-aa-{}", inst.label));
        for _ in 0..K_AA {
            let fr: Vec<i8> = (0..n).map(|_| if rng.next() & 1 == 0 { 1 } else { -1 }).collect();
            let s = spectral(&bl, &fr);
            pu.push(s.p_upper);
            pl.push(s.p_lower);
            p3.push(residue_chi2(&fr, 3).1);
        }
        let (ku, kl, k3) = (ks_uniform(pu.clone()), ks_uniform(pl.clone()), ks_uniform(p3.clone()));
        aa_rows.push(format!(
            "{{\"lead\":\"R2\",\"instance\":\"{}\",\"K\":{K_AA},\"ks_p_upper\":{},\"ks_p_lower\":{},\"ks_chi2_mod3\":{},\"null_valid\":{}}}",
            inst.label, jnum(ku), jnum(kl), jnum(k3), ku <= 0.25 && kl <= 0.25 && k3 <= 0.25
        ));
        eprintln!("R2 AA {}: KS {ku:.3} {kl:.3} {k3:.3}", inst.label);
    }
    let _ = write!(out, "{{\"n_tests\":{n_tests},\"threshold\":{},\"rows\":[\n{}\n]}}\n", jnum(thr), rows.join(",\n"));
    aa_out.push_str(&aa_rows.join(",\n"));
}

// ============================ R3 ============================
/// decomposition set D (as indicator over k) from factor-base index set F: D = {+-i +-j}
fn decomp_set(n: usize, fb: &[usize]) -> Vec<bool> {
    let mut d = vec![false; n];
    for (ii, &i) in fb.iter().enumerate() {
        for &j in &fb[ii..] {
            d[(i + j) % n] = true;
            d[(i + n - j) % n] = true;
            d[(2 * n - i - j) % n] = true;
            d[(j + n - i) % n] = true;
        }
    }
    d[0] = false;
    d
}
fn two_prop(d: &[bool], f: &[i8]) -> (f64, f64, f64, f64, f64) {
    // over k=1..n-1 ; returns (rate+, rate-, lift(max side), z, p)
    let (mut a, mut na, mut b, mut nb) = (0f64, 0f64, 0f64, 0f64);
    for k in 1..d.len() {
        if f[k] > 0 {
            na += 1.0;
            a += d[k] as u8 as f64;
        } else {
            nb += 1.0;
            b += d[k] as u8 as f64;
        }
    }
    let (ra, rb) = (a / na, b / nb);
    let pp = (a + b) / (na + nb);
    let se = (pp * (1.0 - pp) * (1.0 / na + 1.0 / nb)).sqrt();
    let z = (ra - rb) / se;
    let lift = if ra >= rb { ra / rb } else { rb / ra };
    (ra, rb, lift, z, norm_two_sided(z))
}
fn r3(insts: &[Inst], out: &mut String, aa_out: &mut String) {
    let mut rows = vec![];
    let mut aa_rows = vec![];
    let n_prime_tests: usize = insts.iter().filter(|i| i.field == "prime").map(|i| i.preds.len()).sum();
    let n_bin_tests: usize = insts.iter().filter(|i| i.field == "binary").map(|i| i.preds.len() - 1 + 1).sum();
    let n_tests = n_prime_tests + n_bin_tests;
    let thr = ALPHA / n_tests as f64;
    eprintln!("R3 tests {n_tests} threshold {thr:e}");
    for inst in insts {
        let n = inst.n as usize;
        // factor base (Amendment 1): prime -> x < B with |F| >= ceil(0.76 sqrt n) points;
        // binary -> x in span{1..t^(l-1)}, l = floor(m/2)
        let fb: Vec<usize>;
        let fb_desc: String;
        if inst.field == "prime" {
            let target = (0.76 * (n as f64).sqrt()).ceil() as usize;
            let mut xv: Vec<u32> = inst.xs[1..].to_vec();
            xv.sort();
            let bnd = xv[target - 1] + 1;
            fb = (1..n).filter(|&k| inst.xs[k] < bnd).collect();
            fb_desc = format!("x<{bnd}");
        } else {
            let l = (inst.q_bits as u32) / 2;
            fb = (1..n).filter(|&k| inst.xs[k] < (1u32 << l)).collect();
            fb_desc = format!("x in span(1..t^{}), l={l}", l - 1);
        }
        let d = decomp_set(n, &fb);
        let rho = d.iter().filter(|&&v| v).count() as f64 / (n - 1) as f64;
        // predicates (both fields)
        for (name, f) in &inst.preds {
            if name.contains("R1:") {
                continue;
            }
            let (ra, rb, lift, z, p) = two_prop(&d, f);
            rows.push(format!(
                "{{\"lead\":\"R3\",\"instance\":\"{}\",\"field\":\"{}\",\"n\":{n},\"fb\":\"{fb_desc}\",\"fb_points\":{},\"density\":{},\"predicate\":\"{name}\",\"role\":\"test\",\"rate_plus\":{},\"rate_minus\":{},\"lift\":{},\"z\":{},\"p\":{},\"useful_prefilter\":{}}}",
                inst.label, inst.field, fb.len(), jnum(rho), jnum(ra), jnum(rb), jnum(lift), jnum(z), jnum(p), p < thr && lift >= 1.10
            ));
            eprintln!("R3 {} {name}: dens {rho:.3} lift {lift:.4} z {z:.2} p {p:.2e}", inst.label);
        }
        // positive control: 1[R in D] flipped with prob 0.4
        let mut rng = SplitMix::from_label(&format!("R3-pc-{}", inst.label));
        let fpc: Vec<i8> = (0..n).map(|k| { let b = if d[k] { 1i8 } else { -1 }; if rng.unif() < 0.4 { -b } else { b } }).collect();
        let (_, _, lift, z, p) = two_prop(&d, &fpc);
        rows.push(format!("{{\"lead\":\"R3\",\"instance\":\"{}\",\"predicate\":\"planted_flip0.4\",\"role\":\"positive_control\",\"lift\":{},\"z\":{},\"p\":{}}}", inst.label, jnum(lift), jnum(z), jnum(p)));
        // A/A random predicates
        let mut ps = vec![];
        let mut rng = SplitMix::from_label(&format!("R3-aa-{}", inst.label));
        for _ in 0..K_AA {
            let fr: Vec<i8> = (0..n).map(|_| if rng.next() & 1 == 0 { 1 } else { -1 }).collect();
            ps.push(two_prop(&d, &fr).4);
        }
        let ks = ks_uniform(ps);
        // binary: WHT over all linear functionals of x(R)
        if inst.field == "binary" {
            let m = inst.q_bits as u32;
            let size = 1usize << m;
            let wht_stat = |ind: &dyn Fn(usize) -> bool| -> (f64, usize, f64, usize) {
                // x-classes of <G>\{O}
                let mut seen = vec![false; size];
                let mut cls = vec![];
                for k in 1..n {
                    let x = inst.xs[k] as usize;
                    if !seen[x] {
                        seen[x] = true;
                        cls.push((x, ind(k)));
                    }
                }
                let nx = cls.len() as f64;
                let dens = cls.iter().filter(|c| c.1).count() as f64 / nx;
                // scale by integer: g = 2*(I*N - count)  to keep integers: use g*N
                let cnt = cls.iter().filter(|c| c.1).count() as i64;
                let nxi = cls.len() as i64;
                let mut a = vec![0i64; size];
                for &(x, i) in &cls {
                    a[x] = (i as i64) * nxi - cnt;
                }
                wht(&mut a);
                let var = nx * dens * (1.0 - dens) * (nxi as f64).powi(2);
                let (mut best, mut bc) = (0f64, 0usize);
                for c in 1..size {
                    let z = (a[c] as f64).abs() / var.sqrt();
                    if z > best {
                        best = z;
                        bc = c;
                    }
                }
                // count degenerate functionals (constant on support): |W| == 0 identically & excluded
                let neff = (size / 2 - 1) as f64;
                let tail = norm_two_sided(best);
                (best, bc, max_upper(tail, neff), cls.len())
            };
            let (z, c, p, nx) = wht_stat(&|k| d[k]);
            rows.push(format!(
                "{{\"lead\":\"R3\",\"instance\":\"{}\",\"field\":\"binary\",\"predicate\":\"WHT_max_all_linear_functionals\",\"role\":\"test\",\"x_classes\":{nx},\"density\":{},\"z_max\":{},\"argmax_c\":{c},\"p\":{},\"useful_prefilter\":{}}}",
                inst.label, jnum(rho), jnum(z), jnum(p), p < thr
            ));
            eprintln!("R3 WHT {}: zmax {z:.2} at c={c} p {p:.2e}", inst.label);
            // positive control: planted codim-2 subspace
            let mut rng = SplitMix::from_label(&format!("R3-pc-wht-{}", inst.label));
            let (w1, w2) = loop {
                let w1 = 1 + rng.below(size as u64 - 1) as usize;
                let w2 = 1 + rng.below(size as u64 - 1) as usize;
                if w1 != w2 {
                    break (w1, w2);
                }
            };
            let xs = &inst.xs;
            let pc = |k: usize| ((xs[k] as usize & w1).count_ones() % 2 == 0) && ((xs[k] as usize & w2).count_ones() % 2 == 0);
            let (zp, cp, pp, _) = wht_stat(&pc);
            rows.push(format!(
                "{{\"lead\":\"R3\",\"instance\":\"{}\",\"predicate\":\"planted_codim2_subspace\",\"role\":\"positive_control\",\"w1\":{w1},\"w2\":{w2},\"z_max\":{},\"argmax_c\":{cp},\"p\":{}}}",
                inst.label, jnum(zp), jnum(pp)
            ));
            eprintln!("R3 WHT PC {}: z {zp:.1} c {cp} (w1 {w1} w2 {w2})", inst.label);
            // A/A random sets of the same density on x-classes
            let mut ps = vec![];
            for _ in 0..K_AA {
                let mut r2 = SplitMix(rng.next());
                let lab: Vec<bool> = (0..size).map(|_| r2.unif() < rho).collect();
                ps.push(wht_stat(&|k| lab[xs[k] as usize]).2);
            }
            let ksw = ks_uniform(ps);
            aa_rows.push(format!("{{\"lead\":\"R3-WHT\",\"instance\":\"{}\",\"K\":{K_AA},\"ks\":{},\"null_valid\":{}}}", inst.label, jnum(ksw), ksw <= 0.25));
        }
        aa_rows.push(format!("{{\"lead\":\"R3\",\"instance\":\"{}\",\"K\":{K_AA},\"ks\":{},\"null_valid\":{}}}", inst.label, jnum(ks), ks <= 0.25));
    }
    let _ = write!(out, "{{\"n_tests\":{n_tests},\"threshold\":{},\"rows\":[\n{}\n]}}\n", jnum(thr), rows.join(",\n"));
    aa_out.push_str(&aa_rows.join(",\n"));
}

// ============================ R4 ============================
fn s_min(n: u64, d: f64) -> u64 {
    // smallest s with (d^(s/2)+1)^2 >= n
    let mut s = 1u64;
    while (d.powf(s as f64 / 2.0) + 1.0).powi(2) < n as f64 {
        s += 1;
    }
    s
}
fn r4(out: &mut String) {
    let mut rows = vec![];
    let costs = [0.0, 0.01, 0.1, 0.5, 1.0];
    let net = |s: u64| -> String { costs.iter().map(|c| jnum((s as f64).sqrt() / (1.0 + s as f64 * c))).collect::<Vec<_>>().join(",") };
    // D = -7, alpha = (1+sqrt(-7))/2, norm 2; twists of y^2 = x^3 - 35x + 98
    let mut found = 0;
    let mut p = 1u64 << 16;
    while found < 20 {
        p += 1;
        if !is_prime(p) || !matches!(p % 7, 1 | 2 | 4) {
            continue;
        }
        let qr = qr_table(p);
        let nonres = (2..p).find(|&z| qr[z as usize] < 0).unwrap();
        for c in [1u64, nonres] {
            let a = (p - 35 * c % p * c % p) % p;
            let b = 98 * c % p * c % p * c % p;
            let ord = count_prime(p, a, b, &qr);
            if !is_prime(ord) || found >= 20 {
                continue;
            }
            let n = ord;
            let sq = sqrt_mod(n - 7 % n, n).expect("-7 must be a square mod n");
            let inv2 = (n + 1) / 2;
            let l1 = (1 + sq) % n * inv2 % n;
            let l2 = (1 + n - sq) % n * inv2 % n;
            assert_eq!((l1 * l1 % n + 2 + n - l1) % n, 0);
            let s1 = mult_order(l1, n);
            let s2 = mult_order(l2, n);
            let s = s1.min(s2);
            let o2 = mult_order(2, n);
            let sm = s_min(n, 2.0);
            rows.push(format!(
                "{{\"claim\":\"R4-exact\",\"family\":\"D=-7 norm-2\",\"p\":{p},\"twist\":{},\"n\":{n},\"ord_lambda\":[{s1},{s2}],\"s\":{s},\"s_min_bound\":{sm},\"bound_holds\":{},\"ord_2_mod_n\":{o2},\"bound_holds_scalar2\":{},\"net_factor_at_c\":[{}]}}",
                c != 1, s >= sm, o2 >= s_min(n, 4.0), net(s)
            ));
            found += 1;
        }
    }
    // controls j = 0 (order-6 automorphism) and j = 1728 (order 4): verify zeta(G) = lambda G exactly
    let mut cnt0 = 0;
    let mut cnt1 = 0;
    let mut p = 1u64 << 16;
    while cnt0 < 20 || cnt1 < 20 {
        p += 1;
        if !is_prime(p) || p % 12 != 1 && p % 3 != 1 && p % 4 != 1 {
            continue;
        }
        let qr = qr_table(p);
        for (j, ok) in [(0u32, p % 3 == 1 && cnt0 < 20), (1728, p % 4 == 1 && cnt1 < 20)] {
            if !ok {
                continue;
            }
            for coef in 1..p {
                let (a, b) = if j == 0 { (0, coef) } else { (coef, 0) };
                let ord = count_prime(p, a, b, &qr);
                if !is_prime(ord) {
                    continue;
                }
                let n = ord;
                let g = first_point(p, a, b, 1);
                let c = PCurve { p, a, b, order: ord, n, h: 1, g };
                // automorphism and eigenvalue
                let (img, lams): ((u64, u64), Vec<u64>) = if j == 0 {
                    let beta = (2..p).map(|z| powmod(z, (p - 1) / 3, p)).find(|&v| v != 1).unwrap();
                    let w = (2..n).map(|z| powmod(z, (n - 1) / 3, n)).find(|&v| v != 1).unwrap();
                    ((beta * g.0 % p, g.1), vec![w, w * w % n])
                } else {
                    let ip = sqrt_mod(p - 1, p).unwrap();
                    let inn = sqrt_mod(n - 1, n).unwrap();
                    (((p - g.0) % p, ip * g.1 % p), vec![inn, n - inn])
                };
                let lam = lams.iter().copied().find(|&l| pmul(&c, l, Some(g)) == Some(img));
                let s = lam.map(|l| mult_order(l, n)).unwrap_or(0);
                rows.push(format!(
                    "{{\"claim\":\"R4-exact\",\"family\":\"j={j} automorphism (control)\",\"p\":{p},\"n\":{n},\"eigen_verified\":{},\"s\":{s},\"net_factor_at_c\":[{}]}}",
                    lam.is_some(), net(s)
                ));
                if j == 0 { cnt0 += 1 } else { cnt1 += 1 }
                break;
            }
        }
    }
    // Koblitz Frobenius: tau(G) = lambda G, ord(lambda) = m, dual eigenvalue mu - lambda
    for m in [11u32, 13, 17, 19] {
        for c in family_b(m) {
            if !c.label.starts_with("koblitz") {
                continue;
            }
            let n = c.n;
            let mu: i64 = if c.a == 1 { 1 } else { -1 };
            // roots of l^2 - mu l + 2 : disc = mu^2 - 8 = -7
            let sq = sqrt_mod(n - 7, n).unwrap();
            let inv2 = (n + 1) / 2;
            let mun = ((mu + n as i64) as u64) % n;
            let roots = [(mun + sq) % n * inv2 % n, (mun + n - sq) % n * inv2 % n];
            let img = (c.k.sq(c.g.0), c.k.sq(c.g.1));
            let lam = roots.iter().copied().find(|&l| bmul(&c, l, Some(c.g)) == Some(img));
            let s = lam.map(|l| mult_order(l, n)).unwrap_or(0);
            let dual = lam.map(|l| (mun + n - l) % n).unwrap_or(0);
            let sd = mult_order(dual, n);
            rows.push(format!(
                "{{\"claim\":\"R4-exact\",\"family\":\"Koblitz Frobenius (norm 2, ~free)\",\"curve\":\"{}\",\"m\":{m},\"n\":{n},\"eigen_verified\":{},\"s\":{s},\"s_min_bound\":{},\"ord_dual\":{sd},\"ord_2_mod_n\":{},\"net_factor_at_c\":[{}]}}",
                c.label, lam.is_some(), s_min(n, 2.0), mult_order(2, n), net(s)
            ));
        }
    }
    let _ = write!(out, "{{\"costs_c\":[0,0.01,0.1,0.5,1],\"rows\":[\n{}\n]}}\n", rows.join(",\n"));
}

fn selftest() {
    // Bluestein vs naive DFT
    for n in [7usize, 101, 1009] {
        let mut rng = SplitMix::from_label("selftest");
        let x: Vec<f64> = (0..n).map(|_| rng.unif() - 0.5).collect();
        let bl = Bluestein::new(n);
        let xf = bl.dft_real(&x);
        let mut err: f64 = 0.0;
        for a in 0..n {
            let (mut re, mut im) = (0.0, 0.0);
            for k in 0..n {
                let ang = -2.0 * std::f64::consts::PI * ((a * k) % n) as f64 / n as f64;
                re += x[k] * ang.cos();
                im += x[k] * ang.sin();
            }
            err = err.max((xf[a].re - re).abs()).max((xf[a].im - im).abs());
        }
        println!("bluestein n={n} max err {err:.2e}");
        assert!(err < 1e-8);
    }
    println!("chi2_sf(3.841,1)={:.5} chi2_sf(100,2)={:.3e} chi2_sf(1,15)={:.6}", chi2_sf(3.841, 1.0), chi2_sf(100.0, 2.0), chi2_sf(1.0, 15.0));
    println!("norm2(1.96)={:.5} norm2(8)={:.3e}", norm_two_sided(1.96), norm_two_sided(8.0));
    // WHT: delta function
    let mut a = vec![0i64; 8];
    a[3] = 1;
    wht(&mut a);
    assert!(a.iter().all(|v| v.abs() == 1));
    // binary group law: associativity and order on a Koblitz curve
    let c = &family_b(13)[0];
    let mut rng = SplitMix::from_label("selftest-bin");
    for _ in 0..200 {
        let p = bmul(c, rng.below(c.n), Some(c.g));
        let q = bmul(c, rng.below(c.n), Some(c.g));
        let r = bmul(c, rng.below(c.n), Some(c.g));
        assert_eq!(badd(c, badd(c, p, q), r), badd(c, p, badd(c, q, r)));
    }
    println!("binary {} order {} n {} f={:#x} assoc ok", c.label, c.order, c.n, c.k.f);
    let pc = &family_p(16, 1)[0];
    println!("prime p={} A={} B={} n={} G={:?}", pc.p, pc.a, pc.b, pc.n, pc.g);
    assert!(pmul(pc, pc.n, Some(pc.g)).is_none());
    println!("selftest ok");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("selftest") {
        return selftest();
    }
    let outdir = args.get(2).cloned().unwrap_or_else(|| "../results".into());
    std::fs::create_dir_all(&outdir).unwrap();
    let which = args.get(1).map(|s| s.as_str()).unwrap_or("all");
    if which == "r1" || which == "all" {
        let mut s = String::new();
        r1(&mut s);
        std::fs::write(format!("{outdir}/r1.json"), s).unwrap();
    }
    if which == "r4" || which == "all" {
        let mut s = String::new();
        r4(&mut s);
        std::fs::write(format!("{outdir}/r4.json"), s).unwrap();
    }
    if which == "r2" || which == "r3" || which == "all" || which == "r23" {
        let insts = build_instances();
        let mut aa = String::new();
        if which != "r3" {
            let mut s = String::new();
            r2(&insts, &mut s, &mut aa);
            std::fs::write(format!("{outdir}/r2.json"), s).unwrap();
            aa.push_str(",\n");
        }
        if which != "r2" {
            let mut s = String::new();
            r3(&insts, &mut s, &mut aa);
            std::fs::write(format!("{outdir}/r3.json"), s).unwrap();
        }
        std::fs::write(format!("{outdir}/aa.json"), format!("[\n{}\n]\n", aa.trim_end_matches(",\n"))).unwrap();
    }
}
