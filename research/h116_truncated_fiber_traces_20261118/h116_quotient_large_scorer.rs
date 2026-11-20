#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_quotient_digit_scorer.rs");

    pub mod large {
        use super::*;
        use std::path::Path;

        const RADICES: [usize; 9] = [2, 3, 4, 5, 6, 8, 10, 12, 16];
        const REPLICATES: usize = 4096;
        const LABEL_CENSUS: usize = 34_856;
        const RECORD_CENSUS: usize = 69_712;
        const ORDER_SUM: usize = 34_859;

        #[derive(Clone)]
        struct Target {
            slug: String,
            scalar: u64,
            digit: usize,
            ranks: Vec<usize>,
            true_rank: usize,
        }

        struct RadixData {
            radix: usize,
            targets: Vec<Target>,
            rows: BTreeMap<String, Vec<usize>>,
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
            let (ranks, any_tie, minimum_tied) = rank_values(&[5, 1, 1, 3]);
            assert_eq!(ranks, [4, 1, 2, 3]);
            assert!(any_tie && minimum_tied);
            assert_eq!((ceil_log2(2), ceil_log2(5), ceil_log2(16)), (1, 3, 4));
            assert_eq!(ORDER_SUM - 3 * 16, 34_811);
            for radix in RADICES {
                for scalar in radix as u64..=1000 {
                    assert_eq!(scalar, radix as u64 * (scalar / radix as u64) + scalar % radix as u64);
                }
            }
            let xs: Vec<f64> = RADICES.iter().map(|&value| (value as f64).ln()).collect();
            assert!((log_slope(&xs, &xs) - 1.0).abs() < 1e-12);
            println!("h116 quotient large scorer selftest: pass");
            Ok(())
        }

        fn build_radix(
            radix: usize,
            labels: &BTreeMap<Key, u64>,
            canonical: &BTreeMap<Key, Record>,
            digest_by_scalar: &BTreeMap<(String, u64), String>,
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
                    let digest = digest_by_scalar.get(&(key.0.clone(), quotient_scalar))
                        .ok_or("missing quotient digest")?;
                    let quotient = canonical.get(&(key.0.clone(), 1, digest.clone()))
                        .ok_or("missing quotient record")?;
                    values.push(invariant(quotient, 1));
                }
                let (ranks, any_tie, minimum_tied) = rank_values(&values);
                any_tie_count += usize::from(any_tie);
                minimum_tie_count += usize::from(minimum_tied);
                let digit = *scalar as usize % radix;
                targets.push(Target {
                    slug: key.0.clone(),
                    scalar: *scalar,
                    digit,
                    true_rank: ranks[digit],
                    ranks,
                });
            }
            let expected = ORDER_SUM - 3 * radix;
            if targets.len() != expected {
                return Err(format!("radix {radix} target census {}, expected {expected}", targets.len()));
            }

            let mut histogram = vec![0_usize; radix];
            let mut top = vec![0_usize; radix];
            let mut rank_sum = 0_u64;
            let mut sorted_ranks = Vec::with_capacity(targets.len());
            let mut rank_by_scalar = BTreeMap::<(String, u64), usize>::new();
            let mut curve_stats = BTreeMap::<String, (usize, u64)>::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            for (index, target) in targets.iter().enumerate() {
                histogram[target.true_rank - 1] += 1;
                for width in target.true_rank..=radix { top[width - 1] += 1; }
                rank_sum += target.true_rank as u64;
                sorted_ranks.push(target.true_rank);
                rank_by_scalar.insert((target.slug.clone(), target.scalar), target.true_rank);
                let curve = curve_stats.entry(target.slug.clone()).or_default();
                curve.0 += 1;
                curve.1 += target.true_rank as u64;
                rows.entry(target.slug.clone()).or_default().push(index);
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
                    let rank = *rank_by_scalar.get(&(target.slug.clone(), scalar))
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
                quantiles: [quantile(&sorted_ranks, 50, 100), quantile(&sorted_ranks, 90, 100), quantile(&sorted_ranks, 99, 100)],
                any_tie_count,
                minimum_tie_count,
                path_success,
                max_depth,
                max_required_width,
                curve_stats,
            })
        }

        pub fn score(moment_path: &str, label_path: &str, output_path: &Path) -> Result<(), String> {
            let records = parse_moments(moment_path)?;
            let labels = parse_labels(label_path)?;
            if records.len() != RECORD_CENSUS || labels.len() != LABEL_CENSUS {
                return Err("unexpected large-panel census".to_string());
            }
            let mut pairs: BTreeMap<Key, [Option<Record>; 2]> = BTreeMap::new();
            for record in records {
                if record.label != 1 { return Err("unexpected generator label".to_string()); }
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
            let mut digest_by_scalar = BTreeMap::<(String, u64), String>::new();
            for (key, scalar) in &labels {
                digest_by_scalar.insert((key.0.clone(), *scalar), key.2.clone());
            }

            let mut datasets = Vec::new();
            for radix in RADICES { datasets.push(build_radix(radix, &labels, &canonical, &digest_by_scalar)?); }
            let xs: Vec<f64> = datasets.iter().map(|data| (data.radix as f64).ln()).collect();
            let observed_slope = log_slope(&xs, &datasets.iter().map(|data| data.mean_rank.ln()).collect::<Vec<_>>());
            let slugs: Vec<String> = datasets[0].curve_stats.keys().cloned().collect();
            if slugs.len() != 3 { return Err("unexpected curve count".to_string()); }
            let mut curve_slopes = BTreeMap::<String, f64>::new();
            for slug in &slugs {
                let ys: Vec<f64> = datasets.iter().map(|data| {
                    let (count, sum) = data.curve_stats.get(slug).expect("curve statistic");
                    (*sum as f64 / *count as f64).ln()
                }).collect();
                curve_slopes.insert(slug.clone(), log_slope(&xs, &ys));
            }

            let mut rank_sum_exceedances = vec![0_usize; datasets.len()];
            let mut top_log_exceedances = vec![0_usize; datasets.len()];
            let mut slope_exceedances = 0_usize;
            let mut curve_slope_exceedances: BTreeMap<String, usize> =
                slugs.iter().map(|slug| (slug.clone(), 0)).collect();
            for replicate in 0..REPLICATES {
                let mut null_means = Vec::with_capacity(datasets.len());
                let mut curve_null_logs: BTreeMap<String, Vec<f64>> =
                    slugs.iter().map(|slug| (slug.clone(), Vec::with_capacity(datasets.len()))).collect();
                for (data_index, data) in datasets.iter().enumerate() {
                    let mut shuffled_digits = vec![0_usize; data.targets.len()];
                    for (slug, indices) in &data.rows {
                        let mut digits: Vec<usize> = indices.iter().map(|&index| data.targets[index].digit).collect();
                        let mut seed = 0x116_005_u64
                            ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                            ^ (data.radix as u64).wrapping_mul(0xbf58476d1ce4e5b9)
                            ^ slug.bytes().map(u64::from).sum::<u64>();
                        shuffle(&mut digits, &mut seed);
                        for (&index, digit) in indices.iter().zip(digits) { shuffled_digits[index] = digit; }
                    }
                    let mut null_rank_sum = 0_u64;
                    let mut null_top_log = 0_usize;
                    let mut curve_sums: BTreeMap<String, u64> = slugs.iter().map(|slug| (slug.clone(), 0)).collect();
                    let width = ceil_log2(data.radix);
                    for (index, target) in data.targets.iter().enumerate() {
                        let rank = target.ranks[shuffled_digits[index]];
                        null_rank_sum += rank as u64;
                        null_top_log += usize::from(rank <= width);
                        *curve_sums.get_mut(&target.slug).expect("curve sum") += rank as u64;
                    }
                    rank_sum_exceedances[data_index] += usize::from(null_rank_sum <= data.rank_sum);
                    top_log_exceedances[data_index] += usize::from(null_top_log >= data.top[width - 1]);
                    null_means.push((null_rank_sum as f64 / data.targets.len() as f64).ln());
                    for slug in &slugs {
                        let (count, _) = data.curve_stats.get(slug).expect("curve count");
                        curve_null_logs.get_mut(slug).expect("curve logs")
                            .push((*curve_sums.get(slug).expect("curve sum") as f64 / *count as f64).ln());
                    }
                }
                slope_exceedances += usize::from(log_slope(&xs, &null_means) <= observed_slope);
                for slug in &slugs {
                    let null_slope = log_slope(&xs, curve_null_logs.get(slug).expect("curve null logs"));
                    *curve_slope_exceedances.get_mut(slug).expect("curve exceedance") +=
                        usize::from(null_slope <= *curve_slopes.get(slug).expect("curve slope"));
                }
            }

            let curve_slopes_json = slugs.iter().map(|slug| {
                let exceedances = *curve_slope_exceedances.get(slug).expect("curve exceedances");
                format!(concat!("{{\"slug\":\"{}\",\"slope\":{:.12},",
                    "\"null_exceedances\":{},\"empirical_p\":{:.12}}}"),
                    slug, curve_slopes[slug], exceedances,
                    (exceedances + 1) as f64 / (REPLICATES + 1) as f64)
            }).collect::<Vec<_>>().join(",");

            let mut radix_json = Vec::new();
            for (index, data) in datasets.iter().enumerate() {
                let curve_json = data.curve_stats.iter().map(|(slug, (targets, rank_sum))| {
                    format!(concat!("{{\"slug\":\"{}\",\"targets\":{},\"rank_sum\":{},",
                        "\"normalized_mean_rank\":{:.12}}}"), slug, targets, rank_sum,
                        *rank_sum as f64 / (*targets as f64 * data.radix as f64))
                }).collect::<Vec<_>>().join(",");
                let width = ceil_log2(data.radix);
                radix_json.push(format!(concat!("{{\"radix\":{},\"targets\":{},",
                    "\"rank_histogram\":{:?},\"top_t_inclusion\":{:?},",
                    "\"rank_sum\":{},\"mean_rank\":{:.12},\"normalized_mean_rank\":{:.12},",
                    "\"rank_quantiles_p50_p90_p99\":{:?},\"any_value_tie_count\":{},",
                    "\"minimum_tie_count\":{},\"path_success\":{:?},\"max_path_depth\":{},",
                    "\"max_required_width\":{},\"required_width_fraction\":{:.12},",
                    "\"top_log2_width\":{},\"top_log2_count\":{},",
                    "\"rank_sum_null_exceedances\":{},\"rank_sum_empirical_p\":{:.12},",
                    "\"top_log2_null_exceedances\":{},\"top_log2_empirical_p\":{:.12},",
                    "\"per_curve\":[{}]}}"),
                    data.radix, data.targets.len(), data.histogram, data.top, data.rank_sum,
                    data.mean_rank, data.normalized_mean_rank, data.quantiles, data.any_tie_count,
                    data.minimum_tie_count, data.path_success, data.max_depth,
                    data.max_required_width, data.max_required_width as f64 / data.radix as f64,
                    width, data.top[width - 1], rank_sum_exceedances[index],
                    (rank_sum_exceedances[index] + 1) as f64 / (REPLICATES + 1) as f64,
                    top_log_exceedances[index],
                    (top_log_exceedances[index] + 1) as f64 / (REPLICATES + 1) as f64,
                    curve_json));
            }
            let json = format!(concat!("{{\n  \"schema\":\"h116-quotient-large-panel/v1\",\n",
                "  \"radices\":{:?},\n  \"source_targets\":{},\n  \"source_records\":{},\n",
                "  \"invariant_mismatches\":{},\n  \"log_mean_rank_on_log_radix_slope\":{:.12},\n",
                "  \"slope_null_exceedances\":{},\n  \"slope_empirical_p\":{:.12},\n",
                "  \"per_curve_slopes\":[{}],\n  \"replicates\":4096,\n",
                "  \"results\":[{}],\n  \"status\":\"measured\"\n}}\n"),
                RADICES, LABEL_CENSUS, RECORD_CENSUS, invariant_mismatches, observed_slope,
                slope_exceedances, (slope_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                curve_slopes_json, radix_json.join(",\n    "));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("quotient large scorer: slope={observed_slope:.6} slope_p={:.6} max_widths={:?} max_depths={:?}",
                (slope_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                datasets.iter().map(|data| data.max_required_width).collect::<Vec<_>>(),
                datasets.iter().map(|data| data.max_depth).collect::<Vec<_>>());
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::large::selftest(),
        [_, flag, moments, labels, output] if flag == "--score" =>
            frozen::large::score(moments, labels, Path::new(output)),
        _ => Err("usage: h116_quotient_large_scorer --selftest | --score MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient large scorer error: {error}"); std::process::exit(1); }
}
