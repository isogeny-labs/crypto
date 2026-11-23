include!("../../../measurement_hook.rs");
fn main() {
    let _observations = cubic_measure_hook::Guard::start();
    if std::env::args().nth(1).as_deref() == Some("panic") {
        panic!("fixture panic");
    }
    println!("{{\"fixture\":\"rust\",\"value\":42}}");
}
