use std::{collections::BTreeMap, env, fs, path::Path};

const P: i64 = 127;
const R: i64 = 131;
type Pt = Option<(i64, i64)>;
fn m(x: i64, p: i64) -> i64 {
    x.rem_euclid(p)
}
fn inv(x: i64, p: i64) -> i64 {
    (1..p).find(|&z| m(x * z, p) == 1).unwrap()
}
fn plus(a: Pt, b: Pt) -> Pt {
    let (Some((x, y)), Some((u, v))) = (a, b) else {
        return a.or(b);
    };
    if x == u && m(y + v, P) == 0 {
        return None;
    }
    let slope = if x == u {
        m((3 * x * x + 1) * inv(2 * y, P), P)
    } else {
        m((v - y) * inv(m(u - x, P), P), P)
    };
    let nx = m(slope * slope - x - u, P);
    Some((nx, m(slope * (x - nx) - y, P)))
}
fn times(mut n: i64, mut a: Pt) -> Pt {
    let mut out = None;
    while n > 0 {
        if n % 2 == 1 {
            out = plus(out, a);
        }
        a = plus(a, a);
        n /= 2;
    }
    out
}
fn csv(path: &Path) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(str::to_string).collect())
        .collect()
}
fn rank_and_k(rows: &[[i64; 4]]) -> (usize, Option<i64>) {
    let mut a = rows.to_vec();
    let mut pivot = 0;
    for col in 0..3 {
        if let Some(j) = (pivot..a.len()).find(|&j| a[j][col] != 0) {
            a.swap(pivot, j);
            let iv = inv(a[pivot][col], R);
            for k in col..4 {
                a[pivot][k] = m(a[pivot][k] * iv, R);
            }
            for j in 0..a.len() {
                if j != pivot {
                    let f = a[j][col];
                    for k in col..4 {
                        a[j][k] = m(a[j][k] - f * a[pivot][k], R);
                    }
                }
            }
            pivot += 1;
        }
    }
    assert!(a
        .iter()
        .all(|r| r[..3].iter().any(|&v| v != 0) || r[3] == 0));
    (pivot, if pivot == 3 { Some(a[0][3]) } else { None })
}
fn main() {
    let dir = env::args().nth(1).expect("verify-relations OUT_DIR");
    let p = Some((0, 36));
    let q = Some((12, 49));
    assert_eq!(times(131, p), None);
    assert_eq!(times(37, p), q);
    let mut bases: BTreeMap<String, Vec<Pt>> = BTreeMap::new();
    for row in csv(&Path::new(&dir).join("bases.csv")) {
        let name = row[0].clone();
        let index: usize = row[1].parse().unwrap();
        let entry = bases.entry(name).or_default();
        assert_eq!(entry.len(), index);
        let pt = Some((row[2].parse().unwrap(), row[3].parse().unwrap()));
        let (x, y) = pt.unwrap();
        assert_eq!(m(y * y - x * x * x - x - 26, P), 0);
        entry.push(pt);
    }
    let mut rows: BTreeMap<String, Vec<[i64; 4]>> = BTreeMap::new();
    let mut count = 0;
    for row in csv(&Path::new(&dir).join("relations.csv")) {
        let name = &row[0];
        let a: i64 = row[2].parse().unwrap();
        let b: i64 = row[3].parse().unwrap();
        let i: usize = row[4].parse().unwrap();
        let j: usize = row[5].parse().unwrap();
        let fb = &bases[name];
        assert_eq!(plus(times(a, p), times(b, q)), plus(fb[i], fb[j]));
        let mut expect = [b, 0, 0, m(-a, R)];
        for index in [i, j] {
            expect[1 + index / 2] = m(
                expect[1 + index / 2] + if index % 2 == 0 { -1 } else { 1 },
                R,
            );
        }
        let got = [
            row[6].parse().unwrap(),
            row[7].parse().unwrap(),
            row[8].parse().unwrap(),
            row[9].parse().unwrap(),
        ];
        assert_eq!(expect, got);
        rows.entry(name.clone()).or_default().push(got);
        count += 1;
    }
    for row in csv(&Path::new(&dir).join("summary.csv")) {
        let name = &row[0];
        let declared_rank: usize = row[7].parse().unwrap();
        let (rank, k) = rank_and_k(rows.get(name).map(Vec::as_slice).unwrap_or(&[]));
        assert_eq!(rank, declared_rank);
        if row[12] == "full_rank" {
            assert_eq!(k, Some(37));
        } else {
            assert!(k.is_none());
        }
    }
    for row in csv(&Path::new(&dir).join("rho.csv")) {
        let k: i64 = row[3].parse().unwrap();
        assert_eq!(times(k, p), q);
    }
    println!(
        "independent native replay: {count} target-curve rows, all ranks and rho scalars verified"
    );
}
