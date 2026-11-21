#[allow(dead_code)]
mod source {
    include!("../hec_alignment13_20261115/atlas/weil.rs");
    include!("../hec_alignment13_20261115/atlas/theta.rs");
    include!("../hec_alignment13_20261115/atlas/coordinates.rs");
    include!("../hec_alignment13_20261115/atlas/source_api.rs");
    include!("../hec_image13_20261115/relations.rs");
    include!("../hec_geometry13_20261115/jets.rs");
    include!("source_jet.rs");
    include!("fiber.rs");
}
#[allow(dead_code)]
mod target {
    include!("../hec_alignment13_20261115/atlas/target.rs");
    include!("target_support.rs");
}
#[allow(dead_code)]
mod check {
    include!("../hec_alignment13_20261115/atlas/target_program.rs");
    include!("check.rs");
}
fn main() {
    source::run_fiber();
}
