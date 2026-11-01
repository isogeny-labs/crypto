use std::path::Path;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = if args.len() == 4 {
        hashed_cpu_panel_tools::export(Path::new(&args[1]), Path::new(&args[2]), &args[3])
    } else {
        Err("usage: hashed-cpu-panel-tools STUDY_DIR FROZEN_ANALYZER UTC_TIMESTAMP".into())
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
    println!("Wrote complete native-panel export");
}
