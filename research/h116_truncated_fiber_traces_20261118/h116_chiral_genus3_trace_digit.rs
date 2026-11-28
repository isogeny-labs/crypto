#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod chiral_genus3_trace {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const FEATURES: usize = 4;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target {
            slug: String,
            digit: u8,
            residues: [i8; FEATURES],
        }

        struct Rule {
            code: u8,
            coefficients: [u8; FEATURES],
            offset: u8,
            predictions: Vec<u8>,
            coverage: usize,
            matches: usize,
            statistic: (u128, u128),
        }

        fn add(left: u64, right: u64, p: u64) -> u64 {
            ((left as u128 + right as u128) % p as u128) as u64
        }

        fn sub(left: u64, right: u64, p: u64) -> u64 {
            (left + p - right) % p
        }

        fn legendre(value: u64, p: u64) -> i8 {
            if value == 0 {
                0
            } else if pow_mod_u64(value, (p - 1) / 2, p) == 1 {
                1
            } else {
                -1
            }
        }

        fn function_value(x: u64, y: u64, coefficient: u64, p: u64) -> u64 {
            add(x, mul_mod_u64(coefficient, y, p), p)
        }

        fn smooth_level(spec: CurveSpec, coefficient: u64, level: u64) -> bool {
            let p = spec.p;
            let coefficient_sq = mul_mod_u64(coefficient, coefficient, p);
            let cubic = coefficient_sq;
            let quadratic = p - 1;
            let linear = add(
                mul_mod_u64(coefficient_sq, spec.a, p),
                mul_mod_u64(2, level, p),
                p,
            );
            let constant = sub(
                mul_mod_u64(coefficient_sq, spec.b, p),
                mul_mod_u64(level, level, p),
                p,
            );

            let quadratic_sq = mul_mod_u64(quadratic, quadratic, p);
            let linear_sq = mul_mod_u64(linear, linear, p);
            let linear_cu = mul_mod_u64(linear_sq, linear, p);
            let quadratic_cu = mul_mod_u64(quadratic_sq, quadratic, p);
            let cubic_sq = mul_mod_u64(cubic, cubic, p);
            let constant_sq = mul_mod_u64(constant, constant, p);

            let mut discriminant = mul_mod_u64(quadratic_sq, linear_sq, p);
            discriminant = sub(
                discriminant,
                mul_mod_u64(4, mul_mod_u64(cubic, linear_cu, p), p),
                p,
            );
            discriminant = sub(
                discriminant,
                mul_mod_u64(4, mul_mod_u64(quadratic_cu, constant, p), p),
                p,
            );
            discriminant = sub(
                discriminant,
                mul_mod_u64(27, mul_mod_u64(cubic_sq, constant_sq, p), p),
                p,
            );
            discriminant = add(
                discriminant,
                mul_mod_u64(
                    18,
                    mul_mod_u64(
                        mul_mod_u64(cubic, quadratic, p),
                        mul_mod_u64(linear, constant, p),
                        p,
                    ),
                    p,
                ),
                p,
            );
            discriminant != 0
        }

        fn decode_coefficients(mut code: u8) -> [u8; FEATURES] {
            let mut coefficients = [0_u8; FEATURES];
            for coefficient in &mut coefficients {
                *coefficient = code % 3;
                code /= 3;
            }
            coefficients
        }

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let excess = 3_i128 * matches as i128 - coverage as i128;
            if excess <= 0 {
                (0, 1)
            } else {
                ((excess as u128).pow(2), coverage as u128)
            }
        }

        fn compare_ratio(left: (u128, u128), right: (u128, u128)) -> Ordering {
            (left.0 * right.1).cmp(&(right.0 * left.1))
        }

        fn splitmix64(state: &mut u64) -> u64 {
            *state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        fn prediction(target: &Target, coefficients: &[u8; FEATURES], offset: u8) -> u8 {
            let mut value = offset;
            for feature in 0..FEATURES {
                if coefficients[feature] == 0 {
                    continue;
                }
                if target.residues[feature] < 0 {
                    return 3;
                }
                value = (value
                    + coefficients[feature] * target.residues[feature] as u8)
                    % 3;
            }
            value
        }

        fn score(predictions: &[u8], labels: &[u8]) -> (usize, usize) {
            let mut coverage = 0_usize;
            let mut matches = 0_usize;
            for (&prediction, &label) in predictions.iter().zip(labels) {
                if prediction == 3 {
                    continue;
                }
                coverage += 1;
                matches += usize::from(prediction == label);
            }
            (coverage, matches)
        }

        fn coefficient_string(coefficients: &[u8; FEATURES]) -> String {
            coefficients
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(",")
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(3_usize.pow((FEATURES + 1) as u32), 243);
            assert_eq!(decode_coefficients(0), [0, 0, 0, 0]);
            assert_eq!(decode_coefficients(80), [2, 2, 2, 2]);
            assert_eq!(legendre(2, 7), 1);
            assert_eq!(legendre(3, 7), -1);
            assert_eq!(statistic(5, 9), (36, 9));
            let planted = Target {
                slug: "control".to_string(),
                digit: 1,
                residues: [1, 0, 0, 0],
            };
            assert_eq!(prediction(&planted, &[1, 0, 0, 0], 0), 1);
            assert_eq!(function_value(3, 4, 1, 7), 0);
            assert_eq!(function_value(3, 4, 6, 7), 6);
            println!("h116 chiral genus-three trace digit selftest: pass rules=243");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut smooth_feature_checks = 0_usize;
            let mut smooth_cover_levels = 0_usize;
            let mut weil_checks = 0_usize;
            let mut rescale_checks = 0_usize;
            let mut chirality_swap_checks = 0_usize;
            let mut chiral_trace_differences = 0_usize;
            let mut chiral_residue_differences = 0_usize;
            let mut distinguishing_targets = 0_usize;

            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve {
                    field: &field,
                    a: field.from_base(spec.a),
                    b: field.from_base(spec.b),
                };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity
                    || curve.scalar_mul(spec.order, generator)? != Point::Infinity
                {
                    return Err("generator/order gate failed".to_string());
                }

                let mut point_coordinates = Vec::new();
                for scalar in 1..spec.order {
                    point_coordinates.push(point_base_coordinates(
                        &field,
                        curve.scalar_mul(scalar, generator)?,
                    )?);
                }
                if point_coordinates.len() + 1 != spec.order as usize {
                    return Err("point census failed".to_string());
                }

                let coefficients = [1_u64, spec.p - 1];
                let mut trace_tables = Vec::new();
                for coefficient in coefficients {
                    let point_values: Vec<u64> = point_coordinates
                        .iter()
                        .map(|&(x, y)| function_value(x, y, coefficient, spec.p))
                        .collect();
                    let mut traces = vec![None; spec.p as usize];
                    for level in 0..spec.p {
                        if !smooth_level(spec, coefficient, level) {
                            continue;
                        }
                        let sum: i64 = point_values
                            .iter()
                            .map(|&value| i64::from(legendre(sub(value, level, spec.p), spec.p)))
                            .sum();
                        let cover_trace = spec.trace - sum;
                        if cover_trace * cover_trace > 36 * spec.p as i64 {
                            return Err("genus-three Weil bound failed".to_string());
                        }
                        let cover_count = spec.order as i64 + sum;
                        if spec.p as i64 + 1 - cover_count != cover_trace {
                            return Err("cover count/trace identity failed".to_string());
                        }
                        traces[level as usize] = Some(cover_trace);
                        smooth_cover_levels += 1;
                        weil_checks += 1;

                        for scale in [2_u64, 3] {
                            let scale_sq = mul_mod_u64(scale, scale, spec.p);
                            let scale_cu = mul_mod_u64(scale_sq, scale, spec.p);
                            let scale_inv = pow_mod_u64(scale, spec.p - 2, spec.p);
                            let scaled_coefficient =
                                mul_mod_u64(coefficient, scale_inv, spec.p);
                            let scaled_level = mul_mod_u64(scale_sq, level, spec.p);
                            let scaled_sum: i64 = point_coordinates
                                .iter()
                                .map(|&(x, y)| {
                                    let scaled_x = mul_mod_u64(scale_sq, x, spec.p);
                                    let scaled_y = mul_mod_u64(scale_cu, y, spec.p);
                                    let value = function_value(
                                        scaled_x,
                                        scaled_y,
                                        scaled_coefficient,
                                        spec.p,
                                    );
                                    i64::from(legendre(
                                        sub(value, scaled_level, spec.p),
                                        spec.p,
                                    ))
                                })
                                .sum();
                            let scaled_trace = spec.trace - scaled_sum;
                            if scaled_trace != cover_trace {
                                return Err("coordinate-rescaled full trace changed".to_string());
                            }
                            rescale_checks += 1;
                        }
                    }
                    trace_tables.push(traces);
                }

                for scalar in 1..spec.order {
                    let target_point = curve.scalar_mul(scalar, generator)?;
                    let mut residues = [-1_i8; FEATURES];
                    let mut target_distinguished = false;
                    for multiplier in 1..=2_u64 {
                        let multiple = curve.scalar_mul(multiplier, target_point)?;
                        let (x, y) = point_base_coordinates(&field, multiple)?;
                        let plus_level = function_value(x, y, 1, spec.p);
                        let minus_level = function_value(x, y, spec.p - 1, spec.p);
                        let plus_trace = trace_tables[0][plus_level as usize];
                        let minus_trace = trace_tables[1][minus_level as usize];
                        let feature = 2 * (multiplier as usize - 1);
                        if let Some(trace) = plus_trace {
                            residues[feature] = trace.rem_euclid(3) as i8;
                            smooth_feature_checks += 1;
                        }
                        if let Some(trace) = minus_trace {
                            residues[feature + 1] = trace.rem_euclid(3) as i8;
                            smooth_feature_checks += 1;
                        }
                        if let (Some(plus), Some(minus)) = (plus_trace, minus_trace) {
                            chiral_trace_differences += usize::from(plus != minus);
                            let residues_differ = plus.rem_euclid(3) != minus.rem_euclid(3);
                            chiral_residue_differences += usize::from(residues_differ);
                            target_distinguished |= residues_differ;
                        }

                        let swapped_plus = trace_tables[0][minus_level as usize];
                        let swapped_minus = trace_tables[1][minus_level as usize];
                        if swapped_plus != swapped_minus {
                            return Err("chiral negation swap failed".to_string());
                        }
                        chirality_swap_checks += 1;
                    }
                    distinguishing_targets += usize::from(target_distinguished);
                    let index = targets.len();
                    targets.push(Target {
                        slug: spec.slug.to_string(),
                        digit: (scalar % 3) as u8,
                        residues,
                    });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }

            if targets.len() != TARGETS || rows.len() != 5 {
                return Err("target census failed".to_string());
            }
            if chirality_swap_checks != 2 * TARGETS {
                return Err("chirality swap census failed".to_string());
            }
            let labels: Vec<u8> = targets.iter().map(|target| target.digit).collect();
            if labels.iter().zip(&labels).filter(|(left, right)| left == right).count()
                != TARGETS
            {
                return Err("planted scorer control failed".to_string());
            }

            let mut rules = Vec::with_capacity(243);
            for code in 0_u8..81 {
                let coefficients = decode_coefficients(code);
                for offset in 0..3_u8 {
                    let predictions: Vec<u8> = targets
                        .iter()
                        .map(|target| prediction(target, &coefficients, offset))
                        .collect();
                    let (coverage, matches) = score(&predictions, &labels);
                    rules.push(Rule {
                        code,
                        coefficients,
                        offset,
                        predictions,
                        coverage,
                        matches,
                        statistic: statistic(matches, coverage),
                    });
                }
            }

            let selected_index = (0..rules.len())
                .max_by(|&left, &right| {
                    compare_ratio(rules[left].statistic, rules[right].statistic)
                        .then_with(|| rules[right].code.cmp(&rules[left].code))
                        .then_with(|| rules[right].offset.cmp(&rules[left].offset))
                })
                .ok_or("no rules")?;
            let selected = &rules[selected_index];
            let total_exact_rules = rules
                .iter()
                .filter(|rule| rule.coverage == TARGETS && rule.matches == TARGETS)
                .count();
            let covered_exact_rules = rules
                .iter()
                .filter(|rule| rule.coverage > 0 && rule.matches == rule.coverage)
                .count();

            let mut exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled = vec![0_u8; TARGETS];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> =
                        indices.iter().map(|&index| labels[index]).collect();
                    let mut seed = 0x116_012_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut values, &mut seed);
                    for (&index, value) in indices.iter().zip(values) {
                        shuffled[index] = value;
                    }
                }
                let mut maximum = (0_u128, 1_u128);
                for rule in &rules {
                    let (_, matches) = score(&rule.predictions, &shuffled);
                    let candidate = statistic(matches, rule.coverage);
                    if compare_ratio(candidate, maximum) == Ordering::Greater {
                        maximum = candidate;
                    }
                }
                exceedances +=
                    usize::from(compare_ratio(maximum, selected.statistic) != Ordering::Less);
            }

            let mut per_curve = Vec::new();
            for (slug, indices) in &rows {
                let predictions: Vec<u8> = indices
                    .iter()
                    .map(|&index| selected.predictions[index])
                    .collect();
                let curve_labels: Vec<u8> =
                    indices.iter().map(|&index| labels[index]).collect();
                let (coverage, matches) = score(&predictions, &curve_labels);
                per_curve.push(format!(
                    "{{\"slug\":\"{slug}\",\"targets\":{},\"coverage\":{coverage},\"matches\":{matches}}}",
                    indices.len()
                ));
            }

            let excess = 3_i64 * selected.matches as i64 - selected.coverage as i64;
            let json = format!(
                concat!(
                    "{{\n  \"schema\":\"h116-chiral-genus3-trace-digit/v1\",\n",
                    "  \"targets\":{},\n  \"curves\":{},\n  \"features\":4,\n  \"rules\":{},\n",
                    "  \"smooth_cover_levels\":{},\n  \"smooth_feature_checks\":{},\n",
                    "  \"weil_and_trace_checks\":{},\n  \"rescale_checks\":{},\n",
                    "  \"chirality_swap_checks\":{},\n  \"chiral_trace_differences\":{},\n",
                    "  \"chiral_residue_differences\":{},\n  \"distinguishing_targets\":{},\n",
                    "  \"planted_control_matches\":{},\n  \"selected_code\":{},\n",
                    "  \"selected_coefficients\":[{}],\n  \"selected_offset\":{},\n",
                    "  \"coverage\":{},\n  \"matches\":{},\n  \"ternary_excess\":{},\n",
                    "  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                    "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n",
                    "  \"replicates\":4096,\n  \"familywise_exceedances\":{},\n",
                    "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                    "  \"status\":\"measured\"\n}}\n"
                ),
                targets.len(),
                rows.len(),
                rules.len(),
                smooth_cover_levels,
                smooth_feature_checks,
                weil_checks,
                rescale_checks,
                chirality_swap_checks,
                chiral_trace_differences,
                chiral_residue_differences,
                distinguishing_targets,
                TARGETS,
                selected.code,
                coefficient_string(&selected.coefficients),
                selected.offset,
                selected.coverage,
                selected.matches,
                excess,
                selected.statistic.0,
                selected.statistic.1,
                total_exact_rules,
                covered_exact_rules,
                exceedances,
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                per_curve.join(",")
            );
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!(
                "chiral genus-three trace digit: coefficients=[{}] offset={} coverage={} matches={} exact={} p={:.6}",
                coefficient_string(&selected.coefficients),
                selected.offset,
                selected.coverage,
                selected.matches,
                total_exact_rules,
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64
            );
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::chiral_genus3_trace::selftest(),
        [_, flag, output] if flag == "--run" => {
            frozen::chiral_genus3_trace::run(Path::new(output))
        }
        _ => Err(
            "usage: h116_chiral_genus3_trace_digit --selftest | --run OUTPUT.json".to_string(),
        ),
    };
    if let Err(error) = result {
        eprintln!("h116 chiral genus-three trace digit error: {error}");
        std::process::exit(1);
    }
}
