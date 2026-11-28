mod sparse;
mod config { include!("../hec_chartcold13_20261115/config.rs"); }
mod setup {
    include!("../hec_chartcold13_20261115/setup.rs");
}
mod public {
    include!("../hec_chartcold13_20261115/public_input.rs");
}
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("../hec_alignment13_20261115/atlas/target_api.rs");
}
#[allow(dead_code)]
mod lp_trace {
    include!("../hec_chartcold13_20261115/lp_trace.rs");
}
#[allow(dead_code)]
mod points {
    include!("point_sparse.rs");
}
#[allow(dead_code)]
mod model {
    include!("../hec_chartcold13_20261115/runtime_program.rs");
    include!("../hec_chartcold13_20261115/recover_model.rs");
}
#[allow(dead_code)]
mod conic {
    include!("../hec_chartcold13_20261115/conic_dynamic.rs");
    include!("descent_api.rs");
}
fn main() {
    let (pts, logs) = points::fresh();
    conic::run(&pts, &logs);
}
