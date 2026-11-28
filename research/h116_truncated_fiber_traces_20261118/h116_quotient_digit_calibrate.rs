#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_quotient_digit_scorer.rs");

    pub mod calibrate {
        use super::*;
        use std::cmp::Ordering;
        use std::path::Path;

        const EXPECTED_MATCHES: [usize; 32] = [
            57,24,57,16,52,39,51,12,56,11,45,18,47,39,45,37,
            30,19,37,30,48,29,34,47,18,26,20,38,16,40,29,30,
        ];
        const EXPECTED_COVERAGE: [usize; 32] = [
            246,192,204,196,196,186,206,156,218,178,188,208,202,202,184,188,
            214,164,200,136,204,184,194,200,172,176,204,168,198,136,176,208,
        ];
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target {
            row: (String, u64),
            digit: u8,
            predictions: [Option<u8>; 32],
        }

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let signed = 6_i128 * matches as i128 - coverage as i128;
            if signed <= 0 { return (0, 1); }
            ((signed as u128).pow(2), 5 * coverage as u128)
        }

        fn compare_ratio(left: (u128, u128), right: (u128, u128)) -> Ordering {
            (left.0 * right.1).cmp(&(right.0 * left.1))
        }

        fn max_statistic(matches: &[usize; 32], coverage: &[usize; 32]) -> ((u128, u128), Vec<usize>) {
            let mut best = (0_u128, 1_u128);
            let mut rules = Vec::new();
            for rule in 0..32 {
                let candidate = statistic(matches[rule], coverage[rule]);
                match compare_ratio(candidate, best) {
                    Ordering::Greater => { best = candidate; rules.clear(); rules.push(rule); }
                    Ordering::Equal => rules.push(rule),
                    Ordering::Less => {}
                }
            }
            (best, rules)
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        fn reconstruct(moment_path: &str, label_path: &str) -> Result<Vec<Target>, String> {
            let records = parse_moments(moment_path)?;
            let labels = parse_labels(label_path)?;
            if records.len() != 576 || labels.len() != 288 { return Err("unexpected source census".to_string()); }
            let mut pairs: BTreeMap<Key, [Option<Record>; 2]> = BTreeMap::new();
            for record in records {
                let slot = match record.lambda { 1 => 0, 2 => 1, _ => return Err("bad lambda".to_string()) };
                let key = (record.slug.clone(), record.label, record.digest.clone());
                if pairs.entry(key).or_insert_with(|| [None, None])[slot].replace(record).is_some() {
                    return Err("duplicate model record".to_string());
                }
            }
            let mut canonical: BTreeMap<Key, Record> = BTreeMap::new();
            for (key, pair) in pairs {
                let left = pair[0].as_ref().ok_or("missing native model")?;
                let right = pair[1].as_ref().ok_or("missing isomorphic model")?;
                for channel in 0..16 {
                    if invariant(left, channel) != invariant(right, channel) {
                        return Err("invariant mismatch".to_string());
                    }
                }
                canonical.insert(key, left.clone());
            }
            let mut digest_by_scalar: BTreeMap<(String, u64, u64), String> = BTreeMap::new();
            for (key, scalar) in &labels {
                digest_by_scalar.insert((key.0.clone(), key.1, *scalar), key.2.clone());
            }
            let mut targets = Vec::new();
            for (key, scalar) in &labels {
                if *scalar < 6 { continue; }
                let target = canonical.get(key).ok_or("missing target")?;
                let inverse_six = pow_mod(6, target.order - 2, target.order);
                let mut values = [[0_u64; 6]; 16];
                for digit in 0_u64..6 {
                    let quotient_scalar = mul_mod(
                        (scalar + target.order - digit) % target.order,
                        inverse_six,
                        target.order,
                    );
                    let digest = digest_by_scalar.get(&(key.0.clone(), key.1, quotient_scalar))
                        .ok_or("missing quotient digest")?;
                    let quotient = canonical.get(&(key.0.clone(), key.1, digest.clone()))
                        .ok_or("missing quotient record")?;
                    for (channel, row) in values.iter_mut().enumerate() {
                        row[digit as usize] = invariant(quotient, channel);
                    }
                }
                let mut predictions = [None; 32];
                for channel in 0..16 {
                    predictions[2 * channel] = unique_extreme(&values[channel], false);
                    predictions[2 * channel + 1] = unique_extreme(&values[channel], true);
                }
                targets.push(Target {
                    row: (key.0.clone(), key.1), digit: (scalar % 6) as u8, predictions,
                });
            }
            if targets.len() != 258 { return Err("unexpected target census".to_string()); }
            Ok(targets)
        }

        fn quantile(sorted: &[f64], numerator: usize, denominator: usize) -> f64 {
            let index = ((sorted.len() - 1) * numerator + denominator - 1) / denominator;
            sorted[index]
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(statistic(20, 60), (3600, 300));
            assert_eq!(compare_ratio((1, 2), (2, 3)), Ordering::Less);
            let mut left = vec![0_u8, 1, 1, 2, 3, 5];
            let mut right = left.clone();
            let mut seed_left = 0x116_003;
            let mut seed_right = 0x116_003;
            shuffle(&mut left, &mut seed_left);
            shuffle(&mut right, &mut seed_right);
            assert_eq!(left, right);
            left.sort_unstable();
            assert_eq!(left, vec![0, 1, 1, 2, 3, 5]);
            println!("h116 quotient digit calibration selftest: pass");
            Ok(())
        }

        pub fn run(moment_path: &str, label_path: &str, output_path: &Path) -> Result<(), String> {
            let targets = reconstruct(moment_path, label_path)?;
            let mut observed_matches = [0_usize; 32];
            let mut coverage = [0_usize; 32];
            for target in &targets {
                for rule in 0..32 {
                    if let Some(prediction) = target.predictions[rule] {
                        coverage[rule] += 1;
                        observed_matches[rule] += usize::from(prediction == target.digit);
                    }
                }
            }
            if observed_matches != EXPECTED_MATCHES || coverage != EXPECTED_COVERAGE {
                return Err("retained result reconstruction mismatch".to_string());
            }
            let (observed_stat, observed_rules) = max_statistic(&observed_matches, &coverage);
            let observed_raw = *observed_matches.iter().max().expect("rules");
            let observed_ids: Vec<String> = observed_rules.iter()
                .map(|&rule| rule_id(rule / 2, rule % 2 == 1)).collect();

            let mut rows: BTreeMap<(String, u64), Vec<usize>> = BTreeMap::new();
            for (index, target) in targets.iter().enumerate() {
                rows.entry(target.row.clone()).or_default().push(index);
            }
            let mut exceed_stat = 0_usize;
            let mut exceed_raw = 0_usize;
            let mut exact_replicates = 0_usize;
            let mut null_stats = Vec::with_capacity(REPLICATES);
            let mut null_raw = Vec::with_capacity(REPLICATES);
            for replicate in 0..REPLICATES {
                let mut digits = vec![0_u8; targets.len()];
                for ((slug, label), indices) in &rows {
                    let mut row_digits: Vec<u8> = indices.iter().map(|&index| targets[index].digit).collect();
                    let mut seed = 0x116_003_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ *label
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut row_digits, &mut seed);
                    for (&index, digit) in indices.iter().zip(row_digits) { digits[index] = digit; }
                }
                let mut matches = [0_usize; 32];
                for (index, target) in targets.iter().enumerate() {
                    for rule in 0..32 {
                        matches[rule] += usize::from(target.predictions[rule] == Some(digits[index]));
                    }
                }
                let (stat, _) = max_statistic(&matches, &coverage);
                let raw = *matches.iter().max().expect("rules");
                exceed_stat += usize::from(compare_ratio(stat, observed_stat) != Ordering::Less);
                exceed_raw += usize::from(raw >= observed_raw);
                exact_replicates += usize::from((0..32).any(|rule| {
                    coverage[rule] == targets.len() && matches[rule] == targets.len()
                }));
                null_stats.push(stat.0 as f64 / stat.1 as f64);
                null_raw.push(raw as f64);
            }
            null_stats.sort_by(f64::total_cmp);
            null_raw.sort_by(f64::total_cmp);
            let observed_stat_value = observed_stat.0 as f64 / observed_stat.1 as f64;
            let json = format!(
                concat!("{{\n  \"schema\":\"h116-quotient-digit-calibration/v1\",\n",
                "  \"targets\":258,\n  \"rules\":32,\n  \"replicates\":4096,\n",
                "  \"observed_stat_numerator\":{},\n  \"observed_stat_denominator\":{},\n",
                "  \"observed_stat_value\":{:.12},\n  \"observed_z\":{:.12},\n",
                "  \"observed_stat_ids\":{:?},\n  \"observed_raw_max\":{},\n",
                "  \"standardized_exceedances\":{},\n  \"standardized_empirical_p\":{:.12},\n",
                "  \"raw_exceedances\":{},\n  \"raw_empirical_p\":{:.12},\n",
                "  \"null_stat_quantiles\":{{\"p50\":{:.12},\"p90\":{:.12},\"p95\":{:.12},\"p99\":{:.12}}},\n",
                "  \"null_raw_quantiles\":{{\"p50\":{},\"p90\":{},\"p95\":{},\"p99\":{}}},\n",
                "  \"exact_null_replicates\":{},\n  \"status\":\"calibrated\"\n}}\n"),
                observed_stat.0, observed_stat.1, observed_stat_value, observed_stat_value.sqrt(),
                observed_ids, observed_raw, exceed_stat, (exceed_stat + 1) as f64 / (REPLICATES + 1) as f64,
                exceed_raw, (exceed_raw + 1) as f64 / (REPLICATES + 1) as f64,
                quantile(&null_stats, 50, 100), quantile(&null_stats, 90, 100),
                quantile(&null_stats, 95, 100), quantile(&null_stats, 99, 100),
                quantile(&null_raw, 50, 100), quantile(&null_raw, 90, 100),
                quantile(&null_raw, 95, 100), quantile(&null_raw, 99, 100), exact_replicates,
            );
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!(
                "quotient calibration: observed_z={:.6} exceed={exceed_stat}/{REPLICATES} p={:.6} raw_exceed={exceed_raw}/{REPLICATES}",
                observed_stat_value.sqrt(), (exceed_stat + 1) as f64 / (REPLICATES + 1) as f64,
            );
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::calibrate::selftest(),
        [_, flag, moments, labels, output] if flag == "--calibrate" =>
            frozen::calibrate::run(moments, labels, Path::new(output)),
        _ => Err("usage: h116_quotient_digit_calibrate --selftest | --calibrate MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient calibration error: {error}"); std::process::exit(1); }
}
