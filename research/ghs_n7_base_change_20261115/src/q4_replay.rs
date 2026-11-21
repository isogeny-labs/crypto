// Direct q4 point-count replay of the coprime base-change theorem.
// Counts all y for m<=6 and uses the independently cross-checked trace criterion at m=7.
const V: u32 = 0x8f;
const U_MINUS: u32 = 0xc07a;
const U_PLUS: u32 = 0x802f;

fn degree(p: u32) -> u32 { 31 - p.leading_zeros() }

fn remainder(mut a: u32, b: u32) -> u32 {
    let db = degree(b);
    while a != 0 && degree(a) >= db { a ^= b << (degree(a) - db); }
    a
}

fn irreducible(p: u32) -> bool {
    let d = degree(p);
    for k in 1..=d / 2 {
        for low in 0..(1u32 << k) {
            if remainder(p, (1 << k) | low) == 0 { return false; }
        }
    }
    true
}

fn field_modulus(d: u32) -> u32 {
    let p = ((1 << d) | 1..(1 << (d + 1))).step_by(2).find(|&p| irreducible(p)).unwrap();
    assert!(irreducible(p));
    p
}

fn mul(mut a: u32, mut b: u32, d: u32, modulus: u32) -> u32 {
    let mut out = 0;
    while b != 0 {
        if b & 1 != 0 { out ^= a; }
        b >>= 1;
        a <<= 1;
        if a & (1 << d) != 0 { a ^= modulus; }
    }
    out
}

fn pow(mut a: u32, mut n: u32, d: u32, modulus: u32) -> u32 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 { out = mul(out, a, d, modulus); }
        n >>= 1;
        a = mul(a, a, d, modulus);
    }
    out
}

fn eval(poly: u32, x: u32, d: u32, modulus: u32) -> u32 {
    let mut out = 0;
    for i in (0..=degree(poly)).rev() { out = mul(out, x, d, modulus) ^ ((poly >> i) & 1); }
    out
}

fn trace(mut z: u32, d: u32, modulus: u32) -> u32 {
    let mut out = 0;
    for _ in 0..d { out ^= z; z = mul(z, z, d, modulus); }
    assert!(out <= 1);
    out
}

fn count_direct(u: u32, d: u32, modulus: u32) -> u32 {
    let size = 1 << d;
    let mut count = 1; // one point above infinity
    for x in 0..size {
        let vx = eval(V, x, d, modulus);
        let ux = eval(u, x, d, modulus);
        for y in 0..size {
            if mul(y, y, d, modulus) ^ mul(vx, y, d, modulus) == ux { count += 1; }
        }
    }
    count
}

fn count_trace(u: u32, d: u32, modulus: u32) -> u32 {
    let size = 1 << d;
    let mut count = 1;
    for x in 0..size {
        let vx = eval(V, x, d, modulus);
        let ux = eval(u, x, d, modulus);
        if vx == 0 { count += 1; continue; }
        let inv = pow(vx, size - 2, d, modulus);
        let z = mul(ux, mul(inv, inv, d, modulus), d, modulus);
        if trace(z, d, modulus) == 0 { count += 2; }
    }
    count
}

fn main() {
    let predicted = [5, 17, 65, 257, 1025, 4097, 17610];
    for m in 1..=7 {
        let d = 2 * m;
        let modulus = field_modulus(d);
        let minus = count_trace(U_MINUS, d, modulus);
        let plus = count_trace(U_PLUS, d, modulus);
        if m <= 6 {
            assert_eq!(count_direct(U_MINUS, d, modulus), minus);
            assert_eq!(count_direct(U_PLUS, d, modulus), plus);
        }
        assert_eq!(minus, predicted[(m - 1) as usize]);
        assert_eq!(plus, predicted[(m - 1) as usize]);
        println!("m={m} field_degree={d} modulus=0x{modulus:x} N_minus={minus} N_plus={plus} mode={}", if m <= 6 { "direct+trace" } else { "trace" });
    }
    println!("PASS q4 base-change point-count replay for both twists");
}
