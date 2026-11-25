pub fn jacobi_u64(a: u64, n: u64) -> i32 {
    debug_assert!(n & 1 == 1);
    let mut a = a % n;
    let mut n = n;
    let mut t = 1i32;
    while a != 0 {
        let z = a.trailing_zeros();
        a >>= z;
        if z & 1 == 1 && (n & 7 == 3 || n & 7 == 5) {
            t = -t;
        }
        if a < n {
            std::mem::swap(&mut a, &mut n);
            if a & 3 == 3 && n & 3 == 3 {
                t = -t;
            }
        }
        a -= n;
    }
    if n == 1 {
        t
    } else {
        0
    }
}
