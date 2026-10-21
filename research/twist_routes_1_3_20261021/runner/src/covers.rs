use crate::{field::*, identity, jacobian::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Selected {
    pub fixture: String,
    pub c: u64,
    pub twist_scale: u64,
    pub ad: u64,
    pub bd: u64,
    pub order: u64,
}
fn count(p: u64, a: u64, b: u64, qr: &[i64]) -> u64 {
    let mut n = p as i64 + 1;
    for x in 0..p {
        n += qr[((x * x % p * x + a * x + b) % p) as usize];
    }
    n as u64
}
fn torsion(f: &Fixture) -> Value {
    let e = f.curve();
    let k = Sext::new(e);
    let alpha = S([0, 1, 0, 0, 0, 0]);
    let disc = k.neg(k.add(k.mul(k.c(3), k.mul(alpha, alpha)), k.c(4 * e.a % e.p)));
    let sd = k.sqrt(disc).unwrap();
    let half = k.inv(k.c(2));
    let roots = [
        alpha,
        k.mul(k.add(k.neg(alpha), sd), half),
        k.mul(k.sub(k.neg(alpha), sd), half),
    ];
    assert!(roots.iter().all(|x| x.0[3..] == [0, 0, 0]));
    assert_eq!(roots.iter().collect::<BTreeSet<_>>().len(), 3);
    let frob: Vec<_> = roots
        .iter()
        .map(|x| {
            roots
                .iter()
                .position(|y| *y == k.pow(*x, e.p as u128))
                .unwrap()
        })
        .collect();
    assert!(frob.iter().enumerate().all(|(i, x)| i != *x));
    let mut translated = 0;
    let mut rational_translates = 0;
    for p in e.rational() {
        let ps = p.map(|(x, y)| (k.lift(x), k.lift(y)));
        for &root in &roots {
            let t = Some((root, Z));
            assert!(k.on(t, k.c(e.a), k.c(e.b)));
            let s = k.pa(ps, t, k.c(e.a));
            assert_eq!(k.pm(s, 2, k.c(e.a)), k.pm(ps, 2, k.c(e.a)));
            let fs = k.frob(s);
            let expected = k.pa(ps, k.frob(t), k.c(e.a));
            assert_eq!(fs, expected);
            translated += 1;
            if s.is_none_or(|(x, y)| k.fp(x).is_some() && k.fp(y).is_some()) {
                rational_translates += 1;
            }
        }
    }
    assert_eq!(rational_translates, 0);
    json!({"fixture":f.id,"cubic_extension":f.cubic_extension_id,"sextic_extension":f.sextic_extension_id,"status":"verified-exhaustive-rational-translation-control","nonidentity_2_torsion":3,"frobenius_permutation":frob,"translated_points_checked":translated,"translated_points_in_original_rational_group":rational_translates,"doubling_and_galois_constraints_verified":true,"rational_doubling_image":e.rational().iter().map(|p|e.pm(*p,2)).collect::<BTreeSet<_>>().len(),"rational_base_compression_from_nonrational_torsion":false,"solver_degree_or_cost_advantage":null})
}
pub fn run(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut selected = vec![];
    let mut rows = vec![];
    let mut glue = vec![];
    let mut torsions = vec![];
    let mut images = vec![];
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let e = f.curve();
        let p = e.p;
        torsions.push(torsion(f));
        let mut qr = vec![-1i64; p as usize];
        qr[0] = 0;
        for y in 1..p {
            qr[(y * y % p) as usize] = 1;
        }
        let ta = e.d * e.d % p * e.a % p;
        let tb = e.d * e.d % p * e.d % p * e.b % p;
        let nt = 2 * (p + 1) - f.order;
        let j0 = identity(p, ta, tb, nt as u128, None)["j"]
            .as_str()
            .unwrap()
            .to_string();
        let mut singular = 0;
        let mut smooth = 0;
        let mut same_j = 0;
        let mut opposite = 0;
        let mut matched = vec![];
        for c in 0..p {
            let fc = e.rhs(Q(c, 0)).0;
            if fc == 0 {
                singular += 1;
                continue;
            }
            let a2 = (3 * c * c + e.a) % p;
            let ad = (9 * c % p * fc + p - a2 * a2 % p) % p * inv(3, p) % p;
            let bd = (2 * a2 % p * a2 % p * a2 + 27 * fc % p * fc + p
                - 27 * a2 % p * c % p * fc % p)
                % p
                * inv(27 % p, p)
                % p;
            assert_ne!((4 * ad % p * ad % p * ad + 27 * bd % p * bd) % p, 0);
            let nd = count(p, ad, bd, &qr);
            smooth += 1;
            let jd = identity(p, ad, bd, nd as u128, None)["j"]
                .as_str()
                .unwrap()
                .to_string();
            if jd == j0 {
                same_j += 1;
            }
            if nd == nt {
                opposite += 1;
            }
            if jd == j0 && nd == nt {
                let scale = (1..p)
                    .find(|&u| ta * pow(u, 4, p) % p == ad && tb * pow(u, 6, p) % p == bd)
                    .unwrap();
                matched.push(Selected {
                    fixture: f.id.clone(),
                    c,
                    twist_scale: scale,
                    ad,
                    bd,
                    order: nd,
                });
            }
        }
        rows.push(json!({"fixture":f.id,"status":"verified-exhaustive-c-parameter-search","parameters_tested":p,"singular_f_c_zero":singular,"smooth_covers":smooth,"same_j_complements":same_j,"opposite_trace_complements":opposite,"actual_Fp_twist_isomorphisms":matched.len(),"matched_c":matched.iter().map(|m|m.c).collect::<Vec<_>>(),"opposite_trace_without_twist_isomorphism":opposite-matched.len()as u64,"same_j_without_required_twist_isomorphism":same_j-matched.len()as u64,"destination_twist":f.twist_id}));
        let chosen = matched.first().cloned();
        if let Some(s) = chosen {
            let mut full = BTreeSet::new();
            let mut finite = 0;
            for u in 0..p {
                let x = (u * u + s.c) % p;
                if let Some(y) = sqrt(e.rhs(Q(x, 0)).0, p) {
                    finite += if y == 0 { 1 } else { 2 };
                    full.insert(canonical(e, Some((Q(x, 0), Q(y, 0)))));
                }
            }
            for h in [2, 4, 8] {
                let mut image = BTreeSet::new();
                let mut raw = 0;
                for u in 0..h {
                    let x = (u * u + s.c) % p;
                    if let Some(y) = sqrt(e.rhs(Q(x, 0)).0, p) {
                        raw += if y == 0 { 1 } else { 2 };
                        image.insert(canonical(e, Some((Q(x, 0), Q(y, 0)))));
                    }
                }
                images.push(json!({"fixture":f.id,"c":s.c,"H":h,"finite_points_in_restriction":raw,"signed_projected_columns":image.len(),"full_finite_cover_points":finite,"full_projected_columns":full.len(),"genus":2,"degree":2,"infinity_points":2,"same_projected_rational_control":true,"status":"verified-exact-cover-image"}));
            }
            selected.push(s);
        }
        let mut eq = 0;
        let cycle = [1, 2, 0];
        for perm in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            if (0..3).all(|i| perm[cycle[i]] == cycle[perm[i]]) {
                eq += 1;
            }
        }
        assert_eq!(eq, 3);
        let t = p as i64 + 1 - f.order as i64;
        let mut anti3 = 0;
        for a in 0..3 {
            for b in 0..3 {
                for c in 0..3 {
                    for d in 0..3 {
                        if (a * d + 9 - b * c) % 3 != 2 {
                            continue;
                        }
                        let m = [[a, b], [c, d]];
                        let fs = [[0, (3 - p % 3) % 3], [1, t.rem_euclid(3) as u64]];
                        let ft = [[0, (3 - p % 3) % 3], [1, (-t).rem_euclid(3) as u64]];
                        let product = |x: [[u64; 2]; 2], y: [[u64; 2]; 2]| {
                            [
                                [
                                    (x[0][0] * y[0][0] + x[0][1] * y[1][0]) % 3,
                                    (x[0][0] * y[0][1] + x[0][1] * y[1][1]) % 3,
                                ],
                                [
                                    (x[1][0] * y[0][0] + x[1][1] * y[1][0]) % 3,
                                    (x[1][0] * y[0][1] + x[1][1] * y[1][1]) % 3,
                                ],
                            ]
                        };
                        if product(ft, m) == product(m, fs) {
                            anti3 += 1;
                        }
                    }
                }
            }
        }
        glue.push(json!({"fixture":f.id,"two_torsion_equivariant_bijections":eq,"generic_j_geometric_isomorphism_induced_maps":1,"other_two_gluing_candidates":eq-1,"actual_twist_complement_cover_found":!matched.is_empty(),"three_torsion_companion_Frobenius_anti_intertwiners":anti3,"three_gluing_construction":if anti3==0{"inapplicable: Frobenius module obstruction"}else{"not implemented: quotient curve and polarization verification still required"},"status":"executed-gluing-applicability-and-explicit-two-cover-search","full_three_gluing_experiment_complete":anti3==0}));
        println!(
            "{}",
            json!({"stage":"cover-search","fixture":f.id,"parameters":p,"twist_complements":matched.len()})
        );
    }
    write_json(out.join("E09-torsion.json"), &torsions)?;
    write_json(out.join("E10-cover-search.json"), &rows)?;
    write_json(out.join("E11-gluing.json"), &glue)?;
    write_json(out.join("E12-selected-covers.json"), &selected)?;
    write_json(out.join("E13-images.json"), &images)?;
    Ok(vec![
        json!({"experiment":"E09","status":"executed-torsion-and-Galois-controls; solver cost remains separate","rows":torsions.len()}),
        json!({"experiment":"E10","status":"executed-exhaustive-c-search","rows":rows.len()}),
        json!({"experiment":"E11","status":"executed-bounded-gluing-applicability; admitted 3-gluing construction gaps retained","rows":glue.len()}),
        json!({"experiment":"E13","status":"executed-exact-image-diagnostics; pipeline costs remain separate","rows":images.len()}),
    ])
}
fn canonical(e: Quad, p: P) -> P {
    p.min(e.pn(p))
}
fn fp_point(k: Sext, p: SP) -> P {
    p.map(|(x, y)| {
        (
            Q(k.fp(x).expect("norm x not rational"), 0),
            Q(k.fp(y).expect("norm y not rational"), 0),
        )
    })
}
pub fn transfer(fixtures: &[Fixture], selected: &[Selected], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    for s in selected {
        let f = fixtures.iter().find(|f| f.id == s.fixture).unwrap();
        let e = f.curve();
        let h = Chart::new(e, s.c);
        assert_eq!((h.ad, h.bd), (s.ad, s.bd));
        let d = Quad::new(e.p, h.ad, h.bd);
        let ep = e.rational();
        let dp = d.rational();
        assert_eq!(dp.len() as u64, s.order);
        let mut es = vec![];
        let mut ds = vec![];
        for &p in &ep {
            let j = h.pull_e(p);
            assert_eq!(fp_point(h.k, h.norm(&j, false)), e.pm(p, 2));
            assert_eq!(h.norm(&j, true), None);
            assert_eq!(h.add(&j, &h.neg(&j)), Div::zero());
            es.push(j);
        }
        for &p in &dp {
            let j = h.pull_d(p);
            assert_eq!(fp_point(h.k, h.norm(&j, true)), d.pm(p, 2));
            assert_eq!(h.norm(&j, false), None);
            ds.push(j);
        }
        let n = ep.len();
        for i in 0..n {
            assert_eq!(
                h.add(&es[i], &h.pull_e(f.group())),
                h.pull_e(e.pa(ep[i], f.group()))
            );
        }
        let jp = h.pull_e(f.group());
        assert_ne!(jp, Div::zero());
        assert_eq!(h.scalar(&jp, f.order as u128), Div::zero());
        for seed in 0..8 {
            let scalar = 1 + (20261021 + seed * 7919) % (f.order - 1);
            let q = e.pm(f.group(), scalar as u128);
            assert_eq!(h.scalar(&jp, scalar as u128), h.pull_e(q));
            let norm = fp_point(h.k, h.norm(&h.pull_e(q), false));
            assert_eq!(e.pm(norm, inv(2, f.order) as u128), q);
        }
        let exhaustive = f.p == 59;
        let mut class_map = BTreeMap::new();
        let budget = if exhaustive { es.len() * ds.len() } else { 256 };
        for i in 0..budget {
            let ei = if exhaustive {
                i / ds.len()
            } else {
                (i * 31) % es.len()
            };
            let di = if exhaustive {
                i % ds.len()
            } else {
                (i * 67 + 1) % ds.len()
            };
            let j = h.add(&es[ei], &ds[di]);
            let ne = fp_point(h.k, h.norm(&j, false));
            let nd = fp_point(h.k, h.norm(&j, true));
            assert_eq!(ne, e.pm(ep[ei], 2));
            assert_eq!(nd, d.pm(dp[di], 2));
            let old = class_map.insert((ne, nd), j.clone());
            if let Some(old) = old {
                assert_eq!(old, j);
            }
        }
        if exhaustive {
            assert_eq!(class_map.len(), ep.len() * dp.len());
        }
        let classes: Vec<_> = class_map.values().cloned().collect();
        let pairs = classes.len().min(128);
        for i in 0..pairs {
            let x = &classes[i];
            let y = &classes[(i * 37 + 11) % classes.len()];
            let z = h.add(x, y);
            assert_eq!(
                fp_point(h.k, h.norm(&z, false)),
                e.pa(
                    fp_point(h.k, h.norm(x, false)),
                    fp_point(h.k, h.norm(y, false))
                )
            );
            assert_eq!(
                fp_point(h.k, h.norm(&z, true)),
                d.pa(
                    fp_point(h.k, h.norm(x, true)),
                    fp_point(h.k, h.norm(y, true))
                )
            );
            assert_eq!(h.add(&h.add(x, y), &jp), h.add(x, &h.add(y, &jp)));
        }
        // Independent point-count path to the genus-2 Jacobian order.
        let mut n1 = 2u64;
        for x in 0..e.p {
            if let Some(y) = sqrt(e.rhs(Q((x * x + s.c) % e.p, 0)).0, e.p) {
                n1 += if y == 0 { 1 } else { 2 };
            }
        }
        let mut n2 = None;
        if f.p <= 827 {
            let mut count = 2u64;
            for a in 0..e.p {
                for b in 0..e.p {
                    let u = Q(a, b);
                    let t = e.add(e.mul(u, u), Q(s.c, 0));
                    if let Some(y) = e.sqrt(e.rhs(t)) {
                        count += if y == Q(0, 0) { 1 } else { 2 };
                    }
                }
            }
            n2 = Some(count);
            let ss = e.p as i128 + 1 - n1 as i128;
            let order = e.p as i128 * e.p as i128 + 1 - (e.p as i128 + 1) * ss
                + (ss * ss - (e.p as i128 * e.p as i128 + 1 - count as i128)) / 2;
            assert_eq!(order, ep.len() as i128 * dp.len() as i128);
        }
        rows.push(json!({"fixture":f.id,"c":s.c,"destination_twist":f.twist_id,"status":"verified-native-chart-transport","original_infinity":"two rational; explicitly transported, not passed to one-infinity API","field":f.sextic_extension_id,"branch_root_tower_coordinates":h.root,"chart_scale":h.lambda,"odd_model_coefficients_tower_coordinates":h.f,"pullback_source_points_checked":ep.len(),"complement_pullback_points_checked":dp.len(),"origin_and_complement_infinity_charts":true,"source_generator_order":f.order,"scalar_controls":8,"composition_norm_pullback":"[2]","arbitrary_reduced_classes_checked":classes.len(),"all_rational_classes_enumerated":exhaustive,"norm_additivity_and_associativity_samples":pairs,"cover_point_count_Fp":n1,"cover_point_count_Fp2":n2,"jacobian_order_from_two_point_counts":n2.map(|_|ep.len()*dp.len()),"independent_CAS_or_host_replay":false,"formal_group_and_chart_bridge":false,"destination_solver_speedup":null}));
        println!(
            "{}",
            json!({"stage":"transfer","fixture":f.id,"classes":classes.len(),"source_points":ep.len()})
        );
    }
    write_json(out.join("E12-transfers.json"), &rows)?;
    Ok(rows)
}
