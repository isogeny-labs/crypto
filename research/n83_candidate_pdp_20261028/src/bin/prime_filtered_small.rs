//! Exact small filtered-closure development controls; never n83 production.
use serde_json::{json, Value};
use std::collections::{BTreeMap, VecDeque};
type Mono = (u16, u8); // squarefree selector mask, unrestricted U exponent
type Poly = BTreeMap<Mono, u64>;
fn add_term(f: &mut Poly, m: Mono, c: u64, p: u64) {
    let v = (f.get(&m).unwrap_or(&0) + c) % p;
    if v == 0 {
        f.remove(&m);
    } else {
        f.insert(m, v);
    }
}
fn plus(f: &mut Poly, g: &Poly, c: u64, p: u64) {
    for (&m, &v) in g {
        add_term(f, m, v * c % p, p);
    }
}
fn degree(f: &Poly) -> usize {
    f.keys()
        .map(|&(m, u)| m.count_ones() as usize + u as usize)
        .max()
        .unwrap_or(0)
}
fn product(f: &Poly, g: &Poly, w: &[u64], p: u64) -> Poly {
    assert!(p <= 65521 && w.len() <= 12);
    let mut h = Poly::new();
    for (&(a, u), &x) in f {
        for (&(b, v), &y) in g {
            let mut c = x * y % p;
            let mut overlap = a & b;
            while overlap != 0 {
                let j = overlap.trailing_zeros() as usize;
                c = c * w[j] % p;
                overlap &= overlap - 1;
            }
            add_term(&mut h, (a | b, u.checked_add(v).unwrap()), c, p);
        }
    }
    h
}
fn constant(c: u64) -> Poly {
    [((0, 0), c)].into()
}
fn variable(j: usize, n: usize) -> Poly {
    [(if j == n { (0, 1) } else { (1 << j, 0) }, 1)].into()
}
fn s3(x: &Poly, y: &Poly, z: &Poly, w: &[u64], p: u64) -> Poly {
    let mut minus = x.clone();
    plus(&mut minus, y, p - 1, p);
    let mut sum = x.clone();
    plus(&mut sum, y, 1, p);
    let xy = product(x, y, w, p);
    let mut inner = xy.clone();
    plus(&mut inner, &constant(2), 1, p);
    let mut inner = product(&sum, &inner, w, p);
    plus(&mut inner, &constant(6), 1, p);
    let mut f = product(&product(&minus, &minus, w, p), &product(z, z, w, p), w, p);
    plus(&mut f, &product(&inner, z, w, p), p - 2, p);
    let mut q = xy;
    plus(&mut q, &constant(2), p - 1, p);
    plus(&mut f, &product(&q, &q, w, p), 1, p);
    plus(&mut f, &sum, p - 12, p);
    f
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut v = 1;
    while n > 0 {
        if n & 1 != 0 {
            v = v * a % p;
        }
        a = a * a % p;
        n >>= 1;
    }
    v
}
struct Space {
    cols: Vec<Mono>,
    rows: BTreeMap<usize, Vec<u64>>,
    processed: usize,
}
fn reduce(row: &mut [u64], rows: &BTreeMap<usize, Vec<u64>>, p: u64) {
    for (&pivot, b) in rows.iter().rev() {
        let c = row[pivot];
        if c != 0 {
            for j in 0..=pivot {
                row[j] = (row[j] + p - c * b[j] % p) % p;
            }
        }
    }
}
fn row_poly(row: &[u64], cols: &[Mono]) -> Poly {
    cols.iter()
        .zip(row)
        .filter(|(_, c)| **c != 0)
        .map(|(&m, &c)| (m, c))
        .collect()
}
fn closure(fs: &[Poly], w: &[u64], d: usize, p: u64) -> Space {
    assert!(w.len() <= 9 && d <= 5 && [101, 103].contains(&p));
    let mut cols = vec![];
    for mask in 0..(1u16 << w.len()) {
        let k = mask.count_ones() as usize;
        if k <= d {
            for u in 0..=d - k {
                cols.push((mask, u as u8));
            }
        }
    }
    cols.sort_by_key(|&(m, u)| (m.count_ones() + u as u32, m, u));
    assert!(cols.len() <= 4096);
    let index: BTreeMap<_, _> = cols.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut rows = BTreeMap::new();
    let mut processed = 0;
    let mut queue: VecDeque<_> = fs.iter().filter(|f| degree(f) <= d).cloned().collect();
    while let Some(f) = queue.pop_front() {
        processed += 1;
        assert!(processed <= 50000, "development queue cap");
        let mut row = vec![0; cols.len()];
        for (m, c) in f {
            row[index[&m]] = c;
        }
        reduce(&mut row, &rows, p);
        let Some(pivot) = row.iter().rposition(|&c| c != 0) else {
            continue;
        };
        let inv = pow(row[pivot], p - 2, p);
        for c in &mut row {
            *c = *c * inv % p;
        }
        let f = row_poly(&row, &cols);
        if degree(&f) < d {
            for j in 0..=w.len() {
                queue.push_back(product(&f, &variable(j, w.len()), w, p));
            }
        }
        rows.insert(pivot, row);
    }
    Space {
        cols,
        rows,
        processed,
    }
}
fn member(f: &Poly, s: &Space, p: u64) -> bool {
    let index: BTreeMap<_, _> = s.cols.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut row = vec![0; s.cols.len()];
    for (m, &c) in f {
        let Some(&i) = index.get(m) else { return false };
        row[i] = c;
    }
    reduce(&mut row, &s.rows, p);
    row.iter().all(|&c| c == 0)
}
fn closed(s: &Space, w: &[u64], d: usize, p: u64) -> usize {
    let mut checks = 0;
    for row in s.rows.values() {
        let f = row_poly(row, &s.cols);
        if degree(&f) < d {
            for j in 0..=w.len() {
                assert!(member(&product(&f, &variable(j, w.len()), w, p), s, p));
                checks += 1;
            }
        }
    }
    checks
}
fn encode(f: &Poly) -> Value {
    json!(f
        .iter()
        .map(|(&(m, u), &c)| json!([m, u, c]))
        .collect::<Vec<_>>())
}
fn encode_space(s: &Space) -> Value {
    json!({"columns":s.cols,"processed_rows":s.processed,"rank":s.rows.len(),
      "basis":s.rows.iter().map(|(&pivot,row)|json!({"pivot":pivot,"entries":row.iter().enumerate().filter(|(_,c)|**c!=0).map(|(i,&c)|[i as u64,c]).collect::<Vec<_>>()})).collect::<Vec<_>>()})
}
fn main() {
    eprintln!("Development tests only; no production entry point");
    std::process::exit(2);
}
#[cfg(test)]
mod tests {
    use super::*;
    fn retain(name: &str, v: &Value) {
        if let Ok(root) = std::env::var("N83_FILTERED_SMALL_OUT") {
            let path = std::path::Path::new(&root).join(name);
            assert!(!path.exists());
            std::fs::write(path, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
        }
    }
    #[test]
    fn chain_witness_full_filtered_closure() {
        let mut summary = vec![];
        for p in [101, 103] {
            let w: Vec<_> = (0..9).map(|j| 1u64 << (j % 3)).collect();
            let xs: Vec<Poly> = (0..3)
                .map(|b| (0..3).map(|j| ((1u16 << (3 * b + j), 0), 1)).collect())
                .collect();
            let u = variable(9, 9);
            let f0 = s3(&xs[0], &xs[1], &u, &w, p);
            for t in [1, 7, 7 * (p + 1) / 2 % p] {
                let f1 = s3(&u, &xs[2], &constant(t), &w, p);
                let fs = vec![f0.clone(), f1.clone()];
                let h: Poly = [((1 << 6, 0), 1), ((1 << 7, 0), p - 1)].into();
                let g = product(&h, &f1, &w, p);
                assert!(degree(&g) <= 4);
                let v4 = closure(&fs, &w, 4, p);
                let v5 = closure(&fs, &w, 5, p);
                assert_eq!(v4.rows.len(), 2);
                for f in &fs {
                    assert!(member(f, &v4, p));
                    assert!(member(f, &v5, p));
                }
                let lower = member(&g, &v4, p);
                let upper = member(&g, &v5, p);
                assert!(upper);
                let mut mutated = g.clone();
                plus(&mut mutated, &constant(1), 1, p);
                let mutated_upper = member(&mutated, &v5, p);
                assert!(!mutated_upper, "constant mutation must fail membership");
                let delta = (7 + p - 2 * t % p) % p;
                if delta != 0 {
                    assert!(!lower);
                }
                let c4 = closed(&v4, &w, 4, p);
                let c5 = closed(&v5, &w, 5, p);
                let row = json!({"field":p,"m":3,"ell":3,"target":t,"delta":delta,
                  "witness_degree":degree(&g),"member_v4":lower,"member_v5":upper,
                  "mutated_witness_member_v5":mutated_upper,
                  "rank_v4":v4.rows.len(),"rank_v5":v5.rows.len(),
                  "closure_checks_v4":c4,"closure_checks_v5":c5,
                  "scope":"small synthetic polynomial system; no n83 or rational decomposition claim"});
                retain(
                    &format!("p{p}-t{t}.json"),
                    &json!({"schema":"n83.small-filtered-development/v1","summary":row,
                  "weights":w,"generators":fs.iter().map(encode).collect::<Vec<_>>(),"witness":encode(&g),
                  "v4":encode_space(&v4),"v5":encode_space(&v5)}),
                );
                summary.push(row);
            }
        }
        retain(
            "summary.json",
            &json!({"cases":summary,"production_cases":0}),
        );
    }
    #[test]
    fn closure_requires_multiplying_a_new_degree_fall() {
        let p = 101;
        let w = [];
        let u = variable(0, 0);
        let u2 = product(&u, &u, &w, p);
        let mut f = u2.clone();
        plus(&mut f, &u, p - 1, p);
        let s = closure(&[u2, f], &w, 3, p);
        assert!(member(&u, &s, p));
        let u3 = product(&product(&u, &u, &w, p), &u, &w, p);
        assert!(member(&u3, &s, p));
        assert!(!member(&constant(1), &s, p));
        assert_eq!(s.rows.len(), 3);
        assert!(closed(&s, &w, 3, p) > 0);
    }
    #[test]
    fn selector_normalization_and_unrestricted_intermediate() {
        let p = 101;
        let w = [7];
        let z = variable(0, 1);
        let u = variable(1, 1);
        let mut expected = z.clone();
        for c in expected.values_mut() {
            *c = 7;
        }
        assert_eq!(product(&z, &z, &w, p), expected);
        assert_eq!(product(&u, &u, &w, p), [((0, 2), 1)].into());
        let empty = closure(&[], &w, 2, p);
        assert!(!member(&z, &empty, p));
    }
}
