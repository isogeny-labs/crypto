use prime31_rr9_and_closures::p2_6_suite::run_lead_p2_6_suite;

fn main() {
    let report = run_lead_p2_6_suite();
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
