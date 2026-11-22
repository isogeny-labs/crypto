#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_quotient_digit_scorer.rs");

    pub mod oos {
        use super::*;
        use std::cmp::Ordering;
        use std::path::Path;

        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target {
            row: (String, u64),
            slug: String,
            digit: u8,
            prediction: Option<u8>,
        }

        fn statistic(matches: usize, coverage: usize) -> (u128, u128) {
            let signed = 6_i128 * matches as i128 - coverage as i128;
            if signed <= 0 { (0, 1) } else { ((signed as u128).pow(2), 5 * coverage as u128) }
        }

        fn compare_ratio(left: (u128, u128), right: (u128, u128)) -> Ordering {
            (left.0 * right.1).cmp(&(right.0 * left.1))
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        pub fn selftest() -> Result<(), String> {
            assert_eq!(unique_extreme(&[9, 7, 1, 5, 6, 8], false), Some(2));
            assert_eq!(unique_extreme(&[9, 1, 1, 5, 6, 8], false), None);
            for scalar in 0_u64..=1000 {
                assert_eq!(scalar, 6 * (scalar / 6) + scalar % 6);
                assert!(scalar % 6 < 6);
            }
            assert_eq!(statistic(20, 60), (3600, 300));
            println!("h116 quotient digit OOS scorer selftest: pass");
            Ok(())
        }

        pub fn score(moment_path: &str, label_path: &str, output_path: &Path) -> Result<(), String> {
            let records = parse_moments(moment_path)?;
            let labels = parse_labels(label_path)?;
            if records.len() != 9120 || labels.len() != 4560 { return Err("unexpected OOS source census".to_string()); }
            let mut pairs: BTreeMap<Key, [Option<Record>; 2]> = BTreeMap::new();
            for record in records {
                let slot = match record.lambda { 1 => 0, 2 => 1, _ => return Err("bad lambda".to_string()) };
                let key = (record.slug.clone(), record.label, record.digest.clone());
                if pairs.entry(key).or_insert_with(|| [None, None])[slot].replace(record).is_some() {
                    return Err("duplicate model record".to_string());
                }
            }
            let mut canonical: BTreeMap<Key, Record> = BTreeMap::new();
            let mut invariant_mismatches = 0_u64;
            for (key, pair) in pairs {
                let left = pair[0].as_ref().ok_or("missing native model")?;
                let right = pair[1].as_ref().ok_or("missing isomorphic model")?;
                for channel in 0..16 {
                    invariant_mismatches += u64::from(invariant(left, channel) != invariant(right, channel));
                }
                canonical.insert(key, left.clone());
            }
            if invariant_mismatches != 0 { return Err(format!("invariant mismatches: {invariant_mismatches}")); }
            let mut digest_by_scalar: BTreeMap<(String, u64, u64), String> = BTreeMap::new();
            for (key, scalar) in &labels {
                digest_by_scalar.insert((key.0.clone(), key.1, *scalar), key.2.clone());
            }
            let mut targets = Vec::new();
            for (key, scalar) in &labels {
                if *scalar < 6 { continue; }
                let target_record = canonical.get(key).ok_or("missing target record")?;
                let inverse_six = pow_mod(6, target_record.order - 2, target_record.order);
                let mut values = [0_u64; 6];
                for digit in 0_u64..6 {
                    let quotient_scalar = mul_mod(
                        (scalar + target_record.order - digit) % target_record.order,
                        inverse_six,
                        target_record.order,
                    );
                    if quotient_scalar == 0 { return Err("unexpected identity quotient".to_string()); }
                    let digest = digest_by_scalar.get(&(key.0.clone(), key.1, quotient_scalar))
                        .ok_or("missing quotient digest")?;
                    let quotient = canonical.get(&(key.0.clone(), key.1, digest.clone()))
                        .ok_or("missing quotient record")?;
                    values[digit as usize] = invariant(quotient, 1);
                }
                targets.push(Target {
                    row: (key.0.clone(), key.1), slug: key.0.clone(), digit: (scalar % 6) as u8,
                    prediction: unique_extreme(&values, false),
                });
            }
            if targets.len() != 4485 { return Err(format!("unexpected OOS target census: {}", targets.len())); }
            let coverage = targets.iter().filter(|target| target.prediction.is_some()).count();
            let matches = targets.iter().filter(|target| target.prediction == Some(target.digit)).count();
            let observed_stat = statistic(matches, coverage);

            let mut curve_stats: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
            for target in &targets {
                let entry = curve_stats.entry(target.slug.clone()).or_default();
                entry.0 += 1;
                entry.1 += usize::from(target.prediction.is_some());
                entry.2 += usize::from(target.prediction == Some(target.digit));
            }

            let mut rows: BTreeMap<(String, u64), Vec<usize>> = BTreeMap::new();
            for (index, target) in targets.iter().enumerate() {
                rows.entry(target.row.clone()).or_default().push(index);
            }
            let mut exceedances = 0_usize;
            let mut exact_null_replicates = 0_usize;
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
                let null_matches = targets.iter().enumerate()
                    .filter(|(index, target)| target.prediction == Some(digits[*index]))
                    .count();
                let null_stat = statistic(null_matches, coverage);
                exceedances += usize::from(compare_ratio(null_stat, observed_stat) != Ordering::Less);
                exact_null_replicates += usize::from(coverage == targets.len() && null_matches == targets.len());
            }
            let empirical_p = (exceedances + 1) as f64 / (REPLICATES + 1) as f64;
            let stat_value = observed_stat.0 as f64 / observed_stat.1 as f64;
            let signed_excess = 6_i64 * matches as i64 - coverage as i64;
            let status = if coverage == targets.len() && matches == targets.len() && exact_null_replicates == 0 {
                "exact_decoder_candidate"
            } else if signed_excess > 0 && empirical_p <= 0.01 {
                "association_replicated"
            } else {
                "not_replicated"
            };
            let per_curve = curve_stats.iter().map(|(slug, &(count, covered, correct))| {
                format!(concat!("{{\"slug\":\"{}\",\"targets\":{},\"coverage\":{},\"matches\":{},",
                    "\"coverage_rate\":{:.12},\"conditional_accuracy\":{:.12},\"signed_excess_six\":{}}}"),
                    slug, count, covered, correct, covered as f64 / count as f64,
                    if covered == 0 { 0.0 } else { correct as f64 / covered as f64 },
                    6_i64 * correct as i64 - covered as i64)
            }).collect::<Vec<_>>().join(",");
            let json = format!(
                concat!("{{\n  \"schema\":\"h116-quotient-digit-oos-result/v1\",\n",
                "  \"source_targets\":4560,\n  \"source_model_records\":9120,\n",
                "  \"scored_targets\":{},\n  \"rows\":{},\n  \"rule\":\"IX1-argmin\",\n",
                "  \"invariant_mismatches\":{},\n  \"coverage\":{},\n  \"matches\":{},\n",
                "  \"coverage_rate\":{:.12},\n  \"conditional_accuracy\":{:.12},\n",
                "  \"signed_excess_six\":{},\n  \"stat_numerator\":{},\n  \"stat_denominator\":{},\n",
                "  \"stat_value\":{:.12},\n  \"z\":{:.12},\n  \"replicates\":4096,\n",
                "  \"exceedances\":{},\n  \"empirical_p\":{:.12},\n",
                "  \"exact_null_replicates\":{},\n  \"per_curve\":[{}],\n  \"status\":\"{}\"\n}}\n"),
                targets.len(), rows.len(), invariant_mismatches, coverage, matches,
                coverage as f64 / targets.len() as f64,
                if coverage == 0 { 0.0 } else { matches as f64 / coverage as f64 },
                signed_excess, observed_stat.0, observed_stat.1, stat_value, stat_value.sqrt(),
                exceedances, empirical_p, exact_null_replicates, per_curve, status,
            );
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!(
                "quotient OOS scorer: targets={} coverage={coverage} matches={matches} p={empirical_p:.6} status={status}",
                targets.len()
            );
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::oos::selftest(),
        [_, flag, moments, labels, output] if flag == "--score" =>
            frozen::oos::score(moments, labels, Path::new(output)),
        _ => Err("usage: h116_quotient_digit_oos_scorer --selftest | --score MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient OOS scorer error: {error}"); std::process::exit(1); }
}
