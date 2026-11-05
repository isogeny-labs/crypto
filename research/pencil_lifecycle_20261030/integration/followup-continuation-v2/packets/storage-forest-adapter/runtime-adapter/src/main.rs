use serde_json::json;
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let result = if args == ["--self-test"] {
        pencil_readonly_accounting::fixed_controls()
    } else {
        Err("usage: pencil-readonly-accounting --self-test (pure fixed controls only)".into())
    };
    match result {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            println!("{}", json!({"accepted":false,"error":e}));
            std::process::exit(1)
        }
    }
}
