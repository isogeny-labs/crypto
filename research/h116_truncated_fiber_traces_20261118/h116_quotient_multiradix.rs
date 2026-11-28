#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_quotient_digit_scorer.rs");

    pub mod multiradix {
        use super::*;
        use std::path::Path;

        const RADICES: [usize; 12] = [2, 3, 4, 5, 6, 8, 12, 16, 24, 32, 48, 64];
        const REPLICATES: usize = 4096;

        #[derive(Clone)]
        struct Target {
            row: (String, u64),
            slug: String,
            scalar: u64,
            digit: usize,
            ranks: Vec<usize>,
            true_rank: usize,
        }

        struct RadixData {
            radix: usize,
            targets: Vec<Target>,
            rows: BTreeMap<(String, u64), Vec<usize>>,
            histogram: Vec<usize>,
            top: Vec<usize>,
            rank_sum: u64,
            mean_rank: f64,
            normalized_mean_rank: f64,
            quantiles: [usize; 3],
            any_tie_count: usize,
            minimum_tie_count: usize,
            path_success: Vec<usize>,
            max_depth: usize,
            max_required_width: usize,
            curve_stats: BTreeMap<String, (usize, u64)>,
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        fn rank_values(values: &[u64]) -> (Vec<usize>, bool, bool) {
            let mut order: Vec<(u64, usize)> = values.iter().copied().zip(0..values.len()).collect();
            order.sort_unstable();
            let mut ranks = vec![0_usize; values.len()];
            for (index, &(_, digit)) in order.iter().enumerate() { ranks[digit] = index + 1; }
            let any_tie = order.windows(2).any(|window| window[0].0 == window[1].0);
            let minimum_tied = order.len() > 1 && order[0].0 == order[1].0;
            (ranks, any_tie, minimum_tied)
        }

        fn ceil_log2(value: usize) -> usize {
            if value <= 1 { 0 } else { usize::BITS as usize - (value - 1).leading_zeros() as usize }
        }

        fn quantile(sorted: &[usize], numerator: usize, denominator: usize) -> usize {
            let position = (sorted.len() * numerator).div_ceil(denominator).max(1) - 1;
            sorted[position]
        }

        fn log_slope(xs: &[f64], ys: &[f64]) -> f64 {
            let mean_x = xs.iter().sum::<f64>() / xs.len() as f64;
            let mean_y = ys.iter().sum::<f64>() / ys.len() as f64;
            let numerator = xs.iter().zip(ys).map(|(x, y)| (x - mean_x) * (y - mean_y)).sum::<f64>();
            let denominator = xs.iter().map(|x| (x - mean_x).powi(2)).sum::<f64>();
            numerator / denominator
        }

        pub fn selftest() -> Result<(), String> {
            let (ranks, any_tie, minimum_tied) = rank_values(&[9, 1, 1, 7, 5, 3]);
            assert_eq!(ranks, [6, 1, 2, 5, 4, 3]);
            assert!(any_tie && minimum_tied);
            assert_eq!((ceil_log2(2), ceil_log2(3), ceil_log2(64)), (1, 2, 6));
            assert_eq!(quantile(&[1, 2, 3, 4], 50, 100), 2);
            let xs: Vec<f64> = RADICES.iter().map(|&value| (value as f64).ln()).collect();
            let linear: Vec<f64> = RADICES.iter().map(|&value| (value as f64).ln()).collect();
            let square_root: Vec<f64> = RADICES.iter().map(|&value| 0.5 * (value as f64).ln()).collect();
            assert!((log_slope(&xs, &linear) - 1.0).abs() < 1e-12);
            assert!((log_slope(&xs, &square_root) - 0.5).abs() < 1e-12);
            for radix in RADICES {
                for scalar in radix as u64..=1000 {
                    assert_eq!(scalar, radix as u64 * (scalar / radix as u64) + scalar % radix as u64);
                }
            }
            println!("h116 quotient multiradix selftest: pass");
            Ok(())
        }

        fn build_radix(
            radix: usize,
            labels: &BTreeMap<Key, u64>,
            canonical: &BTreeMap<Key, Record>,
            digest_by_scalar: &BTreeMap<(String, u64, u64), String>,
        ) -> Result<RadixData, String> {
            let mut targets = Vec::new();
            let mut any_tie_count = 0_usize;
            let mut minimum_tie_count = 0_usize;
            for (key, scalar) in labels {
                if *scalar < radix as u64 { continue; }
                let target_record = canonical.get(key).ok_or("missing target")?;
                let inverse = pow_mod(radix as u64, target_record.order - 2, target_record.order);
                let mut values = Vec::with_capacity(radix);
                for digit in 0..radix {
                    let quotient_scalar = mul_mod(
                        (scalar + target_record.order - digit as u64) % target_record.order,
                        inverse,
                        target_record.order,
                    );
                    if quotient_scalar == 0 { return Err("unexpected identity quotient".to_string()); }
                    let digest = digest_by_scalar.get(&(key.0.clone(), key.1, quotient_scalar))
                        .ok_or("missing quotient digest")?;
                    let quotient = canonical.get(&(key.0.clone(), key.1, digest.clone()))
                        .ok_or("missing quotient record")?;
                    values.push(invariant(quotient, 1));
                }
                let (ranks, any_tie, minimum_tied) = rank_values(&values);
                any_tie_count += usize::from(any_tie);
                minimum_tie_count += usize::from(minimum_tied);
                let digit = *scalar as usize % radix;
                targets.push(Target {
                    row: (key.0.clone(), key.1),
                    slug: key.0.clone(),
                    scalar: *scalar,
                    digit,
                    true_rank: ranks[digit],
                    ranks,
                });
            }
            let expected = 4575_usize - 15 * radix;
            if targets.len() != expected {
                return Err(format!("radix {radix} target census {}, expected {expected}", targets.len()));
            }

            let mut histogram = vec![0_usize; radix];
            let mut top = vec![0_usize; radix];
            let mut rank_sum = 0_u64;
            let mut sorted_ranks = Vec::with_capacity(targets.len());
            let mut rank_by_scalar = BTreeMap::<(String, u64, u64), usize>::new();
            let mut curve_stats = BTreeMap::<String, (usize, u64)>::new();
            let mut rows = BTreeMap::<(String, u64), Vec<usize>>::new();
            for (index, target) in targets.iter().enumerate() {
                histogram[target.true_rank - 1] += 1;
                for width in target.true_rank..=radix { top[width - 1] += 1; }
                rank_sum += target.true_rank as u64;
                sorted_ranks.push(target.true_rank);
                rank_by_scalar.insert((target.row.0.clone(), target.row.1, target.scalar), target.true_rank);
                let curve = curve_stats.entry(target.slug.clone()).or_default();
                curve.0 += 1;
                curve.1 += target.true_rank as u64;
                rows.entry(target.row.clone()).or_default().push(index);
            }
            sorted_ranks.sort_unstable();

            let mut path_success = vec![0_usize; radix];
            let mut max_depth = 0_usize;
            let mut max_required_width = 0_usize;
            for target in &targets {
                let mut scalar = target.scalar;
                let mut depth = 0_usize;
                let mut required = 0_usize;
                while scalar >= radix as u64 {
                    let rank = *rank_by_scalar.get(&(target.row.0.clone(), target.row.1, scalar))
                        .ok_or("missing recursive rank")?;
                    required = required.max(rank);
                    scalar /= radix as u64;
                    depth += 1;
                }
                for width in required..=radix { path_success[width - 1] += 1; }
                max_depth = max_depth.max(depth);
                max_required_width = max_required_width.max(required);
            }

            let mean_rank = rank_sum as f64 / targets.len() as f64;
            Ok(RadixData {
                radix,
                targets,
                rows,
                histogram,
                top,
                rank_sum,
                mean_rank,
                normalized_mean_rank: mean_rank / radix as f64,
                quantiles: [
                    quantile(&sorted_ranks, 50, 100),
                    quantile(&sorted_ranks, 90, 100),
                    quantile(&sorted_ranks, 99, 100),
                ],
                any_tie_count,
                minimum_tie_count,
                path_success,
                max_depth,
                max_required_width,
                curve_stats,
            })
        }

        pub fn run(moment_path: &str, label_path: &str, output_path: &Path) -> Result<(), String> {
            let records = parse_moments(moment_path)?;
            let labels = parse_labels(label_path)?;
            if records.len() != 9120 || labels.len() != 4560 { return Err("unexpected OOS census".to_string()); }
            let mut pairs: BTreeMap<Key, [Option<Record>; 2]> = BTreeMap::new();
            for record in records {
                let slot = match record.lambda { 1 => 0, 2 => 1, _ => return Err("bad lambda".to_string()) };
                let key = (record.slug.clone(), record.label, record.digest.clone());
                if pairs.entry(key).or_insert_with(|| [None, None])[slot].replace(record).is_some() {
                    return Err("duplicate model record".to_string());
                }
            }
            let mut canonical = BTreeMap::<Key, Record>::new();
            let mut invariant_mismatches = 0_u64;
            for (key, pair) in pairs {
                let left = pair[0].as_ref().ok_or("missing native model")?;
                let right = pair[1].as_ref().ok_or("missing isomorphic model")?;
                for channel in 0..16 {
                    invariant_mismatches += u64::from(invariant(left, channel) != invariant(right, channel));
                }
                canonical.insert(key, left.clone());
            }
            if invariant_mismatches != 0 { return Err("invariant mismatch".to_string()); }
            let mut digest_by_scalar = BTreeMap::<(String, u64, u64), String>::new();
            for (key, scalar) in &labels {
                digest_by_scalar.insert((key.0.clone(), key.1, *scalar), key.2.clone());
            }

            let mut datasets = Vec::new();
            for radix in RADICES { datasets.push(build_radix(radix, &labels, &canonical, &digest_by_scalar)?); }
            let radix_six = datasets.iter().find(|data| data.radix == 6).ok_or("missing radix six")?;
            if radix_six.histogram != [870, 727, 738, 763, 729, 658]
                || radix_six.path_success != [355, 744, 1460, 2345, 3437, 4485]
            {
                return Err("radix-six reconstruction gate failed".to_string());
            }

            let xs: Vec<f64> = datasets.iter().map(|data| (data.radix as f64).ln()).collect();
            let observed_ys: Vec<f64> = datasets.iter().map(|data| data.mean_rank.ln()).collect();
            let observed_slope = log_slope(&xs, &observed_ys);
            let mut rank_sum_exceedances = vec![0_usize; datasets.len()];
            let mut top_log_exceedances = vec![0_usize; datasets.len()];
            let mut slope_exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut null_means = Vec::with_capacity(datasets.len());
                for (data_index, data) in datasets.iter().enumerate() {
                    let mut shuffled_digits = vec![0_usize; data.targets.len()];
                    for ((slug, label), indices) in &data.rows {
                        let mut digits: Vec<usize> = indices.iter().map(|&index| data.targets[index].digit).collect();
                        let mut seed = 0x116_004_u64
                            ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                            ^ (data.radix as u64).wrapping_mul(0xbf58476d1ce4e5b9)
                            ^ *label
                            ^ slug.bytes().map(u64::from).sum::<u64>();
                        shuffle(&mut digits, &mut seed);
                        for (&index, digit) in indices.iter().zip(digits) { shuffled_digits[index] = digit; }
                    }
                    let mut null_rank_sum = 0_u64;
                    let mut null_top_log = 0_usize;
                    let width = ceil_log2(data.radix);
                    for (index, target) in data.targets.iter().enumerate() {
                        let rank = target.ranks[shuffled_digits[index]];
                        null_rank_sum += rank as u64;
                        null_top_log += usize::from(rank <= width);
                    }
                    rank_sum_exceedances[data_index] += usize::from(null_rank_sum <= data.rank_sum);
                    top_log_exceedances[data_index] += usize::from(null_top_log >= data.top[width - 1]);
                    null_means.push((null_rank_sum as f64 / data.targets.len() as f64).ln());
                }
                slope_exceedances += usize::from(log_slope(&xs, &null_means) <= observed_slope);
            }

            let mut radix_json = Vec::new();
            for (index, data) in datasets.iter().enumerate() {
                let curve_json = data.curve_stats.iter().map(|(slug, (targets, rank_sum))| {
                    format!(concat!("{{\"slug\":\"{}\",\"targets\":{},",
                        "\"rank_sum\":{},\"normalized_mean_rank\":{:.12}}}"),
                        slug, targets, rank_sum,
                        *rank_sum as f64 / (*targets as f64 * data.radix as f64))
                }).collect::<Vec<_>>().join(",");
                let width = ceil_log2(data.radix);
                radix_json.push(format!(concat!("{{\"radix\":{},\"targets\":{},",
                    "\"rank_histogram\":{:?},\"top_t_inclusion\":{:?},",
                    "\"rank_sum\":{},\"mean_rank\":{:.12},\"normalized_mean_rank\":{:.12},",
                    "\"rank_quantiles_p50_p90_p99\":{:?},\"any_value_tie_count\":{},",
                    "\"minimum_tie_count\":{},\"path_success\":{:?},\"max_path_depth\":{},",
                    "\"max_required_width\":{},\"top_log2_width\":{},\"top_log2_count\":{},",
                    "\"rank_sum_null_exceedances\":{},\"rank_sum_empirical_p\":{:.12},",
                    "\"top_log2_null_exceedances\":{},\"top_log2_empirical_p\":{:.12},",
                    "\"per_curve\":[{}]}}"),
                    data.radix, data.targets.len(), data.histogram, data.top, data.rank_sum,
                    data.mean_rank, data.normalized_mean_rank, data.quantiles, data.any_tie_count,
                    data.minimum_tie_count, data.path_success, data.max_depth,
                    data.max_required_width, width, data.top[width - 1],
                    rank_sum_exceedances[index],
                    (rank_sum_exceedances[index] + 1) as f64 / (REPLICATES + 1) as f64,
                    top_log_exceedances[index],
                    (top_log_exceedances[index] + 1) as f64 / (REPLICATES + 1) as f64,
                    curve_json));
            }
            let json = format!(concat!("{{\n  \"schema\":\"h116-quotient-multiradix/v1\",\n",
                "  \"radices\":{:?},\n  \"invariant_mismatches\":{},\n",
                "  \"radix_six_histogram_reconstructed\":true,\n",
                "  \"radix_six_paths_reconstructed\":true,\n",
                "  \"log_mean_rank_on_log_radix_slope\":{:.12},\n",
                "  \"slope_null_exceedances\":{},\n  \"slope_empirical_p\":{:.12},\n",
                "  \"replicates\":4096,\n  \"results\":[{}],\n",
                "  \"status\":\"measured\"\n}}\n"),
                RADICES, invariant_mismatches, observed_slope, slope_exceedances,
                (slope_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                radix_json.join(",\n    "));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("quotient multiradix: slope={observed_slope:.6} slope_p={:.6} max_widths={:?}",
                (slope_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                datasets.iter().map(|data| data.max_required_width).collect::<Vec<_>>());
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::multiradix::selftest(),
        [_, flag, moments, labels, output] if flag == "--measure" =>
            frozen::multiradix::run(moments, labels, Path::new(output)),
        _ => Err("usage: h116_quotient_multiradix --selftest | --measure MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient multiradix error: {error}"); std::process::exit(1); }
}
