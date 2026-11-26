//! Independent unfolded Brent rho control on the exact B677 graph target.
//! This policy is explicit here; it is not the repository tuned WZ implementation.

use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde_json::json;
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const R: usize = 16;
const CAP: u64 = 2_000_000;
const SEEDS: [u64; 3] = [
    0x575a_4652_4f4e_5431,
    0x575a_4652_4f4e_5432,
    0x575a_4652_4f4e_5433,
];

#[derive(Clone, Copy)]
struct State {
    point: Point,
    a: u64,
    b: u64,
}

fn next_word(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.n as u128) as u64
}

fn sub(x: u64, y: u64) -> u64 {
    (x + PRIME31.n - y) % PRIME31.n
}

fn inverse(mut x: u64) -> u64 {
    assert_ne!(x, 0);
    let mut out = 1;
    let mut e = PRIME31.n - 2;
    while e != 0 {
        if e & 1 != 0 {
            out = mul(out, x);
        }
        x = mul(x, x);
        e >>= 1;
    }
    out
}

fn advance(ar: &mut Arithmetic, state: State, jumps: &[State; R]) -> State {
    let index = state.point.map_or(0, |(x, _)| x as usize % R);
    let jump = jumps[index];
    let point = ar.add(PRIME31, state.point, jump.point);
    State {
        point,
        a: (state.a + jump.a) % PRIME31.n,
        b: (state.b + jump.b) % PRIME31.n,
    }
}

fn attempt(seed: u64) -> serde_json::Value {
    let started = Instant::now();
    let mut ar = Arithmetic::default();
    let g = Some(PRIME31.g);
    let mut rng = seed;
    let mut jumps = [State {
        point: None,
        a: 0,
        b: 0,
    }; R];
    for jump in &mut jumps {
        let a = next_word(&mut rng) % PRIME31.n;
        let b = next_word(&mut rng) % PRIME31.n;
        let ag = ar.scalar(PRIME31, a, g);
        let bq = ar.scalar(PRIME31, b, Q);
        let point = ar.add(PRIME31, ag, bq);
        *jump = State { point, a, b };
    }
    let a = next_word(&mut rng) % PRIME31.n;
    let b = next_word(&mut rng) % PRIME31.n;
    let ag = ar.scalar(PRIME31, a, g);
    let bq = ar.scalar(PRIME31, b, Q);
    let initial = ar.add(PRIME31, ag, bq);
    let mut tortoise = State {
        point: initial,
        a,
        b,
    };
    let mut hare = tortoise;
    let mut power = 1_u64;
    let mut lam = 0_u64;
    let mut steps = 0_u64;
    while steps < CAP {
        if power == lam {
            tortoise = hare;
            power = power.saturating_mul(2);
            lam = 0;
        }
        hare = advance(&mut ar, hare, &jumps);
        steps += 1;
        lam += 1;
        if tortoise.point == hare.point {
            let denominator = sub(hare.b, tortoise.b);
            let numerator = sub(tortoise.a, hare.a);
            let recovered = (denominator != 0).then(|| mul(numerator, inverse(denominator)));
            let verified = recovered.is_some_and(|d| ar.scalar(PRIME31, d, g) == Q);
            return json!({
                "seed":seed,"steps":steps,"cycle_length":lam,
                "recovered_scalar":if verified {recovered} else {None},
                "collision_scalar":recovered,"verified":verified,
                "charged_ops":ar.counts,"cold_wall_ns":started.elapsed().as_nanos().to_string(),
            });
        }
    }
    json!({"seed":seed,"steps":steps,"recovered_scalar":null,
        "verified":false,"charged_ops":ar.counts,
        "cold_wall_ns":started.elapsed().as_nanos().to_string()})
}

fn main() {
    let mut ar = Arithmetic::default();
    assert!(ar.on_curve(PRIME31, Q));
    assert_eq!(ar.scalar(PRIME31, PRIME31.n, Some(PRIME31.g)), None);
    let cold = Instant::now();
    let mut attempts = Vec::new();
    for seed in SEEDS {
        let result = attempt(seed);
        let complete = result["verified"] == true;
        attempts.push(result);
        if complete {
            break;
        }
    }
    let total_cold_ns = cold.elapsed().as_nanos().to_string();
    println!(
        "{}",
        json!({"policy":"independent-16-jump-unfolded-Brent",
        "curve":PRIME31,"public_target":Q,"step_cap_per_attempt":CAP,
        "attempts":attempts,"total_cold_ns":total_cold_ns})
    );
}
