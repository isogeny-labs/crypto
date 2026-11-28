#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod oos {
        use super::*;

        const OOS_PROTOCOL: &str = "H116-QUOTIENT-DIGIT-OOS-001";

        fn digest(spec: CurveSpec, label: u64, x: u64, y: u64) -> String {
            sha256_hex(format!("{OOS_PROTOCOL}\0{}\0{label}\0{x}\0{y}", spec.slug).as_bytes())
        }

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
            let mut targets = 0_u64;
            let mut records = 0_u64;
            let mut repeat_mismatches = 0_u64;
            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let base_generator = first_base_point(&curve, spec.p)?;
                if curve.scalar_mul(spec.order, base_generator)? != Point::Infinity {
                    return Err(format!("{} generator order failure", spec.slug));
                }
                for label in 1_u64..=3 {
                    let generator = curve.scalar_mul(label, base_generator)?;
                    for scalar in 1..spec.order {
                        let target = curve.scalar_mul(scalar, generator)?;
                        let (x, y) = point_base_coordinates(&field, target)?;
                        let target_digest = digest(spec, label, x, y);
                        writeln!(labels, "{}\t{}\t{}\t{}", spec.slug, label, target_digest, scalar)
                            .map_err(|error| error.to_string())?;
                        for lambda in [1_u64, 2] {
                            let (model_a, model_b, target_x, target_y, reference_x) =
                                mapped_base_inputs(spec, target, generator, &field, lambda)?;
                            let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
                            let (candidate, _) = jet.online(target_x, target_y, reference_x)?;
                            let (repeat, _) = jet.online(target_x, target_y, reference_x)?;
                            repeat_mismatches += u64::from(candidate != repeat);
                            write!(
                                moments,
                                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                                spec.slug, spec.p, spec.a, spec.b, spec.order, spec.trace, spec.degree,
                                label, target_digest, lambda, model_a, model_b,
                            ).map_err(|error| error.to_string())?;
                            write_moments(&mut moments, candidate)?;
                            records += 1;
                        }
                        targets += 1;
                    }
                }
            }
            moments.flush().map_err(|error| error.to_string())?;
            labels.flush().map_err(|error| error.to_string())?;
            println!(
                "quotient OOS producer: targets={targets} records={records} repeat_mismatches={repeat_mismatches}"
            );
            if targets != 4560 || records != 9120 || repeat_mismatches != 0 {
                return Err("OOS producer census/repeat gate failed".to_string());
            }
            Ok(())
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(CURVES.len(), 5);
            assert_eq!(CURVES.iter().map(|spec| spec.order - 1).sum::<u64>() * 3, 4560);
            assert!(CURVES.iter().all(|spec| spec.p > 19 && spec.order > 6));
            println!("h116 quotient digit OOS producer selftest: pass");
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::oos::selftest(),
        [_, flag, moments, labels] if flag == "--generate" =>
            frozen::oos::generate(Path::new(moments), Path::new(labels)),
        _ => Err("usage: h116_quotient_digit_oos_producer --selftest | --generate MOMENTS.tsv LABELS.tsv".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient OOS producer error: {error}"); std::process::exit(1); }
}
