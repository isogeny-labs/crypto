//! Explicit proper image equations from the actual integral CM retraction.
use super::*;
use num_bigint::BigInt as Z;
use num_traits::Zero;
type ZC = (Z, Z);
fn zmul(a: &ZC, b: &ZC) -> ZC {
    (
        &a.0 * &b.0 - Z::from(2) * &a.1 * &b.1,
        &a.0 * &b.1 + &a.1 * &b.0 - &a.1 * &b.1,
    )
}
fn product(a: &[Vec<ZC>], b: &[Vec<ZC>]) -> Vec<Vec<ZC>> {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| {
                    (0..b.len()).fold((Z::zero(), Z::zero()), |(x, y), k| {
                        let z = zmul(&a[i][k], &b[k][j]);
                        (x + z.0, y + z.1)
                    })
                })
                .collect()
        })
        .collect()
}
fn values_z(a: &[Vec<ZC>]) -> Vec<Vec<Value>> {
    a.iter()
        .map(|r| {
            r.iter()
                .map(|(a, b)| json!([a.to_string(), b.to_string()]))
                .collect()
        })
        .collect()
}
pub fn run(dir: &Path) {
    let raw = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let rr = decode_rows(&raw);
    let rows = rr
        .iter()
        .flatten()
        .map(|r| {
            r.iter()
                .map(|(a, b)| (Z::from(*a), Z::from(*b)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let d = read(&dir.join("g28-integral-orbit-retraction.json"));
    let l = d["CM_recovery_coefficients"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|c| {
                    (
                        c[0].as_str().unwrap().parse::<Z>().unwrap(),
                        c[1].as_str().unwrap().parse::<Z>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let e = product(&rows, &l);
    assert_eq!(product(&e, &e), e);
    assert_eq!(product(&e, &rows), rows);
    assert_eq!(product(&l, &e), l);
    assert!(e
        .iter()
        .flatten()
        .all(|(a, b)| a.bits() < 160 && b.bits() < 160));
    let mut orbit = vec![];
    for shift in 0..5 {
        let tau = |i: usize| 5 * (i / 5) + (i % 5 + shift) % 5;
        let p = (0..20)
            .map(|i| {
                (0..20)
                    .map(|j| e[tau(i)][tau(j)].clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(product(&p, &p), p);
        assert_eq!(product(&p, &rows), rows);
        orbit.push(values_z(&p));
    }
    let literal = format!(
        "![{}]",
        e.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(|(a, b)| format!("({a},{b})"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
    let lean=format!("import KoblitzRound20Retraction\nnamespace KoblitzRound20Projector\nopen scoped BigOperators\nopen KoblitzRound19OrbitCover\ndef projector : Fin 20→Fin 20→CM := {literal}\ntheorem actual_source_binding : ∀i j:Fin 20,projector i j=∑k:Fin 4,cmul (KoblitzRound20Retraction.row i k) (KoblitzRound20Retraction.recovery k j) := by decide +kernel\nend KoblitzRound20Projector\n#print axioms KoblitzRound20Projector.actual_source_binding\n");
    save_lean(dir, "KoblitzRound20Projector.lean", lean);
    crate::round02::save(
        dir,
        "g30-proper-orbit-image-equations.json",
        &json!({"schema":"koblitz.round20/g30-v1","projector":values_z(&e),"five_target_rotation_conjugates":orbit,"global_group_equations":"Q_i=sum_j projector[i,j](Q_j);each CM(a,b) means[a]+[b]w","ordinary_group_equations_per_projector":20,"Galois_stable_group_equations":100,"exact_idempotence_and_source_recovery":true,"closed_image_equality_under_written_functor_bridge":true,"F2_descent_under_existing_split_rotation_bridge":true,"companion_map":"identity","small_projective_polynomial_ideal_constructed":false,"final_105_19_quotient_image_constructed":false,"old_affine_quotient_h_saturation_identified":false,"scope":"explicit global proper source-image equations in a product of elliptic curves;not the final quotient's homogeneous ideal"}),
    );
    println!("G30:20 exact proper source-image group equations;100 in the Galois-stable5-orbit;all projectors idempotent and fix the actual source row map");
}
