//! Exact cusp strata for the eight coupled functions. Global Tate bridges are written.
use crate::{
    round02,
    round04::{Curve, Field, Pt},
};
use serde_json::json;
use std::{collections::BTreeSet, path::Path};
fn canon(b: u32, d: u32) -> (u32, u32) {
    (b, d).min(((11 - b) % 11, (11 - d) % 11))
}
fn functions(f: Field, p: (u32, u32), q: (u32, u32)) -> [u32; 8] {
    let (x, y) = p;
    let (u, v) = q;
    [
        x ^ u,
        x ^ u ^ 1,
        y ^ v,
        y ^ v ^ 1,
        f.mul(x, u),
        f.mul(x, u) ^ 1,
        f.mul(x, v) ^ f.mul(u, y),
        f.mul(x, v) ^ f.mul(u, y) ^ 1,
    ]
}
fn middle(f: Field, x: u32) -> (u32, u32) {
    let y = f.mul(x ^ 1, f.inv(f.sq(x) ^ x ^ 1));
    let c = Curve { f, a: 1, b: 1 };
    assert!(c.points().contains(&Pt::A(x, y)));
    (x, y)
}
fn jets(f: Field, p: (u32, u32)) -> Vec<u32> {
    let (x, y) = p;
    let mut v = vec![y];
    for n in 1..9 {
        let sq = if n % 2 == 0 { f.sq(v[n / 2]) } else { 0 };
        let rhs = match n {
            1 => f.sq(x),
            2 => x ^ 1,
            3 => 1,
            _ => 0,
        };
        v.push(f.mul(sq ^ v[n - 1] ^ rhs, f.inv(x)));
    }
    for n in 1..9 {
        let residual = (if n % 2 == 0 { f.sq(v[n / 2]) } else { 0 })
            ^ f.mul(x, v[n])
            ^ v[n - 1]
            ^ match n {
                1 => f.sq(x),
                2 => x ^ 1,
                3 => 1,
                _ => 0,
            };
        assert_eq!(residual, 0)
    }
    v
}
pub fn run(dir: &Path) {
    let f = Field::find(3);
    let mu = Field::find(10);
    let z = (2..1024).map(|a| mu.pow(a, 93)).find(|a| *a != 1).unwrap();
    assert_eq!(mu.pow(z, 11), 1);
    let phases: Vec<_> = (0..11).map(|i| mu.pow(z, i)).collect();
    let mut leading = 0;
    for i in 0..11 {
        for j in 0..11 {
            if i == j {
                continue;
            }
            for m in [1, 2, 3, 5] {
                assert_ne!(mu.pow(phases[i], m), mu.pow(phases[j], m));
                leading += 1;
            }
        }
    }
    let mut labels = BTreeSet::new();
    for b in 0..11 {
        for d in 0..11 {
            if b != 0 || d != 0 {
                labels.insert(canon(b, d));
            }
        }
    }
    assert_eq!(labels.len(), 60);
    let mut known = BTreeSet::new();
    let mut orbits = vec![];
    for label in &labels {
        if known.contains(label) {
            continue;
        }
        let mut orbit = vec![];
        let mut p = *label;
        while known.insert(p) {
            orbit.push(p);
            p = canon(p.0, 6 * p.1 % 11);
        }
        assert_eq!(p, *label);
        orbits.push(orbit)
    }
    assert_eq!(orbits.iter().map(Vec::len).sum::<usize>(), 60);
    let gen = (2..8).find(|x| f.pow(*x, 7) == 1 && *x != 1).unwrap();
    let mut base = vec![
        json!({"name":"O","point":"O"}),
        json!({"name":"T","point":[0,1]}),
    ];
    for a in 1..7 {
        let p = middle(f, f.pow(gen, a));
        base.push(json!({"name":format!("M{a}"),"point":[p.0,p.1]}));
    }
    let names = [
        "x0+xinf",
        "x0+xinf+1",
        "y0+yinf",
        "y0+yinf+1",
        "x0*xinf",
        "x0*xinf+1",
        "x0*yinf+xinf*y0",
        "x0*yinf+xinf*y0+1",
    ];
    let mut poles = [0i64; 8];
    let mut zeros = [0i64; 8];
    let mut records = vec![];
    let mut jet_checks = 0;
    let mut ram_sums = [0, 0];
    let mut product_divisor = vec![];
    for ci in 0..8 {
        for &(b, d) in &labels {
            let canonical = if b == 0 {
                0
            } else if d == 0 {
                11
            } else {
                (11 - b) * ((1..11).find(|i| i * d % 11 == 1).unwrap()) % 11
            };
            let e0 = if canonical == 0 { 121 } else { 1 };
            let ei = if canonical == 11 { 121 } else { 1 };
            if ci == 0 {
                ram_sums[0] += e0;
                ram_sums[1] += ei;
            }
            let mut val = [0i64; 8];
            let target;
            if ci == 0 {
                let max = e0.max(ei);
                let ps = 2 * max;
                let py = 3 * max;
                let pp = 2 * (e0 + ei);
                let pc = (2 * e0 + 3 * ei).max(3 * e0 + 2 * ei);
                val = [-ps, -ps, -py, -py, -pp, -pp, -pc, -pc];
                target = json!(["O", "O"]);
            } else if ci == 1 {
                val[0] = 2;
                val[2] = 1;
                val[4] = 2 * (e0 + ei);
                val[6] = 2;
                target = json!([[0, 1], [0, 1]]);
            } else {
                let x = f.pow(gen, (ci - 1) as u64);
                let p = middle(f, x);
                let ca = (f.pow(p.0, 11), f.pow(p.1, 11));
                let nc = (f.pow(p.0, 2), f.pow(p.1, 2));
                let p0 = if canonical == 0 { ca } else { nc };
                let pi = if canonical == 11 { ca } else { nc };
                target = json!([[p0.0, p0.1], [pi.0, pi.1]]);
                let fs = functions(f, p0, pi);
                if e0 == 1 && ei == 1 {
                    assert_eq!(p0, pi);
                    let jy = jets(f, p0);
                    assert_ne!(jy[1], 0);
                    assert_ne!(f.mul(p0.0, jy[1]) ^ p0.1, 0);
                    for k in 0..8 {
                        if fs[k] == 0 {
                            assert!([0, 2, 6].contains(&k));
                            val[k] = 1;
                        }
                    }
                    jet_checks += 1;
                } else {
                    let jy = jets(f, nc);
                    for k in 0..8 {
                        let series: Vec<_> = (0..9)
                            .map(|n| {
                                let qx = if n == 0 {
                                    nc.0
                                } else if n == 1 {
                                    1
                                } else {
                                    0
                                };
                                let qy = jy[n];
                                let c = if n == 0 && (k % 2 == 1) { 1 } else { 0 };
                                match k / 2 {
                                    0 => qx ^ (if n == 0 { ca.0 } else { 0 }) ^ c,
                                    1 => qy ^ (if n == 0 { ca.1 } else { 0 }) ^ c,
                                    2 => f.mul(ca.0, qx) ^ c,
                                    3 => f.mul(ca.0, qy) ^ f.mul(ca.1, qx) ^ c,
                                    _ => unreachable!(),
                                }
                            })
                            .collect();
                        assert_eq!(series[0], fs[k]);
                        let order = series
                            .iter()
                            .position(|x| *x != 0)
                            .expect("bounded elliptic jet")
                            as i64;
                        val[k] = order;
                    }
                    jet_checks += 1;
                }
            }
            for i in 0..8 {
                if val[i] < 0 {
                    poles[i] -= val[i]
                } else {
                    zeros[i] += val[i]
                }
            }
            if ci <= 1 {
                product_divisor.push(json!({"base_cusp":base[ci],"eleven_exponent_vector_mod_sign":[b,d],"canonical_line":canonical,"valuation":val[4],"all_twelve_targets":if ci==0{"O"}else{"T=(0,1)"},"even_and_odd_projected_images":"zero"}));
            }
            records.push(json!({"base_cusp":ci,"vector_mod_sign":[b,d],"canonical_line":canonical,"ramification_phi0":e0,"ramification_phiinf":ei,"target_pair":target,"valuations":val}));
        }
    }
    assert_eq!(ram_sums, [660, 660]);
    assert_eq!(records.len(), 480);
    assert_eq!(poles, [2520, 2520, 3780, 3780, 2640, 2640, 3900, 3900]);
    assert_eq!(zeros[4], 2640);
    let results:Vec<_>=(0..8).map(|i|json!({"function":names[i],"pole_degree":poles[i],"cuspidal_zero_degree":zeros[i],"residual_noncuspidal_zero_degree":poles[i]-zeros[i],"complete_divisor":i==4,"complete_closed_ordinary_support":false})).collect();
    let product_closed:Vec<_>=["O","T"].iter().flat_map(|c|orbits.iter().map(move|o|{let(b,d)=o[0];let m=if b==0||d==0{244}else{4};json!({"base_cusp":c,"exponent_orbit":o,"closed_degree":o.len(),"multiplicity":if *c=="O"{-(m as i64)}else{m as i64}})})).collect();
    let sections = [
        "X*W+U*Z+c*Z*W",
        "Y*W+V*Z+c*Z*W",
        "X*U+c*Z*W",
        "X*V+U*Y+c*Z*W",
    ];
    round02::save(
        dir,
        "m13-coupled-cusp-valuations.json",
        &json!({"schema":"koblitz.round10/m13-v1","base_cusps":base,"geometric_cusps":480,"level_cusp_vectors":labels,"cusp_records":records,"functions":results,"distinct_leading_coefficients_checked":leading,"middle_cusp_jet_checks":jet_checks,"projection_degree_checks":ram_sums,"ambient_bihomogeneous_sections":sections,"ambient_fixed_components":["V+H","none","V+H","none"],"ambient_polar_classes":[[2,2],[3,3],[2,2],[3,3]],"diagonal_divisor_identity":"div(x0+xinf)=(phi0-phiinf)^*(O)+(phi0+phiinf)^*(O)-2phi0^*(O)-2phiinf^*(O)","difference_map_degree":1280,"sum_map_degree":1360,"scope":"complete cusp strata and all poles under written Tate/modular cusp identification;seven residual noncuspidal supports remain uncomputed"}),
    );
    round02::save(
        dir,
        "m13b-complete-product-divisor.json",
        &json!({"schema":"koblitz.round10/m13b-v1","function":"x0*xinf","geometric_divisor":product_divisor,"closed_divisor":product_closed,"pole_degree":2640,"zero_degree":2640,"residual_zero_degree":0,"F2_cusp_action":"(b,d) maps to (b,d/2) modulo simultaneous sign;both base cusps rational","complete_geometric_and_closed_divisor":true,"new_noncuspidal_columns":0,"relation_rank_promoted":false,"scope":"actual cusp divisor under written normalized Tate-level chart;all projected images zero;does not resolve other seven functions"}),
    );
    println!("M13:480 cusps,all eight pole divisors;complete x0*xinf divisor,22 closed cusps;residuals {:?}",(0..8).map(|i|poles[i]-zeros[i]).collect::<Vec<_>>());
}
