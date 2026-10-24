//! Exact source profiles for the four Q28 boundary points.
use super::*;
pub fn profile(dir: &Path) {
    let f = F::new(360, 45);
    let mut records = vec![];
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let mut rows = vec![];
        for input in 0..12 {
            let mut memo = BTreeMap::new();
            for i in 0..p["nodes"].as_array().unwrap().len() {
                let _ = form(&f, p, i, input, &mut memo);
            }
            let max = memo
                .values()
                .filter_map(Option::as_ref)
                .map(|v| v.d)
                .max()
                .unwrap();
            for node in [95, 97, 123] {
                if let Some(Some(v)) = memo.get(&node) {
                    if v.d > 1 {
                        println!("Q29 profile:g{g},node{node},input{input},degree{},depth{},maximum{max}",v.d,v.k);
                        rows.push(json!({"node":node,"input":input,"degree":v.d,"depth":v.k,"terms":v.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"higher_nodes":memo.iter().filter_map(|(j,z)|z.as_ref().filter(|z|z.d>v.d).map(|z|json!({"node":j,"degree":z.d,"depth":z.k}))).collect::<Vec<_>>()}));
                    }
                }
            }
        }
        records.push(json!({"graph":g,"profiles":rows}));
    }
    crate::round02::save(
        dir,
        "q29-source-profiles.json",
        &json!({"schema":"koblitz.round19/q29-profile-v1","records":records}),
    );
}

type Poly = BTreeMap<(usize, usize), E>;
fn plus(a: &mut Poly, b: &Poly) {
    for (m, c) in b {
        let z = a.get(m).map(|v| v.add(c)).unwrap_or_else(|| c.clone());
        if z.to_biguint() == BigUint::from(0u32) {
            a.remove(m);
        } else {
            a.insert(*m, z);
        }
    }
}
fn times(f: &F, a: &Poly, b: &Poly) -> Poly {
    let mut z = Poly::new();
    for ((u, v), c) in a {
        for ((uu, vv), cc) in b {
            let mut t = Poly::new();
            let m = (*u + uu, *v + vv);
            let w = if *c == f.o() {
                cc.clone()
            } else if *cc == f.o() {
                c.clone()
            } else {
                f.mul(c, cc)
            };
            t.insert(m, w);
            plus(&mut z, &t);
        }
    }
    assert!(z.len() <= 50000);
    z
}
fn scale(f: &F, a: &Poly, c: &E) -> Poly {
    a.iter().map(|(m, v)| (*m, f.mul(v, c))).collect()
}
fn upadd(a: &mut Vec<E>, b: &[E], f: &F) {
    a.resize(a.len().max(b.len()), f.z());
    for (i, c) in b.iter().enumerate() {
        a[i] = a[i].add(c);
    }
    while a.last() == Some(&f.z()) {
        a.pop();
    }
}
fn upmul(a: &[E], b: &[E], f: &F) -> Vec<E> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut z = vec![f.z(); a.len() + b.len() - 1];
    for (i, c) in a.iter().enumerate() {
        if *c == f.z() {
            continue;
        }
        for (j, cc) in b.iter().enumerate() {
            if *cc != f.z() {
                z[i + j] = z[i + j].add(&f.mul(c, cc));
            }
        }
    }
    while z.last() == Some(&f.z()) {
        z.pop();
    }
    z
}
fn uv(f: &F, p: &Form) -> Vec<E> {
    let mut z = vec![f.z(); p.d + 1];
    for ((_, x), c) in &p.p {
        z[*x] = c.clone();
    }
    z
}
fn coefficients(a: &[E]) -> Vec<String> {
    a.iter().map(|c| c.to_biguint().to_str_radix(16)).collect()
}
fn norm(f: &F, a: &[E], q: &[Poly]) -> Poly {
    let d = a.len() - 1;
    assert!((1..=12).contains(&d));
    let lead = f.inv(&a[d]);
    let mut matrix = vec![vec![Poly::new(); d]; d];
    for j in 0..d {
        let mut v = vec![Poly::new(); q.len() + j];
        for (i, c) in q.iter().enumerate() {
            v[i + j] = c.clone();
        }
        for k in (d..v.len()).rev() {
            let top = scale(f, &v[k], &lead);
            if top.is_empty() {
                continue;
            }
            for i in 0..d {
                let mut t = scale(f, &top, &a[i]);
                if i == 0 {
                    plus(&mut t, &times(f, &top, &BTreeMap::from([((1, 0), f.o())])));
                }
                plus(&mut v[k - d + i], &t);
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
                plus(&mut z, &times(f, &dets[mask ^ (1 << j)], &matrix[i][j]));
            }
        }
        dets[mask] = z;
    }
    dets.pop().unwrap()
}
pub fn family(dir: &Path) {
    let f = F::new(360, 45);
    let mut records = vec![];
    let mut lean=String::from("import KoblitzRound19Boundary\nnamespace KoblitzRound19Family\ndef topCheck (ts:List (Nat×Nat×Nat)) : Bool := decide (ts.filter (fun v => v.1=0)=[(0,9,0)])\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let mut memo = BTreeMap::new();
        for i in 0..p["nodes"].as_array().unwrap().len() {
            let _ = form(&f, p, i, 0, &mut memo);
        }
        let higher = uv(&f, memo[&95].as_ref().unwrap());
        assert_eq!(higher.len(), 10);
        let mut unique = BTreeMap::<Vec<String>, (Vec<E>, Vec<usize>)>::new();
        for (i, z) in &memo {
            if let Some(z) = z {
                if z.d < 9 && z.p.contains_key(&(0, z.d)) {
                    let a = uv(&f, z);
                    unique
                        .entry(coefficients(&a))
                        .or_insert_with(|| (a, vec![]))
                        .1
                        .push(*i);
                }
            }
        }
        assert!(unique.len() <= 256);
        let mut family = vec![];
        let mut vanish = vec![];
        let mut exponentlists = vec![];
        for (_, (lower, aliases)) in unique {
            let mut q = higher
                .iter()
                .map(|c| {
                    if *c == f.z() {
                        Poly::new()
                    } else {
                        BTreeMap::from([((0, 0), c.clone())])
                    }
                })
                .collect::<Vec<_>>();
            plus(&mut q[0], &BTreeMap::from([((0, 1), f.o())]));
            let mut r = norm(&f, &lower, &q);
            assert!(r.keys().all(|(u, v)| u + v <= 9));
            assert!(r.keys().filter(|(u, v)| u + v == 9).eq([&(9, 0)]));
            let z = f.inv(&r[&(9, 0)]);
            r = scale(&f, &r, &z);
            let mut lp = vec![vec![f.o()]];
            let mut hp = lp.clone();
            for i in 1..=9 {
                lp.push(upmul(&lp[i - 1], &lower, &f));
                hp.push(upmul(&hp[i - 1], &higher, &f));
            }
            let mut residual = vec![];
            for ((u, v), c) in &r {
                upadd(
                    &mut residual,
                    &upmul(&upmul(&lp[*u], &hp[*v], &f), &[c.clone()], &f),
                    &f,
                );
            }
            assert!(residual.is_empty());
            let bitterms = r
                .iter()
                .filter(|(_, c)| c.to_biguint().bit(0))
                .map(|((u, v), _)| (9 - u - v, *u, *v))
                .collect::<Vec<_>>();
            assert_eq!(
                bitterms
                    .iter()
                    .filter(|(h, _, _)| *h == 0)
                    .copied()
                    .collect::<Vec<_>>(),
                vec![(0, 9, 0)]
            );
            exponentlists.push(format!(
                "[{}]",
                bitterms
                    .iter()
                    .map(|(h, u, v)| format!("({h},{u},{v})"))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            vanish.extend(&aliases);
            family.push(json!({"lower_nodes":aliases,"lower_coefficients":coefficients(&lower),"higher_node":95,"degree":9,"normalized_homogeneous_terms":r.iter().map(|((u,v),c)|json!([9-u-v,u,v,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>(),"selected_F2_terms":bitterms,"exact_univariate_substitution_zero":true}));
        }
        vanish.sort();
        vanish.dedup();
        assert!(vanish.contains(&120));
        lean.push_str(&format!("def family{g} : List (List (Nat×Nat×Nat)) := [{}]\ndef nodes{g} : List Nat := [{}]\ntheorem all_actual_tops{g} : family{g}.all topCheck=true := by decide +kernel\ntheorem actual_Q30_unit{g} : KoblitzRound19Boundary.point{g}.contains 120=true := by decide +kernel\ntheorem family_contains_120_{g} : nodes{g}.contains 120=true := by decide +kernel\n",exponentlists.join(","),vanish.iter().map(ToString::to_string).collect::<Vec<_>>().join(",")));
        println!(
            "Q31:g{g},{} distinct exact resultants exclude{} lower-degree nodes and Q30",
            family.len(),
            vanish.len()
        );
        records.push(json!({"graph":g,"family":family,"radical_boundary_zero_nodes":vanish,"source_certificate":memo.iter().filter_map(|(i,z)|z.as_ref().map(|z|json!({"node":i,"gate":p["nodes"][*i],"degree":z.d,"h_depth":z.k,"terms":z.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>(),"Q30_excluded":true,"full_saturation":false}));
    }
    lean.push_str("end KoblitzRound19Family\n");
    for g in 0..4 {
        for name in [
            format!("all_actual_tops{g}"),
            format!("actual_Q30_unit{g}"),
            format!("family_contains_120_{g}"),
        ] {
            lean.push_str(&format!("#print axioms KoblitzRound19Family.{name}\n"));
        }
    }
    let path = dir.join("KoblitzRound19Family.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "q31-pure-source-resultant-family.json",
        &json!({"schema":"koblitz.round19/q31-v1","records":records,"scope":"source-bound reduced boundary radical consequences from exact polynomial elimination;not full ideal saturation"}),
    );
}
pub fn resultant(dir: &Path) {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let old = read(&dir.join("q28-after-source-square-repairs.json"));
    let mut rows = vec![];
    let mut lean =
        String::from("import KoblitzRound18FinalBoundary\nset_option maxHeartbeats 0\nset_option maxRecDepth 100000\nnamespace KoblitzRound19Separators\n");
    for g in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{g}-F2-relations.json")));
        let p = &doc["program"];
        let mut memo = BTreeMap::new();
        let pp = form(&f, p, 95, 0, &mut memo).unwrap();
        let ss = form(&f, p, 122, 0, &mut memo).unwrap();
        assert_eq!((pp.d, ss.d), (9, 8));
        assert_eq!(p["nodes"][123]["args"], json!([1, 122]));
        let aa = if p["nodes"][21]["op"] == "Constant" {
            assert_eq!(p["nodes"][21]["args"], "0");
            f.z()
        } else {
            assert_eq!(p["nodes"][21]["args"], json!([15, 20]));
            f.o()
        };
        assert!(aa == f.z() || aa == f.o());
        let a = uv(&f, &pp);
        let s = uv(&f, &ss);
        let lead = f.inv(a.last().unwrap());
        let d = 9usize;
        // q(x,V)=V²+x*S(x)*V+S(x)²*(x³+a*x²+1), reduced modulo P(x)-U.
        let square = upmul(&s, &s, &f);
        let mut r = vec![f.z(); square.len() + 3];
        for (i, c) in square.iter().enumerate() {
            r[i] = r[i].add(c);
            r[i + 2] = r[i + 2].add(&f.mul(c, &aa));
            r[i + 3] = r[i + 3].add(c);
        }
        let qdeg = r.len() - 1;
        assert_eq!(qdeg, 19);
        let mut q = vec![Poly::new(); r.len()];
        for (i, c) in r.iter().enumerate() {
            if *c != f.z() {
                q[i].insert((0, 0), c.clone());
            }
        }
        q[0].insert((0, 2), f.o());
        for (i, c) in s.iter().enumerate() {
            if *c != f.z() {
                q[i + 1].insert((0, 1), c.clone());
            }
        }
        let mut matrix = vec![vec![Poly::new(); d]; d];
        for j in 0..d {
            let mut v = vec![Poly::new(); q.len() + j];
            for (i, c) in q.iter().enumerate() {
                v[i + j] = c.clone();
            }
            for k in (d..v.len()).rev() {
                let top = scale(&f, &v[k], &lead);
                if top.is_empty() {
                    continue;
                }
                for i in 0..d {
                    let mut t = scale(&f, &top, &a[i]);
                    if i == 0 {
                        plus(&mut t, &times(&f, &top, &BTreeMap::from([((1, 0), f.o())])));
                    }
                    plus(&mut v[k - d + i], &t);
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
        let mut out = dets.pop().unwrap();
        let degree = out.keys().map(|(u, v)| u + v).max().unwrap();
        assert_eq!(degree, 19);
        assert!(out.keys().filter(|(u, v)| u + v == degree).eq([&(19, 0)]));
        let normal = f.inv(&out[&(19, 0)]);
        out = scale(&f, &out, &normal);
        assert_eq!(out[&(19, 0)], f.o());
        // Independent exact substitution in the elliptic coordinate ring, using a+y*b pairs.
        let mut powers = vec![vec![f.o()]];
        for i in 1..=19 {
            powers.push(upmul(&powers[i - 1], &a, &f));
        }
        let cr = vec![f.o(), f.z(), aa.clone(), f.o()];
        let mut yp = vec![(vec![f.o()], vec![])];
        for i in 1..=18 {
            let (u, v) = &yp[i - 1];
            let vs = upmul(v, &s, &f);
            let first = upmul(&vs, &cr, &f);
            let mut second = upmul(u, &s, &f);
            let mut shifted = vec![f.z()];
            shifted.extend(vs);
            upadd(&mut second, &shifted, &f);
            yp.push((first, second));
        }
        let mut residual = (vec![], vec![]);
        for ((u, v), c) in &out {
            let t = upmul(&powers[*u], &[c.clone()], &f);
            upadd(&mut residual.0, &upmul(&t, &yp[*v].0, &f), &f);
            upadd(&mut residual.1, &upmul(&t, &yp[*v].1, &f), &f);
        }
        assert!(
            residual.0.is_empty() && residual.1.is_empty(),
            "actual elliptic coordinate-ring substitution must vanish"
        );
        let terms = out
            .iter()
            .map(|((u, v), c)| json!([degree - u - v, u, v, c.to_biguint().to_str_radix(16)]))
            .collect::<Vec<_>>();
        let bitterms = out
            .iter()
            .filter(|(_, c)| c.to_biguint().bit(0))
            .map(|((u, v), _)| (degree - u - v, *u, *v))
            .collect::<Vec<_>>();
        let expr = bitterms
            .iter()
            .map(|(h, u, v)| format!("({h},{u},{v})"))
            .collect::<Vec<_>>()
            .join(",");
        lean.push_str(&format!("def terms{g} : List (Nat×Nat×Nat) := [{expr}]\ndef separator{g} (h U V:ZMod 2) : ZMod 2 := (terms{g}.map fun (a,b,c) => h^a*U^b*V^c).sum\ntheorem boundary{g} (V:ZMod 2) : separator{g} 0 1 V=1 := by revert V; decide +kernel\ntheorem actual_Q28_unit{g} : KoblitzRound18FinalBoundary.point{g}.contains 95=true := by decide +kernel\ntheorem actual_y_gate{g} : KoblitzRound12Boundary.program{g}[123]?=some (.mul 1 122) := by decide +kernel\n"));
        assert_eq!(old["records"][g]["model"][95], 1);
        println!("Q29:g{g},coupled norm resultant has{} terms,degree{degree},unique top U^19;exact elliptic substitution zero;Q28 excluded",out.len());
        rows.push(json!({"graph":g,"lower_node":95,"other_node":123,"source_P_node":95,"source_S_node":122,"P_degree":9,"S_degree":8,"ordinary_a":aa.to_biguint().to_str_radix(16),"P":coefficients(&a),"S":coefficients(&s),"source_depths":[pp.k,ss.k],"resultant_homogeneous_terms":terms,"selected_F2_bit":0,"selected_F2_terms":bitterms,"ordinary_degree":19,"unique_top_term":[19,0,"1"],"exact_elliptic_coordinate_ring_substitution_zero":true,"Q28_excluded_under_source_bridge":true,"full_saturation":false,"source_certificate":memo.iter().filter_map(|(i,z)|z.as_ref().map(|z|json!({"node":i,"gate":p["nodes"][*i],"degree":z.d,"h_depth":z.k,"terms":z.p.iter().map(|(m,c)|json!([m.0,m.1,c.to_biguint().to_str_radix(16)])).collect::<Vec<_>>()}))).collect::<Vec<_>>()}));
    }
    lean.push_str("end KoblitzRound19Separators\n");
    for g in 0..4 {
        for n in ["boundary", "actual_Q28_unit", "actual_y_gate"] {
            lean.push_str(&format!("#print axioms KoblitzRound19Separators.{n}{g}\n"));
        }
    }
    let path = dir.join("KoblitzRound19Separators.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "q29-coupled-elliptic-resultants.json",
        &json!({"schema":"koblitz.round19/q29-v1","records":rows,"scope":"exact source-coupled elimination and reduced boundary exclusion,not complete saturation;resultant construction/substitution native,scalar rejection and actual gate bindings Lean"}),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiplication_norm_linear_control() {
        let f = F::new(4, 3);
        let a = vec![f.z(), f.o()];
        let q = vec![
            BTreeMap::from([((0, 1), f.o()), ((0, 0), f.e(7))]),
            BTreeMap::from([((0, 0), f.e(3))]),
            BTreeMap::from([((0, 0), f.o())]),
        ];
        assert_eq!(
            norm(&f, &a, &q),
            BTreeMap::from([
                ((0, 1), f.o()),
                ((0, 0), f.e(7)),
                ((1, 0), f.e(3)),
                ((2, 0), f.o())
            ])
        );
    }
    #[test]
    fn determinant_norm_vanishes_at_independent_small_field_points() {
        let f = F::new(4, 3);
        for degree in 2..=4 {
            let a = (0..=degree)
                .map(|i| {
                    if i == degree {
                        f.o()
                    } else {
                        f.e((3 * i + 5) as u64)
                    }
                })
                .collect::<Vec<_>>();
            let b = vec![f.e(9), f.e(2), f.o(), f.e(7), f.o()];
            let mut q = b
                .iter()
                .map(|c| BTreeMap::from([((0, 0), c.clone())]))
                .collect::<Vec<_>>();
            q[0].insert((0, 1), f.o());
            let r = norm(&f, &a, &q);
            for x in 0..16 {
                let x = f.e(x);
                let eval = |p: &[E]| p.iter().rev().fold(f.z(), |v, c| f.mul(&v, &x).add(c));
                let (u, v) = (eval(&a), eval(&b));
                let got = r.iter().fold(f.z(), |z, ((i, j), c)| {
                    z.add(&f.mul(c, &f.mul(&f.powu(&u, *i as u64), &f.powu(&v, *j as u64))))
                });
                assert_eq!(got, f.z());
            }
        }
    }
}

#[path = "source20.rs"]
pub(crate) mod next;
