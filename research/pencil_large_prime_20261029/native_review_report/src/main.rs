fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: pencil-native-review-report REQUEST.json NEW_OUTPUT_DIRECTORY".into());
    }
    let request = serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    pencil_native_review_report::write_report(&request, std::path::Path::new(&args[2]))
}
