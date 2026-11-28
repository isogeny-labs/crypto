//! H114 native producer: multiradix first-layer carry-cohomology.
//!
//! Standalone by design: compile with `rustc -O h114_multiradix_lab.rs`.
//! It uses no Python path and no external Rust crates.  The protocol was
//! published before this implementation and is pinned below by SHA-256.

use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

const PROTOCOL_SHA256: &str = "8d89092331a64a00d51ce7caa1602b035ff582835f8b6f7287912d07590ed51c";
const SECTION_DOMAIN: u64 = 0x4831_3134_5345_4354;
const LABEL_DOMAIN: u64 = 0x4831_3134_4c41_424c;
const MIX_INC: u64 = 0x9e37_79b9_7f4a_7c15;
const MIX_M1: u64 = 0xbf58_476d_1ce4_e5b9;
const MIX_M2: u64 = 0x94d0_49bb_1331_11eb;
const GENERATOR_MULTIPLIERS: [u64; 4] = [1, 2, 3, 5];
const CHANNELS: [&str; 8] = [
    "raw",
    "plus",
    "minus",
    "forward",
    "backward",
    "curvature",
    "anchor_plus",
    "anchor_minus",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Jacobian {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Clone, Copy, Debug)]
struct Curve {
    slug: &'static str,
    icv1: &'static str,
    p: u64,
    a: u64,
    b: u64,
    r: u64,
}

const CURVES: [Curve; 7] = [
    Curve {
        slug: "icv1-fp7-t19-5c8a8fbb",
        icv1: "ICV1:fp-101:19:83:63:unk:unk:r:5c8a8fbb4306",
        p: 101,
        a: 6,
        b: 9,
        r: 83,
    },
    Curve {
        slug: "icv1-fp7-t19-7e6da47e",
        icv1: "ICV1:fp-127:19:109:21:unk:unk:r:7e6da47e5756",
        p: 127,
        a: 1,
        b: 7,
        r: 109,
    },
    Curve {
        slug: "icv1-fp8-t11-972f2836",
        icv1: "ICV1:fp-149:11:139:26:unk:unk:r:972f2836fd54",
        p: 149,
        a: 2,
        b: 8,
        r: 139,
    },
    Curve {
        slug: "icv1-fp8-tm11-e7c21318",
        icv1: "ICV1:fp-211:-11:223:46:unk:unk:r:e7c21318e16c",
        p: 211,
        a: 1,
        b: 1,
        r: 223,
    },
    Curve {
        slug: "icv1-fp9-t3-f53f79d8",
        icv1: "ICV1:fp-283:3:281:267:unk:unk:r:f53f79d850fc",
        p: 283,
        a: 2,
        b: 7,
        r: 281,
    },
    Curve {
        slug: "icv1-fp9-t5-51f1adea",
        icv1: "ICV1:fp-503:5:499:220:unk:unk:r:51f1adea5613",
        p: 503,
        a: 3,
        b: 8,
        r: 499,
    },
    Curve {
        slug: "icv1-fp10-tm39-d6809684",
        icv1: "ICV1:fp-907:-39:947:340:unk:unk:r:d6809684812b",
        p: 907,
        a: 1,
        b: 1,
        r: 947,
    },
];

const ISO_CURVES: [Curve; 2] = [
    Curve {
        slug: "icv1-fp7-t19-227f3bd5",
        icv1: "ICV1:fp-101:19:83:63:unk:unk:r:227f3bd5a83b",
        p: 101,
        a: 96,
        b: 71,
        r: 83,
    },
    Curve {
        slug: "icv1-fp7-t19-28271374",
        icv1: "ICV1:fp-127:19:109:21:unk:unk:r:282713748f5c",
        p: 127,
        a: 16,
        b: 67,
        r: 109,
    },
];

#[derive(Clone, Copy, Debug, Default)]
struct Counts {
    modular_additions: u64,
    modular_multiplications: u64,
    modular_inversions: u64,
    base_additions: u64,
    base_doublings: u64,
    base_scalar_multiplications: u64,
    lifted_additions: u64,
    lifted_doublings: u64,
    lifted_scalar_multiplications: u64,
    kernel_mixed_additions: u64,
    lift_constructions: u64,
    formal_extractions: u64,
    kappa_evaluations: u64,
    winding_evaluations: u64,
    six_path_evaluations: u64,
    braid_checks: u64,
    channel_evaluations: u64,
    crt_evaluations: u64,
}

impl Counts {
    fn merge(&mut self, other: Counts) {
        self.modular_additions += other.modular_additions;
        self.modular_multiplications += other.modular_multiplications;
        self.modular_inversions += other.modular_inversions;
        self.base_additions += other.base_additions;
        self.base_doublings += other.base_doublings;
        self.base_scalar_multiplications += other.base_scalar_multiplications;
        self.lifted_additions += other.lifted_additions;
        self.lifted_doublings += other.lifted_doublings;
        self.lifted_scalar_multiplications += other.lifted_scalar_multiplications;
        self.kernel_mixed_additions += other.kernel_mixed_additions;
        self.lift_constructions += other.lift_constructions;
        self.formal_extractions += other.formal_extractions;
        self.kappa_evaluations += other.kappa_evaluations;
        self.winding_evaluations += other.winding_evaluations;
        self.six_path_evaluations += other.six_path_evaluations;
        self.braid_checks += other.braid_checks;
        self.channel_evaluations += other.channel_evaluations;
        self.crt_evaluations += other.crt_evaluations;
    }
}

fn norm(value: i128, modulus: u64) -> u64 {
    value.rem_euclid(modulus as i128) as u64
}

fn m_add(left: u64, right: u64, modulus: u64, counts: &mut Counts) -> u64 {
    counts.modular_additions += 1;
    ((left as u128 + right as u128) % modulus as u128) as u64
}

fn m_sub(left: u64, right: u64, modulus: u64, counts: &mut Counts) -> u64 {
    counts.modular_additions += 1;
    norm(left as i128 - right as i128, modulus)
}

fn m_mul(left: u64, right: u64, modulus: u64, counts: &mut Counts) -> u64 {
    counts.modular_multiplications += 1;
    ((left as u128 * right as u128) % modulus as u128) as u64
}

fn inv_plain(value: u64, modulus: u64) -> u64 {
    let (mut old_r, mut r) = (modulus as i128, (value % modulus) as i128);
    let (mut old_s, mut s) = (0i128, 1i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    assert_eq!(old_r, 1, "nonunit inverse");
    norm(old_s, modulus)
}

fn m_inv(value: u64, modulus: u64, counts: &mut Counts) -> u64 {
    counts.modular_inversions += 1;
    inv_plain(value, modulus)
}

impl Curve {
    fn on_curve(&self, point: Point) -> bool {
        let p = self.p;
        let x = point.x % p;
        let y = point.y % p;
        let rhs = norm(
            x as i128 * x as i128 * x as i128 + self.a as i128 * x as i128 + self.b as i128,
            p,
        );
        ((y as u128 * y as u128) % p as u128) as u64 == rhs
    }

    fn neg(&self, point: Option<Point>) -> Option<Point> {
        point.map(|q| Point {
            x: q.x,
            y: if q.y == 0 { 0 } else { self.p - q.y },
        })
    }

    fn add(&self, left: Option<Point>, right: Option<Point>, counts: &mut Counts) -> Option<Point> {
        let (left, right) = match (left, right) {
            (None, value) | (value, None) => return value,
            (Some(left), Some(right)) => (left, right),
        };
        if left.x == right.x && (left.y + right.y) % self.p == 0 {
            return None;
        }
        let slope = if left == right {
            counts.base_doublings += 1;
            let x2 = m_mul(left.x, left.x, self.p, counts);
            let numerator = m_add(m_mul(3, x2, self.p, counts), self.a, self.p, counts);
            let denominator = m_mul(2, left.y, self.p, counts);
            m_mul(
                numerator,
                m_inv(denominator, self.p, counts),
                self.p,
                counts,
            )
        } else {
            counts.base_additions += 1;
            let numerator = m_sub(right.y, left.y, self.p, counts);
            let denominator = m_sub(right.x, left.x, self.p, counts);
            m_mul(
                numerator,
                m_inv(denominator, self.p, counts),
                self.p,
                counts,
            )
        };
        let x3 = m_sub(
            m_sub(m_mul(slope, slope, self.p, counts), left.x, self.p, counts),
            right.x,
            self.p,
            counts,
        );
        let y3 = m_sub(
            m_mul(slope, m_sub(left.x, x3, self.p, counts), self.p, counts),
            left.y,
            self.p,
            counts,
        );
        let out = Point { x: x3, y: y3 };
        assert!(self.on_curve(out));
        Some(out)
    }

    fn mul(&self, mut scalar: u64, point: Option<Point>, counts: &mut Counts) -> Option<Point> {
        counts.base_scalar_multiplications += 1;
        let mut out = None;
        let mut addend = point;
        while scalar != 0 {
            if scalar & 1 == 1 {
                out = self.add(out, addend, counts);
            }
            scalar >>= 1;
            if scalar != 0 {
                addend = self.add(addend, addend, counts);
            }
        }
        out
    }

    fn first_point(&self, counts: &mut Counts) -> Point {
        for x in 0..self.p {
            for y in 1..self.p {
                let point = Point { x, y };
                if self.on_curve(point) && self.mul(self.r, Some(point), counts).is_none() {
                    return point;
                }
            }
        }
        panic!("no order-r point on {}", self.slug);
    }

    fn orbit(&self, generator: Point, counts: &mut Counts) -> Vec<Option<Point>> {
        let mut out = Vec::with_capacity(self.r as usize);
        out.push(None);
        let mut current = None;
        for _ in 1..self.r {
            current = self.add(current, Some(generator), counts);
            out.push(current);
        }
        assert!(self.add(current, Some(generator), counts).is_none());
        let mut unique = BTreeSet::new();
        for point in out.iter().flatten() {
            unique.insert((point.x, point.y));
        }
        assert_eq!(unique.len(), (self.r - 1) as usize);
        out
    }
}

fn on_lifted_curve(curve: Curve, point: Point, counts: &mut Counts) -> bool {
    let n = curve.p * curve.p;
    let lhs = m_mul(point.y, point.y, n, counts);
    let x2 = m_mul(point.x, point.x, n, counts);
    let x3 = m_mul(x2, point.x, n, counts);
    let rhs = m_add(
        m_add(x3, m_mul(curve.a, point.x, n, counts), n, counts),
        curve.b,
        n,
        counts,
    );
    lhs == rhs
}

fn hensel_lift(curve: Curve, point: Point, u: u64, counts: &mut Counts) -> Point {
    let p = curve.p;
    let n = p * p;
    let discrepancy = point.x as i128 * point.x as i128 * point.x as i128
        + curve.a as i128 * point.x as i128
        + curve.b as i128
        - point.y as i128 * point.y as i128;
    assert_eq!(discrepancy.rem_euclid(p as i128), 0);
    let cq = norm(discrepancy / p as i128, p);
    let x2 = m_mul(point.x, point.x, p, counts);
    let tangent = m_add(m_mul(3, x2, p, counts), curve.a, p, counts);
    let numerator = m_add(cq, m_mul(tangent, u % p, p, counts), p, counts);
    let v = m_mul(numerator, m_inv(2 * point.y, p, counts), p, counts);
    counts.lift_constructions += 1;
    let lifted = Point {
        x: (point.x + p * (u % p)) % n,
        y: (point.y + p * v) % n,
    };
    assert!(on_lifted_curve(curve, lifted, counts));
    lifted
}

fn lifted_add(
    curve: Curve,
    left: Option<Point>,
    right: Option<Point>,
    counts: &mut Counts,
) -> Option<Point> {
    let (left, right) = match (left, right) {
        (None, value) | (value, None) => return value,
        (Some(left), Some(right)) => (left, right),
    };
    let n = curve.p * curve.p;
    if left.x == right.x && (left.y + right.y) % n == 0 {
        return None;
    }
    let slope = if left == right {
        counts.lifted_doublings += 1;
        let x2 = m_mul(left.x, left.x, n, counts);
        let numerator = m_add(m_mul(3, x2, n, counts), curve.a, n, counts);
        let denominator = m_mul(2, left.y, n, counts);
        assert_ne!(denominator % curve.p, 0);
        m_mul(numerator, m_inv(denominator, n, counts), n, counts)
    } else {
        counts.lifted_additions += 1;
        let numerator = m_sub(right.y, left.y, n, counts);
        let denominator = m_sub(right.x, left.x, n, counts);
        assert_ne!(denominator % curve.p, 0);
        m_mul(numerator, m_inv(denominator, n, counts), n, counts)
    };
    let x3 = m_sub(
        m_sub(m_mul(slope, slope, n, counts), left.x, n, counts),
        right.x,
        n,
        counts,
    );
    let y3 = m_sub(
        m_mul(slope, m_sub(left.x, x3, n, counts), n, counts),
        left.y,
        n,
        counts,
    );
    let out = Point { x: x3, y: y3 };
    assert!(on_lifted_curve(curve, out, counts));
    Some(out)
}

fn lifted_mul(
    curve: Curve,
    mut scalar: u64,
    point: Option<Point>,
    counts: &mut Counts,
) -> Option<Point> {
    counts.lifted_scalar_multiplications += 1;
    let mut out = None;
    let mut addend = point;
    while scalar != 0 {
        if scalar & 1 == 1 {
            out = lifted_add(curve, out, addend, counts);
        }
        scalar >>= 1;
        if scalar != 0 {
            addend = lifted_add(curve, addend, addend, counts);
        }
    }
    out
}

fn on_projective_curve(curve: Curve, point: Jacobian, counts: &mut Counts) -> bool {
    let n = curve.p * curve.p;
    let y2 = m_mul(point.y, point.y, n, counts);
    let x2 = m_mul(point.x, point.x, n, counts);
    let x3 = m_mul(x2, point.x, n, counts);
    let z2 = m_mul(point.z, point.z, n, counts);
    let z4 = m_mul(z2, z2, n, counts);
    let z6 = m_mul(z4, z2, n, counts);
    let rhs = m_add(
        m_add(
            x3,
            m_mul(curve.a, m_mul(point.x, z4, n, counts), n, counts),
            n,
            counts,
        ),
        m_mul(curve.b, z6, n, counts),
        n,
        counts,
    );
    y2 == rhs
}

fn mixed_kernel_sum(curve: Curve, left: Point, right: Point, counts: &mut Counts) -> Jacobian {
    let n = curve.p * curve.p;
    let h = m_sub(right.x, left.x, n, counts);
    let two_h = m_mul(2, h, n, counts);
    let i = m_mul(two_h, two_h, n, counts);
    let j = m_mul(h, i, n, counts);
    let rr = m_mul(2, m_sub(right.y, left.y, n, counts), n, counts);
    let v = m_mul(left.x, i, n, counts);
    let x3 = m_sub(
        m_sub(m_mul(rr, rr, n, counts), j, n, counts),
        m_mul(2, v, n, counts),
        n,
        counts,
    );
    let y3 = m_sub(
        m_mul(rr, m_sub(v, x3, n, counts), n, counts),
        m_mul(2, m_mul(left.y, j, n, counts), n, counts),
        n,
        counts,
    );
    counts.kernel_mixed_additions += 1;
    let out = Jacobian {
        x: x3,
        y: y3,
        z: two_h,
    };
    assert!(on_projective_curve(curve, out, counts));
    out
}

fn formal_tau(curve: Curve, kernel: Jacobian, counts: &mut Counts) -> u64 {
    let p = curve.p;
    let n = p * p;
    assert_eq!(kernel.z % p, 0);
    assert_ne!(kernel.y % p, 0);
    let xz = m_mul(kernel.x, kernel.z, n, counts);
    let t = m_mul(
        norm(-(xz as i128), n),
        m_inv(kernel.y, n, counts),
        n,
        counts,
    );
    assert_eq!(t % p, 0);
    counts.formal_extractions += 1;
    (t / p) % p
}

fn splitmix_once(input: u64) -> u64 {
    let mut z = input.wrapping_add(MIX_INC);
    z = (z ^ (z >> 30)).wrapping_mul(MIX_M1);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_M2);
    z ^ (z >> 31)
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(MIX_INC);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(MIX_M1);
        z = (z ^ (z >> 27)).wrapping_mul(MIX_M2);
        z ^ (z >> 31)
    }
}

enum Section<'a> {
    Canonical,
    Random {
        generator_multiplier: u64,
        replicate: u64,
    },
    Torsion {
        values: &'a HashMap<Point, u64>,
    },
}

fn control_seed(domain: &str, curve: Curve, generator_multiplier: u64, replicate: u64) -> u64 {
    let material = format!(
        "{}\0{}\0{}\0{}",
        domain, curve.slug, generator_multiplier, replicate
    );
    u64::from_be_bytes(sha256(material.as_bytes())[..8].try_into().unwrap())
}

fn section_u(curve: Curve, point: Point, section: &Section<'_>) -> u64 {
    match section {
        Section::Canonical => 0,
        Section::Random {
            generator_multiplier,
            replicate,
        } => {
            let seed = control_seed(
                "H114/random-section/v1",
                curve,
                *generator_multiplier,
                *replicate,
            );
            let input = seed ^ SECTION_DOMAIN ^ point.x.wrapping_shl(32) ^ point.y;
            splitmix_once(input) % curve.p
        }
        Section::Torsion { values } => *values.get(&point).expect("missing torsion section value"),
    }
}

fn carry_for_points(
    curve: Curve,
    root: Point,
    target: Point,
    multiplier: u64,
    section: &Section<'_>,
    counts: &mut Counts,
) -> u64 {
    let lifted_root = hensel_lift(curve, root, section_u(curve, root, section), counts);
    let lifted_target = hensel_lift(curve, target, section_u(curve, target, section), counts);
    let multiplied = lifted_mul(curve, multiplier, Some(lifted_root), counts)
        .expect("small lifted multiple reached identity");
    assert_eq!(
        Point {
            x: multiplied.x % curve.p,
            y: multiplied.y % curve.p,
        },
        target
    );
    let neg_target = Point {
        x: lifted_target.x,
        y: norm(-(lifted_target.y as i128), curve.p * curve.p),
    };
    formal_tau(
        curve,
        mixed_kernel_sum(curve, multiplied, neg_target, counts),
        counts,
    )
}

#[derive(Clone, Copy, Debug, Default)]
struct KRecord {
    k2: u64,
    k3: u64,
    k5: u64,
}

#[derive(Clone, Copy, Debug, Default)]
struct WRecord {
    n23: i64,
    n25: i64,
    n35: i64,
    w23: i64,
    w25: i64,
    w35: i64,
}

#[derive(Clone, Copy, Debug, Default)]
struct PathRecord {
    heights: [i64; 6],
    braid_residual: i64,
}

fn root_index(index: u64, multiplier: u64, order: u64) -> usize {
    (index * inv_plain(multiplier, order) % order) as usize
}

fn kappa_for_point(
    curve: Curve,
    q: Point,
    multiplier: u64,
    section: &Section<'_>,
    counts: &mut Counts,
) -> u64 {
    let root = curve
        .mul(inv_plain(multiplier, curve.r), Some(q), counts)
        .expect("nonzero multiplication root");
    counts.kappa_evaluations += 1;
    carry_for_points(curve, root, q, multiplier, section, counts)
}

fn kappas_at_index(
    curve: Curve,
    points: &[Option<Point>],
    q_index: u64,
    section: &Section<'_>,
    counts: &mut Counts,
) -> KRecord {
    if q_index == 0 {
        return KRecord::default();
    }
    let q = points[q_index as usize].expect("nonzero indexed point");
    KRecord {
        k2: kappa_for_point(curve, q, 2, section, counts),
        k3: kappa_for_point(curve, q, 3, section, counts),
        k5: kappa_for_point(curve, q, 5, section, counts),
    }
}

fn winding_from_kappas(
    curve: Curve,
    kappas: &[KRecord],
    q_index: u64,
    counts: &mut Counts,
) -> WRecord {
    if q_index == 0 {
        return WRecord::default();
    }
    let q = kappas[q_index as usize];
    let h2 = kappas[root_index(q_index, 2, curve.r)];
    let h3 = kappas[root_index(q_index, 3, curve.r)];
    let h5 = kappas[root_index(q_index, 5, curve.r)];
    let n23 = 2 * h2.k3 as i64 + q.k2 as i64 - 3 * h3.k2 as i64 - q.k3 as i64;
    let n25 = 2 * h2.k5 as i64 + q.k2 as i64 - 5 * h5.k2 as i64 - q.k5 as i64;
    let n35 = 3 * h3.k5 as i64 + q.k3 as i64 - 5 * h5.k3 as i64 - q.k5 as i64;
    assert_eq!(n23.rem_euclid(curve.p as i64), 0);
    assert_eq!(n25.rem_euclid(curve.p as i64), 0);
    assert_eq!(n35.rem_euclid(curve.p as i64), 0);
    let out = WRecord {
        n23,
        n25,
        n35,
        w23: n23 / curve.p as i64,
        w25: n25 / curve.p as i64,
        w35: n35 / curve.p as i64,
    };
    assert!((-3..=2).contains(&out.w23));
    assert!((-5..=2).contains(&out.w25));
    assert!((-5..=3).contains(&out.w35));
    counts.winding_evaluations += 3;
    out
}

fn winding_from_points(
    curve: Curve,
    q: Option<Point>,
    section: &Section<'_>,
    counts: &mut Counts,
) -> WRecord {
    let Some(q) = q else {
        return WRecord::default();
    };
    let kq = KRecord {
        k2: kappa_for_point(curve, q, 2, section, counts),
        k3: kappa_for_point(curve, q, 3, section, counts),
        k5: kappa_for_point(curve, q, 5, section, counts),
    };
    let h2 = curve.mul(inv_plain(2, curve.r), Some(q), counts).unwrap();
    let h3 = curve.mul(inv_plain(3, curve.r), Some(q), counts).unwrap();
    let h5 = curve.mul(inv_plain(5, curve.r), Some(q), counts).unwrap();
    let k3_h2 = kappa_for_point(curve, h2, 3, section, counts);
    let k5_h2 = kappa_for_point(curve, h2, 5, section, counts);
    let k2_h3 = kappa_for_point(curve, h3, 2, section, counts);
    let k5_h3 = kappa_for_point(curve, h3, 5, section, counts);
    let k2_h5 = kappa_for_point(curve, h5, 2, section, counts);
    let k3_h5 = kappa_for_point(curve, h5, 3, section, counts);
    let n23 = 2 * k3_h2 as i64 + kq.k2 as i64 - 3 * k2_h3 as i64 - kq.k3 as i64;
    let n25 = 2 * k5_h2 as i64 + kq.k2 as i64 - 5 * k2_h5 as i64 - kq.k5 as i64;
    let n35 = 3 * k5_h3 as i64 + kq.k3 as i64 - 5 * k3_h5 as i64 - kq.k5 as i64;
    let out = WRecord {
        n23,
        n25,
        n35,
        w23: n23 / curve.p as i64,
        w25: n25 / curve.p as i64,
        w35: n35 / curve.p as i64,
    };
    assert_eq!(n23.rem_euclid(curve.p as i64), 0);
    assert_eq!(n25.rem_euclid(curve.p as i64), 0);
    assert_eq!(n35.rem_euclid(curve.p as i64), 0);
    assert!((-3..=2).contains(&out.w23));
    assert!((-5..=2).contains(&out.w25));
    assert!((-5..=3).contains(&out.w35));
    counts.winding_evaluations += 3;
    out
}

fn path_record_at_index(
    curve: Curve,
    points: &[Option<Point>],
    kappas: &[KRecord],
    windings: &[WRecord],
    q_index: u64,
    section: &Section<'_>,
    counts: &mut Counts,
) -> PathRecord {
    if q_index == 0 {
        return PathRecord::default();
    }
    let q = points[q_index as usize].unwrap();
    let kq = kappas[q_index as usize];
    let h2 = kappas[root_index(q_index, 2, curve.r)];
    let h3 = kappas[root_index(q_index, 3, curve.r)];
    let h5 = kappas[root_index(q_index, 5, curve.r)];
    let h6 = kappas[root_index(q_index, 6, curve.r)];
    let h10 = kappas[root_index(q_index, 10, curve.r)];
    let h15 = kappas[root_index(q_index, 15, curve.r)];
    let heights = [
        6 * h6.k5 as i64 + 2 * h2.k3 as i64 + kq.k2 as i64,
        10 * h10.k3 as i64 + 2 * h2.k5 as i64 + kq.k2 as i64,
        6 * h6.k5 as i64 + 3 * h3.k2 as i64 + kq.k3 as i64,
        15 * h15.k2 as i64 + 3 * h3.k5 as i64 + kq.k3 as i64,
        10 * h10.k3 as i64 + 5 * h5.k2 as i64 + kq.k5 as i64,
        15 * h15.k2 as i64 + 5 * h5.k3 as i64 + kq.k5 as i64,
    ];
    for height in &heights[1..] {
        assert_eq!(
            height.rem_euclid(curve.p as i64),
            heights[0].rem_euclid(curve.p as i64)
        );
    }

    let h30_index = root_index(q_index, 30, curve.r);
    let h30 = points[h30_index].unwrap();
    let lifted_h30 = hensel_lift(curve, h30, section_u(curve, h30, section), counts);
    let lifted_q = hensel_lift(curve, q, section_u(curve, q, section), counts);
    let direct = lifted_mul(curve, 30, Some(lifted_h30), counts).unwrap();
    let orders = [
        [2, 3, 5],
        [2, 5, 3],
        [3, 2, 5],
        [3, 5, 2],
        [5, 2, 3],
        [5, 3, 2],
    ];
    for order in orders {
        let mut path = Some(lifted_h30);
        for multiplier in order.into_iter().rev() {
            path = lifted_mul(curve, multiplier, path, counts);
        }
        assert_eq!(path, Some(direct), "six lifted multiplication paths differ");
    }
    assert_eq!(
        Point {
            x: direct.x % curve.p,
            y: direct.y % curve.p
        },
        q
    );
    let neg_q = Point {
        x: lifted_q.x,
        y: norm(-(lifted_q.y as i128), curve.p * curve.p),
    };
    let path_tau = formal_tau(
        curve,
        mixed_kernel_sum(curve, direct, neg_q, counts),
        counts,
    );
    for height in heights {
        assert_eq!(path_tau, height.rem_euclid(curve.p as i64) as u64);
    }

    let p = curve.p as i64;
    let wq = windings[q_index as usize];
    let wq2 = windings[root_index(q_index, 2, curve.r)];
    let wq3 = windings[root_index(q_index, 3, curve.r)];
    let wq5 = windings[root_index(q_index, 5, curve.r)];
    assert_eq!(heights[0] - heights[2], p * wq.w23);
    assert_eq!(heights[2] - heights[3], 3 * p * wq3.w25);
    assert_eq!(heights[3] - heights[5], p * wq.w35);
    assert_eq!(heights[0] - heights[1], 2 * p * wq2.w35);
    assert_eq!(heights[1] - heights[4], p * wq.w25);
    assert_eq!(heights[4] - heights[5], 5 * p * wq5.w23);
    let braid_residual = wq.w23 + 3 * wq3.w25 + wq.w35 - 2 * wq2.w35 - wq.w25 - 5 * wq5.w23;
    assert_eq!(braid_residual, 0);
    counts.six_path_evaluations += 6;
    counts.braid_checks += 1;
    PathRecord {
        heights,
        braid_residual,
    }
}

fn r_multiple_kernel(curve: Curve, lifted: Point, split: u64, counts: &mut Counts) -> Jacobian {
    assert!(split > 0 && split < curve.r);
    let left = lifted_mul(curve, split, Some(lifted), counts).unwrap();
    let right = lifted_mul(curve, curve.r - split, Some(lifted), counts).unwrap();
    let left_red = Point {
        x: left.x % curve.p,
        y: left.y % curve.p,
    };
    let right_red = Point {
        x: right.x % curve.p,
        y: right.y % curve.p,
    };
    assert!(curve.add(Some(left_red), Some(right_red), counts).is_none());
    mixed_kernel_sum(curve, left, right, counts)
}

fn tau_for_section(curve: Curve, point: Point, u: u64, split: u64, counts: &mut Counts) -> u64 {
    let lifted = hensel_lift(curve, point, u, counts);
    formal_tau(
        curve,
        r_multiple_kernel(curve, lifted, split, counts),
        counts,
    )
}

fn torsion_section(
    curve: Curve,
    points: &[Option<Point>],
    counts: &mut Counts,
) -> HashMap<Point, u64> {
    let mut out = HashMap::new();
    for point in points.iter().skip(1).map(|x| x.unwrap()) {
        let split_half = (curve.r - 1) / 2;
        let split_third = std::cmp::max(1, curve.r / 3);
        let tau0 = tau_for_section(curve, point, 0, split_half, counts);
        let tau0_independent = tau_for_section(curve, point, 0, split_third, counts);
        assert_eq!(tau0, tau0_independent);
        let tau1 = tau_for_section(curve, point, 1, split_half, counts);
        let slope = norm(tau1 as i128 - tau0 as i128, curve.p);
        assert_ne!(slope, 0);
        let u = norm(-(tau0 as i128) * inv_plain(slope, curve.p) as i128, curve.p);
        assert_eq!(tau_for_section(curve, point, u, split_half, counts), 0);
        out.insert(point, u);
    }
    out
}

fn all_permutations(size: usize) -> Vec<Vec<u8>> {
    let mut current = (0..size as u8).collect::<Vec<_>>();
    let mut out = vec![current.clone()];
    loop {
        let Some(i) = (0..size - 1).rev().find(|&i| current[i] < current[i + 1]) else {
            break;
        };
        let j = (i + 1..size)
            .rev()
            .find(|&j| current[i] < current[j])
            .unwrap();
        current.swap(i, j);
        current[i + 1..].reverse();
        out.push(current.clone());
    }
    out
}

fn residue(value: i64, modulus: i64) -> u8 {
    value.rem_euclid(modulus) as u8
}

fn channel_values(wq: i64, wp: i64, wm: i64, wg: i64, wng: i64) -> [i64; 8] {
    [
        wq,
        wp,
        wm,
        wp - wq,
        wq - wm,
        wp - 2 * wq + wm,
        wp - wq - wg,
        wm - wq - wng,
    ]
}

fn feature_codes(
    windings: &[WRecord],
    r: u64,
    generator_index: u64,
    scalar: u64,
    counts: &mut Counts,
) -> [u8; 8] {
    let q = scalar * generator_index % r;
    let plus = (q + generator_index) % r;
    let minus = (q + r - generator_index) % r;
    let neg_g = r - generator_index;
    let qrec = windings[q as usize];
    let prec = windings[plus as usize];
    let mrec = windings[minus as usize];
    let grec = windings[generator_index as usize];
    let ngrec = windings[neg_g as usize];
    let c35 = channel_values(qrec.w35, prec.w35, mrec.w35, grec.w35, ngrec.w35);
    let c25 = channel_values(qrec.w25, prec.w25, mrec.w25, grec.w25, ngrec.w25);
    let c23 = channel_values(qrec.w23, prec.w23, mrec.w23, grec.w23, ngrec.w23);
    let mut out = [0u8; 8];
    for channel in 0..8 {
        let z2 = residue(c35[channel], 2);
        let z3 = residue(c25[channel], 3);
        let z5 = residue(c23[channel], 5);
        out[channel] = z2 * 15 + z3 * 5 + z5;
    }
    counts.channel_evaluations += 24;
    out
}

fn crt_digit(z2: u8, z3: u8, z5: u8) -> u8 {
    ((15 * z2 as u16 + 10 * z3 as u16 + 6 * z5 as u16) % 30) as u8
}

fn mapped_digit(code: u8, pi2: &[u8], pi3: &[u8], pi5: &[u8]) -> u8 {
    let z2 = code / 15;
    let remainder = code % 15;
    let z3 = remainder / 5;
    let z5 = remainder % 5;
    crt_digit(pi2[z2 as usize], pi3[z3 as usize], pi5[z5 as usize])
}

fn digits(permutation: &[u8]) -> String {
    permutation
        .iter()
        .map(|digit| char::from(b'0' + *digit))
        .collect()
}

fn predicate_id(channel: usize, pi2: &[u8], pi3: &[u8], pi5: &[u8]) -> String {
    format!(
        "channel:{}:pi2:{}:pi3:{}:pi5:{}",
        CHANNELS[channel],
        digits(pi2),
        digits(pi3),
        digits(pi5)
    )
}

#[derive(Clone, Debug)]
struct RowScore {
    best_matches: u64,
    targets: u64,
    best_ids: Vec<String>,
    exact_ids: Vec<String>,
    contingency: [[[u64; 30]; 30]; 8],
}

fn score_row(
    windings: &[WRecord],
    r: u64,
    generator_index: u64,
    labels: &[u8],
    permutations2: &[Vec<u8>],
    permutations3: &[Vec<u8>],
    permutations5: &[Vec<u8>],
    counts: &mut Counts,
) -> RowScore {
    assert_eq!(labels.len(), (r - 1) as usize);
    let mut contingency = [[[0u64; 30]; 30]; 8];
    for scalar in 1..r {
        let f = feature_codes(windings, r, generator_index, scalar, counts);
        let label = labels[(scalar - 1) as usize] as usize;
        for channel in 0..8 {
            contingency[channel][f[channel] as usize][label] += 1;
        }
    }
    let mut best_matches = 0;
    let mut best_ids = Vec::new();
    let mut exact_ids = Vec::new();
    for channel in 0..8 {
        for pi2 in permutations2 {
            for pi3 in permutations3 {
                for pi5 in permutations5 {
                    let matches: u64 = (0..30)
                        .map(|code| {
                            let digit = mapped_digit(code as u8, pi2, pi3, pi5);
                            contingency[channel][code][digit as usize]
                        })
                        .sum();
                    counts.crt_evaluations += 30;
                    let id = predicate_id(channel, pi2, pi3, pi5);
                    if matches > best_matches {
                        best_matches = matches;
                        best_ids.clear();
                        best_ids.push(id.clone());
                    } else if matches == best_matches {
                        best_ids.push(id.clone());
                    }
                    if matches == r - 1 {
                        exact_ids.push(id);
                    }
                }
            }
        }
    }
    RowScore {
        best_matches,
        targets: r - 1,
        best_ids,
        exact_ids,
        contingency,
    }
}

fn canonical_labels(r: u64) -> Vec<u8> {
    (1..r).map(|scalar| (scalar % 30) as u8).collect()
}

fn shuffled_labels(curve: Curve, generator_multiplier: u64, replicate: u64) -> Vec<u8> {
    let mut labels = canonical_labels(curve.r);
    let mut rng = SplitMix64 {
        state: control_seed(
            "H114/label-shuffle/v1",
            curve,
            generator_multiplier,
            replicate,
        ) ^ LABEL_DOMAIN,
    };
    for i in (1..labels.len()).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        labels.swap(i, j);
    }
    labels
}

fn append_record(
    raw: &mut String,
    kind: &str,
    curve: Curve,
    tag: &str,
    scalar: u64,
    point: Point,
    kappas: KRecord,
    rec: WRecord,
    paths: PathRecord,
) {
    let numeric = [
        kappas.k2 as i64,
        kappas.k3 as i64,
        kappas.k5 as i64,
        rec.n23,
        rec.n25,
        rec.n35,
        rec.w23,
        rec.w25,
        rec.w35,
        paths.heights[0],
        paths.heights[1],
        paths.heights[2],
        paths.heights[3],
        paths.heights[4],
        paths.heights[5],
        paths.braid_residual,
    ]
    .iter()
    .map(i64::to_string)
    .collect::<Vec<_>>()
    .join("\t");
    writeln!(
        raw,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}",
        kind, curve.slug, tag, scalar, point.x, point.y, numeric
    )
    .unwrap();
}

fn append_score(raw: &mut String, kind: &str, curve: Curve, tag: &str, score: &RowScore) {
    let best = if score.best_ids.is_empty() {
        "-".to_string()
    } else {
        score.best_ids.join(",")
    };
    let exact = if score.exact_ids.is_empty() {
        "-".to_string()
    } else {
        score.exact_ids.join(",")
    };
    writeln!(
        raw,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        kind,
        curve.slug,
        tag,
        score.best_matches,
        score.targets,
        score.best_ids.len(),
        best,
        exact
    )
    .unwrap();
    for (channel, matrix) in score.contingency.iter().enumerate() {
        let flattened = matrix
            .iter()
            .flat_map(|row| row.iter())
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        writeln!(
            raw,
            "MATRIX\t{}\t{}\t{}\t{}\t{}",
            kind, curve.slug, tag, CHANNELS[channel], flattened
        )
        .unwrap();
    }
}

fn parse_digits(text: &str, expected: usize) -> Vec<u8> {
    let out = text.bytes().map(|byte| byte - b'0').collect::<Vec<_>>();
    assert_eq!(out.len(), expected);
    let mut sorted = out.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..expected as u8).collect::<Vec<_>>());
    out
}

fn parse_predicate(id: &str) -> (usize, Vec<u8>, Vec<u8>, Vec<u8>) {
    let parts: Vec<&str> = id.split(':').collect();
    assert_eq!(parts.len(), 8);
    assert_eq!(parts[0], "channel");
    assert_eq!(parts[2], "pi2");
    assert_eq!(parts[4], "pi3");
    assert_eq!(parts[6], "pi5");
    let channel = CHANNELS.iter().position(|name| *name == parts[1]).unwrap();
    (
        channel,
        parse_digits(parts[3], 2),
        parse_digits(parts[5], 3),
        parse_digits(parts[7], 5),
    )
}

fn public_digit(
    curve: Curve,
    generator: Point,
    target: Point,
    predicate: &str,
    counts: &mut Counts,
) -> u8 {
    let section = Section::Canonical;
    let plus = curve.add(Some(target), Some(generator), counts);
    let minus = curve.add(Some(target), curve.neg(Some(generator)), counts);
    let wq = winding_from_points(curve, Some(target), &section, counts);
    let wp = winding_from_points(curve, plus, &section, counts);
    let wm = winding_from_points(curve, minus, &section, counts);
    let wg = winding_from_points(curve, Some(generator), &section, counts);
    let wng = winding_from_points(curve, curve.neg(Some(generator)), &section, counts);
    let c35 = channel_values(wq.w35, wp.w35, wm.w35, wg.w35, wng.w35);
    let c25 = channel_values(wq.w25, wp.w25, wm.w25, wg.w25, wng.w25);
    let c23 = channel_values(wq.w23, wp.w23, wm.w23, wg.w23, wng.w23);
    let (channel, pi2, pi3, pi5) = parse_predicate(predicate);
    counts.channel_evaluations += 24;
    counts.crt_evaluations += 1;
    crt_digit(
        pi2[residue(c35[channel], 2) as usize],
        pi3[residue(c25[channel], 3) as usize],
        pi5[residue(c23[channel], 5) as usize],
    )
}

fn inverse_thirty_certificate(curve: Curve) -> (u64, u64) {
    let inv30 = inv_plain(30, curve.r);
    let inverse_product = inv30
        .checked_mul(30)
        .expect("inverse-thirty product overflow");
    assert_eq!(inverse_product % curve.r, 1);
    let inverse_quotient = (inverse_product - 1) / curve.r;
    assert_eq!(inverse_product, 1 + inverse_quotient * curve.r);
    (inv30, inverse_quotient)
}

fn decode_target(
    curve: Curve,
    generator: Point,
    target: Point,
    predicate: &str,
    counts: &mut Counts,
) -> (u64, u64, bool) {
    let (inv30, _) = inverse_thirty_certificate(curve);
    let mut current = Some(target);
    let mut recovered = 0u64;
    let mut place = 1u64;
    let mut queries = 0u64;
    while let Some(point) = current {
        assert!(queries <= 1 + curve.r.ilog(30) as u64);
        let digit = public_digit(curve, generator, point, predicate, counts) as u64;
        let digit_g = curve.mul(digit, Some(generator), counts);
        current = curve.add(current, curve.neg(digit_g), counts);
        current = curve.mul(inv30, current, counts);
        recovered += digit * place;
        place *= 30;
        queries += 1;
    }
    let verified =
        recovered < curve.r && curve.mul(recovered, Some(generator), counts) == Some(target);
    (recovered, queries, verified)
}

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if values.len() % 2 == 1 {
        values[values.len() / 2]
    } else {
        (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.0
    }
}

#[derive(Default)]
struct DecoderAccounting {
    predicate_rows: u64,
    digit_queries: u64,
    point_subtractions: u64,
    radix_divisions: u64,
    final_ec_checks: u64,
}

impl DecoderAccounting {
    fn record(&mut self, queries: u64) {
        self.digit_queries += queries;
        self.point_subtractions += queries;
        self.radix_divisions += queries;
        self.final_ec_checks += 1;
    }
}

#[derive(Default)]
struct RunSummary {
    curve_rows: u64,
    targets: u64,
    rows_with_exact: u64,
    uniform_exact_ids: Vec<String>,
    rows_decoded: u64,
    decoded_targets: u64,
    decoder: DecoderAccounting,
    uniform_rows_decoded: u64,
    uniform_decoded_targets: u64,
    uniform_decoder: DecoderAccounting,
    canonical_best: Vec<f64>,
    random_best: Vec<f64>,
    label_best: Vec<f64>,
    iso_best: Vec<f64>,
    candidate_counts: Counts,
    torsion_counts: Counts,
    conceptual_predicate_evaluations: u64,
    contingency_permutation_evaluations: u64,
}

fn run(quick: bool) -> (String, RunSummary) {
    let permutations2 = all_permutations(2);
    let permutations3 = all_permutations(3);
    let permutations5 = all_permutations(5);
    assert_eq!(
        permutations2.len() * permutations3.len() * permutations5.len(),
        1_440
    );
    let curve_count = if quick { 1 } else { CURVES.len() };
    let multipliers: &[u64] = if quick {
        &GENERATOR_MULTIPLIERS[..1]
    } else {
        &GENERATOR_MULTIPLIERS
    };
    let control_replicates = if quick { 1 } else { 8 };
    let mut raw = String::new();
    writeln!(raw, "H114_RESULT_V1").unwrap();
    writeln!(raw, "PROTOCOL_SHA256\t{}", PROTOCOL_SHA256).unwrap();
    writeln!(raw, "MODE\t{}", if quick { "quick" } else { "full" }).unwrap();
    let mut summary = RunSummary::default();
    let mut uniform_exact: Option<BTreeSet<String>> = None;
    let mut canonical_rows = Vec::new();

    for (curve_index, &curve) in CURVES[..curve_count].iter().enumerate() {
        let mut candidate_counts = Counts::default();
        let base = curve.first_point(&mut candidate_counts);
        let points = curve.orbit(base, &mut candidate_counts);
        writeln!(
            raw,
            "CURVE\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            curve.slug, curve.icv1, curve.p, curve.a, curve.b, curve.r, base.x, base.y
        )
        .unwrap();

        let canonical = Section::Canonical;
        let mut kappas = vec![KRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            kappas[scalar as usize] =
                kappas_at_index(curve, &points, scalar, &canonical, &mut candidate_counts);
        }
        let mut windings = vec![WRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            windings[scalar as usize] =
                winding_from_kappas(curve, &kappas, scalar, &mut candidate_counts);
        }
        let mut paths = vec![PathRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            paths[scalar as usize] = path_record_at_index(
                curve,
                &points,
                &kappas,
                &windings,
                scalar,
                &canonical,
                &mut candidate_counts,
            );
            append_record(
                &mut raw,
                "CANON",
                curve,
                "0",
                scalar,
                points[scalar as usize].unwrap(),
                kappas[scalar as usize],
                windings[scalar as usize],
                paths[scalar as usize],
            );
        }
        let labels = canonical_labels(curve.r);
        for &multiplier in multipliers {
            let score = score_row(
                &windings,
                curve.r,
                multiplier,
                &labels,
                &permutations2,
                &permutations3,
                &permutations5,
                &mut candidate_counts,
            );
            append_score(&mut raw, "ROW", curve, &multiplier.to_string(), &score);
            summary.curve_rows += 1;
            summary.targets += curve.r - 1;
            summary.conceptual_predicate_evaluations += (curve.r - 1) * 11_520;
            summary.contingency_permutation_evaluations += 11_520;
            summary
                .canonical_best
                .push(score.best_matches as f64 / score.targets as f64);
            if !score.exact_ids.is_empty() {
                summary.rows_with_exact += 1;
            }
            let exact: BTreeSet<String> = score.exact_ids.iter().cloned().collect();
            uniform_exact = Some(match uniform_exact.take() {
                None => exact,
                Some(old) => old.intersection(&exact).cloned().collect(),
            });

            if !score.exact_ids.is_empty() {
                let generator = points[multiplier as usize].unwrap();
                summary.rows_decoded += 1;
                for predicate in &score.exact_ids {
                    summary.decoder.predicate_rows += 1;
                    for scalar in 1..curve.r {
                        let target = points[(scalar * multiplier % curve.r) as usize].unwrap();
                        let (recovered, queries, verified) = decode_target(
                            curve,
                            generator,
                            target,
                            predicate,
                            &mut candidate_counts,
                        );
                        assert!(verified && recovered == scalar);
                        summary.decoder.record(queries);
                        summary.decoded_targets += 1;
                    }
                }
            }
            canonical_rows.push((curve, base, multiplier));
        }

        if curve_index < 2 {
            for replicate in 0..control_replicates {
                let section = Section::Random {
                    generator_multiplier: 1,
                    replicate: replicate as u64,
                };
                let mut random_k = vec![KRecord::default(); curve.r as usize];
                for scalar in 1..curve.r {
                    random_k[scalar as usize] =
                        kappas_at_index(curve, &points, scalar, &section, &mut candidate_counts);
                }
                let mut random_w = vec![WRecord::default(); curve.r as usize];
                for scalar in 1..curve.r {
                    random_w[scalar as usize] =
                        winding_from_kappas(curve, &random_k, scalar, &mut candidate_counts);
                }
                let mut random_paths = vec![PathRecord::default(); curve.r as usize];
                for scalar in 1..curve.r {
                    random_paths[scalar as usize] = path_record_at_index(
                        curve,
                        &points,
                        &random_k,
                        &random_w,
                        scalar,
                        &section,
                        &mut candidate_counts,
                    );
                    append_record(
                        &mut raw,
                        "RANDOM",
                        curve,
                        &replicate.to_string(),
                        scalar,
                        points[scalar as usize].unwrap(),
                        random_k[scalar as usize],
                        random_w[scalar as usize],
                        random_paths[scalar as usize],
                    );
                }
                let score = score_row(
                    &random_w,
                    curve.r,
                    1,
                    &labels,
                    &permutations2,
                    &permutations3,
                    &permutations5,
                    &mut candidate_counts,
                );
                append_score(
                    &mut raw,
                    "RANDOM_ROW",
                    curve,
                    &replicate.to_string(),
                    &score,
                );
                summary
                    .random_best
                    .push(score.best_matches as f64 / score.targets as f64);
                summary.conceptual_predicate_evaluations += (curve.r - 1) * 11_520;
                summary.contingency_permutation_evaluations += 11_520;

                let shuffled = shuffled_labels(curve, 1, replicate as u64);
                let label_score = score_row(
                    &windings,
                    curve.r,
                    1,
                    &shuffled,
                    &permutations2,
                    &permutations3,
                    &permutations5,
                    &mut candidate_counts,
                );
                append_score(
                    &mut raw,
                    "LABEL_ROW",
                    curve,
                    &replicate.to_string(),
                    &label_score,
                );
                summary
                    .label_best
                    .push(label_score.best_matches as f64 / label_score.targets as f64);
                summary.conceptual_predicate_evaluations += (curve.r - 1) * 11_520;
                summary.contingency_permutation_evaluations += 11_520;
            }

            let iso = ISO_CURVES[curve_index];
            let lambda = 2u64;
            let iso_generator = Point {
                x: base.x * lambda.pow(2) % curve.p,
                y: base.y * lambda.pow(3) % curve.p,
            };
            assert!(iso.on_curve(iso_generator));
            let iso_points = iso.orbit(iso_generator, &mut candidate_counts);
            let mut iso_k = vec![KRecord::default(); iso.r as usize];
            for scalar in 1..iso.r {
                iso_k[scalar as usize] = kappas_at_index(
                    iso,
                    &iso_points,
                    scalar,
                    &Section::Canonical,
                    &mut candidate_counts,
                );
            }
            let mut iso_w = vec![WRecord::default(); iso.r as usize];
            for scalar in 1..iso.r {
                iso_w[scalar as usize] =
                    winding_from_kappas(iso, &iso_k, scalar, &mut candidate_counts);
            }
            let mut iso_paths = vec![PathRecord::default(); iso.r as usize];
            for scalar in 1..iso.r {
                iso_paths[scalar as usize] = path_record_at_index(
                    iso,
                    &iso_points,
                    &iso_k,
                    &iso_w,
                    scalar,
                    &Section::Canonical,
                    &mut candidate_counts,
                );
                append_record(
                    &mut raw,
                    "ISO",
                    iso,
                    curve.slug,
                    scalar,
                    iso_points[scalar as usize].unwrap(),
                    iso_k[scalar as usize],
                    iso_w[scalar as usize],
                    iso_paths[scalar as usize],
                );
            }
            let iso_score = score_row(
                &iso_w,
                iso.r,
                1,
                &canonical_labels(iso.r),
                &permutations2,
                &permutations3,
                &permutations5,
                &mut candidate_counts,
            );
            append_score(&mut raw, "ISO_ROW", iso, curve.slug, &iso_score);
            summary
                .iso_best
                .push(iso_score.best_matches as f64 / iso_score.targets as f64);
            summary.conceptual_predicate_evaluations += (iso.r - 1) * 11_520;
            summary.contingency_permutation_evaluations += 11_520;
        }

        let mut torsion_counts = Counts::default();
        let torsion_values = torsion_section(curve, &points, &mut torsion_counts);
        let torsion = Section::Torsion {
            values: &torsion_values,
        };
        let mut torsion_k = vec![KRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            torsion_k[scalar as usize] =
                kappas_at_index(curve, &points, scalar, &torsion, &mut torsion_counts);
        }
        let mut torsion_w = vec![WRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            torsion_w[scalar as usize] =
                winding_from_kappas(curve, &torsion_k, scalar, &mut torsion_counts);
        }
        let mut torsion_paths = vec![PathRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            let k = torsion_k[scalar as usize];
            assert_eq!(k.k2, 0);
            assert_eq!(k.k3, 0);
            assert_eq!(k.k5, 0);
            let rec = torsion_w[scalar as usize];
            assert_eq!(rec.w23, 0);
            assert_eq!(rec.w25, 0);
            assert_eq!(rec.w35, 0);
            torsion_paths[scalar as usize] = path_record_at_index(
                curve,
                &points,
                &torsion_k,
                &torsion_w,
                scalar,
                &torsion,
                &mut torsion_counts,
            );
            append_record(
                &mut raw,
                "TORSION",
                curve,
                "0",
                scalar,
                points[scalar as usize].unwrap(),
                k,
                rec,
                torsion_paths[scalar as usize],
            );
        }
        writeln!(
            raw,
            "COUNTS\t{}\t{}\t{}",
            curve.slug,
            counts_json(candidate_counts),
            counts_json(torsion_counts)
        )
        .unwrap();
        summary.candidate_counts.merge(candidate_counts);
        summary.torsion_counts.merge(torsion_counts);
    }
    summary.uniform_exact_ids = uniform_exact.unwrap_or_default().into_iter().collect();
    for &(curve, base, multiplier) in &canonical_rows {
        let generator = curve
            .mul(multiplier, Some(base), &mut summary.candidate_counts)
            .unwrap();
        for predicate in &summary.uniform_exact_ids {
            summary.uniform_rows_decoded += 1;
            summary.uniform_decoder.predicate_rows += 1;
            for scalar in 1..curve.r {
                let target = curve
                    .mul(scalar, Some(generator), &mut summary.candidate_counts)
                    .unwrap();
                let (recovered, queries, verified) = decode_target(
                    curve,
                    generator,
                    target,
                    predicate,
                    &mut summary.candidate_counts,
                );
                assert!(verified && recovered == scalar);
                summary.uniform_decoder.record(queries);
                summary.uniform_decoded_targets += 1;
            }
            writeln!(
                raw,
                "UNIFORM_DECODE\t{}\t{}\t{}\t{}",
                curve.slug,
                multiplier,
                predicate,
                curve.r - 1
            )
            .unwrap();
        }
    }
    writeln!(
        raw,
        "FINAL\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        summary.curve_rows,
        summary.targets,
        summary.rows_with_exact,
        summary.uniform_exact_ids.len(),
        summary.rows_decoded,
        summary.decoded_targets,
        summary.uniform_rows_decoded,
        summary.uniform_decoded_targets
    )
    .unwrap();
    (raw, summary)
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn json_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| json_string(value))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn range(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mut copied = values.to_vec();
    let middle = median(&mut copied);
    (
        values.iter().copied().fold(f64::INFINITY, f64::min),
        middle,
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    )
}

fn counts_json(counts: Counts) -> String {
    format!(
        "{{\"modular_additions\":{},\"modular_multiplications\":{},\"modular_inversions\":{},\"base_additions\":{},\"base_doublings\":{},\"base_scalar_multiplications\":{},\"lifted_additions\":{},\"lifted_doublings\":{},\"lifted_scalar_multiplications\":{},\"kernel_mixed_additions\":{},\"lift_constructions\":{},\"formal_extractions\":{},\"kappa_evaluations\":{},\"winding_evaluations\":{},\"six_path_evaluations\":{},\"braid_checks\":{},\"channel_evaluations\":{},\"crt_evaluations\":{}}}",
        counts.modular_additions,
        counts.modular_multiplications,
        counts.modular_inversions,
        counts.base_additions,
        counts.base_doublings,
        counts.base_scalar_multiplications,
        counts.lifted_additions,
        counts.lifted_doublings,
        counts.lifted_scalar_multiplications,
        counts.kernel_mixed_additions,
        counts.lift_constructions,
        counts.formal_extractions,
        counts.kappa_evaluations,
        counts.winding_evaluations,
        counts.six_path_evaluations,
        counts.braid_checks,
        counts.channel_evaluations,
        counts.crt_evaluations
    )
}

fn decoder_json(accounting: &DecoderAccounting) -> String {
    format!(
        "{{\"predicate_rows\":{},\"digit_queries\":{},\"point_subtractions\":{},\"radix_divisions\":{},\"final_ec_checks\":{},\"hypothetical_online_winding_evaluations\":{}}}",
        accounting.predicate_rows,
        accounting.digit_queries,
        accounting.point_subtractions,
        accounting.radix_divisions,
        accounting.final_ec_checks,
        9 * accounting.digit_queries + 6 * accounting.predicate_rows
    )
}

fn summary_json(
    summary: &RunSummary,
    digest: &str,
    raw_bytes: usize,
    elapsed_ns: u128,
    quick: bool,
) -> String {
    let canonical = range(&summary.canonical_best);
    let random = range(&summary.random_best);
    let labels = range(&summary.label_best);
    let iso = range(&summary.iso_best);
    format!(
        concat!(
            "{{\n",
            "  \"schema\": \"h114.multiradix-carry-cohomology.result/v1\",\n",
            "  \"mode\": \"{}\",\n",
            "  \"protocol_sha256\": \"{}\",\n",
            "  \"scientific_payload_sha256\": \"{}\",\n",
            "  \"raw_bytes\": {},\n",
            "  \"curve_rows\": {},\n",
            "  \"targets\": {},\n",
            "  \"predicate_count\": 11520,\n",
            "  \"rows_with_exact_predicate\": {},\n",
            "  \"uniform_exact_predicate_ids\": {},\n",
            "  \"rows_decoded\": {},\n",
            "  \"decoded_targets\": {},\n",
            "  \"decoder_accounting\": {},\n",
            "  \"uniform_rows_decoded\": {},\n",
            "  \"uniform_decoded_targets\": {},\n",
            "  \"uniform_decoder_accounting\": {},\n",
            "  \"canonical_best_accuracy\": {{\"minimum\": {:.12}, \"median\": {:.12}, \"maximum\": {:.12}}},\n",
            "  \"random_section_best_accuracy\": {{\"minimum\": {:.12}, \"median\": {:.12}, \"maximum\": {:.12}, \"rows\": {}}},\n",
            "  \"matched_label_best_accuracy\": {{\"minimum\": {:.12}, \"median\": {:.12}, \"maximum\": {:.12}, \"rows\": {}}},\n",
            "  \"coordinate_isomorphism_best_accuracy\": {{\"minimum\": {:.12}, \"median\": {:.12}, \"maximum\": {:.12}, \"rows\": {}}},\n",
            "  \"all_structural_checks_passed\": true,\n",
            "  \"all_torsion_carries_zero\": true,\n",
            "  \"conceptual_predicate_evaluations\": {},\n",
            "  \"contingency_permutation_evaluations\": {},\n",
            "  \"candidate_counts\": {},\n",
            "  \"torsion_control_counts\": {},\n",
            "  \"producer_elapsed_ns\": {}\n",
            "}}\n"
        ),
        if quick { "quick" } else { "full" },
        PROTOCOL_SHA256,
        digest,
        raw_bytes,
        summary.curve_rows,
        summary.targets,
        summary.rows_with_exact,
        json_array(&summary.uniform_exact_ids),
        summary.rows_decoded,
        summary.decoded_targets,
        decoder_json(&summary.decoder),
        summary.uniform_rows_decoded,
        summary.uniform_decoded_targets,
        decoder_json(&summary.uniform_decoder),
        canonical.0,
        canonical.1,
        canonical.2,
        random.0,
        random.1,
        random.2,
        summary.random_best.len(),
        labels.0,
        labels.1,
        labels.2,
        summary.label_best.len(),
        iso.0,
        iso.1,
        iso.2,
        summary.iso_best.len(),
        summary.conceptual_predicate_evaluations,
        summary.contingency_permutation_evaluations,
        counts_json(summary.candidate_counts),
        counts_json(summary.torsion_counts),
        elapsed_ns
    )
}

fn sha256(input: &[u8]) -> [u8; 32] {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut data = input.to_vec();
    let bit_len = (data.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    let mut h = H0;
    for block in data.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn self_test() {
    assert_eq!(
        hex(&sha256(b"abc")),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let permutations2 = all_permutations(2);
    let permutations3 = all_permutations(3);
    let permutations5 = all_permutations(5);
    assert_eq!(permutations2.len(), 2);
    assert_eq!(permutations3.len(), 6);
    assert_eq!(permutations5.len(), 120);
    assert_eq!(2 * 6 * 120 * CHANNELS.len(), 11_520);
    let mut ids = BTreeSet::new();
    for channel in 0..CHANNELS.len() {
        for pi2 in &permutations2 {
            for pi3 in &permutations3 {
                for pi5 in &permutations5 {
                    let id = predicate_id(channel, pi2, pi3, pi5);
                    let parsed = parse_predicate(&id);
                    assert_eq!(parsed.0, channel);
                    assert_eq!(parsed.1, *pi2);
                    assert_eq!(parsed.2, *pi3);
                    assert_eq!(parsed.3, *pi5);
                    assert!(ids.insert(id));
                }
            }
        }
    }
    assert_eq!(ids.len(), 11_520);
    let mut counts = Counts::default();
    let curve = CURVES[0];
    assert_eq!(inverse_thirty_certificate(curve), (36, 13));
    let base = curve.first_point(&mut counts);
    let points = curve.orbit(base, &mut counts);
    let mut kappas = vec![KRecord::default(); curve.r as usize];
    for scalar in 1..curve.r {
        kappas[scalar as usize] =
            kappas_at_index(curve, &points, scalar, &Section::Canonical, &mut counts);
    }
    let mut windings = vec![WRecord::default(); curve.r as usize];
    for scalar in 1..curve.r {
        windings[scalar as usize] = winding_from_kappas(curve, &kappas, scalar, &mut counts);
    }
    let rec = windings[1];
    assert_eq!(rec.n23.rem_euclid(curve.p as i64), 0);
    assert_eq!(rec.n25.rem_euclid(curve.p as i64), 0);
    assert_eq!(rec.n35.rem_euclid(curve.p as i64), 0);
    assert!((-3..=2).contains(&rec.w23));
    assert!((-5..=2).contains(&rec.w25));
    assert!((-5..=3).contains(&rec.w35));
    let mut checked_paths = 0;
    for scalar in 1..curve.r {
        let paths = path_record_at_index(
            curve,
            &points,
            &kappas,
            &windings,
            scalar,
            &Section::Canonical,
            &mut counts,
        );
        assert_eq!(paths.braid_residual, 0);
        checked_paths += 1;
    }
    let online = winding_from_points(curve, points[1], &Section::Canonical, &mut counts);
    assert_eq!(online.w23, rec.w23);
    assert_eq!(online.w25, rec.w25);
    assert_eq!(online.w35, rec.w35);
    let torsion_values = torsion_section(curve, &points, &mut counts);
    let torsion = Section::Torsion {
        values: &torsion_values,
    };
    let mut torsion_k = vec![KRecord::default(); curve.r as usize];
    let mut torsion_w = vec![WRecord::default(); curve.r as usize];
    for scalar in 1..curve.r {
        torsion_k[scalar as usize] = kappas_at_index(curve, &points, scalar, &torsion, &mut counts);
        assert_eq!(torsion_k[scalar as usize].k2, 0);
        assert_eq!(torsion_k[scalar as usize].k3, 0);
        assert_eq!(torsion_k[scalar as usize].k5, 0);
    }
    for scalar in 1..curve.r {
        torsion_w[scalar as usize] = winding_from_kappas(curve, &torsion_k, scalar, &mut counts);
        assert_eq!(torsion_w[scalar as usize].w23, 0);
        assert_eq!(torsion_w[scalar as usize].w25, 0);
        assert_eq!(torsion_w[scalar as usize].w35, 0);
    }
    for scalar in 1..curve.r {
        let path = path_record_at_index(
            curve,
            &points,
            &torsion_k,
            &torsion_w,
            scalar,
            &torsion,
            &mut counts,
        );
        assert_eq!(path.heights, [0; 6]);
        assert_eq!(path.braid_residual, 0);
    }
    for z2 in 0..2 {
        for z3 in 0..3 {
            for z5 in 0..5 {
                let digit = crt_digit(z2, z3, z5);
                assert_eq!(digit % 2, z2);
                assert_eq!(digit % 3, z3);
                assert_eq!(digit % 5, z5);
            }
        }
    }
    println!(
        "self-test ok: {} w23={} w25={} w35={} paths={} braid=0",
        curve.slug, rec.w23, rec.w25, rec.w35, checked_paths
    );
}

fn structural_test() {
    let mut total_points = 0u64;
    for curve in CURVES {
        let mut counts = Counts::default();
        let base = curve.first_point(&mut counts);
        let points = curve.orbit(base, &mut counts);
        let mut kappas = vec![KRecord::default(); curve.r as usize];
        let mut windings = vec![WRecord::default(); curve.r as usize];
        for scalar in 1..curve.r {
            kappas[scalar as usize] =
                kappas_at_index(curve, &points, scalar, &Section::Canonical, &mut counts);
        }
        for scalar in 1..curve.r {
            windings[scalar as usize] = winding_from_kappas(curve, &kappas, scalar, &mut counts);
        }
        for scalar in 1..curve.r {
            let path = path_record_at_index(
                curve,
                &points,
                &kappas,
                &windings,
                scalar,
                &Section::Canonical,
                &mut counts,
            );
            assert_eq!(path.braid_residual, 0);
        }
        total_points += curve.r - 1;
        println!(
            "structural-test curve={} points={} kappas={} windings={} paths={} braid_checks={}",
            curve.slug,
            curve.r - 1,
            counts.kappa_evaluations,
            counts.winding_evaluations,
            counts.six_path_evaluations,
            counts.braid_checks
        );
    }
    println!("structural-test ok: curves=7 points={}", total_points);
}

fn main() {
    let mut raw_path = PathBuf::from("h114-winding-raw.tsv");
    let mut summary_path = PathBuf::from("h114-winding-summary.json");
    let mut quick = false;
    let mut self_test_only = false;
    let mut structural_test_only = false;
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--raw" => {
                i += 1;
                raw_path = PathBuf::from(&args[i]);
            }
            "--summary" => {
                i += 1;
                summary_path = PathBuf::from(&args[i]);
            }
            "--quick" => quick = true,
            "--self-test" => self_test_only = true,
            "--structural-test" => structural_test_only = true,
            other => panic!("unknown argument: {}", other),
        }
        i += 1;
    }
    if self_test_only {
        self_test();
        return;
    }
    if structural_test_only {
        structural_test();
        return;
    }
    let started = Instant::now();
    let (raw, summary) = run(quick);
    let digest = hex(&sha256(raw.as_bytes()));
    let summary_text = summary_json(
        &summary,
        &digest,
        raw.len(),
        started.elapsed().as_nanos(),
        quick,
    );
    fs::write(&raw_path, raw.as_bytes()).expect("write raw output");
    fs::write(&summary_path, summary_text.as_bytes()).expect("write summary output");
    println!(
        "{{\"raw\":{},\"summary\":{},\"scientific_payload_sha256\":\"{}\",\"curve_rows\":{},\"targets\":{},\"rows_with_exact_predicate\":{},\"uniform_exact_predicates\":{}}}",
        json_string(&raw_path.display().to_string()),
        json_string(&summary_path.display().to_string()),
        digest,
        summary.curve_rows,
        summary.targets,
        summary.rows_with_exact,
        summary.uniform_exact_ids.len()
    );
}
