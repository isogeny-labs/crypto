//! Deterministic producer-side oracle for R51--R66.
//!
//! This is deliberately a cyclic-group state machine, not a performance
//! implementation.  It checks the sign bookkeeping that a lazy quotient walk
//! must preserve before the elliptic-curve arm is allowed to exist.

#[derive(Clone, Copy, Debug)]
struct Jump {
    rep: u64,
    a: u64,
    b: u64,
}

#[derive(Clone, Copy, Debug)]
struct Eager {
    rep: u64,
    a: u64,
    b: u64,
}

#[derive(Clone, Copy, Debug)]
struct Lazy {
    rep: u64,
    orient: i8,
    a: u64,
    b: u64,
}

#[derive(Clone, Copy)]
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn add(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + b as u128) % n as u128) as u64
}

fn sub(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 + n as u128 - b as u128) % n as u128) as u64
}

fn mul(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn neg(a: u64, n: u64) -> u64 {
    if a == 0 {
        0
    } else {
        n - a
    }
}

fn signed(a: u64, sign: i8, n: u64) -> u64 {
    match sign {
        1 => a,
        -1 => neg(a, n),
        _ => panic!("invalid sign"),
    }
}

fn inv(a: u64, n: u64) -> Option<u64> {
    if a == 0 {
        return None;
    }
    let (mut old_r, mut r) = (n as i128, a as i128);
    let (mut old_t, mut t) = (0i128, 1i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_t, t) = (t, old_t - q * t);
    }
    if old_r != 1 {
        return None;
    }
    Some(old_t.rem_euclid(n as i128) as u64)
}

fn canonical(p: u64, n: u64) -> (u64, i8) {
    let minus = neg(p, n);
    if p <= minus {
        (p, 1)
    } else {
        (minus, -1)
    }
}

fn mix64(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn partition(h: u64, branches: usize) -> usize {
    if branches.is_power_of_two() {
        (h as usize) & (branches - 1)
    } else {
        (h % branches as u64) as usize
    }
}

fn is_dp(h: u64, bits: u32) -> bool {
    h & ((1u64 << bits) - 1) == 0
}

fn make_eager(a: u64, b: u64, k: u64, n: u64) -> Eager {
    let point = add(a, mul(b, k, n), n);
    let (rep, sign) = canonical(point, n);
    Eager {
        rep,
        a: signed(a, sign, n),
        b: signed(b, sign, n),
    }
}

fn make_jumps(rng: &mut Rng, branches: usize, k: u64, n: u64) -> Vec<Jump> {
    (0..branches)
        .map(|_| {
            let a = rng.next() % n;
            let b = rng.next() % n;
            let e = make_eager(a, b, k, n);
            Jump {
                rep: e.rep,
                a: e.a,
                b: e.b,
            }
        })
        .collect()
}

fn eager_step(state: &mut Eager, jump: Jump, n: u64) {
    let raw = add(state.rep, jump.rep, n);
    state.a = add(state.a, jump.a, n);
    state.b = add(state.b, jump.b, n);
    let (rep, sign) = canonical(raw, n);
    state.rep = rep;
    state.a = signed(state.a, sign, n);
    state.b = signed(state.b, sign, n);
}

fn lazy_step(state: &mut Lazy, jump: Jump, n: u64) {
    // Add the jump with the orientation of the deferred representative.
    state.a = add(state.a, signed(jump.a, state.orient, n), n);
    state.b = add(state.b, signed(jump.b, state.orient, n), n);

    // The quotient transition is computed on canonical representatives.  A
    // canonicalization flip is absorbed into `orient`, not into coefficients.
    let raw = add(state.rep, jump.rep, n);
    let (rep, flip) = canonical(raw, n);
    state.rep = rep;
    state.orient *= flip;
}

fn check_state(eager: Eager, lazy: Lazy, k: u64, n: u64) {
    assert_eq!(eager.rep, lazy.rep);
    assert_eq!(lazy.a, signed(eager.a, lazy.orient, n));
    assert_eq!(lazy.b, signed(eager.b, lazy.orient, n));
    assert_eq!(add(eager.a, mul(eager.b, k, n), n), eager.rep);
    assert_eq!(
        add(lazy.a, mul(lazy.b, k, n), n),
        signed(lazy.rep, lazy.orient, n)
    );
}

fn solve_quotient_collision(
    n: u64,
    s1: i8,
    a1: u64,
    b1: u64,
    s2: i8,
    a2: u64,
    b2: u64,
) -> Option<u64> {
    let numerator = sub(signed(a2, s2, n), signed(a1, s1, n), n);
    let denominator = sub(signed(b1, s1, n), signed(b2, s2, n), n);
    inv(denominator, n).map(|d| mul(numerator, d, n))
}

fn trajectory_oracle() -> (u64, u64, u64) {
    let moduli = [1019u64, 1237, 2017, 4093];
    let branches = [16usize, 32, 64, 128];
    let mut transition_checks = 0u64;
    let mut dp_checks = 0u64;
    let mut cycle_events = 0u64;

    for &n in &moduli {
        for seed in 1..=32u64 {
            let k = (17 * seed + 41) % n;
            for &r in &branches {
                let mut rng = Rng(0x7268_6f5f_6c61_7a79 ^ n ^ seed ^ r as u64);
                let jumps = make_jumps(&mut rng, r, k, n);
                let mut eager = make_eager(rng.next() % n, rng.next() % n, k, n);
                let initial_orient = if seed & 1 == 0 { 1 } else { -1 };
                let mut lazy = Lazy {
                    rep: eager.rep,
                    orient: initial_orient,
                    a: signed(eager.a, initial_orient, n),
                    b: signed(eager.b, initial_orient, n),
                };
                let mut eager_seen = vec![None; n as usize];
                let mut lazy_seen = vec![None; n as usize];

                for step in 0..2048u32 {
                    check_state(eager, lazy, k, n);
                    let h_eager = mix64(eager.rep);
                    let h_lazy = mix64(lazy.rep);
                    assert_eq!(h_eager, h_lazy);
                    assert_eq!(partition(h_eager, r), partition(h_lazy, r));
                    assert_eq!(is_dp(h_eager, 7), is_dp(h_lazy, 7));
                    dp_checks += 1;

                    let e_prev = eager_seen[eager.rep as usize].replace(step);
                    let l_prev = lazy_seen[lazy.rep as usize].replace(step);
                    assert_eq!(e_prev, l_prev);
                    if e_prev.is_some() {
                        cycle_events += 1;
                    }

                    let j = partition(h_eager, r);
                    eager_step(&mut eager, jumps[j], n);
                    lazy_step(&mut lazy, jumps[j], n);
                    transition_checks += 1;
                }
            }
        }
    }
    (transition_checks, dp_checks, cycle_events)
}

fn collision_oracle() -> u64 {
    let moduli = [1019u64, 1237, 2017, 4093];
    let mut rng = Rng(0x636f_6c6c_6973_696f);
    let mut solved = 0u64;
    for &n in &moduli {
        let k = (rng.next() % (n - 1)) + 1;
        for i in 0..20_000u64 {
            let rep = (rng.next() % ((n - 1) / 2)) + 1;
            let s1 = if i & 1 == 0 { 1 } else { -1 };
            let s2 = if i & 2 == 0 { 1 } else { -1 };
            let b1 = rng.next() % n;
            let b2 = rng.next() % n;
            let a1 = sub(signed(rep, s1, n), mul(b1, k, n), n);
            let a2 = sub(signed(rep, s2, n), mul(b2, k, n), n);
            if let Some(found) = solve_quotient_collision(n, s1, a1, b1, s2, a2, b2) {
                assert_eq!(found, k);
                solved += 1;
            }
        }
    }
    solved
}

fn partition_oracle() -> u64 {
    let mut checked = 0u64;
    for r in [16usize, 32, 64, 128] {
        for h in 0..100_000u64 {
            assert_eq!(partition(h, r), (h % r as u64) as usize);
            checked += 1;
        }
    }
    for r in [3usize, 17, 31, 63, 127] {
        for h in 0..100_000u64 {
            assert_eq!(partition(h, r), (h % r as u64) as usize);
            checked += 1;
        }
    }
    checked
}

fn main() {
    let partition_checks = partition_oracle();
    let (transition_checks, dp_checks, cycle_events) = trajectory_oracle();
    let collision_solves = collision_oracle();
    println!(
        "R51-R66 deterministic oracle PASS: partition_checks={partition_checks} \
         transition_checks={transition_checks} dp_checks={dp_checks} \
         repeated_quotient_states={cycle_events} collision_solves={collision_solves}"
    );
}
