// Independent direct-(x,y) replay of the full F2 genus-seven census.
use std::collections::BTreeMap;

const MODS: [u16; 7] = [3, 7, 11, 19, 37, 67, 131];

fn mul(mut a: u16, mut b: u16, degree: usize, modulus: u16) -> u16 {
    let mut out = 0;
    while b != 0 {
        if b & 1 != 0 { out ^= a; }
        b >>= 1;
        a <<= 1;
        if a & (1 << degree) != 0 { a ^= modulus; }
    }
    out
}

fn value(poly: u32, x: u16, degree: usize, modulus: u16) -> u16 {
    let top = 31 - poly.leading_zeros();
    let mut out = 0;
    for i in (0..=top).rev() { out = mul(out, x, degree, modulus) ^ ((poly >> i) & 1) as u16; }
    out
}

fn polynomial_degree(p: u32) -> u32 { 31 - p.leading_zeros() }

fn divisible(mut p: u32, divisor: u32) -> bool {
    let d = polynomial_degree(divisor);
    while p != 0 && polynomial_degree(p) >= d { p ^= divisor << (polynomial_degree(p) - d); }
    p == 0
}

fn irreducible(p: u32) -> bool {
    let d = polynomial_degree(p);
    for k in 1..=d / 2 {
        for bits in 0..(1 << k) {
            if divisible(p, (1 << k) | bits) { return false; }
        }
    }
    true
}

fn shifted(p: u32) -> u32 {
    let mut out = 0;
    let mut power = 1;
    for i in 0..=polynomial_degree(p) {
        if p & (1 << i) != 0 { out ^= power; }
        power = (power << 1) ^ power; // (x+1)^(i+1)
    }
    out
}

fn points(v: u32, u: u32, degree: usize, modulus: u16) -> u32 {
    let mut n = 1; // ramified infinity
    for x in 0..(1 << degree) {
        let vx = value(v, x, degree, modulus);
        let ux = value(u, x, degree, modulus);
        for y in 0..(1 << degree) {
            if mul(y, y, degree, modulus) ^ mul(vx, y, degree, modulus) == ux { n += 1; }
        }
    }
    n
}

fn derivative(p: u32) -> u32 {
    let mut out = 0;
    for i in (1..=polynomial_degree(p)).step_by(2) {
        if p & (1 << i) != 0 { out ^= 1 << (i - 1); }
    }
    out
}

fn smooth(v: u32, u: u32) -> bool {
    for x in 0..128 {
        if value(v, x, 7, 131) != 0 { continue; }
        let ux = value(u, x, 7, 131);
        let ud = value(derivative(u), x, 7, 131);
        let vd = value(derivative(v), x, 7, 131);
        let expr = mul(ud, ud, 7, 131) ^ mul(mul(vd, vd, 7, 131), ux, 7, 131);
        if expr == 0 { return false; }
    }
    true
}

fn main() {
    for &p in &MODS { assert!(irreducible(p as u32)); }
    let septics: Vec<u32> = (128..256).filter(|&p| irreducible(p)).collect();
    assert_eq!(septics.len(), 18);
    let reps: Vec<u32> = septics.iter().copied().filter(|&p| p < shifted(p)).collect();
    assert_eq!(reps.len(), 9);
    for &p in &septics { assert!(septics.contains(&shifted(p))); assert_ne!(p, shifted(p)); }
    let mut stages = [0u64; 8];
    let mut smooth_after_six = 0u64;
    let mut traces = BTreeMap::<i32, u64>::new();
    let mut witnesses = BTreeMap::<i32, (u32, u32, u32)>::new();
    for &v in &reps {
        for lower in 0..(1u32 << 15) {
            let u = (1 << 15) | lower;
            stages[0] += 1;
            let mut passes = true;
            for degree in 1..=6 {
                if points(v, u, degree, MODS[degree - 1]) != (1 << degree) + 1 { passes = false; break; }
                stages[degree] += 1;
            }
            if !passes || !smooth(v, u) { continue; }
            smooth_after_six += 1;
            let n7 = points(v, u, 7, 131);
            let delta = 129 - n7 as i32;
            if delta % 7 != 0 { continue; }
            let t = delta / 7;
            stages[7] += 1;
            *traces.entry(t).or_default() += 1;
            witnesses.entry(t).or_insert((v, u, n7));
        }
    }
    assert_eq!(stages[0], 294912);
    println!("septic_count={} reps={reps:?}", septics.len());
    println!("stages={stages:?} smooth_after_six={smooth_after_six}");
    println!("traces={traces:?}");
    for (t, (v, u, n7)) in witnesses { println!("witness t={t} v=0x{v:x} u=0x{u:x} N7={n7}"); }
}
