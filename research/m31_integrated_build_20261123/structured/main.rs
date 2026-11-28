use m31_structured_f4::{solve, Formulation, Limits, Preprocessing, Problem};
use serde::Deserialize;
use std::io::Read;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    problem: Problem,
    formulation: Formulation,
    preprocessing: Preprocessing,
    #[serde(default)]
    limits: Limits,
}

fn main() {
    let result = (|| -> Result<_, String> {
        let mut bytes = Vec::new();
        std::io::stdin().take(1_048_577).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        if bytes.len() > 1_048_576 { return Err("request exceeds 1 MiB".into()); }
        let request: Request = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        solve(&request.problem, request.formulation, request.preprocessing, request.limits).map_err(|e| e.to_string())
    })();
    match result {
        Ok(report) => {
            println!("{}", serde_json::to_string(&report).expect("serialize report"));
            if report.status == m31_structured_f4::Status::Incomplete { std::process::exit(3); }
        }
        Err(error) => {
            println!("{}", serde_json::json!({"status":"refused", "error":error}));
            std::process::exit(2);
        }
    }
}
