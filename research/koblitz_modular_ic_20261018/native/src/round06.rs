//! Round06 modular image evaluation. Prior abstract labels are refined here.
use crate::{large_field::*, round02, round04};
use num_bigint::BigUint;
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
fn pointx(p: &P) -> E {
    match p {
        P::Affine { x, .. } => x.clone(),
        _ => panic!("finite torsion required"),
    }
}
fn tuple(f: &F, c: &crate::binary_ecc::BinaryCurve, targets: &[P], r: usize, sign: i64) -> Vec<P> {
    let image = |a: usize| {
        let den = (1 + a * r) % 11;
        let index = if den == 0 {
            11
        } else {
            a * (1..11).find(|v| v * den % 11 == 1).unwrap() % 11
        };
        targets[index].clone()
    };
    let mut ds = vec![];
    for i in 0..5 {
        let a = 2usize.pow(i) % 11;
        let mut p = plus(
            c,
            &image(a),
            &if sign == 1 {
                image(11 - a)
            } else {
                neg(&image(11 - a))
            },
        );
        if sign == -1 && i % 2 == 1 {
            p = neg(&p);
        }
        ds.push(p);
    }
    let total = ds.iter().fold(P::Infinity, |v, p| plus(c, &v, p));
    let _ = f;
    ds.iter()
        .map(|p| plus(c, &times(c, p, 5), &neg(&total)))
        .collect()
}
fn target_frob(f: &F, p: &[P], sign: i64) -> Vec<P> {
    (0..5)
        .map(|i| {
            let q = f.frobp(&p[(i + 4) % 5], 1);
            if sign == 1 {
                q
            } else {
                neg(&q)
            }
        })
        .collect()
}
pub fn seven_fibers(dir: &Path) {
    let input = round04::read(&dir.join("i1-degree210-images.json"));
    let f = F::new(210, 129);
    let ec = f.curve(f.o(), f.o());
    let elem = |v: &serde_json::Value| {
        E::from_biguint(
            &BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
            210,
        )
    };
    let point = |v: &serde_json::Value| {
        if v.is_array() {
            P::Affine {
                x: elem(&v[0]),
                y: elem(&v[1]),
            }
        } else {
            P::Infinity
        }
    };
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, r) in input["chains"].as_array().unwrap().iter().enumerate() {
        groups
            .entry(r["first_kernel_roots"].to_string())
            .or_default()
            .push(i);
    }
    let mut records = vec![];
    let cusp = P::Affine { x: f.z(), y: f.o() };
    for members in groups.values() {
        assert_eq!(members.len(), 7);
        let sums: Vec<_> = (0..12)
            .map(|a| {
                members.iter().fold(P::Infinity, |v, i| {
                    plus(&ec, &v, &point(&input["chains"][*i]["hecke_targets"][a]))
                })
            })
            .collect();
        assert!(sums.iter().all(|p| *p == cusp));
        records.push(
            json!({"chains":members,"all12_target_sums":sums.iter().map(pj).collect::<Vec<_>>()}),
        );
    }
    fn pmul(mut a: u32, mut b: u32) -> u32 {
        let mut p = 0;
        while b != 0 {
            if b & 1 != 0 {
                p ^= a;
            }
            a <<= 1;
            b >>= 1;
        }
        p
    }
    let mask = (1 << 13) - 1;
    let v = 2u32;
    let mut x = 0;
    for _ in 0..13 {
        x = ((1 << 2) ^ pmul(x, v) ^ pmul(x, x) ^ pmul(pmul(x, x), x)) & mask;
    }
    assert_eq!(
        (pmul(pmul(x, x), x) ^ pmul(x, x) ^ x ^ pmul(x, v) ^ (1 << 2)) & mask,
        0
    );
    let eta_y = (pmul(x, x) ^ pmul(pmul(x, x) ^ x ^ 1, v)) & mask;
    assert_eq!(x.trailing_zeros(), 2);
    assert_eq!(eta_y.trailing_zeros(), 1);
    round02::save(
        dir,
        "i1c-seven-fiber-cancellation.json",
        &json!({"schema":"koblitz.round06/i1c-v1","seven_fibers":records,"all_target_fiber_checks":96,"constant_sum_cusp":pj(&cusp),"cusp_order":2,"local_parameter":"v=Y+1 at T=(0,1)","local_x_series_bits_through_order12":x,"eta_y_series_bits_through_order12":eta_y,"t0_zero_order":7,"degree7_equation":"x^7+t*(x^6+x^5+x^4)+t^2*(x^6+x^5+x^4+x^3+x^2+x+1)=0","derived_projected_seven_sums_zero":true,"scope":"exact fiber sums and local jets, plus written degree/local geometry; explains zero columns rather than treating rank5 as useful nonzero-column coverage"}),
    );
    println!("I1c:96 seven-fiber target sums equal the order2 cusp; projected sums vanish");
}
pub fn check_images(dir: &Path) {
    let input = round04::read(&dir.join("i1-degree210-images.json"));
    let f = F::new(210, 129);
    assert!(f.irreducible());
    let ec = f.curve(f.o(), f.o());
    let elem = |v: &serde_json::Value| {
        E::from_biguint(
            &BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
            210,
        )
    };
    let point = |v: &serde_json::Value| {
        if v.is_array() {
            P::Affine {
                x: elem(&v[0]),
                y: elem(&v[1]),
            }
        } else {
            P::Infinity
        }
    };
    let targets: Vec<Vec<P>> = input["chains"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r["hecke_targets"]
                .as_array()
                .unwrap()
                .iter()
                .map(&point)
                .collect()
        })
        .collect();
    let succ: Vec<usize> = serde_json::from_value(input["chain_frobenius15"].clone()).unwrap();
    let mut counts = vec![];
    for (si, sign) in [1, -1].into_iter().enumerate() {
        let tuples: Vec<Vec<P>> = (0..616)
            .map(|i| tuple(&f, &ec, &targets[i / 11], i % 11, sign))
            .collect();
        let raw_nonzero = tuples
            .iter()
            .filter(|v| v.iter().any(|p| *p != P::Infinity))
            .count();
        for i in 0..616 {
            let mut v = tuples[i].clone();
            for _ in 0..15 {
                v = target_frob(&f, &v, sign);
            }
            assert_eq!(v, tuples[succ[i / 11] * 11 + (11 - i % 11) % 11]);
        }
        let mut relative_nonzero = 0;
        let mut final_nonzero = 0;
        for r in input["columns"].as_array().unwrap() {
            let orbit: Vec<usize> =
                serde_json::from_value(r["members_over_b_root"].clone()).unwrap();
            let mut relative = vec![P::Infinity; 5];
            for i in orbit {
                for j in 0..5 {
                    relative[j] = plus(&ec, &relative[j], &tuples[i][j]);
                }
            }
            if relative.iter().any(|p| *p != P::Infinity) {
                relative_nonzero += 1;
            }
            let mut final_trace = vec![P::Infinity; 5];
            let mut term = relative;
            for _ in 0..15 {
                for j in 0..5 {
                    final_trace[j] = plus(&ec, &final_trace[j], &term[j]);
                }
                term = target_frob(&f, &term, sign);
            }
            let stored: Vec<P> = r["images"][si]["trace_tuple"]
                .as_array()
                .unwrap()
                .iter()
                .map(&point)
                .collect();
            assert_eq!(final_trace, stored);
            if final_trace.iter().any(|p| *p != P::Infinity) {
                final_nonzero += 1;
            }
        }
        counts.push(json!({"sign":sign,"geometric_tuples_nonzero":raw_nonzero,"all_frobenius15_checks":616,"relative14_traces_nonzero":relative_nonzero,"final15_traces_nonzero":final_nonzero}));
    }
    // Cross-check field multiplication against independent bit-schoolbook reduction.
    let mut checked = 0;
    for row in &targets {
        for p in row {
            if let P::Affine { x, y } = p {
                assert_eq!(f.mul(x, y), x.schoolbook_mul(y, &f.irr));
                checked += 1;
            }
        }
    }
    round02::save(
        dir,
        "i1b-image-checks.json",
        &json!({"schema":"koblitz.round06/i1b-v1","counts":counts,"field_schoolbook_crosschecks":checked,"status":"PASS","scope":"same-agent separate orbit accumulation (14 then15), all616 equivariance checks per sign, maintained-library field schoolbook crosscheck; no external audit"}),
    );
    println!("I1b: all616 Frobenius checks per sign, separate14x15 traces and{checked} field crosschecks PASS");
}
pub fn matrix_check(dir: &Path) {
    use num_bigint::BigInt;
    let old = round04::read(&dir.join("m4-direct-modular-relations.json"));
    let images = round04::read(&dir.join("i1-degree210-images.json"));
    assert_eq!(images["nonzero_by_sign"], json!([0, 0]));
    let mut matrix: Vec<Vec<BigInt>> = old["principal_relations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut v: Vec<BigInt> = r["row_mod11"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| BigInt::from(x.as_u64().unwrap()))
                .collect();
            v.extend(vec![BigInt::from(0); 44]);
            v
        })
        .collect();
    let mut new = vec![BigInt::from(0); 544];
    new.extend(vec![BigInt::from(1); 44]);
    matrix.push(new);
    assert_eq!(round02::rank(matrix.clone(), &BigInt::from(11)), 5);
    let oldonly = matrix.iter().map(|r| r[..544].to_vec()).collect();
    assert_eq!(round02::rank(oldonly, &BigInt::from(11)), 4);
    let mut rows = vec![];
    for sign in 0..2 {
        let columns: Vec<usize> = old["closed_points"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, r)| r["images"][sign]["nonzero"] == true)
            .map(|(i, _)| i)
            .collect();
        let selected = matrix
            .iter()
            .map(|r| columns.iter().map(|i| r[*i].clone()).collect())
            .collect();
        let rank = round02::rank(selected, &BigInt::from(11));
        assert_eq!(rank, 4);
        rows.push(
            json!({"a":if sign==0{1}else{0},"nonzero_image_columns":columns.len(),"rank":rank}),
        );
    }
    round02::save(
        dir,
        "i2-zero-column-matrix-check.json",
        &json!({"schema":"koblitz.round06/i2-v1","evaluated_rank":5,"evaluated_columns":588,"new44_images_zero_both_signs":true,"rank_after_removing_new_zero_columns":4,"nonzero_image_matrix_by_sign":rows,"scope":"extra rank is confined to new zero-image columns; no new constraint on the old nonzero image columns"}),
    );
    println!(
        "I2: rank5/588, rank4 after removing new zero columns; nonzero image matrices remain rank4"
    );
}
pub fn images(dir: &Path) {
    let small = round04::Field {
        m: 15,
        modulus: 46149,
    };
    let s = round04::Field::find(30);
    let map = s.embed(small);
    let b30 = map[2];
    let c30 = round04::Curve { f: s, a: 1, b: b30 };
    let p30 = round04::Pt::A(1, map[11254]);
    assert!(c30.valid(p30));
    assert_eq!(c30.times(p30, 11), round04::Pt::O);
    let order = ((1i64 << 30) + 1 - (231i64 * 231 - 2 * (1 << 15))) as u64;
    let mut h = order;
    while h % 11 == 0 {
        h /= 11;
    }
    let mut q30 = None;
    for x in 1..4096 {
        let Some(z) = c30.point(x) else {
            continue;
        };
        let mut z = c30.times(z, h);
        if z == round04::Pt::O {
            continue;
        }
        while c30.times(z, 11) != round04::Pt::O {
            z = c30.times(z, 11);
        }
        z = c30.times(c30.add(c30.frob(z, 15), c30.neg(z)), 5);
        if z != round04::Pt::O {
            assert_eq!(c30.frob(z, 15), c30.neg(z));
            q30 = Some(z);
            break;
        }
    }
    let zeta = (2..4096)
        .map(|v| s.pow(v, ((1u64 << 30) - 1) / 11))
        .find(|v| *v != 1)
        .unwrap();
    let (q30, _) =
        crate::modular_geometry::paired_basis(c30, p30, q30.expect("eleven basis budget"), zeta);
    let f = F::find(210);
    let root = f.root(s.modulus);
    let embed = |v: u32| f.embed_bits(v as u64, &root);
    let b = embed(b30);
    let original = f.curve(f.o(), b.clone());
    let ec = f.curve(f.o(), f.o());
    let conv = |p: round04::Pt| match p {
        round04::Pt::O => P::Infinity,
        round04::Pt::A(x, y) => P::Affine {
            x: embed(x),
            y: embed(y),
        },
    };
    let (p, q) = (conv(p30), conv(q30));
    assert!(original.is_on_curve(&p) && original.is_on_curve(&q));
    let mut k11 = vec![];
    for a in 0..=11 {
        let z = if a == 11 {
            q.clone()
        } else {
            plus(&original, &p, &times(&original, &q, a))
        };
        k11.push(
            (1..=5)
                .map(|i| pointx(&times(&original, &z, i)))
                .collect::<Vec<_>>(),
        );
    }
    let k7 = kernels7(&f, &b);
    println!("I1: first-stage eight7 kernels split");
    let mut chains = vec![];
    for (i, k) in k7.iter().enumerate() {
        let b7 = qb(&f, &b, k);
        let dual = from_roots(&f, &velu(&f, k, &k7[(i + 1) % 8]));
        let second = kernels7(&f, &b7);
        for l in second {
            if from_roots(&f, &l) == dual {
                continue;
            }
            let b49 = qb(&f, &b7, &l);
            let js = [f.inv(&b), f.inv(&b7), f.inv(&b49)];
            let source = reconstruct(&f, &js);
            let mut targets = vec![];
            for a in &k11 {
                let b0 = qb(&f, &b, a);
                let transported7 = velu(&f, a, k);
                let transported11 = velu(&f, k, a);
                let b70 = qb(&f, &b0, &transported7);
                assert_eq!(b70, qb(&f, &b7, &transported11), "commuting square");
                let last = velu(&f, &transported11, &l);
                let b490 = qb(&f, &b70, &last);
                targets.push(reconstruct(&f, &[f.inv(&b0), f.inv(&b70), f.inv(&b490)]));
            }
            let sum = targets.iter().fold(P::Infinity, |v, p| plus(&ec, &v, p));
            assert_eq!(sum, times(&ec, &source, 4), "Hecke sum");
            chains.push((k.clone(), l, source, targets));
        }
        println!("I1: first kernel{i}, {} chains evaluated", chains.len());
    }
    assert_eq!(chains.len(), 56);
    let key = |k: &[E], l: &[E]| {
        format!(
            "{}:{}",
            polyj(&from_roots(&f, k)),
            polyj(&from_roots(&f, l))
        )
    };
    let index: BTreeMap<_, _> = chains
        .iter()
        .enumerate()
        .map(|(i, (k, l, ..))| (key(k, l), i))
        .collect();
    assert_eq!(index.len(), 56);
    let succ: Vec<usize> = chains
        .iter()
        .map(|(k, l, ..)| {
            let k: Vec<_> = k.iter().map(|v| f.frob(v, 15)).collect();
            let l: Vec<_> = l.iter().map(|v| f.frob(v, 15)).collect();
            index[&key(&k, &l)]
        })
        .collect();
    let mut seen = vec![false; 616];
    let mut orbits = vec![];
    for start in 0..616 {
        if seen[start] {
            continue;
        }
        let mut k = start;
        let mut orbit = vec![];
        loop {
            assert!(!seen[k]);
            seen[k] = true;
            orbit.push(k);
            k = succ[k / 11] * 11 + (11 - k % 11) % 11;
            if k == start {
                break;
            }
        }
        assert_eq!(orbit.len(), 14);
        orbits.push(orbit);
    }
    assert_eq!(orbits.len(), 44);
    let f32 = round04::Field::find(5);
    let map32 = s.embed(f32);
    let ef32: Vec<E> = map32.iter().map(|v| embed(*v)).collect();
    let s4 = embed(s.quadratic(1).unwrap());
    let mut columns = vec![];
    let mut sums = [P::Infinity, P::Infinity];
    let mut nonzero = [0, 0];
    for (col, orb) in orbits.iter().enumerate() {
        let start = orb[0];
        let ci = start / 11;
        let r = start % 11;
        let mut images = vec![];
        for (si, sign) in [1, -1].into_iter().enumerate() {
            let ds = tuple(&f, &ec, &chains[ci].3, r, sign);
            let ns = succ[ci];
            let next = tuple(&f, &ec, &chains[ns].3, (11 - r) % 11, sign);
            let mut frob = ds.clone();
            for _ in 0..15 {
                frob = target_frob(&f, &frob, sign);
            }
            assert_eq!(frob, next, "full normalized Frobenius15");
            let mut image = vec![P::Infinity; 5];
            let mut term = ds;
            for _ in 0..210 {
                for i in 0..5 {
                    image[i] = plus(&ec, &image[i], &term[i]);
                }
                term = target_frob(&f, &term, sign);
            }
            assert_eq!(target_frob(&f, &image, sign), image);
            assert_eq!(
                image.iter().fold(P::Infinity, |v, p| plus(&ec, &v, p)),
                P::Infinity
            );
            assert_eq!(times(&ec, &image[0], 11), P::Infinity);
            let descended = match &image[0] {
                P::Infinity => P::Infinity,
                P::Affine { x, y } => P::Affine {
                    x: x.clone(),
                    y: if sign == 1 {
                        y.clone()
                    } else {
                        y.add(&f.mul(x, &s4))
                    },
                },
            };
            let target = f.curve(f.e(if sign == 1 { 1 } else { 0 }), f.o());
            assert!(target.is_on_curve(&descended));
            assert_eq!(f.frobp(&descended, 5), descended);
            sums[si] = plus(&target, &sums[si], &descended);
            if descended != P::Infinity {
                nonzero[si] += 1;
            }
            let compressed = match &descended {
                P::Infinity => json!({"infinity":true}),
                P::Affine { x, y } => json!([
                    ef32.iter().position(|v| v == x).expect("F32 x"),
                    ef32.iter().position(|v| v == y).expect("F32 y")
                ]),
            };
            images.push(json!({"a":if sign==1{1}else{0},"image_F32":compressed,"image_large_field":pj(&descended),"trace_tuple":image.iter().map(pj).collect::<Vec<_>>()}));
        }
        columns.push(json!({"column":col,"degree":210,"members_over_b_root":orb,"chain":ci,"complement":r,"source":pj(&chains[ci].2),"images":images}));
        println!("I1 column{col}: evaluated both signs");
    }
    assert!(sums.iter().all(|p| *p == P::Infinity), "new principal row");
    round02::save(
        dir,
        "i1-degree210-images.json",
        &json!({"schema":"koblitz.round06/i1-v1","field":{"degree":210,"low_terms":f.irr.low_terms,"Rabin_irreducible":true},"F30_modulus":s.modulus,"F30_embedding_root":ej(&root),"F15_embedding_into_F30":map,"fixed_weil_pairing_F30":zeta,"basis11":[pj(&p),pj(&q)],"F32_modulus":f32.modulus,"F32_embedding":ef32.iter().map(ej).collect::<Vec<_>>(),"chains":chains.iter().map(|(k,l,s,t)|json!({"first_kernel_roots":k.iter().map(ej).collect::<Vec<_>>(),"second_kernel_roots":l.iter().map(ej).collect::<Vec<_>>(),"source":pj(s),"hecke_targets":t.iter().map(pj).collect::<Vec<_>>()})).collect::<Vec<_>>(),"chain_frobenius15":succ,"columns":columns,"closed_columns":44,"closed_degree":210,"nonzero_by_sign":nonzero,"principal_row_sums":sums.iter().map(pj).collect::<Vec<_>>(),"evaluated_matrix_rank":5,"evaluated_matrix_columns":588,"scope":"explicit ordinary7/11 modular kernels and point images, all44 closed traces; prior abstract companion labels refined by actual kernel coordinates; no individual descent or reverse Jacobian reduction"}),
    );
}
