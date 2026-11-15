use hec_native_ic::{fp::Fp, poly::Poly};
use std::{collections::HashSet, fs, path::Path};
type Space = Vec<u16>;
fn pair(a: u16, b: u16) -> bool {
    ((a & b).count_ones() + (a.count_ones() % 2) * (b.count_ones() % 2)) % 2 != 0
}
fn basis(rows: impl IntoIterator<Item = u16>, w: usize) -> Space {
    let mut r = [0u16; 8];
    for mut v in rows {
        for (i, &x) in r[..w].iter().enumerate() {
            if v >> i & 1 != 0 {
                v ^= x;
            }
        }
        if v == 0 {
            continue;
        }
        let p = v.trailing_zeros() as usize;
        for x in &mut r[..w] {
            if *x >> p & 1 != 0 {
                *x ^= v;
            }
        }
        r[p] = v;
    }
    r[..w].iter().copied().filter(|&x| x != 0).collect()
}
fn contains(s: &Space, mut v: u16) -> bool {
    for &r in s {
        if v >> r.trailing_zeros() & 1 != 0 {
            v ^= r;
        }
    }
    v == 0
}
fn lagrangians(g: usize) -> Vec<Space> {
    let w = 2 * g;
    let mut spaces = HashSet::from([Vec::new()]);
    for rank in 0..g {
        let mut next = HashSet::new();
        for s in &spaces {
            for v in 1..1u16 << w {
                if s.iter().all(|&r| !pair(r, v)) && !contains(s, v) {
                    let b = basis(s.iter().copied().chain([v]), w);
                    assert_eq!(b.len(), rank + 1);
                    next.insert(b);
                }
            }
        }
        spaces = next;
    }
    let mut out: Vec<_> = spaces.into_iter().collect();
    out.sort();
    out
}
fn from_full(mut v: u16, g: usize) -> u16 {
    let n = 2 * g + 2;
    assert_eq!(v.count_ones() % 2, 0);
    if v >> (n - 1) & 1 != 0 {
        v ^= (1 << n) - 1;
    }
    v & ((1 << (2 * g)) - 1)
}
fn act(v: u16, perm: &[usize], g: usize) -> u16 {
    let full = v | ((v.count_ones() as u16 % 2) << (2 * g));
    let mut out = 0;
    for (i, &j) in perm.iter().enumerate() {
        if full >> i & 1 != 0 {
            out |= 1 << j;
        }
    }
    from_full(out, g)
}
fn permutation(cycles: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut k = 0;
    for &n in cycles {
        for i in 0..n {
            out.push(k + (i + 1) % n);
        }
        k += n;
    }
    out
}
fn stable(s: &Space, perm: &[usize], g: usize) -> bool {
    s.iter().all(|&v| contains(s, act(v, perm, g)))
}
fn partitions(left: &[usize], rows: &mut Vec<u16>, g: usize, out: &mut Vec<Space>) {
    if left.is_empty() {
        out.push(basis(rows.iter().copied(), 2 * g));
        return;
    }
    for i in 1..left.len() {
        let rest: Vec<_> = left[1..]
            .iter()
            .copied()
            .filter(|&v| v != left[i])
            .collect();
        rows.push(from_full((1 << left[0]) | (1 << left[i]), g));
        partitions(&rest, rows, g, out);
        rows.pop();
    }
}
fn partition_sets(g: usize) -> Vec<Space> {
    let mut out = Vec::new();
    partitions(
        &(0..2 * g + 2).collect::<Vec<_>>(),
        &mut Vec::new(),
        g,
        &mut out,
    );
    let n = out.len();
    out.sort();
    out.dedup();
    assert_eq!(n, out.len());
    out
}
fn report(
    cycles: &[usize],
    g: usize,
    all: &[Space],
    parts: &[Space],
    label: &str,
) -> (usize, usize, usize) {
    let perm = permutation(cycles);
    assert_eq!(perm.len(), 2 * g + 2);
    for i in 0..2 * g {
        for j in 0..2 * g {
            assert_eq!(
                pair(1 << i, 1 << j),
                pair(act(1 << i, &perm, g), act(1 << j, &perm, g))
            );
        }
    }
    let full: Vec<_> = all.iter().filter(|s| stable(s, &perm, g)).collect();
    let paired: Vec<_> = parts.iter().filter(|s| stable(s, &perm, g)).collect();
    // Independent partition test: every weight-two class in a paired kernel
    // maps to another weight-two class belonging to that same paired kernel.
    for s in parts {
        let mut pairs = Vec::new();
        for i in 0..2 * g + 2 {
            for j in i + 1..2 * g + 2 {
                if contains(s, from_full((1 << i) | (1 << j), g)) {
                    pairs.push((i, j));
                }
            }
        }
        // g=1 has complementary pairs representing identical classes; still
        // test set invariance on the full list, without assuming uniqueness.
        let inv = pairs.iter().all(|&(i, j)| {
            let a = perm[i].min(perm[j]);
            let b = perm[i].max(perm[j]);
            pairs.contains(&(a, b))
        });
        assert_eq!(inv, stable(s, &perm, g));
    }
    let fixed = (0..1u16 << (2 * g))
        .filter(|&v| act(v, &perm, g) == v)
        .count();
    assert!(fixed.is_power_of_two());
    let dim = fixed.trailing_zeros() as usize;
    println!("CASE label={label} g={g} cycles={cycles:?} all={} paired={} fixed_dim={dim} first_full={:?}",full.len(),paired.len(),full.first());
    (full.len(), paired.len(), dim)
}
fn integer_partitions(n: usize, min: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if n == 0 {
        out.push(prefix.clone());
        return;
    }
    for i in min..=n {
        prefix.push(i);
        integer_partitions(n - i, i, prefix, out);
        prefix.pop();
    }
}
fn smith(c: &[usize]) -> usize {
    match c {
        [8] | [2, 6] | [1, 1, 6] | [1, 1, 2, 4] => 1,
        [4, 4] => 5,
        [2, 2, 4] | [1, 1, 1, 1, 4] | [2, 3, 3] | [1, 1, 3, 3] => 3,
        [1, 1, 2, 2, 2] => 7,
        [1, 1, 1, 1, 2, 2] => 9,
        [1, 1, 1, 1, 1, 1, 2] => 15,
        [2, 2, 2, 2] => 25,
        [1, 1, 1, 1, 1, 1, 1, 1] => 105,
        _ => 0,
    }
}
fn factors(f: &Poly, p: u64) -> Vec<usize> {
    let fp = Fp::new(p);
    let d = f.n - 1;
    assert_eq!(Poly::gcd(f, &f.deriv(&fp), &fp), Poly::one());
    let x = Poly::from_coeffs(&[0, 1]);
    let mut xp = x;
    let mut ns = vec![0usize; d + 1];
    let mut out = Vec::new();
    for k in 1..=d {
        xp = xp.powmod(p, f, &fp);
        let fixed = Poly::gcd(f, &xp.sub(&x, &fp), &fp).deg() as usize;
        let lower: usize = (1..k).filter(|j| k % j == 0).map(|j| j * ns[j]).sum();
        assert!(fixed >= lower && (fixed - lower) % k == 0);
        ns[k] = (fixed - lower) / k;
        out.extend(std::iter::repeat_n(k, ns[k]));
    }
    assert_eq!(out.iter().sum::<usize>(), d);
    out
}
fn main() {
    let mut caches = Vec::new();
    for g in 1..=4 {
        let all = lagrangians(g);
        let parts = partition_sets(g);
        assert_eq!(
            all.len(),
            (1..=g).map(|i| (1usize << i) + 1).product::<usize>()
        );
        assert_eq!(parts.len(), [3, 15, 105, 945][g - 1]);
        for s in &parts {
            assert!(all.binary_search(s).is_ok());
        }
        let (a, b, _) = report(&vec![1; 2 * g + 2], g, &all, &parts, "identity");
        assert_eq!((a, b), (all.len(), parts.len()));
        caches.push((all, parts));
    }
    let (all, parts) = &caches[2];
    let mut types = Vec::new();
    integer_partitions(8, 1, &mut Vec::new(), &mut types);
    for c in &types {
        let (a, b, _) = report(c, 3, all, parts, "cycle-control");
        assert_eq!(b, smith(c));
        if c == &[1, 7] {
            assert_eq!((a, b), (2, 0));
        }
    }
    let root = std::env::args().nth(1).unwrap();
    let heldout = std::env::args().nth(2).as_deref() == Some("--heldout");
    let mut cohort = Vec::new();
    if heldout {
        for (g, primes) in [
            (2, &[1009, 4001, 16381, 65537][..]),
            (3, &[127, 503, 1009][..]),
            (4, &[31, 61, 127][..]),
        ] {
            for &p in primes {
                for family in 0..2 {
                    cohort.push((g, p, family));
                }
            }
        }
    } else {
        cohort.extend([
            (2, 1009, 0),
            (2, 16381, 0),
            (3, 127, 0),
            (3, 1009, 0),
            (4, 31, 0),
            (4, 127, 0),
        ]);
    }
    let fixture_count = cohort.len();
    for (g, p, family) in cohort {
        let text =
            fs::read_to_string(Path::new(&root).join(format!("select-g{g}-p{p}-f{family}.out")))
                .unwrap();
        let t: Vec<_> = text.split_whitespace().collect();
        assert_eq!(t[0], "FIXTURE");
        assert_eq!(t[1].parse::<usize>().unwrap(), g);
        assert_eq!(t[2].parse::<u64>().unwrap(), p);
        assert_eq!(t[3].parse::<u64>().unwrap(), family);
        let n: usize = t[6].parse().unwrap();
        assert_eq!(n, 2 * g + 2);
        let coeff: Vec<u64> = t[7..7 + n].iter().map(|s| s.parse().unwrap()).collect();
        let mut c = factors(&Poly::from_coeffs(&coeff), p);
        let r = c.len();
        c.push(1);
        c.sort();
        let (all, parts) = &caches[g - 1];
        let (_, _, dim) = report(&c, g, all, parts, &format!("fixture-p{p}-f{family}"));
        assert_eq!(dim, r - 1);
    }
    println!(
        "TERMINAL geometric_genera=4 genus3_cycle_types={} fixtures={fixture_count}",
        types.len()
    );
}
