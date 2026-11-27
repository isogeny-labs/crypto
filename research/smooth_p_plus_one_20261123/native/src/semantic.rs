//! Bounded residue checks for positive fixed-form RURs; no root enumeration.
use serde::Serialize;
use serde_json::Value;
use std::{collections::BTreeSet, time::Instant};

const BYTES: usize = 262_144;
const VARIABLES: usize = 64;
const DEGREE: usize = 2_048;
const TERMS: usize = 20_000;
const TOTAL_DEGREE: usize = 16;
const OPERATIONS: u64 = 8_000_000;
type Poly = Vec<i64>;

#[derive(Debug, Serialize)]
pub struct Receipt {
    status: &'static str,
    equations_checked: usize,
    variables: usize,
    eliminant_degree: Option<usize>,
    scalar_operations: u64,
}

pub fn fixed_form(p: i64, j: i64) -> String {
    let mut terms = vec!["1*A".to_string(), format!("{}*t0", p - 1)];
    for (coefficient, name) in [((-j).rem_euclid(p), "a1"), ((-j * j).rem_euclid(p), "a0")] {
        if coefficient != 0 {
            terms.push(format!("{coefficient}*{name}"));
        }
    }
    terms.join("+")
}

struct Budget {
    deadline: Instant,
    operations: u64,
    limit: u64,
}
impl Budget {
    fn charge(&mut self, n: usize) -> Result<(), String> {
        if Instant::now() >= self.deadline {
            return Err("deadline exhausted".into());
        }
        self.operations = self
            .operations
            .checked_add(n as u64)
            .ok_or("operation overflow")?;
        if self.operations > self.limit {
            return Err("operation cap exhausted".into());
        }
        Ok(())
    }
}

fn trim(q: &mut Poly) {
    while q.last() == Some(&0) {
        q.pop();
    }
}
fn invert(mut x: i64, p: i64, budget: &mut Budget) -> Result<i64, String> {
    if x == 0 {
        return Err("inverse of zero".into());
    }
    let (mut n, mut y) = (p - 2, 1);
    while n > 0 {
        budget.charge(2)?;
        if n & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    Ok(y)
}

fn remainder(mut a: Poly, b: &Poly, p: i64, budget: &mut Budget) -> Result<Poly, String> {
    if b.is_empty() {
        return Err("zero modulus".into());
    }
    trim(&mut a);
    if a.len() < b.len() {
        return Ok(a);
    }
    let inverse = if b.last() == Some(&1) {
        1
    } else {
        invert(*b.last().unwrap(), p, budget)?
    };
    while !a.is_empty() && a.len() >= b.len() {
        budget.charge(b.len() + 1)?;
        let offset = a.len() - b.len();
        let coefficient = a.last().unwrap() * inverse % p;
        for (i, x) in b.iter().enumerate() {
            a[offset + i] = (a[offset + i] - coefficient * x).rem_euclid(p);
        }
        trim(&mut a);
    }
    Ok(a)
}

fn product(a: &Poly, b: &Poly, q: &Poly, p: i64, budget: &mut Budget) -> Result<Poly, String> {
    budget.charge(0)?;
    if a.is_empty() || b.is_empty() {
        return Ok(vec![]);
    }
    let mut value = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        budget.charge(b.len())?;
        for (j, y) in b.iter().enumerate() {
            value[i + j] = (value[i + j] + x * y) % p;
        }
    }
    remainder(value, q, p, budget)
}

fn coprime(mut a: Poly, mut b: Poly, p: i64, budget: &mut Budget) -> Result<bool, String> {
    while !b.is_empty() {
        budget.charge(0)?;
        let r = remainder(a, &b, p, budget)?;
        a = b;
        b = r;
    }
    Ok(a.len() == 1)
}

fn encoded(v: &Value, p: i64) -> Result<Poly, String> {
    let pair = v.as_array().ok_or("missing polynomial encoding")?;
    if pair.len() != 2 {
        return Err("invalid polynomial encoding".into());
    }
    let degree = pair[0].as_i64().ok_or("invalid polynomial degree")?;
    let coefficients = pair[1].as_array().ok_or("missing coefficients")?;
    if coefficients.len() > DEGREE + 1 {
        return Err("polynomial degree cap exhausted".into());
    }
    let mut q = coefficients
        .iter()
        .map(|x| {
            x.as_i64()
                .filter(|c| (0..p).contains(c))
                .ok_or_else(|| "noncanonical RUR coefficient".to_string())
        })
        .collect::<Result<Poly, _>>()?;
    if q == [0] && (degree == -1 || degree == 0) {
        return Ok(vec![]);
    }
    if degree < 0 || q.len() != degree as usize + 1 || q.last() == Some(&0) {
        return Err("inconsistent polynomial degree metadata".into());
    }
    trim(&mut q);
    Ok(q)
}

#[derive(Debug)]
struct Term {
    coefficient: i64,
    powers: Vec<usize>,
    degree: usize,
}

fn names(text: impl Iterator<Item = String>) -> Result<Vec<String>, String> {
    let names: Vec<_> = text.collect();
    if names.is_empty()
        || names.len() > VARIABLES
        || names.iter().any(|n| {
            n.is_empty()
                || n.len() > 64
                || !n.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
                || n.as_bytes()[0].is_ascii_digit()
        })
        || names.iter().collect::<BTreeSet<_>>().len() != names.len()
    {
        return Err("invalid or duplicate variable names".into());
    }
    Ok(names)
}

fn equation(s: &str, names: &[String], p: i64, budget: &mut Budget) -> Result<Vec<Term>, String> {
    let compact: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if compact == "0" {
        return Ok(vec![]);
    }
    let mut seen = BTreeSet::new();
    compact
        .split('+')
        .map(|s| {
            budget.charge(0)?;
            let mut factors = s.split('*');
            let coefficient = factors
                .next()
                .ok_or("empty input term")?
                .parse::<i64>()
                .map_err(|_| "invalid input coefficient")?;
            if coefficient == 0 {
                return Err("zero monomial must be omitted".into());
            }
            if !(1..p).contains(&coefficient) {
                return Err("noncanonical input coefficient".into());
            }
            let mut powers = vec![0; names.len()];
            let mut degree = 0;
            for factor in factors {
                let (name, power) = match factor.split_once('^') {
                    Some((n, e)) => (n, e.parse::<usize>().map_err(|_| "invalid exponent")?),
                    None => (factor, 1),
                };
                let i = names
                    .iter()
                    .position(|n| n == name)
                    .ok_or("unknown input variable")?;
                if power == 0 || power > TOTAL_DEGREE || powers[i] != 0 {
                    return Err("unsupported or repeated input factor".into());
                }
                degree += power;
                if degree > TOTAL_DEGREE {
                    return Err("input total-degree cap exhausted".into());
                }
                powers[i] = power;
            }
            if !seen.insert(powers.clone()) {
                return Err("duplicate input monomial".into());
            }
            Ok(Term {
                coefficient,
                powers,
                degree,
            })
        })
        .collect()
}

fn verify(input: &str, raw: &str, budget: &mut Budget) -> Result<Receipt, String> {
    budget.charge(0)?;
    if input.len() > BYTES || raw.len() > BYTES {
        return Err("input/output byte cap exhausted".into());
    }
    let mut lines = input.splitn(3, '\n');
    let input_names = names(
        lines
            .next()
            .ok_or("missing input names")?
            .split(',')
            .map(|s| s.trim().to_string()),
    )?;
    let p = lines
        .next()
        .ok_or("missing characteristic")?
        .trim()
        .parse::<i64>()
        .map_err(|_| "invalid characteristic")?;
    if p != 127 && p != 8191 {
        return Err("unsupported fixture characteristic".into());
    }
    let equations = lines
        .next()
        .ok_or("missing equations")?
        .trim()
        .split(',')
        .map(|s| equation(s, &input_names, p, budget))
        .collect::<Result<Vec<_>, _>>()?;
    if equations.len() > TERMS || equations.iter().map(Vec::len).sum::<usize>() > TERMS {
        return Err("input term cap exhausted".into());
    }
    let parsed: Value = serde_json::from_str(
        &raw.trim()
            .strip_suffix(':')
            .unwrap_or(raw.trim())
            .replace('\'', "\""),
    )
    .map_err(|_| "invalid RUR JSON")?;
    budget.charge(0)?;
    if parsed == serde_json::json!([-1]) {
        return Ok(Receipt {
            status: "solver_empty_unchecked",
            equations_checked: 0,
            variables: input_names.len(),
            eliminant_degree: None,
            scalar_operations: budget.operations,
        });
    }
    if parsed.as_array().map(Vec::len) != Some(2) || parsed[0] != 0 || parsed[1][0] != p {
        return Err("not the expected finite parametrization".into());
    }
    let block = &parsed[1];
    if block.as_array().map(Vec::len) != Some(6) {
        return Err("invalid parametrization block".into());
    }
    let declared_names = block[3]
        .as_array()
        .ok_or("missing RUR names")?
        .iter()
        .map(|n| {
            n.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "invalid RUR name".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let output_names = names(declared_names.into_iter())?;
    if block[1].as_u64() != Some(output_names.len() as u64)
        || output_names.iter().collect::<BTreeSet<_>>()
            != input_names.iter().collect::<BTreeSet<_>>()
    {
        return Err("input/RUR variable mismatch".into());
    }
    let form = block[4].as_array().ok_or("missing parameter form")?;
    if form.len() != output_names.len()
        || form
            .iter()
            .enumerate()
            .any(|(i, x)| x.as_i64() != Some(i64::from(i + 1 == form.len())))
    {
        return Err("unsupported parameter form".into());
    }
    let param = block[5].as_array().ok_or("missing parametrization")?;
    if param.len() != 2 || param[0] != 1 || param[1].as_array().map(Vec::len) != Some(3) {
        return Err("unsupported parametrization scaling".into());
    }
    let q = encoded(&param[1][0], p)?;
    if q.len() < 2 || q.last() != Some(&1) {
        return Err("eliminant must be monic and nonconstant".into());
    }
    let degree = q.len() - 1;
    let dimension = block[2].as_u64().ok_or("missing quotient dimension")?;
    if dimension != degree as u64 {
        return Err(format!("nonseparating RUR: eliminant degree {degree} differs from quotient dimension {dimension}"));
    }
    let mut derivative: Poly = q
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| (i as i64 * c) % p)
        .collect();
    budget.charge(derivative.len())?;
    trim(&mut derivative);
    if !coprime(q.clone(), derivative, p, budget)? {
        return Err("non-square-free eliminant".into());
    }
    let denominator = remainder(encoded(&param[1][1], p)?, &q, p, budget)?;
    if !coprime(q.clone(), denominator.clone(), p, budget)? {
        return Err("nonunit denominator".into());
    }
    let numerators = param[1][2]
        .as_array()
        .ok_or("missing coordinate numerators")?;
    if numerators.len() + 1 != output_names.len() {
        return Err("coordinate count mismatch".into());
    }
    let mut coordinates = vec![];
    for numerator in numerators {
        if numerator.as_array().map(Vec::len) != Some(1) {
            return Err("unsupported coordinate scaling".into());
        }
        let mut poly = encoded(&numerator[0], p)?;
        if poly.len() > degree {
            return Err("coordinate degree exceeds eliminant".into());
        }
        for x in &mut poly {
            *x = (p - *x) % p;
        }
        coordinates.push(poly);
    }
    coordinates.push(product(&vec![0, 1], &denominator, &q, p, budget)?);
    let coordinates: Vec<_> = input_names
        .iter()
        .map(|name| coordinates[output_names.iter().position(|n| n == name).unwrap()].clone())
        .collect();
    let max_degree = equations
        .iter()
        .flatten()
        .map(|t| t.degree)
        .max()
        .unwrap_or(0);
    let mut denominator_powers = vec![vec![1]];
    for i in 1..=max_degree {
        denominator_powers.push(product(
            &denominator_powers[i - 1],
            &denominator,
            &q,
            p,
            budget,
        )?);
    }
    let mut coordinate_powers = vec![];
    for (i, coordinate) in coordinates.iter().enumerate() {
        let exponent = equations
            .iter()
            .flatten()
            .map(|t| t.powers[i])
            .max()
            .unwrap_or(0);
        let mut powers = vec![vec![1]];
        for j in 1..=exponent {
            powers.push(product(&powers[j - 1], coordinate, &q, p, budget)?);
        }
        coordinate_powers.push(powers);
    }
    for (index, equation) in equations.iter().enumerate() {
        budget.charge(0)?;
        let degree = equation.iter().map(|t| t.degree).max().unwrap_or(0);
        let mut sum = vec![0; q.len() - 1];
        for term in equation {
            let mut poly = vec![term.coefficient];
            for (i, exponent) in term.powers.iter().enumerate().filter(|(_, e)| **e != 0) {
                poly = product(&poly, &coordinate_powers[i][*exponent], &q, p, budget)?;
            }
            poly = product(
                &poly,
                &denominator_powers[degree - term.degree],
                &q,
                p,
                budget,
            )?;
            budget.charge(poly.len())?;
            for (i, x) in poly.iter().enumerate() {
                sum[i] = (sum[i] + x) % p;
            }
        }
        if sum.iter().any(|x| *x != 0) {
            return Err(format!("equation {} has nonzero remainder", index + 1));
        }
    }
    budget.charge(0)?;
    Ok(Receipt {
        status: "positive_parametrization_checked",
        equations_checked: equations.len(),
        variables: input_names.len(),
        eliminant_degree: Some(q.len() - 1),
        scalar_operations: budget.operations,
    })
}

pub fn check(input: &str, raw: &str, deadline: Instant) -> Result<Receipt, String> {
    check_with_limit(input, raw, deadline, OPERATIONS)
}

fn check_with_limit(
    input: &str,
    raw: &str,
    deadline: Instant,
    limit: u64,
) -> Result<Receipt, String> {
    verify(
        input,
        raw,
        &mut Budget {
            deadline,
            operations: 0,
            limit,
        },
    )
    .map_err(|e| {
        if e.starts_with("nonseparating RUR:") {
            e
        } else {
            format!("semantic RUR: {e}")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::time::Duration;

    const INPUT: &str = "x,y,A\n127\n1*x^2+124*x+2,1*y+124*x,1*A+126*x\n";
    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(3)
    }
    fn example() -> Value {
        json!([
            0,
            [
                127,
                3,
                2,
                ["x", "y", "A"],
                [0, 0, 1],
                [
                    1,
                    [
                        [2, [2, 124, 1]],
                        [0, [1]],
                        [[[1, [0, 126]]], [[1, [0, 124]]]]
                    ]
                ]
            ]
        ])
    }
    fn checked(input: &str, raw: &Value) -> Result<Receipt, String> {
        check(input, &raw.to_string(), deadline())
    }
    #[test]
    fn all_fixed_forms_are_zero_free_and_keep_the_mathematical_family() {
        for p in [127, 8191] {
            assert_eq!(fixed_form(p, 0), format!("1*A+{}*t0", p - 1));
            let ns = ["t0", "a1", "a0", "A"].map(str::to_owned).to_vec();
            for j in 0..16 {
                let form = fixed_form(p, j);
                assert!(!form.contains("+0*"));
                let terms = equation(
                    &form,
                    &ns,
                    p,
                    &mut Budget {
                        deadline: deadline(),
                        operations: 0,
                        limit: OPERATIONS,
                    },
                )
                .unwrap();
                let point = [7, 11, 19, 23];
                let value: i64 = terms
                    .iter()
                    .map(|t| t.coefficient * point[t.powers.iter().position(|e| *e == 1).unwrap()])
                    .sum();
                assert_eq!(
                    value.rem_euclid(p),
                    (23 - 7 - j * 11 - j * j * 19).rem_euclid(p)
                );
            }
        }
    }
    #[test]
    fn positive_residues_cover_all_equations_and_nonconstant_unit_denominator() {
        let receipt = checked(INPUT, &example()).unwrap();
        assert_eq!(receipt.equations_checked, 3);
        assert_eq!(receipt.eliminant_degree, Some(2));
        assert!(receipt.scalar_operations > 0);
        let mut rational = example();
        rational[1][5][1][1] = json!([1, [0, 1]]);
        rational[1][5][1][2] = json!([[[1, [2, 124]]], [[1, [6, 118]]]]);
        assert!(checked(INPUT, &rational).is_ok());
    }
    #[test]
    fn mutated_auxiliary_coordinate_and_fixed_form_are_rejected() {
        let mut raw = example();
        raw[1][5][1][2][1][0][1][0] = json!(1);
        assert_eq!(
            checked(INPUT, &raw).unwrap_err(),
            "semantic RUR: equation 2 has nonzero remainder"
        );
        assert_eq!(
            checked(&INPUT.replace("1*A+126*x", "1*A+126*x+1"), &example()).unwrap_err(),
            "semantic RUR: equation 3 has nonzero remainder"
        );
    }
    #[test]
    fn nonunit_denominator_and_repeated_eliminant_are_rejected() {
        let mut raw = example();
        raw[1][5][1][1] = json!([1, [126, 1]]);
        assert_eq!(
            checked(INPUT, &raw).unwrap_err(),
            "semantic RUR: nonunit denominator"
        );
        raw = example();
        raw[1][5][1][0] = json!([2, [1, 125, 1]]);
        assert_eq!(
            checked(INPUT, &raw).unwrap_err(),
            "semantic RUR: non-square-free eliminant"
        );
    }
    #[test]
    fn dimension_failure_keeps_legacy_retry_prefix() {
        let mut raw = example();
        raw[1][2] = json!(3);
        assert_eq!(
            checked(INPUT, &raw).unwrap_err(),
            "nonseparating RUR: eliminant degree 2 differs from quotient dimension 3"
        );
    }
    #[test]
    fn malformed_shapes_and_scaling_fail_before_acceptance() {
        for raw in [
            {
                let mut r = example();
                r[1][5][1][0][0] = json!(3);
                r
            },
            {
                let mut r = example();
                r[1][5][0] = json!(2);
                r
            },
            {
                let mut r = example();
                r[1][3] = json!(["x", "x", "A"]);
                r
            },
            {
                let mut r = example();
                r[1][4] = json!([1, 0, 0]);
                r
            },
            {
                let mut r = example();
                r[1][5][1][2] = json!([]);
                r
            },
            {
                let mut r = example();
                r[1][5][1][2][0][0][1][0] = json!(-1);
                r
            },
        ] {
            assert!(checked(INPUT, &raw)
                .unwrap_err()
                .starts_with("semantic RUR:"));
        }
    }
    #[test]
    fn names_are_remapped_without_discarding_auxiliary_coordinates() {
        let mut raw = example();
        raw[1][3] = json!(["y", "x", "A"]);
        raw[1][5][1][2] = json!([[[1, [0, 124]]], [[1, [0, 126]]]]);
        assert!(checked(INPUT, &raw).is_ok());
    }
    #[test]
    fn noncanonical_or_ambiguous_input_is_rejected() {
        for equation in ["1*x+1*x", "1*z", "1*x*x", "1*A+0*x", "1*x^17", "127*x"] {
            assert!(checked(&format!("x,y,A\n127\n{equation}"), &example()).is_err());
        }
    }
    #[test]
    fn deadlines_operation_caps_and_byte_caps_fail_closed() {
        assert_eq!(
            check(INPUT, &example().to_string(), Instant::now()).unwrap_err(),
            "semantic RUR: deadline exhausted"
        );
        assert_eq!(
            check_with_limit(INPUT, &example().to_string(), deadline(), 1).unwrap_err(),
            "semantic RUR: operation cap exhausted"
        );
        assert_eq!(
            check(&"x".repeat(BYTES + 1), "[-1]", deadline()).unwrap_err(),
            "semantic RUR: input/output byte cap exhausted"
        );
    }
    #[test]
    fn empty_marker_keeps_explicit_unchecked_status() {
        let r = check(INPUT, "[-1]:\n", deadline()).unwrap();
        assert_eq!(r.status, "solver_empty_unchecked");
        assert_eq!(r.equations_checked, 0);
        assert_eq!(r.eliminant_degree, None);
        assert!(check(INPUT, "[-1]", Instant::now()).is_err());
    }
    #[test]
    fn both_saved_rurs_exercise_the_actual_form_and_equation_failures() {
        let select = |text: &str| -> Value {
            text.lines()
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .find(|r| r["mode"] == "ProductionCofactorMsolve" && r["target"] == json!([41, 95]))
                .unwrap()
        };
        let old = select(include_str!(
            "../../evidence/factorwise_extract_v1/f127_d4.jsonl"
        ));
        let new = select(include_str!(
            "../../evidence/liftable_fibers_v1/f127_d4.jsonl"
        ));
        let prefix = old["systems"][0]["input_prefix"].as_str().unwrap();
        let old_raw = old["systems"][0]["attempts"][0]["rur"].as_str().unwrap();
        let new_raw = new["systems"][0]["attempts"][0]["rur"].as_str().unwrap();
        let effective = format!("{prefix},\n1*A+126*t0+1*a1+1*a0\n");
        assert_eq!(
            check(&effective, old_raw, deadline())
                .unwrap()
                .equations_checked,
            9
        );
        let corrected = format!("{prefix},\n{}\n", fixed_form(127, 0));
        assert_eq!(
            check(&corrected, old_raw, deadline()).unwrap_err(),
            "semantic RUR: equation 9 has nonzero remainder"
        );
        assert_eq!(
            check(&effective, new_raw, deadline()).unwrap_err(),
            "semantic RUR: equation 1 has nonzero remainder"
        );
    }
}
