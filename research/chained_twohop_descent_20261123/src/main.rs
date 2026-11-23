//! Finite kernel qualification only. No curves, ECDLP solver, clocks or network.
use std::collections::{BTreeMap, BTreeSet};

type Matrix = [[u8; 4]; 4];
const ROOT_BASIS: [u8; 4] = [0b000011, 0b000110, 0b011000, 0b110000];

fn canon6(x: u8) -> u8 {
    x.min(x ^ 63)
}
fn root_class(v: u8) -> u8 {
    canon6(ROOT_BASIS.iter().enumerate().fold(
        0,
        |a, (i, b)| {
            if v & (1 << i) == 0 { a } else { a ^ b }
        },
    ))
}
fn root_vector(x: u8) -> u8 {
    (0..16)
        .find(|&v| root_class(v) == canon6(x))
        .expect("even root class")
}
fn permute(x: u8, p: &[u8; 6]) -> u8 {
    canon6(p.iter().enumerate().fold(
        0,
        |a, (i, &j)| {
            if x & (1 << i) == 0 { a } else { a | (1 << j) }
        },
    ))
}
fn root_matrix(p: &[u8; 6]) -> Matrix {
    let columns: [u8; 4] = std::array::from_fn(|i| root_vector(permute(ROOT_BASIS[i], p)));
    std::array::from_fn(|i| std::array::from_fn(|j| (columns[j] >> i) & 1))
}
fn apply2(m: &Matrix, v: u8) -> u8 {
    m.iter().enumerate().fold(0, |a, (i, row)| {
        a | ((row
            .iter()
            .enumerate()
            .fold(0, |s, (j, c)| s ^ (c & (v >> j)))
            & 1)
            << i)
    })
}
fn pair2(a: u8, b: u8) -> u8 {
    ((a & (b >> 1)) ^ ((a >> 1) & b) ^ ((a >> 2) & (b >> 3)) ^ ((a >> 3) & (b >> 2))) & 1
}
fn contains(l: u16, v: u8) -> bool {
    l & (1 << v) != 0
}
fn vectors(l: u16) -> Vec<u8> {
    (0..16).filter(|&v| contains(l, v)).collect()
}
fn lagrangians() -> Vec<u16> {
    let mut result = BTreeSet::new();
    for a in 1..16 {
        for b in a + 1..16 {
            if pair2(a, b) == 0 {
                result.insert(1 | (1 << a) | (1 << b) | (1 << (a ^ b)));
            }
        }
    }
    let result: Vec<_> = result.into_iter().collect();
    assert_eq!(result.len(), 15);
    result
}
fn next_permutation(p: &mut [u8; 6]) -> bool {
    let Some(i) = (0..5).rev().find(|&i| p[i] < p[i + 1]) else {
        return false;
    };
    let j = (i + 1..6).rev().find(|&j| p[i] < p[j]).expect("successor");
    p.swap(i, j);
    p[i + 1..].reverse();
    true
}
fn cycle_type(p: &[u8; 6]) -> Vec<u8> {
    let mut seen = [false; 6];
    let mut lengths = Vec::new();
    for start in 0..6 {
        if seen[start] {
            continue;
        }
        let mut x = start;
        let mut n = 0;
        while !seen[x] {
            seen[x] = true;
            n += 1;
            x = usize::from(p[x]);
        }
        lengths.push(n);
    }
    lengths.sort_unstable_by(|a, b| b.cmp(a));
    lengths
}
fn audit_actions() {
    assert_eq!((0..16).map(root_class).collect::<BTreeSet<_>>().len(), 16);
    let mut p = [0, 1, 2, 3, 4, 5];
    let mut matrices = BTreeSet::new();
    let mut census: BTreeMap<Vec<u8>, (u32, u32)> = BTreeMap::new();
    loop {
        let m = root_matrix(&p);
        assert!(matrices.insert(m));
        for a in 0..16 {
            for b in 0..16 {
                assert_eq!(root_class(a ^ b), canon6(root_class(a) ^ root_class(b)));
                assert_eq!(
                    pair2(a, b),
                    ((root_class(a) & root_class(b)).count_ones() & 1) as u8
                );
                assert_eq!(apply2(&m, a), root_vector(permute(root_class(a), &p)));
                assert_eq!(apply2(&m, a ^ b), apply2(&m, a) ^ apply2(&m, b));
                assert_eq!(pair2(apply2(&m, a), apply2(&m, b)), pair2(a, b));
            }
        }
        let fixed = (1..16).filter(|&v| apply2(&m, v) == v).count();
        let mut fixed_pairs = 0;
        for a in 0..6 {
            for b in a + 1..6 {
                if canon6((1 << a) | (1 << b)) == permute((1 << a) | (1 << b), &p) {
                    fixed_pairs += 1;
                }
            }
        }
        assert_eq!(fixed, fixed_pairs);
        let row = census.entry(cycle_type(&p)).or_default();
        row.0 += 1;
        if fixed == 0 {
            row.1 += 1;
        }
        if !next_permutation(&mut p) {
            break;
        }
    }
    assert_eq!(matrices.len(), 720);
    assert_eq!(census.values().map(|x| x.1).sum::<u32>(), 304);
    assert_eq!(census[&vec![3, 3]].1, 40);
    assert_eq!(census[&vec![6]].1, 120);
    assert_eq!(census[&vec![5, 1]].1, 144);
    for (kind, (all, fixed_free)) in &census {
        if *fixed_free != 0 {
            assert!(matches!(kind.as_slice(), [3, 3] | [6] | [5, 1]));
        }
        println!("ACTION|cycle={kind:?}|all={all}|fixed_free={fixed_free}");
    }
    println!("ACTIONS|distinct=720|fixed_free=304|all16_elements_and256_pairs_checked_per_action");
}

// Four C4 coordinates are packed in four two-bit lanes. These are not field elements.
fn add4(a: u8, b: u8) -> u8 {
    (a ^ b) ^ ((a & b & 0x55) << 1)
}
fn double4(x: u8) -> u8 {
    (x & 0x55) << 1
}
fn mod2(x: u8) -> u8 {
    (x & 1) | (((x >> 2) & 1) << 1) | (((x >> 4) & 1) << 2) | (((x >> 6) & 1) << 3)
}
fn embed2(v: u8) -> u8 {
    ((v & 1) << 1) | (((v >> 1) & 1) << 3) | (((v >> 2) & 1) << 5) | (((v >> 3) & 1) << 7)
}
fn torsion_vector(x: u8) -> u8 {
    assert_eq!(x & 0x55, 0);
    mod2(x >> 1)
}
fn coordinates(x: u8) -> [u8; 4] {
    std::array::from_fn(|i| (x >> (2 * i)) & 3)
}
fn pair4(a: u8, b: u8) -> u8 {
    let a = coordinates(a);
    let b = coordinates(b);
    (i16::from(a[0]) * i16::from(b[1]) - i16::from(a[1]) * i16::from(b[0])
        + i16::from(a[2]) * i16::from(b[3])
        - i16::from(a[3]) * i16::from(b[2]))
    .rem_euclid(4) as u8
}
fn apply4(m: &Matrix, x: u8) -> u8 {
    let c = coordinates(x);
    m.iter().enumerate().fold(0, |a, (i, row)| {
        let value = row
            .iter()
            .zip(c)
            .map(|(&a, b)| u16::from(a) * u16::from(b))
            .sum::<u16>()
            & 3;
        a | ((value as u8) << (2 * i))
    })
}
fn similitude(m: &Matrix, mu: u8) -> bool {
    let basis: [u8; 4] = std::array::from_fn(|i| 1 << (2 * i));
    basis.iter().all(|&a| {
        basis
            .iter()
            .all(|&b| pair4(apply4(m, a), apply4(m, b)) == (mu * pair4(a, b)) & 3)
    })
}
fn bitset(points: &[u8]) -> [u64; 4] {
    let mut bits = [0; 4];
    for &x in points {
        bits[usize::from(x >> 6)] |= 1 << (x & 63);
    }
    bits
}
fn bit_contains(bits: &[u64; 4], x: u8) -> bool {
    bits[usize::from(x >> 6)] & (1 << (x & 63)) != 0
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Good,
    Mixed,
    Dual,
}
impl Kind {
    fn label(self) -> &'static str {
        match self {
            Self::Good => "44",
            Self::Mixed => "422",
            Self::Dual => "2222",
        }
    }
    fn index(self) -> usize {
        match self {
            Self::Good => 0,
            Self::Mixed => 1,
            Self::Dual => 2,
        }
    }
}
#[derive(Clone)]
struct Kernel {
    l: u16,
    points: Vec<u8>,
    bits: [u64; 4],
    double: u16,
    two: u16,
    kind: Kind,
    stable: [u32; 4],
}
fn actual_invariants(points: &[u8]) -> (u16, u16) {
    let mut doubled = 0;
    let mut two = 0;
    for &x in points {
        doubled |= 1 << torsion_vector(double4(x));
        if double4(x) == 0 {
            two |= 1 << torsion_vector(x);
        }
    }
    (doubled, two)
}
fn validate(k: &Kernel) -> Result<(), &'static str> {
    if k.points.len() != 16
        || k.bits != bitset(&k.points)
        || k.bits.iter().map(|x| x.count_ones()).sum::<u32>() != 16
    {
        return Err("full kernel encoding");
    }
    if (k.double, k.two) != actual_invariants(&k.points) {
        return Err("doubled/two-torsion invariant");
    }
    if k.two & k.l != k.l {
        return Err("first kernel inclusion");
    }
    for &a in &k.points {
        for &b in &k.points {
            if !bit_contains(&k.bits, add4(a, b)) || pair4(a, b) != 0 {
                return Err("closure/isotropy");
            }
        }
    }
    match k.kind {
        Kind::Good if k.double == k.l && k.two == k.l && k.double.count_ones() == 4 => Ok(()),
        Kind::Mixed if k.double.count_ones() == 2 && k.two.count_ones() == 8 => Ok(()),
        Kind::Dual if k.double == 1 && k.two == u16::MAX => Ok(()),
        _ => Err("group-type invariants"),
    }
}
fn quotient_rep(x: u8, first: &[u8]) -> u8 {
    first
        .iter()
        .map(|&l| add4(x, l))
        .min()
        .expect("nonempty kernel")
}
fn kernels(lags: &[u16]) -> Vec<Kernel> {
    let mut result = Vec::new();
    let mut unique = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
    for &l in lags {
        let first: Vec<_> = vectors(l).into_iter().map(embed2).collect();
        let pre: Vec<_> = (0..=255).filter(|&x| contains(l, mod2(x))).collect();
        assert_eq!(pre.len(), 64);
        let quotient: Vec<_> = pre
            .iter()
            .map(|&x| quotient_rep(x, &first))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(quotient.len(), 16);
        for &a in &quotient {
            for &b in &quotient {
                assert_eq!(pair4(a, b) & 1, 0);
                for &x in &first {
                    assert_eq!(pair4(add4(a, x), b), pair4(a, b));
                }
            }
        }
        let mut planes = BTreeSet::new();
        for (i, &a) in quotient.iter().enumerate().skip(1) {
            for &b in quotient.iter().skip(i + 1) {
                if pair4(a, b) != 0 {
                    continue;
                }
                let mut span = [0, a, b, quotient_rep(add4(a, b), &first)];
                span.sort_unstable();
                assert!(span.windows(2).all(|x| x[0] != x[1]));
                planes.insert(span);
            }
        }
        assert_eq!(planes.len(), 15);
        let mut kinds = [0; 3];
        for span in planes {
            let points: Vec<_> = span
                .iter()
                .flat_map(|&q| first.iter().map(move |&x| add4(q, x)))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let (double, two) = actual_invariants(&points);
            let kind = match double.count_ones() {
                4 => Kind::Good,
                2 => Kind::Mixed,
                1 => Kind::Dual,
                _ => panic!("kernel exponent/type"),
            };
            let k = Kernel {
                l,
                bits: bitset(&points),
                points,
                double,
                two,
                kind,
                stable: [0; 4],
            };
            validate(&k).expect("kernel invariants");
            kinds[kind.index()] += 1;
            unique[kind.index()].insert(k.bits);
            result.push(k);
        }
        assert_eq!(kinds, [8, 6, 1]);
    }
    assert_eq!(result.len(), 225);
    assert_eq!(unique.map(|x| x.len()), [120, 30, 1]);
    println!(
        "KERNELS|first=15|routes=225|per_first=[8,6,1]|unique=[120,30,1]|all_full_groups_and_pairings_checked"
    );
    result
}
fn witness(label: &str, matrix: &Matrix, k: &Kernel) {
    println!(
        "WITNESS|{label}|matrix={matrix:?}|L={:04x}|bits={:016x}{:016x}{:016x}{:016x}|double={:04x}",
        k.l, k.bits[0], k.bits[1], k.bits[2], k.bits[3], k.double
    );
}
fn audit_lifts(records: &mut [Kernel], lags: &[u16]) {
    let permutations = [
        [1, 2, 0, 4, 5, 3],
        [1, 2, 3, 4, 5, 0],
        [1, 2, 3, 4, 0, 5],
        [1, 2, 0, 3, 4, 5],
    ];
    let mut good_witness = false;
    let mut mixed_witness = false;
    for (rep, permutation) in permutations.iter().enumerate() {
        let base = root_matrix(permutation);
        let fixed_free = (1..16).all(|v| apply2(&base, v) != v);
        assert_eq!(fixed_free, rep != 3);
        let stable_l: BTreeMap<_, _> = lags
            .iter()
            .map(|&l| (l, vectors(l).iter().all(|&v| contains(l, apply2(&base, v)))))
            .collect();
        let mut counts = [0_u32; 2];
        let mut stable_routes = [0_u32; 3];
        let mut nonrational = 0_u32;
        for lift in 0_u32..65536 {
            let matrix = std::array::from_fn(|i| {
                std::array::from_fn(|j| base[i][j] + 2 * ((lift >> (4 * i + j)) & 1) as u8)
            });
            for (mi, mu) in [1, 3].into_iter().enumerate() {
                if !similitude(&matrix, mu) {
                    continue;
                }
                counts[mi] += 1;
                let images: [u8; 256] = std::array::from_fn(|x| apply4(&matrix, x as u8));
                for k in records.iter_mut() {
                    if !k
                        .points
                        .iter()
                        .all(|&x| bit_contains(&k.bits, images[usize::from(x)]))
                    {
                        continue;
                    }
                    k.stable[rep] += 1;
                    stable_routes[k.kind.index()] += 1;
                    if k.kind == Kind::Good {
                        assert!(stable_l[&k.l]);
                    }
                    if k.kind == Kind::Mixed {
                        let nonzero = k.double & !1;
                        assert_eq!(nonzero.count_ones(), 1);
                        let p = embed2(nonzero.trailing_zeros() as u8);
                        assert_eq!(images[usize::from(p)], p);
                        assert!(!fixed_free);
                    }
                    if !stable_l[&k.l] {
                        nonrational += 1;
                        if fixed_free {
                            assert_eq!(k.kind, Kind::Dual);
                        }
                    }
                    if fixed_free && k.kind == Kind::Good && !good_witness {
                        witness("stable_good_with_rational_first", &matrix, k);
                        good_witness = true;
                    }
                    if !fixed_free && !stable_l[&k.l] && k.kind == Kind::Mixed && !mixed_witness {
                        witness(
                            "mixed_nonrational_first_with_rational_two_torsion",
                            &matrix,
                            k,
                        );
                        mixed_witness = true;
                    }
                }
            }
        }
        assert_eq!(counts, [1024, 1024]);
        println!(
            "LIFTS|rep={rep}|permutation={permutation:?}|fixed_free={fixed_free}|raw=65536|multipliers={counts:?}|stable_routes={stable_routes:?}|nonrational_first={nonrational}|stable_first={}",
            stable_l.values().filter(|&&x| x).count()
        );
    }
    assert!(good_witness && mixed_witness);
}
fn main() {
    audit_actions();
    let lags = lagrangians();
    let mut records = kernels(&lags);
    let original = records
        .iter()
        .find(|k| k.kind == Kind::Mixed)
        .expect("mixed control")
        .clone();
    let mut mutant = original.clone();
    mutant.double ^= 1;
    let error = validate(&mutant).expect_err("corrupt invariant must fail");
    validate(&original).expect("restored positive");
    println!("CONTROL|corrupt_double_rejected={error}|restored=accepted");
    audit_lifts(&mut records, &lags);
    for (i, k) in records.iter().enumerate() {
        println!(
            "K|route={i:03}|L={:04x}|type={}|double={:04x}|two={:04x}|bits={:016x}{:016x}{:016x}{:016x}|stable={:?}",
            k.l,
            k.kind.label(),
            k.double,
            k.two,
            k.bits[0],
            k.bits[1],
            k.bits[2],
            k.bits[3],
            k.stable
        );
    }
    println!(
        "ACCEPTED|actions=720|fixed_free=304|first=15|routes=225|unique=151|representatives=4|raw_lifts_each=65536|retained_lifts_each=2048|corrupt=1|restored=1|positive_witnesses=2|no_geometric_realization_or_timing|unsigned"
    );
}
