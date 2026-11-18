#[cfg(target_os = "linux")]
include!("../research/ml_dataset_20261028/startup-integration-cli-fixture-native/run.rs");

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("fixture requires the native Linux supervisor contract");
    std::process::exit(1);
}
