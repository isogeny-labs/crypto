use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[derive(Clone)]
struct Record {
    slug: String,
    p: u64,
    label: u64,
    digest: String,
    lambda: u64,
    model_a: u64,
    model_b: u64,
    moments: [u64; 16],
}

fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, p: u64) -> u64 {
    let mut value = 1_u64;
    while exponent != 0 {
        if exponent & 1 == 1 { value = mul_mod(value, base, p); }
        base = mul_mod(base, base, p);
        exponent >>= 1;
    }
    value
}

fn invariant(record: &Record, channel: usize) -> u64 {
    let p = record.p;
    let a3 = mul_mod(mul_mod(record.model_a, record.model_a, p), record.model_a, p);
    let b2 = mul_mod(record.model_b, record.model_b, p);
    let discriminant = (mul_mod(4, a3, p) + mul_mod(27 % p, b2, p)) % p;
    assert_ne!(discriminant, 0);
    let (numerator, denominator) = if channel < 8 {
        (pow_mod(record.moments[channel], 6, p), pow_mod(discriminant, channel as u64 + 1, p))
    } else {
        let j = channel - 8;
        (pow_mod(record.moments[channel], 12, p), pow_mod(discriminant, 2 * j as u64 + 3, p))
    };
    mul_mod(numerator, pow_mod(denominator, p - 2, p), p)
}

fn permutations() -> Vec<[u8; 6]> {
    fn visit(index: usize, value: &mut [u8; 6], output: &mut Vec<[u8; 6]>) {
        if index == value.len() { output.push(*value); return; }
        for swap in index..value.len() {
            value.swap(index, swap);
            visit(index + 1, value, output);
            value.swap(index, swap);
        }
    }
    let mut output = Vec::new();
    visit(0, &mut [0, 1, 2, 3, 4, 5], &mut output);
    output
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn parse_moments(path: &str) -> Result<Vec<Record>, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    if lines.next() != Some("h116-invariant-moments/v1") { return Err("bad moment header".to_string()); }
    lines.map(|line| {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 28 { return Err(format!("bad moment field count: {}", fields.len())); }
        let number = |index: usize| fields[index].parse::<u64>().map_err(|error| error.to_string());
        let mut moments = [0_u64; 16];
        for (index, value) in moments.iter_mut().enumerate() { *value = number(12 + index)?; }
        Ok(Record {
            slug: fields[0].to_string(), p: number(1)?, label: number(7)?,
            digest: fields[8].to_string(), lambda: number(9)?, model_a: number(10)?,
            model_b: number(11)?, moments,
        })
    }).collect()
}

fn parse_labels(path: &str) -> Result<BTreeMap<(String, u64, String), u64>, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut lines = text.lines();
    if lines.next() != Some("h116-invariant-labels/v1") { return Err("bad label header".to_string()); }
    lines.map(|line| {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 4 { return Err("bad label field count".to_string()); }
        let label = fields[1].parse::<u64>().map_err(|error| error.to_string())?;
        let scalar = fields[3].parse::<u64>().map_err(|error| error.to_string())?;
        Ok(((fields[0].to_string(), label, fields[2].to_string()), scalar))
    }).collect()
}

fn predicate_id(channel: usize, permutation: &[u8; 6]) -> String {
    let kind = if channel < 8 { format!("IX{}", channel) } else { format!("IY{}", channel - 8) };
    format!("{kind}-{}{}{}{}{}{}", permutation[0], permutation[1], permutation[2], permutation[3], permutation[4], permutation[5])
}

fn count_digit_matches(bins: &[usize], digits: &[u8], permutation: &[u8; 6]) -> usize {
    bins.iter().zip(digits).filter(|(bin, digit)| permutation[**bin] == **digit).count()
}

fn selftest() -> Result<(), String> {
    let all = permutations();
    assert_eq!(all.len(), 720);
    assert_eq!(all.iter().copied().collect::<BTreeSet<_>>().len(), 720);
    let bins: Vec<usize> = (0..24).map(|index| index % 6).collect();
    let digits: Vec<u8> = bins.iter().map(|&bin| bin as u8).collect();
    let exact: Vec<[u8; 6]> = all.iter().copied()
        .filter(|permutation| count_digit_matches(&bins, &digits, permutation) == bins.len())
        .collect();
    assert_eq!(exact, vec![[0, 1, 2, 3, 4, 5]]);
    let mut mutated = digits.clone();
    mutated[0] = 1;
    assert!(all.iter().all(|permutation| {
        count_digit_matches(&bins, &mutated, permutation) < bins.len()
    }));
    println!("h116 invariant moment scorer selftest: pass");
    Ok(())
}

fn score(moment_path: &str, label_path: &str, output_path: &str) -> Result<(), String> {
    let records = parse_moments(moment_path)?;
    let labels = parse_labels(label_path)?;
    if records.len() != 576 || labels.len() != 288 { return Err("unexpected census size".to_string()); }
    let mut paired: BTreeMap<(String, u64, String), [Option<Record>; 2]> = BTreeMap::new();
    for record in records {
        let slot = if record.lambda == 1 { 0 } else if record.lambda == 2 { 1 } else { return Err("bad lambda".to_string()); };
        let key = (record.slug.clone(), record.label, record.digest.clone());
        paired.entry(key).or_insert_with(|| [None, None])[slot] = Some(record);
    }
    let mut canonical = Vec::new();
    let mut invariant_mismatches = 0_u64;
    for (key, pair) in paired {
        let left = pair[0].as_ref().ok_or("missing native model")?;
        let right = pair[1].as_ref().ok_or("missing isomorphic model")?;
        for channel in 0..16 { invariant_mismatches += u64::from(invariant(left, channel) != invariant(right, channel)); }
        let scalar = *labels.get(&key).ok_or("missing scalar label")?;
        canonical.push((key, left.clone(), scalar));
    }
    if invariant_mismatches != 0 { return Err(format!("invariant mismatch count {invariant_mismatches}")); }
    let mut control_digits = BTreeMap::new();
    let mut rows: BTreeMap<(String, u64), Vec<((String, u64, String), u64)>> = BTreeMap::new();
    for (key, _, scalar) in &canonical { rows.entry((key.0.clone(), key.1)).or_default().push((key.clone(), scalar % 6)); }
    for ((slug, label), values) in &mut rows {
        values.sort_by_key(|item| item.1);
        let mut shuffled: Vec<u64> = values.iter().map(|item| item.1).collect();
        let mut seed = 0x116_001_u64 ^ *label ^ slug.bytes().map(u64::from).sum::<u64>();
        for index in (1..shuffled.len()).rev() {
            let swap = (splitmix64(&mut seed) % (index as u64 + 1)) as usize;
            shuffled.swap(index, swap);
        }
        for ((key, _), digit) in values.iter().zip(shuffled) { control_digits.insert(key.clone(), digit); }
    }
    let permutations = permutations();
    let mut best = 0_usize;
    let mut best_ids = Vec::new();
    let mut channel_best = [0_usize; 16];
    let mut exact = Vec::new();
    let mut p31_exact = BTreeSet::new();
    let mut p47_exact = BTreeSet::new();
    let mut control_exact = Vec::new();
    let mut control_best = 0_usize;
    let mut control_best_ids = Vec::new();
    for channel in 0..16 {
        for permutation in &permutations {
            let id = predicate_id(channel, permutation);
            let matches = canonical.iter().filter(|(_, record, scalar)| {
                let value = invariant(record, channel);
                let bin = ((value as u128 * 6) / record.p as u128).min(5) as usize;
                permutation[bin] as u64 == scalar % 6
            }).count();
            channel_best[channel] = channel_best[channel].max(matches);
            if matches > best {
                best = matches;
                best_ids.clear();
                best_ids.push(id.clone());
            } else if matches == best {
                best_ids.push(id.clone());
            }
            if matches == canonical.len() { exact.push(id.clone()); }
            if canonical.iter().filter(|(_, record, _)| record.p == 31).all(|(_, record, scalar)| {
                let bin = ((invariant(record, channel) as u128 * 6) / record.p as u128).min(5) as usize;
                permutation[bin] as u64 == scalar % 6
            }) { p31_exact.insert(id.clone()); }
            if canonical.iter().filter(|(_, record, _)| record.p == 47).all(|(_, record, scalar)| {
                let bin = ((invariant(record, channel) as u128 * 6) / record.p as u128).min(5) as usize;
                permutation[bin] as u64 == scalar % 6
            }) { p47_exact.insert(id.clone()); }
            let control_matches = canonical.iter().filter(|(key, record, _)| {
                let bin = ((invariant(record, channel) as u128 * 6) / record.p as u128).min(5) as usize;
                permutation[bin] as u64 == *control_digits.get(key).expect("control label")
            }).count();
            if control_matches > control_best {
                control_best = control_matches;
                control_best_ids.clear();
                control_best_ids.push(id.clone());
            } else if control_matches == control_best {
                control_best_ids.push(id.clone());
            }
            if control_matches == canonical.len() { control_exact.push(id); }
        }
    }
    let survivors: Vec<&String> = p31_exact.intersection(&p47_exact).collect();
    let json = format!(
        concat!("{{\n  \"schema\":\"h116-invariant-predicate-result/v1\",\n",
        "  \"records\":{},\n  \"rows\":{},\n  \"channels\":16,\n  \"permutations\":720,\n",
        "  \"predicates\":11520,\n  \"invariant_mismatches\":{},\n  \"best_matches\":{},\n",
        "  \"best_rate\":{:.12},\n  \"best_ids\":{:?},\n  \"channel_best_matches\":{:?},\n",
        "  \"shuffled_control_best_matches\":{},\n  \"shuffled_control_best_rate\":{:.12},\n",
        "  \"shuffled_control_best_ids\":{:?},\n",
        "  \"uniform_exact_count\":{},\n  \"p31_exact_count\":{},\n  \"p47_exact_count\":{},\n",
        "  \"cross_curve_survivor_count\":{},\n  \"shuffled_control_exact_count\":{},\n",
        "  \"uniform_exact_ids\":{:?},\n  \"status\":\"{}\"\n}}\n"),
        canonical.len(), rows.len(), invariant_mismatches, best, best as f64 / canonical.len() as f64,
        best_ids, channel_best, control_best, control_best as f64 / canonical.len() as f64,
        control_best_ids, exact.len(), p31_exact.len(), p47_exact.len(), survivors.len(),
        control_exact.len(), exact,
        if control_exact.is_empty() { if exact.is_empty() { "no_exact_predicate" } else { "candidate" } } else { "invalid_control" },
    );
    fs::write(output_path, json).map_err(|error| error.to_string())?;
    println!("invariant scorer: best={best}/{} exact={} shuffled_control_exact={}", canonical.len(), exact.len(), control_exact.len());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, flag, moments, labels, output] if flag == "--score" => score(moments, labels, output),
        _ => Err("usage: h116_invariant_moment_scorer --selftest | --score MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 invariant moment scorer error: {error}"); std::process::exit(1); }
}
