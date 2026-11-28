#[path = "../canonical-root/fast.rs"]
mod fast;
#[path = "../canonical-root/root.rs"]
mod root;
use hec_native_ic::{counted, fp::Fp, jac::Curve, poly::Poly};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
type M = Vec<Vec<i128>>;
type V = [i128; 4];
fn id() -> M {
    (0..4)
        .map(|i| (0..4).map(|j| (i == j) as i128).collect())
        .collect()
}
fn tr(a: &M) -> M {
    (0..4).map(|j| (0..4).map(|i| a[i][j]).collect()).collect()
}
fn mm(a: &M, b: &M) -> M {
    (0..4)
        .map(|i| {
            (0..4)
                .map(|j| (0..4).map(|k| a[i][k] * b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn det(mut a: M) -> i128 {
    let (mut den, mut sign) = (1, 1);
    for k in 0..3 {
        let Some(p) = (k..4).find(|&i| a[i][k] != 0) else {
            return 0;
        };
        if p != k {
            a.swap(p, k);
            sign = -sign
        }
        let pivot = a[k][k];
        for i in k + 1..4 {
            for j in k + 1..4 {
                let v = a[i][j] * pivot - a[i][k] * a[k][j];
                assert_eq!(v % den, 0);
                a[i][j] = v / den
            }
            a[i][k] = 0;
        }
        den = pivot;
    }
    sign * a[3][3]
}
fn op(a: V) -> M {
    let mut m = vec![vec![0; 4]; 4];
    for j in 0..4 {
        let mut v = [0; 7];
        for i in 0..4 {
            v[i + j] += a[i]
        }
        for k in (4..7).rev() {
            let c = v[k];
            v[k] = 0;
            for i in 0..4 {
                v[k - 4 + i] -= c;
            }
        }
        for i in 0..4 {
            m[i][j] = v[i]
        }
    }
    m
}
fn km(a: V, b: V) -> V {
    let m = op(a);
    std::array::from_fn(|i| (0..4).map(|j| m[i][j] * b[j]).sum())
}
fn bar(a: V) -> V {
    let mut b = [0; 5];
    for j in 0..4 {
        b[(5 - j) % 5] += a[j]
    }
    std::array::from_fn(|i| b[i] - b[4])
}
fn gram(b: &M, g: &M) -> M {
    mm(&mm(&tr(b), g), b)
}
fn rref(mut a: Vec<Vec<u64>>, f: &Fp) -> (Vec<Vec<u64>>, Vec<usize>) {
    let mut piv = vec![];
    for j in 0..4 {
        let Some(i) = (piv.len()..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        let r = piv.len();
        a.swap(r, i);
        let inv = f.inv(a[r][j]);
        for k in j..4 {
            a[r][k] = f.mul(a[r][k], inv)
        }
        for i in 0..a.len() {
            if i == r {
                continue;
            }
            let c = a[i][j];
            for k in j..4 {
                a[i][k] = f.sub(a[i][k], f.mul(c, a[r][k]))
            }
        }
        piv.push(j);
        if piv.len() == a.len() {
            break;
        }
    }
    (a, piv)
}
fn canon(a: Vec<Vec<u64>>, f: &Fp) -> Vec<Vec<u64>> {
    let (a, p) = rref(a, f);
    a[..p.len()].to_vec()
}
fn mv(a: &M, v: &[u64], f: &Fp) -> Vec<u64> {
    (0..4)
        .map(|i| {
            (0..4).fold(0, |s, j| {
                f.add(s, f.mul(a[i][j].rem_euclid(f.p as i128) as u64, v[j]))
            })
        })
        .collect()
}
fn pairing(a: &[u64], b: &[u64], g: &M, f: &Fp) -> u64 {
    (0..4).fold(0, |s, i| {
        (0..4).fold(s, |s, j| {
            f.add(
                s,
                f.mul(f.mul(a[i], b[j]), g[i][j].rem_euclid(f.p as i128) as u64),
            )
        })
    })
}
fn transformed(m: &M, b: &M) -> Option<M> {
    solve_left(b, &mm(m, b))
}
fn solve_left(b: &M, target: &M) -> Option<M> {
    let divisor = det(b.clone());
    let mut out = vec![vec![0; 4]; 4];
    for j in 0..4 {
        for i in 0..4 {
            let mut a = b.clone();
            for k in 0..4 {
                a[k][i] = target[k][j]
            }
            let v = det(a);
            if v % divisor != 0 {
                return None;
            }
            out[i][j] = v / divisor;
        }
    }
    Some(out)
}
fn egcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    let (mut x, mut y, mut u, mut v) = (1, 0, 0, 1);
    while b != 0 {
        let q = a / b;
        (a, b) = (b, a - q * b);
        (x, u) = (u, x - q * u);
        (y, v) = (v, y - q * v);
    }
    if a < 0 {
        (-a, -x, -y)
    } else {
        (a, x, y)
    }
}
fn symplectic(g: &M) -> M {
    let (mut a, mut t) = (g.clone(), id());
    for r in [0usize, 2] {
        let j = r + 1;
        for k in j + 1..4 {
            let (av, bv) = (a[r][j], a[r][k]);
            if bv == 0 {
                continue;
            }
            let (d, x, y) = egcd(av, bv);
            let mut u = id();
            u[j][j] = x;
            u[k][j] = y;
            u[j][k] = -bv / d;
            u[k][k] = av / d;
            a = gram(&u, &a);
            t = mm(&t, &u);
        }
        if a[r][j] == -1 {
            let mut u = id();
            u[j][j] = -1;
            a = gram(&u, &a);
            t = mm(&t, &u);
        }
        assert_eq!(a[r][j], 1);
        for k in j + 1..4 {
            let mut u = id();
            u[r][k] = a[j][k];
            a = gram(&u, &a);
            t = mm(&t, &u);
        }
    }
    assert_eq!(
        a,
        vec![
            vec![0, 1, 0, 0],
            vec![-1, 0, 0, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, -1, 0]
        ]
    );
    assert_eq!(det(t.clone()).abs(), 1);
    t
}
fn descriptor(label: &str, b: &M, den: i128, g: &M, phi: &[u64]) {
    let t = symplectic(g);
    let c = mm(b, &t);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(format!(
            "research/hec_residual_span_20261116/kernel19-lattice-v2/{label}.dat"
        ))
        .unwrap();
    writeln!(file, "2\n{} {}\n{den}", phi[0], phi[1]).unwrap();
    for row in c {
        writeln!(
            file,
            "{}",
            row.iter()
                .map(i128::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    }
}
fn main() {
    counted::disable();
    let c = Curve::new(Fp::new(251), Poly::from_coeffs(&[18, 0, 0, 0, 0, 1]));
    let (_, z, a) = fast::prepare(&c);
    let pi: V = std::array::from_fn(|i| a[i] as i128 - a[4] as i128);
    let fmat = op(pi);
    let vmat = op(bar(pi));
    assert_eq!(
        mm(&fmat, &vmat),
        id().iter()
            .map(|r| r.iter().map(|v| 251 * v).collect())
            .collect::<M>()
    );
    let mut phi = vec![];
    for e in 1..5 {
        let r = c.fp.pow(z, e);
        let x = a.iter().rev().fold(0, |s, &v| {
            c.fp.add(c.fp.mul(s, r), v.rem_euclid(251) as u64)
        });
        if x == 0 {
            phi.push(e)
        }
    }
    assert_eq!(phi.len(), 2);
    let exponent = (1..5)
        .find(|&j| phi.iter().all(|&e| (1..=2).contains(&((j * e) % 5))))
        .unwrap();
    let mut xi = [0; 5];
    xi[exponent as usize] = 1;
    xi[(5 - exponent) as usize] = -1;
    let xi: V = std::array::from_fn(|i| xi[i] - xi[4]);
    let g: M = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    let mut x = [0; 4];
                    let mut y = [0; 4];
                    x[i] = 1;
                    y[j] = 1;
                    let v = km(xi, km(x, bar(y)));
                    let t = 4 * v[0] - v[1..].iter().sum::<i128>();
                    assert_eq!(t % 5, 0);
                    t / 5
                })
                .collect()
        })
        .collect();
    assert_eq!(det(g.clone()), 1);
    assert_eq!(
        gram(&fmat, &g),
        g.iter()
            .map(|r| r.iter().map(|v| 251 * v).collect())
            .collect::<M>()
    );
    let sigma = op([0, 1, 0, 0]);
    assert_eq!(gram(&sigma, &g), g);
    let fp = Fp::new(19);
    let amat: M = fmat
        .iter()
        .map(|r| r.iter().map(|v| ((*v).rem_euclid(19) * 10) % 19).collect())
        .collect();
    let aa = mm(&amat, &amat);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(aa[i][j].rem_euclid(19), if i == j { 18 } else { 0 });
        }
    }
    let ag = gram(&amat, &g);
    for i in 0..4 {
        for j in 0..4 {
            assert_eq!(ag[i][j].rem_euclid(19), g[i][j].rem_euclid(19));
        }
    }
    let u = vec![1, 0, 0, 0];
    let au = mv(&amat, &u, &fp);
    let (v, av) = (0..4)
        .find_map(|j| {
            let mut v = vec![0; 4];
            v[j] = 1;
            let av = mv(&amat, &v, &fp);
            if rref(vec![u.clone(), au.clone(), v.clone(), av.clone()], &fp)
                .1
                .len()
                == 4
            {
                Some((v, av))
            } else {
                None
            }
        })
        .unwrap();
    let mut all = BTreeSet::new();
    let mut kernels = BTreeMap::new();
    for parameter in 0..=361 {
        let w = if parameter == 361 {
            v.clone()
        } else {
            let (a, b) = ((parameter / 19) as u64, (parameter % 19) as u64);
            (0..4)
                .map(|i| fp.add(u[i], fp.add(fp.mul(a, v[i]), fp.mul(b, av[i]))))
                .collect()
        };
        let aw = mv(&amat, &w, &fp);
        let h = canon(vec![w.clone(), aw.clone()], &fp);
        assert_eq!(h.len(), 2);
        assert!(all.insert(h.clone()));
        if pairing(&w, &aw, &g, &fp) == 0 {
            for x in &h {
                for y in &h {
                    assert_eq!(pairing(x, y, &g, &fp), 0)
                }
            }
            let mut hh = h.clone();
            for x in &h {
                hh.push(mv(&fmat, x, &fp));
            }
            assert_eq!(rref(hh, &fp).1.len(), 2);
            let mut hs = h.clone();
            for x in &h {
                hs.push(mv(&sigma, x, &fp));
            }
            assert_eq!(rref(hs, &fp).1.len(), 4);
            kernels.insert(h, parameter);
        }
    }
    assert_eq!(all.len(), 362);
    assert_eq!(kernels.len(), 20);
    println!("SOURCE\tp=251\tc=18\tcm_type={phi:?}\txi_exponent={exponent}\tE={g:?}\tF={fmat:?}\tHermitian_lines=362\tisotropic=20");
    descriptor("source", &id(), 1, &g, &phi);
    let mut unseen: BTreeSet<_> = kernels.keys().cloned().collect();
    let mut reps = vec![];
    while let Some(h) = unseen.iter().next().cloned() {
        let mut x = h.clone();
        let mut orbit = vec![];
        for _ in 0..5 {
            assert!(unseen.remove(&x));
            orbit.push(kernels[&x]);
            x = canon(x.iter().map(|v| mv(&sigma, v, &fp)).collect(), &fp);
        }
        assert_eq!(x, h);
        println!("ORBIT\tparameters={orbit:?}");
        reps.push(h);
    }
    assert_eq!(reps.len(), 4);
    let mut order_constraints = None;
    for (h, parameter) in &kernels {
        let mut constraints = vec![];
        for x in h {
            for y in h {
                constraints.push(
                    (0..4)
                        .map(|j| {
                            let mut k = [0; 4];
                            k[j] = 1;
                            pairing(&mv(&op(k), x, &fp), y, &g, &fp)
                        })
                        .collect(),
                );
            }
        }
        let a = canon(constraints, &fp);
        assert_eq!(a.len(), 2);
        if let Some(ref first) = order_constraints {
            assert_eq!(first, &a)
        } else {
            order_constraints = Some(a)
        }
        println!("KERNEL\tparameter={parameter}\tbasis={h:?}");
    }
    for (index, h) in reps.iter().enumerate() {
        let (h, piv) = rref(h.clone(), &fp);
        let columns: Vec<V> = (0..4)
            .map(|j| {
                if let Some(i) = piv.iter().position(|&p| p == j) {
                    std::array::from_fn(|k| h[i][k] as i128)
                } else {
                    let mut a = [0; 4];
                    a[j] = 19;
                    a
                }
            })
            .collect();
        let b: M = (0..4)
            .map(|i| (0..4).map(|j| columns[j][i]).collect())
            .collect();
        assert_eq!(det(b.clone()).abs(), 361);
        let mut gg = gram(&b, &g);
        for row in &mut gg {
            for x in row {
                assert_eq!(*x % 19, 0);
                *x /= 19
            }
        }
        assert_eq!(det(gg.clone()), 1);
        let ff = transformed(&fmat, &b).unwrap();
        let vv = transformed(&vmat, &b).unwrap();
        assert_eq!(mm(&ff, &vv), mm(&fmat, &vmat));
        assert_eq!(
            gram(&ff, &gg),
            gg.iter()
                .map(|r| r.iter().map(|v| 251 * v).collect())
                .collect::<M>()
        );
        assert!(transformed(&sigma, &b).is_none());
        let ellid = id()
            .iter()
            .map(|r| r.iter().map(|v| 19 * v).collect())
            .collect();
        let q = solve_left(&b, &ellid).unwrap();
        assert_eq!(mm(&b, &q), ellid);
        assert_eq!(det(q.clone()).abs(), 361);
        assert_eq!(mm(&q, &fmat), mm(&ff, &q));
        assert_eq!(
            gram(&q, &gg),
            g.iter()
                .map(|r| r.iter().map(|v| 19 * v).collect())
                .collect::<M>()
        );
        descriptor(&format!("q{index}"), &b, 19, &gg, &phi);
        println!("QUOTIENT\tindex={index}\tkernel_parameter={}\tbasis_numerator={b:?}\tbasis_denominator=19\tE={gg:?}\tF={ff:?}\tV={vv:?}\tQ={q:?}",kernels[&h[..2].to_vec()]);
    }
    println!(
        "ORDER\tstabilizer_constraints={:?}\tindex=361\tpredicted_discriminant=16290125",
        order_constraints.unwrap()
    );
    let q = 1009;
    let mut power = id();
    let mut b = fmat.clone();
    for row in &mut b {
        for x in row {
            *x = x.rem_euclid(q)
        }
    }
    let mut n = 10;
    while n > 0 {
        if n & 1 == 1 {
            power = mm(&power, &b);
            for row in &mut power {
                for x in row {
                    *x = x.rem_euclid(q)
                }
            }
        }
        b = mm(&b, &b);
        for row in &mut b {
            for x in row {
                *x = x.rem_euclid(q)
            }
        }
        n >>= 1;
    }
    let aux = Fp::new(q as u64);
    let mut w = vec![1, 0, 0, 0];
    let mut powers = vec![];
    for _ in 0..4 {
        powers.push(w.clone());
        w = mv(&power, &w, &aux);
    }
    assert_eq!(rref(powers, &aux).1.len(), 4);
    eprintln!("PASS original_model_p251_c18=true stable_lines=362 maximal_isotropic_kernels=20 sigma_orbits=4 quotient_modules=4 exact_principal_F_V_Q_symplectic_checks=true common_order_index361=true Frobenius10_degree4_support=true geometric_curve_and_target_map_not_constructed=true speed_claim=false");
}
