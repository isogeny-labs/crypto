#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/field.rs"]
mod field;
use field::{scalar, E, F, Z};
type Row = Vec<E>;
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn dot(f: &F, a: &Row, b: &Row) -> E {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(Z, |s, (&x, &y)| f.add(s, f.mul(x, y)))
}
fn reduce(f: &F, mut rows: Vec<Row>, n: usize) -> (Vec<Row>, Vec<usize>) {
    let mut piv = vec![];
    for j in 0..n {
        let k = piv.len();
        let Some(i) = (k..rows.len()).find(|&i| rows[i][j] != Z) else {
            continue;
        };
        rows.swap(i, k);
        let inv = f.inv(rows[k][j]);
        let r: Row = rows[k].iter().map(|&x| f.mul(inv, x)).collect();
        for (i, row) in rows.iter_mut().enumerate() {
            let t = row[j];
            if i != k && t != Z {
                for l in j..n {
                    row[l] = f.sub(row[l], f.mul(t, r[l]));
                }
            }
        }
        rows[k] = r;
        piv.push(j);
    }
    rows.truncate(piv.len());
    (rows, piv)
}
fn null(f: &F, rows: &[Row], n: usize) -> Vec<Row> {
    let (r, p) = reduce(f, rows.to_vec(), n);
    (0..n)
        .filter(|j| !p.contains(j))
        .map(|j| {
            let mut x = vec![Z; n];
            x[j] = scalar(1);
            for (i, &k) in p.iter().enumerate() {
                x[k] = f.sub(Z, r[i][j]);
            }
            assert!(rows.iter().all(|r| dot(f, r, &x) == Z));
            x
        })
        .collect()
}
fn square(f: &F, x: &Row) -> Row {
    (0..x.len())
        .flat_map(|i| (i..x.len()).map(move |j| f.mul(x[i], x[j])))
        .collect()
}
fn ideal(f: &F, x: &[Row]) -> Vec<Row> {
    null(f, &x.iter().map(|v| square(f, v)).collect::<Vec<_>>(), 21)
}
fn gradient(f: &F, q: &Row, x: &Row) -> Row {
    let mut g = vec![Z; 6];
    let mut k = 0;
    for i in 0..6 {
        for j in i..6 {
            g[i] = f.add(g[i], f.mul(q[k], x[j]));
            g[j] = f.add(g[j], f.mul(q[k], x[i]));
            k += 1;
        }
    }
    g
}
fn product(f: &F, a: &Row, m: usize, b: &Row, n: usize) -> Row {
    let mut out = vec![Z; (m + n + 1).pow(2)];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            let k = (i / (m + 1) + j / (n + 1)) * (m + n + 1) + i % (m + 1) + j % (n + 1);
            out[k] = f.add(out[k], f.mul(x, y));
        }
    }
    out
}
fn quartics(f: &F, pairs: Vec<Row>) -> Vec<Row> {
    let (basis, piv) = reduce(f, pairs.clone(), 9);
    assert_eq!(piv.len(), 6);
    let pts: Vec<Row> = pairs
        .iter()
        .map(|p| piv.iter().map(|&j| p[j]).collect())
        .collect();
    let quadrics = ideal(f, &pts);
    if quadrics.len() != 6 {
        return vec![];
    }
    let tangent: Vec<_> = pts
        .iter()
        .map(|p| {
            null(
                f,
                &quadrics
                    .iter()
                    .map(|q| gradient(f, q, p))
                    .collect::<Vec<_>>(),
                6,
            )
        })
        .collect();
    assert!(tangent.iter().all(|t| t.len() == 3));
    let mut dual = vec![];
    for i in 0..28 {
        for j in 0..i {
            let mut both = tangent[i].clone();
            both.extend_from_slice(&tangent[j]);
            let v = null(f, &both, 6);
            assert_eq!(v.len(), 1);
            dual.extend(v);
        }
    }
    let equations = ideal(f, &dual);
    if equations.len() != 6 {
        return vec![];
    }
    let mut polys = vec![];
    for i in 0..6 {
        for j in i..6 {
            polys.push(product(f, &basis[i], 2, &basis[j], 2));
        }
    }
    equations
        .iter()
        .map(|q| {
            let mut r = vec![Z; 25];
            for (&a, p) in q.iter().zip(&polys) {
                for (i, &b) in p.iter().enumerate() {
                    r[i] = f.add(r[i], f.mul(a, b));
                }
            }
            r
        })
        .collect()
}
fn monoms(f: &F, t: usize, s: usize, d: usize) -> Row {
    let power = |v: usize, i: usize| {
        if v == 31 {
            scalar((i == d) as u64)
        } else {
            f.pow(scalar(v as u64), i as u64)
        }
    };
    (0..=d)
        .flat_map(|i| (0..=d).map(move |j| f.mul(power(t, i), power(s, j))))
        .collect()
}
fn main() {
    let f = F::new(31, vec![17, 0, 30, 0, 5, 0, 6, 0, 1]);
    let segre = include_str!("../hec_g4_canonical_quadric_20261118/rulings/CONTROL.txt");
    let inv = nums(
        segre
            .lines()
            .next()
            .unwrap()
            .split("inverse_basis=")
            .nth(1)
            .unwrap(),
    );
    let basis: Vec<Row> = inv
        .chunks_exact(4)
        .map(|r| r.iter().map(|&v| scalar(v)).collect())
        .collect();
    let raw = include_str!("../hec_g4_canonical_quadric_20261118/CONTROL.txt");
    let mut planes = vec![None; 256];
    for l in raw.lines().filter(|s| s.starts_with("PLANE ")) {
        let (label, r) = l.split_once(" form=").unwrap();
        let p: Row = nums(r)
            .chunks_exact(9)
            .map(|x| x.try_into().unwrap())
            .collect();
        let a: Row = basis.iter().map(|v| dot(&f, v, &p)).collect();
        planes[nums(label)[0] as usize] = Some(vec![a[0], a[2], f.sub(Z, a[3]), a[1]]);
    }
    let rational = planes
        .iter()
        .flatten()
        .filter(|p| p.iter().all(|e| e[1..].iter().all(|&v| v == 0)))
        .count();
    println!("PLANES rational={rational}");
    let mut span = vec![];
    for delta in 1..256 {
        let pairs: Vec<_> = (0..256)
            .filter(|&i| i < (i ^ delta))
            .filter_map(|i| {
                Some(product(
                    &f,
                    planes[i].as_ref()?,
                    1,
                    planes[i ^ delta].as_ref()?,
                    1,
                ))
            })
            .collect();
        assert_eq!(pairs.len(), 28);
        span.extend(quartics(&f, pairs));
        span = reduce(&f, span, 25).0;
        println!("STEINER delta={delta} quartic_rank={}", span.len());
        assert!(span.len() <= 4);
        if span.len() == 4 {
            break;
        }
    }
    assert_eq!(span.len(), 4);
    let mut points = vec![];
    for t in 0..32 {
        for s in 0..32 {
            let v = monoms(&f, t, s, 4);
            if span.iter().all(|r| dot(&f, r, &v) == Z) {
                points.push((t, s));
            }
        }
    }
    let evaluation: Vec<_> = points.iter().map(|&(t, s)| monoms(&f, t, s, 3)).collect();
    let mut eq = null(&f, &evaluation, 16);
    assert_eq!(eq.len(), 1);
    let row = &mut eq[0];
    let inv = f.inv(*row.iter().find(|&&x| x != Z).unwrap());
    for c in row.iter_mut() {
        *c = f.mul(*c, inv);
        assert!(c[1..].iter().all(|&x| x == 0));
    }
    let lifted = (0..4)
        .map(|i| {
            let mut l = vec![Z; 4];
            l[i] = scalar(1);
            product(&f, row, 3, &l, 1)
        })
        .collect();
    assert_eq!(reduce(&f, lifted, 25).0, span);
    println!(
        "MODEL degrees=3,3 coeff={:?}",
        row.iter().map(|c| c[0]).collect::<Vec<_>>()
    );
    println!("POINTS count={} grid={points:?}", points.len());
    for axis in 0..2 {
        let mut counts = [0; 4];
        let mut triples = vec![];
        for p in 0..32 {
            let q: Vec<_> = points
                .iter()
                .copied()
                .filter(|&(t, s)| if axis == 0 { t == p } else { s == p })
                .collect();
            counts[q.len()] += 1;
            if q.len() == 3 {
                triples.push(p);
            }
        }
        println!("FIBERS axis={axis} distinct_counts={counts:?} triple_params={triples:?}");
    }
}
