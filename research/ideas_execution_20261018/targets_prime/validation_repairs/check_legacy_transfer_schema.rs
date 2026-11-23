//! Read-only diagnosis of the separate, unmodified historical report.
use crypto_lib::cryptanalysis::jacobian_transfer::TransferFile;

fn main() {
    let text = std::fs::read_to_string("docs/curves/jacobian-transfers.json").unwrap();
    let result = serde_json::from_str::<TransferFile>(&text);
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "path": "docs/curves/jacobian-transfers.json",
        "current_typed_consumer_accepts": result.is_ok(),
        "error": result.err().map(|e| e.to_string()),
        "scope": "pre-existing generated-report drift, recorded but not repaired by this task"
    })).unwrap());
}
