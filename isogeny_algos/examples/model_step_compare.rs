//! Matched odd-degree step on one curve and kernel, with the output needed by a path search.
//! Run: cargo run --release --example model_step_compare
use isogeny_algos::curve::{jinv, on_curve, pmul, Curve, Isogeny, Pt};
use isogeny_algos::field::{next_prime, Field, Rng, Zp};
use isogeny_algos::kernel::models::{edwards_isogeny, mont_to_edwards_point, Edwards};
use isogeny_algos::kernel::{montgomery, velu::velu_cyclic};
use std::hint::black_box;
use std::time::Instant;

const P: u64 = 1_073_746_799; // prime, p = -1 (mod 840), p = 7 (mod 8)
const N: u64 = P + 1; // #E(F_p) for E: y^2 = x^3 + x, p = 3 (mod 4)

fn exact_order_point(f: &Zp, e: &Curve<u64>, ell: u64, rng: &mut Rng) -> Pt<u64> {
    let mut power = ell;
    while N % (power * ell) == 0 {
        power *= ell;
    }
    loop {
        let p = e.random_point(f, rng);
        let mut q = pmul(f, e, &p, (N / power) as u128);
        if q == Pt::Inf {
            continue;
        }
        while pmul(f, e, &q, ell as u128) != Pt::Inf {
            q = pmul(f, e, &q, ell as u128);
        }
        if q != Pt::Inf {
            return q;
        }
    }
}

fn as_edwards(f: &Zp, p: Pt<u64>, sqrt_two: u64) -> (u64, u64) {
    let Pt::Aff(x, y) = p else {
        panic!("identity has no affine map")
    };
    assert_ne!(y, 0);
    assert_ne!(f.add(x, 1), 0);
    let (ex, ey) = mont_to_edwards_point(f, x, y);
    (f.mul(ex, sqrt_two), ey)
}

fn median_ns(mut run: impl FnMut(), samples: usize, batch: usize) -> f64 {
    let mut times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let start = Instant::now();
        for _ in 0..batch {
            run();
        }
        times.push(start.elapsed().as_nanos() as f64 / batch as f64);
    }
    times.sort_by(f64::total_cmp);
    times[samples / 2]
}

fn main() {
    assert_eq!(next_prime(P), P);
    let f = Zp::new(P);
    let e = montgomery::to_weierstrass(&f, 0);
    let sqrt_two = f.sqrt(2).expect("2 is square for this prime");
    let ed = Edwards {
        a: 1,
        d: f.div(f.sub(0, 2), 2),
    };
    assert_eq!(N, 840 * 1_278_270);
    assert!(on_curve(&f, &e, &Pt::Aff(0, 0)));
    let mut rng = Rng::new(0x4d4f44454c);
    println!("p,ell,arm,task,median_ns,verified");
    for ell in [3, 5, 7, 11, 13] {
        if N % ell != 0 {
            continue;
        }
        let k = exact_order_point(&f, &e, ell, &mut rng);
        let ke = as_edwards(&f, k, sqrt_two);
        assert!(ed.on_curve(&f, ke));
        let q = loop {
            let q = e.random_point(&f, &mut rng);
            if let Pt::Aff(x, y) = q {
                if y != 0 && f.add(x, 1) != 0 && pmul(&f, &e, &q, ell as u128) != Pt::Inf {
                    break q;
                }
            }
        };
        let qe = as_edwards(&f, q, sqrt_two);
        let wi = velu_cyclic(&f, &e, &k, ell);
        let ei = edwards_isogeny(&f, &ed, ke, ell);
        let Pt::Aff(kx, _) = k else { unreachable!() };
        let Pt::Aff(qx, _) = q else { unreachable!() };
        let mont_a24 = montgomery::a24(&f, 0);
        let mults = montgomery::multiples(&f, mont_a24, (kx, 1), ((ell - 1) / 2) as usize);
        let mont_cod = montgomery::velu_codomain(&f, 0, &mults, ell);
        let wj = jinv(&f, &wi.cod);
        let ej = ei.cod.j(&f);
        let we = wi.eval(&f, &q);
        let ee = ei.eval_explicit(&f, qe);
        assert_eq!(wj, ej, "codomain mismatch at ell={ell}");
        assert_eq!(wj, montgomery::j_invariant(&f, mont_cod));
        assert!(on_curve(&f, &wi.cod, &we));
        assert!(ei.cod.on_curve(&f, ee));
        assert_eq!(ei.eval_x_only(&f, qe.0), ee.0);
        let mx = montgomery::isog_x(&f, &mults, qx);
        let mrhs = f.mul(mx, f.add(f.mul(mx, mx), f.add(f.mul(mont_cod, mx), 1)));
        assert!(f.sqrt(mrhs).is_some(), "Montgomery image is off curve");
        assert_eq!(pmul(&f, &e, &k, ell as u128), Pt::Inf);
        // The path-search successor needs only the codomain j. Full step also evaluates Q.
        for task in ["successor_j", "step_and_eval_x"] {
            let w = median_ns(
                || {
                    let iso = velu_cyclic(&f, &e, black_box(&k), ell);
                    let j = jinv(&f, &iso.cod);
                    black_box(j);
                    if task == "step_and_eval_x" {
                        black_box(iso.eval_x(&f, black_box(qx)));
                    }
                },
                21,
                1_000,
            );
            let ed_ready = median_ns(
                || {
                    let iso = edwards_isogeny(&f, &ed, black_box(ke), ell);
                    let j = iso.cod.j(&f);
                    black_box(j);
                    if task == "step_and_eval_x" {
                        black_box(iso.eval_x_only(&f, black_box(qe.0)));
                    }
                },
                21,
                1_000,
            );
            let mont = median_ns(
                || {
                    let mults = montgomery::multiples(
                        &f,
                        mont_a24,
                        (black_box(kx), 1),
                        ((ell - 1) / 2) as usize,
                    );
                    let cod = montgomery::velu_codomain(&f, 0, &mults, ell);
                    black_box(montgomery::j_invariant(&f, cod));
                    if task == "step_and_eval_x" {
                        black_box(montgomery::isog_x(&f, &mults, black_box(qx)));
                    }
                },
                21,
                1_000,
            );
            let ed_from_standard = median_ns(
                || {
                    let ke_now = as_edwards(&f, black_box(k), sqrt_two);
                    let iso = edwards_isogeny(&f, &ed, ke_now, ell);
                    let j = iso.cod.j(&f);
                    black_box(j);
                    if task == "step_and_eval_x" {
                        let qe_now = as_edwards(&f, black_box(q), sqrt_two);
                        black_box(iso.eval_x_only(&f, qe_now.0));
                    }
                },
                21,
                1_000,
            );
            for (arm, ns) in [
                ("weierstrass_velu", w),
                ("montgomery_x_only", mont),
                ("edwards_preconverted", ed_ready),
                ("edwards_with_conversion", ed_from_standard),
            ] {
                println!("{P},{ell},{arm},{task},{ns:.1},true");
            }
        }
        // One successor beginning with a curve and a deterministic point source. The common
        // point search and cofactor multiplication are charged to every representation.
        for arm in [
            "weierstrass_velu",
            "montgomery_x_only",
            "edwards_with_conversion",
        ] {
            let mut source = Rng::new(0x4d4f44454c + ell);
            let ns = median_ns(
                || {
                    let kernel = exact_order_point(&f, &e, ell, &mut source);
                    let j = match arm {
                        "weierstrass_velu" => jinv(&f, &velu_cyclic(&f, &e, &kernel, ell).cod),
                        "montgomery_x_only" => {
                            let Pt::Aff(x, _) = kernel else {
                                unreachable!()
                            };
                            let ms = montgomery::multiples(
                                &f,
                                mont_a24,
                                (x, 1),
                                ((ell - 1) / 2) as usize,
                            );
                            montgomery::j_invariant(&f, montgomery::velu_codomain(&f, 0, &ms, ell))
                        }
                        _ => {
                            let ek = as_edwards(&f, kernel, sqrt_two);
                            edwards_isogeny(&f, &ed, ek, ell).cod.j(&f)
                        }
                    };
                    black_box(j);
                },
                21,
                100,
            );
            println!("{P},{ell},{arm},full_successor_from_curve,{ns:.1},true");
        }
    }
}
