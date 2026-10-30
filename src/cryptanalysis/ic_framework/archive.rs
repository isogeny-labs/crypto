//! Import content-addressed factory archives without changing their point set.
//! Symmetry is explicit and every canonical-point multiplier is replayed.
use std::{collections::BTreeMap, path::Path, time::Instant};

use factor_base::{
    arithmetic::{parse, Curve, CurveSpec, FieldSpec, PointSpec},
    store,
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

use super::stages::{FactorBaseBuilder, InstanceCtx, Params};
use crate::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crate::cryptanalysis::{
    ic_boundary::{
        invmod, ArtinSchreier, BinaryGroup, BinaryInstance, CountedGroup, CubicClasses, FactorBase,
        GroupOps, NegationClasses, PointClasses, PrimeCurve, PrimeInstance, PrimePoint,
        QuarticClasses, RhoClasses, SignedFrobeniusClasses,
    },
    koblitz_fast::{FastCurve, FastPoint},
    koblitz_index_calculus::{frobenius_eigenvalue, koblitz_point_count, KoblitzCurve},
    semaev_decomp::Gf2,
};

pub enum ArchiveInstance {
    Prime(PrimeInstance),
    Binary(Box<BinaryInstance>),
}
/// Column symmetry is independent of the archive recipe and decomposition oracle.
/// New native families implement `CountedGroup`/`FactorBaseBuilder`; new folds
/// implement `RhoClasses` and reuse `import_with_symmetry` and the rank auditor.
pub const SYMMETRIES: &[&str] = &["none", "negation", "frobenius", "cubic", "quartic"];

fn symmetry(params: &Params) -> Result<&str, String> {
    for key in params.0.keys() {
        if !matches!(key.as_str(), "store" | "manifest" | "symmetry") {
            return Err(format!("unsupported archive parameter `{key}`"));
        }
    }
    let name = params.get("symmetry").unwrap_or("negation");
    if !SYMMETRIES.contains(&name) {
        return Err(format!(
            "unsupported archive symmetry `{name}`; choose {}",
            SYMMETRIES.join(", ")
        ));
    }
    Ok(name)
}

/// Certify the actual imported representation, subgroup and generator. Never
/// substitute a catalog curve just because its degree or order agrees.
fn koblitz_certificate(curve: &BinaryCurve, a: u64, b: u64, order: u64) -> Option<KoblitzCurve> {
    if a > 1 || b != 1 || koblitz_point_count(a as u8, curve.m) != BigUint::from(order) {
        return None;
    }
    let trace = if a == 0 { -1 } else { 1 };
    let lambda = frobenius_eigenvalue(curve, trace, &curve.order)?;
    // Native Koblitz table/rho plugins use a signed action of order 2n.
    // A subgroup defined over a proper subfield, or a Frobenius power equal
    // to negation, needs a different action-size certificate and stays on
    // the generic binary path rather than inheriting a false 2n factor.
    let r = curve.order.to_u64()?;
    let scalar = lambda.to_u64()?;
    let mut power = 1u64;
    for k in 1..=curve.m {
        power = (power as u128 * scalar as u128 % r as u128) as u64;
        if power == r - 1 || (power == 1 && k != curve.m) {
            return None;
        }
    }
    if power != 1 {
        return None;
    }
    Some(KoblitzCurve {
        a: a as u8,
        n: curve.m,
        curve: curve.clone(),
        trace,
        group_order: BigUint::from(order),
        subgroup_order: curve.order.clone(),
        cofactor: curve.cofactor.clone(),
        lambda,
        frobenius_is_endomorphism: true,
        k: 1,
        q: 2,
        a_index: a,
        b_index: 1,
        subfield_basis: vec![F2mElement::one(curve.m)],
    })
}
fn word(s: &str) -> Result<u64, String> {
    parse(s)
        .map_err(|e| e.to_string())?
        .to_u64()
        .ok_or_else(|| "unsupported_archive_width: integer exceeds u64".into())
}
/// Read/hash the manifest and validate its curve before constructing a native instance.
/// Shards and recipe are verified again inside the measured builder phase.
pub fn instance(root: &Path, id: &str) -> Result<ArchiveInstance, String> {
    let p = store::manifest_path(root, id).map_err(|e| e.to_string())?;
    let data = store::read_limited(&p, 32 * 1024 * 1024).map_err(|e| e.to_string())?;
    if store::digest(&data) != id {
        return Err("manifest hash mismatch".into());
    }
    let m: store::Manifest = serde_json::from_slice(&data).map_err(|e| e.to_string())?;
    if m.schema != "factor-base-manifest/v1" {
        return Err("unsupported archive schema".into());
    }
    let s = &m.config.curve;
    // Width checks precede expensive curve validation; never truncate.
    let r = word(&s.subgroup_order)?;
    let h = word(&s.cofactor)?;
    let order = word(&s.group_order)?;
    if r <= 2 {
        return Err("archive IC requires odd prime subgroup order".into());
    }
    let (a, b, x, y) = (
        word(&s.a)?,
        word(&s.b)?,
        word(&s.generator.x)?,
        word(&s.generator.y)?,
    );
    let name = format!("archive-{id}");
    match &s.field {
        FieldSpec::Prime { modulus } => {
            let p = word(modulus)?;
            // PrimePoint::key reserves a sign bit and the identity sentinel.
            if p >= 1u64 << 62 {
                return Err("unsupported_archive_width: prime field must be below 2^62".into());
            }
            Curve::new(s.clone()).map_err(|e| e.to_string())?;
            let mut i = PrimeInstance {
                name,
                curve: PrimeCurve { p, a, b },
                r,
                group_order: order,
                cofactor: h,
                generator: (x, y),
            };
            i.name = i.curve_id().slug;
            Ok(ArchiveInstance::Prime(i))
        }
        FieldSpec::Binary { degree, modulus } => {
            if *degree > FastCurve::MAX_DEGREE as usize {
                return Err("unsupported_archive_width: binary degree exceeds 62".into());
            }
            Curve::new(s.clone()).map_err(|e| e.to_string())?;
            let n = *degree as u32;
            let modulus = word(modulus)?;
            let irr = IrreduciblePoly {
                degree: n,
                low_terms: (0..n).filter(|i| modulus & (1u64 << i) != 0).collect(),
            };
            let element = |v| F2mElement::from_biguint(&BigUint::from(v), n);
            let curve = BinaryCurve {
                m: n,
                irreducible: irr.clone(),
                a: element(a),
                b: element(b),
                generator: BinaryPoint::Affine {
                    x: element(x),
                    y: element(y),
                },
                order: BigUint::from(r),
                cofactor: BigUint::from(h),
            };
            let fast = FastCurve::new(&curve).ok_or("unsupported binary arithmetic")?;
            let gf = Gf2::new(&irr);
            let artin_schreier = ArtinSchreier::new(&gf);
            let koblitz = koblitz_certificate(&curve, a, b, order);
            let mut i = BinaryInstance {
                name,
                n,
                irreducible: irr,
                fast,
                gf,
                artin_schreier,
                a,
                b,
                r,
                group_order: order,
                cofactor: h,
                generator: FastPoint::affine(x, y),
                koblitz,
            };
            i.name = i.curve_id().slug;
            Ok(ArchiveInstance::Binary(Box::new(i)))
        }
    }
}
fn h(x: u64) -> String {
    format!("0x{x:x}")
}
pub fn prime_spec(i: &PrimeInstance) -> CurveSpec {
    CurveSpec {
        field: FieldSpec::Prime {
            modulus: h(i.curve.p),
        },
        a: h(i.curve.a),
        b: h(i.curve.b),
        group_order: h(i.group_order),
        subgroup_order: h(i.r),
        cofactor: h(i.cofactor),
        generator: PointSpec {
            x: h(i.generator.0),
            y: h(i.generator.1),
        },
    }
}
pub fn binary_spec(i: &BinaryInstance) -> CurveSpec {
    let modulus = i
        .irreducible
        .low_terms
        .iter()
        .fold(1u64 << i.n, |m, b| m | (1u64 << b));
    CurveSpec {
        field: FieldSpec::Binary {
            degree: i.n as usize,
            modulus: h(modulus),
        },
        a: h(i.a),
        b: h(i.b),
        group_order: h(i.group_order),
        subgroup_order: h(i.r),
        cofactor: h(i.cofactor),
        generator: PointSpec {
            x: h(i.generator.x),
            y: h(i.generator.y),
        },
    }
}
/// Reuse the archive verifier with another native group or certified symmetry.
/// The backend must bind `expected` to its arithmetic context and decode points
/// in that exact representation. Every returned symmetry coefficient is checked
/// on every imported point; point order and membership are never expanded.
/// This native bridge uses u64 coordinates/scalars; wider backends also require
/// widening InstanceCtx, RelationSolver and the versioned rank receipt.
pub fn import_with_symmetry<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    expected: CurveSpec,
    params: &Params,
    ops: &mut GroupOps,
    convert: impl Fn(u64, u64) -> G::Elt,
    xy: impl Fn(&G::Elt) -> u64,
    classes: &dyn RhoClasses<G>,
) -> Result<FactorBase<G::Elt>, String> {
    let start = Instant::now();
    let root = Path::new(params.get("store").ok_or("archive requires store")?);
    let id = params.get("manifest").ok_or("archive requires manifest")?;
    let header = store::read_limited(
        &store::manifest_path(root, id).map_err(|e| e.to_string())?,
        32 * 1024 * 1024,
    )
    .map_err(|e| e.to_string())?;
    if store::digest(&header) != id {
        return Err("manifest hash mismatch".into());
    }
    let manifest: store::Manifest = serde_json::from_slice(&header).map_err(|e| e.to_string())?;
    if manifest.point_count > 8192 {
        return Err(
            "archive_ic_capacity: at most 8192 signed points supported by rank audit".into(),
        );
    }
    let (m, g) = store::verify(root, id).map_err(|e| format!("archive verification: {e:#}"))?;
    if g.curve.spec != expected
        || ctx.r != word(&expected.subgroup_order)?
        || ctx.cofactor != word(&expected.cofactor)?
        || ctx.group_order != word(&expected.group_order)?
    {
        return Err("archive curve/subgroup/generator identity mismatch".into());
    }
    let generator = convert(word(&expected.generator.x)?, word(&expected.generator.y)?);
    if ctx.generator != generator {
        return Err("archive generator mismatch".into());
    }
    let verification_ns = start.elapsed().as_nanos() as u64;
    let index_start = Instant::now();
    let mut points = Vec::with_capacity(g.points.len());
    let mut columns = BTreeMap::new();
    let mut col_of = Vec::new();
    let mut coef_of = Vec::new();
    for p in &g.points {
        let point = convert(
            p.x.to_u64().ok_or("point width")?,
            p.y.to_u64().ok_or("point width")?,
        );
        if !ctx.group.is_identity(&ctx.group.mul(ops, point, ctx.r)) {
            return Err("native subgroup replay failed".into());
        }
        let (representative, multiplier) = classes.canon(ctx.group, ops, point);
        let coefficient = invmod(multiplier % ctx.r, ctx.r)
            .ok_or("archive symmetry has a noninvertible multiplier")?;
        if ctx.group.mul(ops, point, multiplier % ctx.r) != representative {
            return Err("archive symmetry coefficient replay failed".into());
        }
        let key = ctx.group.key(&representative);
        let next = columns.len();
        let (col, prior) = *columns.entry(key).or_insert((next, representative));
        if prior != representative {
            return Err("archive symmetry representative key collision".into());
        }
        col_of.push(col);
        coef_of.push(coefficient);
        points.push(point);
    }
    let mut fb = FactorBase::from_column_map(
        format!("archive {id}; exact signed points and verified column symmetry"),
        points,
        col_of,
        coef_of,
        columns.len(),
        |p| ctx.group.key(p),
        |p| ctx.group.key(&ctx.group.neg(*p)),
        xy,
    )?;
    // A filtered set is not necessarily a full subspace. Retain a basis only
    // for the unshifted single-space recipe; algebraic consumers still lift/filter.
    if factor_base::recipe::offsets(&g.recipe).is_some_and(|o| o.len() == 1 && word(&o[0]) == Ok(0))
    {
        fb.subspace_basis = g
            .basis
            .as_ref()
            .map(|v| {
                v.iter()
                    .map(|x| x.to_u64().ok_or("basis width".to_string()))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        fb.dimension = fb.subspace_basis.as_ref().map(|v| v.len() as u32);
    }
    fb.cost.count(
        "archive_bytes_read",
        m.objects.iter().map(|o| o.bytes as u64).sum::<u64>() + 2 * header.len() as u64,
    );
    fb.cost.count("archive_replay_ns_unpriced", verification_ns);
    fb.cost.count(
        "archive_index_ns_unpriced",
        index_start.elapsed().as_nanos() as u64,
    );
    fb.cost
        .count("archive_original_generation_and_transfer_unpriced", 1);
    fb.cost.count(
        "archive_symmetry_canonicalisations_unpriced",
        fb.points.len() as u64,
    );
    fb.cost.count("archive_instance_preflight_unpriced", 1);
    fb.cost.wall_ns = start.elapsed().as_nanos() as u64;
    Ok(fb)
}
pub struct PrimeArchiveBase<'a> {
    pub instance: &'a PrimeInstance,
}
impl FactorBaseBuilder<PrimeCurve> for PrimeArchiveBase<'_> {
    fn name(&self) -> &str {
        "archive"
    }
    fn describe(&self, p: &Params) -> String {
        format!(
            "verified factory archive {}",
            p.get("manifest").unwrap_or("?")
        )
    }
    fn build(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        p: &Params,
        ops: &mut GroupOps,
    ) -> Result<FactorBase<PrimePoint>, String> {
        let c = &self.instance.curve;
        if ctx.r != self.instance.r
            || ctx.r <= 2
            || !crate::cryptanalysis::challenge_ladder::is_prime_u64(ctx.r)
        {
            return Err("archive curve/subgroup identity mismatch".into());
        }
        if (ctx.group.p, ctx.group.a, ctx.group.b) != (c.p, c.a, c.b) {
            return Err("native prime curve mismatch".into());
        }
        let setup = Instant::now();
        let classes: Box<dyn RhoClasses<PrimeCurve>> = match symmetry(p)? {
            "none" => Box::new(PointClasses),
            "negation" => Box::new(NegationClasses { r: ctx.r }),
            "cubic" => Box::new(
                CubicClasses::new(c, ctx.generator, ctx.r, ops)
                    .ok_or("cubic symmetry requires a verified j=0 subgroup action")?,
            ),
            "quartic" => Box::new(
                QuarticClasses::new(c, ctx.generator, ctx.r, ops)
                    .ok_or("quartic symmetry requires a verified j=1728 subgroup action")?,
            ),
            other => {
                return Err(format!(
                    "archive symmetry `{other}` is unavailable on a prime-field curve"
                ))
            }
        };
        let setup_ns = setup.elapsed().as_nanos() as u64;
        let mut fb = import_with_symmetry(
            ctx,
            prime_spec(self.instance),
            p,
            ops,
            PrimePoint::affine,
            |p| p.x,
            classes.as_ref(),
        )?;
        fb.cost
            .count("archive_symmetry_preparation_ns_unpriced", setup_ns);
        Ok(fb)
    }
}
pub struct BinaryArchiveBase<'a> {
    pub instance: &'a BinaryInstance,
}
impl<'a> FactorBaseBuilder<BinaryGroup<'a>> for BinaryArchiveBase<'_> {
    fn name(&self) -> &str {
        "archive"
    }
    fn describe(&self, p: &Params) -> String {
        format!(
            "verified factory archive {}",
            p.get("manifest").unwrap_or("?")
        )
    }
    fn build(
        &self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        p: &Params,
        ops: &mut GroupOps,
    ) -> Result<FactorBase<FastPoint>, String> {
        let i = self.instance;
        if ctx.r != i.r
            || ctx.r <= 2
            || !crate::cryptanalysis::challenge_ladder::is_prime_u64(ctx.r)
        {
            return Err("archive curve/subgroup identity mismatch".into());
        }
        if i.n < 2
            || i.n > FastCurve::MAX_DEGREE
            || i.irreducible.degree != i.n
            || i.irreducible.low_terms.iter().any(|&bit| bit >= i.n)
        {
            return Err("unsupported native binary field representation".into());
        }
        let modulus = i
            .irreducible
            .low_terms
            .iter()
            .fold(1u64 << i.n, |m, bit| m | (1u64 << bit));
        if i.gf.n != i.n || i.gf.irr != modulus || i.artin_schreier.degree() != i.n {
            return Err("native binary field cache mismatch".into());
        }
        for g in [&i.fast, ctx.group.0] {
            if (g.n, g.field.n, g.field.irr, g.a, g.b) != (i.n, i.n, modulus, i.a, i.b) {
                return Err("native binary curve mismatch".into());
            }
        }
        let setup = Instant::now();
        let selected = symmetry(p)?;
        let classes: Box<dyn RhoClasses<BinaryGroup<'a>>> = match selected {
            "none" => Box::new(PointClasses),
            "negation" => Box::new(NegationClasses { r: ctx.r }),
            "frobenius" => {
                if i.a > 1
                    || i.b != 1
                    || !i
                        .koblitz
                        .as_ref()
                        .is_some_and(|k| k.k == 1 && k.frobenius_is_endomorphism)
                {
                    return Err("frobenius symmetry requires a certified Koblitz instance".into());
                }
                Box::new(SignedFrobeniusClasses::new(i).ok_or(
                    "frobenius symmetry is unavailable for this native field representation",
                )?)
            }
            other => {
                return Err(format!(
                    "archive symmetry `{other}` is unavailable on a binary curve"
                ))
            }
        };
        let setup_ns = setup.elapsed().as_nanos() as u64;
        let mut fb = import_with_symmetry(
            ctx,
            binary_spec(self.instance),
            p,
            ops,
            FastPoint::affine,
            |p| p.x,
            classes.as_ref(),
        )?;
        if selected == "frobenius" {
            for &point in &fb.points {
                if fb
                    .index_of_key(i.fast.frobenius_k(point, 1).pack())
                    .is_none()
                    || fb
                        .index_of_key(ctx.group.key(&ctx.group.neg(point)))
                        .is_none()
                {
                    return Err("frobenius symmetry requires an archive closed under signed Frobenius; rebuild with a closed recipe".into());
                }
            }
            fb.cost.count(
                "archive_frobenius_closure_maps_unpriced",
                fb.points.len() as u64,
            );
        }
        fb.cost
            .count("archive_symmetry_preparation_ns_unpriced", setup_ns);
        Ok(fb)
    }
}

#[cfg(test)]
#[path = "archive_tests.rs"]
mod tests;
