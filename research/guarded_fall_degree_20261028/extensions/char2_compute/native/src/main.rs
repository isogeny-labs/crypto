use char2_fall_degree::{
    closure, verify_lower, verify_refutation, Closure, Field, Problem, Refutation,
};
use std::{env, fs, process};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("field") if args.len() == 3 => {
            let f = Field::new(args[2].parse()?)?;
            println!("{}", serde_json::to_string_pretty(&f.descriptor)?);
        }
        Some("closure") if args.len() == 4 || args.len() == 5 => {
            let p: Problem = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
            let d: usize = args[3].parse()?;
            let trace = args.get(4).is_some_and(|s| s == "--trace");
            let result = closure(&p, d, trace)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Some("verify-lower") if args.len() == 4 => {
            let p: Problem = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
            let c: Closure = serde_json::from_str(&fs::read_to_string(&args[3])?)?;
            let valid = verify_lower(&p, &c);
            println!("{{\"valid_lower_certificate\":{valid}}}");
            if !valid {
                return Err("invalid lower certificate".into());
            }
        }
        Some("verify-upper") if args.len() == 4 => {
            let p: Problem = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
            let c: Refutation = serde_json::from_str(&fs::read_to_string(&args[3])?)?;
            let valid = verify_refutation(&p, &c);
            println!("{{\"valid_refutation\":{valid}}}");
            if !valid {
                return Err("invalid refutation".into());
            }
        }
        Some("verify-result") if args.len() == 4 => {
            let p: Problem = serde_json::from_str(&fs::read_to_string(&args[2])?)?;
            let c: Closure = serde_json::from_str(&fs::read_to_string(&args[3])?)?;
            let lower = verify_lower(&p, &c);
            let upper = c
                .refutation
                .as_ref()
                .is_some_and(|proof| verify_refutation(&p, proof));
            println!("{{\"valid_lower_certificate\":{lower},\"valid_refutation\":{upper}}}");
            if !lower && !upper {
                return Err("result contains no valid certificate".into());
            }
        }
        _ => {
            return Err(concat!(
            "usage: char2-fall-degree field MODULUS | closure PROBLEM DEGREE [--trace] | ",
            "verify-lower PROBLEM CERT | verify-upper PROBLEM CERT | verify-result PROBLEM RESULT"
        )
            .into())
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
