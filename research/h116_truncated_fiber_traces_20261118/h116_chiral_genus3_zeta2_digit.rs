#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod chiral_genus3_zeta2 {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const FEATURES: usize = 4;
        const OBSERVABLES: usize = 2;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        const OOS_CURVES: [CurveSpec; 3] = [
            CurveSpec {
                slug: "h116-prym-oos-fp607-a1-b7-r577",
                p: 607,
                a: 1,
                b: 7,
                order: 577,
                trace: 31,
                degree: 3,
            },
            CurveSpec {
                slug: "h116-prym-oos-fp617-a1-b32-r647",
                p: 617,
                a: 1,
                b: 32,
                order: 647,
                trace: -29,
                degree: 3,
            },
            CurveSpec {
                slug: "h116-prym-oos-fp631-a1-b5-r619",
                p: 631,
                a: 1,
                b: 5,
                order: 619,
                trace: 13,
                degree: 3,
            },
        ];

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        struct CoverData {
            s1: i64,
            s2: i64,
            e2: i64,
            prym_trace: i64,
            prym_eta: i64,
            prym_discriminant: i64,
        }

        #[derive(Clone)]
        struct Target {
            slug: String,
            digit: u8,
            parity: u8,
            residues: [[i8; FEATURES]; OBSERVABLES],
            prym_residues: [[i8; FEATURES]; OBSERVABLES],
            splitting_bits: [i8; 2 * FEATURES],
            eigenline_bits: [i8; 2 * FEATURES],
            factor_types: [i8; 2 * FEATURES],
        }

        struct Rule {
            observable: usize,
            code: u8,
            coefficients: [u8; FEATURES],
            offset: u8,
            splitting_mask: u16,
            splitting_polarity: u8,
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

        fn base_function_value(x: u64, y: u64, coefficient: u64, p: u64) -> u64 {
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

        fn binary_statistic(matches: usize, coverage: usize) -> (u128, u128) {
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

        fn prediction(
            target: &Target,
            prym_panel: bool,
            observable: usize,
            coefficients: &[u8; FEATURES],
            offset: u8,
        ) -> u8 {
            let mut value = offset;
            for feature in 0..FEATURES {
                if coefficients[feature] == 0 {
                    continue;
                }
                let residue = if prym_panel {
                    target.prym_residues[observable][feature]
                } else {
                    target.residues[observable][feature]
                };
                if residue < 0 {
                    return 3;
                }
                value = (value + coefficients[feature] * residue as u8) % 3;
            }
            value
        }

        fn splitting_bit(discriminant: i64, auxiliary_prime: i64) -> i8 {
            let residue = discriminant.rem_euclid(auxiliary_prime);
            if residue == 0 {
                -1
            } else if (1..auxiliary_prime)
                .any(|value| value * value % auxiliary_prime == residue)
            {
                1
            } else {
                0
            }
        }

        fn prym_weil_value(
            prym_trace: i64,
            prym_eta: i64,
            p: i64,
            value: i64,
            auxiliary_prime: i64,
        ) -> i64 {
            ((((value - prym_trace) * value + prym_eta) * value - p * prym_trace)
                * value
                + p * p)
                .rem_euclid(auxiliary_prime)
        }

        fn prym_real_weil_value(
            prym_trace: i64,
            prym_eta: i64,
            p: i64,
            value: i64,
            auxiliary_prime: i64,
        ) -> i64 {
            (value * value - prym_trace * value + prym_eta - 2 * p)
                .rem_euclid(auxiliary_prime)
        }

        fn prym_eigenline_bit(
            prym_trace: i64,
            prym_eta: i64,
            p: i64,
            auxiliary_prime: i64,
        ) -> i8 {
            i8::from((0..auxiliary_prime).any(|value| {
                prym_weil_value(prym_trace, prym_eta, p, value, auxiliary_prime) == 0
            }))
        }

        fn trim_poly(poly: &mut Vec<i64>, modulus: i64) {
            for coefficient in poly.iter_mut() {
                *coefficient = coefficient.rem_euclid(modulus);
            }
            while poly.len() > 1 && poly.last() == Some(&0) {
                poly.pop();
            }
        }

        fn poly_mul(left: &[i64], right: &[i64], modulus: i64) -> Vec<i64> {
            let mut product = vec![0_i64; left.len() + right.len() - 1];
            for (left_degree, &left_coefficient) in left.iter().enumerate() {
                for (right_degree, &right_coefficient) in right.iter().enumerate() {
                    let degree = left_degree + right_degree;
                    product[degree] = (product[degree]
                        + left_coefficient * right_coefficient)
                        .rem_euclid(modulus);
                }
            }
            trim_poly(&mut product, modulus);
            product
        }

        fn poly_eval(poly: &[i64], value: i64, modulus: i64) -> i64 {
            poly.iter().rev().fold(0_i64, |accumulator, &coefficient| {
                (accumulator * value + coefficient).rem_euclid(modulus)
            })
        }

        fn poly_div_monic(
            dividend: &[i64],
            divisor: &[i64],
            modulus: i64,
        ) -> Option<Vec<i64>> {
            if divisor.len() <= 1 || divisor.last()?.rem_euclid(modulus) != 1 {
                return None;
            }
            let mut remainder = dividend.to_vec();
            trim_poly(&mut remainder, modulus);
            if remainder.len() < divisor.len() {
                return None;
            }
            let mut quotient = vec![0_i64; remainder.len() - divisor.len() + 1];
            while remainder.len() >= divisor.len() && remainder != [0] {
                let shift = remainder.len() - divisor.len();
                let leading = *remainder.last()?;
                quotient[shift] = leading;
                for (degree, &coefficient) in divisor.iter().enumerate() {
                    let index = shift + degree;
                    remainder[index] =
                        (remainder[index] - leading * coefficient).rem_euclid(modulus);
                }
                trim_poly(&mut remainder, modulus);
            }
            if remainder == [0] {
                trim_poly(&mut quotient, modulus);
                Some(quotient)
            } else {
                None
            }
        }

        fn monic_polynomials(degree: usize, modulus: i64) -> Vec<Vec<i64>> {
            let count = (0..degree).fold(1_i64, |accumulator, _| accumulator * modulus);
            let mut polynomials = Vec::with_capacity(count as usize);
            for mut code in 0..count {
                let mut polynomial = Vec::with_capacity(degree + 1);
                for _ in 0..degree {
                    polynomial.push(code % modulus);
                    code /= modulus;
                }
                polynomial.push(1);
                polynomials.push(polynomial);
            }
            polynomials
        }

        fn poly_is_irreducible(poly: &[i64], modulus: i64) -> bool {
            let degree = poly.len() - 1;
            if degree <= 1 {
                return true;
            }
            for divisor_degree in 1..=degree / 2 {
                for divisor in monic_polynomials(divisor_degree, modulus) {
                    if poly_div_monic(poly, &divisor, modulus).is_some() {
                        return false;
                    }
                }
            }
            true
        }

        fn prym_weil_polynomial(
            prym_trace: i64,
            prym_eta: i64,
            p: i64,
            auxiliary_prime: i64,
        ) -> Vec<i64> {
            vec![p * p, -p * prym_trace, prym_eta, -prym_trace, 1]
                .into_iter()
                .map(|coefficient| coefficient.rem_euclid(auxiliary_prime))
                .collect()
        }

        fn prym_factor_type(
            prym_trace: i64,
            prym_eta: i64,
            p: i64,
            auxiliary_prime: i64,
        ) -> Result<(i8, usize), String> {
            let original =
                prym_weil_polynomial(prym_trace, prym_eta, p, auxiliary_prime);
            let mut remaining = original.clone();
            let mut factors = Vec::<Vec<i64>>::new();
            while remaining.len() > 1 {
                if poly_is_irreducible(&remaining, auxiliary_prime) {
                    factors.push(remaining);
                    break;
                }
                let degree = remaining.len() - 1;
                let mut found = None;
                'candidate: for divisor_degree in 1..=degree / 2 {
                    for divisor in monic_polynomials(divisor_degree, auxiliary_prime) {
                        if !poly_is_irreducible(&divisor, auxiliary_prime) {
                            continue;
                        }
                        if let Some(quotient) =
                            poly_div_monic(&remaining, &divisor, auxiliary_prime)
                        {
                            found = Some((divisor, quotient));
                            break 'candidate;
                        }
                    }
                }
                let (factor, quotient) =
                    found.ok_or("Prym quartic factorization failed")?;
                factors.push(factor);
                remaining = quotient;
            }
            let mut reconstructed = vec![1_i64];
            for factor in &factors {
                if !poly_is_irreducible(factor, auxiliary_prime) {
                    return Err("Prym factor irreducibility control failed".to_string());
                }
                reconstructed = poly_mul(&reconstructed, factor, auxiliary_prime);
            }
            if reconstructed != original {
                return Err("Prym factor reconstruction control failed".to_string());
            }
            let mut degrees: Vec<usize> = factors.iter().map(|factor| factor.len() - 1).collect();
            degrees.sort_unstable();
            let factor_type = match degrees.as_slice() {
                [1, 1, 1, 1] => 0,
                [1, 1, 2] => 1,
                [1, 3] => 2,
                [2, 2] => 3,
                [4] => 4,
                _ => return Err("unexpected Prym quartic degree partition".to_string()),
            };
            Ok((factor_type, factors.len()))
        }

        fn transport_predictions(
            labels: &[u8],
            group_ids: &[usize],
            curve_ids: &[usize],
            group_count: usize,
            curve_count: usize,
        ) -> Vec<u8> {
            let mut global = vec![[0_usize; 2]; group_count];
            let mut local = vec![[0_usize; 2]; group_count * curve_count];
            for ((&label, &group), &curve) in labels.iter().zip(group_ids).zip(curve_ids) {
                global[group][label as usize] += 1;
                local[curve * group_count + group][label as usize] += 1;
            }
            labels
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let group = group_ids[index];
                    let curve = curve_ids[index];
                    let zeroes = global[group][0] - local[curve * group_count + group][0];
                    let ones = global[group][1] - local[curve * group_count + group][1];
                    match zeroes.cmp(&ones) {
                        Ordering::Greater => 0,
                        Ordering::Less => 1,
                        Ordering::Equal => 3,
                    }
                })
                .collect()
        }

        fn binary_prediction(
            target: &Target,
            eigenline_panel: bool,
            mask: u16,
            polarity: u8,
        ) -> u8 {
            let mut bit = polarity;
            for feature in 0..(2 * FEATURES) {
                if mask & (1_u16 << feature) == 0 {
                    continue;
                }
                let value = if eigenline_panel {
                    target.eigenline_bits[feature]
                } else {
                    target.splitting_bits[feature]
                };
                if value < 0 {
                    return 3;
                }
                bit ^= value as u8;
            }
            bit
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

        fn observable_name(observable: usize, prym_panel: bool) -> &'static str {
            match (prym_panel, observable) {
                (false, 0) => "s2",
                (false, 1) => "e2",
                (true, 0) => "prym_eta",
                (true, 1) => "prym_real_weil_discriminant",
                _ => unreachable!(),
            }
        }

        fn record_extension_point(
            field: &Field,
            x: Felt,
            y: Felt,
            histograms: &mut [Vec<u32>],
            rescale_checks: &mut usize,
        ) -> Result<(), String> {
            let coefficients = [field.one(), field.neg(field.one())];
            for (coefficient_index, coefficient) in coefficients.into_iter().enumerate() {
                let value = field.add(x, field.mul(coefficient, y));
                let value_index = field.to_index(value) as usize;
                histograms[coefficient_index][value_index] = histograms[coefficient_index]
                    [value_index]
                    .checked_add(1)
                    .ok_or("extension histogram overflow")?;

                for scale in [2_u64, 3] {
                    let scale_sq = mul_mod_u64(scale, scale, field.p);
                    let scale_cu = mul_mod_u64(scale_sq, scale, field.p);
                    let scale_inv = pow_mod_u64(scale, field.p - 2, field.p);
                    let scaled_coefficient = field.mul(coefficient, field.from_base(scale_inv));
                    let scaled_x = field.mul(field.from_base(scale_sq), x);
                    let scaled_y = field.mul(field.from_base(scale_cu), y);
                    let scaled_value =
                        field.add(scaled_x, field.mul(scaled_coefficient, scaled_y));
                    let expected = field.mul(field.from_base(scale_sq), value);
                    if scaled_value != expected {
                        return Err("extension coordinate-rescaling identity failed".to_string());
                    }
                    *rescale_checks += 1;
                }
            }
            Ok(())
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(OBSERVABLES * 3_usize.pow((FEATURES + 1) as u32), 486);
            assert_eq!(decode_coefficients(0), [0, 0, 0, 0]);
            assert_eq!(decode_coefficients(80), [2, 2, 2, 2]);
            assert_eq!(legendre(2, 7), 1);
            assert_eq!(legendre(3, 7), -1);
            assert_eq!(statistic(5, 9), (36, 9));
            let s1 = 5_i64;
            let s2 = -3_i64;
            assert_eq!((s1 * s1 - s2) / 2, 14);
            let planted = Target {
                slug: "control".to_string(),
                digit: 1,
                parity: 1,
                residues: [[1, 0, 0, 0], [2, 0, 0, 0]],
                prym_residues: [[1, 0, 0, 0], [2, 0, 0, 0]],
                splitting_bits: [1, 0, 0, 0, 0, 0, 0, 0],
                eigenline_bits: [0, 1, 0, 0, 0, 0, 0, 0],
                factor_types: [0, 1, 2, 3, 4, 0, 1, 2],
            };
            assert_eq!(prediction(&planted, false, 0, &[1, 0, 0, 0], 0), 1);
            assert_eq!(prediction(&planted, false, 1, &[2, 0, 0, 0], 0), 1);
            assert_eq!(prediction(&planted, true, 0, &[1, 0, 0, 0], 0), 1);
            assert_eq!(binary_prediction(&planted, false, 1, 0), 1);
            assert_eq!(binary_prediction(&planted, true, 2, 0), 1);
            assert_eq!(splitting_bit(4, 5), 1);
            assert_eq!(splitting_bit(2, 5), 0);
            assert_eq!(splitting_bit(0, 5), -1);
            assert_eq!(prym_weil_value(0, 0, 1, 1, 5), 2);
            assert_eq!(prym_eigenline_bit(0, 2, 1, 5), 1);
            assert_eq!(poly_mul(&[1, 1], &[4, 1], 5), vec![4, 0, 1]);
            assert_eq!(poly_div_monic(&[4, 0, 1], &[1, 1], 5), Some(vec![4, 1]));
            assert!(poly_is_irreducible(&[2, 0, 1], 5));
            assert_eq!(prym_factor_type(0, 2, 1, 5)?, (0, 4));
            println!(
                "h116 chiral genus-three zeta2 digit selftest: pass ternary_rules=486 binary_rules=512"
            );
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, false, false, false, false, false)
        }

        pub fn run_prym(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false, false, false, false)
        }

        pub fn run_prym_oos(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, true, false, false, false)
        }

        pub fn run_prym_splitting(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false, true, false, false)
        }

        pub fn run_prym_eigenline(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false, false, true, false)
        }

        pub fn run_prym_factor_type(output_path: &Path) -> Result<(), String> {
            run_with_panel(output_path, true, false, false, false, true)
        }

        fn run_with_panel(
            output_path: &Path,
            prym_panel: bool,
            oos_panel: bool,
            splitting_panel: bool,
            eigenline_panel: bool,
            factorization_panel: bool,
        ) -> Result<(), String> {
            if usize::from(splitting_panel)
                + usize::from(eigenline_panel)
                + usize::from(factorization_panel)
                > 1
            {
                return Err("specialized panel flags are mutually exclusive".to_string());
            }
            let binary_panel = splitting_panel || eigenline_panel;
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut extension_square_table_checks = 0_u64;
            let mut extension_curve_census_checks = 0_usize;
            let mut extension_fixed_point_checks = 0_usize;
            let mut extension_affine_points = 0_u64;
            let mut extension_histogram_checks = 0_usize;
            let mut extension_rescale_checks = 0_usize;
            let mut smooth_cover_levels = 0_usize;
            let mut smooth_feature_checks = 0_usize;
            let mut base_trace_checks = 0_usize;
            let mut extension_trace_checks = 0_usize;
            let mut newton_checks = 0_usize;
            let mut chirality_swap_checks = 0_usize;
            let mut s2_distinguishing_targets = 0_usize;
            let mut e2_distinguishing_targets = 0_usize;
            let mut prym_eta_distinguishing_targets = 0_usize;
            let mut prym_discriminant_distinguishing_targets = 0_usize;
            let mut prym_factor_checks = 0_usize;
            let mut prym_splitting_checks = 0_usize;
            let mut prym_splitting_distinguishing_targets = 0_usize;
            let mut prym_eigenline_polynomial_checks = 0_usize;
            let mut prym_eigenline_quotient_checks = 0_usize;
            let mut prym_eigenline_distinguishing_targets = 0_usize;
            let mut prym_factor_type_checks = 0_usize;
            let mut prym_factor_irreducibility_checks = 0_usize;
            let mut prym_factor_evaluation_checks = 0_usize;
            let mut prym_factor_type_distinguishing_targets = 0_usize;
            let mut prym_factor_type_histogram = [0_usize; 5];

            let curve_specs: &[CurveSpec] = if oos_panel { &OOS_CURVES } else { &CURVES };
            for &spec in curve_specs {
                let base_field = Field::new(spec.p, 1, &[0, 1])?;
                let base_curve = Curve {
                    field: &base_field,
                    a: base_field.from_base(spec.a),
                    b: base_field.from_base(spec.b),
                };
                let generator = first_base_point(&base_curve, spec.p)?;
                if generator == Point::Infinity
                    || base_curve.scalar_mul(spec.order, generator)? != Point::Infinity
                {
                    return Err("generator/order gate failed".to_string());
                }
                let mut base_points = Vec::new();
                for scalar in 1..spec.order {
                    base_points.push(point_base_coordinates(
                        &base_field,
                        base_curve.scalar_mul(scalar, generator)?,
                    )?);
                }
                if base_points.len() + 1 != spec.order as usize {
                    return Err("base point census failed".to_string());
                }

                let modulus = first_irreducible(spec.p, 2)?;
                let field = Field::new(spec.p, 2, &modulus)?;
                let curve = Curve {
                    field: &field,
                    a: field.from_base(spec.a),
                    b: field.from_base(spec.b),
                };
                let q = field.q;
                let sentinel = q;
                let mut square_roots = vec![sentinel; q as usize];
                let mut character = vec![-1_i8; q as usize];
                for index in 0..q {
                    let value = field.from_index(index);
                    let square_index = field.to_index(field.square(value)) as usize;
                    if square_roots[square_index] == sentinel {
                        square_roots[square_index] = index;
                    }
                    character[square_index] = if square_index == 0 { 0 } else { 1 };
                }
                let residue_count = character.iter().filter(|&&value| value == 1).count() as u64;
                let zero_count = character.iter().filter(|&&value| value == 0).count();
                if residue_count != (q - 1) / 2 || zero_count != 1 {
                    return Err("extension quadratic-character census failed".to_string());
                }
                extension_square_table_checks += q;

                let mut histograms = vec![vec![0_u32; q as usize]; 2];
                let mut affine_count = 0_u64;
                let mut fixed_count = 1_u64;
                for x_index in 0..q {
                    let x = field.from_index(x_index);
                    let rhs = field.add(
                        field.add(
                            field.mul(field.square(x), x),
                            field.mul(curve.a, x),
                        ),
                        curve.b,
                    );
                    let root_index = square_roots[field.to_index(rhs) as usize];
                    if root_index == sentinel {
                        continue;
                    }
                    let y = field.from_index(root_index);
                    record_extension_point(
                        &field,
                        x,
                        y,
                        &mut histograms,
                        &mut extension_rescale_checks,
                    )?;
                    affine_count += 1;
                    fixed_count += u64::from(field.is_base(x) && field.is_base(y));
                    if y != field.zero() {
                        let negative_y = field.neg(y);
                        record_extension_point(
                            &field,
                            x,
                            negative_y,
                            &mut histograms,
                            &mut extension_rescale_checks,
                        )?;
                        affine_count += 1;
                        fixed_count +=
                            u64::from(field.is_base(x) && field.is_base(negative_y));
                    }
                }
                let expected_order = trace_group_order(spec.p, spec.trace, 2)?;
                if affine_count + 1 != expected_order {
                    return Err("extension curve census failed".to_string());
                }
                if fixed_count != spec.order {
                    return Err("extension Frobenius-fixed census failed".to_string());
                }
                extension_curve_census_checks += 1;
                extension_fixed_point_checks += 1;
                extension_affine_points += affine_count;
                for histogram in &histograms {
                    if histogram.iter().map(|&count| u64::from(count)).sum::<u64>()
                        != affine_count
                    {
                        return Err("extension function histogram census failed".to_string());
                    }
                    extension_histogram_checks += 1;
                }

                let coefficients = [1_u64, spec.p - 1];
                let mut data_tables = Vec::new();
                let elliptic_s2 = spec.trace * spec.trace - 2 * spec.p as i64;
                for (coefficient_index, coefficient) in coefficients.into_iter().enumerate() {
                    let base_values: Vec<u64> = base_points
                        .iter()
                        .map(|&(x, y)| base_function_value(x, y, coefficient, spec.p))
                        .collect();
                    let mut table = vec![None; spec.p as usize];
                    for level in 0..spec.p {
                        if !smooth_level(spec, coefficient, level) {
                            continue;
                        }
                        let sum1: i64 = base_values
                            .iter()
                            .map(|&value| i64::from(legendre(sub(value, level, spec.p), spec.p)))
                            .sum();
                        let s1 = spec.trace - sum1;
                        let count1 = spec.order as i64 + sum1;
                        if spec.p as i64 + 1 - count1 != s1 || s1 * s1 > 36 * spec.p as i64 {
                            return Err("base cover trace control failed".to_string());
                        }
                        base_trace_checks += 1;

                        let mut sum2 = 0_i64;
                        for (index, &multiplicity) in
                            histograms[coefficient_index].iter().enumerate()
                        {
                            if multiplicity == 0 {
                                continue;
                            }
                            let base_coordinate = index as u64 % spec.p;
                            let shifted_base = sub(base_coordinate, level, spec.p);
                            let shifted_index = index as u64 - base_coordinate + shifted_base;
                            sum2 += i64::from(multiplicity)
                                * i64::from(character[shifted_index as usize]);
                        }
                        let s2 = elliptic_s2 - sum2;
                        let count2 = expected_order as i64 + sum2;
                        if q as i64 + 1 - count2 != s2
                            || s2 * s2 > 36 * spec.p as i64 * spec.p as i64
                        {
                            return Err("extension cover trace control failed".to_string());
                        }
                        extension_trace_checks += 1;

                        let numerator = s1 * s1 - s2;
                        if numerator % 2 != 0 {
                            return Err("Newton integrality control failed".to_string());
                        }
                        let e2 = numerator / 2;
                        if e2 * e2 > 225 * spec.p as i64 * spec.p as i64 {
                            return Err("second zeta coefficient bound failed".to_string());
                        }
                        let prym_trace = s1 - spec.trace;
                        let prym_eta =
                            e2 - spec.p as i64 - spec.trace * prym_trace;
                        let prym_discriminant = prym_trace * prym_trace
                            - 4 * (prym_eta - 2 * spec.p as i64);
                        if s1 != spec.trace + prym_trace
                            || e2 != spec.p as i64 + spec.trace * prym_trace + prym_eta
                        {
                            return Err("Prym factor coefficient identity failed".to_string());
                        }
                        if prym_trace * prym_trace > 16 * spec.p as i64
                            || prym_eta * prym_eta
                                > 36 * spec.p as i64 * spec.p as i64
                            || prym_discriminant < 0
                            || prym_discriminant > 16 * spec.p as i64
                        {
                            return Err("Prym real-Weil bound failed".to_string());
                        }
                        for auxiliary_prime in [5_i64, 7] {
                            let roots = (0..auxiliary_prime)
                                .filter(|&value| {
                                    (value * value - prym_trace * value
                                        + prym_eta - 2 * spec.p as i64)
                                        .rem_euclid(auxiliary_prime)
                                        == 0
                                })
                                .count();
                            let bit = splitting_bit(prym_discriminant, auxiliary_prime);
                            let expected_roots = match bit {
                                -1 => 1,
                                0 => 0,
                                1 => 2,
                                _ => unreachable!(),
                            };
                            if roots != expected_roots {
                                return Err("Prym real-Weil splitting control failed".to_string());
                            }
                            prym_splitting_checks += 1;
                            if spec.p as i64 % auxiliary_prime == 0 {
                                return Err("auxiliary prime equals curve characteristic".to_string());
                            }
                            let full_roots = (0..auxiliary_prime)
                                .filter(|&value| {
                                    prym_weil_value(
                                        prym_trace,
                                        prym_eta,
                                        spec.p as i64,
                                        value,
                                        auxiliary_prime,
                                    ) == 0
                                })
                                .count();
                            let eigenline_bit = prym_eigenline_bit(
                                prym_trace,
                                prym_eta,
                                spec.p as i64,
                                auxiliary_prime,
                            );
                            if (full_roots > 0) != (eigenline_bit == 1) {
                                return Err("Prym eigenline root control failed".to_string());
                            }
                            prym_eigenline_polynomial_checks += 1;
                            for eigenvalue in 1..auxiliary_prime {
                                let inverse = (1..auxiliary_prime)
                                    .find(|candidate| {
                                        (eigenvalue * candidate).rem_euclid(auxiliary_prime) == 1
                                    })
                                    .ok_or("auxiliary eigenvalue inverse missing")?;
                                let real_eigenvalue = (eigenvalue
                                    + (spec.p as i64).rem_euclid(auxiliary_prime) * inverse)
                                    .rem_euclid(auxiliary_prime);
                                let full_value = prym_weil_value(
                                    prym_trace,
                                    prym_eta,
                                    spec.p as i64,
                                    eigenvalue,
                                    auxiliary_prime,
                                );
                                let quotient_value = (eigenvalue * eigenvalue
                                    * prym_real_weil_value(
                                        prym_trace,
                                        prym_eta,
                                        spec.p as i64,
                                        real_eigenvalue,
                                        auxiliary_prime,
                                    ))
                                    .rem_euclid(auxiliary_prime);
                                if full_value != quotient_value {
                                    return Err(
                                        "Prym full/real Weil quotient identity failed".to_string(),
                                    );
                                }
                                prym_eigenline_quotient_checks += 1;
                            }
                            let polynomial = prym_weil_polynomial(
                                prym_trace,
                                prym_eta,
                                spec.p as i64,
                                auxiliary_prime,
                            );
                            for value in 0..auxiliary_prime {
                                if poly_eval(&polynomial, value, auxiliary_prime)
                                    != prym_weil_value(
                                        prym_trace,
                                        prym_eta,
                                        spec.p as i64,
                                        value,
                                        auxiliary_prime,
                                    )
                                {
                                    return Err(
                                        "Prym generic/specialized evaluation control failed"
                                            .to_string(),
                                    );
                                }
                                prym_factor_evaluation_checks += 1;
                            }
                            let (factor_type, irreducible_factors) = prym_factor_type(
                                prym_trace,
                                prym_eta,
                                spec.p as i64,
                                auxiliary_prime,
                            )?;
                            prym_factor_type_histogram[factor_type as usize] += 1;
                            prym_factor_irreducibility_checks += irreducible_factors;
                            prym_factor_type_checks += 1;
                        }
                        prym_factor_checks += 1;
                        newton_checks += 1;
                        smooth_cover_levels += 1;
                        table[level as usize] = Some(CoverData {
                            s1,
                            s2,
                            e2,
                            prym_trace,
                            prym_eta,
                            prym_discriminant,
                        });
                    }
                    data_tables.push(table);
                }

                for scalar in 1..spec.order {
                    let target_point = base_curve.scalar_mul(scalar, generator)?;
                    let mut residues = [[-1_i8; FEATURES]; OBSERVABLES];
                    let mut prym_residues = [[-1_i8; FEATURES]; OBSERVABLES];
                    let mut splitting_bits = [-1_i8; 2 * FEATURES];
                    let mut eigenline_bits = [-1_i8; 2 * FEATURES];
                    let mut factor_types = [-1_i8; 2 * FEATURES];
                    let mut s2_distinguished = false;
                    let mut e2_distinguished = false;
                    let mut prym_eta_distinguished = false;
                    let mut prym_discriminant_distinguished = false;
                    let mut prym_splitting_distinguished = false;
                    let mut prym_eigenline_distinguished = false;
                    let mut prym_factor_type_distinguished = false;
                    for multiplier in 1..=2_u64 {
                        let multiple = base_curve.scalar_mul(multiplier, target_point)?;
                        let (x, y) = point_base_coordinates(&base_field, multiple)?;
                        let plus_level = base_function_value(x, y, 1, spec.p);
                        let minus_level = base_function_value(x, y, spec.p - 1, spec.p);
                        let plus_data = data_tables[0][plus_level as usize];
                        let minus_data = data_tables[1][minus_level as usize];
                        let feature = 2 * (multiplier as usize - 1);
                        if let Some(data) = plus_data {
                            residues[0][feature] = data.s2.rem_euclid(3) as i8;
                            residues[1][feature] = data.e2.rem_euclid(3) as i8;
                            prym_residues[0][feature] = data.prym_eta.rem_euclid(3) as i8;
                            prym_residues[1][feature] =
                                data.prym_discriminant.rem_euclid(3) as i8;
                            for (auxiliary_index, auxiliary_prime) in
                                [5_i64, 7].into_iter().enumerate()
                            {
                                splitting_bits[2 * feature + auxiliary_index] =
                                    splitting_bit(data.prym_discriminant, auxiliary_prime);
                                eigenline_bits[2 * feature + auxiliary_index] =
                                    prym_eigenline_bit(
                                        data.prym_trace,
                                        data.prym_eta,
                                        spec.p as i64,
                                        auxiliary_prime,
                                    );
                                factor_types[2 * feature + auxiliary_index] = prym_factor_type(
                                    data.prym_trace,
                                    data.prym_eta,
                                    spec.p as i64,
                                    auxiliary_prime,
                                )?
                                .0;
                            }
                            smooth_feature_checks += 1;
                        }
                        if let Some(data) = minus_data {
                            residues[0][feature + 1] = data.s2.rem_euclid(3) as i8;
                            residues[1][feature + 1] = data.e2.rem_euclid(3) as i8;
                            prym_residues[0][feature + 1] =
                                data.prym_eta.rem_euclid(3) as i8;
                            prym_residues[1][feature + 1] =
                                data.prym_discriminant.rem_euclid(3) as i8;
                            for (auxiliary_index, auxiliary_prime) in
                                [5_i64, 7].into_iter().enumerate()
                            {
                                splitting_bits[2 * (feature + 1) + auxiliary_index] =
                                    splitting_bit(data.prym_discriminant, auxiliary_prime);
                                eigenline_bits[2 * (feature + 1) + auxiliary_index] =
                                    prym_eigenline_bit(
                                        data.prym_trace,
                                        data.prym_eta,
                                        spec.p as i64,
                                        auxiliary_prime,
                                    );
                                factor_types[2 * (feature + 1) + auxiliary_index] =
                                    prym_factor_type(
                                        data.prym_trace,
                                        data.prym_eta,
                                        spec.p as i64,
                                        auxiliary_prime,
                                    )?
                                    .0;
                            }
                            smooth_feature_checks += 1;
                        }
                        if let (Some(plus), Some(minus)) = (plus_data, minus_data) {
                            s2_distinguished |= plus.s2.rem_euclid(3) != minus.s2.rem_euclid(3);
                            e2_distinguished |= plus.e2.rem_euclid(3) != minus.e2.rem_euclid(3);
                            prym_eta_distinguished |=
                                plus.prym_eta.rem_euclid(3) != minus.prym_eta.rem_euclid(3);
                            prym_discriminant_distinguished |= plus
                                .prym_discriminant
                                .rem_euclid(3)
                                != minus.prym_discriminant.rem_euclid(3);
                            for auxiliary_prime in [5_i64, 7] {
                                prym_splitting_distinguished |=
                                    splitting_bit(
                                        plus.prym_discriminant,
                                        auxiliary_prime,
                                    ) != splitting_bit(
                                        minus.prym_discriminant,
                                        auxiliary_prime,
                                    );
                                prym_eigenline_distinguished |= prym_eigenline_bit(
                                    plus.prym_trace,
                                    plus.prym_eta,
                                    spec.p as i64,
                                    auxiliary_prime,
                                ) != prym_eigenline_bit(
                                    minus.prym_trace,
                                    minus.prym_eta,
                                    spec.p as i64,
                                    auxiliary_prime,
                                );
                                prym_factor_type_distinguished |= prym_factor_type(
                                    plus.prym_trace,
                                    plus.prym_eta,
                                    spec.p as i64,
                                    auxiliary_prime,
                                )?
                                .0 != prym_factor_type(
                                    minus.prym_trace,
                                    minus.prym_eta,
                                    spec.p as i64,
                                    auxiliary_prime,
                                )?
                                .0;
                            }
                        }

                        let swapped_plus = data_tables[0][minus_level as usize];
                        let swapped_minus = data_tables[1][minus_level as usize];
                        if swapped_plus != swapped_minus {
                            return Err("second-zeta chirality swap failed".to_string());
                        }
                        chirality_swap_checks += 1;
                    }
                    s2_distinguishing_targets += usize::from(s2_distinguished);
                    e2_distinguishing_targets += usize::from(e2_distinguished);
                    prym_eta_distinguishing_targets += usize::from(prym_eta_distinguished);
                    prym_discriminant_distinguishing_targets +=
                        usize::from(prym_discriminant_distinguished);
                    prym_splitting_distinguishing_targets +=
                        usize::from(prym_splitting_distinguished);
                    prym_eigenline_distinguishing_targets +=
                        usize::from(prym_eigenline_distinguished);
                    prym_factor_type_distinguishing_targets +=
                        usize::from(prym_factor_type_distinguished);
                    let index = targets.len();
                    targets.push(Target {
                        slug: spec.slug.to_string(),
                        digit: (scalar % 3) as u8,
                        parity: (scalar % 2) as u8,
                        residues,
                        prym_residues,
                        splitting_bits,
                        eigenline_bits,
                        factor_types,
                    });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }

            let expected_targets = if oos_panel { 1840 } else { TARGETS };
            let expected_curves = if oos_panel { OOS_CURVES.len() } else { CURVES.len() };
            if targets.len() != expected_targets || rows.len() != expected_curves {
                return Err("target census failed".to_string());
            }
            if chirality_swap_checks != 2 * expected_targets {
                return Err("chirality swap census failed".to_string());
            }
            let labels: Vec<u8> = if binary_panel || factorization_panel {
                targets.iter().map(|target| target.parity).collect()
            } else {
                targets.iter().map(|target| target.digit).collect()
            };
            if labels.iter().zip(&labels).filter(|(left, right)| left == right).count()
                != expected_targets
            {
                return Err("planted scorer control failed".to_string());
            }

            if factorization_panel {
                let mut group_map = BTreeMap::<[i8; 2 * FEATURES], usize>::new();
                let mut group_ids = Vec::with_capacity(expected_targets);
                for target in &targets {
                    let next_group = group_map.len();
                    let group = *group_map
                        .entry(target.factor_types)
                        .or_insert(next_group);
                    group_ids.push(group);
                }
                let group_count = group_map.len();
                let curve_map: BTreeMap<String, usize> = rows
                    .keys()
                    .enumerate()
                    .map(|(index, slug)| (slug.clone(), index))
                    .collect();
                let curve_ids: Vec<usize> = targets
                    .iter()
                    .map(|target| curve_map[&target.slug])
                    .collect();

                let mut group_label_counts = vec![[0_usize; 2]; group_count];
                for (&group, &label) in group_ids.iter().zip(&labels) {
                    group_label_counts[group][label as usize] += 1;
                }
                let conflicting_groups = group_label_counts
                    .iter()
                    .filter(|counts| counts[0] > 0 && counts[1] > 0)
                    .count();
                let conflicting_targets: usize = group_label_counts
                    .iter()
                    .filter(|counts| counts[0] > 0 && counts[1] > 0)
                    .map(|counts| counts[0] + counts[1])
                    .sum();
                let in_sample_lookup_matches: usize = group_label_counts
                    .iter()
                    .map(|counts| counts[0].max(counts[1]))
                    .sum();

                let predictions = transport_predictions(
                    &labels,
                    &group_ids,
                    &curve_ids,
                    group_count,
                    expected_curves,
                );
                let (coverage, matches) = score(&predictions, &labels);
                let selected_statistic = binary_statistic(matches, coverage);
                let total_exact_rules =
                    usize::from(coverage == expected_targets && matches == expected_targets);
                let covered_exact_rules = usize::from(coverage > 0 && matches == coverage);

                let mut exceedances = 0_usize;
                for replicate in 0..REPLICATES {
                    let mut shuffled = vec![0_u8; expected_targets];
                    for (slug, indices) in &rows {
                        let mut values: Vec<u8> =
                            indices.iter().map(|&index| labels[index]).collect();
                        let mut seed = 0x116_017_u64
                            ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                            ^ slug.bytes().map(u64::from).sum::<u64>();
                        shuffle(&mut values, &mut seed);
                        for (&index, value) in indices.iter().zip(values) {
                            shuffled[index] = value;
                        }
                    }
                    let shuffled_predictions = transport_predictions(
                        &shuffled,
                        &group_ids,
                        &curve_ids,
                        group_count,
                        expected_curves,
                    );
                    let (shuffled_coverage, shuffled_matches) =
                        score(&shuffled_predictions, &shuffled);
                    let shuffled_statistic =
                        binary_statistic(shuffled_matches, shuffled_coverage);
                    exceedances += usize::from(
                        compare_ratio(shuffled_statistic, selected_statistic)
                            != Ordering::Less,
                    );
                }

                let mut per_curve = Vec::new();
                for (slug, indices) in &rows {
                    let curve_predictions: Vec<u8> = indices
                        .iter()
                        .map(|&index| predictions[index])
                        .collect();
                    let curve_labels: Vec<u8> =
                        indices.iter().map(|&index| labels[index]).collect();
                    let (curve_coverage, curve_matches) =
                        score(&curve_predictions, &curve_labels);
                    per_curve.push(format!(
                        "{{\"slug\":\"{slug}\",\"targets\":{},\"coverage\":{curve_coverage},\"matches\":{curve_matches}}}",
                        indices.len()
                    ));
                }

                let excess = 2_i64 * matches as i64 - coverage as i64;
                let json = format!(
                    concat!(
                        "{{\n  \"schema\":\"h116-prym-frobenius-factor-type/v1\",\n",
                        "  \"panel\":\"prym_factor_type_transport\",\n",
                        "  \"targets\":{},\n  \"curves\":{},\n  \"rules\":1,\n",
                        "  \"extension_square_table_checks\":{},\n",
                        "  \"extension_curve_census_checks\":{},\n  \"extension_fixed_point_checks\":{},\n",
                        "  \"extension_affine_points\":{},\n  \"extension_histogram_checks\":{},\n",
                        "  \"extension_rescale_checks\":{},\n  \"smooth_cover_levels\":{},\n",
                        "  \"smooth_feature_checks\":{},\n  \"base_trace_checks\":{},\n",
                        "  \"extension_trace_checks\":{},\n  \"newton_checks\":{},\n",
                        "  \"chirality_swap_checks\":{},\n  \"prym_factor_checks\":{},\n",
                        "  \"prym_splitting_checks\":{},\n  \"prym_eigenline_polynomial_checks\":{},\n",
                        "  \"prym_eigenline_quotient_checks\":{},\n",
                        "  \"prym_factor_type_checks\":{},\n",
                        "  \"prym_factor_irreducibility_checks\":{},\n",
                        "  \"prym_factor_evaluation_checks\":{},\n",
                        "  \"prym_factor_type_histogram\":[{},{},{},{},{}],\n",
                        "  \"prym_factor_type_codebook\":[\"1111\",\"112\",\"13\",\"22\",\"4\"],\n",
                        "  \"prym_factor_type_distinguishing_targets\":{},\n",
                        "  \"factor_type_groups\":{},\n  \"conflicting_groups\":{},\n",
                        "  \"conflicting_targets\":{},\n",
                        "  \"in_sample_lookup_matches\":{},\n",
                        "  \"in_sample_lookup_exact\":{},\n",
                        "  \"selected_observable\":\"complete_quartic_factor_degree_tuple_l5_l7\",\n",
                        "  \"transport\":\"leave_one_curve_out_exact_tuple_majority\",\n",
                        "  \"radix\":2,\n  \"coverage\":{},\n  \"matches\":{},\n",
                        "  \"radix_excess\":{},\n  \"statistic_numerator\":{},\n",
                        "  \"statistic_denominator\":{},\n",
                        "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n",
                        "  \"replicates\":4096,\n  \"shuffle_exceedances\":{},\n",
                        "  \"shuffle_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                        "  \"planted_control_matches\":{},\n  \"status\":\"measured\"\n}}\n"
                    ),
                    targets.len(),
                    rows.len(),
                    extension_square_table_checks,
                    extension_curve_census_checks,
                    extension_fixed_point_checks,
                    extension_affine_points,
                    extension_histogram_checks,
                    extension_rescale_checks,
                    smooth_cover_levels,
                    smooth_feature_checks,
                    base_trace_checks,
                    extension_trace_checks,
                    newton_checks,
                    chirality_swap_checks,
                    prym_factor_checks,
                    prym_splitting_checks,
                    prym_eigenline_polynomial_checks,
                    prym_eigenline_quotient_checks,
                    prym_factor_type_checks,
                    prym_factor_irreducibility_checks,
                    prym_factor_evaluation_checks,
                    prym_factor_type_histogram[0],
                    prym_factor_type_histogram[1],
                    prym_factor_type_histogram[2],
                    prym_factor_type_histogram[3],
                    prym_factor_type_histogram[4],
                    prym_factor_type_distinguishing_targets,
                    group_count,
                    conflicting_groups,
                    conflicting_targets,
                    in_sample_lookup_matches,
                    conflicting_groups == 0,
                    coverage,
                    matches,
                    excess,
                    selected_statistic.0,
                    selected_statistic.1,
                    total_exact_rules,
                    covered_exact_rules,
                    exceedances,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                    per_curve.join(","),
                    expected_targets,
                );
                fs::write(output_path, json).map_err(|error| error.to_string())?;
                println!(
                    "chiral genus-three Prym factor-type transport: groups={} conflicting={} coverage={} matches={} exact={} p={:.6}",
                    group_count,
                    conflicting_groups,
                    coverage,
                    matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
                return Ok(());
            }

            let mut rules = Vec::with_capacity(if oos_panel {
                1
            } else if binary_panel {
                512
            } else {
                486
            });
            if binary_panel {
                for mask in 0_u16..256 {
                    for polarity in 0..=1_u8 {
                        let predictions: Vec<u8> = targets
                            .iter()
                            .map(|target| {
                                binary_prediction(target, eigenline_panel, mask, polarity)
                            })
                            .collect();
                        let (coverage, matches) = score(&predictions, &labels);
                        rules.push(Rule {
                            observable: 0,
                            code: 0,
                            coefficients: [0; FEATURES],
                            offset: 0,
                            splitting_mask: mask,
                            splitting_polarity: polarity,
                            predictions,
                            coverage,
                            matches,
                            statistic: binary_statistic(matches, coverage),
                        });
                    }
                }
            } else if oos_panel {
                let observable = 1_usize;
                let code = 60_u8;
                let coefficients = decode_coefficients(code);
                let offset = 2_u8;
                let predictions: Vec<u8> = targets
                    .iter()
                    .map(|target| {
                        prediction(target, true, observable, &coefficients, offset)
                    })
                    .collect();
                let (coverage, matches) = score(&predictions, &labels);
                rules.push(Rule {
                    observable,
                    code,
                    coefficients,
                    offset,
                    splitting_mask: 0,
                    splitting_polarity: 0,
                    predictions,
                    coverage,
                    matches,
                    statistic: statistic(matches, coverage),
                });
            } else {
                for observable in 0..OBSERVABLES {
                    for code in 0_u8..81 {
                        let coefficients = decode_coefficients(code);
                        for offset in 0..3_u8 {
                            let predictions: Vec<u8> = targets
                                .iter()
                                .map(|target| {
                                    prediction(
                                        target,
                                        prym_panel,
                                        observable,
                                        &coefficients,
                                        offset,
                                    )
                                })
                                .collect();
                            let (coverage, matches) = score(&predictions, &labels);
                            rules.push(Rule {
                                observable,
                                code,
                                coefficients,
                                offset,
                                splitting_mask: 0,
                                splitting_polarity: 0,
                                predictions,
                                coverage,
                                matches,
                                statistic: statistic(matches, coverage),
                            });
                        }
                    }
                }
            }

            let selected_index = (0..rules.len())
                .max_by(|&left, &right| {
                    compare_ratio(rules[left].statistic, rules[right].statistic)
                        .then_with(|| {
                            if binary_panel {
                                rules[right].splitting_mask.cmp(&rules[left].splitting_mask)
                            } else {
                                rules[right].observable.cmp(&rules[left].observable)
                            }
                        })
                        .then_with(|| {
                            if binary_panel {
                                rules[right]
                                    .splitting_polarity
                                    .cmp(&rules[left].splitting_polarity)
                            } else {
                                rules[right].code.cmp(&rules[left].code)
                            }
                        })
                        .then_with(|| rules[right].offset.cmp(&rules[left].offset))
                })
                .ok_or("no rules")?;
            let selected = &rules[selected_index];
            let total_exact_rules = rules
                .iter()
                .filter(|rule| {
                    rule.coverage == expected_targets && rule.matches == expected_targets
                })
                .count();
            let covered_exact_rules = rules
                .iter()
                .filter(|rule| rule.coverage > 0 && rule.matches == rule.coverage)
                .count();

            let mut exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled = vec![0_u8; expected_targets];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> =
                        indices.iter().map(|&index| labels[index]).collect();
                    let mut seed = if eigenline_panel {
                        0x116_016_u64
                    } else if splitting_panel {
                        0x116_015_u64
                    } else if oos_panel {
                        0x116_014_u64
                    } else {
                        0x116_013_u64
                    }
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
                    let candidate = if binary_panel {
                        binary_statistic(matches, rule.coverage)
                    } else {
                        statistic(matches, rule.coverage)
                    };
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

            let radix = if binary_panel { 2_i64 } else { 3_i64 };
            let excess = radix * selected.matches as i64 - selected.coverage as i64;
            let json = format!(
                concat!(
                    "{{\n  \"schema\":\"h116-chiral-genus3-zeta2-digit/v3\",\n",
                    "  \"panel\":\"{}\",\n",
                    "  \"targets\":{},\n  \"curves\":{},\n  \"observables\":2,\n  \"rules\":{},\n",
                    "  \"extension_square_table_checks\":{},\n",
                    "  \"extension_curve_census_checks\":{},\n  \"extension_fixed_point_checks\":{},\n",
                    "  \"extension_affine_points\":{},\n  \"extension_histogram_checks\":{},\n",
                    "  \"extension_rescale_checks\":{},\n  \"smooth_cover_levels\":{},\n",
                    "  \"smooth_feature_checks\":{},\n  \"base_trace_checks\":{},\n",
                    "  \"extension_trace_checks\":{},\n  \"newton_checks\":{},\n",
                    "  \"chirality_swap_checks\":{},\n  \"s2_distinguishing_targets\":{},\n",
                    "  \"e2_distinguishing_targets\":{},\n  \"planted_control_matches\":{},\n",
                    "  \"prym_factor_checks\":{},\n",
                    "  \"prym_eta_distinguishing_targets\":{},\n",
                    "  \"prym_discriminant_distinguishing_targets\":{},\n",
                    "  \"prym_splitting_checks\":{},\n",
                    "  \"prym_splitting_distinguishing_targets\":{},\n",
                    "  \"prym_eigenline_polynomial_checks\":{},\n",
                    "  \"prym_eigenline_quotient_checks\":{},\n",
                    "  \"prym_eigenline_distinguishing_targets\":{},\n",
                    "  \"selected_observable\":\"{}\",\n  \"selected_code\":{},\n",
                    "  \"selected_coefficients\":[{}],\n  \"selected_offset\":{},\n",
                    "  \"selected_mask\":{},\n  \"selected_polarity\":{},\n",
                    "  \"radix\":{},\n  \"coverage\":{},\n  \"matches\":{},\n  \"radix_excess\":{},\n",
                    "  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                    "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n",
                    "  \"replicates\":4096,\n  \"familywise_exceedances\":{},\n",
                    "  \"familywise_empirical_p\":{:.12},\n  \"per_curve\":[{}],\n",
                    "  \"status\":\"measured\"\n}}\n"
                ),
                if eigenline_panel {
                    "prym_eigenline"
                } else if splitting_panel {
                    "prym_splitting"
                } else if oos_panel {
                    "prym_oos"
                } else if prym_panel {
                    "prym"
                } else {
                    "zeta2"
                },
                targets.len(),
                rows.len(),
                rules.len(),
                extension_square_table_checks,
                extension_curve_census_checks,
                extension_fixed_point_checks,
                extension_affine_points,
                extension_histogram_checks,
                extension_rescale_checks,
                smooth_cover_levels,
                smooth_feature_checks,
                base_trace_checks,
                extension_trace_checks,
                newton_checks,
                chirality_swap_checks,
                s2_distinguishing_targets,
                e2_distinguishing_targets,
                expected_targets,
                prym_factor_checks,
                prym_eta_distinguishing_targets,
                prym_discriminant_distinguishing_targets,
                prym_splitting_checks,
                prym_splitting_distinguishing_targets,
                prym_eigenline_polynomial_checks,
                prym_eigenline_quotient_checks,
                prym_eigenline_distinguishing_targets,
                if eigenline_panel {
                    "prym_full_weil_linear_factor_bits_l5_l7"
                } else if splitting_panel {
                    "prym_real_weil_splitting_bits_l5_l7"
                } else {
                    observable_name(selected.observable, prym_panel)
                },
                selected.code,
                coefficient_string(&selected.coefficients),
                selected.offset,
                selected.splitting_mask,
                selected.splitting_polarity,
                radix,
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
            if binary_panel {
                println!(
                    "chiral genus-three Prym {} parity: mask={} polarity={} coverage={} matches={} exact={} p={:.6}",
                    if eigenline_panel { "eigenline" } else { "splitting" },
                    selected.splitting_mask,
                    selected.splitting_polarity,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
            } else {
                println!(
                    "chiral genus-three {} digit: observable={} coefficients=[{}] offset={} coverage={} matches={} exact={} p={:.6}",
                    if oos_panel {
                        "prym-oos"
                    } else if prym_panel {
                        "prym"
                    } else {
                        "zeta2"
                    },
                    observable_name(selected.observable, prym_panel),
                    coefficient_string(&selected.coefficients),
                    selected.offset,
                    selected.coverage,
                    selected.matches,
                    total_exact_rules,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64
                );
            }
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::chiral_genus3_zeta2::selftest(),
        [_, flag, output] if flag == "--run" => {
            frozen::chiral_genus3_zeta2::run(Path::new(output))
        }
        [_, flag, output] if flag == "--run-prym" => {
            frozen::chiral_genus3_zeta2::run_prym(Path::new(output))
        }
        [_, flag, output] if flag == "--run-prym-oos" => {
            frozen::chiral_genus3_zeta2::run_prym_oos(Path::new(output))
        }
        [_, flag, output] if flag == "--run-prym-splitting" => {
            frozen::chiral_genus3_zeta2::run_prym_splitting(Path::new(output))
        }
        [_, flag, output] if flag == "--run-prym-eigenline" => {
            frozen::chiral_genus3_zeta2::run_prym_eigenline(Path::new(output))
        }
        [_, flag, output] if flag == "--run-prym-factor-type" => {
            frozen::chiral_genus3_zeta2::run_prym_factor_type(Path::new(output))
        }
        _ => Err(
            "usage: h116_chiral_genus3_zeta2_digit --selftest | --run OUTPUT.json | --run-prym OUTPUT.json | --run-prym-oos OUTPUT.json | --run-prym-splitting OUTPUT.json | --run-prym-eigenline OUTPUT.json | --run-prym-factor-type OUTPUT.json".to_string(),
        ),
    };
    if let Err(error) = result {
        eprintln!("h116 chiral genus-three zeta2 digit error: {error}");
        std::process::exit(1);
    }
}
