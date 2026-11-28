//! Structure facts for the two lab M31 curves and the j = 0 / j = 1728 classes over GF(2^31 - 1).
//! Prints one JSON document. Two methods per fact where a second exists; `agree` flags.

use m31_structure_rho::structure::*;
use m31_structure_rho::{kdf, powmod, P};

fn fm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn analyse(name: &str, a: u64, b: u64, n: u64, g: Option<(u64, u64)>, q: Option<(u64, u64)>) -> String {
    let mut rng = Rng::new(&format!("structure/{}", name));
    let disc = (fm(4, fm(a, fm(a, a))) + fm(27, fm(b, b))) % P;
    let j_a = fm(fm(1728, fm(4, fm(a, fm(a, a)))), powmod(disc, P - 2, P));
    let c4 = fm(P - 48 % P, a);
    let delta = fm(P - 16, disc);
    let j_b = fm(fm(c4, fm(c4, c4)), powmod(delta, P - 2, P));
    let g = g.unwrap_or_else(|| rand_point(a, b, &mut rng));
    let s = (P as f64).sqrt() as u64;
    let hasse = (P + 1).abs_diff(n) <= 2 * s + 1;
    let a_ok = is_prime(n) && hasse && mul(n, Some(g), a).is_none() && mul_jacobian(n, Some(g), a).is_none();
    let mut hits: Option<Vec<u64>> = None;
    for _ in 0..3 {
        let h = bsgs_hasse(rand_point(a, b, &mut rng), a);
        hits = Some(match hits {
            None => h,
            Some(o) => o.into_iter().filter(|x| h.contains(x)).collect(),
        });
    }
    let hits = hits.unwrap();
    let t = P as i64 + 1 - n as i64;
    let d = t * t - 4 * P as i64; // negative
    let fd = factor(d.unsigned_abs());
    let prod: u64 = fd.iter().map(|(q, e)| q.pow(*e)).product();
    let (mut sq, mut core) = (1u64, 1u64);
    for &(q, e) in &fd {
        sq *= q.pow(e / 2);
        core *= q.pow(e % 2);
    }
    // d_K = -core if -core = 1 mod 4 else -4 core
    let (dk, fpi) = if (core % 4) == 3 { (-(core as i64), sq) } else { (-(4 * core as i64), sq / 2) };
    assert_eq!((fpi * fpi) as i64 * dk, d);
    // 3-volcano level (only prime dividing f_pi here)
    assert!(factor(fpi).iter().all(|(q, _)| *q == 3), "conductor has a prime other than 3");
    let psi3 = vec![(P - fm(a, a)) % P, fm(12, b), fm(6, a), 0, 3];
    let roots_a = n_roots(&psi3);
    let roots_b = n_roots_fermat(&psi3);
    let (ta, tb) = twist(a, b);
    let tw = 2 * P + 2 - n;
    let t3 = three_torsion_size(ta, tb, tw, &mut rng);
    assert_eq!(t.rem_euclid(3), 1, "scalar on E[3] must be -1 for this test");
    let crater_a = roots_a == 4;
    let crater_b = t3 == 9;
    let level = if crater_a { "crater (End = O_K, conductor 1)" } else { "floor (End = Z[pi], conductor 3)" };
    // rational l-isogenies for small l not dividing D: roots of x^2 - t x + p mod l
    let mut iso = vec![];
    for l in (2u64..=47).filter(|&l| is_prime(l)) {
        if d.rem_euclid(l as i64) == 0 {
            iso.push(format!("\"{}\":\"l | D\"", l));
        } else {
            let c = (0..l as i64).filter(|r| (r * r - t * r + P as i64).rem_euclid(l as i64) == 0).count();
            iso.push(format!("\"{}\":{}", l, c));
        }
    }
    let cubic = vec![b, a, 0, 1];
    let l2_roots = n_roots(&cubic);
    let l2_roots_b = n_roots_fermat(&cubic);
    let k_a = mult_order(P % n, n);
    let k_b_small = (1..=5000u64).find(|&k| powmod(P, k, n) == 1);
    let qline = match q {
        Some(qq) => {
            let on = fm(qq.1, qq.1) == (fm(fm(qq.0, qq.0), qq.0) + fm(a, qq.0) + b) % P;
            format!(
                ",\"Q_on_curve\":{},\"Q_eq_987654321G_affine\":{},\"Q_eq_987654321G_jacobian\":{}",
                on,
                mul(987654321, Some(g), a) == Some(qq),
                mul_jacobian(987654321, Some(g), a) == Some(qq)
            )
        }
        None => String::new(),
    };
    format!(
        "{{\"curve\":\"{name}\",\"p\":{P},\"a\":{a},\"b\":{b},\"nonsingular\":{},\
\"j\":{{\"A_formula\":{j_a},\"B_c4_cubed_over_Delta\":{j_b},\"agree\":{},\"is_0_or_1728\":{}}},\
\"order\":{{\"n\":{n},\"A_claimed_prime_hasse_kills_G_affine_and_jacobian\":{a_ok},\"B_bsgs_hasse_candidates\":{:?},\"agree\":{}}},\
\"trace\":{t},\"frobenius_disc\":{{\"D\":{d},\"factorization\":{},\"product_check\":{},\"factors_prime\":{},\"d_K\":{dk},\"conductor_Zpi\":{fpi}}},\
\"end_ring_level_3\":{{\"A_roots_psi3_xp_minus_x\":{roots_a},\"A2_roots_psi3_fermat\":{roots_b},\"B_twist_3torsion_size\":{t3},\"level\":\"{level}\",\"agree\":{}}},\
\"aut_group_order\":{},\"min_degree_nonscalar_endomorphism_ge\":{},\
\"rational_l_isogenies_l_not_dividing_D\":{{{}}},\"l2_cubic_roots\":[{l2_roots},{l2_roots_b}],\
\"embedding_degree\":{{\"A_order_p_mod_n\":{k_a},\"A_check\":{},\"B_none_le_5000\":{},\"n_minus_1\":{}}},\
\"anomalous\":{},\"twist\":{{\"order\":{tw},\"factorization\":{}}}{qline}}}",
        disc != 0,
        j_a == j_b,
        j_a == 0 || j_a == 1728,
        hits,
        a_ok && hits == vec![n],
        fac_json(&fd),
        prod as i64 == -d,
        fd.iter().all(|(q, _)| is_prime(*q)),
        crater_a == crater_b && roots_a == roots_b,
        if j_a == 0 { 6 } else if j_a == 1728 { 4 } else { 2 },
        (dk.unsigned_abs() + 1) / 4,
        iso.join(","),
        powmod(P, k_a, n) == 1,
        k_b_small.is_none(),
        fac_json(&factor(n - 1)),
        n == P,
        fac_json(&factor(tw)),
    )
}

fn j0_class() -> String {
    assert_eq!(P % 3, 1);
    let mut rng = Rng::new("structure/j0");
    let mut sol = None;
    for v in 1u64.. {
        if 3 * v * v > 4 * P {
            break;
        }
        let r = 4 * P - 3 * v * v;
        let t = (r as f64).sqrt() as u64;
        for tt in [t.saturating_sub(1), t, t + 1] {
            if tt * tt == r {
                sol = Some((tt as i64, v as i64));
            }
        }
        if sol.is_some() {
            break;
        }
    }
    let (t0, v0) = sol.unwrap();
    let mut traces = vec![t0, -t0, (t0 + 3 * v0) / 2, -(t0 + 3 * v0) / 2, (t0 - 3 * v0) / 2, -(t0 - 3 * v0) / 2];
    traces.sort();
    let orders: Vec<u64> = traces.iter().map(|t| (P as i64 + 1 - t) as u64).collect();
    let mut rows = vec![];
    for &o in &orders {
        // smallest b with #E(y^2 = x^3 + b) = o: identify by killing random points, confirm by BSGS
        let mut bmin = 0;
        for b in 1..400u64 {
            let pt = rand_point(0, b, &mut rng);
            let c: Vec<u64> = orders.iter().cloned().filter(|&oo| mul(oo, Some(pt), 0).is_none()).collect();
            if c == vec![o] {
                let h = bsgs_hasse(rand_point(0, b, &mut rng), 0);
                if h == vec![o] {
                    bmin = b;
                    break;
                }
            }
        }
        let f = factor(o);
        let big = f.last().unwrap().0;
        rows.push(format!(
            "{{\"n\":{o},\"trace\":{},\"factorization\":{},\"prime\":{},\"largest_prime_factor\":{big},\"cofactor\":{},\"b_min_verified_bsgs\":{bmin}}}",
            P as i64 + 1 - o as i64,
            fac_json(&f),
            is_prime(o),
            o / big
        ));
    }
    let h1728 = bsgs_hasse(rand_point(1, 0, &mut rng), 1);
    let any_prime = orders.iter().any(|&o| is_prime(o));
    format!(
        "{{\"four_p_eq_t2_plus_3v2\":[{t0},{v0}],\"check\":{},\"six_twists\":[{}],\"any_prime_order\":{any_prime},\
\"j1728\":{{\"p_mod_4\":{},\"supersingular\":true,\"y2_x3_plus_x_bsgs\":{:?},\"p_plus_1_factorization\":{},\"embedding_degree\":2}}}}",
        t0 * t0 + 3 * v0 * v0 == 4 * P as i64,
        rows.join(","),
        P % 4,
        h1728,
        fac_json(&factor(P + 1))
    )
}

fn main() {
    assert!(kdf::selftest(), "sha256 self-test");
    let b677 = analyse(
        "B677",
        847514846,
        2087444724,
        2147446831,
        Some((625037715, 2100627251)),
        Some((976979610, 1228979761)),
    );
    let c2 = analyse("M31_minus3_21", P - 3, 21, 2147487637, None, None);
    println!(
        "{{\"tool\":\"m31-structure-rho/structure\",\"master\":\"{}\",\"curves\":[{},{}],\"j0_class\":{}}}",
        MASTER,
        b677,
        c2,
        j0_class()
    );
}
