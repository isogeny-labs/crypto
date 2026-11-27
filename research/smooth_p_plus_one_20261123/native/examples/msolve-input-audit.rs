//! Offline comparison of literal input with the isolated upstream parser.
//! Uses retained assignments from rur-audit; never calls msolve or finds roots.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeSet, env, fs, process::Command};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Term {
    coefficient: i64,
    exponents: Vec<u32>,
}

fn parser(binary: &str, p: i64, names: &[String], polynomial: &str) -> Result<Vec<Term>, String> {
    let output = Command::new(binary)
        .args([p.to_string(), names.join(","), polynomial.to_string()])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() || !output.stderr.is_empty() {
        return Err(format!("parser probe failed: {}", output.status));
    }
    let terms: Vec<Term> = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if terms.is_empty()
        || terms.iter().any(|t| {
            !(0..p).contains(&t.coefficient)
                || t.exponents.len() != names.len()
                || t.exponents.iter().any(|e| *e > 16)
        })
    {
        return Err("invalid parser terms".into());
    }
    Ok(terms)
}

fn literal(poly: &str, p: i64, names: &[String]) -> Result<Vec<Term>, String> {
    let mut seen = BTreeSet::new();
    poly.split('+')
        .map(|part| {
            let mut factors = part.split('*');
            let coefficient = factors
                .next()
                .ok_or("empty term")?
                .parse::<i64>()
                .map_err(|_| "invalid coefficient")?;
            if !(0..p).contains(&coefficient) {
                return Err("noncanonical coefficient".into());
            }
            let mut exponents = vec![0; names.len()];
            for factor in factors {
                let (name, exponent) = match factor.split_once('^') {
                    Some((n, e)) => (n, e.parse::<u32>().map_err(|_| "invalid exponent")?),
                    None => (factor, 1),
                };
                let i = names
                    .iter()
                    .position(|n| n == name)
                    .ok_or("unknown variable")?;
                if !(1..=16).contains(&exponent) || exponents[i] != 0 {
                    return Err("unsupported exponent or repeated factor".into());
                }
                exponents[i] = exponent;
            }
            if !seen.insert(exponents.clone()) {
                return Err("duplicate monomial".into());
            }
            Ok(Term {
                coefficient,
                exponents,
            })
        })
        .collect()
}

fn evaluate(terms: &[Term], assignment: &[i64], p: i64) -> i64 {
    terms.iter().fold(0, |sum, term| {
        let value = term
            .exponents
            .iter()
            .zip(assignment)
            .fold(term.coefficient, |v, (e, x)| {
                (0..*e).fold(v, |a, _| a * x % p)
            });
        (sum + value).rem_euclid(p)
    })
}

fn self_test(binary: &str) -> Result<(), String> {
    let names = ["t0", "a1", "a0", "A"].map(str::to_owned).to_vec();
    let cases = [
        (127, "1*A+126*t0+0*a1+0*a0", "1*A+126*t0+1*a1+1*a0"),
        (127, "1*A+126*t0", "1*A+126*t0"),
        (127, "1*A+126*t0+126*a1+126*a0", "1*A+126*t0+126*a1+126*a0"),
        (8191, "1*A+8190*t0+0*a1+0*a0", "1*A+8190*t0+1*a1+1*a0"),
        (127, "0*A+1*t0", "0*A+1*t0"),
        (127, "1*A-0*t0", "1*A+126*t0"),
        (127, "0", "0"),
    ];
    for (p, input, expected) in cases {
        if parser(binary, p, &names, input)? != literal(expected, p, &names)? {
            return Err(format!("synthetic parser control failed: {input}"));
        }
    }
    if literal("1*A+1*A", 127, &names).is_ok()
        || literal("1*unknown", 127, &names).is_ok()
        || literal("1*A*A", 127, &names).is_ok()
    {
        return Err("literal input rejection control failed".into());
    }
    let form = literal("1*A+126*t0+1*a1+1*a0", 127, &names)?;
    if evaluate(&form, &[9, 2, 3, 4], 127) != 0 || evaluate(&form, &[9, 2, 3, 5], 127) != 1 {
        return Err("residual control failed".into());
    }
    println!(
        "{}",
        json!({"type":"msolve_parser_synthetic_controls","parser_cases":7,
        "literal_rejection_cases":3,"residual_cases":2,"passed":true,"solver_calls":0})
    );
    Ok(())
}

fn read_jsonl(path: &str) -> Result<Vec<Value>, String> {
    fs::read_to_string(path)
        .map_err(|e| e.to_string())?
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|e| e.to_string()))
        .collect()
}

fn audit(binary: &str, path: &str, saved: &[Value]) -> Result<Value, String> {
    let records = read_jsonl(path)?;
    let selected: Vec<_> = records
        .iter()
        .filter(|r| {
            r["prime"] == 127
                && r["depth"] == 4
                && r["target"] == json!([41, 95])
                && r["mode"] == "ProductionCofactorMsolve"
        })
        .collect();
    let saved: Vec<_> = saved.iter().filter(|r| r["source_path"] == path).collect();
    if selected.len() != 1 || saved.len() != 1 {
        return Err("nonunique selected record".into());
    }
    let source = selected[0];
    let saved = saved[0];
    if source["systems"].as_array().map(Vec::len) != Some(1)
        || source["systems"][0]["attempts"].as_array().map(Vec::len) != Some(1)
    {
        return Err("unexpected systems or attempts".into());
    }
    let system = &source["systems"][0];
    let prefix = system["input_prefix"].as_str().ok_or("missing prefix")?;
    let mut lines = prefix.splitn(3, '\n');
    let names: Vec<_> = lines
        .next()
        .ok_or("missing variables")?
        .split(',')
        .map(str::to_owned)
        .collect();
    let p: i64 = lines
        .next()
        .ok_or("missing prime")?
        .parse()
        .map_err(|_| "bad prime")?;
    if p != 127 || names.len() != 8 || json!(names) != saved["input_variables"] {
        return Err("unexpected input identity".into());
    }
    let raw_equations: Vec<String> = lines
        .next()
        .ok_or("missing equations")?
        .split(',')
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();
    let raw_form = system["attempts"][0]["linear_equation"]
        .as_str()
        .ok_or("missing form")?;
    let equations = raw_equations
        .iter()
        .map(|s| parser(binary, p, &names, s))
        .collect::<Result<Vec<_>, _>>()?;
    let originals = raw_equations
        .iter()
        .map(|s| literal(s, p, &names))
        .collect::<Result<Vec<_>, _>>()?;
    let form = parser(binary, p, &names, raw_form)?;
    let original_form = literal(raw_form, p, &names)?;
    let mut roots = vec![];
    for root in saved["roots"].as_array().ok_or("missing prior roots")? {
        let assignment: Vec<i64> =
            serde_json::from_value(root["assignment"].clone()).map_err(|e| e.to_string())?;
        if assignment.len() != names.len() || assignment.iter().any(|x| !(0..p).contains(x)) {
            return Err("invalid prior assignment".into());
        }
        let residuals: Vec<_> = equations
            .iter()
            .map(|e| evaluate(e, &assignment, p))
            .collect();
        let original_residuals: Vec<_> = originals
            .iter()
            .map(|e| evaluate(e, &assignment, p))
            .collect();
        let original_form_residual = evaluate(&original_form, &assignment, p);
        if json!(original_residuals) != root["equation_residuals"]
            || json!(original_form_residual) != root["form_residual"]
        {
            return Err("prior literal audit does not reconcile".into());
        }
        let form_residual = evaluate(&form, &assignment, p);
        roots.push(json!({"parameter":root["parameter"],"assignment":assignment,
            "effective_equation_residuals":residuals,"effective_equations_zero":residuals.iter().all(|x| *x==0),
            "literal_form_residual":original_form_residual,"effective_form_residual":form_residual}));
    }
    let equations_equal = originals == equations;
    let form_equal = original_form == form;
    Ok(
        json!({"type":"retained_msolve_input_audit","format":1,"source_path":path,
        "prime":p,"target":[41,95],"variables":names,"original_equations":equations.len(),
        "original_equations_parse_identically":equations_equal,"form_parses_identically":form_equal,
        "literal_form":raw_form,"parsed_form_terms":form,"parsed_equations":equations,
        "saved_assignment_count":roots.len(),"prior_literal_residuals_reconciled":true,
        "effective_equation_valid_assignments":roots.iter().filter(|r| r["effective_equations_zero"]==true).count(),
        "effective_form_valid_assignments":roots.iter().filter(|r| r["effective_form_residual"]==0).count(),
        "effective_all_valid_assignments":roots.iter().filter(|r| r["effective_equations_zero"]==true && r["effective_form_residual"]==0).count(),
        "roots":roots,"solver_calls":0,"root_searches":0,
        "scope":"isolated upstream v0.7.5 parser excerpt; not installed-binary provenance or general completeness"}),
    )
}

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 2 && args[0] == "--self-test" {
        return self_test(&args[1]);
    }
    if args.len() != 4 {
        return Err(
            "usage: msolve-input-audit PARSER OLD_JSONL NEW_JSONL PRIOR_AUDIT_JSONL".into(),
        );
    }
    let saved = read_jsonl(&args[3])?;
    let mut failed = false;
    for path in &args[1..3] {
        match audit(&args[0], path, &saved) {
            Ok(value) => println!("{value}"),
            Err(error) => {
                failed = true;
                println!(
                    "{}",
                    json!({"type":"msolve_input_audit_error","source_path":path,"error":error})
                );
            }
        }
    }
    if failed {
        Err("one or more input audits failed".into())
    } else {
        Ok(())
    }
}
