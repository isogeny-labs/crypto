fn main() {
    if let Err(error) = twist_relations::tr04::cli(std::env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
