//! Runtime planning only: one cold recovery per admitted invocation.
use std::{io::Write, time::Instant};
#[path = "../feature-native-accounting/meter.rs"] pub mod meter;
#[global_allocator] static ALLOCATOR: meter::Counting = meter::Counting;
#[path = "../feature-native-pair-matched/generated-baseline/recovery.rs"] mod baseline;
#[path = "../feature-native-pair-matched/generated-candidate/recovery.rs"] mod candidate;

const POINT: Option<[u64; 2]> = Some([235, 254]);
const EXPECTED: u64 = 173; // Checker only; never an argument to recovery.

#[derive(Debug, PartialEq)]
enum Mode { Describe, Run { arm: String, contract: String } }

fn parse(args: &[String]) -> Result<Mode, &'static str> {
    if args == ["--describe"] { return Ok(Mode::Describe); }
    if args.len() != 3 || args[0] != "--run-admitted-preflight"
        || !matches!(args[1].as_str(), "baseline" | "candidate" | "aa")
        || args[2].len() != 64 || !args[2].bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err("usage: preflight --describe | --run-admitted-preflight baseline|candidate|aa CONTRACT_SHA256");
    }
    Ok(Mode::Run { arm: args[1].clone(), contract: args[2].to_ascii_lowercase() })
}

fn elapsed_ns(start: Instant, end: Instant) -> Option<u128> {
    end.checked_duration_since(start).map(|d| d.as_nanos())
}

// All state, including prepared buffers and decoder storage, drops inside this
// call before the stop timestamp. Both arms use the existing instrumented code.
#[derive(Default)]
struct PhaseTimes { fixed: Option<u128>, query: Option<u128> }

fn recover_once(arm: &str, times: &mut PhaseTimes) -> Result<u64, &'static str> {
    meter::reset();
    macro_rules! run { ($module:ident) => {{
        let fixed_start = Instant::now();
        let decoder = $module::Decoder::repaired_479().map_err(|_| "setup_failed")?;
        let prepared = decoder.prepare().map_err(|_| "prepare_failed")?;
        times.fixed = elapsed_ns(fixed_start, Instant::now());
        let query_start = Instant::now();
        let result = prepared.recover(POINT);
        times.query = elapsed_ns(query_start, Instant::now());
        let recovered = result.map_err(|_| "recovery_failed")?;
        Ok(recovered.scalar)
    }} }
    match arm { "baseline" | "aa" => run!(baseline), "candidate" => run!(candidate),
        _ => Err("invalid_arm") }
}

fn outcome(result: &Result<u64, &'static str>, duration: Option<u128>) -> &'static str {
    match result {
        Err(stage) => stage,
        Ok(scalar) if *scalar != EXPECTED => "label_mismatch",
        Ok(_) if duration.is_none() => "clock_invalid",
        Ok(_) => "ok",
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = parse(&args).unwrap_or_else(|error| { eprintln!("{error}"); std::process::exit(2) });
    let Mode::Run { arm, contract } = mode else {
        println!("native-pair-preflight/v1; one cold recovery; fixed public Q=(235,254); baseline|candidate|aa; external admission required; no execution");
        return;
    };
    // A start marker survives ordinary recovery failures. Panic/kill/timeout
    // leaves a partial record, never a successful runtime estimate.
    println!("{{\"schema\":\"native-pair-preflight-start/v1\",\"arm\":\"{arm}\",\"contract_sha256\":\"{contract}\"}}");
    std::io::stdout().flush().expect("flush start marker");
    let mut times = PhaseTimes::default();
    let start = Instant::now();
    let result = recover_once(&arm, &mut times);
    let duration = elapsed_ns(start, Instant::now());
    let clock_valid = duration.is_some() && times.fixed.is_some() && times.query.is_some();
    let status = outcome(&result, if clock_valid { duration } else { None });
    let scalar = result.as_ref().map_or("null".to_owned(), |k| k.to_string());
    let nanos = duration.map_or("null".to_owned(), |n| n.to_string());
    let fixed = times.fixed.map_or("null".to_owned(), |n| n.to_string());
    let query = times.query.map_or("null".to_owned(), |n| n.to_string());
    let verified = result == Ok(EXPECTED);
    println!("{{\"schema\":\"native-pair-preflight/v1\",\"arm\":\"{arm}\",\"contract_sha256\":\"{contract}\",\"target\":173,\"scalar\":{scalar},\"verified\":{verified},\"status\":\"{status}\",\"cold_call_ns\":{nanos},\"setup_prepare_ns\":{fixed},\"query_verify_ns\":{query},\"clock_valid\":{clock_valid},\"runtime_planning_only\":true}}");
    std::io::stdout().flush().expect("flush final receipt");
    if status != "ok" { std::process::exit(1); }
}

#[cfg(test)] mod preflight_tests {
    use super::*;
    #[test] fn invocation_and_fixture_contract() {
        let hash = "AB".repeat(32);
        for arm in ["baseline", "candidate", "aa"] {
            assert_eq!(parse(&["--run-admitted-preflight".into(), arm.into(), hash.clone()]),
                Ok(Mode::Run { arm: arm.into(), contract: hash.to_ascii_lowercase() }));
        }
        for args in [vec![], vec!["--run-frozen-panel".into(), hash.clone()],
            vec!["--run-admitted-preflight".into(), "all".into(), hash.clone()],
            vec!["--run-admitted-preflight".into(), "candidate".into(), "x".repeat(64)],
            vec!["--run-admitted-preflight".into(), "baseline".into(), "a".repeat(63)],
            vec!["--describe".into(), "extra".into()]] { assert!(parse(&args).is_err()); }
        assert_eq!(parse(&["--describe".into()]), Ok(Mode::Describe));
        let row = include_str!("../feature-native-recovery/targets.tsv").lines()
            .find(|s| s.starts_with("173\t")).unwrap();
        let fields: Vec<u64> = row.split('\t').map(|s| s.parse().unwrap()).collect();
        assert_eq!(fields, vec![EXPECTED, POINT.unwrap()[0], POINT.unwrap()[1]]);
    }
    #[test] fn failure_and_clock_contract() {
        let start = Instant::now();
        let end = start.checked_add(std::time::Duration::from_nanos(7)).unwrap();
        assert_eq!(elapsed_ns(start, end), Some(7));
        assert_eq!(elapsed_ns(end, start), None);
        assert_eq!(outcome(&Ok(EXPECTED), Some(7)), "ok");
        assert_eq!(outcome(&Ok(EXPECTED), None), "clock_invalid");
        assert_eq!(outcome(&Ok(25), Some(7)), "label_mismatch");
        for stage in ["setup_failed", "prepare_failed", "recovery_failed"] {
            assert_eq!(outcome(&Err(stage), Some(7)), stage);
        }
    }
}
