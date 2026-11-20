use std::collections::{BTreeMap, BTreeSet};
use std::fs;

#[derive(Clone)]
struct Record {
    slug: String,
    p: u64,
    order: u64,
    label: u64,
    digest: String,
    lambda: u64,
    model_a: u64,
    model_b: u64,
    moments: [u64; 16],
}

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut value = 1_u64;
    while exponent != 0 {
        if exponent & 1 == 1 { value = mul_mod(value, base, modulus); }
        base = mul_mod(base, base, modulus);
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
            slug: fields[0].to_string(), p: number(1)?, order: number(4)?, label: number(7)?,
            digest: fields[8].to_string(), lambda: number(9)?, model_a: number(10)?,
            model_b: number(11)?, moments,
        })
    }).collect()
}

type Key = (String, u64, String);

fn parse_labels(path: &str) -> Result<BTreeMap<Key, u64>, String> {
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

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn unique_extreme(values: &[u64; 6], maximum: bool) -> Option<u8> {
    let extreme = if maximum {
        *values.iter().max().expect("six values")
    } else {
        *values.iter().min().expect("six values")
    };
    let positions: Vec<usize> = values.iter().enumerate()
        .filter_map(|(index, value)| (*value == extreme).then_some(index))
        .collect();
    (positions.len() == 1).then_some(positions[0] as u8)
}

fn rule_id(channel: usize, maximum: bool) -> String {
    let kind = if channel < 8 { format!("IX{}", channel) } else { format!("IY{}", channel - 8) };
    format!("{kind}-{}", if maximum { "argmax" } else { "argmin" })
}

fn selftest() -> Result<(), String> {
    let minimum = [9, 8, 7, 1, 6, 5];
    let maximum = [1, 2, 3, 9, 5, 6];
    let tied = [1, 1, 4, 5, 6, 7];
    assert_eq!(unique_extreme(&minimum, false), Some(3));
    assert_eq!(unique_extreme(&maximum, true), Some(3));
    assert_eq!(unique_extreme(&tied, false), None);
    println!("h116 quotient digit scorer selftest: pass");
    Ok(())
}

fn score(moment_path: &str, label_path: &str, output_path: &str) -> Result<(), String> {
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

    #[derive(Clone)]
    struct Target { key: Key, p: u64, digit: u8, predictions: [Option<u8>; 32] }
    let mut targets = Vec::new();
    for (key, scalar) in &labels {
        if *scalar < 6 { continue; }
        let target_record = canonical.get(key).ok_or("missing target record")?;
        let inverse_six = pow_mod(6, target_record.order - 2, target_record.order);
        let mut values = [[0_u64; 6]; 16];
        for digit in 0_u64..6 {
            let quotient_scalar = mul_mod((scalar + target_record.order - digit) % target_record.order, inverse_six, target_record.order);
            if quotient_scalar == 0 { return Err("unexpected identity quotient".to_string()); }
            let digest = digest_by_scalar.get(&(key.0.clone(), key.1, quotient_scalar))
                .ok_or("missing quotient digest")?;
            let quotient_key = (key.0.clone(), key.1, digest.clone());
            let quotient = canonical.get(&quotient_key).ok_or("missing quotient record")?;
            for (channel, row) in values.iter_mut().enumerate() { row[digit as usize] = invariant(quotient, channel); }
        }
        let mut predictions = [None; 32];
        for channel in 0..16 {
            predictions[2 * channel] = unique_extreme(&values[channel], false);
            predictions[2 * channel + 1] = unique_extreme(&values[channel], true);
        }
        targets.push(Target { key: key.clone(), p: target_record.p, digit: (scalar % 6) as u8, predictions });
    }
    if targets.len() != 258 { return Err(format!("unexpected target census: {}", targets.len())); }

    let mut control_digits: BTreeMap<Key, u8> = BTreeMap::new();
    let mut rows: BTreeMap<(String, u64), Vec<(Key, u8)>> = BTreeMap::new();
    for target in &targets {
        rows.entry((target.key.0.clone(), target.key.1)).or_default().push((target.key.clone(), target.digit));
    }
    for ((slug, label), row) in &mut rows {
        row.sort_by(|left, right| left.0.cmp(&right.0));
        let mut shuffled: Vec<u8> = row.iter().map(|item| item.1).collect();
        let mut seed = 0x116_002_u64 ^ *label ^ slug.bytes().map(u64::from).sum::<u64>();
        for index in (1..shuffled.len()).rev() {
            let swap = (splitmix64(&mut seed) % (index as u64 + 1)) as usize;
            shuffled.swap(index, swap);
        }
        for ((key, _), digit) in row.iter().zip(shuffled) { control_digits.insert(key.clone(), digit); }
    }

    let mut matches = [0_usize; 32];
    let mut coverage = [0_usize; 32];
    let mut control_matches = [0_usize; 32];
    let mut p31_exact = BTreeSet::new();
    let mut p47_exact = BTreeSet::new();
    for rule in 0..32 {
        for target in &targets {
            if let Some(prediction) = target.predictions[rule] {
                coverage[rule] += 1;
                matches[rule] += usize::from(prediction == target.digit);
                control_matches[rule] += usize::from(
                    prediction == *control_digits.get(&target.key).expect("control digit")
                );
            }
        }
        let id = rule_id(rule / 2, rule % 2 == 1);
        if targets.iter().filter(|target| target.p == 31).all(|target| target.predictions[rule] == Some(target.digit)) {
            p31_exact.insert(id.clone());
        }
        if targets.iter().filter(|target| target.p == 47).all(|target| target.predictions[rule] == Some(target.digit)) {
            p47_exact.insert(id);
        }
    }

    let best = *matches.iter().max().expect("rules");
    let control_best = *control_matches.iter().max().expect("rules");
    let best_ids: Vec<String> = (0..32).filter(|&rule| matches[rule] == best)
        .map(|rule| rule_id(rule / 2, rule % 2 == 1)).collect();
    let control_best_ids: Vec<String> = (0..32).filter(|&rule| control_matches[rule] == control_best)
        .map(|rule| rule_id(rule / 2, rule % 2 == 1)).collect();
    let exact_ids: Vec<String> = (0..32).filter(|&rule| matches[rule] == targets.len() && coverage[rule] == targets.len())
        .map(|rule| rule_id(rule / 2, rule % 2 == 1)).collect();
    let control_exact_ids: Vec<String> = (0..32).filter(|&rule| {
        control_matches[rule] == targets.len() && coverage[rule] == targets.len()
    }).map(|rule| rule_id(rule / 2, rule % 2 == 1)).collect();
    let survivors: Vec<&String> = p31_exact.intersection(&p47_exact).collect();

    let json = format!(
        concat!("{{\n  \"schema\":\"h116-quotient-digit-result/v1\",\n",
        "  \"source_model_records\":576,\n  \"targets\":{},\n  \"rows\":{},\n",
        "  \"rules\":32,\n  \"invariant_mismatches\":{},\n",
        "  \"matches\":{:?},\n  \"coverage\":{:?},\n",
        "  \"best_matches\":{},\n  \"best_rate\":{:.12},\n  \"best_ids\":{:?},\n",
        "  \"control_matches\":{:?},\n  \"control_best_matches\":{},\n",
        "  \"control_best_rate\":{:.12},\n  \"control_best_ids\":{:?},\n",
        "  \"exact_count\":{},\n  \"exact_ids\":{:?},\n",
        "  \"p31_exact_count\":{},\n  \"p47_exact_count\":{},\n",
        "  \"cross_curve_survivor_count\":{},\n  \"control_exact_count\":{},\n",
        "  \"status\":\"{}\"\n}}\n"),
        targets.len(), rows.len(), invariant_mismatches, matches, coverage, best,
        best as f64 / targets.len() as f64, best_ids, control_matches, control_best,
        control_best as f64 / targets.len() as f64, control_best_ids, exact_ids.len(),
        exact_ids, p31_exact.len(), p47_exact.len(), survivors.len(), control_exact_ids.len(),
        if control_exact_ids.is_empty() { if survivors.is_empty() { "no_exact_rule" } else { "candidate" } } else { "invalid_control" },
    );
    fs::write(output_path, json).map_err(|error| error.to_string())?;
    println!(
        "quotient digit scorer: targets={} best={best}/{} exact={} control_best={control_best}/{} control_exact={}",
        targets.len(), targets.len(), exact_ids.len(), targets.len(), control_exact_ids.len()
    );
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => selftest(),
        [_, flag, moments, labels, output] if flag == "--score" => score(moments, labels, output),
        _ => Err("usage: h116_quotient_digit_scorer --selftest | --score MOMENTS.tsv LABELS.tsv OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 quotient digit scorer error: {error}"); std::process::exit(1); }
}
