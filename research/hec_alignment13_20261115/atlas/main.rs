#[allow(dead_code)]
mod source {
    include!("weil.rs");
    include!("theta.rs");
    include!("coordinates.rs");
    include!("source_api.rs");
}
#[allow(dead_code)]
mod target_program {
    include!("target_program.rs");
    include!("fit_driver.rs");
}
fn main() {
    target_program::run_fit();
}
