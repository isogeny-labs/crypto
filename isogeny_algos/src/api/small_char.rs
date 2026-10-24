//! Entry point for the isogeny between two given curves over a field of small characteristic:
//! GF(2^n) (Couveignes' 2-torsion algorithm or the 2-adic lift), GF(3^n) and F_p (the p-adic
//! lift of Lercier–Sirvent). The checks recompute from the kernel polynomial alone: it divides
//! the l-division polynomial, Kohel's codomain has the requested j-invariant, and the x-map
//! commutes with doubling on random rational points.
use super::*;
use crate::binary::{self, BinCurve};
use crate::find::p_torsion::couveignes_binary;
use crate::find::small_char::{binary_kernel, generic_kernel, ternary_kernel, Residue};
use crate::gf2n::GF2n;
use crate::gf3n::GF3n;
use crate::poly;
use crate::weier::{self, GWCurve};

/// The field of a small-characteristic problem.
#[derive(Clone, Copy, Debug)]
pub enum SmallField {
    /// GF(2^n) in the crate's default polynomial basis; elements as bit patterns
    Binary { n: u32 },
    /// GF(3^n); elements packed base 3 (digit i = coefficient of t^i)
    Ternary { n: u32 },
    /// F_p for a prime p (any size below 2^62; meant for p <= 4l)
    Prime { p: u64 },
}

/// The l-isogeny from E (general Weierstrass [a1, a2, a3, a4, a6]) to a curve of j-invariant j~.
#[derive(Clone, Debug)]
pub struct SmallCharRecord {
    pub field: String,
    pub ell: u64,
    pub method: String,
    /// monic kernel polynomial, lowest coefficient first, elements encoded as in [`SmallField`]
    pub kernel: Vec<u64>,
    /// [a1, a2, a3, a4, a6] of Kohel's codomain
    pub codomain: [u64; 5],
    pub codomain_j: u64,
    /// method statistics (precision and attempts, or 2^k-torsion, tower degree and trials)
    pub detail: String,
    pub checks: Vec<Check>,
}

fn pass(name: &str, ok: bool, detail: String) -> Check {
    Check::new(name, if ok { Status::Pass } else { Status::Fail }, detail)
}

/// Checks over a field through the general Weierstrass formulas (odd characteristic).
fn gw_checks<F: Field>(
    f: &F,
    e: &GWCurve<F::E>,
    h: &crate::poly::Poly<F>,
    jt: F::E,
    ell: u64,
    rng: &mut Rng,
) -> (Vec<Check>, GWCurve<F::E>) {
    let psi = weier::division_poly(f, e, ell as usize);
    let divides = poly::rem(f, &psi, h).iter().all(|&c| f.is_zero(c));
    let cod = weier::kohel_codomain(f, e, h);
    let xs = weier::random_abscissas(f, e, 8, rng);
    let commutes = weier::is_kernel(f, e, h, &xs);
    (
        vec![
            pass(
                "kernel_divides_division_polynomial",
                divides,
                format!("deg psi_{ell} = {}", psi.len() - 1),
            ),
            pass(
                "codomain_j",
                cod.j(f) == jt,
                "j(Kohel codomain) = j~".into(),
            ),
            pass(
                "x_map_commutes_with_doubling",
                commutes,
                format!("{} random abscissas", xs.len()),
            ),
        ],
        cod,
    )
}

/// The l-isogeny (l odd prime) from `curve` = [a1, a2, a3, a4, a6] to a curve of j-invariant
/// `jt` over a small-characteristic field. `method`: "lift" (all fields) or "couveignes"
/// (GF(2^n) only); "auto" takes Couveignes in characteristic 2, the lift otherwise.
pub fn small_char_isogeny(
    field: SmallField,
    curve: [u64; 5],
    jt: u64,
    ell: u64,
    method: &str,
    seed: u64,
) -> Result<SmallCharRecord, String> {
    if ell < 3 || !crate::field::is_prime(ell) {
        return Err(format!("l = {ell} must be an odd prime"));
    }
    let mut rng = Rng::new(seed);
    match field {
        SmallField::Binary { n } => {
            if crate::gf2n::default_tail(n).is_none() {
                return Err(format!("no tabulated GF(2^{n})"));
            }
            let f = GF2n::new(n);
            let [a1, a2, a3, a4, a6] = curve;
            if a1 != 1 || a3 != 0 {
                return Err(
                    "binary curves are y^2 + xy = x^3 + a2 x^2 + a4 x + a6 (a1 = 1, a3 = 0)".into(),
                );
            }
            let e = BinCurve::new(a2, a4, a6);
            if e.disc(&f) == 0 {
                return Err("singular curve".into());
            }
            let m = if method == "auto" {
                "couveignes"
            } else {
                method
            };
            let (h, detail) = match m {
                "couveignes" => {
                    couveignes_binary(&f, &e, jt, ell as usize, &mut rng).map(|(h, o)| {
                        (
                            h,
                            format!(
                                "2^{}-torsion, tower degree {}, {} trials",
                                o.k, o.tower_degree, o.trials
                            ),
                        )
                    })
                }
                "lift" => binary_kernel(&f, &e, jt, ell as usize, &mut rng).map(|(h, o)| {
                    (
                        h,
                        format!("2-adic precision {}, {} attempts", o.precision, o.attempts),
                    )
                }),
                _ => return Err(format!("unknown method {method} (lift, couveignes, auto)")),
            }
            .ok_or_else(|| format!("no {ell}-isogeny to j~ = {jt} found by {m}"))?;
            let psi = binary::division_poly(&f, &e.normalized(&f), ell as usize);
            let divides = poly::rem(&f, &psi, &h).iter().all(|&c| c == 0);
            let cod = binary::kohel_codomain(&f, &e, &h);
            let commutes = binary::is_kernel(&f, &e, &h, &mut rng);
            Ok(SmallCharRecord {
                field: format!("gf2-{n}"),
                ell,
                method: m.into(),
                kernel: h,
                codomain: [1, cod.a2, 0, cod.a4, cod.a6],
                codomain_j: cod.j(&f),
                detail,
                checks: vec![
                    pass(
                        "kernel_divides_division_polynomial",
                        divides,
                        format!("deg psi_{ell} = {}", psi.len() - 1),
                    ),
                    pass(
                        "codomain_j",
                        cod.j(&f) == jt,
                        "j(Kohel codomain) = j~".into(),
                    ),
                    pass(
                        "x_map_commutes_with_doubling",
                        commutes,
                        "random points".into(),
                    ),
                ],
            })
        }
        SmallField::Ternary { n } => {
            if !(1..=40).contains(&n) {
                return Err("GF(3^n) needs 1 <= n <= 40".into());
            }
            if method != "lift" && method != "auto" {
                return Err(format!("unknown method {method} for GF(3^n) (lift)"));
            }
            let f = GF3n::new(n);
            let e = GWCurve::new(curve);
            if f.is_zero(e.disc(&f)) {
                return Err("singular curve".into());
            }
            let (h, o) = ternary_kernel(&f, &e, jt, ell as usize, &mut rng)
                .ok_or_else(|| format!("no {ell}-isogeny to j~ = {jt} found by the 3-adic lift (j~ must be a simple root of Phi_l(j, Y) mod 3)"))?;
            let (checks, cod) = gw_checks(&f, &e, &h, jt, ell, &mut rng);
            Ok(SmallCharRecord {
                field: format!("gf3-{n}"),
                ell,
                method: "lift".into(),
                kernel: h,
                codomain: [cod.a1, cod.a2, cod.a3, cod.a4, cod.a6],
                codomain_j: cod.j(&f),
                detail: format!("3-adic precision {}, {} attempts", o.precision, o.attempts),
                checks,
            })
        }
        SmallField::Prime { p } => {
            if !crate::field::is_prime(p) || !(5..1 << 62).contains(&p) {
                return Err(format!("p = {p} must be a prime in [5, 2^62)"));
            }
            if method != "lift" && method != "auto" {
                return Err(format!("unknown method {method} for F_p (lift)"));
            }
            let f = Zp::new(p);
            let e = GWCurve::new(curve.map(|c| c % p));
            if f.is_zero(e.disc(&f)) {
                return Err("singular curve".into());
            }
            let res = Residue {
                p,
                modulus: vec![0, 1],
            };
            let (h, o) = generic_kernel(&f, &res, &e, jt % p, ell as usize, |x| vec![x], |d| d[0] % p, &mut rng)
                .ok_or_else(|| format!("no {ell}-isogeny to j~ = {jt} found by the p-adic lift (j~ must be a simple root of Phi_l(j, Y) mod p)"))?;
            let (checks, cod) = gw_checks(&f, &e, &h, jt % p, ell, &mut rng);
            Ok(SmallCharRecord {
                field: format!("fp-{p}"),
                ell,
                method: "lift".into(),
                kernel: h,
                codomain: [cod.a1, cod.a2, cod.a3, cod.a4, cod.a6],
                codomain_j: cod.j(&f),
                detail: format!("p-adic precision {}, {} attempts", o.precision, o.attempts),
                checks,
            })
        }
    }
}
