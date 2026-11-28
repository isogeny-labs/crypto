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
    include!("../hec_alignment13_20261115/atlas/target_program.rs");
    include!("recover_model.rs");
}
#[allow(dead_code)]
mod conic {
    include!("../hec_conicdescent13_20261115/main.rs");
    include!("descent_api.rs");
}
fn main() {
    let (pts, logs) = points::fresh();
    conic::run(&pts, &logs);
}
