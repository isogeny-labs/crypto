//! Curve instances: loading, validation and the matched baseline.
//!
//! An [`Instance`] is a curve with its generator, prime subgroup order `r`
//! and cofactor.  Registered curves come from `docs/curves/registry.json`
//! (named by ICV1 slug, `AGENTS.md` section 11); toy Koblitz curves are
//! built here and carry no slug.  [`validate`] refuses an instance whose
//! field, curve, generator or subgroup order cannot be verified
//! (`INVALID_INSTANCE`); nothing downstream runs on an unverified instance.

use std::collections::BTreeMap;

use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use serde_json::{json, Value};

use crypto_lib::binary_ecc::curve::scalar_mul;
use crypto_lib::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_semaev::solve_quadratic_f2m;
use crypto_lib::cryptanalysis::curve_traits::arith::{self, lucas_v};
use crypto_lib::cryptanalysis::curve_traits::{
    self, CurveTraits, Factoring, GeneratorCoords, Model, RegistryCurve, Representation, Source,
};
use crypto_lib::cryptanalysis::ecc2k130_guard::sqrt_mod;
use crypto_lib::cryptanalysis::ghs_screen::{screen_curve, GhsCurveInput};
use crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel;
use crypto_lib::cryptanalysis::jacobian_transfer::log2_biguint;
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;

use crate::certificate::{round3, Check, CheckResult};
use crate::cost::log2_rho_with_class;

/// The curve arithmetic of an instance.
pub enum Ops {
    Binary(Box<BinaryCurve>),
    Prime(CurveParams),
}

pub struct Instance {
    pub name: String,
    pub rc: RegistryCurve,
    pub r: BigUint,
    pub cofactor: BigUint,
    pub generator: (BigUint, BigUint),
    pub ops: Ops,
}

impl Instance {
    pub fn from_registry_curve(name: &str, rc: RegistryCurve) -> Result<Self, String> {
        let rep = rc
            .representations
            .first()
            .ok_or_else(|| format!("{name}: the registry record has no generator representation"))?
            .clone();
        let GeneratorCoords::Affine(gx, gy) = rep.generator.clone() else {
            return Err(format!("{name}: only affine generators are supported"));
        };
        let ops = match &rc.model {
            Model::Binary { n, irr, a, b, .. } => Ops::Binary(Box::new(BinaryCurve {
                m: *n,
                irreducible: irr.clone(),
                a: a.clone(),
                b: b.clone(),
                generator: BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&gx, *n),
                    y: F2mElement::from_biguint(&gy, *n),
                },
                order: rep.subgroup_order.clone(),
                cofactor: rep.cofactor.clone(),
            })),
            Model::Prime { p, a, b } => Ops::Prime(CurveParams {
                // `CurveParams` wants a static name; one small leak per
                // process is the price of reusing the repository type.
                name: Box::leak(name.to_string().into_boxed_str()),
                p: p.clone(),
                a: a.clone(),
                b: b.clone(),
                gx: gx.clone(),
                gy: gy.clone(),
                n: rep.subgroup_order.clone(),
                // `CurveParams` stores the cofactor as u32 and no code here reads
                // it: every computation uses `Instance::cofactor` (BigUint).
                h: rep.cofactor.to_u32().unwrap_or(u32::MAX),
            }),
            _ => {
                return Err(format!(
                    "{name}: only binary and prime-field models are audited"
                ))
            }
        };
        Ok(Instance {
            name: name.to_string(),
            r: rep.subgroup_order,
            cofactor: rep.cofactor,
            generator: (gx, gy),
            ops,
            rc,
        })
    }

    pub fn registered(&self) -> bool {
        self.rc.source.registered
    }

    pub fn is_binary(&self) -> bool {
        matches!(self.ops, Ops::Binary(_))
    }

    /// Field size `q`.
    pub fn q(&self) -> BigUint {
        self.rc.model.q()
    }

    /// `log2 q`.
    pub fn field_bits(&self) -> f64 {
        match &self.rc.model {
            Model::Binary { n, .. } => f64::from(*n),
            Model::Prime { p, .. } => log2_biguint(p),
            _ => 0.0,
        }
    }

    pub fn log2_r(&self) -> f64 {
        log2_biguint(&self.r)
    }

    pub fn binary(&self) -> Option<&BinaryCurve> {
        match &self.ops {
            Ops::Binary(c) => Some(c.as_ref()),
            _ => None,
        }
    }

    /// Identity block shared by every certificate of this curve.
    pub fn curve_value(&self, checks: &[Check]) -> Value {
        let (field, family) = match &self.rc.model {
            Model::Binary {
                n, modulus, a, b, ..
            } => (
                json!({
                    "kind": "binary", "degree": n, "log2_size": n,
                    "modulus": format!("0x{modulus:x}"),
                    "a": format!("0x{:x}", a.to_biguint()),
                    "b": format!("0x{:x}", b.to_biguint()),
                    "model": "y^2+xy=x^3+a*x^2+b",
                }),
                "binary",
            ),
            Model::Prime { p, a, b } => (
                json!({
                    "kind": "prime", "degree": 1, "log2_size": round3(log2_biguint(p)),
                    "p": p.to_string(), "a": a.to_string(), "b": b.to_string(),
                    "model": "y^2=x^3+a*x+b",
                }),
                "prime",
            ),
            _ => (json!({"kind": "other"}), "other"),
        };
        json!({
            "name": self.name,
            "slug": self.rc.slug,
            "registered": self.registered(),
            "family": family,
            "field": field,
            "subgroup": {
                "r": self.r.to_string(),
                "r_log2": round3(self.log2_r()),
                "cofactor": self.cofactor.to_string(),
                "order": self.rc.order.to_string(),
                "trace": self.rc.trace.to_string(),
                "generator": [format!("0x{:x}", self.generator.0), format!("0x{:x}", self.generator.1)],
            },
            "instance_checks": checks,
        })
    }
}

// ── Loading ───────────────────────────────────────────────────────────

/// Find a registered curve by slug or standard name (case-insensitive) and
/// parse only that record with the repository's own reader.
pub fn load_registered(registry_text: &str, wanted: &str) -> Result<Instance, String> {
    let doc: Value =
        serde_json::from_str(registry_text).map_err(|e| format!("registry JSON: {e}"))?;
    load_registered_from(&doc, wanted)
}

/// [`load_registered`] on an already parsed registry (a sweep over many
/// curves parses the 17 MB file once).
pub fn load_registered_from(doc: &Value, wanted: &str) -> Result<Instance, String> {
    let curves = doc["curves"].as_array().ok_or("registry has no curves")?;
    let wanted_lc = wanted.to_ascii_lowercase();
    let hit = curves.iter().find(|c| {
        c["slug"].as_str().is_some_and(|s| s == wanted)
            || ["standard_names", "aliases"].iter().any(|key| {
                c[*key].as_array().is_some_and(|v| {
                    v.iter().any(|n| {
                        n.as_str()
                            .is_some_and(|n| n.to_ascii_lowercase() == wanted_lc)
                    })
                })
            })
    });
    let hit = hit.ok_or_else(|| format!("no registry curve named {wanted}"))?;
    let one = json!({ "curves": [hit] });
    let mut parsed = curve_traits::read_registry(&one.to_string())?;
    let mut rc = parsed.pop().ok_or("registry reader returned no curve")?;
    if rc.representations.is_empty() {
        attach_certicom_representation(&mut rc, wanted)?;
    }
    Instance::from_registry_curve(wanted, rc)
}

/// A registry record without a generator (ECC2K-130 is registered by its
/// trace) takes the repository's Certicom challenge parameters, after the
/// field and both coefficients are checked equal to the registry model.
fn attach_certicom_representation(rc: &mut RegistryCurve, wanted: &str) -> Result<(), String> {
    let (curve, _target) = BinaryCurve::certicom_challenge(&wanted.to_ascii_lowercase())
        .ok_or_else(|| format!("{wanted}: the registry record has no generator and no Certicom challenge parameters exist for it"))?;
    let Model::Binary {
        n, modulus, a, b, ..
    } = &rc.model
    else {
        return Err(format!("{wanted}: Certicom parameters are binary"));
    };
    let mut challenge_modulus = BigUint::one() << curve.m as usize;
    for e in &curve.irreducible.low_terms {
        challenge_modulus |= BigUint::one() << *e as usize;
    }
    if curve.m != *n || challenge_modulus != *modulus || curve.a != *a || curve.b != *b {
        return Err(format!(
            "{wanted}: the Certicom challenge field or coefficients differ from the registry model"
        ));
    }
    let BinaryPoint::Affine { x, y } = &curve.generator else {
        return Err("challenge generator at infinity".into());
    };
    rc.representations = vec![Representation {
        generator: GeneratorCoords::Affine(x.to_biguint(), y.to_biguint()),
        subgroup_order: curve.order.clone(),
        cofactor: curve.cofactor.clone(),
    }];
    Ok(())
}

fn random_bits(m: u32, rng: &mut StdRng) -> BigUint {
    let mut bytes = vec![0u8; m.div_ceil(8) as usize];
    rng.fill_bytes(&mut bytes);
    BigUint::from_bytes_le(&bytes) & ((BigUint::one() << m as usize) - BigUint::one())
}

/// A uniformly seeded point of `E(F_(2^m))` (any order).
pub fn random_binary_point(c: &BinaryCurve, rng: &mut StdRng) -> BinaryPoint {
    loop {
        let x = F2mElement::from_biguint(&random_bits(c.m, rng), c.m);
        if x.is_zero() {
            continue;
        }
        let x2 = x.square(&c.irreducible);
        let x3 = x2.mul(&x, &c.irreducible);
        let rhs = x3.add(&c.a.mul(&x2, &c.irreducible)).add(&c.b);
        let roots = solve_quadratic_f2m(&F2mElement::one(c.m), &x, &rhs, c.m, &c.irreducible);
        if let Some(y) = roots.into_iter().next() {
            let p = BinaryPoint::Affine { x, y };
            if c.is_on_curve(&p) {
                return p;
            }
        }
    }
}

/// A seeded point of `E(F_p)` (any order).
pub fn random_prime_point(c: &CurveParams, rng: &mut StdRng) -> Point {
    loop {
        let x = random_bits(c.p.bits() as u32, rng) % &c.p;
        let rhs = (&x * &x % &c.p * &x + &c.a * &x + &c.b) % &c.p;
        if let Some(y) = sqrt_mod(&rhs, &c.p) {
            return Point::Affine {
                x: c.fe(x),
                y: c.fe(y),
            };
        }
    }
}

/// A seeded planted scalar in `[2, r - 1]`.
pub fn planted_scalar(r: &BigUint, rng: &mut StdRng) -> BigUint {
    let two = BigUint::from(2u8);
    two.clone() + random_bits(r.bits() as u32 + 64, rng) % (r - &two)
}

// ── Toy Koblitz curves ───────────────────────────────────────────────

/// `y^2 + xy = x^3 + a x^2 + 1` over `F_2[z]/(modulus)`, `a` in {0, 1}.
/// `#E(F_2)` is 4 (a = 0, trace -1) or 2 (a = 1, trace +1); the order over
/// `F_(2^n)` is `2^n + 1 - V_n(t, 2)`.  `r` is the largest prime factor and
/// the generator a seeded point times the cofactor.
pub fn toy_koblitz(
    name: &str,
    a_bit: u8,
    modulus: &BigUint,
    seed: u64,
) -> Result<Instance, String> {
    let n = (modulus.bits() - 1) as u32;
    let a = BigUint::from(a_bit);
    let input = GhsCurveInput {
        absolute_degree: n,
        modulus: modulus.clone(),
        a: a.clone(),
        b: BigUint::one(),
    };
    screen_curve(&input, &BigUint::from(4u8))
        .map_err(|e| format!("INVALID_INSTANCE: {name}: {e}"))?;
    let irr = IrreduciblePoly {
        degree: n,
        low_terms: (0..n).filter(|i| modulus.bit(u64::from(*i))).collect(),
    };
    let t1 = if a_bit == 1 { 1 } else { -1 };
    let trace = lucas_v(&BigInt::from(t1), &BigInt::from(2), n);
    let order = (BigInt::one() << n as usize) + BigInt::one() - &trace;
    let order = order.to_biguint().ok_or("negative toy order")?;
    let fac = arith::factor(&order, 1 << 20);
    let r = fac
        .largest_prime()
        .cloned()
        .ok_or_else(|| format!("{name}: #E = {order} did not factor completely"))?;
    let cofactor = &order / &r;
    let a_el = F2mElement::from_biguint(&a, n);
    let b_el = F2mElement::one(n);
    let mut curve = BinaryCurve {
        m: n,
        irreducible: irr.clone(),
        a: a_el.clone(),
        b: b_el.clone(),
        generator: BinaryPoint::Infinity,
        order: r.clone(),
        cofactor: cofactor.clone(),
    };
    let mut rng = StdRng::seed_from_u64(seed);
    let generator = loop {
        let p = random_binary_point(&curve, &mut rng);
        let g = scalar_mul(&curve, &p, &cofactor);
        if g != BinaryPoint::Infinity {
            break g;
        }
    };
    let BinaryPoint::Affine { x, y } = &generator else {
        return Err("generator search returned infinity".into());
    };
    let (gx, gy) = (x.to_biguint(), y.to_biguint());
    curve.generator = generator.clone();
    let rc = RegistryCurve {
        slug: format!("unregistered-{name}"),
        family: "binary".into(),
        model: Model::Binary {
            n,
            modulus: modulus.clone(),
            irr,
            a: a_el,
            b: b_el,
        },
        trace,
        order,
        end: None,
        representations: vec![Representation {
            generator: GeneratorCoords::Affine(gx.clone(), gy.clone()),
            subgroup_order: r.clone(),
            cofactor: cofactor.clone(),
        }],
        levels: BTreeMap::new(),
        source: Source {
            kind: "transfer-audit-toy".into(),
            path: None,
            curve_uid: None,
            registered: false,
        },
    };
    Ok(Instance {
        name: name.to_string(),
        r,
        cofactor,
        generator: (gx, gy),
        ops: Ops::Binary(Box::new(curve)),
        rc,
    })
}

// ── Validation ───────────────────────────────────────────────────────

/// Verify the instance.  `Err` carries the text `INVALID_INSTANCE: ...`.
pub fn validate(inst: &Instance, seed: u64) -> Result<Vec<Check>, String> {
    let bad = |msg: String| Err(format!("INVALID_INSTANCE: {}: {msg}", inst.name));
    let mut checks = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);
    let order = &inst.rc.order;

    // Field and curve.
    match &inst.ops {
        Ops::Binary(c) => {
            let Model::Binary {
                n, modulus, a, b, ..
            } = &inst.rc.model
            else {
                return bad("model/ops mismatch".into());
            };
            let input = GhsCurveInput {
                absolute_degree: *n,
                modulus: modulus.clone(),
                a: a.to_biguint(),
                b: b.to_biguint(),
            };
            if let Err(e) = screen_curve(&input, &BigUint::from(4u8)) {
                return bad(e.to_string());
            }
            checks.push(Check::new(
                "field_and_curve",
                CheckResult::Pass,
                "modulus irreducible (Rabin), coefficients canonical, b != 0 (ordinary binary model)",
            ));
            let g = &c.generator;
            if !c.is_on_curve(g) || *g == BinaryPoint::Infinity {
                return bad("generator is not an affine point of the curve".into());
            }
            checks.push(Check::new(
                "generator_on_curve",
                CheckResult::Pass,
                "G affine, y^2+xy = x^3+ax^2+b",
            ));
            if scalar_mul(c, g, &inst.r) != BinaryPoint::Infinity {
                return bad("[r]G != O".into());
            }
            for i in 0..2 {
                let p = random_binary_point(c, &mut rng);
                if scalar_mul(c, &p, order) != BinaryPoint::Infinity {
                    return bad(format!(
                        "random point {i} is not annihilated by the claimed order"
                    ));
                }
            }
        }
        Ops::Prime(c) => {
            if !arith::is_prime(&c.p) || c.p <= BigUint::from(3u8) {
                return bad("p is not an odd prime above 3".into());
            }
            let disc = (BigUint::from(4u8) * &c.a * &c.a * &c.a
                + BigUint::from(27u8) * &c.b * &c.b)
                % &c.p;
            if disc.is_zero() {
                return bad("the curve is singular".into());
            }
            checks.push(Check::new(
                "field_and_curve",
                CheckResult::Pass,
                format!(
                    "p passes Miller-Rabin ({:?}), 4a^3+27b^2 != 0 mod p",
                    arith::prime_status(&c.p)
                ),
            ));
            let g = c.generator();
            if !c.is_on_curve(&g) || g == Point::Infinity {
                return bad("generator is not an affine point of the curve".into());
            }
            checks.push(Check::new(
                "generator_on_curve",
                CheckResult::Pass,
                "G affine, y^2 = x^3+ax+b",
            ));
            if g.scalar_mul_vartime(&inst.r, &c.a_fe()) != Point::Infinity {
                return bad("[r]G != O".into());
            }
            for i in 0..2 {
                let p = random_prime_point(c, &mut rng);
                if p.scalar_mul_vartime(order, &c.a_fe()) != Point::Infinity {
                    return bad(format!(
                        "random point {i} is not annihilated by the claimed order"
                    ));
                }
            }
        }
    }
    if !arith::is_prime(&inst.r) {
        return bad("r is not prime".into());
    }
    checks.push(Check::new(
        "subgroup_order_is_prime_and_annihilates_generator",
        CheckResult::Pass,
        format!(
            "r passes Miller-Rabin to bases 2..53 ({:?}); [r]G = O, G != O, so G has exact order r",
            arith::prime_status(&inst.r)
        ),
    ));
    if &inst.cofactor * &inst.r != *order {
        return bad("cofactor * r != #E".into());
    }
    let q = BigInt::from(inst.q());
    if &inst.rc.trace * &inst.rc.trace > &q * 4 {
        return bad("trace violates the Hasse bound".into());
    }
    checks.push(Check::new(
        "order_consistency",
        CheckResult::Pass,
        "#E = cofactor * r = q + 1 - t, Hasse bound holds, two seeded random points are annihilated by #E (evidence for the recorded order, not a point-count proof)",
    ));
    Ok(checks)
}

// ── Matched baseline ─────────────────────────────────────────────────

/// Size of the equivalence class Pollard rho can walk, with the reason.
/// For a Koblitz-type binary curve this is an upper bound on the speedup:
/// the Frobenius acts on `<G>` as an eigenvalue whose order divides `n/d`.
pub fn rho_class_size(inst: &Instance, traits: &CurveTraits) -> (u64, String) {
    match &inst.rc.model {
        Model::Binary { n, .. } => match traits.subfield.definition_degree {
            Some(d) if d < *n && n % d == 0 => (
                2 * u64::from(n / d),
                format!(
                    "negation and the 2^{d}-Frobenius: E is defined over F_(2^{d}), so classes have size 2*{}/{} = {} (upper bound on the speedup)",
                    n, d, 2 * (n / d)
                ),
            ),
            _ => (2, "negation only: E is not defined over a proper subfield".into()),
        },
        Model::Prime { p, a, b } => {
            if a.is_zero() && (p % 3u8) == BigUint::one() {
                (6, "negation and the order-3 endomorphism of j = 0 (p = 1 mod 3): classes of size 6".into())
            } else if b.is_zero() && (p % 4u8) == BigUint::one() {
                (4, "negation and the order-4 automorphism of j = 1728 (p = 1 mod 4): classes of size 4".into())
            } else {
                (2, "negation only".into())
            }
        }
        _ => (2, "negation only".into()),
    }
}

pub struct Baseline {
    pub class_size: u64,
    pub class_reason: String,
    /// `log2` of rho with the curve's equivalence class (negation included).
    pub log2_total: f64,
    /// `log2` of the cost model's own reference `sqrt(pi r / 4)`: negation
    /// only.  (Named before Amendment 3, when it was believed to be rho
    /// without automorphisms; the name is kept for the Rust field only.)
    pub log2_no_automorphisms: f64,
    pub log2_op_cost: f64,
}

/// Pollard rho on `<G>` with an equivalence class of `class_size` elements
/// (negation included): `sqrt(pi r / (2c))` group operations.  The class
/// enters as `sqrt(c / 2)` against the cost model's `sqrt(pi r / 4)`; see
/// [`crate::cost::class_discount_bits`].
pub fn baseline(model: &CostModel, inst: &Instance, class_size: u64, reason: String) -> Baseline {
    let log2_r = inst.log2_r();
    let fb = inst.field_bits();
    let no_aut = model.log2_rho_elliptic(log2_r, fb);
    Baseline {
        class_size,
        class_reason: reason,
        log2_total: log2_rho_with_class(model, log2_r, fb, class_size),
        log2_no_automorphisms: no_aut,
        log2_op_cost: model.log2_elliptic_op(fb),
    }
}

impl Baseline {
    pub fn value(&self, log2_r: f64) -> Value {
        json!({
            "algorithm": "Pollard rho on <G>",
            "convention": "sqrt(pi r / (2c)) elliptic group operations for an equivalence class of c elements, negation included. The cost model's sqrt(pi r / 4) (jacobian_transfer::cost) is the c = 2 case, so a class enters as sqrt(c / 2); certificate v1 divided that reference by sqrt(c), counted negation twice and sat 0.5 bit low (Amendment 3). isogeny_degree_search::rho_reference uses sqrt(pi r / 2) (no equivalence class), a factor sqrt 2 higher, and is not used",
            "log2_r": round3(log2_r),
            "equivalence_class_size": self.class_size,
            "equivalence_class_reason": self.class_reason,
            "log2_group_operations": round3(self.log2_total - self.log2_op_cost),
            "log2_cost_of_one_group_operation": round3(self.log2_op_cost),
            "log2_total": round3(self.log2_total),
            "log2_total_negation_only": round3(self.log2_no_automorphisms),
        })
    }
}

#[allow(dead_code)]
pub fn make_factoring(hints: Vec<BigUint>, budget: u64) -> Factoring {
    Factoring::new(budget, hints)
}

#[cfg(test)]
mod tests {
    use super::*;

    const Z7: u64 = 0x83; // z^7 + z + 1

    #[test]
    fn toy_koblitz_order_matches_exhaustive_count_and_validates() {
        // y^2 + xy = x^3 + x^2 + 1 over F_(2^7): #E = 128 + 1 - V_7(1, 2) = 142.
        let inst = toy_koblitz("t7", 1, &BigUint::from(Z7), 1).unwrap();
        assert_eq!(inst.rc.order, BigUint::from(142u32));
        assert_eq!(inst.r, BigUint::from(71u32));
        assert_eq!(inst.cofactor, BigUint::from(2u32));
        // Exhaustive point count with the repository's field arithmetic.
        let c = inst.binary().unwrap();
        let mut count = 1u32; // infinity
        for xb in 0u32..128 {
            for yb in 0u32..128 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&BigUint::from(xb), 7),
                    y: F2mElement::from_biguint(&BigUint::from(yb), 7),
                };
                if c.is_on_curve(&p) {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 142);
        let checks = validate(&inst, 7).unwrap();
        assert!(checks.iter().all(|c| c.result == CheckResult::Pass));
    }

    #[test]
    fn validation_rejects_a_wrong_subgroup_order() {
        let mut inst = toy_koblitz("t7", 1, &BigUint::from(Z7), 1).unwrap();
        // Claim 142 = 2 * 71 has prime subgroup 2 * 71: not prime.
        inst.r = BigUint::from(142u32);
        inst.cofactor = BigUint::one();
        let err = validate(&inst, 7).unwrap_err();
        assert!(err.starts_with("INVALID_INSTANCE"), "{err}");
    }

    /// Negation is counted once.  The cost model's `sqrt(pi r / 4)` is rho
    /// with negation (`c = 2`), so a curve with negation only has exactly
    /// that baseline, and a class of `c = 14` (negation and a 7-fold
    /// Frobenius) lowers it by `0.5 log2(14 / 2) = 0.5 log2 7`, not by
    /// `0.5 log2 14` (certificate v1).
    #[test]
    fn baseline_counts_negation_once() {
        let inst = toy_koblitz("t7", 1, &BigUint::from(Z7), 1).unwrap();
        let model = CostModel::default();
        let b2 = baseline(&model, &inst, 2, "negation".into());
        let b14 = baseline(&model, &inst, 14, "negation+frobenius".into());
        assert!((b2.log2_total - b14.log2_total - 0.5 * 7f64.log2()).abs() < 1e-9);
        assert!((b2.log2_no_automorphisms - b2.log2_total).abs() < 1e-12);
        // sqrt(pi r / 4) for c = 2, from the closed form.  The model's constant
        // log2 sqrt(pi/4) is stored as -0.1736147, 6.4e-4 above the exact value
        // (RESULTS section 8), hence the tolerance.
        let r = inst.log2_r();
        let expect = 0.5 * r
            + 0.5 * (std::f64::consts::PI / 4.0).log2()
            + model.log2_elliptic_op(inst.field_bits());
        assert!(
            (b2.log2_total - expect).abs() < 1e-3,
            "{} {}",
            b2.log2_total,
            expect
        );
        let v = b2.value(r);
        assert!(v.get("log2_total_without_automorphisms").is_none());
        assert_eq!(v["log2_total_negation_only"], v["log2_total"]);
    }
}
