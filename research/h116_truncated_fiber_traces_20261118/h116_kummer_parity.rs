#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod kummer {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::BTreeMap;

        const ANCHORS: usize = 8;
        const REPLICATES: usize = 4096;
        const TARGETS: usize = 1520;

        #[derive(Clone)]
        struct Target {
            slug: String,
            parity: u8,
            signatures: [i8; ANCHORS],
        }

        struct Rule {
            mask: u16,
            polarity: u8,
            coverage_bits: Vec<u64>,
            prediction_bits: Vec<u64>,
            coverage: usize,
            matches: usize,
            statistic: (u128, u128),
        }

        fn legendre(value: u64, p: u64) -> i8 {
            if value == 0 { 0 } else if pow_mod_u64(value, (p - 1) / 2, p) == 1 { 1 } else { -1 }
        }

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let signed = 2_i128 * matches as i128 - coverage as i128;
            if signed <= 0 { (0, 1) } else { ((signed as u128).pow(2), coverage as u128) }
        }

        fn compare_ratio(left: (u128, u128), right: (u128, u128)) -> Ordering {
            (left.0 * right.1).cmp(&(right.0 * left.1))
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64_local(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        fn splitmix64_local(state: &mut u64) -> u64 {
            *state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        }

        fn set_bit(bits: &mut [u64], index: usize, value: bool) {
            if value { bits[index / 64] |= 1_u64 << (index % 64); }
        }

        fn bit_matches(coverage: &[u64], prediction: &[u64], labels: &[u64]) -> usize {
            coverage.iter().zip(prediction).zip(labels).map(|((&covered, &predicted), &label)| {
                (covered & !(predicted ^ label)).count_ones() as usize
            }).sum()
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

        fn build_rule(targets: &[Target], labels: &[u64], mask: u16, polarity: u8) -> Rule {
            let words = targets.len().div_ceil(64);
            let mut coverage_bits = vec![0_u64; words];
            let mut prediction_bits = vec![0_u64; words];
            for (index, target) in targets.iter().enumerate() {
                let signature = product_signature(target, mask);
                if signature == 0 { continue; }
                set_bit(&mut coverage_bits, index, true);
                let split = u8::from(signature == 1);
                set_bit(&mut prediction_bits, index, split ^ polarity == 1);
            }
            let coverage = coverage_bits.iter().map(|word| word.count_ones() as usize).sum();
            let matches = bit_matches(&coverage_bits, &prediction_bits, labels);
            Rule { mask, polarity, coverage_bits, prediction_bits, coverage, matches,
                statistic: statistic(matches, coverage) }
        }

        fn mask_string(mask: u16) -> String {
            let anchors: Vec<String> = (0..ANCHORS).filter(|anchor| mask & (1_u16 << anchor) != 0)
                .map(|anchor| (anchor + 1).to_string()).collect();
            format!("{{{}}}", anchors.join(","))
        }

        pub fn selftest() -> Result<(), String> {
            let masks: Vec<u16> = (1_u16..(1_u16 << ANCHORS))
                .filter(|mask| mask.count_ones() <= 3).collect();
            assert_eq!(masks.len(), 92);
            assert_eq!(statistic(8, 10), (36, 10));
            let coverage = [0b1111_u64];
            let prediction = [0b1010_u64];
            let labels = [0b1110_u64];
            assert_eq!(bit_matches(&coverage, &prediction, &labels), 3);
            let target = Target { slug: "selftest".to_string(), parity: 1,
                signatures: [1, -1, 0, 1, -1, 1, 1, -1] };
            assert_eq!(product_signature(&target, 0b11), -1);
            assert_eq!(product_signature(&target, 0b101), 0);
            println!("h116 Kummer parity selftest: pass rules=184");
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
                for (index, value) in anchor_x.iter_mut().enumerate() {
                    let anchor = curve.scalar_mul(index as u64 + 1, generator)?;
                    *value = point_base_coordinates(&field, anchor)?.0;
                }
                for scalar in 1..spec.order {
                    let point = curve.scalar_mul(scalar, generator)?;
                    let x = point_base_coordinates(&field, point)?.0;
                    let mut signatures = [0_i8; ANCHORS];
                    for anchor in 0..ANCHORS {
                        signatures[anchor] = legendre((x + spec.p - anchor_x[anchor]) % spec.p, spec.p);
                    }
                    let index = targets.len();
                    targets.push(Target { slug: spec.slug.to_string(), parity: (scalar % 2) as u8, signatures });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != 5 { return Err("target census failed".to_string()); }
            let words = targets.len().div_ceil(64);
            let mut label_bits = vec![0_u64; words];
            for (index, target) in targets.iter().enumerate() { set_bit(&mut label_bits, index, target.parity == 1); }

            let masks: Vec<u16> = (1_u16..(1_u16 << ANCHORS))
                .filter(|mask| mask.count_ones() <= 3).collect();
            let mut rules = Vec::with_capacity(184);
            for mask in masks {
                for polarity in 0..=1 { rules.push(build_rule(&targets, &label_bits, mask, polarity)); }
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
                let mut shuffled_labels = vec![0_u64; words];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> = indices.iter().map(|&index| targets[index].parity).collect();
                    let mut seed = 0x116_006_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut values, &mut seed);
                    for (&index, value) in indices.iter().zip(values) { set_bit(&mut shuffled_labels, index, value == 1); }
                }
                let mut maximum = (0_u128, 1_u128);
                for rule in &rules {
                    let matches = bit_matches(&rule.coverage_bits, &rule.prediction_bits, &shuffled_labels);
                    let candidate = statistic(matches, rule.coverage);
                    if compare_ratio(candidate, maximum) == Ordering::Greater { maximum = candidate; }
                }
                exceedances += usize::from(compare_ratio(maximum, selected.statistic) != Ordering::Less);
            }

            let mut curve_json = Vec::new();
            for (slug, indices) in &rows {
                let mut coverage = 0_usize;
                let mut matches = 0_usize;
                for &index in indices {
                    let signature = product_signature(&targets[index], selected.mask);
                    if signature == 0 { continue; }
                    coverage += 1;
                    let prediction = u8::from(signature == 1) ^ selected.polarity;
                    matches += usize::from(prediction == targets[index].parity);
                }
                curve_json.push(format!("{{\"slug\":\"{slug}\",\"targets\":{},\"coverage\":{coverage},\"matches\":{matches}}}", indices.len()));
            }
            let signed_excess = 2_i64 * selected.matches as i64 - selected.coverage as i64;
            let json = format!(concat!("{{\n  \"schema\":\"h116-kummer-parity/v1\",\n",
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
                (exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                curve_json.join(","));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("Kummer parity: subset={} polarity={} coverage={} matches={} exact={} p={:.6}",
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
        [_, flag] if flag == "--selftest" => frozen::kummer::selftest(),
        [_, flag, output] if flag == "--run" => frozen::kummer::run(Path::new(output)),
        _ => Err("usage: h116_kummer_parity --selftest | --run OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 Kummer parity error: {error}"); std::process::exit(1); }
}
