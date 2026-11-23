#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/field.rs"]
mod field;
use field::{scalar, E, F, Z};
fn nums(s: &str) -> Vec<u64> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
fn sqroot(f: &F, a: E, z: E) -> E {
    if a == Z {
        return Z;
    }
    assert_eq!(f.pow(a, (f.q - 1) / 2), scalar(1));
    let mut q = f.q - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1
    }
    let (mut c, mut x, mut t, mut m) = (f.pow(z, q), f.pow(a, (q + 1) / 2), f.pow(a, q), s);
    while t != scalar(1) {
        let mut i = 0;
        let mut tt = t;
        while tt != scalar(1) {
            tt = f.mul(tt, tt);
            i += 1;
            assert!(i < m)
        }
        let b = f.pow(c, 1u64 << (m - i - 1));
        x = f.mul(x, b);
        c = f.mul(b, b);
        t = f.mul(t, c);
        m = i
    }
    assert_eq!(f.mul(x, x), a);
    x.min(f.sub(Z, x))
}
fn sign(f: &F, a: E, m: usize) -> E {
    if m.count_ones() % 2 == 1 {
        f.sub(Z, a)
    } else {
        a
    }
}
fn array(s: &str) -> Vec<E> {
    nums(s)
        .chunks_exact(9)
        .map(|v| v.try_into().unwrap())
        .collect()
}
fn clifford(f: &F, roots: &[E], basis: &[usize], plane: &[field::P]) -> Vec<Vec<E>> {
    let mut z = Z;
    z[1] = 1;
    let mut rows = vec![vec![Z; 16]; 64];
    for (i, &r) in roots.iter().enumerate() {
        let wi = if i == 8 { 255 } else { 1 << i };
        let coord = (0usize..256)
            .find(|&s| {
                basis
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| s & (1 << j) != 0)
                    .fold(0, |a, (_, b)| a ^ b)
                    == wi
            })
            .unwrap();
        let (x, b) = (coord & 15, coord >> 4);
        let derivative = roots
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .fold(scalar(1), |a, (_, &s)| f.mul(a, f.sub(r, s)));
        let weight = sqroot(&f, sign(&f, f.inv(derivative), x & b), z);
        for k in 0..4 {
            for col in 0..16 {
                let entry = &mut rows[k * 16 + (col ^ x)][col];
                *entry = f.add(
                    *entry,
                    sign(&f, f.mul(weight, f.eval(&plane[k], r)), b & col),
                );
            }
        }
    }
    rows
}
#[allow(dead_code)]
#[path = "../hec_g4_four_torsion_20261118/jac.rs"]
mod jac;
use jac::{C, D};
fn plane(c: &C, d: &D) -> Vec<field::P> {
    let f = c.f;
    let m = d.u.len() - 1;
    let v = c.exact(&f.ps(&c.ff, &f.pm(&d.v, &d.v)), &d.u);
    let mut rows = Vec::new();
    for i in 0..4 - m {
        let mut p = vec![Z; i];
        p.extend(&d.u);
        rows.push(p);
    }
    for i in 0..m {
        let mut xv = vec![Z; i];
        xv.extend(&v);
        let mut xr = vec![Z; i];
        xr.extend(&d.v);
        let (_, a) = f.div(&xv, &d.u);
        let (q, _) = f.div(&c.scale(&xr, scalar(29)), &d.u);
        rows.push(f.pa(&f.pa(&c.scale(&xv, scalar(29)), &a), &f.pm(&q, &d.v)));
    }
    assert_eq!(rows.len(), 4);
    rows
}
fn null(f: &F, mut a: Vec<Vec<E>>, n: usize) -> Vec<Vec<E>> {
    let original = a.clone();
    let mut piv = Vec::new();
    for j in 0..n {
        let k = piv.len();
        let Some(i) = (k..a.len()).find(|&i| a[i][j] != Z) else {
            continue;
        };
        a.swap(k, i);
        let iv = f.inv(a[k][j]);
        let row: Vec<_> = a[k].iter().map(|&x| f.mul(x, iv)).collect();
        for (i, r) in a.iter_mut().enumerate() {
            if i == k {
                continue;
            }
            let t = r[j];
            if t == Z {
                continue;
            }
            for l in j..n {
                r[l] = f.sub(r[l], f.mul(t, row[l]));
            }
        }
        a[k] = row;
        piv.push(j);
    }
    (0..n)
        .filter(|j| !piv.contains(j))
        .map(|j| {
            let mut v = vec![Z; n];
            v[j] = scalar(1);
            for (i, &k) in piv.iter().enumerate() {
                v[k] = f.sub(Z, a[i][j]);
            }
            for r in &original {
                assert_eq!(dot(f, r, &v), Z);
            }
            v
        })
        .collect()
}
fn dot(f: &F, x: &[E], y: &[E]) -> E {
    x.iter().zip(y).fold(Z, |a, (&b, &c)| f.add(a, f.mul(b, c)))
}
fn normalize(f: &F, v: &mut [E]) {
    let iv = f.inv(*v.iter().find(|&&x| x != Z).unwrap());
    for x in v {
        *x = f.mul(*x, iv);
    }
}
fn shifted(v: &field::P, i: usize) -> field::P {
    let mut w = vec![Z; i];
    w.extend(v);
    field::trim(w)
}
fn jet(c: &C, d: &D, j: usize) -> Vec<field::P> {
    let f = c.f;
    let u = &d.u;
    let r = &d.v;
    assert_eq!(u.len(), 5);
    let v = c.exact(&f.ps(&c.ff, &f.pm(r, r)), u);
    let h = u[j + 1..].to_vec();
    let du = f.rem(&c.scale(&f.pm(r, &h), scalar(29)), u);
    let dr = f.rem(&f.pm(&v, &h), u);
    let dv = c.exact(
        &f.ps(&c.scale(&f.pm(r, &dr), scalar(29)), &f.pm(&v, &du)),
        u,
    );
    for k in 0..4 {
        let residue = f.rem(&shifted(&h, k), u);
        assert_eq!(*residue.get(3).unwrap_or(&Z), scalar((k == j) as u64));
    }
    (0..4)
        .map(|i| {
            let (p, _) = f.div(&shifted(&v, i), u);
            let (q, _) = f.div(&c.scale(&shifted(r, i), scalar(29)), u);
            let dxv = shifted(&dv, i);
            let da = f.rem(&f.ps(&dxv, &f.pm(&p, &du)), u);
            let (dq, _) = f.div(
                &f.ps(&c.scale(&shifted(&dr, i), scalar(29)), &f.pm(&q, &du)),
                u,
            );
            f.pa(
                &f.pa(&c.scale(&dxv, scalar(29)), &da),
                &f.pa(&f.pm(&dq, r), &f.pm(&q, &dr)),
            )
        })
        .collect()
}
fn fourier(f: &F, v: &[E], b: usize) -> E {
    v.iter()
        .enumerate()
        .fold(Z, |s, (i, &x)| f.add(s, sign(f, x, i & b)))
}
fn gradients(c: &C, roots: &[E], basis: &[usize], d: &D) -> (Vec<E>, Vec<Vec<E>>) {
    let f = c.f;
    let rows = clifford(f, roots, basis, &plane(c, d));
    let ker = null(f, rows.clone(), 16);
    assert_eq!(ker.len(), 1);
    let v = ker[0].clone();
    let fix = v.iter().position(|&x| x != Z).unwrap();
    let grad = (0..4)
        .map(|j| {
            let dm = clifford(f, roots, basis, &jet(c, d, j));
            let mut eq = rows.clone();
            for (r, m) in eq.iter_mut().zip(dm) {
                r.push(dot(f, &m, &v));
            }
            let mut gauge = vec![Z; 17];
            gauge[fix] = scalar(1);
            eq.push(gauge);
            let sol = null(f, eq, 17);
            assert_eq!(sol.len(), 1);
            let scale = f.inv(sol[0][16]);
            sol[0][..16].iter().map(|&x| f.mul(x, scale)).collect()
        })
        .collect();
    (v, grad)
}
fn product(f: &F, a: &[E], b: &[E]) -> Vec<E> {
    let mut v = vec![];
    for i in 0..4 {
        for j in i..4 {
            let t = f.mul(a[i], b[j]);
            v.push(if i == j {
                t
            } else {
                f.add(t, f.mul(a[j], b[i]))
            });
        }
    }
    v
}
fn quadric(f: &F, planes: &[Option<Vec<E>>]) {
    let mut equations = Vec::new();
    let mut profile = [0usize; 11];
    for delta in 1..256 {
        let pairs: Vec<_> = (0..256)
            .filter(|&i| i < (i ^ delta))
            .filter_map(|i| Some(product(f, planes[i].as_ref()?, planes[i ^ delta].as_ref()?)))
            .collect();
        assert_eq!(pairs.len(), 28);
        let orth = null(f, pairs, 10);
        let rank = 10 - orth.len();
        assert!(rank <= 7);
        profile[rank] += 1;
        if rank == 7 {
            equations.extend(orth);
        }
    }
    let mut q = null(f, equations, 10);
    println!("STEINER ranks={profile:?} intersection={}", q.len());
    assert_eq!(q.len(), 1);
    normalize(f, &mut q[0]);
    let q = &q[0];
    assert!(q.iter().all(|x| x[1..].iter().all(|&v| v == 0)));
    let mut mat = [[Z; 4]; 4];
    let mut k = 0;
    for i in 0..4 {
        for j in i..4 {
            mat[i][j] = if i == j {
                q[k]
            } else {
                f.mul(q[k], scalar(16))
            };
            mat[j][i] = mat[i][j];
            k += 1;
        }
    }
    let mut det = scalar(1);
    for j in 0..4 {
        let i = (j..4).find(|&i| mat[i][j] != Z).unwrap();
        if i != j {
            mat.swap(i, j);
            det = f.sub(Z, det);
        }
        let pivot = mat[j][j];
        det = f.mul(det, pivot);
        let iv = f.inv(pivot);
        for i in j + 1..4 {
            let t = f.mul(mat[i][j], iv);
            for k in j..4 {
                mat[i][k] = f.sub(mat[i][k], f.mul(t, mat[j][k]));
            }
        }
    }
    let points = (1u64..31u64.pow(4))
        .filter(|&n| {
            let v = (0..4)
                .map(|i| scalar(n / 31u64.pow(i) % 31))
                .collect::<Vec<_>>();
            if v.iter().find(|&&x| x != Z) != Some(&scalar(1)) {
                return false;
            }
            let mut h = Z;
            let mut k = 0;
            for i in 0..4 {
                for j in i..4 {
                    h = f.add(h, f.mul(q[k], f.mul(v[i], v[j])));
                    k += 1;
                }
            }
            h == Z
        })
        .count();
    let split = f.pow(det, 15) == scalar(1);
    assert_eq!(points, if split { 1024 } else { 962 });
    println!(
        "QUADRIC coeff={:?} determinant={} split={split} points={points}",
        q.iter().map(|x| x[0]).collect::<Vec<_>>(),
        det[0]
    );
}
fn main() {
    let raw = include_str!("../hec_g4_four_torsion_20261118/CONTROL.txt");
    let first = raw.lines().next().unwrap();
    let roots = array(first.split("roots=").nth(1).unwrap());
    let f = F::new(
        31,
        nums(
            first
                .split("modulus=")
                .nth(1)
                .unwrap()
                .split(']')
                .next()
                .unwrap(),
        ),
    );
    let c = C {
        f: &f,
        ff: field::poly(&[1, 16, 0, 1, 0, 0, 0, 0, 0, 1]),
        g: 4,
    };
    let line = include_str!("../hec_g4_spinor_marking_20261118/panel/CONTROL.txt")
        .lines()
        .find(|s| s.starts_with("KERNEL id=2 "))
        .unwrap();
    let basis: Vec<_> = nums(
        line.split("basis=")
            .nth(1)
            .unwrap()
            .split(']')
            .next()
            .unwrap(),
    )
    .into_iter()
    .map(|u| u as usize)
    .collect();
    let halves: Vec<_> = raw
        .lines()
        .filter(|s| s.starts_with("SELECTED "))
        .take(8)
        .map(|s| {
            let (u, v) = s.split(" u=").nth(1).unwrap().split_once(" v=").unwrap();
            D {
                u: array(u),
                v: array(v),
            }
        })
        .collect();
    let origin = null(&f, clifford(&f, &roots, &basis, &plane(&c, &c.zero())), 16);
    assert_eq!(
        origin[0],
        array(line.split("origin_vector=").nth(1).unwrap())
    );
    for b in 0..16 {
        assert_ne!(fourier(&f, &origin[0], b), Z);
    }
    let generators: Vec<_> = basis[..4]
        .iter()
        .map(|coeff| {
            (0..8)
                .filter(|i| coeff & (1 << i) != 0)
                .fold(c.zero(), |a, i| c.add(&a, &halves[i]))
        })
        .collect();
    let mut planes = vec![None; 256];
    for mask in 1usize..16 {
        let p = (0..4)
            .filter(|i| mask & (1 << i) != 0)
            .fold(c.zero(), |a, i| c.add(&a, &generators[i]));
        let (v, grad) = gradients(&c, &roots, &basis, &p);
        let mut hits = 0;
        for b in 0..16 {
            if fourier(&f, &v, b) != Z {
                continue;
            }
            let mut l: Vec<_> = grad.iter().map(|g| fourier(&f, g, b)).collect();
            normalize(&f, &mut l);
            let label = mask + 16 * b;
            println!("PLANE label={label} form={l:?}");
            planes[label] = Some(l);
            hits += 1;
        }
        assert_eq!(hits, 8);
    }
    let distinct = planes
        .iter()
        .flatten()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(distinct.len(), 120);
    for l in &distinct {
        let conjugate = l.iter().map(|&v| f.pow(v, 31)).collect::<Vec<_>>();
        assert!(distinct.contains(&conjugate));
    }
    quadric(&f, &planes);
}
