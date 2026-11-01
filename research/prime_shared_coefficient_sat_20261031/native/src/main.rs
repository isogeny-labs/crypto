use prime_shared_coeff_ir::{
    boolean::BooleanEncoding, build_ir, e0_f17_panel, e1::run_e1_sat_panel, InstanceSpec,
};
use std::path::Path;
use std::process::ExitCode;

fn parse_u64(value: &str, name: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("invalid {name}: expected an unsigned decimal integer"))
}

fn emit(args: &[String]) -> Result<(), String> {
    if args.len() != 9 {
        return Err(
            "usage: prime-shared-coeff-ir emit P A B M T C TARGET_X TARGET_Y json|msolve|qf-ff|cnf|cnf-xor"
                .to_owned(),
        );
    }
    let spec = InstanceSpec {
        modulus: parse_u64(&args[0], "P")?,
        curve_a: parse_u64(&args[1], "A")?,
        curve_b: parse_u64(&args[2], "B")?,
        arity: parse_u64(&args[3], "M")?
            .try_into()
            .map_err(|_| "M does not fit usize".to_owned())?,
        depth: parse_u64(&args[4], "T")?
            .try_into()
            .map_err(|_| "T does not fit usize".to_owned())?,
        terminal_c: parse_u64(&args[5], "C")?,
        target_x: parse_u64(&args[6], "TARGET_X")?,
        target_y: parse_u64(&args[7], "TARGET_Y")?,
    };
    let ir = build_ir(spec)?;
    match args[8].as_str() {
        "json" => println!("{}", ir.canonical_json_pretty()?),
        "msolve" => print!("{}", ir.to_msolve()?),
        "qf-ff" => print!("{}", ir.to_qf_ff()?),
        "cnf" => print!("{}", BooleanEncoding::from_ir(&ir)?.to_cnf()),
        "cnf-xor" => print!("{}", BooleanEncoding::from_ir(&ir)?.to_cnf_xor()),
        _ => return Err("format must be json, msolve, qf-ff, cnf, or cnf-xor".to_owned()),
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("emit") => emit(&args[1..]),
        Some("e0-panel") if args.len() == 1 => {
            println!(
                "{}",
                serde_json::to_string_pretty(&e0_f17_panel()?).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        Some("e1-sat-panel") if args.len() == 7 => {
            let report = run_e1_sat_panel(
                Path::new(&args[1]),
                Path::new(&args[2]),
                Path::new(&args[3]),
                Path::new(&args[4]),
                Path::new(&args[5]),
                &args[6],
            )?;
            println!(
                "{}: {}/{} exact target/backend matches; {} checked UNSAT proofs",
                report.verdict,
                report.exact_target_backend_matches,
                report.expected_target_backend_matches,
                report.checked_terminal_unsat_proofs
            );
            Ok(())
        }
        _ => {
            Err(
                "usage: prime-shared-coeff-ir emit ... | prime-shared-coeff-ir e0-panel | prime-shared-coeff-ir e1-sat-panel CADICAL LRAT_TRIM CRYPTOMINISAT CAKE_XLRUP OUTPUT_DIR SOURCE_REVISION"
                    .to_owned(),
            )
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
