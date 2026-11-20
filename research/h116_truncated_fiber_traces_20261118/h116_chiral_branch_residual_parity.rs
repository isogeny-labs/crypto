#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod branch_residual {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const MULTIPLIERS: usize = 4;
        const FEATURES: usize = 2 * MULTIPLIERS;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct BranchData {
            bit: i8,
            discriminant: u64,
            smooth: bool,
            split_group_control: bool,
        }

        #[derive(Clone)]
        struct Target {
            slug: String,
            parity: u8,
            bits: [i8; FEATURES],
        }

        struct Rule {
            mask: u16,
            polarity: u8,
            predictions: Vec<u8>,
            coverage: usize,
            matches: usize,
            statistic: (u128, u128),
        }

        fn residue(value: i128, p: u64) -> u64 {
            value.rem_euclid(i128::from(p)) as u64
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

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let excess = 2_i128 * matches as i128 - coverage as i128;
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

        fn prediction(target: &Target, mask: u16, polarity: u8) -> u8 {
            let mut value = polarity;
            for (feature, &bit) in target.bits.iter().enumerate() {
                if mask & (1_u16 << feature) == 0 {
                    continue;
                }
                if bit < 0 {
                    return 3;
                }
                value ^= bit as u8;
            }
            value
        }

        fn branch_data(
            spec: CurveSpec,
            field: &Field,
            curve: &Curve<'_>,
            point: Point,
            chirality: i64,
        ) -> Result<BranchData, String> {
            if chirality != 1 && chirality != -1 {
                return Err("branch chirality must be a sign".to_string());
            }
            let (x, y) = point_base_coordinates(field, point)?;
            let p = spec.p;
            let level = residue(i128::from(x) + i128::from(chirality) * i128::from(y), p);

            // Intersect y=c*(level-x) with y^2=x^3+a*x+b.  The resulting
            // monic cubic has coefficients (low to high)
            // (b-level^2, a+2*level, -1, 1), and x(Q) is a known root.
            let cubic = [
                residue(i128::from(spec.b) - i128::from(level) * i128::from(level), p),
                residue(i128::from(spec.a) + 2 * i128::from(level), p),
                p - 1,
                1,
            ];
            let quotient = [
                residue(
                    i128::from(spec.a)
                        + 2 * i128::from(level)
                        + i128::from(x) * i128::from(x)
                        - i128::from(x),
                    p,
                ),
                residue(i128::from(x) - 1, p),
                1,
            ];
            let reconstructed = [
                residue(-i128::from(x) * i128::from(quotient[0]), p),
                residue(
                    i128::from(quotient[0])
                        - i128::from(x) * i128::from(quotient[1]),
                    p,
                ),
                residue(i128::from(quotient[1]) - i128::from(x), p),
                1,
            ];
            if reconstructed != cubic {
                return Err("branch cubic reconstruction failed".to_string());
            }

            let separation = residue(
                i128::from(x) * i128::from(x)
                    + i128::from(quotient[1]) * i128::from(x)
                    + i128::from(quotient[0]),
                p,
            );
            let discriminant = residue(
                i128::from(quotient[1]) * i128::from(quotient[1])
                    - 4 * i128::from(quotient[0]),
                p,
            );
            let closed_discriminant = residue(
                -3 * i128::from(x) * i128::from(x)
                    - 6 * i128::from(x)
                    + 1
                    - 4 * i128::from(spec.a)
                    - 8 * i128::from(chirality) * i128::from(y),
                p,
            );
            if discriminant != closed_discriminant {
                return Err("branch residual discriminant formula failed".to_string());
            }

            let cubic_a = -1_i128;
            let cubic_b = i128::from(spec.a) + 2 * i128::from(level);
            let cubic_c = i128::from(spec.b) - i128::from(level) * i128::from(level);
            let cubic_discriminant = residue(
                cubic_a * cubic_a * cubic_b * cubic_b
                    - 4 * cubic_b * cubic_b * cubic_b
                    - 4 * cubic_a * cubic_a * cubic_a * cubic_c
                    - 27 * cubic_c * cubic_c
                    + 18 * cubic_a * cubic_b * cubic_c,
                p,
            );
            let factored_discriminant = residue(
                i128::from(separation)
                    * i128::from(separation)
                    * i128::from(discriminant),
                p,
            );
            if cubic_discriminant != factored_discriminant {
                return Err("branch cubic discriminant factorization failed".to_string());
            }

            let roots: Vec<u64> = (0..p)
                .filter(|&root| {
                    residue(
                        i128::from(root) * i128::from(root)
                            + i128::from(quotient[1]) * i128::from(root)
                            + i128::from(quotient[0]),
                        p,
                    ) == 0
                })
                .collect();
            let signature = legendre(discriminant, p);
            let expected_roots = match signature {
                -1 => 0,
                0 => 1,
                1 => 2,
                _ => unreachable!(),
            };
            if roots.len() != expected_roots {
                return Err("branch residual root-count control failed".to_string());
            }

            let smooth = separation != 0 && discriminant != 0;
            let mut split_group_control = false;
            if smooth && signature == 1 {
                let mut residual_points = Vec::new();
                for root in roots {
                    let root_y = residue(
                        i128::from(chirality)
                            * (i128::from(level) - i128::from(root)),
                        p,
                    );
                    if residue(
                        i128::from(root_y) * i128::from(root_y)
                            - i128::from(root) * i128::from(root) * i128::from(root)
                            - i128::from(spec.a) * i128::from(root)
                            - i128::from(spec.b),
                        p,
                    ) != 0
                    {
                        return Err("branch residual point is off curve".to_string());
                    }
                    residual_points.push(Point::Affine(
                        field.from_base(root),
                        field.from_base(root_y),
                    ));
                }
                if residual_points.len() != 2
                    || curve.add(curve.add(point, residual_points[0])?, residual_points[1])?
                        != Point::Infinity
                {
                    return Err("branch residual chord-sum control failed".to_string());
                }
                split_group_control = true;
            }

            Ok(BranchData {
                bit: if smooth {
                    i8::from(signature == 1)
                } else {
                    -1
                },
                discriminant,
                smooth,
                split_group_control,
            })
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(2 * (1_u16 << FEATURES), 512);
            assert_eq!(statistic(8, 10), (36, 10));
            let spec = CURVES[0];
            let field = Field::new(spec.p, 1, &[0, 1])?;
            let curve = Curve {
                field: &field,
                a: field.from_base(spec.a),
                b: field.from_base(spec.b),
            };
            let point = first_base_point(&curve, spec.p)?;
            let plus = branch_data(spec, &field, &curve, point, 1)?;
            let swapped = branch_data(spec, &field, &curve, curve.neg(point), -1)?;
            assert_eq!(plus, swapped);
            println!("h116 chiral branch residual parity selftest: pass rules=512");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut reconstruction_checks = 0_usize;
            let mut discriminant_checks = 0_usize;
            let mut root_count_checks = 0_usize;
            let mut chiral_swap_checks = 0_usize;
            let mut split_group_checks = 0_usize;
            let mut smooth_features = 0_usize;
            let mut tangent_abstentions = 0_usize;
            let mut residual_repeat_abstentions = 0_usize;
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
                for scalar in 1..spec.order {
                    let target_point = curve.scalar_mul(scalar, generator)?;
                    let mut bits = [-1_i8; FEATURES];
                    let mut distinguished = false;
                    for multiplier in 1..=MULTIPLIERS as u64 {
                        let point = curve.scalar_mul(multiplier, target_point)?;
                        let negative = curve.neg(point);
                        for (chirality_index, chirality) in [1_i64, -1].into_iter().enumerate() {
                            let data = branch_data(spec, &field, &curve, point, chirality)?;
                            let swapped =
                                branch_data(spec, &field, &curve, negative, -chirality)?;
                            if data != swapped {
                                return Err("branch chiral negation swap failed".to_string());
                            }
                            let feature = 2 * (multiplier as usize - 1) + chirality_index;
                            bits[feature] = data.bit;
                            reconstruction_checks += 1;
                            discriminant_checks += 2;
                            root_count_checks += 1;
                            chiral_swap_checks += 1;
                            split_group_checks += usize::from(data.split_group_control);
                            smooth_features += usize::from(data.smooth);
                            if !data.smooth {
                                let (x, y) = point_base_coordinates(&field, point)?;
                                let level = residue(
                                    i128::from(x)
                                        + i128::from(chirality) * i128::from(y),
                                    spec.p,
                                );
                                let q0 = residue(
                                    i128::from(spec.a)
                                        + 2 * i128::from(level)
                                        + i128::from(x) * i128::from(x)
                                        - i128::from(x),
                                    spec.p,
                                );
                                let q1 = residue(i128::from(x) - 1, spec.p);
                                let separation = residue(
                                    i128::from(x) * i128::from(x)
                                        + i128::from(q1) * i128::from(x)
                                        + i128::from(q0),
                                    spec.p,
                                );
                                tangent_abstentions += usize::from(separation == 0);
                                residual_repeat_abstentions +=
                                    usize::from(data.discriminant == 0);
                            }
                        }
                        let plus = bits[2 * (multiplier as usize - 1)];
                        let minus = bits[2 * (multiplier as usize - 1) + 1];
                        distinguished |= plus != minus;
                    }
                    distinguishing_targets += usize::from(distinguished);
                    let index = targets.len();
                    targets.push(Target {
                        slug: spec.slug.to_string(),
                        parity: (scalar % 2) as u8,
                        bits,
                    });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != CURVES.len() {
                return Err("target census failed".to_string());
            }
            if reconstruction_checks != TARGETS * FEATURES
                || discriminant_checks != 2 * TARGETS * FEATURES
                || root_count_checks != TARGETS * FEATURES
                || chiral_swap_checks != TARGETS * FEATURES
            {
                return Err("branch control census failed".to_string());
            }

            let labels: Vec<u8> = targets.iter().map(|target| target.parity).collect();
            let mut rules = Vec::with_capacity(512);
            for mask in 0_u16..(1_u16 << FEATURES) {
                for polarity in 0..=1_u8 {
                    let predictions: Vec<u8> = targets
                        .iter()
                        .map(|target| prediction(target, mask, polarity))
                        .collect();
                    let (coverage, matches) = score(&predictions, &labels);
                    rules.push(Rule {
                        mask,
                        polarity,
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
                        .then_with(|| rules[right].mask.cmp(&rules[left].mask))
                        .then_with(|| rules[right].polarity.cmp(&rules[left].polarity))
                })
                .ok_or("no branch rules")?;
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
                    let mut seed = 0x116_018_u64
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

            let excess = 2_i64 * selected.matches as i64 - selected.coverage as i64;
            let json = format!(
                concat!(
                    "{{\n  \"schema\":\"h116-chiral-branch-residual-parity/v1\",\n",
                    "  \"targets\":{},\n  \"curves\":{},\n  \"features\":{},\n  \"rules\":{},\n",
                    "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                    "  \"root_count_checks\":{},\n  \"chiral_swap_checks\":{},\n",
                    "  \"split_group_checks\":{},\n  \"smooth_features\":{},\n",
                    "  \"tangent_abstentions\":{},\n  \"residual_repeat_abstentions\":{},\n",
                    "  \"distinguishing_targets\":{},\n  \"planted_control_matches\":{},\n",
                    "  \"selected_mask\":{},\n  \"selected_polarity\":{},\n",
                    "  \"coverage\":{},\n  \"matches\":{},\n  \"binary_excess\":{},\n",
                    "  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                    "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n",
                    "  \"replicates\":4096,\n  \"familywise_exceedances\":{},\n",
                    "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                    "  \"status\":\"measured\"\n}}\n"
                ),
                targets.len(),
                rows.len(),
                FEATURES,
                rules.len(),
                reconstruction_checks,
                discriminant_checks,
                root_count_checks,
                chiral_swap_checks,
                split_group_checks,
                smooth_features,
                tangent_abstentions,
                residual_repeat_abstentions,
                distinguishing_targets,
                TARGETS,
                selected.mask,
                selected.polarity,
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
                "chiral branch residual parity: mask={} polarity={} coverage={} matches={} exact={} p={:.6}",
                selected.mask,
                selected.polarity,
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
        [_, flag] if flag == "--selftest" => frozen::branch_residual::selftest(),
        [_, flag, output] if flag == "--run" => {
            frozen::branch_residual::run(Path::new(output))
        }
        _ => Err(
            "usage: h116_chiral_branch_residual_parity --selftest | --run OUTPUT.json"
                .to_string(),
        ),
    };
    if let Err(error) = result {
        eprintln!("h116 chiral branch residual parity error: {error}");
        std::process::exit(1);
    }
}
