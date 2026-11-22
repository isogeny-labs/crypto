#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod large {
        use super::*;

        const PROTOCOL: &str = "H116-QUOTIENT-LARGE-PANEL-001";
        const LARGE_CURVES: [CurveSpec; 3] = [
            CurveSpec { slug: "h116-large-fp5003-a1-b15-r5081", p: 5003, a: 1, b: 15, order: 5081, trace: -77, degree: 635 },
            CurveSpec { slug: "h116-large-fp10007-a1-b28-r9851", p: 10007, a: 1, b: 28, order: 9851, trace: 157, degree: 4925 },
            CurveSpec { slug: "h116-large-fp20011-a1-b36-r19927", p: 20011, a: 1, b: 36, order: 19927, trace: 85, degree: 19926 },
        ];
        const TARGETS: u64 = 34_856;
        const RECORDS: u64 = 69_712;

        fn is_prime(value: u64) -> bool {
            if value < 2 { return false; }
            if value % 2 == 0 { return value == 2; }
            let mut divisor = 3_u64;
            while divisor * divisor <= value {
                if value % divisor == 0 { return false; }
                divisor += 2;
            }
            true
        }

        fn group_order(p: u64, a: u64, b: u64) -> u64 {
            let mut character_sum = 0_i64;
            for x in 0..p {
                let cubic = (mul_mod_u64(mul_mod_u64(x, x, p), x, p)
                    + mul_mod_u64(a, x, p) + b) % p;
                if cubic != 0 {
                    character_sum += if pow_mod_u64(cubic, (p - 1) / 2, p) == 1 { 1 } else { -1 };
                }
            }
            (p as i64 + 1 + character_sum) as u64
        }

        fn first_prime_order_b(p: u64, a: u64, limit: u64) -> Option<(u64, u64)> {
            for b in 1..=limit {
                let discriminant = (mul_mod_u64(4, mul_mod_u64(mul_mod_u64(a, a, p), a, p), p)
                    + mul_mod_u64(27, mul_mod_u64(b, b, p), p)) % p;
                if discriminant == 0 { continue; }
                let order = group_order(p, a, b);
                if is_prime(order) { return Some((b, order)); }
            }
            None
        }

        fn digest(spec: CurveSpec, x: u64, y: u64) -> String {
            sha256_hex(format!("{PROTOCOL}\0{}\0{x}\0{y}", spec.slug).as_bytes())
        }

        fn write_moments<W: Write>(writer: &mut W, value: Moments) -> Result<(), String> {
            for item in value.x.into_iter().chain(value.y) {
                write!(writer, "\t{item}").map_err(|error| error.to_string())?;
            }
            writeln!(writer).map_err(|error| error.to_string())
        }

        pub fn selftest() -> Result<(), String> {
            assert!(LARGE_CURVES.iter().all(|spec| is_prime(spec.p) && is_prime(spec.order)));
            for spec in LARGE_CURVES {
                assert_eq!(first_prime_order_b(spec.p, spec.a, spec.b), Some((spec.b, spec.order)));
                assert_eq!(spec.p as i64 + 1 - spec.order as i64, spec.trace);
            }
            assert_eq!(LARGE_CURVES.iter().map(|spec| spec.order - 1).sum::<u64>(), TARGETS);
            println!("h116 quotient large producer selftest: pass curves=3 targets={TARGETS}");
            Ok(())
        }

        pub fn generate(moment_path: &Path, label_path: &Path) -> Result<(), String> {
            let mut moments = BufWriter::new(File::create(moment_path).map_err(|error| error.to_string())?);
            let mut labels = BufWriter::new(File::create(label_path).map_err(|error| error.to_string())?);
            writeln!(moments, "h116-invariant-moments/v1").map_err(|error| error.to_string())?;
            writeln!(labels, "h116-invariant-labels/v1").map_err(|error| error.to_string())?;
            let mut targets = 0_u64;
            let mut records = 0_u64;
            let mut repeat_mismatches = 0_u64;
            for spec in LARGE_CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity || curve.scalar_mul(spec.order, generator)? != Point::Infinity {
                    return Err(format!("{} generator/order failure", spec.slug));
                }
                for scalar in 1..spec.order {
                    let target = curve.scalar_mul(scalar, generator)?;
                    let (x, y) = point_base_coordinates(&field, target)?;
                    let target_digest = digest(spec, x, y);
                    writeln!(labels, "{}\t1\t{}\t{}", spec.slug, target_digest, scalar)
                        .map_err(|error| error.to_string())?;
                    for lambda in [1_u64, 2] {
                        let (model_a, model_b, target_x, target_y, reference_x) =
                            mapped_base_inputs(spec, target, generator, &field, lambda)?;
                        let jet = Jet::new(spec.p, model_a, model_b, spec.trace)?;
                        let (candidate, _) = jet.online(target_x, target_y, reference_x)?;
                        let (repeat, _) = jet.online(target_x, target_y, reference_x)?;
                        repeat_mismatches += u64::from(candidate != repeat);
                        write!(moments, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t1\t{}\t{}\t{}\t{}",
                            spec.slug, spec.p, spec.a, spec.b, spec.order, spec.trace, spec.degree,
                            target_digest, lambda, model_a, model_b)
                            .map_err(|error| error.to_string())?;
                        write_moments(&mut moments, candidate)?;
                        records += 1;
                    }
                    targets += 1;
                }
            }
            moments.flush().map_err(|error| error.to_string())?;
            labels.flush().map_err(|error| error.to_string())?;
            println!("quotient large producer: targets={targets} records={records} repeat_mismatches={repeat_mismatches}");
            if targets != TARGETS || records != RECORDS || repeat_mismatches != 0 {
                return Err("large producer census/repeat gate failed".to_string());
            }
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::large::selftest(),
        [_, flag, moments, labels] if flag == "--generate" =>
            frozen::large::generate(Path::new(moments), Path::new(labels)),
        _ => Err("usage: h116_quotient_large_producer --selftest | --generate MOMENTS.tsv LABELS.tsv".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient large producer error: {error}"); std::process::exit(1); }
}
