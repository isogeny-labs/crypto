#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod miller {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const ANCHORS: usize = 8;
        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target { slug: String, parity: u8, signatures: [i8; ANCHORS] }
        struct Rule { mask: u16, polarity: u8, predictions: Vec<u8>, coverage: usize,
            matches: usize, statistic: (u128, u128) }

        fn legendre(value: u64, p: u64) -> i8 {
            if value == 0 { 0 } else if pow_mod_u64(value, (p - 1) / 2, p) == 1 { 1 } else { -1 }
        }
        fn sub(left: u64, right: u64, p: u64) -> u64 { (left + p - right) % p }
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
            for anchor in 0..ANCHORS {
                if mask & (1_u16 << anchor) != 0 {
                    if target.signatures[anchor] == 0 { return 0; }
                    product *= target.signatures[anchor];
                }
            }
            product
        }
        fn mask_string(mask: u16) -> String {
            let values: Vec<String> = (0..ANCHORS).filter(|anchor| mask & (1_u16 << anchor) != 0)
                .map(|anchor| (anchor + 1).to_string()).collect();
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
            let masks: Vec<u16> = (1_u16..(1_u16 << ANCHORS)).filter(|mask| mask.count_ones() <= 3).collect();
            assert_eq!(masks.len(), 92);
            assert_eq!(score(&[0, 1, 2, 1], &[0, 0, 1, 1]), (3, 2));
            assert_eq!(statistic(8, 10), (36, 10));
            assert_eq!(sub(2, 5, 7), 4);
            println!("h116 Miller Kummer parity selftest: pass rules=184");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            for spec in CURVES {
                let field = Field::new(spec.p, 1, &[0, 1])?;
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity || curve.scalar_mul(spec.order, generator)? != Point::Infinity {
                    return Err("generator/order gate failed".to_string());
                }
                let mut anchor_x = [0_u64; ANCHORS];
                let mut anchor_y = [0_u64; ANCHORS];
                let mut slopes = [0_u64; ANCHORS];
                let mut double_x = [0_u64; ANCHORS];
                for anchor in 0..ANCHORS {
                    let point = curve.scalar_mul(anchor as u64 + 1, generator)?;
                    (anchor_x[anchor], anchor_y[anchor]) = point_base_coordinates(&field, point)?;
                    let numerator = (mul_mod_u64(3, mul_mod_u64(anchor_x[anchor], anchor_x[anchor], spec.p), spec.p)
                        + spec.a) % spec.p;
                    let denominator = mul_mod_u64(2, anchor_y[anchor], spec.p);
                    if denominator == 0 { return Err("unexpected two-torsion anchor".to_string()); }
                    slopes[anchor] = mul_mod_u64(numerator, pow_mod_u64(denominator, spec.p - 2, spec.p), spec.p);
                    double_x[anchor] = point_base_coordinates(
                        &field, curve.scalar_mul(2 * (anchor as u64 + 1), generator)?)?.0;
                }
                for scalar in 1..spec.order {
                    let point = curve.scalar_mul(scalar, generator)?;
                    let (x, y) = point_base_coordinates(&field, point)?;
                    let mut signatures = [0_i8; ANCHORS];
                    for anchor in 0..ANCHORS {
                        let line = sub(sub(y, anchor_y[anchor], spec.p),
                            mul_mod_u64(slopes[anchor], sub(x, anchor_x[anchor], spec.p), spec.p), spec.p);
                        let vertical = sub(x, double_x[anchor], spec.p);
                        let line_character = legendre(line, spec.p);
                        let vertical_character = legendre(vertical, spec.p);
                        signatures[anchor] = if line_character == 0 || vertical_character == 0 {
                            0
                        } else {
                            line_character * vertical_character
                        };
                    }
                    let index = targets.len();
                    targets.push(Target { slug: spec.slug.to_string(), parity: (scalar % 2) as u8, signatures });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != 5 { return Err("target census failed".to_string()); }
            let labels: Vec<u8> = targets.iter().map(|target| target.parity).collect();
            let masks: Vec<u16> = (1_u16..(1_u16 << ANCHORS)).filter(|mask| mask.count_ones() <= 3).collect();
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
                    let mut seed = 0x116_008_u64
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
            let json = format!(concat!("{{\n  \"schema\":\"h116-miller-kummer-parity/v1\",\n",
                "  \"targets\":{},\n  \"curves\":{},\n  \"anchors\":{},\n  \"rules\":{},\n",
                "  \"selected_mask\":{},\n  \"selected_subset\":\"{}\",\n",
                "  \"selected_polarity\":{},\n  \"coverage\":{},\n  \"matches\":{},\n",
                "  \"signed_excess\":{},\n  \"statistic_numerator\":{},\n",
                "  \"statistic_denominator\":{},\n  \"total_exact_rules\":{},\n",
                "  \"covered_exact_rules\":{},\n  \"replicates\":4096,\n",
                "  \"familywise_exceedances\":{},\n  \"familywise_empirical_p\":{:.12},\n",
                "  \"per_curve\":[{}],\n  \"status\":\"measured\"\n}}\n"),
                targets.len(), rows.len(), ANCHORS, rules.len(), selected.mask,
                mask_string(selected.mask), selected.polarity, selected.coverage, selected.matches,
                signed_excess, selected.statistic.0, selected.statistic.1, total_exact_rules,
                covered_exact_rules, exceedances,
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64, curve_json.join(","));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("Miller Kummer parity: subset={} polarity={} coverage={} matches={} exact={} p={:.6}",
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
        [_, flag] if flag == "--selftest" => frozen::miller::selftest(),
        [_, flag, output] if flag == "--run" => frozen::miller::run(Path::new(output)),
        _ => Err("usage: h116_miller_kummer_parity --selftest | --run OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 Miller Kummer parity error: {error}"); std::process::exit(1); }
}
