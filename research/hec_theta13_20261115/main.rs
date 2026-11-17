#[allow(dead_code)]
mod previous {
    include!("../hec_weil13_20261115/main.rs");
    include!("gate.rs");
}
fn main() {
    previous::run_theta();
}
