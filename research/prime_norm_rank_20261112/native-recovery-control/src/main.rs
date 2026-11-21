//! Fixed finite regression only; no attack solver, benchmark or random search.
mod branch_repair;
mod group_bridge;
mod public_jets;
mod signed_hankel;
use serde_json::{json, Value};
use std::{fs::OpenOptions, io::Write};
const P: u64 = 17;
type Poly = Vec<u64>;
type Mat = Vec<Vec<u64>>;
fn add(a: u64, b: u64) -> u64 {
    (a + b) % P
}
fn sub(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn mul(a: u64, b: u64) -> u64 {
    a * b % P
}
fn power(mut a: u64, mut n: u64) -> u64 {
    let mut v = 1;
    while n > 0 {
        if n & 1 == 1 {
            v = mul(v, a);
        }
        a = mul(a, a);
        n >>= 1;
    }
    v
}
fn inv(a: u64) -> u64 {
    assert_ne!(a, 0);
    power(a, P - 2)
}
fn trim(mut v: Poly) -> Poly {
    while v.last() == Some(&0) {
        v.pop();
    }
    v
}
fn coeff(a: &Poly, i: usize) -> u64 {
    *a.get(i).unwrap_or(&0)
}
fn ps(a: &Poly, b: &Poly) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| sub(coeff(a, i), coeff(b, i)))
            .collect(),
    )
}
fn pa(a: &Poly, b: &Poly) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| add(coeff(a, i), coeff(b, i)))
            .collect(),
    )
}
fn pm(a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut v = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            v[i + j] = add(v[i + j], mul(*x, *y));
        }
    }
    trim(v)
}
fn scale(a: &Poly, c: u64) -> Poly {
    trim(a.iter().map(|x| mul(*x, c)).collect())
}
fn eval(a: &Poly, x: u64) -> u64 {
    a.iter().rev().fold(0, |v, c| add(mul(v, x), *c))
}
fn remainder(a: &Poly, f: &Poly) -> Poly {
    assert_eq!(f.last(), Some(&1));
    let mut r = trim(a.clone());
    while r.len() >= f.len() {
        let shift = r.len() - f.len();
        let c = *r.last().unwrap();
        for (j, x) in f.iter().enumerate() {
            r[shift + j] = sub(r[shift + j], mul(c, *x));
        }
        r = trim(r);
    }
    r
}
fn mono(n: usize) -> Poly {
    let mut v = vec![0; n + 1];
    v[n] = 1;
    v
}
fn roots(xs: &[u64]) -> Poly {
    xs.iter().fold(vec![1], |v, x| pm(&v, &vec![sub(0, *x), 1]))
}
fn transpose(a: &Mat) -> Mat {
    (0..a[0].len())
        .map(|j| a.iter().map(|r| r[j]).collect())
        .collect()
}
fn mm(a: &Mat, b: &Mat) -> Mat {
    assert_eq!(a[0].len(), b.len());
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| (0..b.len()).fold(0, |v, k| add(v, mul(a[i][k], b[k][j]))))
                .collect()
        })
        .collect()
}
fn mv(a: &Mat, x: &[u64]) -> Vec<u64> {
    a.iter()
        .map(|r| {
            assert_eq!(r.len(), x.len());
            r.iter().zip(x).fold(0, |v, (a, b)| add(v, mul(*a, *b)))
        })
        .collect()
}
fn rank(a: &Mat) -> usize {
    let mut m = a.clone();
    let mut r = 0;
    for c in 0..m[0].len() {
        let Some(pivot) = (r..m.len()).find(|i| m[*i][c] != 0) else {
            continue;
        };
        m.swap(r, pivot);
        let v = inv(m[r][c]);
        for x in &mut m[r] {
            *x = mul(*x, v);
        }
        for i in 0..m.len() {
            if i != r {
                let v = m[i][c];
                for j in 0..m[i].len() {
                    m[i][j] = sub(m[i][j], mul(v, m[r][j]));
                }
            }
        }
        r += 1;
        if r == m.len() {
            break;
        }
    }
    r
}
fn next_permutation(v: &mut [usize]) -> bool {
    let Some(i) = (0..v.len().saturating_sub(1))
        .rev()
        .find(|i| v[*i] < v[*i + 1])
    else {
        return false;
    };
    let j = (i + 1..v.len()).rev().find(|j| v[*j] > v[i]).unwrap();
    v.swap(i, j);
    v[i + 1..].reverse();
    true
}
/// Independent Leibniz determinant; not Gaussian elimination.
fn det(a: &Mat) -> u64 {
    let n = a.len();
    assert!(a.iter().all(|r| r.len() == n));
    if n == 0 {
        return 1;
    }
    let mut perm: Vec<_> = (0..n).collect();
    let mut out = 0;
    loop {
        let term = (0..n).fold(1, |v, i| mul(v, a[i][perm[i]]));
        let inversions = (0..n)
            .map(|i| (i + 1..n).filter(|j| perm[i] > perm[*j]).count())
            .sum::<usize>();
        out = if inversions % 2 == 0 {
            add(out, term)
        } else {
            sub(out, term)
        };
        if !next_permutation(&mut perm) {
            break;
        }
    }
    out
}
fn adjugate(a: &Mat) -> Mat {
    let n = a.len();
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    // adj(i,j) deletes original row j and column i.
                    let minor: Mat = (0..n)
                        .filter(|r| *r != j)
                        .map(|r| (0..n).filter(|c| *c != i).map(|c| a[r][c]).collect())
                        .collect();
                    let d = det(&minor);
                    if (i + j) % 2 == 0 {
                        d
                    } else {
                        sub(0, d)
                    }
                })
                .collect()
        })
        .collect()
}
fn pairing(a: &Mat, b: &Mat) -> u64 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(0, |v, (a, b)| {
        assert_eq!(a.len(), b.len());
        add(v, a.iter().zip(b).fold(0, |v, (x, y)| add(v, mul(*x, *y))))
    })
}
fn reduce(s: &Mat, ell: &[u64]) -> Mat {
    let n = ell.len() - 1;
    assert_eq!(s.len(), ell.len());
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    add(
                        sub(
                            sub(s[i + 1][j + 1], mul(ell[i + 1], s[0][j + 1])),
                            mul(ell[j + 1], s[i + 1][0]),
                        ),
                        mul(mul(ell[i + 1], ell[j + 1]), s[0][0]),
                    )
                })
                .collect()
        })
        .collect()
}
fn identity(n: usize, c: u64) -> Mat {
    (0..n)
        .map(|i| (0..n).map(|j| if i == j { c } else { 0 }).collect())
        .collect()
}
fn check_adjugate(a: &Mat) {
    let d = det(a);
    let adj = adjugate(a);
    assert_eq!(mm(a, &adj), identity(a.len(), d));
    assert_eq!(mm(&adj, a), identity(a.len(), d));
}
fn control() -> Value {
    let f = vec![2, 2, 0, 1];
    let big_f = vec![16, 12, 8, 1];
    let h = vec![8, 1];
    let u = vec![12, 6, 11, 16];
    let v = vec![4, 6, 1];
    let b = scale(&h, 6);
    assert_eq!((big_f.len(), h.len(), f.len()), (4, 2, 4));
    assert_eq!(big_f.last(), Some(&1));
    assert_eq!(h.last(), Some(&1));
    assert_eq!(b, vec![14, 6]);
    assert_eq!(pa(&pm(&u, &big_f), &pm(&v, &pm(&f, &h))), vec![1]);
    assert_eq!(sub(eval(&big_f, 0), mul(6, eval(&b, 0))), 0);
    assert_ne!(eval(&big_f, 0), 0);
    assert_ne!(add(mul(4, power(2, 3)), mul(27 % P, power(2, 2))), 0);
    let lifts = [(0, 6), (3, 1), (5, 1), (6, 3), (7, 6), (10, 11), (13, 7)];
    for (x, y) in lifts {
        assert_ne!(y, 0);
        assert_eq!(mul(y, y), eval(&f, x));
    }
    let a_family: Vec<_> = (0..6)
        .map(|i| if i < 4 { mono(i) } else { vec![] })
        .collect();
    let b_family: Vec<_> = (0..6)
        .map(|i| if i >= 4 { mono(i - 4) } else { vec![] })
        .collect();
    let entries: Vec<Vec<Poly>> = (0..6)
        .map(|i| {
            (0..6)
                .map(|j| {
                    ps(
                        &pm(&a_family[i], &a_family[j]),
                        &pm(&f, &pm(&b_family[i], &b_family[j])),
                    )
                })
                .collect()
        })
        .collect();
    let ell: Vec<_> = (0..6)
        .map(|i| sub(eval(&a_family[i], 0), mul(6, eval(&b_family[i], 0))))
        .collect();
    assert_eq!(ell[0], 1);
    let s: Mat = entries
        .iter()
        .map(|r| {
            r.iter()
                .map(|p| coeff(&remainder(&pm(p, &v), &big_f), 2))
                .collect()
        })
        .collect();
    let t: Mat = (0..6)
        .map(|i| {
            (0..5)
                .map(|j| {
                    if i == 0 {
                        sub(0, ell[j + 1])
                    } else if i == j + 1 {
                        1
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let red = reduce(&s, &ell);
    assert_eq!(red, mm(&mm(&transpose(&t), &s), &t));
    assert_eq!(red, transpose(&red));
    assert_eq!(rank(&red), 4);
    assert_eq!(det(&red), 0);
    check_adjugate(&red);
    let adj = adjugate(&red);
    let pivot = (0..5)
        .find(|i| adj[*i][*i] != 0)
        .expect("nonzero diagonal cofactor");
    let column: Vec<_> = adj.iter().map(|r| r[pivot]).collect();
    let scalar = inv(adj[pivot][pivot]);
    for i in 0..5 {
        for j in 0..5 {
            assert_eq!(adj[i][j], mul(scalar, mul(column[i], column[j])));
        }
    }
    let w0 = vec![16, 12, 8, 1, 14, 6];
    let reduced_w0 = w0[1..].to_vec();
    assert_eq!(mv(&t, &reduced_w0), w0);
    assert_eq!(mv(&red, &reduced_w0), vec![0; 5]);
    let norm = ps(&pm(&big_f, &big_f), &pm(&f, &pm(&b, &b)));
    let positive_roots = [0, 3, 5, 6, 10, 13];
    let changed_roots = [0, 3, 5, 6, 7, 10];
    let positive = roots(&positive_roots);
    let changed = roots(&changed_roots);
    assert_eq!(norm, positive);
    assert_ne!(norm, changed);
    assert_eq!((positive.len(), changed.len()), (7, 7));
    assert_eq!(positive.last(), Some(&1));
    assert_eq!(changed.last(), Some(&1));
    assert_eq!(eval(&positive, 0), 0);
    assert_eq!(eval(&changed, 0), 0);
    let coefficient_matrix = |k: usize| -> Mat {
        entries
            .iter()
            .map(|r| r.iter().map(|p| coeff(p, k)).collect())
            .collect()
    };
    let leading = reduce(&coefficient_matrix(6), &ell);
    let lambda = pairing(&leading, &adj);
    assert_ne!(lambda, 0);
    let mut cases = Vec::new();
    for (name, target, xs, expected) in [
        ("positive", &positive, &positive_roots, true),
        ("changed_support", &changed, &changed_roots, false),
    ] {
        for (i, x) in xs.iter().enumerate() {
            assert!(lifts.iter().any(|(known_x, _)| known_x == x));
            for other in &xs[..i] {
                assert_ne!(x, other);
            }
        }
        let mut pairings = Vec::new();
        let mut residuals = Vec::new();
        for j in 0..6 {
            let direct: Mat = (0..6)
                .map(|i| {
                    (0..6)
                        .map(|k| {
                            sub(
                                coeff(&entries[i][k], j),
                                mul(coeff(&entries[i][k], 6), coeff(target, j)),
                            )
                        })
                        .collect()
                })
                .collect();
            let reduced = reduce(&direct, &ell);
            assert_eq!(reduced, mm(&mm(&transpose(&t), &direct), &t));
            pairings.push(pairing(&reduced, &adj));
            residuals.push(sub(coeff(&norm, j), mul(coeff(&norm, 6), coeff(target, j))));
        }
        assert_eq!(
            pairings,
            residuals
                .iter()
                .map(|r| mul(lambda, *r))
                .collect::<Vec<_>>()
        );
        assert_eq!(pairings.iter().all(|x| *x == 0), expected);
        cases.push(json!({"case":name,"roots":xs,"target_coefficients":target,"rank":rank(&red),"cofactor_pairings":pairings,"norm_residuals":residuals,"accepted_for_fixed_certificate":expected}));
    }
    json!({"schema":"norm.native-certificate-control/v1","p":P,"curve":{"a":2,"b":2},"branch":{"q":0,"y":6},
        "F":big_f,"H":h,"U":u,"V":v,"alpha":1,"beta":6,"branch_coefficients":ell,
        "full_moment_matrix":s,"insertion_matrix":t,"reduced_matrix":red,"reduced_rank":4,"determinant":0,
        "adjugate":adj,"pivot":pivot,"cofactor_scale":lambda,"norm_coefficients":norm,"cases":cases,
        "physical_artifact_invocations":1,"standing":"unsigned producer evidence","solver_executed":false,
        "scope":"fixed finite implementation regression; changed support is invalid for this certificate, not globally unsatisfiable; no DLP, timing, yield or asymptotic inference"})
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "--public-jets" {
        return public_jets::write(&args[2]);
    }
    if args.len() == 3 && args[1] == "--signed-hankel" {
        return signed_hankel::write(&args[2]);
    }
    if args.len() == 3 && args[1] == "--branch-repair" {
        return branch_repair::write(&args[2]);
    }
    if args.len() == 3 && args[1] == "--group-bridge" {
        return group_bridge::write(&args[2]);
    }
    if args.len() != 2 {
        return Err("usage: norm-certificate-control FRESH_OUTPUT_JSON".into());
    }
    check_adjugate(&vec![vec![1, 2], vec![3, 4]]);
    let result = control();
    let data = serde_json::to_vec_pretty(&result)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[1])?;
    file.write_all(&data)?;
    println!(
        "rank=4; positive pairings zero; changed-support pairings nonzero; no solver executed"
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonsymmetric_adjugate_orientation() {
        let a = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(det(&a), 15);
        assert_eq!(rank(&a), 2);
        assert_eq!(adjugate(&a), vec![vec![4, 15], vec![14, 1]]);
        check_adjugate(&a);
    }
    #[test]
    fn singular_rank_and_both_products() {
        let a = vec![vec![1, 2, 3], vec![2, 4, 6], vec![0, 0, 0]];
        assert_eq!(rank(&a), 1);
        assert_eq!(det(&a), 0);
        check_adjugate(&a);
    }
    #[test]
    fn fixed_actual_case_and_changed_valid_support() {
        control();
    }
}
