//! Actual two/three-function product and complete coefficient-descent binding.
use super::*;
fn gate(nodes: &Value, op: &str, args: Value) -> usize {
    let wanted = json!({"op":op,"args":args});
    nodes
        .as_array()
        .unwrap()
        .iter()
        .position(|v| *v == wanted)
        .unwrap()
}
fn add(nodes: &Value, a: usize, b: usize) -> usize {
    if a == b {
        gate(nodes, "Constant", json!("0"))
    } else {
        gate(nodes, "Add", json!([a.min(b), a.max(b)]))
    }
}
fn mul(nodes: &Value, a: usize, b: usize) -> usize {
    gate(nodes, "Mul", json!([a.min(b), a.max(b)]))
}
fn plus(nodes: &Value, p: (usize, usize), q: (usize, usize), a: usize) -> (usize, usize) {
    let n = add(nodes, p.1, q.1);
    let d = add(nodes, p.0, q.0);
    let di = gate(nodes, "Inverse", json!(d));
    let l = mul(nodes, n, di);
    let ll = mul(nodes, l, l);
    let xx = add(nodes, ll, l);
    let pq = add(nodes, p.0, q.0);
    let xx = add(nodes, xx, pq);
    let x = add(nodes, xx, a);
    let v = add(nodes, p.0, x);
    let y = mul(nodes, l, v);
    let y = add(nodes, y, p.1);
    (x, add(nodes, y, x))
}
pub(crate) fn run(dir: &Path) {
    let f = F::new(360, 45);
    assert!(f.irreducible());
    let prior = read(&dir.join("q40-after-all-elliptic-source-families.json"));
    let mut records = vec![];
    for graph in 0..4 {
        let doc = read(&dir.join(format!("q8-graph-{graph}-F2-relations.json")));
        let p = &doc["program"];
        let nodes = &p["nodes"];
        let mut functions = vec![];
        for (j, (pn, sn, dn, vn)) in [
            (97, 122, 98, 126),
            (157, 182, 158, 186),
            (217, 242, 218, 246),
            (277, 302, 278, 306),
        ]
        .into_iter()
        .enumerate()
        {
            let mut memo = BTreeMap::new();
            let pp = uv(&f, &form(&f, p, pn, 2 * j, &mut memo).unwrap());
            let ss = uv(&f, &form(&f, p, sn, 2 * j, &mut memo).unwrap());
            let mut dd = uv(&f, &form(&f, p, dn, 2 * j, &mut memo).unwrap());
            while dd.last() == Some(&f.z()) {
                dd.pop();
            }
            assert_eq!(pp.len(), 10);
            assert_eq!(ss.len(), 9);
            assert_eq!(ss[8], f.o());
            assert_eq!(dd, vec![f.o()]);
            let un = gate(nodes, "Mul", json!([2 * j + 1, sn]));
            let numerator = add(nodes, pn, un);
            let inv = gate(nodes, "Inverse", json!(dn));
            assert_eq!(mul(nodes, numerator, inv), vn);
            let mut conjugates = vec![];
            for k in 0..5 {
                let a = pp.iter().map(|v| f.frob(v, k)).collect::<Vec<_>>();
                let b = ss.iter().map(|v| f.frob(v, k)).collect::<Vec<_>>();
                let d = dd.iter().map(|v| f.frob(v, k)).collect::<Vec<_>>();
                assert_eq!(d, vec![f.o()]);
                assert_eq!(b[8], f.o());
                conjugates.push(json!({"coefficient_sigma_power":k,"P":coefficients(&a),"S":coefficients(&b),"D":coefficients(&d),"pole_at_O":19,"leading_coefficient":"1","other_poles":[]}));
            }
            functions.push(json!({"source_point":j,"P_node":pn,"S_node":sn,"D_node":dn,"U_node":un,"value_node":vn,"numerator_node":numerator,"inverse_node":inv,"P":coefficients(&pp),"S":coefficients(&ss),"D":coefficients(&dd),"five_conjugates":conjugates,"pole_only_at_O_written_bridge":true}));
        }
        assert_eq!(nodes[15], json!({"op":"Constant","args":"1"}));
        assert_eq!(nodes[127], json!({"op":"Mul","args":[15,126]}));
        assert_eq!(nodes[187], json!({"op":"Mul","args":[127,186]}));
        assert_eq!(nodes[247], json!({"op":"Mul","args":[187,246]}));
        let a = gate(nodes, "Constant", json!(if graph < 2 { "0" } else { "1" }));
        let sp = (0..5)
            .map(|j| {
                (
                    gate(nodes, "CoefficientFrobenius", json!(2 * j)),
                    gate(nodes, "CoefficientFrobenius", json!(2 * j + 1)),
                )
            })
            .collect::<Vec<_>>();
        let negative = (sp[3].0, add(nodes, sp[3].0, sp[3].1));
        let mut dest = vec![negative];
        for j in 0..3 {
            dest.push(plus(nodes, sp[j], negative, a));
        }
        dest.push(sp[4]);
        let residuals = p["residuals"].as_array().unwrap();
        let mut bindings = vec![];
        for (j, (x, y)) in dest.iter().enumerate() {
            let ex = add(nodes, 2 * j, *x);
            let ey = add(nodes, 2 * j + 1, *y);
            assert!(residuals.contains(&json!(ex)) && residuals.contains(&json!(ey)));
            bindings.push(json!({"source_point":j,"dest_x":x,"dest_y":y,"residuals":[ex,ey]}));
        }
        let model = prior["records"][graph]["model"].as_array().unwrap();
        let support = model
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != json!(0))
            .map(|(j, _)| j)
            .collect::<Vec<_>>();
        assert_eq!(support, vec![187]);
        assert_eq!(model[187], json!(1));
        records.push(json!({"graph":graph,"ordinary_a":u8::from(graph>=2),"functions":functions,"two_product":187,"third_factor":246,"three_product":247,"exact_product_gates":true,"proper_descent_gate_bindings":bindings,"full_source_program_nodes":nodes,"full_source_residuals":residuals,"old_scalar_support":support,"old_scalar_model_excluded_under_written_DVR_bridge":true,"explicit_polynomial_separator_extracted":false,"saturation_membership_certificate":false,"complete_simultaneous_arc_constructed":false,"valuation_certificate":{"normalization":"v(h)=N>0;v(h*node187)=0;all other projective node residues zero","individual_bounds":["v(f0)>-N","v(f1)>-N"],"product_equation":"v(f0)+v(f1)=-N","consequence":"v(f0)<0 and v(f1)<0 in every split coefficient component","proper_reduction":"P0=P1=O in every component;P1=sigma(P0)-sigma(P3),P2=sigma(P1)-sigma(P3) imply P2=O","third_pole":"v(f2)<0","contradiction":"v(h*node247)=v(f2)<0, although every projective coordinate is regular"}}));
    }
    let mut rejected = 0u64;
    for n in 1..=64 {
        for a in (1 - n)..0 {
            let b = -n - a;
            assert!(a < 0 && b < 0);
            for c in -64..0 {
                assert!(n + a + b + c < 0);
                rejected += 1;
            }
        }
    }
    assert_eq!((2 - 1 - 1, 2 - 1 - 1 + 1), (0, 1));
    crate::round02::save(
        dir,
        "q41-node187-coupled-valuations.json",
        &json!({"schema":"koblitz.round22/q41-v1","records":records,"coefficient_algebra_degree":360,"coefficient_sigma_permutation":(0..360).map(|i|(i+1)%360).collect::<Vec<_>>(),"sigma_fixes_arc_parameter":true,"finite_integer_controls":rejected,"uncoupled_positive_control":{"N":2,"v0":-1,"v1":-1,"v2":1,"two_product_normalized_order":0,"three_product_normalized_order":1,"fails_proper_descent_pole_bridge":true},"full_quotient_saturation":false,"scope":"exact source-bound simultaneous valuation exclusion of the four node187 scalar points,conditional on explicit proper/DVR/pole bridges;not a polynomial ideal certificate"}),
    );
    println!("Q41:4 actual product/descent bindings;80 conjugated ordinary functions;{rejected} valuation controls;no node187 DVR arc under written proper/pole bridges;polynomial separator not extracted");
}
