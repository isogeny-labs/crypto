//! Complete integral equivariant recovery on the actual augmentation factors.
use crate::{large_field as lf, round02, round04::read};
use num_bigint::BigInt as Z;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use serde_json::{json, Value};
use std::path::Path;

type C = (Z, Z);
type M = Vec<Vec<C>>;
type IM = Vec<Vec<Z>>;
fn c(a: i64, b: i64) -> C {
    (a.into(), b.into())
}
fn ca(x: &C, y: &C) -> C {
    (&x.0 + &y.0, &x.1 + &y.1)
}
fn cn(x: &C) -> C {
    (-&x.0, -&x.1)
}
fn cm(x: &C, y: &C) -> C {
    (
        &x.0 * &y.0 - 2 * &x.1 * &y.1,
        &x.0 * &y.1 + &x.1 * &y.0 - &x.1 * &y.1,
    )
}
fn eye(n: usize) -> M {
    (0..n)
        .map(|i| (0..n).map(|j| c(i64::from(i == j), 0)).collect())
        .collect()
}
fn zero(m: usize, n: usize) -> M {
    vec![vec![c(0, 0); n]; m]
}
fn mul(a: &M, b: &M) -> M {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| (0..b.len()).fold(c(0, 0), |s, k| ca(&s, &cm(&a[i][k], &b[k][j]))))
                .collect()
        })
        .collect()
}
fn madd(a: &mut M, b: &M) {
    for (r, s) in a.iter_mut().zip(b) {
        for (x, y) in r.iter_mut().zip(s) {
            *x = ca(x, y);
        }
    }
}
fn scale(a: &M, b: &C) -> M {
    a.iter()
        .map(|r| r.iter().map(|v| cm(v, b)).collect())
        .collect()
}
fn value(a: &M) -> Value {
    json!(a
        .iter()
        .map(|r| r
            .iter()
            .map(|x| [x.0.to_string(), x.1.to_string()])
            .collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn ivalue(a: &IM) -> Value {
    json!(a
        .iter()
        .map(|r| r.iter().map(ToString::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn lit(a: &M) -> String {
    format!(
        "![{}]",
        a.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(|(a, b)| format!("({a},{b})"))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn ilit(a: &IM) -> String {
    format!(
        "![{}]",
        a.iter()
            .map(|r| format!(
                "![{}]",
                r.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .collect::<Vec<_>>()
            .join(",")
    )
}
fn ii(n: usize) -> IM {
    (0..n)
        .map(|i| (0..n).map(|j| Z::from(i == j)).collect())
        .collect()
}
fn imul(a: &IM, b: &IM) -> IM {
    (0..a.len())
        .map(|i| {
            (0..b[0].len())
                .map(|j| (0..b.len()).map(|k| &a[i][k] * &b[k][j]).sum())
                .collect()
        })
        .collect()
}
fn row(a: &mut IM, i: usize, j: usize, q: &Z) {
    let r = a[j].clone();
    for (x, y) in a[i].iter_mut().zip(r) {
        *x += q * y;
    }
}
fn col(a: &mut IM, i: usize, j: usize, q: &Z) {
    for r in a {
        let t = q * &r[j];
        r[i] += t;
    }
}
fn swapcol(a: &mut IM, i: usize, j: usize) {
    for r in a {
        r.swap(i, j);
    }
}
struct Smith {
    d: IM,
    u: IM,
    v: IM,
    ui: IM,
    vi: IM,
    rank: usize,
    ops: usize,
}
fn smith(input: &IM) -> Smith {
    let (m, n) = (input.len(), input[0].len());
    let (mut a, mut u, mut v, mut ui, mut vi) = (input.clone(), ii(m), ii(n), ii(m), ii(n));
    let (mut rank, mut ops) = (0, 0);
    for k in 0..m.min(n) {
        let pivot = (k..m)
            .flat_map(|i| (k..n).map(move |j| (i, j)))
            .filter(|(i, j)| !a[*i][*j].is_zero())
            .min_by_key(|(i, j)| a[*i][*j].abs());
        let Some((i, j)) = pivot else {
            break;
        };
        a.swap(k, i);
        u.swap(k, i);
        swapcol(&mut ui, k, i);
        swapcol(&mut a, k, j);
        swapcol(&mut v, k, j);
        vi.swap(k, j);
        loop {
            ops += 1;
            assert!(ops <= 100000, "registered Smith step cap");
            assert!(
                a.iter()
                    .chain(&u)
                    .chain(&v)
                    .chain(&ui)
                    .chain(&vi)
                    .flatten()
                    .all(|z| z.bits() < 4096),
                "registered integer bit cap"
            );
            if let Some(i) = (k + 1..m).find(|i| !a[*i][k].is_zero()) {
                let q = -a[i][k].div_floor(&a[k][k]);
                row(&mut a, i, k, &q);
                row(&mut u, i, k, &q);
                col(&mut ui, k, i, &(-&q));
                if !a[i][k].is_zero() {
                    a.swap(k, i);
                    u.swap(k, i);
                    swapcol(&mut ui, k, i);
                }
                continue;
            }
            if let Some(j) = (k + 1..n).find(|j| !a[k][*j].is_zero()) {
                let q = -a[k][j].div_floor(&a[k][k]);
                col(&mut a, j, k, &q);
                col(&mut v, j, k, &q);
                row(&mut vi, k, j, &(-&q));
                if !a[k][j].is_zero() {
                    swapcol(&mut a, k, j);
                    swapcol(&mut v, k, j);
                    vi.swap(k, j);
                }
                continue;
            }
            if let Some((i, _)) = (k + 1..m)
                .flat_map(|i| (k + 1..n).map(move |j| (i, j)))
                .find(|(i, j)| !a[*i][*j].mod_floor(&a[k][k]).is_zero())
            {
                row(&mut a, k, i, &Z::one());
                row(&mut u, k, i, &Z::one());
                col(&mut ui, i, k, &(-Z::one()));
                continue;
            }
            break;
        }
        if a[k][k].is_negative() {
            for x in &mut a[k] {
                *x = -&*x;
            }
            for x in &mut u[k] {
                *x = -&*x;
            }
            for r in &mut ui {
                r[k] = -&r[k];
            }
        }
        rank += 1;
    }
    assert_eq!(imul(&imul(&u, input), &v), a);
    assert_eq!(imul(&u, &ui), ii(m));
    assert_eq!(imul(&ui, &u), ii(m));
    assert_eq!(imul(&v, &vi), ii(n));
    assert_eq!(imul(&vi, &v), ii(n));
    for i in 0..m {
        for j in 0..n {
            if i != j {
                assert!(a[i][j].is_zero());
            }
        }
    }
    for i in 1..rank {
        assert!(a[i][i].mod_floor(&a[i - 1][i - 1]).is_zero());
    }
    Smith {
        d: a,
        u,
        v,
        ui,
        vi,
        rank,
        ops,
    }
}
fn solve(s: &Smith, target: &[Z]) -> Option<(Z, Vec<Z>)> {
    let y =
        s.u.iter()
            .map(|r| r.iter().zip(target).map(|(a, b)| a * b).sum::<Z>())
            .collect::<Vec<_>>();
    if y[s.rank..].iter().any(|z| !z.is_zero()) {
        return None;
    }
    let mut d = Z::one();
    for (i, yi) in y.iter().enumerate().take(s.rank) {
        d = d.lcm(&(&s.d[i][i] / s.d[i][i].gcd(yi)));
    }
    let mut z = vec![Z::zero(); s.v.len()];
    for i in 0..s.rank {
        z[i] = &d * &y[i] / &s.d[i][i];
    }
    let x =
        s.v.iter()
            .map(|r| r.iter().zip(&z).map(|(a, b)| a * b).sum())
            .collect();
    Some((d, x))
}
fn polynomial(a: &M) -> Vec<C> {
    let mut acc = c(0, 0);
    let mut out = vec![c(0, 0); 4];
    for k in (0..4).rev() {
        acc = ca(&acc, &a[k][0]);
        out[k] = acc.clone();
    }
    out
}
fn poly_matrix(p: &[C], powers: &[M]) -> M {
    let mut out = zero(4, 4);
    for (a, r) in p.iter().zip(powers) {
        madd(&mut out, &scale(r, a));
    }
    out
}
fn parse(v: &Value) -> M {
    v.as_array()
        .unwrap()
        .iter()
        .map(|r| {
            r.as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    (
                        v[0].as_str().unwrap().parse().unwrap(),
                        v[1].as_str().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}
fn scalar(curve: &crate::binary_ecc::BinaryCurve, p: &lf::P, n: &Z) -> lf::P {
    let mut k = n.magnitude().clone();
    let mut p = p.clone();
    let mut q = lf::P::Infinity;
    while !k.is_zero() {
        if k.is_odd() {
            q = lf::plus(curve, &q, &p);
        }
        p = lf::plus(curve, &p, &p);
        k >>= 1;
    }
    if n.is_negative() {
        lf::neg(&q)
    } else {
        q
    }
}
fn apply(f: &lf::F, a: u64, m: &M, v: &[lf::P]) -> Vec<lf::P> {
    let curve = f.curve(f.e(a), f.o());
    m.iter()
        .map(|r| {
            r.iter().zip(v).fold(lf::P::Infinity, |s, ((x, y), p)| {
                let w = f.frobp(p, 1);
                let w = if a == 1 { lf::neg(&w) } else { w };
                lf::plus(
                    &curve,
                    &s,
                    &lf::plus(&curve, &scalar(&curve, p, x), &scalar(&curve, &w, y)),
                )
            })
        })
        .collect()
}
fn matrix_list(a: &[M]) -> Value {
    Value::Array(a.iter().map(value).collect())
}

pub fn run(dir: &Path) {
    assert!(!dir.join("g32-normzero-integral-recovery.json").exists());
    let data = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let phi = (0..20)
        .map(|i| {
            (0..4)
                .map(|j| {
                    c(
                        data["orbits"][i / 5][i % 5][j][0].as_i64().unwrap(),
                        data["orbits"][i / 5][i % 5][j][1].as_i64().unwrap(),
                    )
                })
                .collect()
        })
        .collect::<M>();
    let prior = read(&dir.join("g28-integral-orbit-retraction.json"));
    let old_l = parse(&prior["CM_recovery_coefficients"]);
    assert_eq!(mul(&old_l, &phi), eye(4));
    let r = vec![
        vec![c(-1, 0); 4],
        vec![c(1, 0), c(0, 0), c(0, 0), c(0, 0)],
        vec![c(0, 0), c(1, 0), c(0, 0), c(0, 0)],
        vec![c(0, 0), c(0, 0), c(1, 0), c(0, 0)],
    ];
    let mut powers = vec![eye(4)];
    for k in 0..4 {
        powers.push(mul(&powers[k], &r));
    }
    let mut norm = zero(4, 4);
    for p in &powers {
        madd(&mut norm, p);
    }
    assert_eq!(norm, zero(4, 4));
    let cyclic = (0..4)
        .map(|i| (0..4).map(|j| powers[j][i][0].clone()).collect())
        .collect::<M>();
    let cyclic_inv = (0..4)
        .map(|i| (0..4).map(|j| c(i64::from(j >= i), 0)).collect())
        .collect::<M>();
    assert_eq!(mul(&cyclic, &cyclic_inv), eye(4));
    assert_eq!(mul(&cyclic_inv, &cyclic), eye(4));
    let j = (0..5)
        .map(|i| {
            (0..4)
                .map(|k| c(if i == 4 { -1 } else { i64::from(k == 3 - i) }, 0))
                .collect()
        })
        .collect::<M>();
    let p = (0..4)
        .map(|i| (0..5).map(|k| c(i64::from(k == 3 - i), 0)).collect())
        .collect::<M>();
    let pi = (0..5)
        .map(|i| (0..5).map(|k| c(i64::from(k == (i + 1) % 5), 0)).collect())
        .collect::<M>();
    assert_eq!(mul(&p, &j), eye(4));
    assert_eq!(mul(&pi, &j), mul(&j, &r));
    let mut maps = vec![];
    let mut restricted = vec![];
    let mut coefficients = vec![];
    for block in 0..4 {
        let ph = phi[5 * block..5 * block + 5].to_vec();
        let m = mul(&p, &ph);
        assert_eq!(mul(&j, &m), ph);
        assert_eq!(mul(&m, &r), mul(&r, &m));
        let q = polynomial(&m);
        assert_eq!(poly_matrix(&q, &powers), m);
        coefficients.push(q);
        restricted.push(mul(
            &old_l
                .iter()
                .map(|r| r[5 * block..5 * block + 5].to_vec())
                .collect(),
            &j,
        ));
        maps.push(m);
    }
    let mut check = zero(4, 4);
    for b in 0..4 {
        madd(&mut check, &mul(&restricted[b], &maps[b]));
    }
    assert_eq!(check, eye(4));
    let mut h = vec![vec![Z::zero(); 32]; 8];
    for b in 0..4 {
        for k in 0..4 {
            for w in 0..2 {
                let z = polynomial(&mul(
                    &scale(&powers[k], &c(i64::from(w == 0), i64::from(w == 1))),
                    &maps[b],
                ));
                for t in 0..4 {
                    h[2 * t][8 * b + 2 * k + w] = z[t].0.clone();
                    h[2 * t + 1][8 * b + 2 * k + w] = z[t].1.clone();
                }
            }
        }
    }
    let s = smith(&h);
    let target = (0..8).map(|i| Z::from(i == 0)).collect::<Vec<_>>();
    let (d, x) = solve(&s, &target).expect("nonzero scalar recovery");
    let bezout = (0..4)
        .map(|b| {
            poly_matrix(
                &(0..4)
                    .map(|k| (x[8 * b + 2 * k].clone(), x[8 * b + 2 * k + 1].clone()))
                    .collect::<Vec<_>>(),
                &powers,
            )
        })
        .collect::<Vec<_>>();
    let mut check = zero(4, 4);
    for b in 0..4 {
        assert_eq!(mul(&bezout[b], &r), mul(&r, &bezout[b]));
        madd(&mut check, &mul(&bezout[b], &maps[b]));
    }
    assert_eq!(check, scale(&eye(4), &(d.clone(), Z::zero())));
    // Independent coefficient-pairing construction from the old non-equivariant recovery.
    let dual = restricted
        .iter()
        .map(|l| {
            let a = (0..4)
                .map(|k| mul(l, &powers[k])[3][0].clone())
                .collect::<Vec<_>>();
            let b = (0..4)
                .map(|k| a[..4 - k].iter().fold(c(0, 0), |z, v| ca(&z, v)))
                .collect::<Vec<_>>();
            poly_matrix(&b, &powers)
        })
        .collect::<Vec<_>>();
    let mut check = zero(4, 4);
    for b in 0..4 {
        assert_eq!(mul(&dual[b], &r), mul(&r, &dual[b]));
        madd(&mut check, &mul(&dual[b], &maps[b]));
    }
    assert_eq!(check, eye(4));
    let twice = h
        .iter()
        .map(|r| r.iter().map(|z| z * 2).collect())
        .collect::<IM>();
    let scaled = solve(&smith(&twice), &target).unwrap().0;
    assert_eq!(scaled, Z::from(2));
    assert!(solve(&smith(&vec![vec![Z::zero(); 32]; 8]), &target).is_none());
    assert_eq!(d, Z::one());
    assert_eq!(s.rank, 8);
    assert!((0..8).all(|i| s.d[i][i].is_one()));
    let smith_w = imul(&s.v.iter().map(|r| r[..8].to_vec()).collect(), &s.u);
    assert_eq!(imul(&h, &smith_w), ii(8));
    let smith_bezout = bezout;
    let bezout = dual.clone();
    let mut w = vec![vec![Z::zero(); 8]; 32];
    for b in 0..4 {
        for t in 0..8 {
            let basis = scale(
                &powers[t / 2],
                &c(i64::from(t % 2 == 0), i64::from(t % 2 == 1)),
            );
            let coefficients = polynomial(&mul(&bezout[b], &basis));
            for k in 0..4 {
                w[8 * b + 2 * k][t] = coefficients[k].0.clone();
                w[8 * b + 2 * k + 1][t] = coefficients[k].1.clone();
            }
        }
    }
    assert_eq!(imul(&h, &w), ii(8));
    let mut corrupt = bezout.clone();
    corrupt[0][0][0] = ca(&corrupt[0][0][0], &c(1, 0));
    let mut bad = zero(4, 4);
    for b in 0..4 {
        madd(&mut bad, &mul(&corrupt[b], &maps[b]));
    }
    assert_ne!(bad, eye(4));
    let f = lf::F::new(5, 5);
    let mut fixtures = vec![];
    for a in 0..2 {
        let curve = f.curve(f.e(a), f.o());
        let mut generator = None;
        'find: for x in 0..32 {
            for y in 0..32 {
                let q = lf::P::Affine {
                    x: f.e(x),
                    y: f.e(y),
                };
                if curve.is_on_curve(&q)
                    && !matches!(q, lf::P::Infinity)
                    && matches!(lf::times(&curve, &q, 11), lf::P::Infinity)
                {
                    generator = Some(q);
                    break 'find;
                }
            }
        }
        let generator = generator.expect("actual public order11 point");
        for k in 0..11 {
            let q = lf::times(&curve, &generator, k);
            let v = (0..4)
                .map(|i| f.frobp(&q, (3 - i) as u32))
                .collect::<Vec<_>>();
            let rv = apply(&f, a, &r, &v);
            assert_eq!(v.iter().map(|v| f.frobp(v, 1)).collect::<Vec<_>>(), rv);
            let mut recovered = vec![lf::P::Infinity; 4];
            let mut images = vec![];
            for b in 0..4 {
                let im = apply(&f, a, &maps[b], &v);
                let back = apply(&f, a, &bezout[b], &im);
                for i in 0..4 {
                    recovered[i] = lf::plus(&curve, &recovered[i], &back[i]);
                }
                images.push(im);
            }
            assert_eq!(recovered, v);
            fixtures.push(json!({"a":a,"scalar":k,"generator":lf::pj(&generator),"source":v.iter().map(lf::pj).collect::<Vec<_>>(),"images":images.iter().map(|r|r.iter().map(lf::pj).collect::<Vec<_>>()).collect::<Vec<_>>(),"recovered":recovered.iter().map(lf::pj).collect::<Vec<_>>(),"F2_descent_verified":true}));
        }
    }
    let mut out=format!("import KoblitzRound21DescentData\nnamespace KoblitzRound22Recovery\nopen scoped BigOperators\nopen KoblitzRound19OrbitCover\ndef rotation : Fin 4→Fin 4→CM := {}\n",lit(&r));
    for b in 0..4 {
        out.push_str(&format!(
            "def map{b} : Fin 4→Fin 4→CM := {}\ndef recovery{b} : Fin 4→Fin 4→CM := {}\n",
            lit(&maps[b]),
            lit(&bezout[b])
        ));
    }
    out.push_str("theorem actual_integral_recovery : ∀i j:Fin 4,(∑k:Fin 4,(cmul (recovery0 i k) (map0 k j)+cmul (recovery1 i k) (map1 k j)+cmul (recovery2 i k) (map2 k j)+cmul (recovery3 i k) (map3 k j)))=(if i=j then((1,0):CM)else(0,0)) := by decide +kernel\n");
    for b in 0..4 {
        out.push_str(&format!("theorem actual_map_commutes{b} : ∀i j:Fin 4,(∑k:Fin 4,cmul (map{b} i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (map{b} k j)) := by decide +kernel\ntheorem actual_recovery_commutes{b} : ∀i j:Fin 4,(∑k:Fin 4,cmul (recovery{b} i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) (recovery{b} k j)) := by decide +kernel\n"));
    }
    out.push_str(&format!("def integerSystem : Matrix (Fin 8) (Fin 32) ℤ := {}\ndef integerRightInverse : Matrix (Fin 32) (Fin 8) ℤ := {}\ntheorem actual_full_integer_lattice : integerSystem*integerRightInverse=1 := by decide +kernel\nend KoblitzRound22Recovery\n",ilit(&h),ilit(&w)));
    out=out.replace("end KoblitzRound22Recovery\n", "def actualMap (j : Fin 4) : Fin 4→Fin 4→CM := ![map0,map1,map2,map3] j\ntheorem actual_augmentation_binding : ∀j i k:Fin 4,actualMap j i k=KoblitzRound20Retraction.row ⟨5*j.val+3-i.val,by omega⟩ k := by decide +kernel\ntheorem actual_regular_norm_zero : ∀j k:Fin 4,(∑s:Fin 5,KoblitzRound20Retraction.row ⟨5*j.val+s.val,by omega⟩ k)=(0,0) := by decide +kernel\ntheorem actual_rotation_binding : rotation=KoblitzRound21DescentData.rotation := by decide +kernel\nend KoblitzRound22Recovery\n");
    for name in std::iter::once("actual_integral_recovery".to_string())
        .chain((0..4).flat_map(|b| {
            [
                format!("actual_map_commutes{b}"),
                format!("actual_recovery_commutes{b}"),
            ]
        }))
        .chain(std::iter::once("actual_full_integer_lattice".to_string()))
        .chain(
            [
                "actual_augmentation_binding",
                "actual_regular_norm_zero",
                "actual_rotation_binding",
            ]
            .into_iter()
            .map(str::to_owned),
        )
    {
        out.push_str(&format!("#print axioms KoblitzRound22Recovery.{name}\n"));
    }
    std::fs::write(dir.join("KoblitzRound22Recovery.lean"), out).unwrap();
    round02::save(
        dir,
        "g32-normzero-integral-recovery.json",
        &json!({"schema":"koblitz.round22/g32-v1","CM_relation":"w^2+w+2=0","augmentation_embedding_J":value(&j),"augmentation_projection_P":value(&p),"rotation":value(&r),"cyclic_basis":value(&cyclic),"cyclic_basis_inverse":value(&cyclic_inv),"M":matrix_list(&maps),"M_polynomial_coefficients":coefficients.iter().map(|r|r.iter().map(|(a,b)|[a.to_string(),b.to_string()]).collect::<Vec<_>>()).collect::<Vec<_>>(),"B":matrix_list(&bezout),"Smith_B":matrix_list(&smith_bezout),"canonical_recovery":"integral coefficient-pairing construction;Smith solution separately retained","independent_pairing_recovery":matrix_list(&dual),"integer_system":ivalue(&h),"Smith_D":ivalue(&s.d),"Smith_U":ivalue(&s.u),"Smith_V":ivalue(&s.v),"Smith_U_inverse":ivalue(&s.ui),"Smith_V_inverse":ivalue(&s.vi),"Smith_rank":s.rank,"Smith_operations":s.ops,"integer_right_inverse":ivalue(&w),"Smith_integer_right_inverse":ivalue(&smith_w),"least_positive_recovery_scalar":d.to_string(),"solution":x.iter().map(ToString::to_string).collect::<Vec<_>>(),"complete_linear_system":true,"coefficient_box_search":false,"controls":{"twice_system_least_scalar":scaled.to_string(),"zero_system_no_positive_scalar":true,"corrupted_recovery_rejected":true},"F32_order11_fixtures":fixtures,"final_polarized_quotient_constructed":false,"modular_Jacobian_map_constructed":false,"scope":"exact integral CM augmentation maps and both-sign evaluated order11 source controls;geometric CM/descent bridge remains explicit"}),
    );
    println!("G32:rank8 full integer image;least scalar{d};22 both-sign order11 fixtures;independent pairing recovery;Smith steps{}",s.ops);
}

fn expand(a: &M) -> IM {
    (0..2 * a.len())
        .map(|i| {
            (0..2 * a[0].len())
                .map(|j| {
                    let (x, y) = &a[i / 2][j / 2];
                    match (i % 2, j % 2) {
                        (0, 0) => x.clone(),
                        (0, 1) => -2 * y,
                        (1, 0) => y.clone(),
                        _ => x - y,
                    }
                })
                .collect()
        })
        .collect()
}
/// Parse the stored result and verify through integer regular representations,
/// independent of the CM-matrix multiplication used by the producer.
pub fn verify(dir: &Path) {
    let d = read(&dir.join("g32-normzero-integral-recovery.json"));
    let maps = d["M"]
        .as_array()
        .unwrap()
        .iter()
        .map(parse)
        .collect::<Vec<_>>();
    let bs = d["B"]
        .as_array()
        .unwrap()
        .iter()
        .map(parse)
        .collect::<Vec<_>>();
    let r = expand(&parse(&d["rotation"]));
    let mut total = vec![vec![Z::zero(); 8]; 8];
    for j in 0..4 {
        let m = expand(&maps[j]);
        let b = expand(&bs[j]);
        assert_eq!(imul(&m, &r), imul(&r, &m));
        assert_eq!(imul(&b, &r), imul(&r, &b));
        let prod = imul(&b, &m);
        for i in 0..8 {
            for k in 0..8 {
                total[i][k] += &prod[i][k];
            }
        }
    }
    assert_eq!(total, ii(8));
    let get = |name: &str| {
        d[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                r.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().parse::<Z>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<IM>()
    };
    let h = get("integer_system");
    let w = get("integer_right_inverse");
    assert_eq!(imul(&h, &w), ii(8));
    let su = get("Smith_U");
    let sv = get("Smith_V");
    let sd = get("Smith_D");
    assert_eq!(imul(&imul(&su, &h), &sv), sd);
    for (a, b) in [(&su, get("Smith_U_inverse")), (&sv, get("Smith_V_inverse"))] {
        assert_eq!(imul(a, &b), ii(a.len()));
        assert_eq!(imul(&b, a), ii(a.len()));
    }
    for i in 0..8 {
        for j in 0..32 {
            assert_eq!(sd[i][j], Z::from(i == j));
        }
    }
    let kernel = sv.iter().map(|r| r[8..].to_vec()).collect::<IM>();
    assert_eq!(imul(&h, &kernel), vec![vec![Z::zero(); 24]; 8]);
    let j = parse(&d["augmentation_embedding_J"]);
    let jt = (0..4)
        .map(|i| (0..5).map(|k| j[k][i].clone()).collect())
        .collect::<M>();
    let metric = mul(&jt, &j);
    let realmetric = metric
        .iter()
        .map(|r| {
            r.iter()
                .map(|z| {
                    assert!(z.1.is_zero());
                    z.0.clone()
                })
                .collect()
        })
        .collect::<IM>();
    let ms = smith(&realmetric);
    assert_eq!(
        (0..4).map(|i| ms.d[i][i].clone()).collect::<Vec<_>>(),
        [1, 1, 1, 5].map(Z::from)
    );
    let old = read(&dir.join("g25-correct-polarization-orbit-sections.json"));
    let mut weighted = zero(4, 4);
    let mut wrong = zero(4, 4);
    for k in 0..4 {
        let adj = (0..4)
            .map(|i| {
                (0..4)
                    .map(|j| {
                        let (a, b) = &maps[k][j][i];
                        (a - b, -b)
                    })
                    .collect()
            })
            .collect::<M>();
        let weight = c(old["weights"][k].as_i64().unwrap(), 0);
        madd(
            &mut weighted,
            &scale(&mul(&mul(&adj, &metric), &maps[k]), &weight),
        );
        madd(&mut wrong, &scale(&mul(&adj, &maps[k]), &weight));
    }
    let expected = (0..4)
        .map(|i| {
            (0..4)
                .map(|j| {
                    c(
                        old["actual_Gram"][i][j][0].as_i64().unwrap(),
                        old["actual_Gram"][i][j][1].as_i64().unwrap(),
                    )
                })
                .collect()
        })
        .collect::<M>();
    assert_eq!(weighted, expected);
    assert_ne!(wrong, expected);
    assert_eq!(d["least_positive_recovery_scalar"], "1");
    let poly = |mat: &M| {
        polynomial(mat)
            .iter()
            .map(|(a, b)| [a.to_string(), b.to_string()])
            .collect::<Vec<_>>()
    };
    let mut ids = vec![];
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap();
    let path = "research/koblitz_modular_ic_20261018/lean/KoblitzRound22Recovery.lean";
    let phash = hex::encode(crate::sha256_impl::sha256(
        &std::fs::read(repo.join(path)).unwrap(),
    ));
    for i in 0..8 {
        use crate::identity_certificate::{self as ic, Expr as X};
        let mut terms = vec![];
        for j in 0..4 {
            let b = expand(&bs[j]);
            let m = expand(&maps[j]);
            for k in 0..8 {
                for t in 0..8 {
                    if !b[i][k].is_zero() && !m[k][t].is_zero() {
                        terms.push(X::product(vec![
                            X::Const(b[i][k].to_string().parse().unwrap()),
                            X::Const(m[k][t].to_string().parse().unwrap()),
                            X::var(&format!("x{t}")),
                        ]));
                    }
                }
            }
        }
        let lhs = X::sum(terms);
        let st=ic::IdentityStatement{name:format!("actual-augmentation-recovery-coordinate{i}"),source:"KoblitzRound22Recovery.actual_integral_recovery;independent Z-regular representation of CM matrices".into(),variables:(0..8).map(|j|format!("x{j}")).collect(),degree_bound:1,lhs,rhs:X::var(&format!("x{i}"))};
        let cert=ic::issue(st,202610260100+i as u64,64,"exact integer matrix certificate and separate Lean kernel proof;128 additional parsed checks",Some(ic::ProofRef{kind:"lean4".into(),path:path.into(),sha256:phash.clone()})).unwrap();
        assert!(matches!(
            ic::check_with_extra(&cert, Some((202610260200 + i as u64, 128))).unwrap(),
            ic::Verdict::Accept { .. }
        ));
        let name = format!("identity-recovery-{i}.json");
        let file = dir.join(&name);
        assert!(!file.exists());
        round02::save(dir, &name, &serde_json::to_value(&cert).unwrap());
        ids.push(cert.id);
    }
    assert!(!dir.join("g32-independent-integer-replay.json").exists());
    round02::save(
        dir,
        "g32-independent-integer-replay.json",
        &json!({"schema":"koblitz.round22/g32-independent-v1","integer_regular_representation_sum":ivalue(&total),"commutations_checked":8,"Smith_identity_and_two_sided_integer_inverses":true,"all_Smith_factors_one":true,"integer_right_inverse_verified":true,"kernel_rank":24,"kernel_basis":ivalue(&kernel),"all_solutions":"canonical_particular+kernel_basis*z, z in Z^24;Smith unimodular certificate proves completeness","augmentation_induced_metric":value(&metric),"augmentation_metric_Smith_diagonal":[1,1,1,5],"actual_weighted_polarization_Gram":value(&weighted),"replacing_induced_metric_by_identity_rejected":true,"M_polynomial_coefficients":maps.iter().map(poly).collect::<Vec<_>>(),"B_polynomial_coefficients":bs.iter().map(poly).collect::<Vec<_>>(),"identity_certificate_ids":ids}),
    );
    println!("G32 independent integer replay:8 commuting matrices;sum B*M=I8;full Smith certificate;8 source-bound IDC records");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smith_handles_rectangular_obstructions() {
        let h = vec![
            vec![Z::from(6), Z::from(0), Z::from(4)],
            vec![Z::from(0), Z::from(10), Z::from(0)],
        ];
        let s = smith(&h);
        assert_eq!(solve(&s, &[Z::from(1), Z::from(1)]).unwrap().0, Z::from(10));
        let h = vec![vec![Z::from(2), Z::from(4)], vec![Z::from(0), Z::from(0)]];
        assert!(solve(&smith(&h), &[Z::from(0), Z::from(1)]).is_none());
    }
}

/// A direct, small two-map certificate, derived by inversion modulo7.
/// The complete32-column Smith solution remains frozen separately.
pub fn two_map(dir: &Path) {
    let data = read(&dir.join("g32-normzero-integral-recovery.json"));
    let r = parse(&data["rotation"]);
    let mut powers = vec![eye(4)];
    for k in 0..3 {
        powers.push(mul(&powers[k], &r));
    }
    let maps = data["M"]
        .as_array()
        .unwrap()
        .iter()
        .map(parse)
        .collect::<Vec<_>>();
    let bcoef = vec![c(1, 3), c(2, 5), c(3, 0), c(1, 2)];
    let ccoef = vec![c(-2, 4), c(0, 0), c(-3, 0), c(-2, 3)];
    let b = poly_matrix(&bcoef, &powers);
    let cc = poly_matrix(&ccoef, &powers);
    let mut sum = mul(&b, &maps[1]);
    madd(&mut sum, &mul(&cc, &maps[3]));
    assert_eq!(sum, eye(4));
    for a in [&b, &cc] {
        assert_eq!(mul(a, &r), mul(&r, a));
    }
    let mut integer = imul(&expand(&b), &expand(&maps[1]));
    let cprod = imul(&expand(&cc), &expand(&maps[3]));
    for i in 0..8 {
        for j in 0..8 {
            integer[i][j] += &cprod[i][j];
        }
    }
    assert_eq!(integer, ii(8));
    let mut out=format!("import KoblitzRound22TwoMap\nnamespace KoblitzRound22TwoMapData\nopen scoped BigOperators\nopen KoblitzRound19OrbitCover KoblitzRound22Recovery\ndef smallB : Fin 4→Fin 4→CM := {}\ndef smallC : Fin 4→Fin 4→CM := {}\n",lit(&b),lit(&cc));
    out.push_str(&format!("def powers : Fin 4→Fin 4→Fin 4→CM := ![{}]\ndef bcoeff : Fin 4→CM := ![{}]\ndef ccoeff : Fin 4→CM := ![{}]\n",powers.iter().map(lit).collect::<Vec<_>>().join(","),bcoef.iter().map(|(a,b)|format!("({a},{b})")).collect::<Vec<_>>().join(","),ccoef.iter().map(|(a,b)|format!("({a},{b})")).collect::<Vec<_>>().join(",")));
    out.push_str("theorem actual_two_map_recovery : ∀i j:Fin 4,(∑k:Fin 4,(cmul (smallB i k) (map1 k j)+cmul (smallC i k) (map3 k j)))=(if i=j then((1,0):CM)else(0,0)) := by decide +kernel\n");
    for name in ["smallB", "smallC"] {
        out.push_str(&format!("theorem {name}_commutes : ∀i j:Fin 4,(∑k:Fin 4,cmul ({name} i k) (rotation k j))=(∑k:Fin 4,cmul (rotation i k) ({name} k j)) := by decide +kernel\n"));
    }
    out.push_str("theorem actual_b_coefficients : ∀i j:Fin 4,smallB i j=(∑k:Fin 4,cmul (bcoeff k) (powers k i j)) := by decide +kernel\ntheorem actual_c_coefficients : ∀i j:Fin 4,smallC i j=(∑k:Fin 4,cmul (ccoeff k) (powers k i j)) := by decide +kernel\nend KoblitzRound22TwoMapData\n");
    for name in [
        "actual_two_map_recovery",
        "smallB_commutes",
        "smallC_commutes",
        "actual_b_coefficients",
        "actual_c_coefficients",
    ] {
        out.push_str(&format!("#print axioms KoblitzRound22TwoMapData.{name}\n"));
    }
    assert!(!dir.join("KoblitzRound22TwoMapData.lean").exists());
    std::fs::write(dir.join("KoblitzRound22TwoMapData.lean"), out).unwrap();
    let f = lf::F::new(5, 5);
    let mut fixtures = 0;
    for row in data["F32_order11_fixtures"].as_array().unwrap() {
        let a = row["a"].as_u64().unwrap();
        let curve = f.curve(f.e(a), f.o());
        let source = row["source"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| crate::reverse06::dp(&f, p))
            .collect::<Vec<_>>();
        let images = &row["images"];
        let i1 = images[1]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| crate::reverse06::dp(&f, p))
            .collect::<Vec<_>>();
        let i3 = images[3]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| crate::reverse06::dp(&f, p))
            .collect::<Vec<_>>();
        let mut back = apply(&f, a, &b, &i1);
        let other = apply(&f, a, &cc, &i3);
        for i in 0..4 {
            back[i] = lf::plus(&curve, &back[i], &other[i]);
        }
        assert_eq!(back, source);
        fixtures += 1;
    }
    assert_eq!(fixtures, 22);
    assert!(!dir.join("g33-small-two-map-recovery.json").exists());
    round02::save(
        dir,
        "g33-small-two-map-recovery.json",
        &json!({"schema":"koblitz.round22/g33-v1","actual_map_indices":[1,3],"B0_and_B2_zero":true,"B1_coefficients":bcoef.iter().map(|(a,b)|[a.to_string(),b.to_string()]).collect::<Vec<_>>(),"B3_coefficients":ccoef.iter().map(|(a,b)|[a.to_string(),b.to_string()]).collect::<Vec<_>>(),"B1":value(&b),"B3":value(&cc),"integer_recovery_sum":ivalue(&integer),"commutation_verified":true,"both_sign_order11_fixture_checks":fixtures,"coefficient_box_search":false,"construction":"exact inverse of M1 modulo7;B3=(I-B1*M1)/7;separate complete32-column Smith certificate retained","weighted_four_map_polarization_unchanged":true}),
    );
    println!("G33:small integral recovery using only M1 and M3=7I;22 both-sign fixtures;exact CM and independent integer products;no coefficient search");
}
