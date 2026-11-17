fn main() {
    match torus_section_search::run_controls() {
        Ok(v) => println!(
            "{}",
            serde_json::to_string_pretty(&v).expect("JSON value serializes")
        ),
        Err(e) => {
            eprintln!("section-search controls refused: {e}");
            std::process::exit(1);
        }
    }
}
