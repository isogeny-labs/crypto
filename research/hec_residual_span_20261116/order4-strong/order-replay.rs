use hec_native_ic::{fp::Fp, jac::Curve, poly::Poly};
use std::{collections::BTreeMap, fs};
fn number(s: &str) -> u64 {
    s.parse().unwrap()
}
fn sqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            return x;
        }
        x = y
    }
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}
fn factors(mut n: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            v.push(d);
            while n % d == 0 {
                n /= d
            }
        }
        d += 1
    }
    if n > 1 {
        v.push(n)
    }
    v
}
fn curve(p: u64, k: u64) -> Curve {
    Curve::new(
        Fp::new(p),
        Poly::from_coeffs(&[0, 2, 0, k % 4, 0, (k / 4) % 4, 0, k / 16, 0, 1]),
    )
}
fn choose(n: u32, k: u32) -> i128 {
    let mut a = 1;
    for j in 0..k {
        a = a * (n - j) as i128 / (j + 1) as i128
    }
    a
}
fn interval(p: u64, ns: &BTreeMap<u64, u64>) -> (u64, u64) {
    let q = p as i128;
    let s1 = q + 1 - ns[&1] as i128;
    let s2 = q * q + 1 - ns[&2] as i128;
    let c1 = -s1;
    let c2 = (s1 * s1 - s2) / 2;
    assert_eq!((s1 * s1 - s2) % 2, 0);
    let mut a = 0;
    let mut b = 0;
    for j in 0..=8 {
        if j % 2 == 0 {
            a += choose(8, j) * q.pow(j / 2)
        } else {
            b += choose(8, j) * q.pow((j - 1) / 2)
        }
    }
    let t = sqrt((b * b * q) as u128) as i128;
    let floor = sqrt(p as u128);
    let ceil = floor + u128::from(floor * floor < p as u128);
    let center = q.pow(4) + 1 + (q.pow(3) + 1) * c1 + (q * q + 1) * c2;
    let radius = (q + 1) * 56 * q * ceil as i128 + 70 * q * q;
    (
        (a - t).max(center - radius).max(1) as u64,
        (a + t).min(center + radius) as u64,
    )
}
fn main() {
    let root = "research/hec_residual_span_20261116/order4-strong";
    let read = |name: &str| -> Vec<Vec<String>> {
        fs::read_to_string(format!("{root}/{name}.tsv"))
            .unwrap()
            .lines()
            .skip(1)
            .map(|l| l.split('\t').map(str::to_string).collect())
            .collect()
    };
    let mut counts: BTreeMap<(u64, u64), BTreeMap<u64, u64>> = BTreeMap::new();
    for v in read("POINTS") {
        assert!(counts
            .entry((number(&v[0]), number(&v[1])))
            .or_default()
            .insert(number(&v[2]), number(&v[4]))
            .is_none())
    }
    for (&(p, k), ns) in &counts {
        let c = curve(p, k);
        let mut points = 1;
        for x in 0..p {
            let v = c.f.eval(x, &c.fp);
            points += if v == 0 {
                1
            } else if c.fp.legendre(v) == 1 {
                2
            } else {
                0
            };
        }
        assert_eq!(points, ns[&1]);
    }
    for n in 0..10000u128 {
        let r = sqrt(n);
        assert!(r * r <= n && (r + 1) * (r + 1) > n);
    }
    let mut orders = BTreeMap::new();

    let mut unique = BTreeMap::new();
    let mut checked = 0;
    println!("p\tmodel\tpoint_order\tlcm\tunique_N\texact_order_pass");
    for v in read("ORDERS") {
        let p = number(&v[0]);
        let k = number(&v[1]);
        let key = (p, k);
        let c = curve(p, k);
        let x = number(&v[2]);
        let y = number(&v[3]);
        let o = number(&v[4]);
        assert!(o > 0);
        assert_eq!(c.fp.mul(y, y), c.f.eval(x, &c.fp));
        let d = c.place(x, y);
        assert!(!c.is_zero(&d));
        assert!(c.is_zero(&c.mul(&d, o)));
        for r in factors(o) {
            assert!(!c.is_zero(&c.mul(&d, o / r)))
        }
        let branch = c.place(0, 0);
        assert!(!c.is_zero(&branch));
        assert!(c.is_zero(&c.mul(&branch, 2)));
        let old = *orders.get(&key).unwrap_or(&2);
        let l = (old / gcd(old, o)).checked_mul(o).unwrap();
        orders.insert(key, l);
        assert_eq!(l, number(&v[5]));
        let (lo, hi) = interval(p, &counts[&key]);
        assert_eq!((lo, hi), (number(&v[6]), number(&v[7])));
        let first = lo.div_ceil(l) * l;
        let u = if first <= hi && first.checked_add(l).unwrap() > hi {
            Some(first)
        } else {
            None
        };
        assert_eq!(u, v[8].parse::<u64>().ok());
        if let Some(n) = u {
            unique.insert(key, n);
        }
        checked += 1;
        println!("{p}\t{k}\t{o}\t{l}\t{}\ttrue", v[8]);
    }
    let mut selected = 0;
    for v in read("CONTROL") {
        if v[12] != "true" {
            continue;
        }
        let p = number(&v[0]);
        let k = number(&v[1]);
        let n = number(&v[7]);
        let ell = number(&v[8]);
        let key = (p, k);
        assert_eq!(unique[&key], n);
        assert_eq!(factors(ell), vec![ell]);
        assert_eq!(n / ell, 2);
        assert_eq!(n % ell, 0);
        assert_ne!((n / ell) % ell, 0);
        let ceil = sqrt(p as u128) + 1;
        assert!(ell as u128 > (ceil + 1).pow(6));
        let cs = &counts[&key];
        let q = p as i128;
        let s1 = q + 1 - cs[&1] as i128;
        let s2 = q * q + 1 - cs[&2] as i128;
        let s3 = q.pow(3) + 1 - cs[&3] as i128;
        let c1 = -s1;
        let c2 = (s1 * s1 - s2) / 2;
        let z = -(s3 + c1 * s2 + c2 * s1);
        assert_eq!(z % 3, 0);
        let c3 = z / 3;
        let c4 = n as i128 - q.pow(4) - 1 - (q.pow(3) + 1) * c1 - (q * q + 1) * c2 - (q + 1) * c3;
        let f = vec![
            q.pow(4),
            q.pow(3) * c1,
            q * q * c2,
            q * c3,
            c4,
            c3,
            c2,
            c1,
            1,
        ];
        let row = read("WEIL")
            .into_iter()
            .find(|r| number(&r[0]) == p && number(&r[1]) == k)
            .unwrap();
        let expected: Vec<i128> = row[2]
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        assert_eq!(f, expected);
        assert_eq!(f.iter().sum::<i128>(), n as i128);
        selected += 1;
    }
    assert_eq!(selected, 3);
    assert_eq!(checked, 39);
    eprintln!("exact_point_orders={checked} selected_orders={selected} weil_reconstructions=3 shared_native_group_arithmetic=true extension_counts_not_reenumerated=true");
}
