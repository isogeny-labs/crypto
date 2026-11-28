#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod divpoly_character {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const FEATURES: usize = 8;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target { slug: String, parity: u8, signatures: [i8; FEATURES] }
        struct Rule { mask: u16, polarity: u8, predictions: Vec<u8>, coverage: usize,
            matches: usize, statistic: (u128, u128) }

        fn add(left: u64, right: u64, p: u64) -> u64 {
            ((left as u128 + right as u128) % p as u128) as u64
        }
        fn sub(left: u64, right: u64, p: u64) -> u64 { (left + p - right) % p }
        fn cube(value: u64, p: u64) -> u64 {
            mul_mod_u64(mul_mod_u64(value, value, p), value, p)
        }
        fn legendre(value: u64, p: u64) -> i8 {
            if value == 0 { 0 } else if pow_mod_u64(value, (p - 1) / 2, p) == 1 { 1 } else { -1 }
        }

        fn division_values(p: u64, a: u64, b: u64, x: u64, y: u64) -> Result<[u64; 11], String> {
            let mut w = [0_u64; 11];
            w[1] = 1;
            w[2] = mul_mod_u64(2, y, p);
            if w[2] == 0 { return Err("division recurrence met two-torsion".to_string()); }
            let x2 = mul_mod_u64(x, x, p);
            let x3 = mul_mod_u64(x2, x, p);
            let x4 = mul_mod_u64(x2, x2, p);
            let x6 = mul_mod_u64(x3, x3, p);
            let a2 = mul_mod_u64(a, a, p);
            let a3 = mul_mod_u64(a2, a, p);
            let b2 = mul_mod_u64(b, b, p);

            let mut w3 = mul_mod_u64(3, x4, p);
            w3 = add(w3, mul_mod_u64(6, mul_mod_u64(a, x2, p), p), p);
            w3 = add(w3, mul_mod_u64(12, mul_mod_u64(b, x, p), p), p);
            w[3] = sub(w3, a2, p);

            let mut inner = x6;
            inner = add(inner, mul_mod_u64(5, mul_mod_u64(a, x4, p), p), p);
            inner = add(inner, mul_mod_u64(20, mul_mod_u64(b, x3, p), p), p);
            inner = sub(inner, mul_mod_u64(5, mul_mod_u64(a2, x2, p), p), p);
            inner = sub(inner, mul_mod_u64(4, mul_mod_u64(mul_mod_u64(a, b, p), x, p), p), p);
            inner = sub(inner, mul_mod_u64(8, b2, p), p);
            inner = sub(inner, a3, p);
            w[4] = mul_mod_u64(mul_mod_u64(4, y, p), inner, p);

            let w2_inverse = pow_mod_u64(w[2], p - 2, p);
            for index in 5..=10 {
                if index % 2 == 1 {
                    let n = (index - 1) / 2;
                    w[index] = sub(
                        mul_mod_u64(w[n + 2], cube(w[n], p), p),
                        mul_mod_u64(w[n - 1], cube(w[n + 1], p), p), p);
                } else {
                    let n = index / 2;
                    let bracket = sub(
                        mul_mod_u64(w[n + 2], mul_mod_u64(w[n - 1], w[n - 1], p), p),
                        mul_mod_u64(w[n - 2], mul_mod_u64(w[n + 1], w[n + 1], p), p), p);
                    w[index] = mul_mod_u64(mul_mod_u64(w[n], bracket, p), w2_inverse, p);
                }
            }
            Ok(w)
        }

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let signed = 2_i128 * matches as i128 - coverage as i128;
            if signed <= 0 { (0, 1) } else { ((signed as u128).pow(2), coverage as u128) }
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
        fn product_signature(target: &Target, mask: u16) -> i8 {
            let mut product = 1_i8;
            for feature in 0..FEATURES {
                if mask & (1_u16 << feature) != 0 {
                    if target.signatures[feature] == 0 { return 0; }
                    product *= target.signatures[feature];
                }
            }
            product
        }
        fn mask_string(mask: u16) -> String {
            let values: Vec<String> = (0..FEATURES).filter(|feature| mask & (1_u16 << feature) != 0)
                .map(|feature| (feature + 2).to_string()).collect();
            format!("{{{}}}", values.join(","))
        }
        fn score(predictions: &[u8], labels: &[u8]) -> (usize, usize) {
            let mut coverage = 0_usize;
            let mut matches = 0_usize;
            for (&prediction, &label) in predictions.iter().zip(labels) {
                if prediction == 2 { continue; }
                coverage += 1;
                matches += usize::from(prediction == label);
            }
            (coverage, matches)
        }

        pub fn selftest() -> Result<(), String> {
            let masks: Vec<u16> = (1_u16..(1_u16 << FEATURES)).filter(|mask| mask.count_ones() <= 3).collect();
            assert_eq!(masks.len(), 92);
            let values = division_values(5, 1, 1, 0, 1)?;
            assert_eq!(&values[0..5], &[0, 1, 2, 4, 4]);
            assert_eq!(score(&[0, 1, 2, 1], &[0, 0, 1, 1]), (3, 2));
            println!("h116 division-polynomial character selftest: pass rules=184");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut identity_checks = 0_usize;
            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity || curve.scalar_mul(spec.order, generator)? != Point::Infinity {
                    return Err("generator/order gate failed".to_string());
                }
                for scalar in 1..spec.order {
                    let point = curve.scalar_mul(scalar, generator)?;
                    let (x, y) = point_base_coordinates(&field, point)?;
                    let values = division_values(spec.p, spec.a, spec.b, x, y)?;
                    for multiplier in 2..=9 {
                        if values[multiplier] == 0 { return Err("unexpected small torsion".to_string()); }
                        let quotient = mul_mod_u64(
                            mul_mod_u64(values[multiplier - 1], values[multiplier + 1], spec.p),
                            pow_mod_u64(mul_mod_u64(values[multiplier], values[multiplier], spec.p), spec.p - 2, spec.p),
                            spec.p);
                        let expected_x = sub(x, quotient, spec.p);
                        let actual_x = point_base_coordinates(
                            &field, curve.scalar_mul(multiplier as u64, point)?)?.0;
                        if expected_x != actual_x { return Err("division-polynomial x identity failed".to_string()); }
                        identity_checks += 1;
                    }
                    let mut signatures = [0_i8; FEATURES];
                    for feature in 0..FEATURES { signatures[feature] = legendre(values[feature + 2], spec.p); }
                    let index = targets.len();
                    targets.push(Target { slug: spec.slug.to_string(), parity: (scalar % 2) as u8, signatures });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != 5 || identity_checks != TARGETS * FEATURES {
                return Err("target or identity census failed".to_string());
            }
            let labels: Vec<u8> = targets.iter().map(|target| target.parity).collect();
            let masks: Vec<u16> = (1_u16..(1_u16 << FEATURES)).filter(|mask| mask.count_ones() <= 3).collect();
            let mut rules = Vec::with_capacity(184);
            for mask in masks {
                for polarity in 0..=1 {
                    let predictions: Vec<u8> = targets.iter().map(|target| match product_signature(target, mask) {
                        0 => 2,
                        signature => u8::from(signature == 1) ^ polarity,
                    }).collect();
                    let (coverage, matches) = score(&predictions, &labels);
                    rules.push(Rule { mask, polarity, predictions, coverage, matches,
                        statistic: statistic(matches, coverage) });
                }
            }
            let selected_index = (0..rules.len()).max_by(|&left, &right| {
                compare_ratio(rules[left].statistic, rules[right].statistic)
                    .then_with(|| rules[right].mask.cmp(&rules[left].mask))
                    .then_with(|| rules[right].polarity.cmp(&rules[left].polarity))
            }).ok_or("no rules")?;
            let selected = &rules[selected_index];
            let total_exact_rules = rules.iter().filter(|rule| rule.coverage == TARGETS && rule.matches == TARGETS).count();
            let covered_exact_rules = rules.iter().filter(|rule| rule.matches == rule.coverage).count();
            let mut exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled = vec![0_u8; TARGETS];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> = indices.iter().map(|&index| labels[index]).collect();
                    let mut seed = 0x116_009_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut values, &mut seed);
                    for (&index, value) in indices.iter().zip(values) { shuffled[index] = value; }
                }
                let mut maximum = (0_u128, 1_u128);
                for rule in &rules {
                    let (_, matches) = score(&rule.predictions, &shuffled);
                    let candidate = statistic(matches, rule.coverage);
                    if compare_ratio(candidate, maximum) == Ordering::Greater { maximum = candidate; }
                }
                exceedances += usize::from(compare_ratio(maximum, selected.statistic) != Ordering::Less);
            }
            let mut curve_json = Vec::new();
            for (slug, indices) in &rows {
                let predictions: Vec<u8> = indices.iter().map(|&index| selected.predictions[index]).collect();
                let curve_labels: Vec<u8> = indices.iter().map(|&index| labels[index]).collect();
                let (coverage, matches) = score(&predictions, &curve_labels);
                curve_json.push(format!("{{\"slug\":\"{slug}\",\"targets\":{},\"coverage\":{coverage},\"matches\":{matches}}}", indices.len()));
            }
            let signed_excess = 2_i64 * selected.matches as i64 - selected.coverage as i64;
            let json = format!(concat!("{{\n  \"schema\":\"h116-divpoly-character-parity/v1\",\n",
                "  \"targets\":{},\n  \"curves\":{},\n  \"features\":{},\n  \"rules\":{},\n",
                "  \"identity_checks\":{},\n  \"selected_mask\":{},\n  \"selected_indices\":\"{}\",\n",
                "  \"selected_polarity\":{},\n  \"coverage\":{},\n  \"matches\":{},\n",
                "  \"signed_excess\":{},\n  \"statistic_numerator\":{},\n",
                "  \"statistic_denominator\":{},\n  \"total_exact_rules\":{},\n",
                "  \"covered_exact_rules\":{},\n  \"replicates\":4096,\n",
                "  \"familywise_exceedances\":{},\n  \"familywise_empirical_p\":{:.12},\n",
                "  \"per_curve\":[{}],\n  \"status\":\"measured\"\n}}\n"),
                targets.len(), rows.len(), FEATURES, rules.len(), identity_checks, selected.mask,
                mask_string(selected.mask), selected.polarity, selected.coverage, selected.matches,
                signed_excess, selected.statistic.0, selected.statistic.1, total_exact_rules,
                covered_exact_rules, exceedances,
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64, curve_json.join(","));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("division-polynomial character: indices={} polarity={} coverage={} matches={} exact={} p={:.6}",
                mask_string(selected.mask), selected.polarity, selected.coverage, selected.matches,
                total_exact_rules, (exceedances + 1) as f64 / (REPLICATES + 1) as f64);
            Ok(())
        }
    }
}

use std::path::Path;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::divpoly_character::selftest(),
        [_, flag, output] if flag == "--run" => frozen::divpoly_character::run(Path::new(output)),
        _ => Err("usage: h116_divpoly_character_parity --selftest | --run OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 division-polynomial character error: {error}"); std::process::exit(1); }
}
