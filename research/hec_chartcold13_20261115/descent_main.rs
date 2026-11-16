mod config;
mod setup {
    include!("setup.rs");
}
mod public {
    include!("public_input.rs");
}
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("../hec_alignment13_20261115/atlas/target_api.rs");
}
#[allow(dead_code)]
mod lp_trace {
    include!("lp_trace.rs");
}
#[allow(dead_code)]
mod points {
    include!("point_logs.rs");
}
#[allow(dead_code)]
mod model {
    include!("runtime_program.rs");
    include!("recover_model.rs");
}
#[allow(dead_code)]
mod conic {
    include!("conic_dynamic.rs");
    include!("descent_api.rs");
}
fn main() {
    let (pts, logs) = points::fresh();
    conic::run(&pts, &logs);
}
