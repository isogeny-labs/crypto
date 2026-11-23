#[allow(dead_code)]
#[path = "../../hec_g4_four_torsion_20261118/field.rs"]
mod field;
use field::{E, F, Z};
use std::collections::BTreeMap;
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn main() {
    let f = F::new(31, vec![17, 0, 30, 0, 5, 0, 6, 0, 1]);
    let raw = include_str!("../../hec_g4_canonical_quadric_20261118/CONTROL.txt");
    let mut planes = BTreeMap::new();
    for line in raw.lines().filter(|s| s.starts_with("PLANE ")) {
        let (a, b) = line.split_once(" form=").unwrap();
        let v: Vec<E> = nums(b)
            .chunks_exact(9)
            .map(|r| r.try_into().unwrap())
            .collect();
        assert_eq!(v.len(), 4);
        assert_eq!(v.iter().find(|&&x| x != Z), Some(&field::scalar(1)));
        assert!(planes.insert(v, nums(a)[0] as usize).is_none());
    }
    let mut domain = [0; 9];
    let mut image = [0; 9];
    let mut pairs = vec![];
    for (v, &a) in &planes {
        let w: Vec<_> = v.iter().map(|&x| f.pow(x, 31)).collect();
        let b = planes[&w];
        pairs.push((a, b));
        let (mut x, mut y) = (256 | a, b);
        for j in 0..9 {
            if x & (1 << j) == 0 {
                continue;
            }
            if domain[j] == 0 {
                domain[j] = x;
                image[j] = y;
                break;
            }
            x ^= domain[j];
            y ^= image[j];
            if x == 0 {
                assert_eq!(y, 0);
            }
        }
    }
    assert!(domain.iter().all(|&x| x != 0));
    let frob = |c: usize| {
        let (mut x, mut y) = (256 | c, 0);
        for j in 0..9 {
            if x & (1 << j) != 0 {
                x ^= domain[j];
                y ^= image[j];
            }
        }
        assert_eq!(x, 0);
        y
    };
    assert!(pairs.iter().all(|&(a, b)| frob(a) == b));
    let mut fixed = vec![];
    for c in 0..256 {
        let n = frob(c);
        assert_eq!(
            planes.values().any(|&x| x == c),
            planes.values().any(|&x| x == n)
        );
        let mut v = c;
        for _ in 0..8 {
            v = frob(v);
        }
        assert_eq!(v, c);
        if n == c {
            assert!(!planes.values().any(|&x| x == c));
            fixed.push(c);
            println!(
                "RATIONAL characteristic={c} half_mask={} section_character={}",
                c & 15,
                c >> 4
            );
        }
    }
    println!(
        "FROBENIUS origin={} columns={:?} fixed={fixed:?}",
        frob(0),
        (0..8).map(|j| frob(1 << j) ^ frob(0)).collect::<Vec<_>>()
    );
}
