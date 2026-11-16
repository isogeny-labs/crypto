//! Exact neutral GF(2) all-root adapter around unchanged MatrixF4/MatrixF5.
use crypto_lib::cryptanalysis::koblitz_groebner::{
    solve_boolean_system, SolveOptions, SolverEngine, SplitRule,
};
use crypto_lib::cryptanalysis::pq_groebner_f2::{F2BoolMono, F2BoolPoly};
use crypto_lib::hash::sha256::sha256;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{self, Read};

const REVISION: &str = "3d737486869d1ca77e14ad56bdaf376916eecce7";
const SCHEMA: &str = "neutral_boolean_polynomial_system_v1";

fn digest(bytes: &[u8]) -> String {
    sha256(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn options(args: &[String]) -> Result<BTreeMap<String, String>, String> {
    if args.len() % 2 != 0 { return Err("Arguments must be flag/value pairs".into()); }
    let mut result = BTreeMap::new();
    for pair in args.chunks_exact(2) {
        if !["--algorithm", "--input", "--max-degree", "--node-budget", "--max-solutions"].contains(&pair[0].as_str()) {
            return Err(format!("Unknown flag {}", pair[0]));
        }
        if result.insert(pair[0].clone(), pair[1].clone()).is_some() {
            return Err(format!("Repeated flag {}", pair[0]));
        }
    }
    Ok(result)
}

fn number(input: &Value, name: &str) -> Result<usize, String> {
    input.get(name).and_then(Value::as_u64).and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| format!("{name} must be a nonnegative integer"))
}

fn parse_polynomials(input: &Value, n: usize) -> Result<Vec<F2BoolPoly>, String> {
    let equations = input.get("equations").and_then(Value::as_array).ok_or("Missing equations array")?;
    if input.get("equation_count").is_some() && number(input, "equation_count")? != equations.len() {
        return Err("equation_count disagrees with the input array".into());
    }
    equations.iter().map(|equation| {
        let monomials = equation.as_array().ok_or("Each polynomial must be a monomial array")?;
        let mut monos = Vec::with_capacity(monomials.len());
        for monomial in monomials {
            let indices = monomial.as_array().ok_or("Each monomial must be a sorted index array")?;
            let mut mask = 0u64;
            let mut previous = None;
            for index in indices {
                let v = index.as_u64().and_then(|v|usize::try_from(v).ok()).ok_or("Variable indices must be integers")?;
                if v >= n { return Err("Monomial references an out-of-range variable".into()); }
                if previous.is_some_and(|p|p >= v) { return Err("Monomial variable indices must be strictly increasing".into()); }
                previous = Some(v);
                mask |= 1u64 << v;
            }
            monos.push(F2BoolMono::from_mask(mask));
        }
        Ok(F2BoolPoly::from_monos(monos, n))
    }).collect()
}

fn check_environment(engine: SolverEngine, algorithm: &str) -> Result<(), String> {
    if engine.effective() != engine {
        return Err("Unsupported environment: KIC_F4_INHERIT changes the named engine".into());
    }
    if let Ok(criterion) = std::env::var("KIC_F4_CRITERION") {
        let required = if algorithm == "F5" { "f5" } else { "none" };
        if criterion != required {
            return Err("Unsupported environment: KIC_F4_CRITERION changes the named algorithm".into());
        }
    }
    if std::env::var("IC_GROEBNER_CACHE_URL").is_ok_and(|v| !v.is_empty()) ||
        ["IC_PREPROCESS_CACHE", "IC_REDUCTION_CACHE", "IC_PARAMETER_CACHE"].iter()
        .any(|name|std::env::var(name).as_deref() == Ok("redis")) {
        return Err("Unsupported environment: external Redis caching violates the cold-call contract".into());
    }
    Ok(())
}

fn execute(input: &Value, raw: &[u8], algorithm: &str, max_degree: u32,
    node_budget: usize, max_solutions: usize) -> Result<Value, String> {
    if input.get("schema").and_then(Value::as_str) != Some(SCHEMA) {
        return Err("Unsupported input schema".into());
    }
    let field = input.get("field").ok_or("Missing field metadata")?;
    if field.get("kind").and_then(Value::as_str) != Some("prime") ||
        field.get("p").or_else(||field.get("characteristic")).and_then(Value::as_u64) != Some(2) {
        return Err("Unsupported: native matrix adapter accepts GF(2) polynomials only".into());
    }
    if input.get("domain") != Some(&json!([0,1])) {
        return Err("Unsupported: the neutral polynomial domain must be [0,1]".into());
    }
    let n = number(input, "n_vars")?;
    if n == 0 || n > 64 {
        return Err("Unsupported: original native monomial encoding requires 1 through 64 variables; n83's339-variable system is unsupported".into());
    }
    if max_degree == 0 || node_budget == 0 || max_solutions == 0 {
        return Err("Degree, node budget, and solution cap must be positive".into());
    }
    let engine = match algorithm {
        "F4" => SolverEngine::MatrixF4 { max_degree },
        "F5" => SolverEngine::MatrixF5 { max_degree },
        _ => return Err("Unsupported algorithm: select the genuine MatrixF4 or MatrixF5 engine".into()),
    };
    check_environment(engine, algorithm)?;
    let equations = parse_polynomials(input, n)?;
    let input_sha256 = digest(raw);
    if std::env::var("ROOTFINDER_INSTANCE_SHA256").is_ok_and(|expected| expected != input_sha256) {
        return Err("Input SHA256 disagrees with the frozen orchestration hash".into());
    }
    let opts = SolveOptions { engine, max_solutions, node_budget, split_rule: SplitRule::Auto };
    let (assignments, stats) = solve_boolean_system(&equations, n, &opts);
    let verified = assignments.iter().all(|&mask| {
        (n == 64 || mask >> n == 0) && equations.iter().all(|p| p.eval(mask) == 0)
    });
    let reported_assignments = assignments.len();
    let mut roots = assignments;
    roots.sort_unstable();
    roots.dedup();
    // Upstream stops at its output cap without necessarily setting exhausted.
    // Equality with the cap remains unresolved even if no further root exists.
    let complete = verified && !stats.exhausted && !stats.unsupported && reported_assignments < max_solutions;
    let status = if !verified { "verification_failed" } else if stats.unsupported { "unsupported" }
        else if complete { "complete" } else { "exhausted" };
    let solver_environment: BTreeMap<_,_> = ["KIC_F4_MAX_DEGREE_ONLY", "KIC_LINEAR_ELIM", "KIC_SPLIT_RULE",
        "KIC_TAIL_ENUM", "KIC_F4_BATCH", "KIC_F4_MAX_ROWS", "KIC_F4_MAX_COLS"]
        .iter().filter_map(|name|std::env::var(name).ok().map(|value|((*name).to_string(),value))).collect();
    Ok(json!({
        "schema": "neutral_boolean_all_roots_result_v1", "schema_version": 1,
        "algorithm": algorithm, "implementation": "aburan28/crypto genuine Boolean matrix solver",
        "source_revision": REVISION, "revision": REVISION,
        "instance_id": input.get("instance_id"), "input_sha256": input_sha256,
        "field": field, "n_vars":n, "domain":[0,1],
        "roots_hex": roots.iter().map(|mask|format!("0x{mask:x}")).collect::<Vec<_>>(),
        "root_count": roots.len(), "count_exact": complete,
        "reported_assignments_before_deduplication": reported_assignments,
        "complete": complete, "verified": verified, "status": status, "stats": stats,
        "configuration": {"engine":engine,"max_degree":max_degree,"node_budget":node_budget,
            "max_solutions":max_solutions,"split_rule":"Auto","recorded_environment":solver_environment},
        "pipeline": ["input", "GF(2)_field_domain", "native_polynomials", "solve", "extract", "verify", "serialize"],
        "contract": "all original-variable Boolean roots of the supplied neutral coordinate equations"
    }))
}

fn run(args: &[String]) -> Result<Value, String> {
    let flags = options(args)?;
    let algorithm = flags.get("--algorithm").map(String::as_str).unwrap_or("F4");
    let raw = if let Some(path) = flags.get("--input") {
        std::fs::read(path).map_err(|e|e.to_string())?
    } else {
        let mut raw = Vec::new();
        io::stdin().read_to_end(&mut raw).map_err(|e|e.to_string())?;
        raw
    };
    let input: Value = serde_json::from_slice(&raw).map_err(|e|e.to_string())?;
    let max_degree = flags.get("--max-degree").map(String::as_str).unwrap_or("3").parse::<u32>().map_err(|e|e.to_string())?;
    let node_budget = flags.get("--node-budget").map(String::as_str).unwrap_or("1000000").parse::<usize>().map_err(|e|e.to_string())?;
    let max_solutions = flags.get("--max-solutions").map(String::as_str).unwrap_or("1000000").parse::<usize>().map_err(|e|e.to_string())?;
    execute(&input, &raw, algorithm, max_degree, node_budget, max_solutions)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let algorithm = args.windows(2).find(|p|p[0] == "--algorithm").map(|p|p[1].as_str()).unwrap_or("F4");
    let value = run(&args).unwrap_or_else(|reason|json!({
        "schema":"neutral_boolean_all_roots_result_v1", "schema_version":1,
        "algorithm":algorithm,"implementation":"aburan28/crypto genuine Boolean matrix solver",
        "source_revision":REVISION,"revision":REVISION,
        "complete":false,"count_exact":false,"verified":false,
        "status":if reason.starts_with("Unsupported") {"unsupported"} else {"failed"},
        "reason":reason
    }));
    println!("{value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_native_arms_match_independent_small_cubic_oracles() {
        let mut random = 0x9f06a73727bc0981u64;
        for case in 0..24 {
            let n = 1+case%8;
            let mut equations: Vec<Vec<Vec<usize>>> = Vec::new();
            for _ in 0..4 {
                let mut monomials = Vec::new();
                for _ in 0..13 {
                    random ^= random << 13; random ^= random >> 7; random ^= random << 17;
                    let indices: Vec<_> = (0..n).filter(|&v|random & (1 << v) != 0).take(3).collect();
                    monomials.push(indices);
                }
                equations.push(monomials);
            }
            let expected: Vec<_> = (0u64..(1 << n)).filter(|mask| {
                equations.iter().all(|poly|poly.iter().filter(|mono|mono.iter().all(|&v|mask >> v & 1 == 1)).count()%2 == 0)
            }).map(|mask|format!("0x{mask:x}")).collect();
            let input = json!({"schema":SCHEMA,"field":{"kind":"prime","p":2},
                "domain":[0,1],"n_vars":n,"equations":equations,"instance_id":format!("test{case}")});
            let raw = serde_json::to_vec(&input).unwrap();
            for algorithm in ["F4","F5"] {
                let result = execute(&input,&raw,algorithm,3,1_000_000,1_000_000).unwrap();
                assert_eq!(result["complete"],true,"{case} {algorithm}");
                assert_eq!(result["verified"],true);
                assert_eq!(result["roots_hex"],json!(expected),"{case} {algorithm}");
            }
        }
    }

    #[test]
    fn output_cap_and_large_encoding_fail_closed() {
        let input = json!({"schema":SCHEMA,"field":{"kind":"prime","p":2},
            "domain":[0,1],"n_vars":3,"equations":[[]]});
        let raw = serde_json::to_vec(&input).unwrap();
        for algorithm in ["F4","F5"] {
            let result = execute(&input,&raw,algorithm,3,1_000_000,1).unwrap();
            assert_eq!(result["root_count"],1);
            assert_eq!(result["complete"],false);
            assert_eq!(result["count_exact"],false);
            assert_eq!(result["verified"],true);
        }
        let mut large = input;
        large["n_vars"] = json!(339);
        let error = execute(&large,&raw,"F4",3,1_000_000,1_000_000).unwrap_err();
        assert!(error.starts_with("Unsupported"));
    }
}
