//! Sparse solver integration for the extension-field IC relation matrix.
#[path = "../research/extension_ic_graph_20261115/sparse_kernel.rs"]
mod sparse_kernel;
#[path = "../research/extension_ic_graph_20261115/sparse_kernel_driver.rs"]
mod sparse_kernel_driver;
#[path = "../research/extension_ic_graph_20261115/sparse_kernel_input.rs"]
mod sparse_kernel_input;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("prepare") if a.len() == 4 => {
            sparse_kernel_input::prepare(&a[1..3], std::path::Path::new(&a[3]))
        }
        Some(method) if a.len() == 3 => sparse_kernel_driver::run(
            method,
            std::path::Path::new(&a[1]),
            std::path::Path::new(&a[2]),
        ),
        _ => Err(
            "usage: WORKER prepare FULL_RAW QUADRATIC_RAW NEW_INPUT | METHOD INPUT NEW_OUTPUT"
                .into(),
        ),
    }
}
