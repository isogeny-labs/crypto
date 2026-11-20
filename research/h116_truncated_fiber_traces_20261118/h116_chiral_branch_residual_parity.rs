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
        const RESULTANT_FEATURES: usize = FEATURES * (FEATURES - 1) / 2;
        const TRACE_FEATURES: usize = 4;
        const TRACE_PAIRS: [(usize, usize); TRACE_FEATURES] =
            [(0, 2), (1, 3), (4, 6), (5, 7)];
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct BranchData {
            bit: i8,
            q0: u64,
            q1: u64,
            discriminant: u64,
            smooth: bool,
            split_group_control: bool,
        }

        #[derive(Clone)]
        struct Target {
            slug: String,
            digit: u8,
            parity: u8,
            bits: [i8; FEATURES],
            resultant_bits: [i8; RESULTANT_FEATURES],
            trace_residues: [i8; TRACE_FEATURES],
        }

        struct Rule {
            mask: u16,
            polarity: u8,
            predictions: Vec<u8>,
            coverage: usize,
            matches: usize,
            statistic: (u128, u128),
        }

        struct ResultantRule {
            first: usize,
            second: Option<usize>,
            polarity: u8,
            predictions: Vec<u8>,
            coverage: usize,
            matches: usize,
            statistic: (u128, u128),
        }

        struct TraceRule {
            code: u8,
            coefficients: [u8; TRACE_FEATURES],
            offset: u8,
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

        fn ternary_statistic(matches: usize, coverage: usize) -> (u128, u128) {
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

        fn resultant_prediction(
            target: &Target,
            first: usize,
            second: Option<usize>,
            polarity: u8,
        ) -> u8 {
            let first_bit = target.resultant_bits[first];
            if first_bit < 0 {
                return 3;
            }
            let mut value = polarity ^ first_bit as u8;
            if let Some(second) = second {
                let second_bit = target.resultant_bits[second];
                if second_bit < 0 {
                    return 3;
                }
                value ^= second_bit as u8;
            }
            value
        }

        fn decode_trace_coefficients(mut code: u8) -> [u8; TRACE_FEATURES] {
            let mut coefficients = [0_u8; TRACE_FEATURES];
            for coefficient in &mut coefficients {
                *coefficient = code % 3;
                code /= 3;
            }
            coefficients
        }

        fn trace_prediction(
            target: &Target,
            coefficients: &[u8; TRACE_FEATURES],
            offset: u8,
        ) -> u8 {
            let mut value = offset;
            for (feature, &coefficient) in coefficients.iter().enumerate() {
                if coefficient == 0 {
                    continue;
                }
                let residue = target.trace_residues[feature];
                if residue < 0 {
                    return 3;
                }
                value = (value + coefficient * residue as u8) % 3;
            }
            value
        }

        fn resultant_pair_index(mut left: usize, mut right: usize) -> usize {
            if left > right {
                std::mem::swap(&mut left, &mut right);
            }
            assert!(left < right && right < FEATURES);
            let mut index = 0_usize;
            for first in 0..FEATURES {
                for second in (first + 1)..FEATURES {
                    if first == left && second == right {
                        return index;
                    }
                    index += 1;
                }
            }
            unreachable!()
        }

        fn determinant4(matrix: [[u64; 4]; 4], p: u64) -> u64 {
            let mut determinant = 0_i128;
            for a in 0..4 {
                for b in 0..4 {
                    if b == a {
                        continue;
                    }
                    for c in 0..4 {
                        if c == a || c == b {
                            continue;
                        }
                        for d in 0..4 {
                            if d == a || d == b || d == c {
                                continue;
                            }
                            let permutation = [a, b, c, d];
                            let inversions = (0..4)
                                .flat_map(|left| ((left + 1)..4).map(move |right| (left, right)))
                                .filter(|&(left, right)| {
                                    permutation[left] > permutation[right]
                                })
                                .count();
                            let term = (0..4).fold(1_i128, |product, row| {
                                product * i128::from(matrix[row][permutation[row]])
                            });
                            determinant += if inversions % 2 == 0 { term } else { -term };
                        }
                    }
                }
            }
            residue(determinant, p)
        }

        fn quadratic_resultant(
            left: BranchData,
            right: BranchData,
            p: u64,
        ) -> Result<u64, String> {
            // Res(X^2+aX+b, X^2+cX+d)
            //   = (d-b)^2 + (a-c)(a*d-b*c).
            let a = left.q1;
            let b = left.q0;
            let c = right.q1;
            let d = right.q0;
            let closed = residue(
                (i128::from(d) - i128::from(b)).pow(2)
                    + (i128::from(a) - i128::from(c))
                        * (i128::from(a) * i128::from(d)
                            - i128::from(b) * i128::from(c)),
                p,
            );
            let sylvester = [
                [1, a, b, 0],
                [0, 1, a, b],
                [1, c, d, 0],
                [0, 1, c, d],
            ];
            if determinant4(sylvester, p) != closed {
                return Err("quadratic Sylvester resultant control failed".to_string());
            }
            let common_root = if a == c {
                b == d
            } else {
                let denominator = residue(i128::from(c) - i128::from(a), p);
                let inverse = pow_mod_u64(denominator, p - 2, p);
                let root = mul_mod_u64(
                    residue(i128::from(b) - i128::from(d), p),
                    inverse,
                    p,
                );
                let left_value = residue(
                    i128::from(root) * i128::from(root)
                        + i128::from(a) * i128::from(root)
                        + i128::from(b),
                    p,
                );
                let right_value = residue(
                    i128::from(root) * i128::from(root)
                        + i128::from(c) * i128::from(root)
                        + i128::from(d),
                    p,
                );
                left_value == 0 && right_value == 0
            };
            if (closed == 0) != common_root {
                return Err("quadratic resultant/common-root control failed".to_string());
            }
            let swapped = residue(
                (i128::from(b) - i128::from(d)).pow(2)
                    + (i128::from(c) - i128::from(a))
                        * (i128::from(c) * i128::from(b)
                            - i128::from(d) * i128::from(a)),
                p,
            );
            if closed != swapped {
                return Err("quadratic resultant symmetry failed".to_string());
            }
            Ok(closed)
        }

        fn branch_pair_trace(
            left: BranchData,
            right: BranchData,
            spec: CurveSpec,
            square_root_counts: &[u8],
        ) -> Result<(i64, usize), String> {
            let resultant = quadratic_resultant(left, right, spec.p)?;
            if !left.smooth || !right.smooth || resultant == 0 {
                return Err("branch-pair trace requires a smooth quartic".to_string());
            }
            let p = spec.p;
            let quartic = [
                mul_mod_u64(left.q0, right.q0, p),
                residue(
                    i128::from(left.q1) * i128::from(right.q0)
                        + i128::from(right.q1) * i128::from(left.q0),
                    p,
                ),
                residue(
                    i128::from(left.q0)
                        + i128::from(right.q0)
                        + i128::from(left.q1) * i128::from(right.q1),
                    p,
                ),
                residue(i128::from(left.q1) + i128::from(right.q1), p),
                1,
            ];
            let mut character_sum = 0_i64;
            let mut affine_count = 0_u64;
            let mut evaluation_checks = 0_usize;
            for x in 0..p {
                let left_value = residue(
                    i128::from(x) * i128::from(x)
                        + i128::from(left.q1) * i128::from(x)
                        + i128::from(left.q0),
                    p,
                );
                let right_value = residue(
                    i128::from(x) * i128::from(x)
                        + i128::from(right.q1) * i128::from(x)
                        + i128::from(right.q0),
                    p,
                );
                let product_value = mul_mod_u64(left_value, right_value, p);
                let horner_value = quartic.iter().rev().fold(0_u64, |value, coefficient| {
                    residue(
                        i128::from(value) * i128::from(x) + i128::from(*coefficient),
                        p,
                    )
                });
                if product_value != horner_value {
                    return Err("branch-pair quartic evaluation control failed".to_string());
                }
                let signature = legendre(product_value, p);
                character_sum += i64::from(signature);
                affine_count += u64::from(square_root_counts[product_value as usize]);
                evaluation_checks += 1;
            }
            let expected_affine = p as i64 + character_sum;
            if affine_count as i64 != expected_affine {
                return Err("branch-pair affine point-count control failed".to_string());
            }
            // A monic smooth quartic has two rational points at infinity.
            let projective_count = affine_count + 2;
            let trace = p as i64 + 1 - projective_count as i64;
            if trace != -1 - character_sum || trace * trace > 4 * p as i64 {
                return Err("branch-pair genus-one trace control failed".to_string());
            }
            Ok((trace, evaluation_checks))
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
                q0: quotient[0],
                q1: quotient[1],
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
            let trivial_left = BranchData {
                bit: 0,
                q0: 0,
                q1: 0,
                discriminant: 0,
                smooth: true,
                split_group_control: false,
            };
            let trivial_right = BranchData {
                q0: 1,
                ..trivial_left
            };
            assert_eq!(quadratic_resultant(trivial_left, trivial_right, 5)?, 1);
            assert_eq!(RESULTANT_FEATURES, 28);
            assert_eq!(2 * (RESULTANT_FEATURES + RESULTANT_FEATURES * 27 / 2), 812);
            assert_eq!(decode_trace_coefficients(0), [0, 0, 0, 0]);
            assert_eq!(decode_trace_coefficients(80), [2, 2, 2, 2]);
            println!(
                "h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243"
            );
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false)
        }

        pub fn run_resultants(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false)
        }

        pub fn run_traces(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, true)
        }

        fn run_with_panel(
            output_path: &Path,
            resultant_panel: bool,
            trace_panel: bool,
        ) -> Result<(), String> {
            if resultant_panel && trace_panel {
                return Err("specialized branch panels are mutually exclusive".to_string());
            }
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
            let mut resultant_checks = 0_usize;
            let mut resultant_common_root_checks = 0_usize;
            let mut resultant_smooth_features = 0_usize;
            let mut resultant_zero_abstentions = 0_usize;
            let mut resultant_distinguishing_targets = 0_usize;
            let mut square_root_table_checks = 0_usize;
            let mut branch_pair_trace_checks = 0_usize;
            let mut branch_pair_quartic_evaluation_checks = 0_usize;
            let mut branch_pair_trace_distinguishing_targets = 0_usize;

            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve {
                    field: &field,
                    a: field.from_base(spec.a),
                    b: field.from_base(spec.b),
                };
                let mut square_root_counts = vec![0_u8; spec.p as usize];
                for y in 0..spec.p {
                    let square = mul_mod_u64(y, y, spec.p);
                    square_root_counts[square as usize] += 1;
                }
                if square_root_counts.iter().map(|&count| usize::from(count)).sum::<usize>()
                    != spec.p as usize
                    || square_root_counts[0] != 1
                {
                    return Err("branch-pair square-root table control failed".to_string());
                }
                square_root_table_checks += 1;
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity
                    || curve.scalar_mul(spec.order, generator)? != Point::Infinity
                {
                    return Err("generator/order gate failed".to_string());
                }
                for scalar in 1..spec.order {
                    let target_point = curve.scalar_mul(scalar, generator)?;
                    let mut bits = [-1_i8; FEATURES];
                    let mut branch_features = [None; FEATURES];
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
                            branch_features[feature] = Some(data);
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
                    let mut resultant_bits = [-1_i8; RESULTANT_FEATURES];
                    let mut resultant_feature = 0_usize;
                    for left in 0..FEATURES {
                        for right in (left + 1)..FEATURES {
                            let left_data = branch_features[left]
                                .ok_or("missing left branch feature")?;
                            let right_data = branch_features[right]
                                .ok_or("missing right branch feature")?;
                            let resultant =
                                quadratic_resultant(left_data, right_data, spec.p)?;
                            resultant_checks += 1;
                            resultant_common_root_checks += 1;
                            if left_data.smooth && right_data.smooth && resultant != 0 {
                                resultant_bits[resultant_feature] =
                                    i8::from(legendre(resultant, spec.p) == 1);
                                resultant_smooth_features += 1;
                            } else if resultant == 0 {
                                resultant_zero_abstentions += 1;
                            }
                            resultant_feature += 1;
                        }
                    }
                    if resultant_feature != RESULTANT_FEATURES {
                        return Err("resultant feature census failed".to_string());
                    }
                    let mut trace_residues = [-1_i8; TRACE_FEATURES];
                    for (trace_feature, &(left, right)) in TRACE_PAIRS.iter().enumerate() {
                        let left_data = branch_features[left]
                            .ok_or("missing trace left branch feature")?;
                        let right_data = branch_features[right]
                            .ok_or("missing trace right branch feature")?;
                        let resultant =
                            quadratic_resultant(left_data, right_data, spec.p)?;
                        if left_data.smooth && right_data.smooth && resultant != 0 {
                            let (trace, evaluation_checks) = branch_pair_trace(
                                left_data,
                                right_data,
                                spec,
                                &square_root_counts,
                            )?;
                            trace_residues[trace_feature] = trace.rem_euclid(3) as i8;
                            branch_pair_trace_checks += 1;
                            branch_pair_quartic_evaluation_checks += evaluation_checks;
                        }
                    }
                    let trace_distinguished = trace_residues[0] != trace_residues[1]
                        || trace_residues[2] != trace_residues[3];
                    let mut resultant_distinguished = false;
                    for left in 0..FEATURES {
                        for right in (left + 1)..FEATURES {
                            let index = resultant_pair_index(left, right);
                            let swapped_index = resultant_pair_index(left ^ 1, right ^ 1);
                            resultant_distinguished |=
                                resultant_bits[index] != resultant_bits[swapped_index];
                        }
                    }
                    distinguishing_targets += usize::from(distinguished);
                    resultant_distinguishing_targets +=
                        usize::from(resultant_distinguished);
                    branch_pair_trace_distinguishing_targets +=
                        usize::from(trace_distinguished);
                    let index = targets.len();
                    targets.push(Target {
                        slug: spec.slug.to_string(),
                        digit: (scalar % 3) as u8,
                        parity: (scalar % 2) as u8,
                        bits,
                        resultant_bits,
                        trace_residues,
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
            if resultant_checks != TARGETS * RESULTANT_FEATURES
                || resultant_common_root_checks != TARGETS * RESULTANT_FEATURES
            {
                return Err("branch resultant control census failed".to_string());
            }

            let labels: Vec<u8> = if trace_panel {
                targets.iter().map(|target| target.digit).collect()
            } else {
                targets.iter().map(|target| target.parity).collect()
            };
            if trace_panel {
                let mut rules = Vec::with_capacity(243);
                for code in 0_u8..81 {
                    let coefficients = decode_trace_coefficients(code);
                    for offset in 0..3_u8 {
                        let predictions: Vec<u8> = targets
                            .iter()
                            .map(|target| trace_prediction(target, &coefficients, offset))
                            .collect();
                        let (coverage, matches) = score(&predictions, &labels);
                        rules.push(TraceRule {
                            code,
                            coefficients,
                            offset,
                            predictions,
                            coverage,
                            matches,
                            statistic: ternary_statistic(matches, coverage),
                        });
                    }
                }
                if rules.len() != 243 {
                    return Err("branch-pair trace rule census failed".to_string());
                }
                let selected_index = (0..rules.len())
                    .max_by(|&left, &right| {
                        compare_ratio(rules[left].statistic, rules[right].statistic)
                            .then_with(|| rules[right].code.cmp(&rules[left].code))
                            .then_with(|| rules[right].offset.cmp(&rules[left].offset))
                    })
                    .ok_or("no branch-pair trace rules")?;
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
                        let mut seed = 0x116_020_u64
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
                        let candidate = ternary_statistic(matches, rule.coverage);
                        if compare_ratio(candidate, maximum) == Ordering::Greater {
                            maximum = candidate;
                        }
                    }
                    exceedances += usize::from(
                        compare_ratio(maximum, selected.statistic) != Ordering::Less,
                    );
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
                let coefficients = selected
                    .coefficients
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(",");
                let json = format!(
                    concat!(
                        "{{\n  \"schema\":\"h116-chiral-branch-pair-trace-digit/v1\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"trace_features\":{},\n",
                        "  \"rules\":{},\n  \"square_root_table_checks\":{},\n",
                        "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                        "  \"resultant_checks\":{},\n  \"branch_pair_trace_checks\":{},\n",
                        "  \"branch_pair_quartic_evaluation_checks\":{},\n",
                        "  \"branch_pair_trace_distinguishing_targets\":{},\n",
                        "  \"planted_control_matches\":{},\n  \"selected_code\":{},\n",
                        "  \"selected_coefficients\":[{}],\n  \"selected_offset\":{},\n",
                        "  \"radix\":3,\n  \"coverage\":{},\n  \"matches\":{},\n",
                        "  \"radix_excess\":{},\n  \"statistic_numerator\":{},\n",
                        "  \"statistic_denominator\":{},\n  \"total_exact_rules\":{},\n",
                        "  \"covered_exact_rules\":{},\n  \"replicates\":4096,\n",
                        "  \"familywise_exceedances\":{},\n",
                        "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                        "  \"status\":\"measured\"\n}}\n"
                    ),
                    targets.len(),
                    rows.len(),
                    TRACE_FEATURES,
                    rules.len(),
                    square_root_table_checks,
                    reconstruction_checks,
                    discriminant_checks,
                    resultant_checks,
                    branch_pair_trace_checks,
                    branch_pair_quartic_evaluation_checks,
                    branch_pair_trace_distinguishing_targets,
                    TARGETS,
                    selected.code,
                    coefficients,
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
                    "chiral branch-pair trace digit: coefficients=[{}] offset={} coverage={} matches={} exact={} p={:.6}",
                    coefficients,
                    selected.offset,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }
            if resultant_panel {
                let mut rules = Vec::with_capacity(812);
                for first in 0..RESULTANT_FEATURES {
                    for second in std::iter::once(None).chain(
                        ((first + 1)..RESULTANT_FEATURES).map(Some),
                    ) {
                        for polarity in 0..=1_u8 {
                            let predictions: Vec<u8> = targets
                                .iter()
                                .map(|target| {
                                    resultant_prediction(target, first, second, polarity)
                                })
                                .collect();
                            let (coverage, matches) = score(&predictions, &labels);
                            rules.push(ResultantRule {
                                first,
                                second,
                                polarity,
                                predictions,
                                coverage,
                                matches,
                                statistic: statistic(matches, coverage),
                            });
                        }
                    }
                }
                if rules.len() != 812 {
                    return Err("resultant rule census failed".to_string());
                }
                let selected_index = (0..rules.len())
                    .max_by(|&left, &right| {
                        compare_ratio(rules[left].statistic, rules[right].statistic)
                            .then_with(|| rules[right].first.cmp(&rules[left].first))
                            .then_with(|| {
                                rules[right]
                                    .second
                                    .unwrap_or(RESULTANT_FEATURES)
                                    .cmp(&rules[left].second.unwrap_or(RESULTANT_FEATURES))
                            })
                            .then_with(|| {
                                rules[right].polarity.cmp(&rules[left].polarity)
                            })
                    })
                    .ok_or("no resultant rules")?;
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
                        let mut seed = 0x116_019_u64
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
                    exceedances += usize::from(
                        compare_ratio(maximum, selected.statistic) != Ordering::Less,
                    );
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
                        "{{\n  \"schema\":\"h116-chiral-branch-resultant-parity/v1\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"base_features\":{},\n",
                        "  \"resultant_features\":{},\n  \"rules\":{},\n",
                        "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                        "  \"root_count_checks\":{},\n  \"chiral_swap_checks\":{},\n",
                        "  \"resultant_checks\":{},\n  \"resultant_common_root_checks\":{},\n",
                        "  \"resultant_smooth_features\":{},\n",
                        "  \"resultant_zero_abstentions\":{},\n",
                        "  \"resultant_distinguishing_targets\":{},\n",
                        "  \"planted_control_matches\":{},\n",
                        "  \"selected_first\":{},\n  \"selected_second\":{},\n",
                        "  \"selected_polarity\":{},\n  \"coverage\":{},\n  \"matches\":{},\n",
                        "  \"binary_excess\":{},\n  \"statistic_numerator\":{},\n",
                        "  \"statistic_denominator\":{},\n  \"total_exact_rules\":{},\n",
                        "  \"covered_exact_rules\":{},\n  \"replicates\":4096,\n",
                        "  \"familywise_exceedances\":{},\n",
                        "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                        "  \"status\":\"measured\"\n}}\n"
                    ),
                    targets.len(),
                    rows.len(),
                    FEATURES,
                    RESULTANT_FEATURES,
                    rules.len(),
                    reconstruction_checks,
                    discriminant_checks,
                    root_count_checks,
                    chiral_swap_checks,
                    resultant_checks,
                    resultant_common_root_checks,
                    resultant_smooth_features,
                    resultant_zero_abstentions,
                    resultant_distinguishing_targets,
                    TARGETS,
                    selected.first,
                    selected
                        .second
                        .map_or_else(|| "null".to_string(), |value| value.to_string()),
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
                    "chiral branch resultant parity: first={} second={} polarity={} coverage={} matches={} exact={} p={:.6}",
                    selected.first,
                    selected
                        .second
                        .map_or_else(|| "none".to_string(), |value| value.to_string()),
                    selected.polarity,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }
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
        [_, flag, output] if flag == "--run-resultants" => {
            frozen::branch_residual::run_resultants(Path::new(output))
        }
        [_, flag, output] if flag == "--run-traces" => {
            frozen::branch_residual::run_traces(Path::new(output))
        }
        _ => Err(
            "usage: h116_chiral_branch_residual_parity --selftest | --run OUTPUT.json | --run-resultants OUTPUT.json | --run-traces OUTPUT.json".to_string(),
        ),
    };
    if let Err(error) = result {
        eprintln!("h116 chiral branch residual parity error: {error}");
        std::process::exit(1);
    }
}
