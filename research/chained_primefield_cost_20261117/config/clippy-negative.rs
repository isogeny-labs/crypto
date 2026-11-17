fn main() {
    let a = 1_u64;
    std::hint::black_box(a == a);
}
