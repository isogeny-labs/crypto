mod degree_driver;
fn main() -> degree_driver::Result<()> {
    degree_driver::run(
        &[5, 6],
        nagao_template::Limits {
            rows: 1536,
            columns: 1024,
            cells: 4_000_000,
            degree: 16,
        },
    )
}
