// Independent source-curve and W1-presentation replay over F_(2^14).
const D: u32 = 14;
const MODULUS: u32 = 0x4021;
const SIZE: u32 = 1 << D;

fn mul(mut a: u32, mut b: u32) -> u32 {
    let mut out = 0;
    while b != 0 {
        if b & 1 != 0 { out ^= a; }
        b >>= 1;
        a <<= 1;
        if a & SIZE != 0 { a ^= MODULUS; }
    }
    out
}

fn pow(mut a: u32, mut n: u32) -> u32 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 { out = mul(out, a); }
        a = mul(a, a);
        n >>= 1;
    }
    out
}

fn eval_old(mask: u32, w: u32) -> u32 {
    let mut out = 0;
    for i in (0..7).rev() { out = mul(out, w) ^ ((mask >> i) & 1); }
    out
}

fn trace(mut a: u32) -> u32 {
    let mut out = 0;
    for _ in 0..D { out ^= a; a = mul(a, a); }
    assert!(out <= 1);
    out
}

fn kernel_value(a: u32, q: u32, arm: usize) -> u32 {
    let sigma = |x| pow(x, q);
    let s1 = sigma(a);
    let s2 = sigma(s1);
    let s3 = sigma(s2);
    match arm { 1 => a ^ s1 ^ s3, 2 => a ^ s2 ^ s3, _ => unreachable!() }
}

fn count_elliptic(a: u32, b: u32) -> u32 {
    let mut n = 2; // infinity and the unique x=0 point
    for x in 1..SIZE {
        let inv = pow(x, SIZE - 2);
        let term = x ^ a ^ mul(b, mul(inv, inv));
        if trace(term) == 0 { n += 2; }
    }
    n
}

fn main() {
    let w = (0..SIZE).find(|&z| pow(z, 7) ^ z ^ 1 == 0).unwrap();
    assert_eq!(pow(w, 7) ^ w ^ 1, 0);
    let old: Vec<u32> = (0..128).map(|mask| eval_old(mask, w)).collect();
    assert_eq!(old.iter().copied().collect::<std::collections::BTreeSet<_>>().len(), 128);
    let b = pow(w, 3);
    let mut presentation = None;
    for arm in 1..=2 {
        let v: Vec<u32> = (1..128).filter(|&i| kernel_value(old[i as usize], 2, arm) == 0).collect();
        assert_eq!(v.len(), 7);
        for &r_mask in &v {
            for &v_mask in &v {
                let r = old[r_mask as usize];
                let c = old[v_mask as usize];
                if pow(mul(r, c ^ 1), 2) == b {
                    presentation = Some((arm, r_mask, v_mask, r, c));
                    break;
                }
            }
            if presentation.is_some() { break; }
        }
        if presentation.is_some() { break; }
    }
    let (old_arm, r_mask, v_mask, r, v) = presentation.expect("old W1 presentation");
    let new_arm = (1..=2).find(|&arm| kernel_value(r, 4, arm) == 0 && kernel_value(v, 4, arm) == 0).expect("new W1 arm");
    let order_minus = count_elliptic(1, b);
    let order_plus = count_elliptic(0, b);
    assert_eq!(order_minus, 16560);
    assert_eq!(order_plus, 16560);
    println!("modulus=0x{MODULUS:x} embedded_root_w=0x{w:x} embedded_b=0x{b:x}");
    println!("W1 old_arm={old_arm} r_old_mask=0x{r_mask:x} v_old_mask=0x{v_mask:x} r=0x{r:x} v=0x{v:x} new_arm={new_arm}");
    println!("E_minus_order={order_minus} E_plus_order={order_plus} trace=-175");
    println!("PASS q4 source and W1 presentation replay");
}
