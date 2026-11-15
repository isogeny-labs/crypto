use std::{fmt::Write, fs};
fn optimum(a: f64, d: f64, alpha: f64, beta: f64, sigma: f64, k: f64, rho: f64, w: f64) -> f64 {
    (alpha * a / (beta * d) * k.powf(sigma) / (w * rho.powf(beta))).powf(1.0 / (alpha + beta))
}
fn cost(
    b: f64,
    a: f64,
    d: f64,
    alpha: f64,
    beta: f64,
    sigma: f64,
    k: f64,
    rho: f64,
    w: f64,
) -> f64 {
    a * k.powf(sigma) * b.powf(-alpha) + d * w * rho.powf(beta) * b.powf(beta)
}
fn close(a: f64, b: f64) {
    assert!((a / b - 1.0).abs() < 1e-9, "{a} != {b}");
}
fn main() {
    let dir = std::env::args().nth(1).expect("output dir");
    fs::create_dir_all(&dir).unwrap();
    let mut checks = 0;
    for alpha in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0] {
        for beta in [2.0, 3.0] {
            for sigma in [0.5, 1.0] {
                for k in [1.0f64, 2.0, 10.0, 100.0] {
                    for w in [1.0f64, 2.0, 10.0] {
                        for gain in [1.0f64, 10.0, 100.0] {
                            for a in [1e-6, 1.0, 1e6] {
                                for d in [1e-6, 1.0, 1e6] {
                                    let rho = gain.powf(-(alpha + beta) / (alpha * beta))
                                        * k.powf(-sigma / alpha)
                                        * w.powf(-1.0 / beta);
                                    let b0 = optimum(a, d, alpha, beta, sigma, 1.0, 1.0, 1.0);
                                    let b = optimum(a, d, alpha, beta, sigma, k, rho, w);
                                    let t0 = cost(b0, a, d, alpha, beta, sigma, 1.0, 1.0, 1.0);
                                    let t = cost(b, a, d, alpha, beta, sigma, k, rho, w);
                                    close(t / t0, 1.0 / gain);
                                    close(
                                        alpha * a * k.powf(sigma) * b.powf(-alpha),
                                        beta * d * w * rho.powf(beta) * b.powf(beta),
                                    );
                                    close(b / b0, (gain * k.powf(sigma)).powf(1.0 / alpha));
                                    close(rho * b / b0, (gain * w).powf(-1.0 / beta));
                                    for delta in [-0.01f64, 0.01] {
                                        assert!(
                                            cost(
                                                b * delta.exp(),
                                                a,
                                                d,
                                                alpha,
                                                beta,
                                                sigma,
                                                k,
                                                rho,
                                                w
                                            ) >= t * (1.0 - 1e-12)
                                        );
                                    }
                                    for factor in [0.5f64, 2.0] {
                                        let rp = rho * factor;
                                        let bp = optimum(a, d, alpha, beta, sigma, k, rp, w);
                                        let ratio =
                                            cost(bp, a, d, alpha, beta, sigma, k, rp, w) / t0;
                                        let predicted = k.powf(sigma * beta / (alpha + beta))
                                            * w.powf(alpha / (alpha + beta))
                                            * rp.powf(alpha * beta / (alpha + beta));
                                        close(ratio, predicted);
                                        assert_eq!(ratio < 1.0 / gain, factor < 1.0);
                                    }
                                    checks += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut csv =
        String::from("model,g,k,alpha,sigma,rho_max,base_growth,pool_over_B0,retained_over_B0\n");
    let mut guards =
        String::from("model,g,k,N_over_B0,base_fits,birthday_fraction,birthday_guard\n");
    let mut svg=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1000' height='355' viewBox='0 0 1000 355'><rect width='1000' height='355' fill='#f5f8fa'/><g font-family='Arial' fill='#233746'><text x='25' y='35' font-size='23' font-weight='bold'>Overharvest density k = 10: requirements for a 10× model gain</text><text x='25' y='65' font-size='17'>Zero-large-prime model · quadratic algebra · no weight penalty · conditional, not measured</text>");
    let (mut rows, mut eligible, mut excluded) = (0, 0, 0);
    for g in [2, 3, 4] {
        for (model, alpha, sigma) in [
            ("zero_lp", g as f64 - 1.0, 1.0),
            ("one_lp_birthday", g as f64 - 1.5, 0.5),
            ("one_lp_saturated", g as f64 - 2.0, 1.0),
        ] {
            if alpha <= 0.0 {
                println!("INVALID model={model} g={g} alpha={alpha} no_interior_optimum=true");
                continue;
            }
            for k in [2.0f64, 10.0, 100.0, 300.0] {
                let rho = 10.0f64.powf(-(alpha + 2.0) / (alpha * 2.0)) * k.powf(-sigma / alpha);
                let growth = (10.0 * k.powf(sigma)).powf(1.0 / alpha);
                let retained = rho * growth;
                close(retained, 10.0f64.powf(-0.5));
                writeln!(csv,"{model},{g},{k},{alpha},{sigma},{rho:.12e},{growth:.12e},{:.12e},{retained:.12e}",k*growth).unwrap();
                rows += 1;
                for ratio in [10.0, 100.0, 1000.0] {
                    let fits = growth <= ratio * (1.0 + 1e-12);
                    let fraction = if model == "one_lp_birthday" {
                        Some((2.0 * k * growth / 10000.0).sqrt())
                    } else {
                        None
                    };
                    let collision = fraction.is_none_or(|f| f <= 0.1);
                    if fits && collision {
                        eligible += 1;
                    } else {
                        excluded += 1;
                    }
                    writeln!(
                        guards,
                        "{model},{g},{k},{ratio},{fits},{},{}",
                        fraction.map_or("NA".into(), |f| format!("{f:.12e}")),
                        if fraction.is_none() {
                            "NA"
                        } else if collision {
                            "true"
                        } else {
                            "false"
                        }
                    )
                    .unwrap();
                }
                if model == "zero_lp" && k == 10.0 {
                    let y = 115 + (g - 2) * 75;
                    write!(svg,"<text x='25' y='{y}' font-size='22'>Genus {g}</text><text x='180' y='{y}' font-size='22' fill='#146b7e'>retain ≤ {:.3}%</text><text x='490' y='{y}' font-size='22'>base ×{growth:.2}</text><text x='725' y='{y}' font-size='22'>pool ×{:.2}</text>",100.0*rho,k*growth).unwrap();
                }
            }
        }
    }
    svg.push_str("<text x='25' y='310' font-size='17'>Pool is relative to the original optimum B₀. All three retain a core of 0.3162 B₀ at equality.</text><text x='25' y='338' font-size='17'>Finite catalogue, collision regime, target rank and omitted costs must be checked separately.</text></g></svg>");
    fs::write(format!("{dir}/THRESHOLDS.csv"), csv).unwrap();
    fs::write(format!("{dir}/DOMAIN-GUARDS.csv"), guards).unwrap();
    fs::write(format!("{dir}/THRESHOLDS.svg"), svg).unwrap();
    println!("CHECKS parameter_cases={checks} relative_tolerance=1e-9 objective_balance_minimum_threshold_direction=true");
    println!("TABLE rows={rows} illustrative_domain_cases={} algebraically_eligible={eligible} excluded={excluded}",eligible+excluded);
    println!(
        "UNSPECIFIED double_lp=true observed_rho=false observed_weight=false measured_speed=false"
    );
}
