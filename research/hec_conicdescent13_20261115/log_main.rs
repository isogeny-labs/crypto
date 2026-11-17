#[allow(dead_code)]
mod conic {
    include!("main.rs");
    include!("log_api.rs");
}
#[path = "../../crates/hec-native-ic/src/fp.rs"]
mod fp;
#[path = "../../crates/hec-native-ic/src/jac.rs"]
mod jac;
#[path = "../../crates/hec-native-ic/src/poly.rs"]
mod poly;
#[path = "../../crates/hec-native-ic/src/rng.rs"]
mod rng;
fn main() {
    conic::run_logs();
}
