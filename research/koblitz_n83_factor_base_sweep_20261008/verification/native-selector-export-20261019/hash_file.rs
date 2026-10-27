fn main() {
    let path = std::env::args_os().nth(1).expect("file path");
    let bytes = std::fs::read(path).expect("read file");
    println!("{}", blake3::hash(&bytes).to_hex());
}
