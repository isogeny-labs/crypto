use prime_shared_coeff_ir::{build_ir, e0_f17_panel, InstanceSpec};
use std::process::ExitCode;

fn parse_u64(value: &str, name: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("invalid {name}: expected an unsigned decimal integer"))
}

fn emit(args: &[String]) -> Result<(), String> {
    if args.len() != 9 {
        return Err(
            "usage: prime-shared-coeff-ir emit P A B M T C TARGET_X TARGET_Y json|msolve|qf-ff"
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
        _ => return Err("format must be json, msolve, or qf-ff".to_owned()),
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
        _ => {
            Err("usage: prime-shared-coeff-ir emit ... | prime-shared-coeff-ir e0-panel".to_owned())
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
