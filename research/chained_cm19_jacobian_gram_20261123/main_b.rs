#[path = "covers_b.rs"]
mod covers;
#[path = "field_b.rs"]
mod field;

use covers::{EPoint, Models, P};
use field::Field;

const TARGETS: [EPoint; 11] = [None, Some((1, 3)), Some((1, 4)), Some((2, 2)), Some((2, 5)), Some((3, 2)), Some((3, 5)), Some((4, 1)), Some((4, 6)), Some((5, 1)), Some((5, 6))];
const GRAM: [[i32; 3]; 3] = [[2, -1, 0], [-1, 2, 0], [0, 0, 3]];

fn scalar(m: &Models<'_>, p: EPoint, a: i32) -> EPoint {
    let out = m.emul(p, a.unsigned_abs() as usize);
    if a < 0 { m.eneg(out) } else { out }
}

fn field_controls(f: &Field) {
    for a in 0..f.q {
        assert_eq!(f.pow(a, f.q), a);
        assert_eq!(f.add(a, f.neg(a)), 0);
        if a != 0 { assert_eq!(f.div(a, a), 1); }
    }
    println!("ARITHMETIC\tq={}\tall_elements={}\tno_old_triples_replay\tPASS", f.q, f.q);
}

fn main() {
    println!("ACTUAL_JACOBIAN_COVER_GRAM\tC7/E7\tF49_squares_F343_cubes\tunsigned_development_not_benchmark");
    let f49 = Field::new(7, 2, vec![3, 0]);
    let f343 = Field::new(7, 3, vec![6, 5, 0]);
    field_controls(&f49);
    field_controls(&f343);
    let mut all = [[[None; 11]; 3]; 3];
    let mut raw_zero_cross = Vec::new();
    let mut omitted_zero_offset_positive = 0;
    let mut fibre_count = 0;
    let mut weighted_points = 0;
    let mut rational_fibres = 0;
    let mut extension_fibres = 0;
    for j in 0..3 {
        let m = Models { f: if j == 2 { &f343 } else { &f49 } };
        let base = m.fibre(j, None);
        let offsets = [0, 1, 2].map(|i| m.push_sum(i, &base));
        println!("BASE_FIBRE\tj={}\tq={}\tfibre={:?}\tpush_offsets={:?}", j, m.f.q, base, offsets);
        for (k, &p) in TARGETS.iter().enumerate() {
            let fibre = m.fibre(j, p);
            fibre_count += 1;
            weighted_points += fibre.iter().map(|x| x.1).sum::<usize>();
            let rational = fibre.iter().all(|(q, _)| q.iter().all(|&x| x < 7));
            if rational { rational_fibres += 1; } else { extension_fibres += 1; }
            println!("GEOMETRIC_FIBRE\tj={}\ttarget={:?}\tfibre={:?}\trational={}\tPASS", j, p, fibre, rational);
            for (i, row) in all.iter_mut().enumerate() {
                let raw = m.push_sum(i, &fibre);
                let image = m.eadd(raw, m.eneg(offsets[i]));
                assert!(image.is_none_or(|(x, y)| x < 7 && y < 7));
                assert_eq!(image, scalar(&m, p, GRAM[i][j]), "actual fibre push/pull i={i} j={j} target={p:?}");
                row[j][k] = image;
                if i == 2 && j == 0 { raw_zero_cross.push((raw, image)); }
                if i == 0 && j == 0 && offsets[i].is_none() { assert_eq!(raw, image); omitted_zero_offset_positive += 1; }
                println!("PUSH_PULL\ti={}\tj={}\ttarget={:?}\traw={:?}\toffset={:?}\timage={:?}\tPASS", i, j, p, raw, offsets[i], image);
            }
        }
    }
    let m = Models { f: &f49 };
    let mut group_laws = 0;
    for (i, row) in all.iter().enumerate() {
        for (j, images) in row.iter().enumerate() {
            for (a, &p) in TARGETS.iter().enumerate() {
                for (b, &q) in TARGETS.iter().enumerate() {
                    let sum = m.eadd(p, q);
                    let s = TARGETS.iter().position(|&v| v == sum).expect("qualified E7 fixture closed");
                    assert_eq!(images[s], m.eadd(images[a], images[b]), "map group law {i},{j}");
                    group_laws += 1;
                }
            }
        }
    }
    assert_eq!(raw_zero_cross.len(), 11);
    let missing_basepoint_rejections = raw_zero_cross.iter().filter(|(raw, image)| raw != image).count();
    assert_eq!(missing_basepoint_rejections, 11);
    assert_eq!(omitted_zero_offset_positive, 11);
    println!("BASEPOINT_MUTANT\tactual_i2_j0_missing_correction_rejects={}\ti0_j0_zero_offset_positive={}\tpristine_restored\tPASS", missing_basepoint_rejections, omitted_zero_offset_positive);
    let m3 = Models { f: &f343 };
    let ramified = m3.fibre(2, Some((1, 3)));
    assert_eq!(ramified.len(), 1);
    assert_eq!(ramified[0].1, 3);
    assert_ne!(ramified.iter().map(|_| 1_usize).sum::<usize>(), 3);
    let wrong_sum = m3.forward(2, ramified[0].0);
    assert_ne!(wrong_sum, m3.push_sum(2, &ramified));
    let positive_infinity = m3.apply(P, [0, 0, 1]);
    let negative_infinity = m3.apply(P, [0, 1, 0]);
    assert_eq!(m3.forward(2, positive_infinity), None);
    assert_eq!(m3.forward(2, negative_infinity), Some((1, 3)));
    assert_ne!(Some((1, 3)), m3.forward(2, positive_infinity));
    println!("RAMIFICATION_INFINITY_MUTANTS\tmissing_multiplicity_fails_degree_and_actual_push\tswapped_infinity_fails_actual_image\tpristine_restored\tPASS");
    println!("SUMMARY\tfibres={}\tweighted_geometric_points={}\trational_fibres={}\textension_fibres={}\tactual_maps=9\timages=99\taddition_laws={}\tgram={:?}\tPASS", fibre_count, weighted_points, rational_fibres, extension_fibres, group_laws, GRAM);
    println!("SCOPE\tactual_geometric_fibre_Pic0_push_pull_on_all_E7(F7)\tglobal_Jacobian_degree9_is_written_argument\tno_full_genus3_group_engine\tno_CM19_S_map\tno_cost_advantage");
}
