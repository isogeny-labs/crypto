use std::collections::BTreeSet;
const L: u64 = 239753;
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn inv(a: u64) -> u64 {
    assert!(a != 0);
    let (mut t, mut k, mut v) = (a, L - 2, 1);
    while k != 0 {
        if k % 2 == 1 {
            v = v * t % L
        }
        t = t * t % L;
        k /= 2;
    }
    assert_eq!(a * v % L, 1);
    v
}
// Coefficient pair for a+b*i, i²=-1, encoded in base 31.
fn mul(x: u64, y: u64) -> u64 {
    let (a, b, c, d) = (x % 31, x / 31, y % 31, y / 31);
    (a * c + 30 * b * d) % 31 + 31 * ((a * d + b * c) % 31)
}
fn add(x: u64, y: u64) -> u64 {
    (x % 31 + y % 31) % 31 + 31 * ((x / 31 + y / 31) % 31)
}
fn main() {
    assert!((2..=490).all(|i| L % i != 0));
    let mut sq = [0; 961];
    for x in 0..961 {
        sq[mul(x, x) as usize] += 1;
    }
    assert_eq!(sq[30], 2);
    assert!((0..31).all(|i| i * i % 31 != 30));
    let mut n2 = 1i64;
    for x in 0..961 {
        let z = mul(x, x);
        let z2 = mul(z, z);
        let x9 = mul(mul(z2, z2), x);
        let y = add(add(x9, mul(z, x)), add(mul(16, x), 1));
        n2 += sq[y as usize];
    }
    let s2 = 962 - n2;
    assert_eq!(s2 % 2, 0);
    let a2 = -s2 / 2;
    let twist = include_str!("../hec_g4_segre_cubic_20261118/twist/CONTROL.txt");
    assert!(
        twist.contains("TWIST traces=[0, -360] quotient_trace3=-360 quotient_matches_model=true")
    );
    let a3 = 120i64;
    let without = 1 + 31i64.pow(4) + a2 * (1 + 961) + a3 * 32;
    let bound = 70 * 961;
    let possible: Vec<_> = (-bound..=bound)
        .filter(|a| (without + a) % L as i64 == 0)
        .collect();
    assert_eq!(possible.len(), 1);
    let a4 = possible[0];
    let order = without + a4;
    let cofactor = order / L as i64;
    assert!((1..32).contains(&cofactor));
    assert!(7u64.pow(8) < 32 * L);
    println!("GROUP n2={n2} traces=[0,{s2},-360] a2={a2} a3={a3} a4={a4} order={order} cofactor={cofactor}");
    let rec = include_str!("../hec_g4_segre_cubic_20261118/relations/CONTROL.txt");
    let divs: Vec<_> = rec
        .lines()
        .filter_map(|s| s.split_once(" divisor=").map(|(_, r)| nums(r)))
        .collect();
    assert_eq!(divs.len(), 81);
    let rows: Vec<Vec<_>> = divs
        .iter()
        .skip(1)
        .map(|d| {
            assert_eq!(d.len(), 32);
            assert_eq!(d.iter().sum::<u64>(), 6);
            d.iter()
                .zip(&divs[0])
                .map(|(a, b)| (a + L - b) % L)
                .collect()
        })
        .collect();
    let mut mat = rows.clone();
    let mut piv = vec![];
    for j in 0..32 {
        let k = piv.len();
        let Some(i) = (k..mat.len()).find(|&i| mat[i][j] != 0) else {
            continue;
        };
        mat.swap(k, i);
        let iv = inv(mat[k][j]);
        let row: Vec<_> = mat[k].iter().map(|v| v * iv % L).collect();
        for (i, r) in mat.iter_mut().enumerate() {
            if i != k {
                let c = r[j];
                for l in j..32 {
                    r[l] = (r[l] + L - c * row[l] % L) % L;
                }
            }
        }
        mat[k] = row;
        piv.push(j);
    }
    assert_eq!(piv.len(), 30);
    let free = (0..32).find(|i| !piv.contains(i)).unwrap();
    let mut v = [0; 32];
    v[free] = 1;
    for (i, &j) in piv.iter().enumerate() {
        v[j] = (L - mat[i][free]) % L;
    }
    let base = v[0];
    for x in &mut v {
        *x = (*x + L - base) % L;
    }
    let anchor = v.iter().position(|&x| x != 0).unwrap();
    let iv = inv(v[anchor]);
    for x in &mut v {
        *x = *x * iv % L;
    }
    for r in rows {
        assert_eq!(r.iter().zip(v).map(|(a, b)| a * b).sum::<u64>() % L, 0);
    }
    let canon = divs[0].iter().zip(v).map(|(a, b)| a * b).sum::<u64>() % L;
    let model = include_str!("../hec_g4_segre_cubic_20261118/CONTROL.txt");
    let pts = nums(
        model
            .lines()
            .find(|s| s.starts_with("POINTS "))
            .unwrap()
            .split("grid=")
            .nth(1)
            .unwrap(),
    );
    assert_eq!(pts.len(), 64);
    for (i, p) in pts.chunks_exact(2).enumerate() {
        println!("POINT i={i} t={} s={} log={}", p[0], p[1], v[i]);
    }
    println!(
        "CALIBRATION base=0 anchor={anchor} distinct_logs={} canonical_minus_six_base={canon}",
        v.into_iter().collect::<BTreeSet<_>>().len()
    );
}
