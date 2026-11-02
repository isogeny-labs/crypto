fn main() {
    use crypto_lib::cryptanalysis::f4_fp;
    let _ = f4_fp::F4Options::new(f4_fp::Ordering::Grevlex, 4);
    println!("ok");
}
