type Point = [u32; 4];
const P: u32 = 31;
fn inv(x: u32) -> u32 {
    (1..P).find(|y| x * y % P == 1).unwrap()
}
fn points(n: u32) -> Vec<Vec<u32>> {
    (1..P.pow(n))
        .filter_map(|k| {
            let v = (0..n).map(|i| k / P.pow(i) % P).collect::<Vec<_>>();
            (v.iter().find(|&&x| x != 0) == Some(&1)).then_some(v)
        })
        .collect()
}
fn dot(a: Point, b: Point) -> u32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum::<u32>() % P
}
fn dual(m: &[Point; 4], x: Point) -> Point {
    m.map(|r| dot(r, x))
}
fn pair(x: Point, y: Point, m: &[Point; 4]) -> u32 {
    dot(x, dual(m, y))
}
fn hyperbolic(lines: &[Point], m: &[Point; 4]) -> [Point; 2] {
    let e = lines[0];
    let f = *lines.iter().find(|&&v| pair(e, v, m) != 0).unwrap();
    let s = inv(pair(e, f, m));
    [e, f.map(|x| x * s % P)]
}
fn ratio(x: u32, y: u32) -> usize {
    assert!(x != 0 || y != 0);
    if x == 0 {
        31
    } else {
        (inv(x) * y % P) as usize
    }
}
fn main() {
    let line = include_str!("../CONTROL.txt")
        .lines()
        .find(|s| s.starts_with("QUADRIC "))
        .unwrap();
    let q = line
        .split("coeff=[")
        .nth(1)
        .unwrap()
        .split(']')
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut m = [[0; 4]; 4];
    let mut k = 0;
    for i in 0..4 {
        for j in i..4 {
            m[i][j] = if i == j { 2 * q[k] % P } else { q[k] };
            m[j][i] = m[i][j];
            k += 1;
        }
    }
    let all: Vec<Point> = points(4)
        .into_iter()
        .map(|v| v.try_into().unwrap())
        .collect();
    let iso: Vec<_> = all
        .iter()
        .copied()
        .filter(|&x| pair(x, x, &m) == 0)
        .collect();
    let [e, f] = hyperbolic(&iso, &m);
    let perp: Vec<_> = iso
        .iter()
        .copied()
        .filter(|&x| pair(x, e, &m) == 0 && pair(x, f, &m) == 0)
        .collect();
    let [g, h] = hyperbolic(&perp, &m);
    let forms = [f, e, h, g].map(|x| dual(&m, x));
    println!("FORMS a,b,c,d={forms:?} inverse_basis={:?}", [e, f, g, h]);
    let mut grid = [[0; 32]; 32];
    for x in all {
        let [a, b, c, d] = forms.map(|r| dot(r, x));
        let recon: Point = std::array::from_fn(|i| (a * e[i] + b * f[i] + c * g[i] + d * h[i]) % P);
        assert_eq!(recon, x);
        assert_eq!((a * b + c * d) % P, 16 * pair(x, x, &m) % P);
        if (a * b + c * d) % P != 0 {
            continue;
        }
        // Rank-one matrix [[a,c],[-d,b]] gives column and row maps.
        let neg = (P - d) % P;
        let u = if a != 0 || neg != 0 {
            ratio(a, neg)
        } else {
            ratio(c, b)
        };
        let v = if a != 0 || c != 0 {
            ratio(a, c)
        } else {
            ratio(neg, b)
        };
        if (a != 0 || neg != 0) && (c != 0 || b != 0) {
            assert_eq!(u, ratio(c, b));
        }
        if (a != 0 || c != 0) && (neg != 0 || b != 0) {
            assert_eq!(v, ratio(neg, b));
        }
        grid[u][v] += 1;
    }
    assert!(grid.into_iter().flatten().all(|n| n == 1));
    println!("PASS ambient=30784 grid=32x32 points=1024 fibre_size=32");
}
