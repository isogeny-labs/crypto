#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod explore {
        use super::*;

        const FIXTURES: [CurveSpec; 2] = [
            CurveSpec { slug: "h115-control-fp31", p: 31, a: 1, b: 19, order: 37, trace: -5, degree: 4 },
            CurveSpec { slug: "h115-control-fp47", p: 47, a: 1, b: 38, order: 61, trace: -13, degree: 3 },
        ];

        fn write_moments<W: Write>(writer: &mut W, value: Moments) -> Result<(), String> {
            for item in value.x.into_iter().chain(value.y) {
                write!(writer, "\t{item}").map_err(|error| error.to_string())?;
            }
            writeln!(writer).map_err(|error| error.to_string())
        }

        pub fn generate(moment_path: &Path, label_path: &Path) -> Result<(), String> {
            let mut moments = BufWriter::new(File::create(moment_path).map_err(|error| error.to_string())?);
            let mut labels = BufWriter::new(File::create(label_path).map_err(|error| error.to_string())?);
            writeln!(moments, "h116-invariant-moments/v1").map_err(|error| error.to_string())?;
            writeln!(labels, "h116-invariant-labels/v1").map_err(|error| error.to_string())?;
            let mut records = 0_u64;
            let mut mismatches = 0_u64;
            let mut repeat_mismatches = 0_u64;
            for spec in FIXTURES {
                let modulus = first_irreducible(spec.p, spec.degree)?;
                let field = Field::new(spec.p, spec.degree, &modulus)?;
                let curve = Curve {
                    field: &field,
                    a: field.from_base(spec.a),
                    b: field.from_base(spec.b),
                };
                let setup = direct_setup(&field, &curve, spec)?;
                for label in 1_u64..=3 {
                    let generator = curve.scalar_mul(label, setup.base_generator)?;
                    let reference_section = section(&curve, spec, generator)?;
                    for scalar in 1..spec.order {
                        let target = curve.scalar_mul(scalar, generator)?;
                        let (x, y) = point_base_coordinates(&field, target)?;
                        let digest = target_digest(spec, label, x, y);
                        writeln!(labels, "{}\t{}\t{}\t{}", spec.slug, label, digest, scalar)
                            .map_err(|error| error.to_string())?;
                        for lambda in [1_u64, 2] {
                            let (model_a, model_b, tx, ty, rx) =
                                mapped_base_inputs(spec, target, generator, &field, lambda)?;
                            let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
                            let (candidate, _) = jet.online(tx, ty, rx)?;
                            let (candidate_repeat, _) = jet.online(tx, ty, rx)?;
                            let target_direct = absolute_direct_moments(
                                &curve,
                                section(&curve, spec, target)?,
                                setup.kernel_generator,
                                spec.order,
                                lambda,
                            )?;
                            let reference_direct = absolute_direct_moments(
                                &curve,
                                reference_section,
                                setup.kernel_generator,
                                spec.order,
                                lambda,
                            )?;
                            let direct = direct_relative(target_direct, reference_direct, spec.p);
                            mismatches += u64::from(candidate != direct);
                            repeat_mismatches += u64::from(candidate != candidate_repeat);
                            write!(
                                moments,
                                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                                spec.slug,
                                spec.p,
                                spec.a,
                                spec.b,
                                spec.order,
                                spec.trace,
                                spec.degree,
                                label,
                                digest,
                                lambda,
                                model_a,
                                model_b,
                            )
                            .map_err(|error| error.to_string())?;
                            write_moments(&mut moments, candidate)?;
                            records += 1;
                        }
                    }
                }
            }
            moments.flush().map_err(|error| error.to_string())?;
            labels.flush().map_err(|error| error.to_string())?;
            println!(
                "invariant moment producer: records={records} candidate_direct_mismatches={mismatches} repeat_mismatches={repeat_mismatches}"
            );
            if records != 576 || mismatches != 0 || repeat_mismatches != 0 {
                return Err(format!(
                    "identity gate failed: records={records} candidate_direct_mismatches={mismatches} repeat_mismatches={repeat_mismatches}"
                ));
            }
            Ok(())
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(FIXTURES.len(), 2);
            assert_eq!(FIXTURES[0].order - 1 + FIXTURES[1].order - 1, 96);
            assert!(FIXTURES.iter().all(|spec| spec.p > 19));
            println!("h116 invariant moment producer selftest: pass");
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::explore::selftest(),
        [_, flag, moment_path, label_path] if flag == "--generate" =>
            frozen::explore::generate(Path::new(moment_path), Path::new(label_path)),
        _ => Err("usage: h116_invariant_moment_producer --selftest | --generate MOMENTS.tsv LABELS.tsv".to_string()),
    };
    if let Err(error) = result {
        eprintln!("h116 invariant moment producer error: {error}");
        std::process::exit(1);
    }
}
