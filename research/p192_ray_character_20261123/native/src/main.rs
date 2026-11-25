//! Exact square-class grid coverage and first-hit point-work bounds.
//!
//! For 2^c = +/-5 (mod r), the M by L grid covers exponent intervals
//! [cj, cj+M-1] (mod N), 0 <= j < L, N=(r-1)/4. The first-return form of
//! the three-gap theorem gives the gap multiset in polylogarithmic time.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};

const R_DEC: &str = "6277101735386680763835789423176059013767194773182842284081";
const SMALL_LOG_MODULUS: u64 = 4 * 5 * 2389;

fn bu(value: u64) -> BigUint {
    BigUint::from(value)
}

fn parse_big(value: &str) -> Result<BigUint, String> {
    BigUint::parse_bytes(value.as_bytes(), 10)
        .ok_or_else(|| format!("invalid nonnegative decimal integer: {value}"))
}

fn p192_r() -> BigUint {
    parse_big(R_DEC).expect("fixed decimal order")
}

fn floor_sum(
    mut length: BigUint,
    mut modulus: BigUint,
    mut slope: BigUint,
    mut intercept: BigUint,
) -> BigUint {
    let mut total = BigUint::zero();
    loop {
        if length.is_zero() {
            return total;
        }
        let (q, remainder) = slope.div_rem(&modulus);
        total += (&length * (&length - BigUint::one()) / bu(2)) * q;
        slope = remainder;
        let (q, remainder) = intercept.div_rem(&modulus);
        total += &length * q;
        intercept = remainder;
        let last = &slope * &length + &intercept;
        if last < modulus {
            return total;
        }
        let (q, remainder) = last.div_rem(&modulus);
        length = q;
        intercept = remainder;
        std::mem::swap(&mut modulus, &mut slope);
    }
}

fn count_below(
    count: &BigUint,
    modulus: &BigUint,
    step: &BigUint,
    threshold: &BigUint,
) -> Result<BigUint, String> {
    if threshold > modulus {
        return Err("threshold outside modular interval".into());
    }
    let high = floor_sum(
        count.clone(),
        modulus.clone(),
        step.clone(),
        modulus - threshold,
    );
    let base = floor_sum(
        count.clone(),
        modulus.clone(),
        step.clone(),
        BigUint::zero(),
    );
    Ok(count - (high - base))
}

fn first_threshold(
    count: &BigUint,
    modulus: &BigUint,
    step: &BigUint,
    wanted: &BigUint,
) -> Result<BigUint, String> {
    let mut low = BigUint::zero();
    let mut high = modulus + BigUint::one();
    while &low + BigUint::one() < high {
        let mid = (&low + &high) / bu(2);
        if count_below(count, modulus, step, &mid)? >= *wanted {
            high = mid;
        } else {
            low = mid;
        }
    }
    Ok(high)
}

fn mod_inverse(value: &BigUint, modulus: &BigUint) -> Result<BigUint, String> {
    let a = BigInt::from_biguint(Sign::Plus, value.clone());
    let n = BigInt::from_biguint(Sign::Plus, modulus.clone());
    let bezout = a.extended_gcd(&n);
    if bezout.gcd != BigInt::one() {
        return Err("rotation step has no inverse after gcd reduction".into());
    }
    bezout
        .x
        .mod_floor(&n)
        .to_biguint()
        .ok_or_else(|| "nonnegative modular inverse conversion failed".into())
}

#[derive(Clone, Debug)]
struct GapReport {
    modulus: BigUint,
    step: BigUint,
    query_points: BigUint,
    rotation_period: BigUint,
    lower_point: BigUint,
    lower_index: BigUint,
    upper_point: BigUint,
    upper_index: BigUint,
    gaps: BTreeMap<BigUint, BigUint>,
    largest_gap: BigUint,
}

fn gap_report(modulus: &BigUint, step: &BigUint, count: &BigUint) -> Result<GapReport, String> {
    if modulus.is_zero() {
        return Err("modulus must be positive".into());
    }
    let step = step % modulus;
    let gcd = step.gcd(modulus);
    let period = modulus / &gcd;
    if count < &bu(2) || count > &period {
        return Err("query count outside distinct rotation orbit".into());
    }
    let below = first_threshold(count, modulus, &step, &bu(2))? - BigUint::one();
    let above = first_threshold(count, modulus, &step, count)? - BigUint::one();
    let inverse = mod_inverse(&(&step / &gcd), &period)?;
    let lower_index = (&below / &gcd * &inverse) % &period;
    let upper_index = (&above / &gcd * &inverse) % &period;
    if lower_index.is_zero()
        || upper_index.is_zero()
        || &lower_index >= count
        || &upper_index >= count
    {
        return Err("extremal index is outside the orbit".into());
    }
    let left = below.clone();
    let right = modulus - &above;
    if &lower_index + &upper_index < *count {
        return Err("first-return indices violate gap count".into());
    }
    let long_count = &lower_index + &upper_index - count;
    let mut gaps = BTreeMap::new();
    *gaps.entry(left.clone()).or_insert_with(BigUint::zero) += count - &lower_index;
    *gaps.entry(right.clone()).or_insert_with(BigUint::zero) += count - &upper_index;
    if !long_count.is_zero() {
        *gaps.entry(&left + &right).or_insert_with(BigUint::zero) += &long_count;
    }
    let gap_count: BigUint = gaps.values().cloned().sum();
    let gap_sum: BigUint = gaps
        .iter()
        .map(|(length, multiplicity)| length * multiplicity)
        .sum();
    if gap_count != *count || gap_sum != *modulus {
        return Err("gap multiset does not partition the circle".into());
    }
    let largest_gap = if long_count.is_zero() {
        max(left, right)
    } else {
        left + right
    };
    Ok(GapReport {
        modulus: modulus.clone(),
        step,
        query_points: count.clone(),
        rotation_period: period,
        lower_point: below,
        lower_index,
        upper_point: above,
        upper_index,
        gaps,
        largest_gap,
    })
}

fn covered_labels(report: &GapReport, table_points: &BigUint) -> Result<BigUint, String> {
    if table_points.is_zero() {
        return Err("table length must be positive".into());
    }
    Ok(report
        .gaps
        .iter()
        .map(|(gap, multiplicity)| min(gap, table_points) * multiplicity)
        .sum())
}

fn p192_small_log() -> Result<BigUint, String> {
    let r = p192_r();
    let n = (&r - BigUint::one()) / bu(4);
    let modulus = bu(SMALL_LOG_MODULUS);
    if &n % &modulus != BigUint::zero() {
        return Err("small log modulus does not divide square-class order".into());
    }
    let exponent = &n / &modulus;
    let base = bu(2).modpow(&exponent, &r);
    let target = bu(5).modpow(&exponent, &r);
    let mut current = BigUint::one();
    let mut solution = None;
    for k in 0..SMALL_LOG_MODULUS {
        if (current == target || current == &r - &target) && solution.replace(bu(k)).is_some() {
            return Err("small quotient log is not unique".into());
        }
        current = current * &base % &r;
    }
    solution.ok_or_else(|| "small quotient log not found".into())
}

fn p192_order_checks() -> Result<(), String> {
    let r = p192_r();
    let n = (&r - BigUint::one()) / bu(4);
    let large_a = parse_big("9564682313913860059195669")?;
    let large_b = parse_big("3433859179316188682119986911")?;
    let factors = [bu(2), bu(5), bu(2389), large_a, large_b];
    let product = bu(4) * bu(5) * bu(2389) * &factors[3] * &factors[4];
    if product != n {
        return Err("square-class order factorization mismatch".into());
    }
    let sign_is_one = |value: &BigUint| *value == BigUint::one() || *value == &r - BigUint::one();
    if bu(2).modpow(&n, &r) != &r - BigUint::one() {
        return Err("2 does not satisfy expected square-class exponent".into());
    }
    for factor in &factors {
        if sign_is_one(&bu(2).modpow(&(&n / factor), &r)) {
            return Err(format!(
                "2 has a smaller order modulo sign at factor {factor}"
            ));
        }
    }
    let half_n = &n / bu(2);
    if bu(5).modpow(&half_n, &r) != &r - BigUint::one() {
        return Err("5 does not satisfy expected half-order exponent".into());
    }
    for factor in &factors {
        if sign_is_one(&bu(5).modpow(&(&half_n / factor), &r)) {
            return Err(format!(
                "5 has a smaller order modulo sign at factor {factor}"
            ));
        }
    }
    Ok(())
}

fn to_f64(value: &BigUint) -> Result<f64, String> {
    value
        .to_f64()
        .ok_or_else(|| "large integer could not be converted to f64".into())
}

fn rho_reference() -> Result<f64, String> {
    Ok((std::f64::consts::PI * to_f64(&p192_r())? / 4.0).sqrt())
}

fn optimistic_point_cost(report: &GapReport, table_points: &BigUint) -> Result<(f64, f64), String> {
    if covered_labels(report, table_points)? != report.modulus {
        return Err("optimistic cost requires a full-cover grid".into());
    }
    let (full, remainder) = report.modulus.div_rem(table_points);
    let first_hit_sum = if full.is_zero() {
        BigUint::zero()
    } else {
        table_points * &full * (&full - BigUint::one()) / bu(2) + remainder * &full
    };
    let mean = to_f64(&first_hit_sum)? / to_f64(&report.modulus)?;
    let table_cost = 0.5 * to_f64(&(table_points - BigUint::one()))?;
    let query_cost = 0.75 * (to_f64(&(&report.query_points - BigUint::one()))? + mean);
    let cost = table_cost + query_cost;
    Ok((cost, cost / rho_reference()?))
}

#[derive(Clone)]
struct Segment {
    a: BigUint,
    b: BigUint,
    low: BigUint,
    high: BigUint,
}

impl Segment {
    fn width(&self) -> BigUint {
        &self.high - &self.low
    }
}

fn uncovered_at(
    modulus: &BigUint,
    step: &BigUint,
    table_points: &BigUint,
    count: &BigUint,
    cache: &mut HashMap<BigUint, BigUint>,
) -> Result<BigUint, String> {
    if let Some(covered) = cache.get(count) {
        return Ok(modulus - covered);
    }
    let report = gap_report(modulus, step, count)?;
    let covered = covered_labels(&report, table_points)?;
    cache.insert(count.clone(), covered.clone());
    Ok(modulus - covered)
}

fn make_segment(
    modulus: &BigUint,
    step: &BigUint,
    table_points: &BigUint,
    a: BigUint,
    b: BigUint,
    cache: &mut HashMap<BigUint, BigUint>,
) -> Result<Segment, String> {
    let length = &b - &a;
    let last = &b - BigUint::one();
    let low = &length * uncovered_at(modulus, step, table_points, &last, cache)?;
    let high = &length * uncovered_at(modulus, step, table_points, &a, cache)?;
    Ok(Segment { a, b, low, high })
}

struct FirstHitBounds {
    sum_low: BigUint,
    sum_high: BigUint,
    coverage_evaluations: usize,
    splits: usize,
    cost_low: f64,
    cost_high: f64,
}

fn first_hit_bounds(
    report: &GapReport,
    table_points: &BigUint,
    max_splits: usize,
) -> Result<FirstHitBounds, String> {
    if covered_labels(report, table_points)? != report.modulus {
        return Err("first-hit cost requires a full-cover grid".into());
    }
    let mut cache = HashMap::new();
    cache.insert(BigUint::one(), min(table_points, &report.modulus).clone());
    let initial = make_segment(
        &report.modulus,
        &report.step,
        table_points,
        BigUint::one(),
        report.query_points.clone(),
        &mut cache,
    )?;
    let mut total_low = initial.low.clone();
    let mut total_high = initial.high.clone();
    let mut segments = vec![initial];
    let mut heap = BinaryHeap::new();
    heap.push((segments[0].width(), 0_usize));
    let mut splits = 0_usize;
    while splits < max_splits {
        let Some((width, id)) = heap.pop() else { break };
        if width.is_zero() {
            break;
        }
        let old = segments[id].clone();
        let mid = (&old.a + &old.b) / bu(2);
        let left = make_segment(
            &report.modulus,
            &report.step,
            table_points,
            old.a,
            mid.clone(),
            &mut cache,
        )?;
        let right = make_segment(
            &report.modulus,
            &report.step,
            table_points,
            mid,
            old.b,
            &mut cache,
        )?;
        total_low = total_low - old.low + &left.low + &right.low;
        total_high = total_high - old.high + &left.high + &right.high;
        let left_id = segments.len();
        heap.push((left.width(), left_id));
        segments.push(left);
        let right_id = segments.len();
        heap.push((right.width(), right_id));
        segments.push(right);
        splits += 1;
    }
    let fixed = 0.5 * to_f64(&(table_points - BigUint::one()))?
        + 0.75 * to_f64(&(&report.query_points - BigUint::one()))?;
    let denominator = 2.0 * to_f64(&report.modulus)?;
    let cost_low = fixed + 1.5 * to_f64(&total_low)? / denominator;
    let cost_high = fixed + 1.5 * to_f64(&total_high)? / denominator;
    Ok(FirstHitBounds {
        sum_low: total_low,
        sum_high: total_high,
        coverage_evaluations: cache.len() + 1,
        splits,
        cost_low,
        cost_high,
    })
}

fn run_controls() -> Result<(usize, usize), String> {
    let mut cases = 0_usize;
    for n in 5_u64..100 {
        for step in 1..n {
            let period = n / step.gcd(&n);
            if period < 2 {
                continue;
            }
            let mut counts = BTreeSet::new();
            counts.insert(2);
            counts.insert(period);
            counts.insert(2 + (17 * n + 13 * step) % (period - 1));
            for count in counts {
                let mut residues: Vec<u64> = (0..count).map(|j| j * step % n).collect();
                residues.sort_unstable();
                let mut gaps: Vec<u64> =
                    residues.windows(2).map(|pair| pair[1] - pair[0]).collect();
                gaps.push(n - residues[residues.len() - 1]);
                let mut counts = BTreeMap::<u64, u64>::new();
                for gap in &gaps {
                    *counts.entry(*gap).or_default() += 1;
                }
                let report = gap_report(&bu(n), &bu(step), &bu(count))?;
                let found: BTreeMap<u64, u64> = report
                    .gaps
                    .iter()
                    .map(|(gap, multiplicity)| {
                        (gap.to_u64().unwrap(), multiplicity.to_u64().unwrap())
                    })
                    .collect();
                if report.largest_gap != bu(*gaps.iter().max().unwrap())
                    || found != counts
                    || report.lower_point != bu(residues[1])
                    || report.upper_point != bu(residues[residues.len() - 1])
                {
                    return Err(format!(
                        "gap control failed at N={n}, step={step}, L={count}"
                    ));
                }
                let mut table_sizes = BTreeSet::new();
                table_sizes.insert(1);
                table_sizes.insert(max(1, gaps.iter().max().unwrap() / 2));
                table_sizes.insert(*gaps.iter().max().unwrap());
                for table in table_sizes {
                    let direct: BTreeSet<u64> = (0..count)
                        .flat_map(|j| (0..table).map(move |i| (j * step + i) % n))
                        .collect();
                    if covered_labels(&report, &bu(table))? != bu(direct.len() as u64) {
                        return Err(format!("coverage control failed at N={n}, step={step}"));
                    }
                }
                for threshold in [0, 1, n / 2, n] {
                    let direct = residues.iter().filter(|value| **value < threshold).count();
                    if count_below(&bu(count), &bu(n), &bu(step), &bu(threshold))?
                        != bu(direct as u64)
                    {
                        return Err(format!("floor-sum control failed at N={n}, step={step}"));
                    }
                }
                cases += 1;
            }
        }
    }
    if p192_small_log()? != bu(32974) {
        return Err("P-192 small-factor log control failed".into());
    }
    p192_order_checks()?;
    let mut first_hit_cases = 0_usize;
    let mut state = 27467_u64;
    for _ in 0..500 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let n = 5 + state % 145;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let step = 1 + state % (n - 1);
        let period = n / step.gcd(&n);
        if period < 2 {
            continue;
        }
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let count = 2 + state % (period - 1);
        let report = gap_report(&bu(n), &bu(step), &bu(count))?;
        let table = report.largest_gap.to_u64().unwrap();
        let mut first_hit = vec![None; n as usize];
        for j in 0..count {
            for i in 0..table {
                first_hit[((j * step + i) % n) as usize].get_or_insert(j);
            }
        }
        if first_hit.iter().any(Option::is_none) {
            return Err("first-hit control lacks full coverage".into());
        }
        let direct_sum: u64 = first_hit.iter().map(|value| value.unwrap()).sum();
        let bounds = first_hit_bounds(&report, &bu(table), count as usize)?;
        if bounds.sum_low != bu(direct_sum) || bounds.sum_high != bu(direct_sum) {
            return Err(format!(
                "first-hit control failed at N={n}, step={step}, L={count}"
            ));
        }
        first_hit_cases += 1;
    }
    Ok((cases, first_hit_cases))
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut self_test = false;
    let mut exponent = None;
    let mut query_points = None;
    let mut table_points = None;
    let mut splits = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--self-test" => self_test = true,
            "--p192-c" => {
                exponent = Some(parse_big(&args.next().ok_or("missing --p192-c value")?)?)
            }
            "--count" => {
                query_points = Some(parse_big(&args.next().ok_or("missing --count value")?)?)
            }
            "--table" => {
                table_points = Some(parse_big(&args.next().ok_or("missing --table value")?)?)
            }
            "--first-hit-splits" => {
                splits = Some(
                    args.next()
                        .ok_or("missing --first-hit-splits value")?
                        .parse::<usize>()
                        .map_err(|_| "invalid --first-hit-splits value")?,
                )
            }
            _ => return Err(format!("unknown flag: {flag}")),
        }
    }
    if self_test {
        if exponent.is_some()
            || query_points.is_some()
            || table_points.is_some()
            || splits.is_some()
        {
            return Err("--self-test cannot be combined with grid arguments".into());
        }
        let (gap_cases, first_hit_cases) = run_controls()?;
        println!(
            "{}",
            json!({
                "status":"pass", "gap_coverage_cases":gap_cases,
                "first_hit_exact_cases":first_hit_cases,
            "p192_log_mod_47780":"32974",
            "p192_order_checks":"pass"
            })
        );
        return Ok(());
    }
    let c = exponent.ok_or("supply --p192-c and --count, or --self-test")?;
    let count = query_points.ok_or("supply --count with --p192-c")?;
    let r = p192_r();
    let n = (&r - BigUint::one()) / bu(4);
    let power = bu(2).modpow(&c, &r);
    if power != bu(5) && power != &r - bu(5) {
        return Err("supplied exponent does not satisfy 2^c = +/-5 (mod r)".into());
    }
    if &c % bu(SMALL_LOG_MODULUS) != p192_small_log()? {
        return Err("supplied exponent fails small-factor log check".into());
    }
    let report = gap_report(&n, &c, &count)?;
    let minimum_table = report.largest_gap.clone();
    let table = table_points.unwrap_or_else(|| minimum_table.clone());
    let coverage = covered_labels(&report, &table)?;
    let optimal_coverage = covered_labels(&report, &minimum_table)?;
    let gap_rows: Vec<_> = report
        .gaps
        .iter()
        .map(|(gap, multiplicity)| json!({"gap":gap.to_string(), "count":multiplicity.to_string()}))
        .collect();
    let mut result = json!({
        "modulus":n.to_string(), "step":report.step.to_string(),
        "query_points":report.query_points.to_string(),
        "rotation_period":report.rotation_period.to_string(),
        "lower_point":report.lower_point.to_string(),
        "lower_index":report.lower_index.to_string(),
        "upper_point":report.upper_point.to_string(),
        "upper_index":report.upper_index.to_string(),
        "gap_multiplicities":gap_rows,
        "minimum_full_cover_table_points":minimum_table.to_string(),
        "minimum_table_covered_labels":optimal_coverage.to_string(),
        "proposed_table_points":table.to_string(),
        "proposed_covered_labels":coverage.to_string(),
        "rho_reference_group_additions":rho_reference()?,
        "assumed_table_step_group_additions":0.5,
        "assumed_query_step_group_additions":1.5,
        "p192_log_mod_47780":"32974"
    });
    if coverage == n {
        let (optimistic, optimistic_ratio) = optimistic_point_cost(&report, &table)?;
        result["optimistic_expected_point_cost"] = json!(optimistic);
        result["optimistic_expected_point_cost_over_rho"] = json!(optimistic_ratio);
        if let Some(budget) = splits {
            let bounds = first_hit_bounds(&report, &table, budget)?;
            result["first_hit_splits"] = json!(bounds.splits);
            result["coverage_evaluations"] = json!(bounds.coverage_evaluations);
            result["positive_first_hit_sum_lower"] = json!(bounds.sum_low.to_string());
            result["positive_first_hit_sum_upper"] = json!(bounds.sum_high.to_string());
            result["positive_first_hit_sum_denominator"] = json!(n.to_string());
            result["expected_point_cost_lower"] = json!(bounds.cost_low);
            result["expected_point_cost_upper"] = json!(bounds.cost_high);
            result["expected_point_cost_over_rho_lower"] =
                json!(bounds.cost_low / rho_reference()?);
            result["expected_point_cost_over_rho_upper"] =
                json!(bounds.cost_high / rho_reference()?);
        }
    } else if splits.is_some() {
        return Err("a non-covering grid cannot decide the negative sign".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("p192-ray-grid-certificate: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_enumeration_controls() {
        let (gap_cases, first_hit_cases) = run_controls().unwrap();
        assert!(gap_cases > 10_000);
        assert_eq!(first_hit_cases, 500);
    }

    #[test]
    fn rejects_noncovering_grid_for_sign_cost() {
        let report = gap_report(&bu(60), &bu(11), &bu(5)).unwrap();
        let too_short = &report.largest_gap - BigUint::one();
        assert!(covered_labels(&report, &too_short).unwrap() < bu(60));
        assert!(first_hit_bounds(&report, &too_short, 8).is_err());
    }

    #[test]
    fn table_longer_than_label_circle_has_zero_first_hit_index() {
        let report = gap_report(&bu(60), &bu(11), &bu(5)).unwrap();
        let table = bu(61);
        assert_eq!(covered_labels(&report, &table).unwrap(), bu(60));
        assert!(optimistic_point_cost(&report, &table)
            .unwrap()
            .0
            .is_finite());
        let bounds = first_hit_bounds(&report, &table, 8).unwrap();
        assert_eq!(bounds.sum_low, BigUint::zero());
        assert_eq!(bounds.sum_high, BigUint::zero());
    }
}
