#[allow(dead_code)]
mod previous {
    include!("../hec_weil13_20261115/main.rs");
    include!("../hec_theta13_20261115/gate.rs");
    include!("../hec_kummer13_20261115/coordinates.rs");
    include!("relations.rs");
}
fn main() {
    previous::run_image();
}
