//! sgsecant: native exact triangular/FGLM extractor and complete-cost harness for the
//! split-graph secant presentation. Development checks (`check*`), prospective panel
//! (`panel`), and a tiny `smoke` timing run. No dependencies.
#[path = "sgsecant_alg.rs"]
mod alg;
#[path = "sgsecant_ec.rs"]
mod ec;
#[path = "sgsecant_ops.rs"]
mod ops;
#[path = "sgsecant_run.rs"]
mod run;
#[path = "sgsecant_sys.rs"]
mod sys;
#[path = "sgsecant_panel.rs"]
mod panel;
#[path = "sgsecant_check.rs"]
mod check;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "check" => check::check_f17(),
        "check-legacy" => check::check_legacy(args.get(2).map(|s| s.as_str()).unwrap_or("83"), args.get(3).map(|s| s.as_str()).unwrap_or("3")),
        "check-cap" => check::check_cap(),
        "check-unit" => check::check_unit(),
        "panel" => panel::panel_main(&args[2..]),
        "help" | _ => {
            eprintln!("usage: sgsecant check | check-unit | check-legacy <p> <d> | check-cap | panel <protocol-args>");
        }
    }
}
