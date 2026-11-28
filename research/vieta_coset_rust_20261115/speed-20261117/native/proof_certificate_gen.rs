//! Emit integer multiples of17 for Lean to check. This generator is not
//! trusted: each emitted relation is checked by Lean's kernel via ring laws.
use std::collections::BTreeMap;
type Poly = BTreeMap<(usize, usize), i64>;
fn term(i: usize, j: usize, c: i64) -> Poly {
    BTreeMap::from([((i, j), c)])
}
fn add(a: &Poly, b: &Poly) -> Poly {
    let mut r = a.clone();
    for (k, v) in b {
        *r.entry(*k).or_default() += v;
    }
    r.retain(|_, v| *v != 0);
    r
}
fn scale(a: &Poly, c: i64) -> Poly {
    a.iter()
        .filter_map(|(k, v)| (*v * c != 0).then_some((*k, *v * c)))
        .collect()
}
fn mul(a: &Poly, b: &Poly) -> Poly {
    let mut r = Poly::new();
    for ((i, j), v) in a {
        for ((k, l), w) in b {
            *r.entry((i + k, j + l)).or_default() += v * w;
        }
    }
    r.retain(|_, v| *v != 0);
    r
}
fn line(a: i64, b: i64, x: bool) -> Poly {
    add(&term(0, 0, b), &term(usize::from(x), usize::from(!x), a))
}
fn emit(name: &str, l: &Poly, r: &Poly) {
    let d = add(l, &scale(r, -1));
    let mut parts = Vec::new();
    for ((i, j), c) in d {
        assert_eq!(c % 17, 0);
        let mut s = format!("({} : P)", c / 17);
        if i > 0 {
            s += &format!("*x^{i}");
        }
        if j > 0 {
            s += &format!("*y^{j}");
        }
        parts.push(s);
    }
    println!("{name}: linear_combination ({})*h17", parts.join(" + "));
}
fn main() {
    let x = term(1, 0, 1);
    let y = term(0, 1, 1);
    let one = term(0, 0, 1);
    let f0 = add(&add(&mul(&x, &x), &one), &scale(&mul(&x, &y), -2));
    let f1 = add(&add(&mul(&y, &y), &one), &scale(&y, -8));
    let mut sum = Poly::new();
    for (name, a, child, lo, hi, ax, ay, cx, bx, by) in [
        ("10", 10, 11, 8, 6, 11, 16, 3, 9, 6),
        ("12", 12, 11, 9, 12, 11, 16, 14, 8, 11),
        ("2", 2, 14, 12, 11, 6, 2, 4, 7, 8),
        ("9", 9, 14, 5, 7, 6, 2, 13, 10, 9),
    ] {
        let ex = line(lo, hi, true);
        let ey = line(ax, ay, false);
        let e = mul(&ex, &ey);
        sum = add(&sum, &e);
        emit(
            &format!("coordinate{name}x"),
            &mul(&add(&x, &term(0, 0, -a)), &e),
            &add(&mul(&line(cx, bx, false), &f0), &mul(&scale(&x, by), &f1)),
        );
        let factor = scale(&ex, if child == 11 { 11 } else { 6 });
        // The human-readable certificate writes coefficients canonically modulo17.
        let canonical: Poly = factor
            .into_iter()
            .map(|(k, v)| (k, v.rem_euclid(17)))
            .collect();
        emit(
            &format!("coordinate{name}y"),
            &mul(&add(&y, &term(0, 0, -child)), &e),
            &mul(&canonical, &f1),
        );
    }
    emit("partition", &sum, &one);
}
