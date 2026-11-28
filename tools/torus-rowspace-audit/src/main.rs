use std::path::Path;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let result = if args.get(1).is_some_and(|x| x == "--identity") {
        serde_json::to_string_pretty(&torus_rowspace_audit::controls::sources())
            .map(|s| println!("{s}"))
            .map_err(|e| e.to_string())
    } else if let Some(out) = args.get(1) {
        torus_rowspace_audit::controls::run(Path::new(out))
    } else {
        Err("expected NEW output directory or --identity".into())
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
