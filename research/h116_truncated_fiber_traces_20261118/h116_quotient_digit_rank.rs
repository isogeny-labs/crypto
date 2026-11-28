#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_quotient_digit_scorer.rs");

    pub mod rank {
        use super::*;
        use std::path::Path;

        const REPLICATES: usize = 4096;
        const WEIGHTS: [u64; 6] = [60, 30, 20, 15, 12, 10];

        #[derive(Clone)]
        struct Target {
            row: (String, u64),
            slug: String,
            scalar: u64,
            digit: u8,
            ranks: [u8; 6],
            true_rank: u8,
            any_value_tie: bool,
            minimum_tied: bool,
            unique_minimum_prediction: Option<u8>,
        }

        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        fn ranks(values: &[u64; 6]) -> ([u8; 6], bool) {
            let mut order = [(0_u64, 0_u8); 6];
            for digit in 0..6 { order[digit] = (values[digit], digit as u8); }
            order.sort_unstable();
            let mut output = [0_u8; 6];
            for (index, &(_, digit)) in order.iter().enumerate() { output[digit as usize] = index as u8 + 1; }
            let tied = order.windows(2).any(|window| window[0].0 == window[1].0);
            (output, tied)
        }

        pub fn selftest() -> Result<(), String> {
            let (rank, tied) = ranks(&[9, 1, 1, 7, 5, 3]);
            assert!(tied);
            assert_eq!(rank, [6, 1, 2, 5, 4, 3]);
            assert_eq!(WEIGHTS, [60, 30, 20, 15, 12, 10]);
            let synthetic = [1_u8, 2, 1, 4];
            assert_eq!(*synthetic.iter().max().expect("path"), 4);
            println!("h116 quotient digit rank selftest: pass");
            Ok(())
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
            if invariant_mismatches != 0 { return Err("invariant mismatch".to_string()); }
            let mut digest_by_scalar: BTreeMap<(String, u64, u64), String> = BTreeMap::new();
            for (key, scalar) in &labels {
                digest_by_scalar.insert((key.0.clone(), key.1, *scalar), key.2.clone());
            }
            let mut targets = Vec::new();
            for (key, scalar) in &labels {
                if *scalar < 6 { continue; }
                let target_record = canonical.get(key).ok_or("missing target")?;
                let inverse_six = pow_mod(6, target_record.order - 2, target_record.order);
                let mut values = [0_u64; 6];
                for digit in 0_u64..6 {
                    let quotient_scalar = mul_mod(
                        (scalar + target_record.order - digit) % target_record.order,
                        inverse_six,
                        target_record.order,
                    );
                    let digest = digest_by_scalar.get(&(key.0.clone(), key.1, quotient_scalar))
                        .ok_or("missing quotient digest")?;
                    let quotient = canonical.get(&(key.0.clone(), key.1, digest.clone()))
                        .ok_or("missing quotient record")?;
                    values[digit as usize] = invariant(quotient, 1);
                }
                let (all_ranks, any_value_tie) = ranks(&values);
                let minimum = *values.iter().min().ok_or("empty candidate values")?;
                let minimum_tied = values.iter().filter(|&&value| value == minimum).count() != 1;
                let unique_minimum_prediction = unique_extreme(&values, false);
                let digit = (scalar % 6) as u8;
                targets.push(Target {
                    row: (key.0.clone(), key.1), slug: key.0.clone(), scalar: *scalar, digit,
                    ranks: all_ranks,
                    true_rank: all_ranks[digit as usize],
                    any_value_tie,
                    minimum_tied,
                    unique_minimum_prediction,
                });
            }
            if targets.len() != 4485 { return Err("unexpected target census".to_string()); }
            let selected_coverage = targets.iter()
                .filter(|target| target.unique_minimum_prediction.is_some())
                .count();
            let selected_matches = targets.iter()
                .filter(|target| target.unique_minimum_prediction == Some(target.digit))
                .count();
            let any_value_tie_count = targets.iter().filter(|target| target.any_value_tie).count();
            let minimum_tie_count = targets.iter().filter(|target| target.minimum_tied).count();

            let mut histogram = [0_usize; 6];
            let mut top = [0_usize; 6];
            let mut observed_mrr60 = 0_u64;
            for target in &targets {
                histogram[target.true_rank as usize - 1] += 1;
                observed_mrr60 += WEIGHTS[target.true_rank as usize - 1];
                for width in target.true_rank as usize..=6 { top[width - 1] += 1; }
            }
            // The total-order rank-one count must include the earlier unique-minimum successes
            // plus deterministic wins inside tied minima, so it cannot be used as the old gate.
            if selected_coverage != 4328 || selected_matches != 845 {
                return Err("retained selected-rule gate failed".to_string());
            }

            let mut rank_by_scalar: BTreeMap<(String, u64, u64), u8> = BTreeMap::new();
            for target in &targets {
                rank_by_scalar.insert((target.row.0.clone(), target.row.1, target.scalar), target.true_rank);
            }
            let mut path_success = [0_usize; 6];
            let mut depth_histogram = BTreeMap::<usize, usize>::new();
            let mut per_curve_paths = BTreeMap::<String, [usize; 6]>::new();
            let mut max_depth = 0_usize;
            let mut max_required_width = 0_u8;
            for target in &targets {
                let mut scalar = target.scalar;
                let mut depth = 0_usize;
                let mut required = 0_u8;
                while scalar >= 6 {
                    let rank = *rank_by_scalar.get(&(target.row.0.clone(), target.row.1, scalar))
                        .ok_or("missing recursive rank")?;
                    required = required.max(rank);
                    scalar /= 6;
                    depth += 1;
                }
                *depth_histogram.entry(depth).or_default() += 1;
                max_depth = max_depth.max(depth);
                max_required_width = max_required_width.max(required);
                for width in required as usize..=6 {
                    path_success[width - 1] += 1;
                    per_curve_paths.entry(target.slug.clone()).or_insert([0; 6])[width - 1] += 1;
                }
            }

            let mut rows = BTreeMap::<(String, u64), Vec<usize>>::new();
            for (index, target) in targets.iter().enumerate() {
                rows.entry(target.row.clone()).or_default().push(index);
            }
            let observed_top1 = top[0];
            let observed_top2 = top[1];
            let mut mrr_exceedances = 0_usize;
            let mut top1_exceedances = 0_usize;
            let mut top2_exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled_digits = vec![0_u8; targets.len()];
                for ((slug, label), indices) in &rows {
                    let mut digits: Vec<u8> = indices.iter().map(|&index| targets[index].digit).collect();
                    let mut seed = 0x116_003_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ *label
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut digits, &mut seed);
                    for (&index, digit) in indices.iter().zip(digits) { shuffled_digits[index] = digit; }
                }
                let mut mrr60 = 0_u64;
                let mut top1_count = 0_usize;
                let mut top2_count = 0_usize;
                for (index, target) in targets.iter().enumerate() {
                    let rank = target.ranks[shuffled_digits[index] as usize];
                    mrr60 += WEIGHTS[rank as usize - 1];
                    top1_count += usize::from(rank == 1);
                    top2_count += usize::from(rank <= 2);
                }
                mrr_exceedances += usize::from(mrr60 >= observed_mrr60);
                top1_exceedances += usize::from(top1_count >= observed_top1);
                top2_exceedances += usize::from(top2_count >= observed_top2);
            }

            let exponents: Vec<f64> = (1..=6).map(|width| (width as f64).ln() / 6_f64.ln()).collect();
            let depth_json = depth_histogram.iter().map(|(depth, count)| format!("\"{depth}\":{count}"))
                .collect::<Vec<_>>().join(",");
            let curve_json = per_curve_paths.iter().map(|(slug, counts)| {
                format!("{{\"slug\":\"{slug}\",\"path_success\":{:?}}}", counts)
            }).collect::<Vec<_>>().join(",");
            let json = format!(
                concat!("{{\n  \"schema\":\"h116-quotient-digit-rank/v1\",\n",
                "  \"targets\":{},\n  \"rows\":{},\n  \"invariant_mismatches\":{},\n",
                "  \"retained_unique_min_coverage\":{},\n  \"retained_unique_min_matches\":{},\n",
                "  \"any_value_tie_count\":{},\n  \"minimum_tie_count\":{},\n",
                "  \"rank_histogram\":{:?},\n  \"top_t_inclusion\":{:?},\n",
                "  \"observed_mrr60\":{},\n  \"mean_reciprocal_rank\":{:.12},\n",
                "  \"path_success\":{:?},\n  \"path_success_rates\":{:?},\n",
                "  \"max_path_depth\":{},\n  \"depth_histogram\":{{{}}},\n",
                "  \"max_required_branching_width\":{},\n  \"branching_exponents\":{:?},\n",
                "  \"per_curve_paths\":[{}],\n  \"replicates\":4096,\n",
                "  \"mrr_exceedances\":{},\n  \"mrr_empirical_p\":{:.12},\n",
                "  \"top1_exceedances\":{},\n  \"top1_empirical_p\":{:.12},\n",
                "  \"top2_exceedances\":{},\n  \"top2_empirical_p\":{:.12},\n",
                "  \"status\":\"measured\"\n}}\n"),
                targets.len(), rows.len(), invariant_mismatches, selected_coverage, selected_matches,
                any_value_tie_count, minimum_tie_count,
                histogram, top, observed_mrr60,
                observed_mrr60 as f64 / (60.0 * targets.len() as f64), path_success,
                path_success.map(|count| count as f64 / targets.len() as f64), max_depth,
                depth_json, max_required_width, exponents, curve_json, mrr_exceedances,
                (mrr_exceedances + 1) as f64 / (REPLICATES + 1) as f64, top1_exceedances,
                (top1_exceedances + 1) as f64 / (REPLICATES + 1) as f64, top2_exceedances,
                (top2_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
            );
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!(
                "quotient rank: histogram={histogram:?} path_success={path_success:?} max_width={max_required_width} mrr_p={:.6}",
                (mrr_exceedances + 1) as f64 / (REPLICATES + 1) as f64,
            );
            Ok(())
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::rank::selftest(),
        [_, flag, moments, labels, output] if flag == "--rank" =>
            frozen::rank::run(moments, labels, Path::new(output)),
        _ => Err("usage: h116_quotient_digit_rank --selftest | --rank MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient rank error: {error}"); std::process::exit(1); }
}
