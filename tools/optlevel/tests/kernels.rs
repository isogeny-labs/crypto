//! The Rust calibration kernels compute what they say: every instruction-set
//! variant gives the same result, and the modular kernels agree with a
//! reference written with plain 128-bit arithmetic.
use optlevel::kernels::{self, Limit, Rng, P31, P62};

fn fixed(passes: u64) -> Limit {
    Limit {
        seconds: 0.0,
        passes: Some(passes),
        timer_every_pass: false,
    }
}
fn powmod(mut b: u128, mut e: u128, m: u128) -> u128 {
    let mut r = 1;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

#[test]
fn every_isa_gives_the_same_checksum() {
    let isas = kernels::available_isas();
    assert!(isas.contains(&"scalar"));
    for k in [
        "muladd_u32",
        "muladd_u64",
        "mulwide_u64",
        "modmuladd_p31",
        "modmuladd_p62",
        "horner_p31",
        "xor_u64",
        "popcnt_u64",
        "chase",
        "clock",
    ] {
        // odd sizes exercise the vector loops' remainders
        for n in [1usize, 67, 1000] {
            let sums: Vec<u64> = isas
                .iter()
                .map(|i| kernels::run(k, i, n, fixed(3)).unwrap().checksum)
                .collect();
            assert!(
                sums.windows(2).all(|w| w[0] == w[1]),
                "{k} n={n}: {isas:?} -> {sums:x?}"
            );
        }
    }
    // floating point: the fused variants agree with each other exactly
    #[cfg(target_arch = "x86_64")]
    for k in ["fma_f64", "triad"] {
        let fused: Vec<&&str> = isas.iter().filter(|i| **i != "base").collect();
        let sums: Vec<u64> = fused
            .iter()
            .map(|i| kernels::run(k, i, 999, fixed(3)).unwrap().checksum)
            .collect();
        assert!(sums.windows(2).all(|w| w[0] == w[1]), "{k}: {sums:x?}");
    }
}

#[test]
fn counts_and_protocol_line() {
    let o = kernels::run("modmuladd_p31", "scalar", 100, fixed(7)).unwrap();
    assert_eq!(
        (o.ops, o.elements, o.working_set, o.op_class.as_str()),
        (700, 100, 800, "modmuladd_p31")
    );
    let d: serde_json::Value = serde_json::from_str(&o.json_line()).unwrap();
    assert_eq!(d["schema"], "optlevel.ops/v1");
    assert_eq!(d["ops"], 700);
    assert_eq!(d["bytes_declared"], 8400);
    let h = kernels::run("horner_p31", "scalar", 50, fixed(2)).unwrap();
    assert_eq!((h.op_class.as_str(), h.ops), ("modmuladd_p31", 100));
    let c = kernels::run("chase", "scalar", 64, fixed(2)).unwrap();
    // a single cycle of n nodes: after whole passes the walk is back at node 0
    assert_eq!((c.op_class.as_str(), c.checksum), ("ptr_hop", 0));
    assert_eq!(
        kernels::run("clock", "scalar", 1000, fixed(4))
            .unwrap()
            .checksum,
        5000
    ); // warm-up + 4 passes
    assert!(kernels::run("nope", "scalar", 0, fixed(1)).is_err());
    assert!(kernels::run("clock", "no-such-isa", 0, fixed(1)).is_err());
}

#[test]
fn modmuladd_p31_matches_a_plain_reference() {
    let (n, passes) = (257usize, 4u64);
    let mut rng = Rng::new();
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for _ in 0..n {
        a.push(rng.next() % P31 as u64);
        b.push(rng.next() % P31 as u64);
    }
    // the kernel multiplies in Montgomery form: mont(b, m) = b * m * 2^-32 mod p
    let p = P31 as u128;
    let rinv = powmod(1 << 32, p - 2, p);
    for _ in 0..passes + 1 {
        for i in 0..n {
            a[i] = ((a[i] as u128 + b[i] as u128 * 1234567891u128 % p * rinv) % p) as u64;
        }
    }
    let want: u64 = a.iter().sum();
    for isa in kernels::available_isas() {
        assert_eq!(
            kernels::run("modmuladd_p31", isa, n, fixed(passes))
                .unwrap()
                .checksum,
            want,
            "{isa}"
        );
    }
}

#[test]
fn modmuladd_p62_matches_a_plain_reference() {
    let (n, passes) = (129usize, 3u64);
    let mut rng = Rng::new();
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for _ in 0..n {
        a.push(rng.next() % P62);
        b.push(rng.next() % P62);
    }
    let p = P62 as u128;
    // 2^-64 mod p by Fermat; products are reduced step by step to stay inside u128
    let r = (1u128 << 64) % p;
    let rinv = {
        let (mut acc, mut base, mut e) = (1u128, r, p - 2);
        let mulmod = |x: u128, y: u128| -> u128 {
            let (mut res, mut x, mut y) = (0u128, x % p, y);
            while y > 0 {
                if y & 1 == 1 {
                    res = (res + x) % p;
                }
                x = (x << 1) % p;
                y >>= 1;
            }
            res
        };
        while e > 0 {
            if e & 1 == 1 {
                acc = mulmod(acc, base);
            }
            base = mulmod(base, base);
            e >>= 1;
        }
        acc
    };
    let mulmod = |x: u128, y: u128| -> u128 {
        let (mut res, mut x, mut y) = (0u128, x % p, y);
        while y > 0 {
            if y & 1 == 1 {
                res = (res + x) % p;
            }
            x = (x << 1) % p;
            y >>= 1;
        }
        res
    };
    for _ in 0..passes + 1 {
        for i in 0..n {
            a[i] =
                ((a[i] as u128 + mulmod(mulmod(b[i] as u128, 0x123456789ABCDEF), rinv)) % p) as u64;
        }
    }
    let want = a.iter().fold(0u64, |s, x| s.wrapping_add(*x));
    for isa in kernels::available_isas() {
        assert_eq!(
            kernels::run("modmuladd_p62", isa, n, fixed(passes))
                .unwrap()
                .checksum,
            want,
            "{isa}"
        );
    }
}

#[test]
fn timed_run_reports_a_rate() {
    let o = kernels::run(
        "muladd_u64",
        kernels::available_isas()[0],
        0,
        Limit {
            seconds: 0.02,
            passes: None,
            timer_every_pass: false,
        },
    )
    .unwrap();
    assert!(o.seconds >= 0.02 && o.seconds < 1.0, "{}", o.seconds);
    assert!(
        o.ops as f64 / o.seconds > 1e8,
        "{} ops in {} s",
        o.ops,
        o.seconds
    );
    assert!(o.passes_per_timer_read >= 1);
}
