fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
fn is_prime_u64(p: u64) -> bool {
    p >= 2 && !(2..).take_while(|d| d * d <= p).any(|d| p % d == 0)
}
pub fn scalar(j: &QuarticJacobian, mut a: QClass, mut n: u64) -> QClass {
    let mut r = j.zero();
    while n > 0 {
        if n & 1 != 0 {
            r = j.add_raw(&r, &a)
        }
        a = j.add_raw(&a, &a);
        n >>= 1;
    }
    r
}
pub fn difference(j: &QuarticJacobian, p: [u64; 3], q: [u64; 3]) -> QClass {
    if p == q {
        return j.zero();
    }
    let a: Vec<_> = j
        .points
        .iter()
        .filter(|x| **x != p && **x != q)
        .copied()
        .take(2)
        .collect();
    let x = j.from_points([p, a[0], a[1]]).unwrap();
    let y = j.from_points([q, a[0], a[1]]).unwrap();
    j.add_raw(&x, &j.neg_raw(&y))
}
fn solve3(a: [[u64; 3]; 3], b: [u64; 3]) -> Option<[u64; 3]> {
    let p = 127;
    let mut m: [[u64; 4]; 3] = std::array::from_fn(|i| [a[i][0], a[i][1], a[i][2], b[i]]);
    for c in 0..3 {
        let r = (c..3).find(|i| m[*i][c] != 0)?;
        m.swap(c, r);
        let z = invm(m[c][c], p);
        for j in c..4 {
            m[c][j] = m[c][j] * z % p;
        }
        for i in 0..3 {
            if i == c {
                continue;
            }
            let z = m[i][c];
            for k in c..4 {
                m[i][k] = (m[i][k] + p - z * m[c][k] % p) % p;
            }
        }
    }
    Some(std::array::from_fn(|i| m[i][3]))
}
fn rr(a: [u64; 3], b: [u64; 3], u: [u64; 4]) -> [u64; 3] {
    let mut t = [0; 5];
    for i in 0..3 {
        for j in 0..3 {
            t[i + j] = (t[i + j] + a[i] * b[j]) % 127;
        }
    }
    for k in (3..=4).rev() {
        let z = t[k];
        for i in 0..=3 {
            t[k - 3 + i] = (t[k - 3 + i] + 127 - z * u[i] % 127) % 127;
        }
    }
    [t[0], t[1], t[2]]
}
pub fn graph_chart(j: &QuarticJacobian, x: QClass, swap: bool) -> Option<([u64; 4], [u64; 3])> {
    let ms = monomials(3);
    let idx = |a, b| {
        ms.iter()
            .position(|m| m.0 == (if swap { 3 - a - b } else { a }) && m.1 == b)
            .unwrap()
    };
    let col = |a, b| std::array::from_fn(|i| u64::from(x.0[10 * i + idx(a, b)]));
    let cols: [[u64; 3]; 3] = [col(0, 0), col(1, 0), col(2, 0)];
    let mat = std::array::from_fn(|i| std::array::from_fn(|k| cols[k][i]));
    let a = solve3(mat, col(3, 0))?;
    let y = solve3(mat, col(0, 1))?;
    let u = [
        (127 - a[0]) % 127,
        (127 - a[1]) % 127,
        (127 - a[2]) % 127,
        1,
    ];
    let mut xp = [[0; 3]; 4];
    let mut yp = xp;
    xp[0] = [1, 0, 0];
    yp[0] = xp[0];
    for i in 1..4 {
        xp[i] = rr(xp[i - 1], [0, 1, 0], u);
        yp[i] = rr(yp[i - 1], y, u);
    }
    let mut ann = vec![vec![0; 10]; 3];
    for (_k, (a, b, _)) in ms.iter().enumerate() {
        let t = rr(xp[*a as usize], yp[*b as usize], u);
        let expect = col(*a, *b);
        for i in 0..3 {
            let z = (0..3).fold(0, |z, l| (z + cols[l][i] * t[l]) % 127);
            if z != expect[i] {
                return None;
            }
            ann[i][idx(*a, *b)] = t[i];
        }
    }
    assert_eq!(
        j.class_from_conditions(ann).unwrap(),
        x,
        "graph class roundtrip"
    );
    Some((u, y))
}

pub fn graph(j: &QuarticJacobian, x: QClass) -> Option<([u64; 4], [u64; 3])> {
    graph_chart(j, x, false)
}
