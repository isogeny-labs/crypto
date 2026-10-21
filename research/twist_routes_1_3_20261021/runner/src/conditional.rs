use crate::{field::*, write_json, Fixture, Result};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, path::Path};
fn isqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let (mut lo, mut hi) = (0u128, 1u128 << ((128 - n.leading_zeros() + 1) / 2));
    while lo + 1 < hi {
        let m = (lo + hi) / 2;
        if m * m <= n {
            lo = m
        } else {
            hi = m
        }
    }
    if hi * hi == n {
        hi
    } else {
        lo
    }
}
fn embedding(p: &BigUint, r: &BigUint, limit: u64) -> Option<u64> {
    let mut x = BigUint::one();
    for k in 1..=limit {
        x = x * p % r;
        if x.is_one() {
            return Some(k);
        }
    }
    None
}
pub fn run(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut applicability = vec![];
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let n = BigUint::from(f.order);
        let nt = BigUint::from(2 * (f.p + 1) - f.order);
        let gcd = n.gcd(&nt);
        applicability.push(json!({"fixture":f.id,"source_order":f.order,"twist_order":nt.to_string(),"gcd":gcd.to_string(),"nonzero_r_primary_hom_to_rational_twist_possible":!gcd.is_one(),"embedding_degree":embedding(&BigUint::from(f.p),&n,f.order),"bound":f.order,"public_parameter_check":true}));
    }
    let registry: Value = serde_json::from_slice(&fs::read("docs/curves/registry.json")?)?;
    for name in ["P-192", "P-224", "P-256", "P-384", "P-521"] {
        let row = registry["curves"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| {
                x["standard_names"]
                    .as_array()
                    .is_some_and(|ns| ns.iter().any(|n| n == name))
            })
            .ok_or(format!("standard {name} not found"))?;
        let p: BigUint = row["params"]["p"].as_str().unwrap().parse()?;
        let r: BigUint = row["order"].as_str().unwrap().parse()?;
        let nt = (&p + BigUint::one()) * 2u32 - &r;
        let g = r.gcd(&nt);
        applicability.push(json!({"fixture":name,"source_registry_slug":row["slug"],"source_order":r.to_string(),"twist_order":nt.to_string(),"gcd":g.to_string(),"nonzero_r_primary_hom_to_rational_twist_possible":!g.is_one(),"embedding_degree_if_at_most_1024":embedding(&p,&r,1024),"otherwise_embedding_degree_lower_bound":1024,"field_parameter_is_published":true,"large_order_primality_source":"standard domain parameters; not a new primality proof","attack_or_target_run":false}));
    }
    let mut lifts = vec![];
    for f in fixtures.iter().filter(|f| f.role == "structural") {
        let e = f.curve();
        let mut squares = 0;
        let mut nonsquares = 0;
        let mut total = 0;
        let mut projections = BTreeSet::new();
        let mut by_d = vec![];
        let mut zero_relations = 0;
        for d in -64i64..=64 {
            if d == 0 {
                continue;
            }
            let dr = d.rem_euclid(e.p as i64) as u64;
            if dr == 0 {
                continue;
            }
            let root = sqrt(dr, e.p);
            if root.is_some() {
                squares += 1
            } else {
                nonsquares += 1;
            }
            let mut points = BTreeSet::new();
            for x in -64i64..=64 {
                let rhs = x as i128 * x as i128 * x as i128 + f.a as i128 * x as i128 + f.b as i128;
                if rhs % d as i128 != 0 {
                    continue;
                }
                let ys = rhs / d as i128;
                if ys < 0 {
                    continue;
                }
                let y = isqrt(ys as u128);
                if y > 4096 || y * y != ys as u128 {
                    continue;
                }
                for y in [y as i64, -(y as i64)] {
                    points.insert((x, y));
                }
            }
            let mut row_images = BTreeSet::new();
            if let Some(s) = root {
                for &(x, y) in &points {
                    let p = Some((
                        Q(x.rem_euclid(e.p as i64) as u64, 0),
                        Q(y.rem_euclid(e.p as i64) as u64 * s % e.p, 0),
                    ));
                    assert!(e.on(p));
                    row_images.insert(p);
                    projections.insert(p);
                }
            }
            // Opposite integral points are explicit trivial controls, never independent relations.
            let trivial = points
                .iter()
                .filter(|(x, y)| *y > 0 && points.contains(&(*x, -*y)))
                .count();
            zero_relations += trivial;
            total += points.len();
            by_d.push(json!({"d":d,"square_mod_p":root.is_some(),"integral_points":points.len(),"distinct_original_curve_images":row_images.len(),"trivial_opposite_pairs_excluded":trivial}));
        }
        let all = e.rational();
        let targets:Vec<_>=(0..8).map(|seed|{let k=1+(20261021+seed*7919)%(f.order-1);let q=e.pm(f.group(),k as u128);json!({"seed":seed,"target":q,"has_integral_lift_in_frozen_box":projections.contains(&q)})}).collect();
        let coverage = all.iter().filter(|p| projections.contains(p)).count();
        lifts.push(json!({"fixture":f.id,"status":"executed-bounded-integral-lift-ingress","d_range":[-64,64],"x_range":[-64,64],"absolute_y_cap":4096,"d_square_mod_p_cases":squares,"d_nonsquare_mod_p_controls":nonsquares,"integral_point_records_including_repeated_reductions":total,"unique_images_on_original_curve":projections.len(),"nonidentity_target_population":f.order-1,"population_targets_with_frozen_integral_lift":coverage,"independent_single_target_controls":targets,"trivial_pairs_excluded":zero_relations,"certified_Mordell_Weil_rank_or_independence":null,"nontrivial_rational_relation_search":"not implemented; this is bounded integral ingress, not the full arithmetic-lift route","by_twist":by_d}));
    }
    write_json(out.join("E16-applicability.json"), &applicability)?;
    write_json(out.join("E17-lifts.json"), &lifts)?;
    Ok(vec![
        json!({"experiment":"E16","status":"executed-exact-gcd-and-bounded-embedding-degree-applicability","rows":applicability.len()}),
        json!({"experiment":"E17","status":"executed-bounded-integral-ingress; nontrivial rational relation/height search remains open","rows":lifts.len()}),
    ])
}
