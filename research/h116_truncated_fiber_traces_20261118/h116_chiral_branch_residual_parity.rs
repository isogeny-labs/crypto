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
        const INVARIANT_FEATURES: usize = 2 * TRACE_FEATURES;
        const KUMMER_FEATURES: usize = 2 * TRACE_FEATURES;
        const ORBIT_PREDICATES_PER_FIBER: usize = 3;
        const ORBIT_FEATURES: usize = ORBIT_PREDICATES_PER_FIBER * KUMMER_FEATURES;
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
            invariant_bits: [i8; INVARIANT_FEATURES],
            kummer_bits: [i8; KUMMER_FEATURES],
            orbit_bits: [i8; ORBIT_FEATURES],
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

        fn invariant_prediction(target: &Target, mask: u16, polarity: u8) -> u8 {
            let mut value = polarity;
            for (feature, &bit) in target.invariant_bits.iter().enumerate() {
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

        fn kummer_prediction(target: &Target, mask: u16, polarity: u8) -> u8 {
            let mut value = polarity;
            for (feature, &bit) in target.kummer_bits.iter().enumerate() {
                if mask & (1_u16 << feature) != 0 {
                    if bit < 0 {
                        return 3;
                    }
                    value ^= bit as u8;
                }
            }
            value
        }

        fn orbit_prediction(
            target: &Target,
            first: usize,
            second: Option<usize>,
            polarity: u8,
        ) -> u8 {
            let first_bit = target.orbit_bits[first];
            if first_bit < 0 {
                return 3;
            }
            let mut value = polarity ^ first_bit as u8;
            if let Some(second) = second {
                let second_bit = target.orbit_bits[second];
                if second_bit < 0 {
                    return 3;
                }
                value ^= second_bit as u8;
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
            let quartic = branch_pair_quartic(left, right, p);
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

        fn branch_pair_quartic(left: BranchData, right: BranchData, p: u64) -> [u64; 5] {
            [
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
            ]
        }

        fn binary_quartic_invariants(quartic: [u64; 5], p: u64) -> (u64, u64) {
            let e = i128::from(quartic[0]);
            let d = i128::from(quartic[1]);
            let c = i128::from(quartic[2]);
            let b = i128::from(quartic[3]);
            let a = i128::from(quartic[4]);
            let invariant_i = residue(12 * a * e - 3 * b * d + c * c, p);
            let invariant_j = residue(
                72 * a * c * e + 9 * b * c * d - 27 * a * d * d
                    - 27 * b * b * e
                    - 2 * c * c * c,
                p,
            );
            (invariant_i, invariant_j)
        }

        fn marked_kernel_halves(
            coefficient_a: u64,
            coefficient_b: u64,
            p: u64,
            square_roots: &[Vec<u64>],
        ) -> Result<(bool, usize), String> {
            let discriminant = residue(
                i128::from(coefficient_a) * i128::from(coefficient_a)
                    - 4 * i128::from(coefficient_b),
                p,
            );
            if coefficient_b == 0 || discriminant == 0 {
                return Err("marked two-isogeny model is singular".to_string());
            }

            // Independently enumerate every affine point of
            // y^2=x^3+A*x^2+B*x and apply the general Weierstrass doubling
            // law, looking for doubles equal to the marked kernel point
            // T=(0,0).
            let mut affine_points = 0_usize;
            let mut enumerated_halves = 0_usize;
            for x in 0..p {
                let rhs = residue(
                    i128::from(x).pow(3)
                        + i128::from(coefficient_a) * i128::from(x).pow(2)
                        + i128::from(coefficient_b) * i128::from(x),
                    p,
                );
                for &y in &square_roots[rhs as usize] {
                    affine_points += 1;
                    if y == 0 {
                        continue;
                    }
                    let numerator = residue(
                        3 * i128::from(x).pow(2)
                            + 2 * i128::from(coefficient_a) * i128::from(x)
                            + i128::from(coefficient_b),
                        p,
                    );
                    let denominator = residue(2 * i128::from(y), p);
                    let slope = mul_mod_u64(
                        numerator,
                        pow_mod_u64(denominator, p - 2, p),
                        p,
                    );
                    let doubled_x = residue(
                        i128::from(slope).pow(2)
                            - i128::from(coefficient_a)
                            - 2 * i128::from(x),
                        p,
                    );
                    let doubled_y = residue(
                        -i128::from(y)
                            + i128::from(slope)
                                * (i128::from(x) - i128::from(doubled_x)),
                        p,
                    );
                    enumerated_halves += usize::from(doubled_x == 0 && doubled_y == 0);
                }
            }

            // The duplication equation independently reduces to x(P)^2=B.
            // Recount the candidates from that equation and require exact
            // agreement with the group-law enumeration.
            let mut formula_halves = 0_usize;
            for x in 0..p {
                if mul_mod_u64(x, x, p) != coefficient_b {
                    continue;
                }
                let rhs = residue(
                    i128::from(x).pow(3)
                        + i128::from(coefficient_a) * i128::from(x).pow(2)
                        + i128::from(coefficient_b) * i128::from(x),
                    p,
                );
                formula_halves += square_roots[rhs as usize].len();
            }
            if enumerated_halves != formula_halves || enumerated_halves > 4 {
                return Err("marked kernel halving formula control failed".to_string());
            }
            Ok((enumerated_halves != 0, affine_points))
        }

        fn marked_halving_orbit_type(
            coefficient_a: u64,
            coefficient_b: u64,
            p: u64,
            square_roots: &[Vec<u64>],
            extension: &Field,
        ) -> Result<u8, String> {
            if extension.p != p || extension.d != 4 {
                return Err("halving orbit control requires the quartic extension".to_string());
            }
            let a = extension.from_base(coefficient_a);
            let b = extension.from_base(coefficient_b);
            let two = extension.from_base(2);
            let three = extension.from_base(3);
            let Some(root_b) = extension.sqrt(b)? else {
                return Err("marked kernel x-coordinate did not split over degree four".to_string());
            };
            if extension.square(root_b) != b || root_b == extension.zero() {
                return Err("marked kernel x-coordinate square-root control failed".to_string());
            }

            let mut points = Vec::with_capacity(4);
            for x in [root_b, extension.neg(root_b)] {
                let x2 = extension.square(x);
                let rhs = extension.add(
                    extension.add(extension.mul(x2, x), extension.mul(a, x2)),
                    extension.mul(b, x),
                );
                let Some(y) = extension.sqrt(rhs)? else {
                    return Err("marked kernel y-coordinate did not split over degree four".to_string());
                };
                if y == extension.zero() || extension.square(y) != rhs {
                    return Err("marked kernel y-coordinate square-root control failed".to_string());
                }
                points.push((x, y));
                points.push((x, extension.neg(y)));
            }
            points.sort_by_key(|&(x, y)| (extension.to_index(x), extension.to_index(y)));
            points.dedup();
            if points.len() != 4 {
                return Err("marked halving fiber does not contain four points".to_string());
            }

            for &(x, y) in &points {
                let x2 = extension.square(x);
                let numerator = extension.add(
                    extension.add(extension.mul(three, x2), extension.mul(two, extension.mul(a, x))),
                    b,
                );
                let denominator = extension.mul(two, y);
                let slope = extension.div(numerator, denominator)?;
                let doubled_x = extension.sub(
                    extension.sub(extension.square(slope), a),
                    extension.mul(two, x),
                );
                let doubled_y = extension.add(
                    extension.neg(y),
                    extension.mul(slope, extension.sub(x, doubled_x)),
                );
                if doubled_x != extension.zero() || doubled_y != extension.zero() {
                    return Err("marked degree-four point does not double to the kernel".to_string());
                }
            }

            let mut permutation = [usize::MAX; 4];
            for (index, &(x, y)) in points.iter().enumerate() {
                let image = (extension.frobenius(x), extension.frobenius(y));
                permutation[index] = points
                    .iter()
                    .position(|&point| point == image)
                    .ok_or("Frobenius left the marked halving fiber")?;
            }
            let mut image_census = permutation;
            image_census.sort_unstable();
            if image_census != [0, 1, 2, 3] {
                return Err("marked halving Frobenius action is not a permutation".to_string());
            }
            let mut seen = [false; 4];
            let mut cycles = Vec::new();
            for start in 0..4 {
                if seen[start] {
                    continue;
                }
                let mut current = start;
                let mut length = 0_usize;
                loop {
                    if seen[current] {
                        if current != start {
                            return Err("invalid marked halving Frobenius cycle".to_string());
                        }
                        break;
                    }
                    seen[current] = true;
                    length += 1;
                    current = permutation[current];
                }
                cycles.push(length);
            }
            cycles.sort_unstable();
            let orbit_type = match cycles.as_slice() {
                [1, 1, 1, 1] => 0,
                [1, 1, 2] => 1,
                [2, 2] => 2,
                [4] => 3,
                _ => return Err("unexpected marked halving Frobenius cycle type".to_string()),
            };

            // Independent character formula for the same four cycle types.
            let dual_coefficient_b = residue(
                i128::from(coefficient_a).pow(2) - 4 * i128::from(coefficient_b),
                p,
            );
            let expected_type = if legendre(coefficient_b, p) == -1 {
                if legendre(dual_coefficient_b, p) == 1 { 2 } else { 3 }
            } else {
                let mut fixed_points = 0_usize;
                for x in 0..p {
                    if mul_mod_u64(x, x, p) != coefficient_b {
                        continue;
                    }
                    let rhs = residue(
                        i128::from(x).pow(3)
                            + i128::from(coefficient_a) * i128::from(x).pow(2)
                            + i128::from(coefficient_b) * i128::from(x),
                        p,
                    );
                    fixed_points += square_roots[rhs as usize].len();
                }
                match fixed_points {
                    4 => 0,
                    2 => 1,
                    0 => 2,
                    _ => return Err("invalid rational marked-halving census".to_string()),
                }
            };
            if orbit_type != expected_type {
                return Err("marked halving Frobenius/character control failed".to_string());
            }
            Ok(orbit_type)
        }

        fn marked_two_isogeny_halving(
            left: BranchData,
            right: BranchData,
            p: u64,
            trace: i64,
            square_roots: &[Vec<u64>],
            extension: &Field,
        ) -> Result<([i8; 2], [u8; 2], usize), String> {
            let resultant = quadratic_resultant(left, right, p)?;
            if !left.smooth || !right.smooth || resultant == 0 {
                return Err("marked two-isogeny requires a smooth quartic".to_string());
            }
            let quartic = branch_pair_quartic(left, right, p);
            let (invariant_i, invariant_j) = binary_quartic_invariants(quartic, p);
            let marked_root = residue(
                3 * i128::from(left.q1) * i128::from(right.q1)
                    - 6 * i128::from(left.q0)
                    - 6 * i128::from(right.q0),
                p,
            );
            let resolvent_value = residue(
                i128::from(marked_root).pow(3)
                    - 27 * i128::from(invariant_i) * i128::from(marked_root)
                    - 27 * i128::from(invariant_j),
                p,
            );
            if resolvent_value != 0 {
                return Err("marked factorization does not give a resolvent root".to_string());
            }

            // Translate the marked two-torsion point to (0,0), obtaining
            // y^2=x^3+A*x^2+B*x.  The two displayed identities are checked
            // against the independently computed resultant/discriminants.
            let coefficient_a = residue(3 * i128::from(marked_root), p);
            let coefficient_b = residue(
                3 * i128::from(marked_root).pow(2) - 27 * i128::from(invariant_i),
                p,
            );
            let dual_coefficient_a = residue(-2 * i128::from(coefficient_a), p);
            let dual_coefficient_b = residue(
                i128::from(coefficient_a).pow(2) - 4 * i128::from(coefficient_b),
                p,
            );
            if coefficient_b != mul_mod_u64(81 % p, resultant, p)
                || dual_coefficient_b
                    != mul_mod_u64(
                        81 % p,
                        mul_mod_u64(left.discriminant, right.discriminant, p),
                        p,
                    )
            {
                return Err("marked two-isogeny coefficient identity failed".to_string());
            }

            let (kernel_halves, source_affine_points) = marked_kernel_halves(
                coefficient_a,
                coefficient_b,
                p,
                square_roots,
            )?;
            let (dual_kernel_halves, dual_affine_points) = marked_kernel_halves(
                dual_coefficient_a,
                dual_coefficient_b,
                p,
                square_roots,
            )?;
            let source_orbit_type = marked_halving_orbit_type(
                coefficient_a,
                coefficient_b,
                p,
                square_roots,
                extension,
            )?;
            let dual_orbit_type = marked_halving_orbit_type(
                dual_coefficient_a,
                dual_coefficient_b,
                p,
                square_roots,
                extension,
            )?;
            let expected_affine_points = (p as i64 - trace) as usize;
            if source_affine_points != expected_affine_points
                || dual_affine_points != expected_affine_points
            {
                return Err("marked two-isogeny point-count control failed".to_string());
            }
            Ok((
                [i8::from(kernel_halves), i8::from(dual_kernel_halves)],
                [source_orbit_type, dual_orbit_type],
                source_affine_points + dual_affine_points,
            ))
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
            let (invariant_i, invariant_j) =
                binary_quartic_invariants([106, 0, 0, 0, 1], 107);
            assert_eq!(invariant_i, 95);
            assert_eq!(invariant_j, 0);
            let mut square_roots = vec![Vec::new(); 5];
            for y in 0..5_u64 {
                square_roots[(y * y % 5) as usize].push(y);
            }
            assert_eq!(marked_kernel_halves(1, 1, 5, &square_roots)?, (true, 7));
            let quartic_modulus = first_irreducible(5, 4)?;
            let quartic_extension = Field::new(5, 4, &quartic_modulus)?;
            assert_eq!(
                marked_halving_orbit_type(1, 1, 5, &square_roots, &quartic_extension)?,
                1
            );
            println!(
                "h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243 invariant_rules=512 kummer_rules=512 orbit_rules=600"
            );
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false, false, false, false)
        }

        pub fn run_resultants(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false, false, false, false)
        }

        pub fn run_traces(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, true, false, false, false)
        }

        pub fn run_invariants(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false, true, false, false)
        }

        pub fn run_kummer(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false, false, true, false)
        }

        pub fn run_orbits(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false, false, false, true)
        }

        fn run_with_panel(
            output_path: &Path,
            resultant_panel: bool,
            trace_panel: bool,
            invariant_panel: bool,
            kummer_panel: bool,
            orbit_panel: bool,
        ) -> Result<(), String> {
            if usize::from(resultant_panel)
                + usize::from(trace_panel)
                + usize::from(invariant_panel)
                + usize::from(kummer_panel)
                + usize::from(orbit_panel)
                > 1
            {
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
            let mut binary_quartic_invariant_checks = 0_usize;
            let mut binary_quartic_invariant_distinguishing_targets = 0_usize;
            let mut marked_two_isogeny_checks = 0_usize;
            let mut marked_two_isogeny_point_checks = 0_usize;
            let mut marked_two_isogeny_distinguishing_targets = 0_usize;
            let mut marked_halving_orbit_histogram = [0_usize; 4];
            let mut marked_halving_orbit_distinguishing_targets = 0_usize;

            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve {
                    field: &field,
                    a: field.from_base(spec.a),
                    b: field.from_base(spec.b),
                };
                let orbit_modulus = first_irreducible(spec.p, 4)?;
                let orbit_extension = Field::new(spec.p, 4, &orbit_modulus)?;
                let mut square_root_counts = vec![0_u8; spec.p as usize];
                let mut square_roots = vec![Vec::new(); spec.p as usize];
                for y in 0..spec.p {
                    let square = mul_mod_u64(y, y, spec.p);
                    square_root_counts[square as usize] += 1;
                    square_roots[square as usize].push(y);
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
                    let mut invariant_bits = [-1_i8; INVARIANT_FEATURES];
                    let mut kummer_bits = [-1_i8; KUMMER_FEATURES];
                    let mut orbit_bits = [-1_i8; ORBIT_FEATURES];
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
                            let quartic =
                                branch_pair_quartic(left_data, right_data, spec.p);
                            let (invariant_i, invariant_j) =
                                binary_quartic_invariants(quartic, spec.p);
                            let quartic_discriminant = mul_mod_u64(
                                mul_mod_u64(
                                    left_data.discriminant,
                                    right_data.discriminant,
                                    spec.p,
                                ),
                                mul_mod_u64(resultant, resultant, spec.p),
                                spec.p,
                            );
                            let invariant_discriminant = residue(
                                4 * i128::from(invariant_i).pow(3)
                                    - i128::from(invariant_j).pow(2),
                                spec.p,
                            );
                            if invariant_discriminant
                                != mul_mod_u64(27, quartic_discriminant, spec.p)
                            {
                                return Err(
                                    "binary quartic invariant/discriminant control failed"
                                        .to_string(),
                                );
                            }
                            invariant_bits[2 * trace_feature] = if invariant_i == 0 {
                                -1
                            } else {
                                i8::from(legendre(invariant_i, spec.p) == 1)
                            };
                            invariant_bits[2 * trace_feature + 1] = if invariant_j == 0 {
                                -1
                            } else {
                                i8::from(legendre(invariant_j, spec.p) == 1)
                            };
                            binary_quartic_invariant_checks += 1;
                            let (halving_bits, orbit_types, point_checks) = marked_two_isogeny_halving(
                                left_data,
                                right_data,
                                spec.p,
                                trace,
                                &square_roots,
                                &orbit_extension,
                            )?;
                            kummer_bits[2 * trace_feature] = halving_bits[0];
                            kummer_bits[2 * trace_feature + 1] = halving_bits[1];
                            for side in 0..2 {
                                let orbit_type = usize::from(orbit_types[side]);
                                marked_halving_orbit_histogram[orbit_type] += 1;
                                let fiber = 2 * trace_feature + side;
                                orbit_bits[3 * fiber] = i8::from(orbit_type <= 1);
                                orbit_bits[3 * fiber + 1] = i8::from(orbit_type == 0);
                                orbit_bits[3 * fiber + 2] = i8::from(orbit_type == 3);
                            }
                            marked_two_isogeny_checks += 1;
                            marked_two_isogeny_point_checks += point_checks;
                        }
                    }
                    let trace_distinguished = trace_residues[0] != trace_residues[1]
                        || trace_residues[2] != trace_residues[3];
                    let invariant_distinguished = invariant_bits[0..2] != invariant_bits[2..4]
                        || invariant_bits[4..6] != invariant_bits[6..8];
                    let kummer_distinguished = kummer_bits[0..2] != kummer_bits[2..4]
                        || kummer_bits[4..6] != kummer_bits[6..8];
                    let orbit_distinguished = orbit_bits[0..6] != orbit_bits[6..12]
                        || orbit_bits[12..18] != orbit_bits[18..24];
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
                    binary_quartic_invariant_distinguishing_targets +=
                        usize::from(invariant_distinguished);
                    marked_two_isogeny_distinguishing_targets +=
                        usize::from(kummer_distinguished);
                    marked_halving_orbit_distinguishing_targets +=
                        usize::from(orbit_distinguished);
                    let index = targets.len();
                    targets.push(Target {
                        slug: spec.slug.to_string(),
                        digit: (scalar % 3) as u8,
                        parity: (scalar % 2) as u8,
                        bits,
                        resultant_bits,
                        trace_residues,
                        invariant_bits,
                        kummer_bits,
                        orbit_bits,
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
            if orbit_panel {
                let marked_halving_orbit_nonabstaining_predicates: usize = targets
                    .iter()
                    .map(|target| target.orbit_bits.iter().filter(|&&bit| bit >= 0).count())
                    .sum();
                let mut rules = Vec::with_capacity(600);
                for first in 0..ORBIT_FEATURES {
                    for second in std::iter::once(None)
                        .chain(((first + 1)..ORBIT_FEATURES).map(Some))
                    {
                        for polarity in 0..=1_u8 {
                            let predictions: Vec<u8> = targets
                                .iter()
                                .map(|target| orbit_prediction(target, first, second, polarity))
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
                if rules.len() != 600 {
                    return Err("marked halving orbit rule census failed".to_string());
                }
                let selected_index = (0..rules.len())
                    .max_by(|&left, &right| {
                        compare_ratio(rules[left].statistic, rules[right].statistic)
                            .then_with(|| rules[right].first.cmp(&rules[left].first))
                            .then_with(|| rules[right].second.cmp(&rules[left].second))
                            .then_with(|| rules[right].polarity.cmp(&rules[left].polarity))
                    })
                    .ok_or("no marked halving orbit rules")?;
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
                        let mut seed = 0x116_023_u64
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
                let selected_second = selected
                    .second
                    .map_or_else(|| "null".to_string(), |value| value.to_string());
                let json = format!(
                    concat!(
                        "{{\n  \"schema\":\"h116-chiral-marked-halving-frobenius-orbit-parity/v1\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"quartic_models\":{},\n",
                        "  \"marked_fibers\":{},\n  \"orbit_predicates\":{},\n  \"rules\":{},\n",
                        "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                        "  \"resultant_checks\":{},\n  \"branch_pair_trace_checks\":{},\n",
                        "  \"marked_halving_orbit_checks\":{},\n",
                        "  \"marked_halving_orbit_character_checks\":{},\n",
                        "  \"marked_halving_orbit_frobenius_point_checks\":{},\n",
                        "  \"marked_halving_orbit_histogram\":[{},{},{},{}],\n",
                        "  \"marked_halving_orbit_nonabstaining_predicates\":{},\n",
                        "  \"marked_halving_orbit_distinguishing_targets\":{},\n",
                        "  \"planted_control_matches\":{},\n  \"selected_first\":{},\n",
                        "  \"selected_second\":{},\n  \"selected_polarity\":{},\n",
                        "  \"coverage\":{},\n  \"matches\":{},\n  \"binary_excess\":{},\n",
                        "  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                        "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n",
                        "  \"replicates\":4096,\n  \"familywise_exceedances\":{},\n",
                        "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                        "  \"status\":\"measured\"\n}}\n"
                    ),
                    targets.len(),
                    rows.len(),
                    TRACE_FEATURES,
                    KUMMER_FEATURES,
                    ORBIT_FEATURES,
                    rules.len(),
                    reconstruction_checks,
                    discriminant_checks,
                    resultant_checks,
                    branch_pair_trace_checks,
                    2 * marked_two_isogeny_checks,
                    2 * marked_two_isogeny_checks,
                    8 * marked_two_isogeny_checks,
                    marked_halving_orbit_histogram[0],
                    marked_halving_orbit_histogram[1],
                    marked_halving_orbit_histogram[2],
                    marked_halving_orbit_histogram[3],
                    marked_halving_orbit_nonabstaining_predicates,
                    marked_halving_orbit_distinguishing_targets,
                    TARGETS,
                    selected.first,
                    selected_second,
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
                    "chiral marked-halving Frobenius orbit parity: first={} second={} polarity={} coverage={} matches={} exact={} p={:.6}",
                    selected.first,
                    selected_second,
                    selected.polarity,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }
            if kummer_panel {
                let marked_two_isogeny_nonabstaining_features: usize = targets
                    .iter()
                    .map(|target| {
                        target.kummer_bits.iter().filter(|&&bit| bit >= 0).count()
                    })
                    .sum();
                let marked_two_isogeny_true_features: usize = targets
                    .iter()
                    .map(|target| {
                        target.kummer_bits.iter().filter(|&&bit| bit == 1).count()
                    })
                    .sum();
                let mut rules = Vec::with_capacity(512);
                for mask in 0_u16..(1_u16 << KUMMER_FEATURES) {
                    for polarity in 0..=1_u8 {
                        let predictions: Vec<u8> = targets
                            .iter()
                            .map(|target| kummer_prediction(target, mask, polarity))
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
                    .ok_or("no marked two-isogeny rules")?;
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
                        let mut seed = 0x116_022_u64
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
                        "{{\n  \"schema\":\"h116-chiral-marked-two-isogeny-halving-parity/v2\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"quartic_models\":{},\n",
                        "  \"halving_features\":{},\n  \"rules\":{},\n",
                        "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                        "  \"resultant_checks\":{},\n  \"branch_pair_trace_checks\":{},\n",
                        "  \"marked_two_isogeny_checks\":{},\n",
                        "  \"marked_two_isogeny_coefficient_checks\":{},\n",
                        "  \"marked_two_isogeny_halving_formula_checks\":{},\n",
                        "  \"marked_two_isogeny_group_order_checks\":{},\n",
                        "  \"marked_two_isogeny_point_checks\":{},\n",
                        "  \"marked_two_isogeny_nonabstaining_features\":{},\n",
                        "  \"marked_two_isogeny_true_features\":{},\n",
                        "  \"marked_two_isogeny_distinguishing_targets\":{},\n",
                        "  \"planted_control_matches\":{},\n  \"selected_mask\":{},\n",
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
                    TRACE_FEATURES,
                    KUMMER_FEATURES,
                    rules.len(),
                    reconstruction_checks,
                    discriminant_checks,
                    resultant_checks,
                    branch_pair_trace_checks,
                    marked_two_isogeny_checks,
                    marked_two_isogeny_checks,
                    2 * marked_two_isogeny_checks,
                    2 * marked_two_isogeny_checks,
                    marked_two_isogeny_point_checks,
                    marked_two_isogeny_nonabstaining_features,
                    marked_two_isogeny_true_features,
                    marked_two_isogeny_distinguishing_targets,
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
                    "chiral marked two-isogeny halving parity: mask={} polarity={} coverage={} matches={} exact={} p={:.6}",
                    selected.mask,
                    selected.polarity,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }
            if invariant_panel {
                let invariant_nonzero_features: usize = targets
                    .iter()
                    .map(|target| {
                        target
                            .invariant_bits
                            .iter()
                            .filter(|&&bit| bit >= 0)
                            .count()
                    })
                    .sum();
                let mut rules = Vec::with_capacity(512);
                for mask in 0_u16..(1_u16 << INVARIANT_FEATURES) {
                    for polarity in 0..=1_u8 {
                        let predictions: Vec<u8> = targets
                            .iter()
                            .map(|target| invariant_prediction(target, mask, polarity))
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
                    .ok_or("no binary-quartic invariant rules")?;
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
                        let mut seed = 0x116_021_u64
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
                        "{{\n  \"schema\":\"h116-chiral-binary-quartic-invariant-parity/v1\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"quartic_models\":{},\n",
                        "  \"invariant_features\":{},\n  \"rules\":{},\n",
                        "  \"reconstruction_checks\":{},\n  \"discriminant_checks\":{},\n",
                        "  \"resultant_checks\":{},\n  \"branch_pair_trace_checks\":{},\n",
                        "  \"binary_quartic_invariant_checks\":{},\n",
                        "  \"binary_quartic_invariant_nonzero_features\":{},\n",
                        "  \"binary_quartic_invariant_distinguishing_targets\":{},\n",
                        "  \"planted_control_matches\":{},\n  \"selected_mask\":{},\n",
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
                    TRACE_FEATURES,
                    INVARIANT_FEATURES,
                    rules.len(),
                    reconstruction_checks,
                    discriminant_checks,
                    resultant_checks,
                    branch_pair_trace_checks,
                    binary_quartic_invariant_checks,
                    invariant_nonzero_features,
                    binary_quartic_invariant_distinguishing_targets,
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
                    "chiral binary-quartic invariant parity: mask={} polarity={} coverage={} matches={} exact={} p={:.6}",
                    selected.mask,
                    selected.polarity,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }
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
        [_, flag, output] if flag == "--run-invariants" => {
            frozen::branch_residual::run_invariants(Path::new(output))
        }
        [_, flag, output] if flag == "--run-kummer" => {
            frozen::branch_residual::run_kummer(Path::new(output))
        }
        [_, flag, output] if flag == "--run-orbits" => {
            frozen::branch_residual::run_orbits(Path::new(output))
        }
        _ => Err(
            "usage: h116_chiral_branch_residual_parity --selftest | --run OUTPUT.json | --run-resultants OUTPUT.json | --run-traces OUTPUT.json | --run-invariants OUTPUT.json | --run-kummer OUTPUT.json | --run-orbits OUTPUT.json".to_string(),
        ),
    };
    if let Err(error) = result {
        eprintln!("h116 chiral branch residual parity error: {error}");
        std::process::exit(1);
    }
}
