#[allow(dead_code)]
mod source {
    include!("../hec_alignment13_20261115/atlas/weil.rs");
    include!("../hec_alignment13_20261115/atlas/theta.rs");
    include!("../hec_alignment13_20261115/atlas/coordinates.rs");
    include!("../hec_alignment13_20261115/atlas/source_api.rs");
    include!("../hec_image13_20261115/relations.rs");
    include!("../hec_geometry13_20261115/jets.rs");
    include!("../hec_calibrationfiber13_20261115/source_jet.rs");
    include!("fiber_select.rs");
}
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("../hec_calibrationfiber13_20261115/target_support.rs");
}
#[allow(dead_code)]
mod mapper {
    include!("../hec_alignment13_20261115/atlas/target_program.rs");
    include!("fit.rs");
}
fn main() {
    mapper::run();
}
