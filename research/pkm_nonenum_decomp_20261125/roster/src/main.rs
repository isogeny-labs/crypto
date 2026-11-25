//! Frozen, solver-blind curve and target selection for the PKM F4 panel.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::env;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

type Point = (u64, u64);

fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut out = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            out = out * base % modulus;
        }
        base = base * base % modulus;
        exponent >>= 1;
    }
    out
}

fn sub_mod(left: u64, right: u64, modulus: u64) -> u64 {
    (left + modulus - right) % modulus
}

fn add_points(left: Option<Point>, right: Option<Point>, a: u64, p: u64) -> Option<Point> {
    let ((x1, y1), (x2, y2)) = match (left, right) {
        (None, other) | (other, None) => return other,
        (Some(left), Some(right)) => (left, right),
    };
    if x1 == x2 && (y1 + y2) % p == 0 {
        return None;
    }
    let (numerator, denominator) = if x1 == x2 {
        ((3 * x1 * x1 + a) % p, (2 * y1) % p)
    } else {
        (sub_mod(y2, y1, p), sub_mod(x2, x1, p))
    };
    let slope = numerator * pow_mod(denominator, p - 2, p) % p;
    let x3 = sub_mod(sub_mod(slope * slope % p, x1, p), x2, p);
    let y3 = sub_mod(slope * sub_mod(x1, x3, p) % p, y1, p);
    Some((x3, y3))
}

fn multiply(mut scalar: u64, point: Point, a: u64, p: u64) -> Option<Point> {
    let mut result = None;
    let mut step = Some(point);
    while scalar > 0 {
        if scalar & 1 == 1 {
            result = match (result, step) {
                (None, other) => other,
                (other, None) => other,
                (Some(left), Some(right)) => add_points(Some(left), Some(right), a, p),
            };
        }
        step = match step {
            Some(value) => add_points(Some(value), Some(value), a, p),
            None => None,
        };
        scalar >>= 1;
    }
    result
}

fn square_roots(p: u64) -> (Vec<u8>, Vec<Option<u64>>) {
    let mut count = vec![0u8; p as usize];
    let mut first = vec![None; p as usize];
    for y in 0..p {
        let index = (y * y % p) as usize;
        count[index] += 1;
        if first[index].is_none() {
            first[index] = Some(y);
        }
    }
    (count, first)
}

fn curve_order(p: u64, a: u64, b: u64, root_count: &[u8]) -> u64 {
    let mut order = 1;
    for x in 0..p {
        let rhs = (x * x % p * x + a * x + b) % p;
        order += u64::from(root_count[rhs as usize]);
    }
    order
}

fn first_generator(p: u64, a: u64, b: u64, first_root: &[Option<u64>]) -> Point {
    for x in 0..p {
        let rhs = (x * x % p * x + a * x + b) % p;
        if let Some(y) = first_root[rhs as usize] {
            return (x, y);
        }
    }
    panic!("prime-order curve has no affine point")
}

fn bytes_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        use std::fmt::Write;
        write!(&mut out, "{byte:02x}").expect("format digest");
    }
    out
}

fn digest_scalar(t: u32, curve_index: usize, target_index: usize, counter: u32, r: u64) -> (u64, String) {
    let input = format!("pkm-native-f4-v1|{t}|{curve_index}|{target_index}|{counter}");
    let digest = Sha256::digest(input.as_bytes());
    let mut rem = 0;
    for &byte in digest.iter() {
        rem = (rem * 256 + u64::from(byte)) % (r - 1);
    }
    (1 + rem, bytes_hex(&digest))
}

fn relation_seed(t: u32, curve_index: usize, target_index: usize) -> String {
    let input = format!("pkm-native-f4-relations-v1|{t}|{curve_index}|{target_index}");
    bytes_hex(&Sha256::digest(input.as_bytes()))
}

fn select_curve(p: u64) -> (u64, u64, u64, Point, u64, Vec<u8>, Vec<Option<u64>>) {
    let (root_count, first_root) = square_roots(p);
    let mut rejected = 0;
    for a in 1..p {
        for b in 1..p {
            if (4 * a * a % p * a + 27 * b * b) % p == 0 {
                rejected += 1;
                continue;
            }
            let r = curve_order(p, a, b, &root_count);
            if prime(r) && r != p {
                let generator = first_generator(p, a, b, &first_root);
                assert_eq!(multiply(r, generator, a, p), None);
                return (a, b, r, generator, rejected, root_count, first_root);
            }
            rejected += 1;
        }
    }
    panic!("no eligible prime-order curve at p={p}")
}

fn primitive_root(p: u64, b: u64) -> u64 {
    for candidate in 2..p {
        let root = pow_mod(candidate, (p - 1) / b, p);
        if pow_mod(root, b / 2, p) != 1 {
            return root;
        }
    }
    panic!("no primitive root of order {b} modulo {p}")
}

fn factor_base(p: u64, a: u64, b_curve: u64, b_map: u64, zeta: u64,
               first_root: &[Option<u64>]) -> (Vec<u64>, Vec<Point>) {
    let mut xs = Vec::with_capacity(b_map as usize);
    let mut x = 1;
    for _ in 0..b_map {
        xs.push(x);
        x = x * zeta % p;
    }
    xs.sort_unstable();
    xs.dedup();
    assert_eq!(xs.len(), b_map as usize);
    let mut points = Vec::new();
    for &x in &xs {
        let rhs = (x * x % p * x + a * x + b_curve) % p;
        if let Some(y) = first_root[rhs as usize] {
            points.push((x, y));
            if y != 0 {
                points.push((x, p - y));
            }
        }
    }
    points.sort_unstable();
    (xs, points)
}

fn select_target(t: u32, curve_index: usize, target_index: usize,
                 r: u64, generator: Point, a: u64, p: u64,
                 used: &mut HashSet<u64>) -> Value {
    for counter in 0..u32::MAX {
        let (scalar, digest) = digest_scalar(t, curve_index, target_index, counter, r);
        if used.insert(scalar) {
            let point = multiply(scalar, generator, a, p).expect("nonzero scalar on prime group");
            return json!({"target_index":target_index,"counter":counter,
                          "digest_sha256":digest,"scalar":scalar,"point":[point.0,point.1],
                          "relation_seed_sha256":relation_seed(t, curve_index, target_index)});
        }
    }
    panic!("could not select distinct scalar")
}

fn build() -> Value {
    let mut curves = Vec::new();
    for t in 3..=5u32 {
        let b_map = 1u64 << t;
        let p_min = (b_map * b_map * b_map + 5) / 6;
        let mut p = p_min;
        let mut curve_index = 0;
        while curve_index < 3 {
            if p % b_map == 1 && prime(p) {
                let (a, b, r, generator, rejected, _, first_root) = select_curve(p);
                let zeta = primitive_root(p, b_map);
                let (xs, points) = factor_base(p, a, b, b_map, zeta, &first_root);
                let mut used = HashSet::new();
                let targets: Vec<Value> = (0..16)
                    .map(|j| select_target(t, curve_index, j, r, generator, a, p, &mut used))
                    .collect();
                curves.push(json!({"t":t,"curve_index":curve_index,"p_min":p_min,
                                   "p":p,"a":a,"b":b,"r":r,"group_order":r,
                                   "generator":[generator.0,generator.1],
                                   "rejected_pairs":rejected,"v2_p_minus_1":(p-1).trailing_zeros(),
                                   "g":1,"zeta":zeta,"map_degree":b_map,
                                   "kummer_abscissae":xs,"factor_base_points":points,
                                   "targets":targets}));
                curve_index += 1;
            }
            p += 1;
        }
    }
    json!({"schema":"pkm-native-f4-roster/v1",
           "selection":"ROSTER_SELECTION_20261125.md","curve_index_base":0,
           "curves":curves})
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let path = args.next().ok_or("usage: pkm-native-f4-roster OUTPUT.json")?;
    if args.next().is_some() {
        return Err("usage: pkm-native-f4-roster OUTPUT.json".into());
    }
    let roster = build();
    let output = OpenOptions::new().write(true).create_new(true).open(path)?;
    let mut writer = BufWriter::new(output);
    serde_json::to_writer_pretty(&mut writer, &roster)?;
    writer.flush()?;
    writer.get_ref().sync_all()?;
    Ok(())
}
