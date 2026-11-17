//! Unique symmetric theta lifts, matched under the written symmetric pullback bridge.
use super::*;
fn negatives(p: &[P]) -> Vec<P> {
    p.iter().map(lf::neg).collect()
}
fn sym_scale(f: &F, c: &BinaryCurve, a: &[P], p: &[P]) -> Option<(E, E)> {
    let na = negatives(a);
    let d = f.mul(
        &theta(f, c, a, &negatives(p))?,
        &theta(f, c, a, &shifted(c, p, &na))?,
    );
    if d.is_zero() {
        return None;
    }
    let scale = f.frob(&f.inv(&d), f.m() - 1);
    assert_eq!(f.mul(&f.sq(&scale), &d), f.o());
    Some((scale, d))
}
pub fn run(dir: &Path) {
    let old = read(&dir.join("g18-factored-odd-kernel-theta-lifts.json"));
    let kernel = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let f = F::new(168, 95);
    assert!(f.irreducible());
    let mut records = vec![];
    let mut lean = String::from("import KoblitzRound18\nnamespace KoblitzRound18Theta\n");
    for (ri, row) in kernel["records"].as_array().unwrap().iter().enumerate() {
        let graph = row["graph"].as_u64().unwrap();
        let c = f.curve(f.e(u64::from(row["trace"] == 1)), f.o());
        let gens = row["kernel_generators"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| point(&f, v))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let probes = old["records"][ri]["accepted_probes"].as_array().unwrap();
        let mut root = f.o();
        for r in row["torsion_bases"].as_array().unwrap() {
            root = f.mul(&root, &decode(&f, &r["primitive_Weil_pairing"]));
        }
        assert_eq!(f.powu(&root, 105), f.o());
        for l in [3, 5, 7] {
            assert_ne!(f.powu(&root, 105 / l), f.o());
        }
        let powers = (0..105).map(|k| f.powu(&root, k)).collect::<Vec<_>>();
        let log = |v: &E| powers.iter().position(|x| x == v).expect("105-th root") as u64;
        let mut operators = vec![];
        let mut scales = vec![];
        let mut accepted = vec![];
        for (j, a) in gens.iter().enumerate() {
            let d = [3u64, 3, 105, 105][j];
            let prior = decode(
                &f,
                &old["records"][ri]["operators"][j]["normalizing_scalar"],
            );
            let mut values = vec![];
            let mut selected = None;
            for probe in probes {
                let p = probe["tuple"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| point(&f, v))
                    .collect::<Vec<_>>();
                let (scale, raw) = sym_scale(&f, &c, a, &p).expect("symmetry probe meets divisor");
                if let Some(s) = &selected {
                    assert_eq!(&scale, s);
                } else {
                    selected = Some(scale.clone());
                }
                let delta = f.mul(&f.sq(&prior), &raw);
                let character = f.mul(&scale, &f.inv(&prior));
                assert_eq!(f.mul(&f.sq(&character), &delta), f.o());
                assert_eq!(f.powu(&character, d), f.o());
                let period = period(&f, &c, a, &p, d).unwrap();
                assert_eq!(f.mul(&f.powu(&scale, d), &period), f.o());
                let sa = a.iter().map(|v| f.frobp(v, 1)).collect::<Vec<_>>();
                let sp = p.iter().map(|v| f.frobp(v, 1)).collect::<Vec<_>>();
                let (s_scale, s_raw) = sym_scale(&f, &c, &sa, &sp).unwrap();
                assert_eq!(s_scale, f.sq(&scale));
                assert_eq!(s_raw, f.sq(&raw));
                values.push(json!({"raw_inversion_discrepancy":lf::ej(&raw),"old_normalized_discrepancy":lf::ej(&delta),"canonical_scalar":lf::ej(&scale),"old_to_canonical_character":lf::ej(&character),"old_character_exponent_mod105":log(&character),"discrepancy_exponent_mod105":log(&delta),"period_scalar":lf::ej(&period),"normalized_period":lf::ej(&f.mul(&f.powu(&scale,d),&period)),"standard_coefficient_Frobenius_scale":lf::ej(&s_scale),"standard_Frobenius_compatibility_checked":true}));
            }
            let k = values[0]["old_character_exponent_mod105"].as_u64().unwrap();
            let dl = values[0]["discrepancy_exponent_mod105"].as_u64().unwrap();
            assert_eq!((2 * k + dl) % 105, 0);
            assert_eq!((d * k) % 105, 0);
            lean.push_str(&format!("theorem actual_symmetric_character_{graph}_{j} : ((2*{k}+{dl}:ZMod 105)=0 ∧ ({d}*{k}:ZMod 105)=0) := by decide +kernel\n"));
            scales.push(selected.unwrap());
            operators.push(json!({"generator":j,"order":d,"checks":values,"symmetric_normalization_unique":true}));
        }
        for probe in probes {
            let p = probe["tuple"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| point(&f, v))
                .collect::<Vec<_>>();
            let mut checks = vec![];
            for i in 0..4 {
                for j in i + 1..4 {
                    let lhs = f.mul(
                        &f.mul(&scales[i], &theta(&f, &c, &gens[i], &p).unwrap()),
                        &f.mul(
                            &scales[j],
                            &theta(&f, &c, &gens[j], &shifted(&c, &p, &gens[i])).unwrap(),
                        ),
                    );
                    let rhs = f.mul(
                        &f.mul(&scales[j], &theta(&f, &c, &gens[j], &p).unwrap()),
                        &f.mul(
                            &scales[i],
                            &theta(&f, &c, &gens[i], &shifted(&c, &p, &gens[j])).unwrap(),
                        ),
                    );
                    assert_eq!(lhs, rhs);
                    checks.push(json!({"pair":[i,j],"lhs":lf::ej(&lhs),"rhs":lf::ej(&rhs)}));
                }
            }
            accepted.push(json!({"commutators":checks}));
        }
        println!("G19:graph{graph},four unique symmetric lifts,12 inversion/period and12 standard Frobenius controls;canonical matching uses written symmetric bundle bridge");
        records.push(json!({"graph":graph,"operators":operators,"primitive_root105":lf::ej(&root),"commutator_checks":accepted,"canonical_matching":"unique symmetric lift under actual symmetric pullback bundle-class bridge;global divisor geometry written","standard_split_product_Frobenius_checked":true,"target_rotation_F2_descent_constructed":false,"243_sections_extracted":false}));
    }
    lean.push_str("end KoblitzRound18Theta\n");
    for g in [0, 2] {
        for j in 0..4 {
            lean.push_str(&format!(
                "#print axioms KoblitzRound18Theta.actual_symmetric_character_{g}_{j}\n"
            ));
        }
    }
    let path = dir.join("KoblitzRound18Theta.lean");
    assert!(!path.exists());
    std::fs::write(path, lean).unwrap();
    crate::round02::save(
        dir,
        "g19-canonical-symmetric-theta-characters.json",
        &json!({"schema":"koblitz.round18/g19-v1","records":records,"field_degree":168,"modulus":f.modulus().to_str_radix(16),"scope":"explicit symmetric lifts and old-to-canonical characters;native field checks and written divisor/pullback bridge;target rotation/F2 section descent separate"}),
    );
}
