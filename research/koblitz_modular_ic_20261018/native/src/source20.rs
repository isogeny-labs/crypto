//! Coupled quotient elimination for the actual remaining Q32 source cones.
use super::*;
fn ringmul(f: &F, a: &(Vec<E>, Vec<E>), b: &(Vec<E>, Vec<E>), curve: &[E]) -> (Vec<E>, Vec<E>) {
    let yy = upmul(&a.1, &b.1, f);
    let mut z = upmul(&a.0, &b.0, f);
    upadd(&mut z, &upmul(&yy, curve, f), f);
    let mut w = upmul(&a.0, &b.1, f);
    upadd(&mut w, &upmul(&a.1, &b.0, f), f);
    let mut xy = vec![f.z()];
    xy.extend(yy);
    upadd(&mut w, &xy, f);
    (z, w)
}
fn powers(f: &F, v: &(Vec<E>, Vec<E>), n: usize, c: &[E]) -> Vec<(Vec<E>, Vec<E>)> {
    let mut out = vec![(vec![f.o()], vec![])];
    for i in 1..=n {
        out.push(ringmul(f, &out[i - 1], v, c));
    }
    out
}
pub fn quotient(dir: &Path) {
    let f = F::new(360, 45);
    let prior = read(&dir.join("q32-after-pure-source-repairs.json"));
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound19FinalBoundary\nset_option maxHeartbeats 0\nnamespace KoblitzRound20Separators\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let mut memo = BTreeMap::new();
        let pp = form(&f, p, 97, 0, &mut memo).unwrap();
        let dd = form(&f, p, 98, 0, &mut memo).unwrap();
        let ss = form(&f, p, 122, 0, &mut memo).unwrap();
        let a = uv(&f, &pp);
        let mut denominator = uv(&f, &dd);
        while denominator.last() == Some(&f.z()) {
            denominator.pop();
        }
        let s = uv(&f, &ss);
        let d = a.len() - 1;
        assert!(
            d <= 12 && denominator.len() <= d,
            "registered monic-algebra trigger"
        );
        for (i, op, args) in [
            (123, "Mul", json!([1, 122])),
            (124, "Add", json!([97, 123])),
            (125, "Inverse", json!(98)),
            (126, "Mul", json!([124, 125])),
        ] {
            assert_eq!(p["nodes"][i]["op"], op);
            assert_eq!(p["nodes"][i]["args"], args);
        }
        let aa = if p["nodes"][21]["op"] == "Constant" {
            assert_eq!(p["nodes"][21]["args"], "0");
            f.z()
        } else {
            assert_eq!(p["nodes"][21]["args"], json!([15, 20]));
            f.o()
        };
        let curve = vec![f.o(), f.z(), aa.clone(), f.o()];
        let sq = upmul(&s, &s, &f);
        let constant = upmul(&sq, &curve, &f);
        let mut q = vec![Poly::new(); constant.len()];
        for (i, c) in constant.iter().enumerate() {
            if *c != f.z() {
                q[i].insert((0, 0), c.clone());
            }
        }
        q[0].insert((2, 0), f.o());
        for (i, c) in s.iter().enumerate() {
            if *c != f.z() {
                q[i + 1].insert((1, 0), c.clone());
            }
        }
        // Reduce in P(x)+U+D(x)V, whose x-leading coefficient is constant.
        let inv = f.inv(&a[d]);
        let mut relation = (0..d)
            .map(|i| {
                let mut z = Poly::new();
                if a[i] != f.z() {
                    z.insert((0, 0), a[i].clone());
                }
                if i < denominator.len() && denominator[i] != f.z() {
                    z.insert((0, 1), denominator[i].clone());
                }
                if i == 0 {
                    z.insert((1, 0), f.o());
                }
                z
            })
            .collect::<Vec<_>>();
        for r in &mut relation {
            *r = scale(&f, r, &inv);
        }
        let mut matrix = vec![vec![Poly::new(); d]; d];
        for j in 0..d {
            let mut v = vec![Poly::new(); q.len() + j];
            for (i, c) in q.iter().enumerate() {
                v[i + j] = c.clone();
            }
            for k in (d..v.len()).rev() {
                let top = v[k].clone();
                if !top.is_empty() {
                    for i in 0..d {
                        plus(&mut v[k - d + i], &times(&f, &top, &relation[i]));
                    }
                }
                v[k].clear();
            }
            for i in 0..d {
                matrix[i][j] = v[i].clone();
            }
        }
        let mut dets = vec![Poly::new(); 1 << d];
        dets[0].insert((0, 0), f.o());
        for mask in 1usize..1 << d {
            let i = mask.count_ones() as usize - 1;
            let mut z = Poly::new();
            for j in 0..d {
                if mask >> j & 1 != 0 {
                    plus(&mut z, &times(&f, &dets[mask ^ (1 << j)], &matrix[i][j]));
                }
            }
            dets[mask] = z;
        }
        let mut result = dets.pop().unwrap();
        let degree = result.keys().map(|(u, v)| u + v).max().unwrap();
        let lc = result
            .iter()
            .find(|((u, v), _)| u + v == degree)
            .unwrap()
            .1
            .clone();
        result = scale(&f, &result, &f.inv(&lc));
        let maxu = result.keys().map(|m| m.0).max().unwrap();
        let maxv = result.keys().map(|m| m.1).max().unwrap();
        let up = powers(&f, &(vec![], s.clone()), maxu, &curve);
        let vp = powers(&f, &(a.clone(), s.clone()), maxv, &curve);
        let dp = powers(&f, &(denominator.clone(), vec![]), maxv, &curve);
        let mut residual = (vec![], vec![]);
        for ((u, v), c) in &result {
            let m = ringmul(
                &f,
                &ringmul(&f, &up[*u], &vp[*v], &curve),
                &dp[maxv - v],
                &curve,
            );
            upadd(&mut residual.0, &upmul(&m.0, &[c.clone()], &f), &f);
            upadd(&mut residual.1, &upmul(&m.1, &[c.clone()], &f), &f);
        }
        assert!(
            residual.0.is_empty() && residual.1.is_empty(),
            "independent denominator-cleared elliptic substitution"
        );
        let top = result
            .iter()
            .filter(|((u, v), _)| u + v == degree)
            .map(|((u, v), c)| json!([u, v, c.to_biguint().to_str_radix(16)]))
            .collect::<Vec<_>>();
        let boundary_value = result
            .iter()
            .filter(|((u, v), _)| u + v == degree)
            .fold(f.z(), |z, (_, c)| z.add(c));
        let mixed =
            top.len() == 1 && top[0][0].as_u64().unwrap() > 0 && top[0][1].as_u64().unwrap() > 0;
        assert_eq!(prior["records"][g]["model"][123], 1);
        assert_eq!(prior["records"][g]["model"][126], 1);
        let terms = result
            .iter()
            .map(|((u, v), c)| json!([degree - u - v, u, v, c.to_biguint().to_str_radix(16)]))
            .collect::<Vec<_>>();
        let scalar = result
            .iter()
            .filter(|(_, c)| c.to_biguint().bit(0))
            .map(|((u, v), _)| (degree - u - v, *u, *v))
            .collect::<Vec<_>>();
        let lits = scalar
            .iter()
            .map(|(h, u, v)| format!("({h},{u},{v})"))
            .collect::<Vec<_>>()
            .join(",");
        lean.push_str(&format!("def terms{g} : List (Nat×Nat×Nat) := [{lits}]\ndef boundary{g} (U V:ZMod 2):ZMod 2 := (terms{g}.map fun (h,u,v)=> (0:ZMod 2)^h*U^u*V^v).sum\ntheorem actual_scalar_satisfaction{g} : boundary{g} 1 1=0 := by decide +kernel\ntheorem actual_Q32_units{g} : KoblitzRound19FinalBoundary.point{g}.contains 123=true ∧ KoblitzRound19FinalBoundary.point{g}.contains 126=true := by decide +kernel\ntheorem actual_denominator_gate{g} : KoblitzRound12Boundary.program{g}[125]?=some (.inv 98) := by decide +kernel\n"));
        assert_eq!(denominator, vec![f.o()]);
        assert_eq!(boundary_value, f.z());
        assert_eq!(top.len(), 8);
        let exps = [0usize, 1, 2, 3, 16, 17, 18, 19];
        assert!(top
            .iter()
            .zip(exps)
            .all(|(v, u)| v == &json!([u, 19 - u, "1"])));
        let tops = exps
            .iter()
            .map(|u| format!("({u},{})", 19 - u))
            .collect::<Vec<_>>()
            .join(",");
        lean.push_str(&format!("theorem actual_top_shape{g} : (terms{g}.filterMap fun (h,u,v)=> if h=0 then some (u,v) else none)=[{tops}] := by decide +kernel\n"));
        println!("Q33:g{g},degrees P{d}/D{}/S{},{} resultant terms,total degree{degree},top{top:?},Q32 excluded{}",denominator.len()-1,s.len()-1,result.len(),boundary_value!=f.z());
        records.push(json!({"graph":g,"P_node":97,"S_node":122,"D_node":98,"U_node":123,"V_node":126,"P":coefficients(&a),"S":coefficients(&s),"D":coefficients(&denominator),"ordinary_a":aa.to_biguint().to_str_radix(16),"matrix":matrix.iter().map(|r|r.iter().map(|v|v.iter().map(|((u,v),c)|json!([u,v,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>(),"resultant_homogeneous_terms":terms,"ordinary_degree":degree,"top_terms":top,"top_is_single_mixed_monomial":mixed,"new_zero_product_pair":if mixed{json!([123,126])}else{json!(null)},"new_boundary_linear_pair":[123,126],"top_is_sum_power19":true,"Q32_boundary_residue":boundary_value.to_biguint().to_str_radix(16),"Q32_excluded":boundary_value!=f.z(),"denominator_clear_power":maxv,"exact_cleared_elliptic_substitution_zero":true,"full_saturation":false}));
    }
    lean.push_str("end KoblitzRound20Separators\n");
    for g in 0..4 {
        for n in [
            "actual_scalar_satisfaction",
            "actual_top_shape",
            "actual_Q32_units",
            "actual_denominator_gate",
        ] {
            lean.push_str(&format!("#print axioms KoblitzRound20Separators.{n}{g}\n"));
        }
    }
    assert!(!dir.join("KoblitzRound20Separators.lean").exists());
    std::fs::write(dir.join("KoblitzRound20Separators.lean"), lean).unwrap();
    crate::round02::save(
        dir,
        "q33-coupled-quotient-resultant.json",
        &json!({"schema":"koblitz.round20/q33-v1","records":records,"scope":"exact coupled rational source elimination;geometric closure uses written unit-chart bridge;not whole ideal saturation"}),
    );
}
