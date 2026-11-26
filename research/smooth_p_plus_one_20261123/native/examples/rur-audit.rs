//! Offline residual checks of two retained F127 RUR records; never calls a solver.
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::{env, fs};

type Equation = Vec<(i64, Vec<u32>)>;

fn power(mut x: i64, mut n: u32, p: i64) -> i64 {
    let mut y = 1;
    while n != 0 {
        if n & 1 != 0 {
            y = y * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    y
}

fn univariate(q: &[i64], x: i64, p: i64) -> i64 {
    q.iter().rev().fold(0, |y, c| (y * x + c).rem_euclid(p))
}

fn equation(text: &str, names: &[String], p: i64) -> Result<Equation, String> {
    let compact = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    if compact.is_empty() {
        return Err("empty serialized equation".into());
    }
    let mut seen = BTreeSet::new();
    compact
        .split('+')
        .map(|term| {
            let mut parts = term.split('*');
            let coefficient = parts
                .next()
                .ok_or("missing coefficient")?
                .parse::<i64>()
                .map_err(|_| "unsupported coefficient")?;
            if !(0..p).contains(&coefficient) {
                return Err("noncanonical coefficient".into());
            }
            let mut powers = vec![0u32; names.len()];
            for part in parts {
                let (name, exponent) = match part.split_once('^') {
                    Some((name, n)) => (name, n.parse::<u32>().map_err(|_| "invalid exponent")?),
                    None => (part, 1),
                };
                if !(1..=16).contains(&exponent) {
                    return Err("exponent outside audit bounds".into());
                }
                let index = names
                    .iter()
                    .position(|x| x == name)
                    .ok_or("unknown input variable")?;
                if powers[index] != 0 {
                    return Err("repeated variable within monomial".into());
                }
                powers[index] = exponent;
            }
            if !seen.insert(powers.clone()) {
                return Err("repeated input monomial".into());
            }
            Ok((coefficient, powers))
        })
        .collect()
}

fn residual(eq: &Equation, point: &[i64], p: i64) -> i64 {
    eq.iter().fold(0, |sum, (coefficient, exponents)| {
        let term = exponents
            .iter()
            .zip(point)
            .fold(*coefficient, |v, (e, x)| v * power(*x, *e, p) % p);
        (sum + term) % p
    })
}

fn polynomial(encoded: &Value, p: i64) -> Result<Vec<i64>, String> {
    let degree = encoded[0].as_i64().ok_or("missing polynomial degree")?;
    let q = encoded[1]
        .as_array()
        .ok_or("missing polynomial coefficients")?
        .iter()
        .map(|v| {
            v.as_i64()
                .ok_or("invalid polynomial coefficient".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if degree == -1 && q == [0] {
        return Ok(q);
    }
    if degree < 0
        || degree > 256
        || q.len() != degree as usize + 1
        || q.last().is_none_or(|x| x.rem_euclid(p) == 0)
    {
        return Err("inconsistent polynomial degree metadata".into());
    }
    Ok(q)
}

fn diagnose(prefix: &str, linear: &str, raw: &str) -> Result<Value, String> {
    let mut lines = prefix.splitn(3, '\n');
    let names = lines
        .next()
        .ok_or("missing input names")?
        .split(',')
        .map(|x| x.trim().to_string())
        .collect::<Vec<_>>();
    if names.len() < 2
        || names.iter().any(|n| n.is_empty())
        || names.iter().collect::<BTreeSet<_>>().len() != names.len()
    {
        return Err("duplicate or empty input names".into());
    }
    let p = lines
        .next()
        .ok_or("missing characteristic")?
        .parse::<i64>()
        .map_err(|_| "invalid characteristic")?;
    if p != 127 {
        return Err("retained-record audit is limited to F127".into());
    }
    let equations = lines
        .next()
        .ok_or("missing equations")?
        .split(',')
        .map(|s| equation(s, &names, p))
        .collect::<Result<Vec<_>, _>>()?;
    let form = equation(linear, &names, p)?;
    let rur: Value = serde_json::from_str(&raw.trim().trim_end_matches(':').replace('\'', "\""))
        .map_err(|e| e.to_string())?;
    if rur[0].as_i64() != Some(0) || rur[1][0].as_i64() != Some(p) {
        return Err("not the expected finite F127 parametrization".into());
    }
    let block = &rur[1];
    let variables = block[3]
        .as_array()
        .ok_or("missing RUR names")?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .ok_or("invalid RUR name".to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    if block[1].as_u64() != Some(variables.len() as u64)
        || variables.len() != names.len()
        || variables.iter().collect::<BTreeSet<_>>() != names.iter().collect::<BTreeSet<_>>()
        || block[5][0].as_u64() != Some(1)
    {
        return Err("unsupported RUR names or scaling".into());
    }
    let dimension = block[2].as_u64().ok_or("missing quotient dimension")?;
    let data = &block[5][1];
    let eliminant = polynomial(&data[0], p)?;
    let denominator = polynomial(&data[1], p)?;
    let numerators = data[2].as_array().ok_or("missing numerators")?;
    if numerators.len() + 1 != variables.len()
        || numerators
            .iter()
            .any(|n| n.as_array().map(Vec::len) != Some(1))
    {
        return Err("unsupported numerator count or scalar factors".into());
    }
    let numerators = numerators
        .iter()
        .map(|n| polynomial(&n[0], p))
        .collect::<Result<Vec<_>, _>>()?;
    let mut roots = vec![];
    for root in 0..p {
        if univariate(&eliminant, root, p) != 0 {
            continue;
        }
        let den = univariate(&denominator, root, p);
        if den == 0 {
            roots.push(json!({"parameter":root,"denominator":den,"assignment":null,
                "equations_zero":false,"form_zero":false}));
            continue;
        }
        let inverse = power(den, (p - 2) as u32, p);
        let mut decoded = numerators
            .iter()
            .map(|q| (-univariate(q, root, p) * inverse).rem_euclid(p))
            .collect::<Vec<_>>();
        decoded.push(root);
        let point = names
            .iter()
            .map(|name| decoded[variables.iter().position(|n| n == name).unwrap()])
            .collect::<Vec<_>>();
        let values = equations
            .iter()
            .map(|eq| residual(eq, &point, p))
            .collect::<Vec<_>>();
        let form_value = residual(&form, &point, p);
        roots.push(
            json!({"parameter":root,"denominator":den,"assignment":point,
            "equation_residuals":values,"form_residual":form_value,
            "equations_zero":values.iter().all(|v| *v == 0),"form_zero":form_value == 0}),
        );
    }
    Ok(
        json!({"prime":p,"input_variables":names,"rur_variables":variables,
        "serialized_equations":equations.len(),"quotient_dimension":dimension,
        "eliminant_degree":eliminant.len()-1,
        "dimension_guard_passes":dimension == (eliminant.len()-1) as u64,
        "rational_roots":roots.len(),
        "equation_valid_roots":roots.iter().filter(|r| r["equations_zero"] == true).count(),
        "form_valid_roots":roots.iter().filter(|r| r["form_zero"] == true).count(),
        "all_residuals_zero_roots":roots.iter().filter(|r| r["equations_zero"] == true && r["form_zero"] == true).count(),
        "roots":roots,"solver_calls":0,
        "scope":"direct F127 residual evaluation of recorded coordinates; no general completeness certificate"}),
    )
}

fn audit_file(path: &str) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let records = text
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let selected = records
        .iter()
        .filter(|r| {
            r["prime"] == 127
                && r["depth"] == 4
                && r["target"] == json!([41, 95])
                && r["mode"] == "ProductionCofactorMsolve"
        })
        .collect::<Vec<_>>();
    if selected.len() != 1 {
        return Err("expected exactly one retained comparator record".into());
    }
    let record = selected[0];
    let systems = record["systems"].as_array().ok_or("missing systems")?;
    if systems.len() != 1 {
        return Err("expected one comparator system".into());
    }
    let system = &systems[0];
    let attempts = system["attempts"].as_array().ok_or("missing attempts")?;
    if attempts.len() != 1 {
        return Err("expected one retained form attempt".into());
    }
    let mut result = diagnose(
        system["input_prefix"].as_str().ok_or("missing prefix")?,
        attempts[0]["linear_equation"]
            .as_str()
            .ok_or("missing form")?,
        attempts[0]["rur"].as_str().ok_or("missing retained RUR")?,
    )?;
    result["type"] = json!("retained_rur_residual_audit");
    result["format"] = json!(1);
    result["source_path"] = json!(path);
    result["target"] = record["target"].clone();
    result["original_result_status"] = record["result"]["status"].clone();
    result["original_relations"] = record["result"]["relations"].clone();
    Ok(result)
}

fn main() -> Result<(), String> {
    let paths = env::args().skip(1).collect::<Vec<_>>();
    if paths.len() != 2 {
        return Err("usage: rur-audit OLD_JSONL NEW_JSONL".into());
    }
    let mut failed = false;
    for path in paths {
        match audit_file(&path) {
            Ok(record) => println!("{record}"),
            Err(error) => {
                println!(
                    "{}",
                    json!({"type":"retained_rur_audit_error","source_path":path,"error":error})
                );
                failed = true;
            }
        }
    }
    if failed {
        Err("one or more retained records could not be audited".into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn example() -> Value {
        json!([
            0,
            [
                127,
                2,
                2,
                ["x", "A"],
                [0, 1],
                [1, [[2, [2, 124, 1]], [0, [1]], [[[1, [0, 126]]]]]]
            ]
        ])
    }
    fn check(rur: &Value) -> Result<Value, String> {
        diagnose("x,A\n127\n1*x^2+124*x+2", "1*A+126*x", &rur.to_string())
    }
    #[test]
    fn documented_coordinate_convention_satisfies_equations_and_form() {
        let result = check(&example()).unwrap();
        assert_eq!(result["rational_roots"], 2);
        assert_eq!(result["all_residuals_zero_roots"], 2);
        assert_eq!(result["roots"][0]["assignment"], json!([1, 1]));
        assert_eq!(result["roots"][1]["assignment"], json!([2, 2]));
    }
    #[test]
    fn coordinate_mutation_is_not_hidden_by_dimension_guard() {
        let mut rur = example();
        rur[1][5][1][2][0][0][1][0] = json!(1);
        let result = check(&rur).unwrap();
        assert_eq!(result["dimension_guard_passes"], true);
        assert_eq!(result["all_residuals_zero_roots"], 0);
        assert_eq!(result["form_valid_roots"], 0);
    }
    #[test]
    fn inconsistent_polynomial_metadata_is_rejected() {
        let mut rur = example();
        rur[1][5][1][0][0] = json!(3);
        assert!(check(&rur).is_err());
    }
    #[test]
    fn repeated_monomials_and_unknown_variables_are_rejected() {
        let names = vec!["x".into(), "A".into()];
        assert!(equation("1*x+2*x", &names, 127).is_err());
        assert!(equation("1*y", &names, 127).is_err());
        assert!(equation("1*x*x", &names, 127).is_err());
    }
}
