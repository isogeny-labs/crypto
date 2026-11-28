const P: u32 = 31;
const L: u64 = 239753;
type Div = [u32; 32];
fn nums(s: &str) -> Vec<u32> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn eval(c: &[u32], x: u32) -> u32 {
    c.iter().rev().fold(0, |v, &a| (v * x + a) % P)
}
fn roots(mut c: Vec<u32>) -> Vec<u32> {
    let mut out = vec![];
    while c.last() == Some(&0) {
        out.push(P);
        c.pop();
    }
    assert!(!c.is_empty());
    for x in 0..P {
        while eval(&c, x) == 0 {
            out.push(x);
            let mut q = vec![0; c.len() - 1];
            let mut v = 0;
            for i in (1..c.len()).rev() {
                v = (c[i] + v * x) % P;
                q[i - 1] = v;
            }
            assert_eq!((c[0] + x * v) % P, 0);
            c = q;
        }
    }
    out
}
fn inv(x: u32) -> u32 {
    (1..P).find(|y| x * y % P == 1).unwrap()
}
fn point(x: u32, y: u32) -> u32 {
    assert_ne!((x, y), (0, 0));
    if x == 0 {
        P
    } else {
        y * inv(x) % P
    }
}
fn val(a: u32, b: u32, x: u32) -> u32 {
    if x == P {
        b
    } else {
        (a + b * x) % P
    }
}
fn prod(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % P;
        }
    }
    r
}
fn div(points: &[(u32, u32)], support: &[(u32, u32)]) -> Div {
    let mut d = [0; 32];
    for x in support {
        d[points.iter().position(|p| p == x).unwrap()] += 1;
    }
    d
}
fn rank(rows: &[Vec<u64>]) -> usize {
    let mut a = rows.to_vec();
    let mut n = 0;
    for j in 0..32 {
        let Some(i) = (n..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(i, n);
        let (mut x, mut k, mut z) = (a[n][j], L - 2, 1);
        while k > 0 {
            if k % 2 == 1 {
                z = z * x % L
            }
            x = x * x % L;
            k /= 2;
        }
        let r: Vec<_> = a[n].iter().map(|v| v * z % L).collect();
        for row in &mut a[n + 1..] {
            let v = row[j];
            for l in j..32 {
                row[l] = (row[l] + L - v * r[l] % L) % L;
            }
        }
        n += 1;
    }
    n
}
fn differences(d: &[Div]) -> Vec<Vec<u64>> {
    d.iter()
        .skip(1)
        .map(|a| {
            (0..32)
                .map(|i| (a[i] as u64 + L - d[0][i] as u64) % L)
                .collect()
        })
        .collect()
}
fn main() {
    let raw = include_str!("../CONTROL.txt");
    let coeff = nums(
        raw.lines()
            .find(|s| s.starts_with("MODEL "))
            .unwrap()
            .split("coeff=")
            .nth(1)
            .unwrap(),
    );
    let ps = nums(
        raw.lines()
            .find(|s| s.starts_with("POINTS "))
            .unwrap()
            .split("grid=")
            .nth(1)
            .unwrap(),
    );
    let pts: Vec<_> = ps.chunks_exact(2).map(|p| (p[0], p[1])).collect();
    assert_eq!(pts.len(), 32);
    let mut fibers = [[None; 32]; 2];
    let mut relations = vec![];
    for axis in 0..2 {
        let mut split = vec![];
        for x in 0..=P {
            let c: Vec<_> = (0..4)
                .map(|j| {
                    let v: Vec<_> = (0..4)
                        .map(|i| coeff[if axis == 0 { 4 * i + j } else { 4 * j + i }])
                        .collect();
                    if x == P {
                        v[3]
                    } else {
                        eval(&v, x)
                    }
                })
                .collect();
            let r = roots(c);
            if r.len() == 3 {
                let support: Vec<_> = r
                    .into_iter()
                    .map(|y| if axis == 0 { (x, y) } else { (y, x) })
                    .collect();
                let d = div(&pts, &support);
                fibers[axis][x as usize] = Some(d);
                split.push(d);
            }
        }
        relations.extend(differences(&split));
        println!(
            "RULING axis={axis} split={} cumulative_rank={}",
            split.len(),
            rank(&relations)
        );
    }
    let mut split = vec![];
    let mut total = 0;
    let mut red = 0;
    let mut simple = 0;
    for n in 1..P.pow(4) {
        let c: Vec<_> = (0..4).map(|i| n / P.pow(i) % P).collect();
        if c.iter().find(|&&a| a != 0) != Some(&1) {
            continue;
        }
        total += 1;
        let (a, b, g, h) = (c[0], c[1], c[2], c[3]);
        let d = if (a * h + P * P - b * g) % P == 0 {
            let x = (0..=P)
                .find(|&t| val(a, g, t) == 0 && val(b, h, t) == 0)
                .unwrap();
            let y = (0..=P)
                .find(|&s| val(a, b, s) == 0 && val(g, h, s) == 0)
                .unwrap();
            let (Some(v), Some(w)) = (fibers[0][x as usize], fibers[1][y as usize]) else {
                continue;
            };
            red += 1;
            std::array::from_fn(|i| v[i] + w[i])
        } else {
            let mut polynomial = vec![0; 7];
            for i in 0..4 {
                for j in 0..4 {
                    let mut term = vec![coeff[4 * i + j]];
                    for _ in 0..3 - j {
                        term = prod(&term, &[b, h]);
                    }
                    for _ in 0..j {
                        term = prod(&term, &[(P - a) % P, (P - g) % P]);
                    }
                    for (k, y) in term.iter().enumerate() {
                        polynomial[k + i] = (polynomial[k + i] + y) % P;
                    }
                }
            }
            let r = roots(polynomial);
            if r.len() != 6 {
                continue;
            }
            let support: Vec<_> = r
                .into_iter()
                .map(|t| (t, point(val(b, h, t), (P - val(a, g, t)) % P)))
                .collect();
            div(&pts, &support)
        };
        assert_eq!(d.iter().sum::<u32>(), 6);
        for (i, &(t, s)) in pts.iter().enumerate() {
            assert_eq!(d[i] > 0, val(val(a, g, t), val(b, h, t), s) == 0);
        }
        if d.iter().all(|&x| x <= 1) {
            simple += 1
        }
        println!("SECTION code={n} divisor={d:?}");
        split.push(d);
    }
    assert_eq!(total, 30784);
    let section_rows = differences(&split);
    let section_rank = rank(&section_rows);
    relations.extend(section_rows);
    let r = rank(&relations);
    println!("RELATIONS total={total} split={} reducible={red} six_distinct={simple} section_rank={section_rank} combined_rank={r} remaining_after_degree={}",split.len(),31-r);
}
