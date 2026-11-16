// Deterministic public software fixture construction, not a performance run.
use hashed_ecdl_cpu::paper::Curve;
fn main() {
    let p = 65521u64;
    let mut roots = vec![0u64; p as usize];
    for y in 0..p {
        roots[(y * y % p) as usize] += 1;
    }
    for b in 1..=100 {
        let r = 1
            + (0..p)
                .map(|x| roots[((x * x % p * x + (p - 3) * x + b) % p) as usize])
                .sum::<u64>();
        if let Ok(c) = Curve::new(p, p - 3, b, r) {
            for z in 0..2 * p {
                if let Some(point) = c.decode(z) {
                    println!("p={p} a={} b={b} r={r} P={point:?}", p - 3);
                    return;
                }
            }
        }
    }
    panic!("no fixture");
}
