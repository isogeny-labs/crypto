//! Complete finite linear systems on five four-point del Pezzo models.
use crate::{gf2k::SmallField, lpoly, weil};
use serde_json::json;
type P = [u16; 3];
fn plane(q: usize) -> Vec<P> {
    (0..q)
        .flat_map(|x| (0..q).map(move |y| [1, x as u16, y as u16]))
        .chain((0..q).map(|y| [0, 1, y as u16]))
        .chain(std::iter::once([0, 0, 1]))
        .collect()
}
fn determinant(f: &SmallField, a: P, b: P, c: P) -> u16 {
    f.mul(a[0], f.mul(b[1], c[2]) ^ f.mul(b[2], c[1]))
        ^ f.mul(a[1], f.mul(b[0], c[2]) ^ f.mul(b[2], c[0]))
        ^ f.mul(a[2], f.mul(b[0], c[1]) ^ f.mul(b[1], c[0]))
}
fn general(f: &SmallField, pts: &[P]) -> bool {
    for i in 0..pts.len() {
        for j in i + 1..pts.len() {
            for k in j + 1..pts.len() {
                if determinant(f, pts[i], pts[j], pts[k]) == 0 {
                    return false;
                }
            }
        }
    }
    true
}
fn config(f: &SmallField, partition: &[usize]) -> Vec<P> {
    let mut orbits = Vec::new();
    let mut seen = Vec::new();
    for p in plane(f.q) {
        if seen.contains(&p) {
            continue;
        }
        let mut orbit = vec![p];
        let mut q = p.map(|x| f.sqr(x));
        while q != p {
            orbit.push(q);
            q = q.map(|x| f.sqr(x));
        }
        seen.extend(orbit.clone());
        orbits.push(orbit);
    }
    fn choose(f: &SmallField, orbits: &[Vec<P>], parts: &[usize], pts: Vec<P>) -> Option<Vec<P>> {
        if parts.is_empty() {
            return Some(pts);
        }
        for orbit in orbits {
            if orbit.len() != parts[0] || orbit.iter().any(|p| pts.contains(p)) {
                continue;
            }
            let mut next = pts.clone();
            next.extend(orbit);
            if general(f, &next) {
                if let Some(done) = choose(f, orbits, &parts[1..], next) {
                    return Some(done);
                }
            }
        }
        None
    }
    choose(f, &orbits, partition, Vec::new()).expect("general-position four-point configuration")
}
fn monomials() -> Vec<[usize; 3]> {
    (0..=6)
        .flat_map(|i| (0..=6 - i).map(move |j| [i, j, 6 - i - j]))
        .collect()
}
fn jet(f: &SmallField, p: P, m: [usize; 3], dx: usize, dy: usize) -> u16 {
    let fixed = p.iter().position(|x| *x != 0).unwrap();
    let vars: Vec<_> = (0..3).filter(|i| *i != fixed).collect();
    let (a, b) = (m[vars[0]], m[vars[1]]);
    if dx > a || dy > b || dx & a != dx || dy & b != dy {
        return 0;
    }
    f.mul(
        f.pow(p[vars[0]], (a - dx) as u64),
        f.pow(p[vars[1]], (b - dy) as u64),
    )
}
fn basis(f: &SmallField, pts: &[P], monos: &[[usize; 3]]) -> Vec<u32> {
    let mut rows = Vec::new();
    for p in pts {
        for (dx, dy) in [(0, 0), (1, 0), (0, 1)] {
            let vals: Vec<_> = monos.iter().map(|m| jet(f, *p, *m, dx, dy)).collect();
            for k in 0..f.k {
                let mask = vals
                    .iter()
                    .enumerate()
                    .fold(0u32, |a, (i, v)| a | (((v >> k) & 1) as u32) << i);
                rows.push(mask);
            }
        }
    }
    let mut pivots = Vec::new();
    for col in 0..28 {
        let Some(p) = (pivots.len()..rows.len()).find(|i| rows[*i] >> col & 1 == 1) else {
            continue;
        };
        let r = pivots.len();
        rows.swap(p, r);
        for i in 0..rows.len() {
            if i != r && rows[i] >> col & 1 == 1 {
                rows[i] ^= rows[r];
            }
        }
        pivots.push(col);
    }
    assert_eq!(pivots.len(), 12);
    let mut basis = Vec::new();
    for free in (0..28).filter(|c| !pivots.contains(c)) {
        let mut mask = 1 << free;
        for (i, c) in pivots.iter().enumerate() {
            if rows[i] >> free & 1 == 1 {
                mask |= 1 << c;
            }
        }
        assert!(rows.iter().all(|r| (r & mask).count_ones() % 2 == 0));
        basis.push(mask);
    }
    assert_eq!(basis.len(), 16);
    basis
}
fn embedding(fd: &SmallField, f: &SmallField) -> Vec<u16> {
    assert_eq!(f.k % fd.k, 0);
    let root = (0..f.q as u16)
        .find(|x| {
            let mut value = 0;
            for i in (0..=fd.k).rev() {
                value = f.mul(value, *x) ^ ((fd.poly >> i) & 1) as u16;
            }
            value == 0
        })
        .unwrap();
    (0..fd.q)
        .map(|a| {
            let mut value = 0;
            for i in (0..fd.k).rev() {
                value = f.mul(value, root) ^ ((a >> i) & 1) as u16;
            }
            value
        })
        .collect()
}
fn coordinates(fd: &SmallField, f: &SmallField, pts: &[P], partition: &[usize]) -> Vec<P> {
    let mut rational = Vec::new();
    let mut pos = 0;
    for degree in partition {
        if f.k as usize % degree == 0 {
            let sub = SmallField::new(*degree as u32);
            // Convert from the configuration field to its degree-d subfield by matching powers.
            let into_fd = embedding(&sub, fd);
            let into_f = embedding(&sub, f);
            for p in &pts[pos..pos + degree] {
                rational.push(p.map(|v| {
                    let index = into_fd.iter().position(|x| *x == v).unwrap();
                    into_f[index]
                }));
            }
        }
        pos += degree;
    }
    rational
}
fn evaluate_basis(mono_values: &[u16], basis: &[u32]) -> Vec<u16> {
    basis
        .iter()
        .map(|mask| {
            mono_values
                .iter()
                .enumerate()
                .filter(|(i, _)| mask >> i & 1 == 1)
                .fold(0, |a, (_, v)| a ^ v)
        })
        .collect()
}
fn surface_values(
    f: &SmallField,
    points: &[P],
    basis: &[u32],
    monos: &[[usize; 3]],
) -> Vec<Vec<u16>> {
    let mut values = Vec::new();
    for p in plane(f.q) {
        if points.contains(&p) {
            continue;
        }
        let mv: Vec<_> = monos
            .iter()
            .map(|m| {
                f.mul(
                    f.mul(f.pow(p[0], m[0] as u64), f.pow(p[1], m[1] as u64)),
                    f.pow(p[2], m[2] as u64),
                )
            })
            .collect();
        values.push(evaluate_basis(&mv, basis));
    }
    for p in points {
        for (s, t) in (0..f.q as u16)
            .map(|t| (1, t))
            .chain(std::iter::once((0, 1)))
        {
            let mv: Vec<_> = monos
                .iter()
                .map(|m| {
                    f.mul(jet(f, *p, *m, 2, 0), f.sqr(s))
                        ^ f.mul(jet(f, *p, *m, 1, 1), f.mul(s, t))
                        ^ f.mul(jet(f, *p, *m, 0, 2), f.sqr(t))
                })
                .collect();
            values.push(evaluate_basis(&mv, basis));
        }
    }
    assert_eq!(values.len(), f.q * f.q + f.q + 1 + points.len() * f.q);
    values
}
fn counts(f: &SmallField, values: &[Vec<u16>]) -> Vec<i16> {
    let mut hist = vec![0i64; 1 << 16];
    for val in values {
        for u in 0..f.q as u16 {
            let mask = val.iter().enumerate().fold(0usize, |a, (i, v)| {
                a | ((f.trace(f.mul(u, *v)) as usize) << i)
            });
            hist[mask] += 1;
        }
    }
    let mut step = 1;
    while step < hist.len() {
        for start in (0..hist.len()).step_by(2 * step) {
            for i in 0..step {
                let a = hist[start + i];
                let b = hist[start + i + step];
                hist[start + i] = a + b;
                hist[start + i + step] = a - b;
            }
        }
        step *= 2;
    }
    hist.into_iter()
        .map(|a| {
            assert_eq!(a % f.q as i64, 0);
            (a / f.q as i64) as i16
        })
        .collect()
}
pub fn run(path: &std::path::Path) {
    assert!(!path.exists());
    let monos = monomials();
    let mut families = Vec::new();
    for partition in [
        vec![1, 1, 1, 1],
        vec![2, 1, 1],
        vec![2, 2],
        vec![3, 1],
        vec![4],
    ] {
        let fd = SmallField::new(*partition.iter().max().unwrap() as u32);
        let points = config(&fd, &partition);
        let basis = basis(&fd, &points, &monos);
        let mut vectors = Vec::new();
        let mut controls = Vec::new();
        for k in 1..=6 {
            let f = SmallField::new(k);
            let rational = coordinates(&fd, &f, &points, &partition);
            let values = surface_values(&f, &rational, &basis, &monos);
            let ns = counts(&f, &values);
            for bits in [0, 1, 31, 137, 12345, 65535] {
                let direct = values
                    .iter()
                    .filter(|v| {
                        v.iter()
                            .enumerate()
                            .filter(|(i, _)| bits >> i & 1 == 1)
                            .fold(0, |a, (_, x)| a ^ x)
                            == 0
                    })
                    .count();
                assert_eq!(direct, ns[bits] as usize);
                controls.push(json!({"k":k,"section":bits,"count":direct}));
            }
            vectors.push(ns);
        }
        let mut candidates = Vec::new();
        for bits in 1..1 << 16 {
            let ns: Vec<_> = vectors.iter().map(|v| v[bits] as i64).collect();
            let Some(p) = lpoly::char_poly_from_counts(&ns, 2) else {
                continue;
            };
            for t in [-1, 1] {
                if let Some(q) =
                    lpoly::divide_exact(&p, &weil::to_i128(&weil::trace_zero_poly(t, 5)))
                {
                    let plane_form = basis
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| bits >> i & 1 == 1)
                        .fold(0, |a, (_, b)| a ^ b);
                    candidates.push(json!({"section":bits,"plane_form_bits":plane_form,"trace":t,"counts":ns,"polynomial":p,"cofactor":q,"strict_transform_smoothness":"not yet checked"}));
                }
            }
        }
        println!(
            "LC2 {:?}: 65535 sections, {} polynomial candidates",
            partition,
            candidates.len()
        );
        families.push(json!({"frobenius_cycle_type":partition,"configuration_field_modulus":fd.poly,"points":points,"monomials":monos,"basis_masks":basis,"forms_tested":65535,"direct_controls":controls,"candidates":candidates}));
    }
    std::fs::write(path,serde_json::to_string_pretty(&json!({"schema":"koblitz.experiment/lc2-v1","families":families,"forms_tested_total":5*65535,"scope":"five blowup-of-four-points del Pezzo families; all F2 sections of -2K; two other S5 twist types and unbalanced trigonal curves not covered"})).unwrap()+"\n").unwrap();
}
