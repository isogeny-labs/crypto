#[allow(dead_code, unused_imports)]
mod small;
fn main() {
    small::run(
        &std::env::args()
            .nth(1)
            .expect("usage: paired-trace-resultant ROUND57_DIRECTORY"),
    );
}
