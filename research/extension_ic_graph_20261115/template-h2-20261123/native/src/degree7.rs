mod degree_driver;
fn main() -> degree_driver::Result<()> {
    degree_driver::run(
        &[7],
        nagao_template::Limits {
            rows: 3072,
            columns: 2048,
            cells: 16_000_000,
            degree: 16,
        },
    )
}
