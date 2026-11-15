// Exhaustive hyperelliptic genus-seven screen over F2. See HYPERELLIPTIC_CENSUS_PROTOCOL.md.
use std::collections::{BTreeMap, BTreeSet};

const MODS: [u32; 7] = [0x3, 0x7, 0xb, 0x13, 0x25, 0x43, 0x83];
const W1_TRACES: [i32; 10] = [-17, -15, -9, -7, -1, 1, 7, 9, 15, 17];

fn degree(x: u64) -> i32 {
    if x == 0 { -1 } else { 63 - x.leading_zeros() as i32 }
}

fn rem(mut a: u64, b: u64) -> u64 {
    let d = degree(b);
    while a != 0 && degree(a) >= d { a ^= b << (degree(a) - d); }
    a
}

fn irreducible(p: u32) -> bool {
    let d = degree(p as u64);
    if d < 1 { return false; }
    for k in 1..=d / 2 {
        for low in 0..(1u32 << k) {
            let divisor = (1u32 << k) | low;
            if rem(p as u64, divisor as u64) == 0 { return false; }
        }
    }
    true
}

fn translate(p: u32) -> u32 {
    let mut out = 0;
    for i in 0..=degree(p as u64) as u32 {
        if (p >> i) & 1 == 0 { continue; }
        for j in 0..=i {
            if (j & !i) == 0 { out ^= 1 << j; }
        }
    }
    out
}

fn mul(a: u32, b: u32, m: u32, modulus: u32) -> u32 {
    let mut z = 0;
    for i in 0..m { if (b >> i) & 1 != 0 { z ^= a << i; } }
    for i in (m..=(2 * m - 2)).rev() {
        if (z >> i) & 1 != 0 { z ^= modulus << (i - m); }
    }
    z
}

fn eval(p: u32, x: u32, m: u32, modulus: u32) -> u32 {
    let mut z = 0;
    for i in (0..=degree(p as u64) as u32).rev() {
        z = mul(z, x, m, modulus) ^ ((p >> i) & 1);
    }
    z
}

fn trace(mut z: u32, m: u32, modulus: u32) -> u32 {
    let mut t = 0;
    for _ in 0..m { t ^= z; z = mul(z, z, m, modulus); }
    assert!(t <= 1);
    t
}

fn derivative(p: u32) -> u64 {
    let mut z = 0;
    for i in (1..=degree(p as u64) as u32).step_by(2) {
        if (p >> i) & 1 != 0 { z ^= 1u64 << (i - 1); }
    }
    z
}

fn square(p: u64) -> u64 {
    if p == 0 { return 0; }
    let mut z = 0;
    for i in 0..=degree(p) as u32 {
        if (p >> i) & 1 != 0 { z ^= 1u64 << (2 * i); }
    }
    z
}

fn poly_mul(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }
    let mut z = 0;
    for i in 0..=degree(b) as u32 {
        if (b >> i) & 1 != 0 { z ^= a << i; }
    }
    z
}

fn smooth(v: u32, u: u32) -> bool {
    let expr = square(derivative(u)) ^ poly_mul(square(derivative(v)), u as u64);
    rem(expr, v as u64) != 0
}

struct FieldScreen { m: u32, modulus: u32, v_eval: Vec<u32>, inv_v_squared: Vec<u32>, traces: Vec<u32> }

impl FieldScreen {
    fn new(m: u32, modulus: u32, v: u32) -> Self {
        let size = 1usize << m;
        let mut inverses = vec![0; size];
        for a in 1..size {
            let b = (1..size).find(|&b| mul(a as u32, b as u32, m, modulus) == 1).unwrap();
            inverses[a] = b as u32;
        }
        let v_eval: Vec<u32> = (0..size).map(|x| eval(v, x as u32, m, modulus)).collect();
        let inv_v_squared = v_eval.iter().map(|&a| {
            if a == 0 { 0 } else { mul(inverses[a as usize], inverses[a as usize], m, modulus) }
        }).collect();
        let traces = (0..size).map(|a| trace(a as u32, m, modulus)).collect();
        Self { m, modulus, v_eval, inv_v_squared, traces }
    }

    fn points(&self, u: u32) -> u32 {
        let mut n = 1; // unique rational point over infinity
        for x in 0..(1u32 << self.m) {
            let y = eval(u, x, self.m, self.modulus);
            if self.v_eval[x as usize] == 0 { n += 1; }
            else {
                let z = mul(y, self.inv_v_squared[x as usize], self.m, self.modulus);
                if self.traces[z as usize] == 0 { n += 2; }
            }
        }
        n
    }
}

fn main() {
    for &p in &MODS { assert!(irreducible(p), "bad field modulus {p:x}"); }
    let septics: Vec<u32> = (128..256).filter(|&p| irreducible(p)).collect();
    assert_eq!(septics.len(), 18);
    let reps: Vec<u32> = septics.iter().copied().filter(|&p| p < translate(p)).collect();
    assert_eq!(reps.len(), 9);
    for &p in &septics { assert!(septics.contains(&translate(p))); assert_ne!(p, translate(p)); }
    println!("field_moduli={MODS:?} irreducible_septics={} support_reps={reps:?}", septics.len());
    let mut stage = [0u64; 8];
    let mut smooth_count = 0u64;
    let mut trace_counts: BTreeMap<i32, u64> = BTreeMap::new();
    let mut witnesses: BTreeMap<i32, (u32, u32, u32)> = BTreeMap::new();
    let mut candidates: BTreeMap<i32, BTreeSet<(u32, u32)>> = BTreeMap::new();
    for &v in &reps {
        let screens: Vec<FieldScreen> = MODS.iter().enumerate().map(|(i, &p)| FieldScreen::new((i + 1) as u32, p, v)).collect();
        for lower in 0..(1u32 << 15) {
            let u = (1u32 << 15) | lower;
            stage[0] += 1;
            let mut passed = true;
            for m in 1..=6 {
                if screens[m - 1].points(u) != (1u32 << m) + 1 { passed = false; break; }
                stage[m] += 1;
            }
            if !passed || !smooth(v, u) { continue; }
            smooth_count += 1;
            let n7 = screens[6].points(u);
            if (129 - n7 as i32) % 7 != 0 { continue; }
            let t = (129 - n7 as i32) / 7;
            stage[7] += 1;
            *trace_counts.entry(t).or_default() += 1;
            witnesses.entry(t).or_insert((v, u, n7));
            candidates.entry(t).or_default().insert((v, u));
            println!("candidate v=0x{v:x} u=0x{u:x} t={t} N7={n7}");
        }
    }
    assert_eq!(stage[0], 9 * (1 << 15));
    println!("stages_total_then_N1_to_N6_then_integral_N7={stage:?} smooth_after_N6={smooth_count}");
    println!("trace_counts={trace_counts:?}");
    for (&t, &(v, u, _)) in &witnesses {
        let orbit: BTreeSet<(u32, u32)> = (0..256u64).map(|r| {
            let shift = square(r) ^ poly_mul(v as u64, r);
            (v, u ^ shift as u32)
        }).collect();
        assert_eq!(orbit.len(), 128, "unexpected y-shift orbit size at trace {t}");
        assert_eq!(candidates[&t], orbit, "candidate set differs from one y-shift orbit at trace {t}");
        println!("orbit t={t} size={} unique_v=0x{v:x}", orbit.len());
    }
    for (t, (v, u, n7)) in &witnesses {
        println!("witness t={t} v=0x{v:x} u=0x{u:x} N7={n7} W1={}", W1_TRACES.contains(t));
    }
    for t in W1_TRACES { println!("W1_trace t={t} count={}", trace_counts.get(&t).copied().unwrap_or(0)); }
}
