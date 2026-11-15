//! Exact bounded search. No timing, solver advantage or formal geometric proof.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use p256_genus_check::{inv, parameters, sub, transfer, Divisor, Jacobian, Point, Poly};
use serde_json::{json, Value};
use smith_reference::smith_kernel_coverage as sk;

fn canon(mask: u8) -> u8 {
    if mask & 0x80 == 0 {
        mask
    } else {
        !mask
    }
}
fn permute(mask: u8, p: &[usize; 8]) -> u8 {
    let mut result = 0;
    for (i, &j) in p.iter().enumerate() {
        if mask & (1 << i) != 0 {
            result |= 1 << j;
        }
    }
    canon(result)
}
fn plane(matching: &[[usize; 2]]) -> [u8; 3] {
    let anchor = matching
        .iter()
        .position(|p| p.contains(&0) && p.contains(&1))
        .unwrap();
    let mut result = vec![];
    let [a, b] = matching[anchor];
    for (i, &[c, d]) in matching.iter().enumerate() {
        if i != anchor {
            result.push(canon((1 << a) | (1 << b) | (1 << c) | (1 << d)));
        }
    }
    result.sort_unstable();
    result.try_into().unwrap()
}
fn contains_plane(kernel: &[u8; 7], p: &[u8; 3]) -> bool {
    p.iter().all(|v| kernel.contains(v))
}
pub fn c3_kernels(prerequisite: &Value) -> Value {
    let model = &prerequisite["models"][0];
    let perm: [usize; 8] = model["frobenius_permutation"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as usize)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let planes: Vec<_> = model["subgroups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            let matching: Vec<[usize; 2]> = s["matching"]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| {
                    [
                        p[0].as_u64().unwrap() as usize,
                        p[1].as_u64().unwrap() as usize,
                    ]
                })
                .collect();
            plane(&matching)
        })
        .collect();
    assert_eq!(planes.len(), 3);
    for p in &planes {
        assert!(p.iter().all(|v| p.contains(&permute(*v, &perm))));
        assert_eq!(canon(p[0] ^ p[1]), p[2]);
        assert!(p.iter().all(|v| *v != 0));
        for x in p {
            for y in p {
                assert_eq!((x & y).count_ones() % 2, 0);
            }
        }
    }
    let lags = sk::lagrangians();
    let stable: Vec<_> = lags
        .iter()
        .filter(|(k, _)| k.iter().all(|v| k.contains(&permute(*v, &perm))))
        .collect();
    let rows: Vec<_> = stable.iter().map(|(k, pair)| {
        let contained: Vec<_> = planes.iter().enumerate().filter(|(_, w)| contains_plane(k, w)).map(|(i,_)|i).collect();
        json!({"kernel_nonzero_masks":k,"pair_type":pair,"elliptic_planes_contained":contained,
            "polarization_verdict":if contained.is_empty(){"unresolved"}else{"decomposable_by_principal_elliptic_factor"}})
    }).collect();
    let covered = rows
        .iter()
        .filter(|r| {
            !r["elliptic_planes_contained"]
                .as_array()
                .unwrap()
                .is_empty()
        })
        .count();
    json!({"geometric_lagrangians":lags.len(),"geometric_pair_kernels":lags.iter().filter(|(_,p)|*p).count(),
        "rational_lagrangians":stable.len(),"elliptic_two_torsion_planes":planes,
        "covered_by_elliptic_plane":covered,"kernels":rows,
        "standing":"producer finite combinatorics plus written polarization argument; no signed geometric proof"})
}

fn matchings(points: &[usize]) -> Vec<Vec<[usize; 2]>> {
    if points.is_empty() {
        return vec![vec![]];
    }
    let mut result = vec![];
    for i in 1..points.len() {
        let rest: Vec<_> = points
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != 0 && *j != i)
            .map(|(_, v)| *v)
            .collect();
        for mut tail in matchings(&rest) {
            tail.insert(0, [points[0], points[i]]);
            result.push(tail);
        }
    }
    result
}
fn stable_matching(m: &[[usize; 2]], perm: &[usize]) -> bool {
    m.iter().all(|&[a, b]| {
        m.iter()
            .any(|&[c, d]| (perm[a] == c && perm[b] == d) || (perm[a] == d && perm[b] == c))
    })
}
pub fn c2_kernels() -> Value {
    let (p, b, _, _) = parameters();
    let f = p256_genus_check::equation(2, &b, &p);
    let degrees = p256_cover_classification::factor_degrees(&f, &p);
    let mut perm = vec![0; 6];
    let mut start = 0;
    for &d in &degrees {
        let d = d as usize;
        for i in 0..d {
            perm[start + i] = start + (i + 1) % d;
        }
        start += d;
    }
    assert_eq!(start, 6);
    let all = matchings(&(0..6).collect::<Vec<_>>());
    let stable: Vec<_> = all
        .iter()
        .filter(|m| stable_matching(m, &perm))
        .cloned()
        .collect();
    json!({"factor_degrees":degrees,"geometric_richelot_kernels":all.len(),"rational_richelot_kernels":stable.len(),
        "frobenius_permutation":perm,"stable_matchings":stable,
        "interpretation":"identify the antipodal pairing with t -> -t; its quadratic coefficient matrix has zero middle column and yields the split Richelot quotient"})
}

fn pow_poly(mut a: Poly, exponent: &BigUint, f: &Poly, p: &BigUint) -> Poly {
    let mut e = exponent.clone();
    let mut r = Poly::one();
    while !e.is_zero() {
        if e.bit(0) {
            r = r.mul(&a, p).rem(f, p);
        }
        a = a.mul(&a, p).rem(f, p);
        e >>= 1;
    }
    r
}
fn three_polynomial(a: &BigUint, b: &BigUint, p: &BigUint) -> Poly {
    Poly::new(
        vec![
            sub(&BigUint::zero(), &(a * a % p * inv(&3u32.into(), p) % p), p),
            4u32 * b % p,
            2u32 * a % p,
            BigUint::zero(),
            BigUint::one(),
        ],
        p,
    )
}

#[derive(Clone)]
pub struct Curve {
    pub p: BigUint,
    pub a: BigUint,
    pub b: BigUint,
}
impl Curve {
    pub fn rhs(&self, x: &BigUint) -> BigUint {
        (x * x * x + &self.a * x + &self.b) % &self.p
    }
    pub fn contains(&self, q: &Point) -> bool {
        q.as_ref()
            .is_none_or(|(x, y)| x < &self.p && y < &self.p && y * y % &self.p == self.rhs(x))
    }
    pub fn add(&self, q: &Point, r: &Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (q, r) else {
            return if q.is_none() { r.clone() } else { q.clone() };
        };
        let p = &self.p;
        if x == u && (y + v) % p == BigUint::zero() {
            return None;
        }
        let slope = if x == u {
            (3u32 * x * x + &self.a) * inv(&(2u32 * y % p), p) % p
        } else {
            sub(v, y, p) * inv(&sub(u, x, p), p) % p
        };
        let xx = sub(&sub(&(&slope * &slope), x, p), u, p);
        Some((xx.clone(), sub(&(&slope * sub(x, &xx, p)), y, p)))
    }
    pub fn mul(&self, k: &BigUint, q: &Point) -> Point {
        let (mut k, mut a, mut r) = (k.clone(), q.clone(), None);
        while !k.is_zero() {
            if k.bit(0) {
                r = self.add(&r, &a);
            }
            a = self.add(&a, &a);
            k >>= 1;
        }
        r
    }
}

#[derive(Clone)]
pub struct ThreeMap {
    pub source: Curve,
    pub target: Curve,
    pub r: BigUint,
    v: BigUint,
    u: BigUint,
}
impl ThreeMap {
    pub fn new(source: Curve, r: BigUint) -> Result<Self, &'static str> {
        let p = &source.p;
        let psi = three_polynomial(&source.a, &source.b, p);
        let value = psi
            .0
            .iter()
            .rev()
            .fold(BigUint::zero(), |acc, c| (acc * &r + c) % p);
        if !value.is_zero() || source.rhs(&r).is_zero() {
            return Err("not a nonzero order-three kernel");
        }
        let v = 2u32 * (3u32 * &r * &r + &source.a) % p;
        let u = 4u32 * source.rhs(&r) % p;
        let target = Curve {
            p: p.clone(),
            a: sub(&source.a, &(5u32 * &v), p),
            b: sub(&source.b, &(7u32 * (&u + &r * &v)), p),
        };
        let result = Self {
            source,
            target,
            r,
            v,
            u,
        };
        if !result.cleared_identity() {
            return Err("Vélu curve identity failed");
        }
        Ok(result)
    }
    pub fn eval(&self, q: &Point) -> Result<Point, &'static str> {
        if !self.source.contains(q) {
            return Err("invalid source point");
        }
        let Some((x, y)) = q else {
            return Ok(None);
        };
        if *x == self.r {
            return Ok(None);
        }
        let p = &self.source.p;
        let d = inv(&sub(x, &self.r, p), p);
        let d2 = &d * &d % p;
        let d3 = &d2 * &d % p;
        let xx = (x + &self.v * &d + &self.u * &d2) % p;
        let yy = y * sub(&BigUint::one(), &(&self.v * &d2 + 2u32 * &self.u * &d3), p) % p;
        let out = Some((xx, yy));
        if !self.target.contains(&out) {
            return Err("invalid image point");
        }
        Ok(out)
    }
    pub fn dual(&self, q: &Point) -> Result<Point, &'static str> {
        let p = &self.source.p;
        let next = Self::new(
            self.target.clone(),
            sub(&BigUint::zero(), &(3u32 * &self.r), p),
        )?;
        assert_eq!(next.target.a, 81u32 * &self.source.a % p);
        assert_eq!(next.target.b, 729u32 * &self.source.b % p);
        let result = next
            .eval(q)?
            .map(|(x, y)| (x * inv(&9u32.into(), p) % p, y * inv(&27u32.into(), p) % p));
        if !self.source.contains(&result) {
            return Err("invalid dual image");
        }
        Ok(result)
    }
    pub fn cleared_identity(&self) -> bool {
        let p = &self.source.p;
        let x = Poly::monomial(1);
        let d = x.sub(&Poly::new(vec![self.r.clone()], p), p);
        let n = x
            .mul(&d.pow(2, p), p)
            .add(&d.scale(&self.v, p), p)
            .add(&Poly::new(vec![self.u.clone()], p), p);
        let m = d
            .pow(3, p)
            .sub(&d.scale(&self.v, p), p)
            .sub(&Poly::new(vec![2u32 * &self.u % p], p), p);
        let f = Poly::new(
            vec![
                self.source.b.clone(),
                self.source.a.clone(),
                BigUint::zero(),
                BigUint::one(),
            ],
            p,
        );
        f.mul(&m.pow(2, p), p)
            == n.pow(3, p)
                .add(&n.mul(&d.pow(4, p), p).scale(&self.target.a, p), p)
                .add(&d.pow(6, p).scale(&self.target.b, p), p)
    }
}

pub fn rational_three_kernel() -> Value {
    let (p, b, n, _) = parameters();
    let a = &p - 3u32;
    // psi_3 / 3 = x^4 + 2a*x^2 + 4b*x - a^2/3.
    let psi = three_polynomial(&a, &b, &p);
    let x = Poly::monomial(1);
    let rational = psi
        .xgcd(&pow_poly(x.clone(), &p, &psi, &p).sub(&x, &p), &p)
        .2
        .monic(&p);
    assert_eq!(psi.rem(&rational, &p), Poly::zero());
    assert_eq!(pow_poly(x.clone(), &p, &rational, &p), x.rem(&rational, &p));
    let mut map_output = Value::Null;
    if rational.degree() == Some(1) {
        let r = sub(&BigUint::zero(), &rational.coeff(0), &p);
        let phi = ThreeMap::new(
            Curve {
                p: p.clone(),
                a: a.clone(),
                b: b.clone(),
            },
            r.clone(),
        )
        .unwrap();
        let (_, _, _, g) = parameters();
        let mapped_g = phi.eval(&g).unwrap();
        assert!(mapped_g.is_some());
        assert_eq!(phi.target.mul(&n, &mapped_g), None);
        let return_multiplier = inv(&3u32.into(), &n);
        let mut checks = vec![];
        for k in [0u64, 1, 2, 3, 7, 37, 101] {
            let q = phi.source.mul(&k.into(), &g);
            let image = phi.eval(&q).unwrap();
            assert_eq!(image, phi.target.mul(&k.into(), &mapped_g));
            let dual = phi.dual(&image).unwrap();
            assert_eq!(dual, phi.source.mul(&3u32.into(), &q));
            assert_eq!(phi.source.mul(&return_multiplier, &dual), q);
            assert_eq!(
                phi.eval(&phi.source.add(&q, &g)).unwrap(),
                phi.target.add(&image, &mapped_g)
            );
            checks.push(k);
        }
        // Hasse's interval contains a unique multiple of n, so this n-order
        // nonzero image also pins the target's full point order to n.
        let sqrt_p = p.sqrt();
        assert!(n > 4u32 * (&sqrt_p + 1u32));
        let target_minus_three_model_squareclass = sub(
            &BigUint::zero(),
            &(&phi.target.a * inv(&3u32.into(), &p) % &p),
            &p,
        )
        .modpow(&((&p - 1u32) >> 1), &p);
        map_output = json!({"degree":3,"kernel_x":r.to_string(),"v":phi.v.to_string(),"u":phi.u.to_string(),
            "kernel_y_squareclass":phi.source.rhs(&r).modpow(&((&p-1u32)>>1),&p).to_string(),
            "target":{"p":p.to_string(),"a":phi.target.a.to_string(),"b":phi.target.b.to_string(),"order":n.to_string(),
                "generator_image":mapped_g.as_ref().map(|(x,y)|vec![x.to_string(),y.to_string()]),
                "minus_three_scaling_squareclass":target_minus_three_model_squareclass.to_string()},
            "cleared_curve_identity":true,"checked_public_scalars":checks,"dual_composition":"[3]",
            "inverse_on_image":"[3^-1 mod n] times dual", "destination_solver":"generic prime-order DLP; no specialized faster solver supplied",
            "measured_speedup":null,"novelty":"standard Vélu control; small-degree existence was already recorded by the repository P-256 census"});
    }
    json!({"division_polynomial_normalized_coefficients":psi.0.iter().map(|x|x.to_string()).collect::<Vec<_>>(),
        "rational_kernel_x_factor":rational.0.iter().map(|x|x.to_string()).collect::<Vec<_>>(),
        "rational_kernel_x_degree":rational.degree(),"subgroup_coprime_to_three":&n%3u32!=BigUint::zero(),"map":map_output})
}

pub fn audit() -> Value {
    let prerequisites = p256_cover_classification::actual_audit();
    json!({"schema":"p256-transfer-search/v1","source_icv1":"icv1-fp256-t89188191154553853111372247798585809583-f188c491",
        "c3":c3_kernels(&prerequisites),"c2":c2_kernels(),"elliptic_three":rational_three_kernel(),
        "symmetry_image":symmetry_image(),
        "f2_companion_screen":companion_screen(),
        "measured_speedup":null,"standing":"unsigned producer correctness checks; speedup requirement remains open"})
}

pub fn companion_screen() -> Value {
    let (p, b, n, _) = parameters();
    let a = &p - 3u32;
    let bb = sub(&(&b * &b % &p), &2u32.into(), &p);
    let psi = three_polynomial(&a, &bb, &p);
    let x = Poly::monomial(1);
    let factor = psi
        .xgcd(&pow_poly(x.clone(), &p, &psi, &p).sub(&x, &p), &p)
        .2;
    assert_eq!(psi.rem(&factor, &p), Poly::zero());
    let mut trace = None;
    let mut ordinate = Value::Null;
    let mut witness = Value::Null;
    if factor.degree() == Some(0) {
        trace = Some(0u32);
    }
    if factor.degree() == Some(1) {
        let r = sub(&BigUint::zero(), &factor.coeff(0), &p);
        let c = Curve {
            p: p.clone(),
            a,
            b: bb.clone(),
        };
        let square = c.rhs(&r).modpow(&((&p - 1u32) >> 1), &p) == BigUint::one();
        trace = Some(if square { 2u32 } else { 1u32 });
        ordinate = json!({"kernel_x":r.to_string(),"ordinate_is_square":square});
        if square {
            let y = c.rhs(&r).modpow(&((&p + 1u32) >> 2), &p);
            let q = Some((r, y));
            assert!(c.contains(&q));
            assert_eq!(c.mul(&3u32.into(), &q), None);
            assert!(p.clone() + 1u32 + 2u32 * (p.sqrt() + 1u32) < 3u32 * &n);
            witness = json!({"point":q.map(|(x,y)|vec![x.to_string(),y.to_string()]),"exact_order":3,
                "rational_n_subgroup":"excluded: a group of order divisible by both 3 and n would exceed Hasse's upper bound"});
        }
    }
    let source_trace = ((&p + 1u32 - &n) % 3u32)
        .to_u32_digits()
        .first()
        .copied()
        .unwrap_or(0);
    json!({"model":"F2 after shift: y^2=x^3-3x+(b_P256^2-2)","coefficient_b":bb.to_string(),
        "psi3_rational_root_factor_degree":factor.degree(),"ordinate":ordinate,
        "trace_mod_3":trace,"p256_trace_mod_3":source_trace,
        "small_torsion_witness":witness,
        "same_field_elliptic_homomorphism":match trace {Some(t) if t!=source_trace=>"excluded by different Frobenius traces modulo 3",_=>"unresolved by this screen"},
        "full_companion_order":null})
}

fn cube_root_of_unity(prime: &BigUint) -> BigUint {
    assert_eq!(prime % 3u32, BigUint::one());
    let exponent = (prime - 1u32) / 3u32;
    (2u32..)
        .map(|a| BigUint::from(a).modpow(&exponent, prime))
        .find(|x| *x != BigUint::one())
        .unwrap()
}

// Pullback by rho(t,z)=(zeta*t,zeta^2*z); on point divisors use rho^-1.
fn rotate(d: &Divisor, zeta: &BigUint, p: &BigUint) -> Divisor {
    let degree = d.0.degree().unwrap();
    let u = Poly::new(
        d.0 .0
            .iter()
            .enumerate()
            .map(|(i, c)| c * zeta.modpow(&BigUint::from(i + 2 * degree), p) % p)
            .collect(),
        p,
    );
    let v = Poly::new(
        d.1 .0
            .iter()
            .enumerate()
            .map(|(i, c)| c * zeta.modpow(&BigUint::from(i + 1), p) % p)
            .collect(),
        p,
    );
    (u, v)
}
fn divisor_json(d: &Divisor) -> Value {
    json!({"u":d.0.0.iter().map(|x|x.to_string()).collect::<Vec<_>>(),
        "v":d.1.0.iter().map(|x|x.to_string()).collect::<Vec<_>>()})
}
fn eigen_image(q: &Point, c: &Curve, j: &Jacobian, zeta: &BigUint, lambda: &BigUint) -> Divisor {
    let left = transfer(3, q, &c.p);
    let right = rotate(&transfer(3, &c.mul(lambda, q), &c.p), zeta, &c.p);
    j.add(&left, &j.neg(&right))
}
pub fn symmetry_image() -> Value {
    let (p, b, n, g) = parameters();
    let c = Curve {
        p: p.clone(),
        a: &p - 3u32,
        b: b.clone(),
    };
    let j = Jacobian::new(3, &b, &p);
    let zeta = cube_root_of_unity(&p);
    let lambda = cube_root_of_unity(&n);
    assert_eq!((&lambda * &lambda + &lambda + 1u32) % &n, BigUint::zero());
    let a = transfer(3, &g, &p);
    let sa = rotate(&a, &zeta, &p);
    let ssa = rotate(&sa, &zeta, &p);
    assert_eq!(rotate(&ssa, &zeta, &p), a);
    assert_eq!(j.add(&j.add(&a, &sa), &ssa), j.zero());
    let minus_half = sub(&BigUint::zero(), &inv(&2u32.into(), &n), &n);
    assert_ne!(sa, transfer(3, &c.mul(&minus_half, &g), &p));
    let image = eigen_image(&g, &c, &j, &zeta, &lambda);
    assert!(j.valid(&image));
    assert_ne!(image, j.zero());
    assert_eq!(j.mul(&n, &image), j.zero());
    assert_eq!(rotate(&image, &zeta, &p), j.mul(&lambda, &image));
    // A pure endomorphism walk D -> D+S(D) stays in the same signed orbit.
    // A candidate solver must use independent increments and handle short cycles.
    let si = rotate(&image, &zeta, &p);
    let ssi = rotate(&si, &zeta, &p);
    assert_eq!(j.add(&image, &si), j.neg(&ssi));
    assert_eq!(j.add(&image, &ssi), j.neg(&si));
    let mut orbit = vec![];
    let mut current = image.clone();
    for _ in 0..3 {
        orbit.push(current.clone());
        orbit.push(j.neg(&current));
        current = rotate(&current, &zeta, &p);
    }
    assert_eq!(current, image);
    for i in 0..6 {
        for k in 0..i {
            assert_ne!(orbit[i], orbit[k]);
        }
    }
    let mut checked = vec![];
    for scalar in [0u32, 1, 2, 37] {
        let q = c.mul(&scalar.into(), &g);
        let iq = eigen_image(&q, &c, &j, &zeta, &lambda);
        assert!(j.valid(&iq));
        assert_eq!(iq, j.mul(&scalar.into(), &image));
        assert_eq!(
            eigen_image(&c.add(&q, &g), &c, &j, &zeta, &lambda),
            j.add(&iq, &image)
        );
        assert_eq!(rotate(&iq, &zeta, &p), j.mul(&lambda, &iq));
        checked.push(scalar);
    }
    let norm_multiplier = (&lambda + 2u32) % &n;
    assert!(!norm_multiplier.is_zero());
    json!({"formula":"i_lambda(P)=alpha_0(P)-alpha_1([lambda]P), alpha_1=rho^* alpha_0",
        "rho":"(t,z) -> (zeta*t,zeta^2*z)","zeta_mod_p":zeta.to_string(),"lambda_mod_n":lambda.to_string(),
        "subgroup_order":n.to_string(),"generator_image":divisor_json(&image),"orbit_size_with_negation":orbit.len(),
        "checked_public_scalars":checked,"norm_on_image_multiplier":norm_multiplier.to_string(),
        "general_divisor_norm_implemented":false,"scalar_recovery":"Destination log is the same scalar; verify it on P-256",
        "ideal_walk_step_ratio_vs_negation":"1/sqrt(3), assuming a uniform quotient walk and fully handled short cycles",
        "break_even_per_step_cost":"Destination addition plus canonicalization must cost less than sqrt(3) times the matched source step; charge transport and recovery separately",
        "product_control":"E x E supports the same finite eigenline using a 2x2 integer matrix; componentwise addition uses two E additions, so the free-fold count model costs 2/sqrt(3) source-addition units",
        "novelty":"General finite-subgroup eigenline construction, not a curve-specific hidden trapdoor or a novelty claim",
        "destination_solver":"Automorphism-folded Pollard rho candidate; cycle-safe solver and matched complete-cost run not implemented",
        "measured_speedup":null,"standing":"producer actual-field divisor checks; heuristic walk count only, no signed geometric or runtime verdict"})
}

pub fn identity_certificates() -> Value {
    use p256_genus_check::identity_certificate::{
        check_with_extra, issue, Expr as E, IdentityStatement, Verdict,
    };
    fn pow(e: E, n: usize) -> E {
        E::product(vec![e; n])
    }
    fn scale(c: i64, e: E) -> E {
        E::times(E::Const(c), e)
    }
    let (x, u, s) = (E::var("x"), E::var("u"), E::var("s"));
    // Parameterize an inflection kernel Q=(3u^2,s), tangent slope 3u.
    // This gives a universal polynomial identity without a quotient-ring assumption.
    let r = scale(3, pow(u.clone(), 2));
    let a = E::sum(vec![
        scale(6, E::times(u.clone(), s.clone())),
        scale(-27, pow(u.clone(), 4)),
    ]);
    let b = E::sum(vec![
        pow(s.clone(), 2),
        scale(-18, E::times(pow(u.clone(), 3), s.clone())),
        scale(54, pow(u.clone(), 6)),
    ]);
    let v = scale(12, E::times(u, s.clone()));
    let w = scale(4, pow(s, 2));
    let d = E::plus(x.clone(), E::minus(r.clone()));
    let n = E::sum(vec![
        E::times(x.clone(), pow(d.clone(), 2)),
        E::times(v.clone(), d.clone()),
        w.clone(),
    ]);
    let m = E::sum(vec![
        pow(d.clone(), 3),
        E::minus(E::times(v.clone(), d.clone())),
        scale(-2, w.clone()),
    ]);
    let aa = E::plus(a.clone(), scale(-5, v.clone()));
    let bb = E::plus(b.clone(), scale(-7, E::plus(w, E::times(r, v))));
    let f = E::sum(vec![pow(x.clone(), 3), E::times(a, x), b]);
    let lhs = E::times(f, pow(m, 2));
    let rhs = E::sum(vec![
        pow(n.clone(), 3),
        E::product(vec![aa, n, pow(d.clone(), 4)]),
        E::times(bb, pow(d, 6)),
    ]);
    let st=IdentityStatement{name:"Normalized degree-three Vélu cleared curve identity".into(),
        source:"Standard Vélu formula; inflection parameterization and exact specialization in transfer_search/REPORT.md".into(),
        variables:vec!["x".into(),"u".into(),"s".into()],degree_bound:18,lhs,rhs};
    let cert=issue(st,2026111505,32,"Producer record using the unchanged existing checker; F61 identity replay, not an actual-P256 or signed geometric proof",None).unwrap();
    assert!(matches!(
        check_with_extra(&cert, Some((0x72cc51, 16))).unwrap(),
        Verdict::Accept { .. }
    ));
    let t = E::var("t");
    let b = E::var("b");
    let z = E::var("zeta");
    let lhs = E::sum(vec![
        pow(E::times(z.clone(), t.clone()), 7),
        E::times(b.clone(), pow(E::times(z.clone(), t.clone()), 4)),
        E::times(z.clone(), t.clone()),
        E::minus(E::times(
            z.clone(),
            E::sum(vec![
                pow(t.clone(), 7),
                E::times(b.clone(), pow(t.clone(), 4)),
                t.clone(),
            ]),
        )),
    ]);
    let rhs = E::product(vec![
        z.clone(),
        E::plus(pow(z.clone(), 3), E::Const(-1)),
        E::plus(
            E::times(E::plus(pow(z, 3), E::Const(1)), pow(t.clone(), 7)),
            E::times(b, pow(t, 4)),
        ),
    ]);
    let rotation = IdentityStatement {
        name: "C3 rotation preserves the curve when zeta cubed is one".into(),
        source: "transfer_search/REPORT.md; factorized universal polynomial relation".into(),
        variables: vec!["t".into(), "b".into(), "zeta".into()],
        degree_bound: 14,
        lhs,
        rhs,
    };
    let rotation = issue(
        rotation,
        2026111506,
        32,
        "Producer algebraic identity record, not a signed geometric verdict",
        None,
    )
    .unwrap();
    assert!(matches!(
        check_with_extra(&rotation, Some((0x72cc52, 16))).unwrap(),
        Verdict::Accept { .. }
    ));
    json!([cert, rotation])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kernel_enumeration_and_controls() {
        assert_eq!(matchings(&(0..6).collect::<Vec<_>>()).len(), 15);
        assert_eq!(matchings(&(0..8).collect::<Vec<_>>()).len(), 105);
        let a = c3_kernels(&p256_cover_classification::actual_audit());
        assert_eq!(a["rational_lagrangians"], 9);
        assert_eq!(a["covered_by_elliptic_plane"], 9);
        assert!(a["kernels"]
            .as_array()
            .unwrap()
            .iter()
            .all(|k| k["elliptic_planes_contained"].as_array().unwrap().len() == 1));
        let planes: Vec<[u8; 3]> =
            serde_json::from_value(a["elliptic_two_torsion_planes"].clone()).unwrap();
        let all = sk::lagrangians();
        assert!(all
            .iter()
            .any(|(k, _)| !planes.iter().any(|w| contains_plane(k, w))));
        let relabel = [7, 6, 5, 4, 3, 2, 1, 0];
        for (k, _) in &all {
            let mut new = k.map(|x| permute(x, &relabel));
            new.sort_unstable();
            for w in &planes {
                let ww = w.map(|x| permute(x, &relabel));
                assert_eq!(contains_plane(k, w), contains_plane(&new, &ww));
            }
        }
    }
    #[test]
    fn actual_three_transfer() {
        let a = rational_three_kernel();
        assert_eq!(a["rational_kernel_x_degree"], 1);
        assert_eq!(a["map"]["cleared_curve_identity"], true);
        let a = c2_kernels();
        assert_eq!(a["factor_degrees"], json!([6]));
        assert_eq!(a["rational_richelot_kernels"], 1);
        identity_certificates();
        let f2 = companion_screen();
        assert_eq!(f2["trace_mod_3"], 2);
        assert_eq!(f2["small_torsion_witness"]["exact_order"], 3);
    }
    #[test]
    fn actual_automorphism_eigenline() {
        let a = symmetry_image();
        assert_eq!(a["orbit_size_with_negation"], 6);
        assert!(a["measured_speedup"].is_null());
    }
    #[test]
    fn small_field_exhaustive_maps_and_negative_controls() {
        for pp in [7u32, 13, 19, 31] {
            let p = BigUint::from(pp);
            let a = &p - 3u32;
            let mut chosen = None;
            'outer: for bb in 1..pp {
                let b = BigUint::from(bb);
                if (4u32 * &a * &a * &a + 27u32 * &b * &b) % &p == BigUint::zero() {
                    continue;
                }
                let c = Curve {
                    p: p.clone(),
                    a: a.clone(),
                    b,
                };
                for rr in 0..pp {
                    if let Ok(m) = ThreeMap::new(c.clone(), rr.into()) {
                        chosen = Some(m);
                        break 'outer;
                    }
                }
            }
            let m = chosen.unwrap();
            let mut points = vec![None];
            for x in 0..pp {
                for y in 0..pp {
                    let q = Some((x.into(), y.into()));
                    if m.source.contains(&q) {
                        points.push(q);
                    }
                }
            }
            let images: Vec<_> = points.iter().map(|q| m.eval(q).unwrap()).collect();
            for (i, q) in points.iter().enumerate() {
                assert_eq!(m.dual(&images[i]).unwrap(), m.source.mul(&3u32.into(), q));
                for (j, r) in points.iter().enumerate() {
                    assert_eq!(
                        m.eval(&m.source.add(q, r)).unwrap(),
                        m.target.add(&images[i], &images[j])
                    );
                }
            }
            let mut wrong = m.clone();
            wrong.target.b = (&wrong.target.b + 1u32) % &p;
            assert!(!wrong.cleared_identity());
            assert!(m.eval(&Some((p.clone(), BigUint::one()))).is_err());
        }
    }
}
