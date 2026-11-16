//! Native conformance components for three registered follow-ons.
//! These are not a whole-query benchmark or a completed IC oracle.
pub mod boolean;
pub mod field;
pub mod identity;
pub mod quotient;
pub mod hash {
    pub mod sha256 {
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            use sha2::{Digest, Sha256};
            Sha256::digest(bytes).into()
        }
    }
}
use boolean::{closure, correspondence, intersection, symmetric_s4, CertifiedRow, Element, Poly};
use field::{Curve, Field};
use serde_json::{json, Value};

pub fn write_evidence(name: &str, v: &Value) {
    if let Some(dir) = std::env::var_os("IC_THREE_LEADS_EVIDENCE_DIR") {
        let dir = std::path::PathBuf::from(dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        assert!(!path.exists(), "evidence is write-once");
        std::fs::write(path, serde_json::to_vec_pretty(v).unwrap()).unwrap();
    }
}
/// Resultant and its exact Bezout witness in the original two-equation chain.
pub fn chain_bridge(r: u32, f: Field, g: u32) -> (Element, Vec<Poly>, Vec<Vec<Poly>>) {
    let basis = [1, g];
    let x = boolean::field_variables(f, &basis, 0);
    let y = boolean::field_variables(f, &basis, 2);
    let z = boolean::field_variables(f, &basis, 4);
    let t = Element::constant(f, r);
    let w = Element::vars(f, 6, f.n as usize);
    let one = Element::constant(f, 1);
    let a = x.add(&y).square(f);
    let b = x.mul(&y, f);
    let c = b.square(f).add(&one);
    let d = z.add(&t).square(f);
    let e = z.mul(&t, f);
    let ff = e.square(f).add(&one);
    let u = a.mul(&e, f).add(&b.mul(&d, f));
    let v = a.mul(&ff, f).add(&c.mul(&d, f));
    let result = v
        .square(f)
        .add(&u.mul(&b.mul(&ff, f).add(&c.mul(&e, f)), f));
    let p = d
        .mul(&u, f)
        .mul(&w, f)
        .add(&e.mul(&u, f))
        .add(&d.mul(&v, f));
    let q = a
        .mul(&u, f)
        .mul(&w, f)
        .add(&b.mul(&u, f))
        .add(&a.mul(&v, f));
    let left = boolean::f3(&x, &y, &w, f);
    let right = boolean::f3(&z, &t, &w, f);
    assert_eq!(p.mul(&left, f).add(&q.mul(&right, f)).0, result.0);
    let generators: Vec<_> = left.0.iter().chain(&right.0).cloned().collect();
    let mut certificates = vec![vec![Poly::default(); 2 * f.n as usize]; f.n as usize];
    for (offset, mult) in [(0, &p), (f.n as usize, &q)] {
        for (i, coeff) in mult.0.iter().enumerate() {
            for j in 0..f.n as usize {
                let structure = f.mul(1 << i, 1 << j);
                for (k, row) in certificates.iter_mut().enumerate() {
                    if structure >> k & 1 != 0 {
                        row[j + offset] = row[j + offset].add(coeff);
                    }
                }
            }
        }
    }
    for (row, multipliers) in result.0.iter().zip(&certificates) {
        assert!(CertifiedRow {
            row: row.clone(),
            multipliers: multipliers.clone()
        }
        .replay(&generators));
    }
    (result, generators, certificates)
}
pub fn transfer_control(r: u32, g: u32) -> Value {
    let f = Field {
        n: 5,
        modulus: 0x25,
    };
    let (es, pull) = correspondence(f, g);
    let bases = boolean::product_bases(f, g);
    let sym_es = [
        boolean::field_variables(f, &bases[0], 0),
        boolean::field_variables(f, &bases[1], 2),
        boolean::field_variables(f, &bases[2], 5),
    ];
    let sym = symmetric_s4(&sym_es, r, f);
    assert!(sym.0.iter().all(|p| p.degree() <= 2));
    let direct = symmetric_s4(&es, r, f);
    let (result, chain, chain_certs) = chain_bridge(r, f, g);
    assert_eq!(direct.0, result.0);
    assert_eq!(
        sym.0
            .iter()
            .map(|p| p.substitute(&pull))
            .collect::<Vec<_>>(),
        direct.0
    );
    let space = closure(&sym.0, 9, 2);
    let mut records = vec![];
    let mut pulled_certificates = vec![];
    for cr in space.rows.values() {
        assert!(cr.replay(&sym.0));
        let image = cr.row.substitute(&pull);
        let lifted_coeffs: Vec<_> = cr.multipliers.iter().map(|p| p.substitute(&pull)).collect();
        let mut witness = vec![Poly::default(); chain.len()];
        for (m, base) in lifted_coeffs.iter().zip(&chain_certs) {
            for (dst, src) in witness.iter_mut().zip(base) {
                *dst = dst.add(&m.mul(src));
            }
        }
        let cert = CertifiedRow {
            row: image.clone(),
            multipliers: witness,
        };
        assert!(cert.replay(&chain));
        pulled_certificates.push(cert.clone());
        records.push(json!({"symmetric":cr,"pullback":image,"chain_certificate":cert}));
    }
    let filtered = boolean::low_degree_image(&pulled_certificates, 4);
    assert!(filtered.rows.values().all(|c| c.replay(&chain)));
    let selected: Vec<_> = filtered.rows.values().map(|r| r.row.clone()).collect();
    let c = Curve {
        f,
        a2: 0,
        a4: 0,
        a6: 1,
    };
    let target = c.lift(r).first().copied();
    let (mut algebraic, mut physical, mut subgroup) = (0, 0, 0);
    for bits in 0..64u16 {
        if direct.0.iter().all(|p| !p.eval(bits)) {
            algebraic += 1;
            let xs = [
                ((bits & 1) as u32) ^ if bits & 2 != 0 { g } else { 0 },
                (((bits >> 2) & 1) as u32) ^ if bits & 8 != 0 { g } else { 0 },
                (((bits >> 4) & 1) as u32) ^ if bits & 32 != 0 { g } else { 0 },
            ];
            let mut physical_hit = false;
            let mut subgroup_hit = false;
            for p in c.lift(xs[0]) {
                for q in c.lift(xs[1]) {
                    for s in c.lift(xs[2]) {
                        if Some(c.add(c.add(p, q), s)) == target {
                            physical_hit = true;
                            subgroup_hit |= c.mul(p, 11) == field::Point::Infinity
                                && c.mul(q, 11) == field::Point::Infinity
                                && c.mul(s, 11) == field::Point::Infinity;
                        }
                    }
                }
            }
            physical += u32::from(physical_hit);
            subgroup += u32::from(subgroup_hit);
        }
    }
    let mut chain_solutions = 0;
    for bits in 0..1u16 << 11 {
        if chain.iter().all(|p| !p.eval(bits)) {
            chain_solutions += 1;
            assert!(selected.iter().all(|p| !p.eval(bits)));
        }
    }
    assert!(chain_solutions > 0, "positive chain control is required");
    if g != 2 {
        assert!(
            subgroup > 0,
            "prime-subgroup positive control must have a verified relation"
        );
    }
    let any = &space.rows.values().next().unwrap();
    let mut broken = (*any).clone();
    broken.row = broken.row.add(&Poly::one());
    assert!(!broken.replay(&sym.0));
    json!({"field_degree":5,"modulus":0x25,"curve_a2":0,"curve_b":1,"factor_base_basis":[1,g],"target_x":r,
        "symmetric_variables":9,"direct_x_variables":6,"chain_variables":11,"symmetric_degree2_closure_rank":space.rows.len(),
        "source_max_degree":direct.0.iter().map(Poly::degree).max(),"pullback_max_degree":records.iter().map(|v|serde_json::from_value::<Poly>(v["pullback"].clone()).unwrap().degree()).max(),
        "selected_consequences":selected.len(),"selected_chain_certificates":filtered.rows.values().collect::<Vec<_>>(),"selection_checks_all_linear_combinations":true,"certificate_rows":records,"all_direct_assignments_checked":64,"all_chain_assignments_checked":2048,
        "algebraic_x_assignments":algebraic,"physical_ambient_x_assignments":physical,"physical_subgroup_x_assignments":subgroup,
        "chain_solutions":chain_solutions,"solution_set_preserved":true,"corrupted_certificate_rejected":true,
        "scope":"exact component certificates; not a complete-query comparison","complete_query_cost":null,"speedup":null})
}
pub fn branch_control(r: u32, g: u32) -> Value {
    let f = Field {
        n: 5,
        modulus: 0x25,
    };
    let (es, _) = correspondence(f, g);
    let direct = symmetric_s4(&es, r, f);
    let gens: Vec<Vec<_>> = (0..4)
        .map(|a| direct.0.iter().map(|p| p.specialize(6, 2, a)).collect())
        .collect();
    let spaces: Vec<_> = gens.iter().map(|g| closure(g, 4, 4)).collect();
    let (common_inputs, common_input_rank, reused_spaces) =
        boolean::shared_input_elimination(&gens, 4, 4);
    for (reference, reused) in spaces.iter().zip(&reused_spaces) {
        assert_eq!(reference.rows.len(), reused.rows.len());
        assert!(reference.rows.values().all(|r| reused.contains(&r.row)));
        assert!(reused.rows.values().all(|r| reference.contains(&r.row)));
    }
    let mut shared = spaces[0].clone();
    for other in &spaces[1..] {
        shared = intersection(&shared, other);
    }
    let mut leaves = vec![];
    let mut common_certs = vec![];
    let mut saw_positive = false;
    let mut checked = 0;
    for branch in 0..4 {
        let certs: Vec<_> = shared
            .rows
            .values()
            .map(|row| {
                spaces[branch]
                    .certificate(&row.row)
                    .expect("intersection membership")
            })
            .collect();
        assert!(certs.iter().all(|c| c.replay(&gens[branch])));
        let mut solutions = 0;
        for rest in 0..16u16 {
            let full = branch as u16 | (rest << 2);
            let original = direct.0.iter().all(|p| !p.eval(full));
            let specialized = gens[branch].iter().all(|p| !p.eval(rest));
            assert_eq!(original, specialized);
            checked += 1;
            if original {
                solutions += 1;
                assert!(shared.rows.values().all(|c| !c.row.eval(rest)));
            }
        }
        saw_positive |= solutions > 0;
        assert_eq!(spaces[branch].contains(&Poly::one()), solutions == 0);
        common_certs.push(certs);
        leaves.push(json!({"branch":branch,"rank":spaces[branch].rows.len(),"solutions":solutions,"refuted":solutions==0}));
    }
    assert!(saw_positive);
    if let Some(c) = common_certs.iter().flatten().next() {
        let mut corrupted = c.clone();
        corrupted.row = corrupted.row.add(&Poly::one());
        assert!(!corrupted.replay(&gens[0]));
    }
    json!({"target_x":r,"factor_base_basis":[1,g],"guessed_variables":[0,1],"complete_branch_count":4,"active_variables":4,"degree":4,
        "independent_leaf_ranks":spaces.iter().map(|x|x.rows.len()).collect::<Vec<_>>(),"shared_intersection_rank":shared.rows.len(),"literal_common_input_rows":common_inputs,"shared_input_block_rank":common_input_rank,"shared_input_elimination_matches_independent_closure":true,
        "all_assignments_checked":checked,"per_leaf":leaves,"branch_generators":gens,"per_leaf_shared_membership_certificates":common_certs,
        "all_leaf_outcomes_preserved":true,"equal_rank_used_as_equivalence":false,
        "scope":"exact intersection on a complete small Boolean control; no scaling or total-cost conclusion","complete_branch_set_cost":null,"speedup":null})
}

pub fn resultant_identity() -> identity::IdentityStatement {
    use identity::Expr as E;
    let v = |x: &str| E::var(x);
    let mul = |a, b| E::times(a, b);
    let add = |a, b| E::plus(a, b);
    let sub = |a, b| E::plus(a, E::minus(b));
    let u = sub(mul(v("a"), v("e")), mul(v("b"), v("d")));
    let vv = sub(mul(v("a"), v("f")), mul(v("c"), v("d")));
    let lhs = sub(
        mul(vv.clone(), vv.clone()),
        mul(u.clone(), sub(mul(v("b"), v("f")), mul(v("c"), v("e")))),
    );
    let p = sub(
        add(mul(mul(v("d"), u.clone()), v("z")), mul(v("e"), u.clone())),
        mul(v("d"), vv.clone()),
    );
    let q = sub(
        mul(v("a"), vv),
        add(mul(mul(v("a"), u.clone()), v("z")), mul(v("b"), u)),
    );
    let left = add(
        add(mul(v("a"), mul(v("z"), v("z"))), mul(v("b"), v("z"))),
        v("c"),
    );
    let right = add(
        add(mul(v("d"), mul(v("z"), v("z"))), mul(v("e"), v("z"))),
        v("f"),
    );
    identity::IdentityStatement{name:"Quadratic resultant Bezout witness used by the chain pullback".into(),source:"followons_20261029/src/lib.rs::chain_bridge; resultant identity over Z reduces to characteristic2".into(),variables:["a","b","c","d","e","f","z"].map(str::to_owned).to_vec(),degree_bound:7,lhs,rhs:add(mul(p,left),mul(q,right))}
}
pub fn complementary_identity() -> identity::IdentityStatement {
    use identity::Expr as E;
    let v = |s: &str| E::var(s);
    let product = |vs: Vec<E>| E::product(vs);
    identity::IdentityStatement {
        name: "Complementary Koblitz quotient endomorphism identity".into(),
        source: "phi=s V^3; psi=s F^3; commuting F,V and F V=2 give psi phi=s^2*8; s=23 gives4232"
            .into(),
        variables: ["s", "f", "v"].map(str::to_owned).to_vec(),
        degree_bound: 8,
        lhs: product(vec![
            product(vec![v("s"), v("f"), v("f"), v("f")]),
            product(vec![v("s"), v("v"), v("v"), v("v")]),
        ]),
        rhs: product(vec![
            v("s"),
            v("s"),
            product(vec![v("f"), v("v")]),
            product(vec![v("f"), v("v")]),
            product(vec![v("f"), v("v")]),
        ]),
    }
}
pub fn identity_controls() -> Value {
    let mut records = vec![];
    for statement in [resultant_identity(), complementary_identity()] {
        let cert=identity::issue(statement.clone(),0x20261029,32,"native issuer and separately seeded checker in same development session; no external-runner claim",Some(identity::ProofRef{kind:"lean4".into(),path:"research/ic_algebraic_crossover_20261023/followons_20261029/ThreeLeads.lean".into(),sha256:hex::encode(hash::sha256::sha256(include_bytes!("../ThreeLeads.lean")))})).unwrap();
        let verdict = identity::check_with_extra(&cert, Some((0xd15c71c7, 64))).unwrap();
        assert!(matches!(verdict, identity::Verdict::Accept { .. }));
        let mut wrong = statement;
        wrong.rhs = identity::Expr::plus(wrong.rhs, identity::Expr::Const(1));
        assert!(identity::issue(wrong, 0x20261029, 32, "negative control", None).is_err());
        let checker = match verdict {
            identity::Verdict::Accept {
                points,
                extra_points,
                false_accept_log2,
            } => {
                json!({"verdict":"Accept","points":points,"extra_points":extra_points,"false_accept_log2":false_accept_log2})
            }
            _ => panic!("identity rejected"),
        };
        records.push(json!({"certificate":cert,"checker":checker,"added_one_negative_control_rejected":true}));
    }
    json!({"schema":"ic-three-lead-identity-checks/v1","records":records,"formal_proofs_checked_separately":true})
}
/// Fixed positive control with three prime-subgroup legs, chosen deterministically.
pub fn prime_case() -> (u32, u32) {
    let c = Curve {
        f: Field {
            n: 5,
            modulus: 0x25,
        },
        a2: 0,
        a4: 0,
        a6: 1,
    };
    let p = c
        .points()
        .into_iter()
        .map(|p| c.mul(p, 4))
        .find(|&p| p != field::Point::Infinity)
        .unwrap();
    let field::Point::Affine(g, _) = p else {
        unreachable!()
    };
    let field::Point::Affine(r, _) = c.mul(p, 3) else {
        unreachable!()
    };
    assert!(g > 1);
    assert_eq!(c.mul(p, 11), field::Point::Infinity);
    (r, g)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn degree_filter_recovers_cancellations_between_basis_rows() {
        let x = Poly::var(0);
        let y = Poly::var(1);
        let z = Poly::var(2);
        let xyz = x.mul(&y).mul(&z);
        let gens = [xyz.add(&x), xyz.add(&y)];
        let rows: Vec<_> = gens
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let mut ms = vec![Poly::default(); 2];
                ms[i] = Poly::one();
                CertifiedRow {
                    row: p.clone(),
                    multipliers: ms,
                }
            })
            .collect();
        let selected = boolean::low_degree_image(&rows, 1);
        assert_eq!(selected.rows.len(), 1);
        assert!(selected.contains(&x.add(&y)));
        assert!(selected.rows.values().all(|c| c.replay(&gens)));
    }
    #[test]
    fn equal_rank_does_not_imply_shared_rows() {
        let a = closure(&[Poly::var(0)], 2, 1);
        let b = closure(&[Poly::var(1)], 2, 1);
        assert_eq!(a.rows.len(), b.rows.len());
        assert_eq!(intersection(&a, &b).rows.len(), 0);
    }
    #[test]
    fn certified_transfer_replays_against_the_original_chain() {
        let v = json!({"schema":"ic-three-lead-transfer-conformance/v1","controls":[transfer_control(0,2),transfer_control(1,2),transfer_control(prime_case().0,prime_case().1)]});
        write_evidence("transfer.json", &v);
    }
    #[test]
    fn shared_rows_require_a_certificate_in_every_branch() {
        let v = json!({"schema":"ic-three-lead-sharing-conformance/v1","controls":[branch_control(0,2),branch_control(1,2),branch_control(prime_case().0,prime_case().1)]});
        write_evidence("sharing.json", &v);
    }
    #[test]
    fn independent_velu_and_endomorphism_quotients_agree() {
        let v = json!({"schema":"ic-three-lead-quotient-conformance/v1","small_controls":[quotient::small_control(5,0x25,11,4,2),quotient::small_control(6,0x43,7,8,3),quotient::small_control(7,0x83,29,4,2)],"original_e0":quotient::original_e0_control()});
        write_evidence("quotient.json", &v);
    }
    #[test]
    fn integer_identities_and_corrupted_controls_are_checked() {
        write_evidence("identities.json", &identity_controls());
    }
}
