fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let Err(error) = twist_relations::tr04_protocol::cli(&args) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
