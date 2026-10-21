//! Import content-addressed factory archives without changing their point set.
//! Only negation is folded: no Frobenius coefficient map is inferred.
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
        ArtinSchreier, BinaryGroup, BinaryInstance, CountedGroup, FactorBase, GroupOps, PrimeCurve,
        PrimeInstance, PrimePoint,
    },
    koblitz_fast::{FastCurve, FastPoint},
    semaev_decomp::Gf2,
};

pub enum ArchiveInstance {
    Prime(PrimeInstance),
    Binary(Box<BinaryInstance>),
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
            // The generic binary path preserves the imported representation.
            // No catalog-specific Frobenius certificate is manufactured.
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
                koblitz: None,
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
fn import<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    expected: CurveSpec,
    params: &Params,
    ops: &mut GroupOps,
    convert: impl Fn(u64, u64) -> G::Elt,
    xy: impl Fn(&G::Elt) -> u64,
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
        let key = ctx.group.key(&point);
        let negative = ctx.group.key(&ctx.group.neg(point));
        let next = columns.len();
        let col = *columns.entry(key.min(negative)).or_insert(next);
        col_of.push(col);
        coef_of.push(if key <= negative { 1 } else { ctx.r - 1 });
        points.push(point);
    }
    let mut fb = FactorBase::from_column_map(
        format!("archive {id}; exact signed points, negation columns"),
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
        if (ctx.group.p, ctx.group.a, ctx.group.b) != (c.p, c.a, c.b) {
            return Err("native prime curve mismatch".into());
        }
        import(
            ctx,
            prime_spec(self.instance),
            p,
            ops,
            PrimePoint::affine,
            |p| p.x,
        )
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
        let c = &self.instance.fast;
        let g = ctx.group.0;
        if (g.n, g.field.irr, g.a, g.b) != (c.n, c.field.irr, c.a, c.b) {
            return Err("native binary curve mismatch".into());
        }
        import(
            ctx,
            binary_spec(self.instance),
            p,
            ops,
            FastPoint::affine,
            |p| p.x,
        )
    }
}

#[cfg(test)]
#[path = "archive_tests.rs"]
mod tests;
