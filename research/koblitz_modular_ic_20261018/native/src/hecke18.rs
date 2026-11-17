//! Actual CM residual Hecke systems: primitive diamonds do not separate the rational11 lines.
use crate::round04::read;
use num_traits::ToPrimitive;
use serde_json::json;
use std::{collections::BTreeMap, path::Path};
fn pw(mut a: i64, mut k: u64) -> i64 {
    let mut b = 1;
    while k > 0 {
        if k % 2 == 1 {
            b = b * a % 11;
        }
        a = a * a % 11;
        k /= 2;
    }
    b
}
fn eps(p: i64) -> i64 {
    if [1, 2, 4].contains(&(p % 7)) {
        1
    } else {
        -1
    }
}
pub fn run(dir: &Path) {
    let cm = read(&dir.join("cm17-explicit-level77-incidence.json"));
    assert_eq!(cm["finite_field_records"][4]["field_degree"], 5);
    let mut systems = vec![];
    let mut grouped: BTreeMap<(i64, i64), Vec<(u64, i64, usize)>> = BTreeMap::new();
    for k in [1u64, 3, 7, 9] {
        let z = pw(2, k);
        for alpha in [5i64, 7] {
            let roots = [
                alpha * pw(z, 4) % 11,
                (1 - alpha).rem_euclid(11) * pw(z, 7) % 11,
            ];
            for (sign, root) in roots.iter().enumerate() {
                grouped
                    .entry((z, *root))
                    .or_default()
                    .push((k, alpha, sign));
            }
            systems.push(json!({"eta_exponent":k,"alpha_residue":alpha,"diamond2":z,"E1_Frobenius":roots[0],"E0_Frobenius":roots[1],"Hecke_T2":(roots[0]+roots[1])%11,"determinant":2*z%11}));
        }
    }
    let pplus = crate::cmray15::an(5, 1);
    let pminus = crate::cmray15::an(5, -1);
    let eval = |p: &Vec<num_bigint::BigInt>, x: i64| {
        p.iter()
            .rev()
            .fold(0i64, |v, c| (v * x + c.to_i64().unwrap()).rem_euclid(11))
    };
    for (name, p) in [("E1_Frobenius", &pplus), ("E0_Frobenius", &pminus)] {
        let observed = systems
            .iter()
            .map(|r| r[name].as_i64().unwrap())
            .collect::<std::collections::BTreeSet<_>>();
        let actual = (0..11)
            .filter(|x| eval(p, *x) == 0)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(observed, actual);
        assert_eq!(observed.len(), 8);
    }
    let collisions = grouped
        .iter()
        .filter(|(_, v)| v.iter().any(|x| x.2 == 0) && v.iter().any(|x| x.2 == 1))
        .map(|((d, f), v)| json!({"diamond2":d,"Frobenius":f,"labels":v}))
        .collect::<Vec<_>>();
    assert_eq!(collisions.len(), 2);
    assert!(collisions
        .iter()
        .any(|v| v["Frobenius"] == 1 && v["diamond2"] == 6));
    let units = (1..11)
        .map(|u| {
            assert_eq!(u * pw(u, 9) % 11, 1);
            json!({"u":u,"eta_inverse":pw(u,9),"CM_value":u*pw(u,9)%11})
        })
        .collect::<Vec<_>>();
    let mut pairs = 0;
    for a in 0..11 {
        for b in 0..11 {
            let u = (a + 5 * b) % 11;
            let v = (a + 7 * b) % 11;
            if u != 0 && v != 0 {
                assert_eq!(u * pw(u, 9) % 11, 1);
                assert_eq!(v * pw(v, 9) % 11, 1);
                assert_eq!(u * v % 11, v * u % 11);
                pairs += 1;
            }
        }
    }
    assert_eq!(pairs, 100);
    let mut primes = vec![];
    for p in (2i64..=97)
        .filter(|p| (2..).take_while(|d| d * d <= *p).all(|d| p % d != 0) && *p != 7 && *p != 11)
    {
        let e = eps(p);
        let generators = (-20i64..=20)
            .flat_map(|a| {
                (-20i64..=20).filter_map(move |b| {
                    if a * a + a * b + 2 * b * b == p {
                        Some((a, b))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();
        if e == 1 {
            assert!(!generators.is_empty());
            for (a, b) in &generators {
                let u = (a + 5 * b).rem_euclid(11);
                let v = (a + 7 * b).rem_euclid(11);
                assert_ne!(u, 0);
                assert_ne!(v, 0);
                assert_eq!(u * pw(u, 9) % 11, 1);
                assert_eq!(v * pw(v, 9) % 11, 1);
            }
        } else {
            assert!(generators.is_empty());
        }
        primes.push(json!({"prime":p,"epsilon_minus7":e,"integral_norm_generators":generators,"residual_Hecke_Tp":1+e,"residual_diamond_p":(e*pw(p%11,9)).rem_euclid(11),"shared_both_signs":true}));
    }
    let lean="import KoblitzRound18\nnamespace KoblitzRound18Hecke\ntheorem all_residue_inverse_characters : ∀ u:ZMod 11,u≠0 → u*u^9=1 := by decide +kernel\ntheorem actual_shared_rational_system : (5*(2:ZMod 11)^(4*9)=1 ∧ 7*(2:ZMod 11)^(7*9)=1 ∧ (2:ZMod 11)^9=6) := by decide +kernel\ntheorem no_scalar_separator (p:ZMod 11 → ZMod 11 → ZMod 11) : ¬ (p 1 6=1 ∧ p 1 6=0) := by intro h;have : (1:ZMod 11)=0 := h.1.symm.trans h.2;norm_num at this\nend KoblitzRound18Hecke\n#print axioms KoblitzRound18Hecke.all_residue_inverse_characters\n#print axioms KoblitzRound18Hecke.actual_shared_rational_system\n#print axioms KoblitzRound18Hecke.no_scalar_separator\n";
    let path = dir.join("KoblitzRound18Hecke.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    println!("CM20:two shared joint Frobenius/diamond systems;fixed11 system(F,D)=(1,6);100 unit-pair CM controls and{} unramified primes;unramified residual Hecke separation fails",primes.len());
    crate::round02::save(
        dir,
        "cm20-shared-residual-Hecke-systems.json",
        &json!({"schema":"koblitz.round18/cm20-v1","CM_type":{"alpha_minpoly":[2,-1,1],"residue_roots":[5,7],"primitive_residue_generator":2,"eta_exponents_alpha_beta":[4,7]},"actual_source_Frobenius_polynomials":{"E1":pplus.iter().map(ToString::to_string).collect::<Vec<_>>(),"E0":pminus.iter().map(ToString::to_string).collect::<Vec<_>>()},"both_eight_root_sets_bound_to_actual_sources":true,"systems":systems,"joint_collisions":collisions,"unit_character_controls":units,"two_unit_coordinate_pairs_checked":pairs,"unramified_prime_controls":primes,"shared_fixed11_F_D":[1,6],"written_residual_CM_system":"eta inverse modulo the prime11 gives phi(gamma)=1 and induced residual system1+epsilon_minus7;the two signs share it","arbitrary_integral_maps_excluded":false,"actual_modular_Hom_lattice_computed":false,"scope":"actual CM residue systems and unramified Hecke congruence;no polynomial in shared scalar systems separates the rational11 lines;not nonexistence of geometric maps"}),
    );
}
