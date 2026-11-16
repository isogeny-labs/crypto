//! Exact prerequisite checks for the two named genus-3 hyperelliptic models.
//! This is not a general Smith codomain/isogeny implementation.
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use p256_genus_check::{inv, parameters, sub, Poly};
use serde_json::{json, Value};
use smith_reference::{smith_kernel_coverage as sk, smith_prop24 as sp};

pub fn modpow(a: &Poly, exponent: &BigUint, modulus: &Poly, p: &BigUint) -> Poly {
    let (mut a, mut e, mut result) = (a.rem(modulus, p), exponent.clone(), Poly::one());
    while !e.is_zero() {
        if e.bit(0) {
            result = result.mul(&a, p).rem(modulus, p);
        }
        a = a.mul(&a, p).rem(modulus, p);
        e >>= 1;
    }
    result
}
pub fn factor_degrees(f: &Poly, p: &BigUint) -> Vec<u32> {
    assert_eq!(f.xgcd(&f.derivative(p), p).2, Poly::one());
    let n = f.degree().unwrap();
    let x = Poly::monomial(1);
    let mut xp = x.clone();
    let mut count = vec![0usize; n + 1];
    let mut out = vec![];
    for d in 1..=n {
        xp = modpow(&xp, p, f, p);
        let degree = f.xgcd(&xp.sub(&x, p), p).2.degree().unwrap();
        let accounted = (1..d)
            .filter(|e| d % e == 0)
            .map(|e| e * count[e])
            .sum::<usize>();
        assert!(degree >= accounted && (degree - accounted).is_multiple_of(d));
        count[d] = (degree - accounted) / d;
        out.extend(std::iter::repeat_n(d as u32, count[d]));
    }
    assert_eq!(out.iter().sum::<u32>(), n as u32);
    out.sort_unstable();
    out
}
fn chi(a: &BigUint, p: &BigUint) -> i8 {
    if a.is_zero() {
        0
    } else if a.modpow(&((p - BigUint::one()) / 2u32), p) == BigUint::one() {
        1
    } else {
        -1
    }
}
fn sqrt_3mod4(a: &BigUint, p: &BigUint) -> Option<BigUint> {
    assert_eq!(p % 4u32, 3u32.into());
    let r = a.modpow(&((p + BigUint::one()) / 4u32), p);
    (&r * &r % p == a % p).then_some(r)
}

struct Extension {
    p: BigUint,
    f: Poly,
    q: BigUint,
}
impl Extension {
    fn new(p: &BigUint, f: Poly) -> Self {
        assert_eq!(factor_degrees(&f, p), vec![f.degree().unwrap() as u32]);
        let q = p.pow(f.degree().unwrap() as u32);
        Self { p: p.clone(), f, q }
    }
    fn c(&self, n: u32) -> Poly {
        Poly::new(vec![n.into()], &self.p)
    }
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        a.add(b, &self.p)
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        a.sub(b, &self.p)
    }
    fn neg(&self, a: &Poly) -> Poly {
        a.neg(&self.p)
    }
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        a.mul(b, &self.p).rem(&self.f, &self.p)
    }
    fn pow(&self, a: &Poly, e: &BigUint) -> Poly {
        modpow(a, e, &self.f, &self.p)
    }
    fn frob(&self, a: &Poly) -> Poly {
        self.pow(a, &self.p)
    }
    fn sqrt(&self, a: &Poly) -> Option<Poly> {
        assert_eq!(&self.q % 4u32, 3u32.into());
        let r = self.pow(a, &((&self.q + BigUint::one()) / 4u32));
        (self.mul(&r, &r) == *a).then_some(r)
    }
    fn eval(&self, f: &Poly, x: &Poly) -> Poly {
        f.0.iter().rev().fold(Poly::zero(), |a, c| {
            self.add(&self.mul(&a, x), &Poly(vec![c.clone()]))
        })
    }
    fn orbit(&self, a: Poly) -> Vec<Poly> {
        let mut out = vec![a.clone()];
        let mut next = self.frob(&a);
        while next != a {
            assert!(out.len() < self.f.degree().unwrap());
            out.push(next.clone());
            next = self.frob(&next);
        }
        out
    }
}
#[derive(Clone)]
enum Branch {
    Finite(Poly),
    Infinity,
}
type Matching = Vec<(usize, usize)>;
fn matchings(unused: &[usize]) -> Vec<Matching> {
    if unused.is_empty() {
        return vec![vec![]];
    }
    let a = unused[0];
    let mut out = vec![];
    for i in 1..unused.len() {
        let b = unused[i];
        let rest: Vec<_> = unused
            .iter()
            .copied()
            .filter(|x| *x != a && *x != b)
            .collect();
        for mut m in matchings(&rest) {
            m.push((a, b));
            m.sort_unstable();
            out.push(m);
        }
    }
    out.sort();
    out
}
fn stable(m: &Matching, perm: &[usize; 8]) -> bool {
    m.iter().all(|(a, b)| {
        let (x, y) = (perm[*a], perm[*b]);
        m.contains(&(x.min(y), x.max(y)))
    })
}
fn row_space(
    mut a: Vec<Vec<BigUint>>,
    columns: usize,
    p: &BigUint,
) -> (Vec<Vec<BigUint>>, Vec<usize>, Vec<Vec<BigUint>>) {
    let mut piv = vec![];
    let mut r = 0;
    for col in 0..columns {
        let Some(i) = (r..a.len()).find(|i| !a[*i][col].is_zero()) else {
            continue;
        };
        a.swap(r, i);
        let k = inv(&a[r][col], p);
        for v in &mut a[r] {
            *v = &*v * &k % p;
        }
        for i in 0..a.len() {
            if i != r {
                let k = a[i][col].clone();
                for j in 0..columns {
                    a[i][j] = sub(&a[i][j], &(&k * &a[r][j] % p), p);
                }
            }
        }
        piv.push(col);
        r += 1;
        if r == a.len() {
            break;
        }
    }
    let mut basis = vec![];
    for free in (0..columns).filter(|c| !piv.contains(c)) {
        let mut v = vec![BigUint::zero(); columns];
        v[free] = BigUint::one();
        for (i, col) in piv.iter().enumerate() {
            v[*col] = sub(&BigUint::zero(), &a[i][free], p);
        }
        basis.push(v);
    }
    a.truncate(r);
    (a, piv, basis)
}
fn qform(a: &[BigUint], b: &[BigUint], p: &BigUint) -> BigUint {
    (0..6).fold(BigUint::zero(), |v, i| (v + &a[i] * &b[(i + 3) % 6]) % p)
}
fn matrix_rows(k: &Extension, points: &[Branch], m: &Matching) -> Vec<Vec<BigUint>> {
    let mut out = vec![];
    for (i, j) in m {
        let (a, b, c) = match (&points[*i], &points[*j]) {
            (Branch::Finite(a), Branch::Finite(b)) => (k.c(1), k.neg(&k.add(a, b)), k.mul(a, b)),
            (Branch::Finite(a), Branch::Infinity) | (Branch::Infinity, Branch::Finite(a)) => {
                (k.c(0), k.c(1), k.neg(a))
            }
            _ => panic!("distinct points required"),
        };
        let ac = k.mul(&a, &c);
        let row = [
            k.mul(&a, &a),
            k.mul(&a, &b),
            ac.clone(),
            k.mul(&c, &c),
            k.neg(&k.mul(&c, &b)),
            k.sub(&k.mul(&b, &b), &ac),
        ];
        // A base-field vector lies in ker(M) iff each extension-basis component vanishes.
        for component in 0..k.f.degree().unwrap() {
            out.push(row.iter().map(|v| v.coeff(component)).collect());
        }
    }
    out
}
fn homogeneous_cubic(k: &Extension, coeff: &[BigUint], point: &Branch) -> Poly {
    match point {
        Branch::Infinity => Poly::new(vec![coeff[0].clone()], &k.p),
        Branch::Finite(x) => coeff.iter().fold(Poly::zero(), |a, c| {
            k.add(&k.mul(&a, x), &Poly::new(vec![c.clone()], &k.p))
        }),
    }
}
fn candidate(k: &Extension, gamma: &[BigUint], points: &[Branch], m: &Matching) -> Value {
    let z = BigUint::zero();
    let neg = |x: &BigUint| sub(&z, x, &k.p);
    let rows = vec![
        vec![z.clone(), neg(&gamma[3]), neg(&gamma[4]), neg(&gamma[5])],
        vec![
            gamma[3].clone(),
            z.clone(),
            neg(&gamma[2]),
            gamma[1].clone(),
        ],
        vec![
            gamma[4].clone(),
            gamma[2].clone(),
            z.clone(),
            neg(&gamma[0]),
        ],
        vec![
            gamma[5].clone(),
            neg(&gamma[1]),
            gamma[0].clone(),
            z.clone(),
        ],
    ];
    let (rows, piv, _) = row_space(rows, 4, &k.p);
    assert_eq!(piv.len(), 2);
    let n = &rows[0];
    let d = &rows[1];
    let np = Poly::new(n.iter().rev().cloned().collect(), &k.p);
    let dp = Poly::new(d.iter().rev().cloned().collect(), &k.p);
    let infinity_basepoint = n[0].is_zero() && d[0].is_zero();
    let gcd_degree = np.xgcd(&dp, &k.p).2.degree().unwrap();
    let genuine = !infinity_basepoint && gcd_degree == 0;
    let pairs = m.iter().all(|(a, b)| {
        k.mul(
            &homogeneous_cubic(k, n, &points[*a]),
            &homogeneous_cubic(k, d, &points[*b]),
        ) == k.mul(
            &homogeneous_cubic(k, n, &points[*b]),
            &homogeneous_cubic(k, d, &points[*a]),
        )
    });
    json!({"numerator_binary_cubic_desc":n.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "denominator_binary_cubic_desc":d.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "gcd_degree":gcd_degree,"basepoint_at_infinity":infinity_basepoint,
        "genuine_degree_three":genuine,"uncancelled_pair_conditions":pairs,
        "prop4_status":if genuine&&pairs{"not_computed"}else{"not_reached_degree_drop"}})
}
fn diagnose(k: &Extension, points: &[Branch], m: &Matching) -> Value {
    let (_, piv, basis) = row_space(matrix_rows(k, points, m), 6, &k.p);
    let rank = piv.len();
    if rank != 4 {
        return json!({"matching":m,"matrix_rank":rank,"status":"rank_degenerate","candidate_maps":[]});
    }
    let (a, b) = (&basis[0], &basis[1]);
    let aa = qform(a, a, &k.p);
    let ab = qform(a, b, &k.p);
    let bb = qform(b, b, &k.p);
    let disc = sub(&(&ab * &ab % &k.p), &(&aa * &bb % &k.p), &k.p);
    let squareclass = chi(&disc, &k.p);
    if squareclass < 0 {
        return json!({"matching":m,"matrix_rank":rank,"discriminant_squareclass":-1,"status":"no_rational_pencil_point","candidate_maps":[]});
    }
    let combine = |mu: &BigUint, lambda: &BigUint| {
        a.iter()
            .zip(b)
            .map(|(a, b)| (mu * a + lambda * b) % &k.p)
            .collect::<Vec<_>>()
    };
    let mut gammas = vec![];
    if !bb.is_zero() {
        let r = sqrt_3mod4(&disc, &k.p).unwrap();
        let inverse = inv(&bb, &k.p);
        for root in [r.clone(), sub(&BigUint::zero(), &r, &k.p)] {
            let lambda = sub(&root, &ab, &k.p) * &inverse % &k.p;
            let g = combine(&BigUint::one(), &lambda);
            if !gammas.contains(&g) {
                gammas.push(g);
            }
        }
    } else if !ab.is_zero() {
        gammas.push(b.clone());
        let lambda = sub(&BigUint::zero(), &aa, &k.p) * inv(&(&ab * 2u32 % &k.p), &k.p) % &k.p;
        gammas.push(combine(&BigUint::one(), &lambda));
    } else if !aa.is_zero() {
        gammas.push(b.clone());
    } else {
        return json!({"matching":m,"matrix_rank":rank,"status":"pencil_identically_isotropic_unresolved","candidate_maps":[]});
    }
    let maps: Vec<_> = gammas.iter().map(|g| candidate(k, g, points, m)).collect();
    let valid = maps
        .iter()
        .filter(|v| v["genuine_degree_three"] == true && v["uncancelled_pair_conditions"] == true)
        .count();
    json!({"matching":m,"matrix_rank":rank,"discriminant_squareclass":squareclass,
        "valid_degree_three_maps":valid,"status":if valid==0{"degree_drop"}else{"prop4_pending"},"candidate_maps":maps})
}

pub fn model_audit(p: &BigUint, b: &BigUint, model: &str) -> Value {
    assert_eq!(
        p % 12u32,
        7u32.into(),
        "this wide-field implementation uses p=7 mod 12"
    );
    let cubic = Poly::small(&[0, -3, 0, 1], p).add(&Poly::new(vec![b.clone()], p), p);
    assert_eq!(
        factor_degrees(&cubic, p),
        vec![3],
        "fixture requires irreducible elliptic cubic"
    );
    let mut h = Poly::monomial(7);
    h = match model {
        "C3" => h
            .add(&Poly::monomial(4).scale(b, p), p)
            .add(&Poly::monomial(1), p),
        "H5" => h
            .sub(&Poly::monomial(3).scale(&3u32.into(), p), p)
            .add(&Poly::monomial(1).scale(b, p), p),
        _ => panic!("unknown model"),
    };
    let degrees = factor_degrees(&h, p);
    let mut ct = degrees.clone();
    ct.push(1);
    ct.sort_unstable();
    let minus_b = sub(&BigUint::zero(), b, p);
    let (k, first, second) = if model == "H5" && chi(&minus_b, p) < 0 {
        let sextic = h.exact(&Poly::monomial(1), p);
        let k = Extension::new(p, sextic);
        (k, Poly::monomial(1), None)
    } else {
        let k = Extension::new(p, cubic);
        let x = Poly::monomial(1);
        if model == "C3" {
            let delta = k.sub(&k.mul(&x, &x), &k.c(4));
            let root = k.sqrt(&delta).expect("split quadratic over Fp3");
            let t = k.add(&x, &root).scale(&inv(&2u32.into(), p), p);
            let other = k.sub(&x, &t);
            (k, t, Some(other))
        } else {
            let t = k.sqrt(&x).expect("square root over Fp3");
            let other = k.neg(&t);
            (k, t, Some(other))
        }
    };
    let mut finite = k.orbit(first);
    if let Some(s) = second {
        finite.extend(k.orbit(s));
    }
    assert_eq!(finite.len(), 6);
    for i in 0..6 {
        assert_eq!(k.eval(&h, &finite[i]), Poly::zero());
        for j in 0..i {
            assert_ne!(finite[i], finite[j]);
        }
    }
    let mut points = vec![Branch::Finite(Poly::zero()), Branch::Infinity];
    points.extend(finite.iter().cloned().map(Branch::Finite));
    let mut perm = [0usize; 8];
    perm[0] = 0;
    perm[1] = 1;
    for (i, r) in finite.iter().enumerate() {
        perm[i + 2] = 2 + finite.iter().position(|s| *s == k.frob(r)).unwrap();
    }
    let all = matchings(&(0..8).collect::<Vec<_>>());
    assert_eq!(all.len(), 105);
    let stable: Vec<_> = all.iter().filter(|m| stable(m, &perm)).collect();
    let reference = sk::stable_kernels(&perm, &sk::lagrangians());
    assert_eq!(stable.len(), reference.0 as usize);
    let diagnoses: Vec<_> = stable.iter().map(|m| diagnose(&k, &points, m)).collect();
    json!({"model":model,"field_prime":p.to_string(),"b":b.to_string(),"factor_degrees_affine":degrees,
        "weierstrass_cycle_type":ct,"extension_degree_used":k.f.degree(),"frobenius_permutation":perm,
        "all_pair_partitions_checked":all.len(),"stable_pair_partitions":stable.len(),
        "stable_all_lagrangians":reference.1,"stable_non_pair_lagrangians":reference.1-reference.0,
        "subgroups":diagnoses,"standing":"producer exact prerequisite check; no signed verdict; no Smith codomain constructed"})
}
pub fn actual_audit() -> Value {
    let (p, b, n, _) = parameters();
    let delta = sub(&(&b * &b % &p), &4u32.into(), &p);
    json!({"schema":"p256-smith-prerequisites/v1","p_mod_12":(&p%12u32).to_u32().unwrap(),
        "cover_coefficient_discriminant_squareclass":chi(&delta,&p),"minus_b_squareclass":chi(&sub(&BigUint::zero(),&b,&p),&p),
        "p256_subgroup_order_odd":n.bit(0),"models":[model_audit(&p,&b,"C3"),model_audit(&p,&b,"H5")]})
}
pub fn reference_controls() -> Value {
    let mut rows = vec![];
    for p in [7u64, 19, 31, 43] {
        let b = (1..p)
            .find(|b| !(0..p).any(|x| (x * x % p * x + 3 * p - 3 * x % p + b) % p == 0))
            .unwrap();
        for model in ["C3", "H5"] {
            let mut f = vec![0u64; 8];
            f[7] = 1;
            if model == "C3" {
                f[1] = 1;
                f[4] = b;
            } else {
                f[1] = b;
                f[3] = p - 3;
            }
            let reference = sp::analyze_curve(&f, p, &mut sp::SplitMix64(20261115)).unwrap();
            let wide = model_audit(&p.into(), &b.into(), model);
            assert_eq!(json!(reference.cycle_type), wide["weierstrass_cycle_type"]);
            assert_eq!(
                reference.n_tractable(),
                wide["stable_pair_partitions"].as_u64().unwrap() as usize
            );
            let mut r: Vec<_> = reference
                .subgroups
                .iter()
                .map(|s| {
                    (
                        s.m_rank,
                        s.maps.iter().filter(|m| m.degree3 && m.pairs_ok).count(),
                    )
                })
                .collect();
            let mut w: Vec<_> = wide["subgroups"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| {
                    (
                        s["matrix_rank"].as_u64().unwrap() as usize,
                        s["valid_degree_three_maps"].as_u64().unwrap_or(0) as usize,
                    )
                })
                .collect();
            r.sort_unstable();
            w.sort_unstable();
            assert_eq!(r, w);
            rows.push(json!({"p":p,"b":b,"model":model,"reference_agrees":true,"kernel_rank_and_degree_counts":r}));
        }
    }
    let positive = sp::example1();
    assert!(positive.passed);
    json!({"small_field_comparisons":rows,"smith_paper_example1_positive_control":positive.passed,
        "scope":"comparison against pre-existing word-size implementation; not independent signed execution verification"})
}

pub fn catalogue() -> Value {
    let (p, b, n, _) = parameters();
    let audit = actual_audit();
    let mut direct = vec![];
    let models = [
        (
            "C2",
            2,
            "hyperelliptic",
            2,
            "z^2=f(t^2)",
            "(t^2,z)",
            2,
            2,
            "X",
        ),
        (
            "C3",
            3,
            "hyperelliptic",
            2,
            "z^2=t^7+b*t^4+t",
            "(t+1/t,z/t^2)",
            1,
            4,
            "X^2-4",
        ),
        (
            "C4",
            4,
            "hyperelliptic",
            3,
            "z^2=f(t^3)",
            "(t^3,z)",
            1,
            3,
            "",
        ),
        (
            "C5",
            5,
            "hyperelliptic",
            4,
            "z^2=f(t^4)",
            "(t^4,z)",
            2,
            3,
            "",
        ),
        (
            "D3",
            3,
            "nonhyperelliptic",
            2,
            "s^4=f(t)",
            "(t,s^2)",
            1,
            4,
            "Y",
        ),
        (
            "D4",
            4,
            "nonhyperelliptic",
            2,
            "s^4=t^2*f(t)",
            "(t,s^2/t)",
            1,
            6,
            "X*Y",
        ),
        (
            "D5",
            5,
            "nonhyperelliptic",
            2,
            "s^4=t^2*(t-1)^2*f(t)",
            "(t,s^2/(t*(t-1)))",
            1,
            8,
            "X*(X-1)*Y",
        ),
    ];
    for (id, g, geometry, d, equation, map, infinities, branches, squareclass) in models {
        let smith_gate = if g != 3 {
            "wrong_genus"
        } else if geometry == "hyperelliptic" {
            "genus3_hyperelliptic"
        } else {
            "already_nonhyperelliptic"
        };
        let arithmetic = if id == "C3" || id == "C4" {
            "implemented_producer_checked"
        } else {
            "not_implemented"
        };
        let factors = match id {
            "C2" => vec!["E", "F2"],
            "C3" => vec!["E", "E", "F3"],
            "C4" => vec!["E", "A3"],
            "C5" => vec!["E", "F2", "J(H5)"],
            "D3" => vec!["E", "Prym_dimension_2"],
            "D4" => vec!["E", "Prym_dimension_3"],
            _ => vec!["E", "Prym_dimension_4"],
        };
        let inertia = match id {
            "C4" => json!([{"index":"3","geometric_points":"3"}]),
            "C5" => {
                json!([{"index":"4","geometric_points":"2"},{"index":"2","geometric_points":"2"}])
            }
            _ => json!([{"index":"2","geometric_points":branches.to_string()}]),
        };
        let smith = if id == "C3" {
            json!({"input_gate":smith_gate,"status":"direct_smith_excluded","weierstrass_cycle_type":audit["models"][0]["weierstrass_cycle_type"],"stable_pair_kernels":"3","stable_non_pair_kernels":"6","obligation":"All three rational pair kernels are rank-degenerate. Non-pair kernels, other source models and extension fields are separate routes."})
        } else {
            json!({"input_gate":smith_gate,"status":"not_applicable","obligation":"The standard forward Smith construction requires a genus-3 hyperelliptic input."})
        };
        let autos = match id {
            "C2" => vec!["(t,z)->(-t,z)", "(t,z)->(t,-z)"],
            "C3" => vec![
                "(t,z)->(1/t,z/t^4)",
                "(t,z)->(zeta*t,zeta^2*z), zeta^3=1",
                "(t,z)->(t,-z)",
            ],
            "C4" => vec!["(t,z)->(zeta*t,z), zeta^3=1", "(t,z)->(t,-z)"],
            "C5" => vec![
                "over Fp: (t,z)->(-t,z)",
                "(t,z)->(t,-z)",
                "over Fp2: (t,z)->(i*t,z), i^2=-1",
            ],
            _ => vec!["over Fp: (t,s)->(t,-s)", "over Fp2: (t,s)->(t,i*s), i^2=-1"],
        };
        direct.push(json!({"schema":"p256-cover-traits/v1","id":id,"object_kind":"finite_cover",
          "base_field":{"characteristic":p.to_string(),"absolute_degree":"1","constant_field_verified":null},
          "source":{"equation":equation,"genus":g.to_string(),"geometry":geometry,"normalization_required":true,"rational_infinity_points":infinities.to_string()},
          "target":"E: Y^2=X^3-3X+b (standard P-256)",
          "representation":{"rational_map":map,"equivalence":"isomorphism_over_fixed_E","quadratic_squareclass":if squareclass.is_empty(){None}else{Some(squareclass)}},
          "traits":{"degree":{"total":d.to_string(),"separable":d.to_string(),"inseparable_exponent":"0"},
            "ramification":{"type":"tame","different_degree":(2*g-2).to_string(),"geometric_branch_points":branches.to_string(),"inertia_and_jumps":inertia},
            "galois":{"over_base":id!="C5","geometric":true,"known_base_deck_group":if id=="C4"{"C3"}else{"C2"},"known_geometric_deck_group":format!("C{d}"),"arithmetic_monodromy":if id=="C5"{None}else{Some(format!("C{d}"))}},
            "jacobian":{"dimension":g.to_string(),"known_isogeny_factors":factors,"polarization":"The Jacobian is principally polarized; displayed factorization is only up to isogeny.","p_rank":{"lower_bound":if id=="C3"{"2"}else{"1"},"upper_bound":g.to_string(),"exact":null},"frobenius_polynomial":null,"exact_point_order":null},
            "transfer":{"map_kind":"cover_pullback","subgroup_order":n.to_string(),"kernel_intersection":"trivial by degree coprimality","degree_coprime":true,"inverse_on_image":"[d^-1 mod n] times the norm; for C3/C4 explicit coefficient decoding is implemented","status":"theoretical"},
            "smith":smith,
            "implementation":{"geometry":if id.starts_with('C'){"implemented_producer_checked"}else{"theoretical"},"pullback":arithmetic,"norm":"not_implemented","arithmetic":arithmetic,"solver":if arithmetic=="implemented_producer_checked"{"implemented_producer_checked"}else{"not_implemented"}},
            "known_automorphisms":autos,"full_automorphism_group":null,
            "gonality":if geometry=="hyperelliptic"{json!({"geometric_exact":"2"})}else if g<=4{json!({"geometric_exact":"3","base_field_exact":null})}else{json!({"geometric_lower":"3","geometric_upper":"4","base_field_upper":"4"})}},
          "evidence":{"standing":"producer_assertion","sources":["FRAMEWORK.md","results/smith-p256.json","../results/native-p256.json"],"claim_id":null,"scope":"Mathematical geometry and transfer traits, with producer checks as marked. Solver checks are toy known-answer images only; no P-256 DLP run. General norm is unimplemented even where image decoding exists."}}));
    }
    json!({"schema":"p256-cover-catalogue/v1","scope":"Seven worked representatives within an unbounded complete representation framework; not a complete list of covers or isomorphism classes.",
      "parameters":{"p":p.to_string(),"b":b.to_string(),"n":n.to_string()},"direct_covers":direct,
      "companion_curves":[
        {"id":"F2","genus":"1","geometry":"elliptic","equation":"v^2=u^3-3u^2+b^2","origin":"quotient of C2: (u,v)=(b/t^2,b*z/t^3)","P256_embedding":"not_established"},
        {"id":"F3","genus":"1","geometry":"elliptic","equation":"v^2=u^3+b*u^2+u","origin":"quotient of C3: (u,v)=(t^3,t*z)","P256_embedding":"excluded over Fp by point-order parity: F3 has rational two-torsion and E has odd order"},
        {"id":"H5","genus":"3","geometry":"hyperelliptic","equation":"v^2=u^7-3u^3+b*u","origin":"quotient of C5: (u,v)=(t^2,t*z)","P256_embedding":"not_established; the standard C5 image has zero projection to this complementary factor","smith_audit":audit["models"][1]}
      ],"abstract_factors":[{"id":"A3","dimension":"3","curve_model":null},{"id":"Prym(D3/E)","dimension":"2","curve_model":null},{"id":"Prym(D4/E)","dimension":"3","curve_model":null},{"id":"Prym(D5/E)","dimension":"4","curve_model":null}],
      "unknowns_policy":"Null means not computed or not established; a known automorphism subgroup is not the full automorphism group."})
}

pub fn validate_worked_catalogue(v: &Value) -> Result<(), String> {
    let n = parameters().2;
    let covers = v["direct_covers"]
        .as_array()
        .ok_or("missing direct_covers")?;
    if covers.len() != 7 {
        return Err("expected seven worked representatives".into());
    }
    for record in covers {
        let q = |path: &str| {
            record
                .pointer(path)
                .and_then(Value::as_str)
                .ok_or_else(|| format!("missing {path}"))
                .and_then(|s| s.parse::<u32>().map_err(|_| format!("invalid {path}")))
        };
        let g = q("/source/genus")?;
        let d = q("/traits/degree/separable")?;
        if !(2..=5).contains(&g) || !(2..=4).contains(&d) {
            return Err("outside the declared worked-catalogue bounds".into());
        }
        if q("/traits/degree/total")? != d || q("/traits/degree/inseparable_exponent")? != 0 {
            return Err("worked profiles must be separable".into());
        }
        if g < 2 || q("/traits/ramification/different_degree")? != 2 * g - 2 {
            return Err("Riemann-Hurwitz mismatch".into());
        }
        let hyper = record["source"]["geometry"] == "hyperelliptic";
        if hyper && g > 2 * d - 1 {
            return Err("hyperelliptic cover exceeds genus bound".into());
        }
        if g == 2 && !hyper {
            return Err("genus two must be hyperelliptic".into());
        }
        if n.clone() % BigUint::from(d) == BigUint::zero() {
            return Err("degree not coprime to n".into());
        }
        let inertia = record["traits"]["ramification"]["inertia_and_jumps"]
            .as_array()
            .ok_or("missing inertia")?;
        let mut different = 0u64;
        for item in inertia {
            let quantity = |key: &str| {
                item[key]
                    .as_str()
                    .ok_or_else(|| format!("missing inertia {key}"))
                    .and_then(|s| {
                        s.parse::<u32>()
                            .map_err(|_| format!("invalid inertia {key}"))
                    })
            };
            let e = quantity("index")?;
            let count = quantity("geometric_points")?;
            if e < 2 {
                return Err("listed ramification index must exceed one".into());
            }
            different += u64::from(e - 1) * u64::from(count);
        }
        if different != u64::from(2 * g - 2) {
            return Err("tame different does not match inertia".into());
        }
        let gate = record["traits"]["smith"]["input_gate"]
            .as_str()
            .ok_or("missing smith gate")?;
        if gate == "genus3_hyperelliptic" && !(g == 3 && hyper) {
            return Err("invalid Smith input gate".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod classification_tests {
    use super::*;
    #[test]
    fn all_135_lagrangians_and_105_pairings() {
        let all = sk::lagrangians();
        assert_eq!(all.len(), 135);
        assert_eq!(all.iter().filter(|a| a.1).count(), 105);
        assert_eq!(matchings(&(0..8).collect::<Vec<_>>()).len(), 105);
    }
    #[test]
    fn small_reference_and_positive_control() {
        reference_controls();
    }
    #[test]
    fn actual_p256_models() {
        let v = actual_audit();
        let c = &v["models"][0];
        assert_eq!(c["weierstrass_cycle_type"], json!([1, 1, 3, 3]));
        assert_eq!(c["stable_pair_partitions"], 3);
        for s in c["subgroups"].as_array().unwrap() {
            assert_eq!(s["matrix_rank"], 3);
        }
    }
    #[test]
    fn catalogue_and_negative_traits() {
        let v = catalogue();
        validate_worked_catalogue(&v).unwrap();
        let mut bad = v.clone();
        bad["direct_covers"][5]["source"]["geometry"] = json!("hyperelliptic");
        assert!(validate_worked_catalogue(&bad).is_err());
        let mut bad = v.clone();
        bad["direct_covers"][0]["traits"]["ramification"]["different_degree"] = json!("3");
        assert!(validate_worked_catalogue(&bad).is_err());
        let mut bad = v.clone();
        bad["direct_covers"][0]["traits"]["degree"]["separable"] = json!("0");
        assert!(validate_worked_catalogue(&bad).is_err());
        let mut bad = v;
        bad["direct_covers"][0]["traits"]["ramification"]["inertia_and_jumps"][0]["index"] =
            json!("bad");
        assert!(validate_worked_catalogue(&bad).is_err());
    }
}
