use super::*;
pub fn generate(dir: &str) {
    let dir = Path::new(dir);
    fs::create_dir(dir).unwrap();
    let mut all = vec![];
    let mut rejected = vec![];
    for bits in [16u32, 20, 24, 28, 32, 36, 40] {
        let low = 16 * (1u64 << (bits - 1));
        let mut p = (low as f64).sqrt().ceil() as u64;
        while p * p < low {
            p += 1;
        }
        p += (3 + 4 - p % 4) % 4;
        let limit = 8u64 << (bits / 2);
        let mut fixtures = vec![];
        while fixtures.len() < 3 && p < limit {
            if is_prime(p) {
                if let Some(i) = scholten_instance(p, 20261018 + p) {
                    let rb = 64 - i.r.leading_zeros();
                    if rb == bits && i.h <= 64 {
                        let s = Scholten::new(p);
                        let mut rng = Rng(20261018 + p);
                        for _ in 0..32 {
                            let a = rng.next() % i.r;
                            let b = rng.next() % i.r;
                            let x = i.curve.mul(i.generator, a);
                            let y = i.curve.mul(i.generator, b);
                            let tx = s.tau(s.from_short(x));
                            assert_eq!(s.norm(tx), s.e.mul(s.from_short(x), 2));
                            assert_eq!(
                                s.tau(s.from_short(i.curve.add(x, y))),
                                s.j.add(tx, s.tau(s.from_short(y)))
                            );
                        }
                        eprintln!("order cohort bits={bits} p={p} r={} h={}", i.r, i.h);
                        fixtures.push(i);
                    } else {
                        rejected.push(json!({"requested_bits":bits,"p":p,"r":i.r,"h":i.h,"reason":"outside exact subgroup-bit/cofactor eligibility"}));
                    }
                } else {
                    rejected.push(json!({"requested_bits":bits,"p":p,"reason":"no Hasse-unique odd prime subgroup certificate"}));
                }
            }
            p += 4;
        }
        assert_eq!(
            fixtures.len(),
            3,
            "insufficient primes at exact r bits={bits}"
        );
        let cap = (100 * fixtures.iter().map(|i| i.curve.f.p).max().unwrap() + 10000).to_string();
        let curves: Vec<_> = fixtures.iter().map(curve).collect();
        let arms = vec![
            arm(
                "rho_plain",
                "rho.three_leads_plain_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "rho_neg",
                "rho.three_leads_negation_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "jac_rho",
                "rho.three_leads_jacobian_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "genus_ic",
                "ic.three_leads_genus2_v2",
                json!({"trial_cap":cap,"independent":"0","unique_relations":"1"}),
            ),
        ];
        write(
            dir.join(format!("order-genus-{bits}.json")),
            &spec(
                &format!("three-leads-order-genus-{bits}"),
                curves.clone(),
                60,
                arms,
            ),
        );
        for (name, ind, unique) in [("independent", "1", "1"), ("unfiltered", "0", "0")] {
            write(
                dir.join(format!("order-genus-{bits}-{name}.json")),
                &spec(
                    &format!("three-leads-order-genus-{bits}-{name}"),
                    curves.clone(),
                    1,
                    vec![arm(
                        name,
                        "ic.three_leads_genus2_v2",
                        json!({"trial_cap":cap,"independent":ind,"unique_relations":unique}),
                    )],
                ),
            );
        }
        all.extend(fixtures);
    }
    write(dir.join("fixtures.json"), &json!(all));
    write(dir.join("rejected.json"), &json!(rejected));
}
