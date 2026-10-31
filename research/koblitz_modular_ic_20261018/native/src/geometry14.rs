//! Actual polarization pullbacks and finite character descent, not section coordinates.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
type V = [usize; 2];
fn mv(m: [[usize; 2]; 2], v: V) -> V {
    [
        (m[0][0] * v[0] + m[0][1] * v[1]) % 19,
        (m[1][0] * v[0] + m[1][1] * v[1]) % 19,
    ]
}
fn plus(a: V, b: V) -> V {
    [(a[0] + b[0]) % 19, (a[1] + b[1]) % 19]
}
fn dot(a: V, b: V) -> usize {
    (a[0] * b[0] + a[1] * b[1]) % 19
}
pub fn run(dir: &Path) {
    let graphs = read(&dir.join("q5-mixed-translation-atlases.json"));
    let forms = read(&dir.join("gl2b-integral-polarizations.json"));
    let mut records = vec![];
    for (gi, g) in graphs["records"].as_array().unwrap().iter().enumerate() {
        let hrow = forms["candidates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| {
                r["congruence_coset"] == g["coset"]
                    && r["coefficients"] == g["coefficients"]
                    && r["target_trace"] == g["target_trace"]
            })
            .expect("actual H binding");
        let h: M = serde_json::from_value(hrow["hermitian_matrix"].clone()).unwrap();
        assert_eq!(h, ct(&h));
        assert_eq!(det(&h), O::n(19, 0));
        let mut pull = vec![vec![O::n(0, 0); 5]; 5];
        for i in 0..4 {
            for j in 0..4 {
                pull[i][j] = h[i][j].mul(O::n(6, 0));
            }
        }
        pull[4][4] = O::n(114, 0);
        assert_eq!(pull, ct(&pull));
        assert_eq!(det(&pull), O::n(6i64.pow(5) * 361, 0));
        let coords: Vec<V> = g["kernel"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| serde_json::from_value(r["coefficients"].clone()).unwrap())
            .collect();
        assert_eq!(coords.len(), 361);
        let index: BTreeMap<V, usize> = coords.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        assert_eq!(index.len(), 361);
        let perm: Vec<usize> =
            serde_json::from_value(g["kernel_Frobenius_permutation"].clone()).unwrap();
        assert_eq!(perm.iter().copied().collect::<BTreeSet<_>>().len(), 361);
        let images: Vec<V> = perm.iter().map(|i| coords[*i]).collect();
        let a = images[index[&[1, 0]]];
        let b = images[index[&[0, 1]]];
        let m = [[a[0], b[0]], [a[1], b[1]]];
        for (i, v) in coords.iter().enumerate() {
            assert_eq!(mv(m, *v), images[i]);
        }
        for (i, v) in coords.iter().enumerate() {
            for (j, w) in coords.iter().enumerate() {
                assert_eq!(images[index[&plus(*v, *w)]], plus(images[i], images[j]));
            }
        }
        let d = (m[0][0] * m[1][1] + 19 * 19 - m[0][1] * m[1][0]) % 19;
        let inv = (1..19).find(|i| i * d % 19 == 1).unwrap();
        let dual = [
            [(m[1][1] * inv) % 19, ((19 - m[1][0]) * inv) % 19],
            [((19 - m[0][1]) * inv) % 19, (m[0][0] * inv) % 19],
        ];
        for v in &coords {
            for w in &coords {
                assert_eq!(dot(mv(dual, *v), mv(m, *w)), dot(*v, *w));
            }
        }
        let character = dual.map(|r| r.map(|v| 2 * v % 19));
        for v in &coords {
            for w in &coords {
                assert_eq!(dot(mv(character, *v), mv(m, *w)), 2 * dot(*v, *w) % 19);
            }
        }
        let mut seen = BTreeSet::new();
        let mut orbits = vec![];
        for v in &coords {
            if seen.contains(v) {
                continue;
            }
            let mut orbit = vec![];
            let mut w = *v;
            loop {
                assert!(seen.insert(w));
                orbit.push(w);
                w = mv(character, w);
                if w == *v {
                    break;
                }
                assert!(orbit.len() <= 360);
            }
            assert_eq!(360 % orbit.len(), 0);
            orbits.push(orbit);
        }
        assert_eq!(seen.len(), 361);
        println!("G14:graph{gi},actual kernel action{m:?},{} dual character orbits,6lambda pullback determinant{}",orbits.len(),det(&pull).a);
        records.push(json!({"graph":gi,"target_trace":g["target_trace"],"coset":g["coset"],"coefficients":g["coefficients"],"H":h,"pullback_6lambda_matrix":pull,"pullback_Euler_characteristic":det(&pull).a,"kernel_Frobenius_matrix":m,"F19_linear_dual_matrix":dual,"Cartier_character_Frobenius_matrix":character,"character_cyclotomic_multiplier":2,"character_orbits":orbits,"actual_kernel_points_checked":361,"addition_checks":361*361,"linear_dual_pairing_checks":361*361,"Cartier_pairing_checks":361*361,"character_block_dimension_under_written_bridge":7776,"section_semilinear_block_matrices_computed":false}));
    }
    round02::save(
        dir,
        "g14-canonical-polarized-target.json",
        &json!({"schema":"koblitz.round14/g14-v1","records":records,"canonical_bundle":"M=(id_A,lambda_A)^*P represents2lambda_A;M^3 represents6lambda_A","dimension":5,"sections_on_A":6usize.pow(5),"sections_on_B":6usize.pow(5)*361,"ambient_projective_dimension":7775,"Hilbert_polynomial":"7776*t^5","projective_degree":120*7776,"actual_global_sections_computed":false,"closed_projective_equations_computed":false,"full_circuit_h_saturation_computed":false,"bridge":"Poincare bundle,ample vanishing and Lefschetz very ampleness;written geometry,not a full Lean scheme proof"}),
    );
}
