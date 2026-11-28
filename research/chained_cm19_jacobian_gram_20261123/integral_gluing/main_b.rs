#[path = "arith_b.rs"]
mod arith;
#[path = "elliptic_b.rs"]
mod elliptic;
#[path = "field_b.rs"]
mod field;

use arith::*;
use elliptic::{EPoint, Models};
use field::Field;
use std::collections::BTreeSet;

const RAW: &str =
    include_str!("../../chained_cm19_hermitian_20261123/first-C-native/similitudes.stdout");
const TARGETS: [EPoint; 11] = [
    None,
    Some((1, 3)),
    Some((1, 4)),
    Some((2, 2)),
    Some((2, 5)),
    Some((3, 2)),
    Some((3, 5)),
    Some((4, 1)),
    Some((4, 6)),
    Some((5, 1)),
    Some((5, 6)),
];
type Triple = [EPoint; 3];

fn integral_lattice(t: Z) -> Matrix {
    assert_eq!((2 + t.norm()) % 3, 0);
    [
        [Z((2 + t.norm()) / 3, 0), Z(-1, 0), t.conj()],
        [Z(-1, 0), Z(2, 0), ZERO],
        [t, ZERO, Z(3, 0)],
    ]
}

fn action(m: &Models<'_>, coefficients: [[i128; 3]; 3], p: Triple) -> Triple {
    coefficients.map(|row| {
        row.into_iter().zip(p).fold(None, |s, (a, q)| {
            m.eadd(s, m.emul(q, a.rem_euclid(11) as usize))
        })
    })
}

fn triple_mul(m: &Models<'_>, p: Triple, n: usize) -> Triple {
    p.map(|q| m.emul(q, n))
}

fn selected_actual_points(id: usize, m0: Matrix) {
    let f = Field::new(7, 1, vec![0]);
    let m = Models { f: &f };
    let coefficients = m0.map(|row| row.map(|z| (5 * z.eval11(3)).rem_euclid(11)));
    let mut images = BTreeSet::new();
    let mut bad_images = BTreeSet::new();
    let mut rank_drop = coefficients;
    rank_drop[0] = [0; 3];
    for &p in &TARGETS {
        for &q in &TARGETS {
            for &r in &TARGETS {
                let value = [p, q, r];
                assert!(
                    images.insert(action(&m, coefficients, value)),
                    "actual F-coordinate collision"
                );
                bad_images.insert(action(&m, rank_drop, value));
            }
        }
    }
    assert_eq!(images.len(), 1331);
    assert_eq!(bad_images.len(), 121);
    for &p in &TARGETS {
        assert!(m.evalid(p));
        assert_eq!(m.emul(p, 10), m.eneg(p));
        let original = [m.emul(p, 2), m.eneg(p), None];
        let image = action(&m, coefficients, original);
        for n in 0..11 {
            assert_eq!(
                action(&m, coefficients, triple_mul(&m, original, n)),
                triple_mul(&m, image, n)
            );
        }
        println!(
            "ORIGINAL_SUBGROUP\tid={}\tP={:?}\tF_input={:?}\tF_image={:?}\tPASS",
            id, p, original, image
        );
    }
    println!("ACTUAL_F_COORDINATES\tid={}\tmatrix_mod11={:?}\tpoints=1331\tdistinct_images=1331\tactual_rank_drop_mutant_images=121\tsubgroup_scalar_laws=121\tPASS", id, coefficients);
}

fn main() {
    println!("PROFILE\tactual_C7_integral3_gluing_and_frozen_H_map_filter\tunsigned_not_benchmark");
    assert_eq!(W.mul(W), W.sub(Z(5, 0)));
    let mut isotropic = Vec::new();
    let mut wrong_form = Vec::new();
    let mut line = 0;
    for a in 0..3 {
        for b in 0..3 {
            let t = Z(if a == 2 { -1 } else { a }, if b == 2 { -1 } else { b });
            let residue = (2 + t.norm()).rem_euclid(3);
            println!(
                "PROJECTIVE_LINE\tid={}\tt={:?}\tform_residue={}\tisotropic={}",
                line,
                t,
                residue,
                residue == 0
            );
            if residue == 0 {
                isotropic.push(t);
            }
            if (1 + t.norm()).rem_euclid(3) == 0 {
                wrong_form.push(t);
            }
            line += 1;
        }
    }
    println!(
        "PROJECTIVE_LINE\tid={}\tvertical\tform_residue=1\tisotropic=false",
        line
    );
    assert_eq!(line + 1, 10);
    assert_eq!(isotropic.len(), 4);
    assert_eq!(wrong_form.len(), 4);
    assert!(wrong_form.iter().all(|t| (2 + t.norm()).rem_euclid(3) != 0));
    println!("FORM_MUTANT\tchanged_diagonal_count_still4\tactual_G_integrality_rejects4\tpristine_restored\tPASS");
    let u: Matrix = [[ZERO, Z(-1, 0), ONE], [ONE, ZERO, ZERO], [ZERO, ONE, ZERO]];
    assert_eq!(determinant(u), ONE);
    let t = ONE.add(W);
    let mut product = 0;
    let mut indecomposable_candidates = 0;
    for parameter in isotropic {
        let k = integral_lattice(parameter);
        assert_eq!(determinant(k), ONE);
        assert!(k[0][0].0 > 0 && 2 * k[0][0].0 - 1 > 0);
        let norm_one = k[0][0] == ONE;
        if norm_one {
            product += 1;
            assert_eq!(parameter.norm(), 1);
        } else {
            indecomposable_candidates += 1;
            assert_eq!(parameter.norm(), 7);
            let basis = if parameter == t {
                u
            } else {
                assert_eq!(parameter, Z(-1, -1));
                mmul(D, u)
            };
            assert_eq!(gram(basis, k), H);
        }
        println!(
            "OVERLATTICE\tt={:?}\tK={:?}\tnorm_one_product={}\texplicit_H_isometry={}\tPASS",
            parameter,
            flat(k),
            norm_one,
            !norm_one
        );
    }
    assert_eq!((product, indecomposable_candidates), (2, 2));
    let tp: Matrix = [
        [ONE, ONE, ZERO],
        [ZERO.sub(t), ZERO, ONE],
        [Z(3, 0).sub(t), ZERO, ONE],
    ];
    let tm = mmul(tp, D);
    assert_eq!(determinant(tp), Z(3, 0));
    assert_eq!(determinant(tm), Z(-3, 0));
    assert_eq!(gram(tp, H), G);
    assert_eq!(gram(tm, H), G);
    assert_eq!(mmul(adjugate(tp), tp), scale(identity(), 3));
    assert_eq!(mmul(tp, adjugate(tp)), scale(identity(), 3));
    let mut changed = tp;
    changed[0][0] = changed[0][0].add(ONE);
    assert_ne!(gram(changed, H), G);
    assert_eq!(gram(tp, H), G);
    println!("BRIDGE\tTplus={:?}\tTminus={:?}\tboth_full_Gram_identities\tactual_entry_mutant_rejects\tpristine_restored\tPASS", flat(tp), flat(tm));
    let input = import_maps(RAW);
    assert!(input.iter().any(|m| m.s == scale(identity(), 2)));
    assert_eq!(
        compatible(scale(identity(), 2), tp),
        Some(scale(identity(), 2))
    );
    let mut wrong_s = scale(identity(), 2);
    wrong_s[0][1] = wrong_s[0][1].add(ONE);
    assert_ne!(gram(wrong_s, H), scale(H, 4));
    let l: Matrix = [
        [Z(2, 0), ONE, ZERO],
        [ONE, Z(2, 0), ZERO],
        [ZERO, ZERO, ONE],
    ];
    assert_eq!(mmul(G, l), scale(identity(), 3));
    let mut accepted = 0;
    let mut mixed = 0;
    let mut trivial = 0;
    let mut selected = None;
    for m in input {
        let twin = compatible(m.s, tp);
        println!(
            "COMPATIBILITY\tid={}\tmixed4422={}\tpreserves_both={}",
            m.id,
            m.mixed,
            twin.is_some()
        );
        if let Some(s_minus) = twin {
            accepted += 1;
            if m.mixed {
                mixed += 1;
            } else {
                trivial += 1;
            }
            assert_eq!(gram(s_minus, H), scale(H, 4));
            assert_eq!(determinant(s_minus).norm(), 64);
            assert_eq!(
                s_minus
                    .iter()
                    .flatten()
                    .all(|z| z.0 % 2 == 0 && z.1 % 2 == 0),
                !m.mixed
            );
            let a0 = mmul(adjugate(tp), mmul(m.s, tp));
            assert_eq!(gram(a0, G), scale(G, 36));
            let m0 = mmul(G, mmul(a0, l));
            assert_eq!(determinant(m0), determinant(m.s).mul(Z(729, 0)));
            for root in [3, 9] {
                let coeff = m0.map(|row| row.map(|z| Z((5 * z.eval11(root)).rem_euclid(11), 0)));
                assert_ne!(determinant(coeff).0.rem_euclid(11), 0);
            }
            println!(
                "ACCEPTED\tid={}\tA_numerator_over3={:?}\tminus_H_matrix={:?}\tPASS",
                m.id,
                flat(a0),
                flat(s_minus)
            );
            if m.mixed && selected.is_none() {
                selected = Some((m.id, m0));
            }
        }
    }
    println!("SUMMARY\tprojective_lines=10\tisotropic=4\tnorm_one_product=2\tH_isometric_sign_candidates=2\tfrozen_input_maps=156\tcompatible={}\tmixed4422={}\t2divisible={}\tno_old_enumeration_replay\tPASS", accepted, mixed, trivial);
    if let Some((id, m0)) = selected {
        println!(
            "FIRST_COMPATIBLE_MIXED\tid={}\tF_action_numerator_over9={:?}",
            id,
            flat(m0)
        );
        selected_actual_points(id, m0);
    } else {
        println!("NEGATIVE\tno_compatible_mixed_in_frozen156\tno_selected_mixed_actual_action_run\tactual_gluing_sign_or_different_target_still_needed");
    }
    println!("SCOPE\tmodule_dictionary_and_Jacobian_indecomposability_are_written_dependencies\tactual_gluing_sign_unresolved\tno_full_divisor_engine_or_generic_Lean_or_cost_advantage");
}
