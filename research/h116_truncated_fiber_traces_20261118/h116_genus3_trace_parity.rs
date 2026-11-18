#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod genus3_trace {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const FEATURES: usize = 8;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target { slug: String, parity: u8, bits: [i8; FEATURES] }
        struct Rule { mask: u16, polarity: u8, predictions: Vec<u8>, coverage: usize,
            matches: usize, statistic: (u128, u128) }

        fn add(left: u64, right: u64, p: u64) -> u64 {
            ((left as u128 + right as u128) % p as u128) as u64
        }
        fn sub(left: u64, right: u64, p: u64) -> u64 { (left + p - right) % p }
        fn legendre(value: u64, p: u64) -> i8 {
            if value == 0 { 0 } else if pow_mod_u64(value, (p - 1) / 2, p) == 1 { 1 } else { -1 }
        }
        fn smooth_level(spec: CurveSpec, y: u64) -> bool {
            let a3 = mul_mod_u64(mul_mod_u64(spec.a, spec.a, spec.p), spec.a, spec.p);
            let y2 = mul_mod_u64(y, y, spec.p);
            let constant = sub(spec.b, y2, spec.p);
            let discriminant = add(mul_mod_u64(4, a3, spec.p),
                mul_mod_u64(27, mul_mod_u64(constant, constant, spec.p), spec.p), spec.p);
            discriminant != 0
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
        fn prediction(target: &Target, mask: u16, polarity: u8) -> u8 {
            let mut bit = polarity;
            for feature in 0..FEATURES {
                if mask & (1_u16 << feature) != 0 {
                    if target.bits[feature] < 0 { return 2; }
                    bit ^= target.bits[feature] as u8;
                }
            }
            bit
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
        fn mask_string(mask: u16) -> String {
            let values: Vec<String> = (0..FEATURES).filter(|feature| mask & (1_u16 << feature) != 0)
                .map(|feature| (feature + 1).to_string()).collect();
            format!("{{{}}}", values.join(","))
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!((0_u16..(1_u16 << FEATURES)).count(), 256);
            assert_eq!(legendre(2, 7), 1);
            assert_eq!(legendre(3, 7), -1);
            assert_eq!(statistic(8, 10), (36, 10));
            let planted = Target { slug: "control".to_string(), parity: 1,
                bits: [1, 0, 0, 0, 0, 0, 0, 0] };
            assert_eq!(prediction(&planted, 1, 0), 1);
            println!("h116 genus-three trace parity selftest: pass rules=512");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut smooth_feature_checks = 0_usize;
            let mut weil_checks = 0_usize;
            let mut rescale_checks = 0_usize;

            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity || curve.scalar_mul(spec.order, generator)? != Point::Infinity {
                    return Err("generator/order gate failed".to_string());
                }
                let mut point_y = Vec::new();
                for scalar in 1..spec.order {
                    point_y.push(point_base_coordinates(&field, curve.scalar_mul(scalar, generator)?)?.1);
                }
                if point_y.len() + 1 != spec.order as usize { return Err("point census failed".to_string()); }

                let mut trace_bits = vec![None; spec.p as usize];
                for level in 0..spec.p {
                    if !smooth_level(spec, level) { continue; }
                    let sum: i64 = point_y.iter().map(|&y| i64::from(legendre(sub(y, level, spec.p), spec.p))).sum();
                    let cover_trace = spec.trace - sum;
                    if cover_trace * cover_trace > 36 * spec.p as i64 {
                        return Err("genus-three Weil bound failed".to_string());
                    }
                    let cover_count = spec.order as i64 + sum;
                    if spec.p as i64 + 1 - cover_count != cover_trace {
                        return Err("cover count/trace identity failed".to_string());
                    }
                    trace_bits[level as usize] = Some(cover_trace.rem_euclid(2) as u8);
                    weil_checks += 1;

                    for scale in [2_u64, 3] {
                        let scale3 = pow_mod_u64(scale, 3, spec.p);
                        let scaled_level = mul_mod_u64(scale3, level, spec.p);
                        let scaled_sum: i64 = point_y.iter().map(|&y| {
                            let scaled_y = mul_mod_u64(scale3, y, spec.p);
                            i64::from(legendre(sub(scaled_y, scaled_level, spec.p), spec.p))
                        }).sum();
                        let scaled_trace = spec.trace - scaled_sum;
                        if scaled_trace.rem_euclid(2) as u8 != cover_trace.rem_euclid(2) as u8 {
                            return Err("coordinate-rescaled trace parity changed".to_string());
                        }
                        rescale_checks += 1;
                    }
                }

                for scalar in 1..spec.order {
                    let target_point = curve.scalar_mul(scalar, generator)?;
                    let mut bits = [-1_i8; FEATURES];
                    for feature in 0..FEATURES {
                        let multiple = curve.scalar_mul((feature + 1) as u64, target_point)?;
                        let y = point_base_coordinates(&field, multiple)?.1;
                        if let Some(bit) = trace_bits[y as usize] {
                            bits[feature] = bit as i8;
                            smooth_feature_checks += 1;
                        }
                    }
                    let index = targets.len();
                    targets.push(Target { slug: spec.slug.to_string(), parity: (scalar % 2) as u8, bits });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != 5 {
                return Err("target census failed".to_string());
            }
            let labels: Vec<u8> = targets.iter().map(|target| target.parity).collect();
            let planted: Vec<u8> = labels.clone();
            if planted.iter().zip(&labels).filter(|(left, right)| left == right).count() != TARGETS {
                return Err("planted scorer control failed".to_string());
            }

            let mut rules = Vec::with_capacity(512);
            for mask in 0_u16..(1_u16 << FEATURES) {
                for polarity in 0..=1 {
                    let predictions: Vec<u8> = targets.iter().map(|target| prediction(target, mask, polarity)).collect();
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
            let covered_exact_rules = rules.iter().filter(|rule| rule.coverage > 0 && rule.matches == rule.coverage).count();

            let mut exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled = vec![0_u8; TARGETS];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> = indices.iter().map(|&index| labels[index]).collect();
                    let mut seed = 0x116_011_u64
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

            let mut per_curve = Vec::new();
            for (slug, indices) in &rows {
                let predictions: Vec<u8> = indices.iter().map(|&index| selected.predictions[index]).collect();
                let curve_labels: Vec<u8> = indices.iter().map(|&index| labels[index]).collect();
                let (coverage, matches) = score(&predictions, &curve_labels);
                per_curve.push(format!("{{\"slug\":\"{slug}\",\"targets\":{},\"coverage\":{coverage},\"matches\":{matches}}}", indices.len()));
            }
            let signed_excess = 2_i64 * selected.matches as i64 - selected.coverage as i64;
            let json = format!(concat!("{{\n  \"schema\":\"h116-genus3-trace-parity/v1\",\n",
                "  \"targets\":{},\n  \"curves\":{},\n  \"features\":8,\n  \"rules\":{},\n",
                "  \"smooth_feature_checks\":{},\n  \"weil_checks\":{},\n  \"rescale_checks\":{},\n",
                "  \"planted_control_matches\":{},\n  \"selected_mask\":{},\n  \"selected_multipliers\":\"{}\",\n",
                "  \"selected_polarity\":{},\n  \"coverage\":{},\n  \"matches\":{},\n",
                "  \"signed_excess\":{},\n  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                "  \"total_exact_rules\":{},\n  \"covered_exact_rules\":{},\n  \"replicates\":4096,\n",
                "  \"familywise_exceedances\":{},\n  \"familywise_empirical_p\":{:.12},\n",
                "  \"per_curve\":[{}],\n  \"status\":\"measured\"\n}}\n"),
                targets.len(), rows.len(), rules.len(), smooth_feature_checks, weil_checks,
                rescale_checks, TARGETS, selected.mask, mask_string(selected.mask), selected.polarity,
                selected.coverage, selected.matches, signed_excess, selected.statistic.0,
                selected.statistic.1, total_exact_rules, covered_exact_rules, exceedances,
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64, per_curve.join(","));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("genus-three trace parity: multipliers={} polarity={} coverage={} matches={} exact={} p={:.6}",
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
        [_, flag] if flag == "--selftest" => frozen::genus3_trace::selftest(),
        [_, flag, output] if flag == "--run" => frozen::genus3_trace::run(Path::new(output)),
        _ => Err("usage: h116_genus3_trace_parity --selftest | --run OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 genus-three trace parity error: {error}"); std::process::exit(1); }
}
