//! Diagnostic same-target negation-map rho on the public Prime31 fixture.
//! Full setup, walking, collision recovery, and replay are timed.
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde_json::json;
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const R: usize = 64;
const CAP: u64 = 2_000_000;

#[derive(Clone, Copy)]
struct State {
    point: Point,
    a: u64,
    b: u64,
}

fn word(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
#[inline]
fn fadd(x: u64, y: u64) -> u64 {
    let t = x + y;
    if t >= P {
        t - P
    } else {
        t
    }
}
#[inline]
fn fsub(x: u64, y: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        x + P - y
    }
}
#[inline]
fn fmul(x: u64, y: u64) -> u64 {
    let t = x * y;
    let t = (t & P) + (t >> 31);
    let t = (t & P) + (t >> 31);
    if t >= P {
        t - P
    } else {
        t
    }
}
fn finv(mut x: u64) -> u64 {
    assert_ne!(x, 0);
    let mut y = 1;
    let mut e = P - 2;
    while e > 0 {
        if e & 1 != 0 {
            y = fmul(y, x)
        }
        x = fmul(x, x);
        e >>= 1
    }
    y
}
fn add(u: Point, v: Point) -> Point {
    match (u, v) {
        (None, q) | (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let slope = if x1 == x2 {
                if fadd(y1, y2) == 0 {
                    return None;
                }
                let num = fadd(fmul(3, fmul(x1, x1)), PRIME31.a);
                fmul(num, finv(fadd(y1, y1)))
            } else {
                fmul(fsub(y2, y1), finv(fsub(x2, x1)))
            };
            let x3 = fsub(fsub(fmul(slope, slope), x1), x2);
            let y3 = fsub(fmul(slope, fsub(x1, x3)), y1);
            Some((x3, y3))
        }
    }
}
fn nsub(x: u64, y: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        x + N - y
    }
}
fn nadd(x: u64, y: u64) -> u64 {
    let t = x + y;
    if t >= N {
        t - N
    } else {
        t
    }
}
fn nneg(x: u64) -> u64 {
    if x == 0 {
        0
    } else {
        N - x
    }
}
fn nmult(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % N as u128) as u64
}
fn ninv(mut x: u64) -> u64 {
    assert_ne!(x, 0);
    let mut y = 1;
    let mut e = N - 2;
    while e > 0 {
        if e & 1 != 0 {
            y = nmult(y, x)
        }
        x = nmult(x, x);
        e >>= 1
    }
    y
}
fn fold(mut s: State) -> State {
    if let Some((x, y)) = s.point {
        if y > P / 2 {
            s.point = Some((x, P - y));
            s.a = nneg(s.a);
            s.b = nneg(s.b)
        }
    }
    s
}
fn mix(v: u64) -> u64 {
    let mut h = v ^ 0x2545_F491_4F6C_DD1D;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    h
}
fn key(s: State) -> u64 {
    s.point.map_or(0, |(x, _)| x)
}
fn step(mut s: State, jumps: &[State; R], adds: &mut u64) -> State {
    let j0 = ((mix(key(s)) >> 24) as usize) & (R - 1);
    for t in 0..R {
        let j = (j0 + t) & (R - 1);
        let q = jumps[j];
        let raw = State {
            point: add(s.point, q.point),
            a: nadd(s.a, q.a),
            b: nadd(s.b, q.b),
        };
        *adds += 1;
        let flipped = raw.point.is_some_and(|(_, y)| y > P / 2);
        let candidate = fold(raw);
        if !(flipped && t + 1 < R && (((mix(key(candidate)) >> 24) as usize) & (R - 1)) == j) {
            return candidate;
        }
    }
    s = fold(s);
    s
}
fn main() {
    use std::collections::HashMap;
    let seed_arg = std::env::args()
        .nth(1)
        .expect("required rho seed (hex u64)");
    assert_eq!(std::env::args().count(), 2, "one rho seed required");
    let seed = u64::from_str_radix(seed_arg.strip_prefix("0x").expect("0x seed required"), 16)
        .expect("invalid hex u64 seed");
    let cold = Instant::now();
    let mut ar = Arithmetic::default();
    let g = Some(PRIME31.g);
    let mut rng = seed;
    let mut jumps = [State {
        point: None,
        a: 0,
        b: 0,
    }; R];
    let mut setup_adds = 0u64;
    for j in &mut jumps {
        let a = word(&mut rng) % N;
        let b = word(&mut rng) % N;
        *j = State {
            point: add(ar.scalar(PRIME31, a, g), ar.scalar(PRIME31, b, Q)),
            a,
            b,
        };
        setup_adds += 1;
    }
    let mut table: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut walks = 0u64;
    let mut steps = 0u64;
    let mut adds = setup_adds;
    let mut cycles = 0u64;
    let mut dps = 0u64;
    let mut scalar = None;
    'search: for _ in 0..2048 {
        walks += 1;
        let a = word(&mut rng) % N;
        let b = word(&mut rng) % N;
        let mut s = fold(State {
            point: add(ar.scalar(PRIME31, a, g), ar.scalar(PRIME31, b, Q)),
            a,
            b,
        });
        adds += 1;
        let mut recent: Vec<State> = Vec::with_capacity(16);
        for _ in 0..CAP {
            let before = adds;
            s = step(s, &jumps, &mut adds);
            steps += 1;
            if s.point.is_none() {
                break;
            }
            if let Some(i) = recent.iter().position(|v| v.point == s.point) {
                cycles += 1;
                let chosen = recent[i..]
                    .iter()
                    .min_by_key(|v| key(**v))
                    .copied()
                    .unwrap();
                s = fold(State {
                    point: add(chosen.point, chosen.point),
                    a: nadd(chosen.a, chosen.a),
                    b: nadd(chosen.b, chosen.b),
                });
                adds += 1;
                recent.clear();
                if s.point.is_none() {
                    break;
                }
            }
            if recent.len() == 16 {
                recent.remove(0);
            }
            recent.push(s);
            if mix(key(s)) & 127 == 0 {
                dps += 1;
                match table.get(&key(s)) {
                    Some(&(pa, pb)) if pb != s.b => {
                        let d = nmult(nsub(s.a, pa), ninv(nsub(pb, s.b)));
                        if ar.scalar(PRIME31, d, g) == Q {
                            scalar = Some(d);
                            break 'search;
                        }
                    }
                    None => {
                        table.insert(key(s), (s.a, s.b));
                    }
                    _ => {}
                }
                break;
            }
            if adds - before > R as u64 {
                panic!("lookahead exceeded jump count")
            }
        }
    }
    println!(
        "{}",
        json!({"policy":"diagnostic-v5-inspired-64jump-fold-lookahead-cycle16-dp7","seed":seed,"curve":PRIME31,"public_target":Q,"recovered_scalar":scalar,"verified":scalar.is_some(),"cold_wall_ns":cold.elapsed().as_nanos().to_string(),"steps":steps,"group_adds":adds,"walks":walks,"cycles":cycles,"distinguished_points":dps,"stored_distinguished_points":table.len()})
    );
}
