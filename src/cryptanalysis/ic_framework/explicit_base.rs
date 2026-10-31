//! Exact, negation-closed factor bases from user-supplied x coordinates.
//! These are raw curve points, with the same cofactor convention as the
//! existing abscissa bases. They do not assert a subspace or signed hash law.

use std::collections::BTreeSet;
use std::time::Instant;

use super::stages::{FactorBaseBuilder, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    BinaryGroup, BinaryInstance, CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint,
};
use crate::cryptanalysis::koblitz_fast::FastPoint;

fn coordinates(params: &Params, bound: u64) -> Result<Vec<u64>, String> {
    if params.0.keys().any(|k| k != "xs") {
        return Err("explicit-x accepts only xs=<decimal-or-0x-hex>;...".into());
    }
    let raw = params.get("xs").ok_or("explicit-x requires xs")?;
    let mut seen = BTreeSet::new();
    raw.split(';')
        .map(|token| {
            let token = token.trim();
            let x = if let Some(hex) = token.strip_prefix("0x") {
                u64::from_str_radix(hex, 16)
            } else {
                token.parse::<u64>()
            }
            .map_err(|_| format!("invalid explicit-x coordinate `{token}`"))?;
            if x >= bound {
                return Err(format!("explicit-x coordinate {x} is outside the field"));
            }
            if !seen.insert(x) {
                return Err(format!("duplicate explicit-x coordinate {x}"));
            }
            Ok(x)
        })
        .collect()
}

/// Lift an exact x list on a prime-field curve, preserving its column order.
pub struct PrimeExplicitXBase;

impl FactorBaseBuilder<PrimeCurve> for PrimeExplicitXBase {
    fn name(&self) -> &str {
        "explicit-x"
    }
    fn describe(&self, _: &Params) -> String {
        "explicit x coordinates; both signs, one column per supplied x".into()
    }
    fn parameters(&self) -> &[(&str, &str)] {
        &[(
            "xs",
            "nonempty semicolon-separated decimal or 0x-hex coordinates",
        )]
    }
    fn build(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        params: &Params,
        _: &mut GroupOps,
    ) -> Result<FactorBase<PrimePoint>, String> {
        let start = Instant::now();
        let g = ctx.group;
        let xs = coordinates(params, g.p)?;
        let (mut points, mut cols, mut coeffs) = (Vec::new(), Vec::new(), Vec::new());
        for (col, &x) in xs.iter().enumerate() {
            let y = g
                .sqrt(g.rhs(x))
                .ok_or_else(|| format!("explicit-x coordinate {x} has no curve point"))?;
            let p = PrimePoint::affine(x, y.min((g.p - y) % g.p));
            let q = g.neg(p);
            points.push(p);
            cols.push(col);
            coeffs.push(1);
            if q != p {
                points.push(q);
                cols.push(col);
                coeffs.push(ctx.r - 1);
            }
        }
        let mut fb = FactorBase::from_column_map(
            self.describe(params),
            points,
            cols,
            coeffs,
            xs.len(),
            |p| g.key(p),
            |p| g.key(&g.neg(*p)),
            |p| p.x,
        )?;
        fb.cost.count("abscissae_scanned", xs.len() as u64);
        fb.cost.count("sqrt_solves", xs.len() as u64);
        // Building the signed inventory also negated one point per x; the
        // constructor separately charged negations used to build its index.
        fb.cost.work("index_point_negations", xs.len() as u64);
        fb.cost
            .count("explicit_x_sign_order_uncharged", xs.len() as u64);
        fb.cost.count("explicit_x_parse_uncharged", xs.len() as u64);
        fb.cost.wall_ns = start.elapsed().as_nanos() as u64;
        Ok(fb)
    }
}

/// Lift an exact x list in the binary instance's polynomial representation.
pub struct BinaryExplicitXBase<'a> {
    /// The curve and its existing Artin-Schreier solver.
    pub instance: &'a BinaryInstance,
}

impl<'a> FactorBaseBuilder<BinaryGroup<'a>> for BinaryExplicitXBase<'a> {
    fn name(&self) -> &str {
        "explicit-x"
    }
    fn describe(&self, _: &Params) -> String {
        "explicit x coordinates; both signs, one column per supplied x".into()
    }
    fn parameters(&self) -> &[(&str, &str)] {
        &[(
            "xs",
            "nonempty semicolon-separated decimal or 0x-hex polynomial coordinates",
        )]
    }
    fn build(
        &self,
        ctx: &InstanceCtx<BinaryGroup<'a>>,
        params: &Params,
        _: &mut GroupOps,
    ) -> Result<FactorBase<FastPoint>, String> {
        let start = Instant::now();
        let inst = self.instance;
        let xs = coordinates(
            params,
            1u64.checked_shl(inst.n)
                .ok_or("explicit-x requires n < 64")?,
        )?;
        let (mut points, mut cols, mut coeffs) = (Vec::new(), Vec::new(), Vec::new());
        for (col, &x) in xs.iter().enumerate() {
            let mut lifted = inst.points_with_x(x);
            if lifted.is_empty() {
                return Err(format!("explicit-x coordinate {x} has no curve point"));
            }
            lifted.sort_unstable_by_key(|p| p.y);
            for (sign, p) in lifted.into_iter().enumerate() {
                points.push(p);
                cols.push(col);
                coeffs.push(if sign == 0 { 1 } else { ctx.r - 1 });
            }
        }
        let g = ctx.group;
        let mut fb = FactorBase::from_column_map(
            self.describe(params),
            points,
            cols,
            coeffs,
            xs.len(),
            |p| g.key(p),
            |p| g.key(&g.neg(*p)),
            |p| p.x,
        )?;
        fb.cost.count("abscissae_scanned", xs.len() as u64);
        fb.cost
            .count("as_solves", xs.iter().filter(|&&x| x != 0).count() as u64);
        fb.cost.work(
            "index_point_negations",
            xs.iter().filter(|&&x| x != 0).count() as u64,
        );
        fb.cost.count(
            "explicit_x_sign_order_uncharged",
            xs.iter().filter(|&&x| x != 0).count() as u64,
        );
        if xs.contains(&0) {
            fb.cost.count(
                "explicit_x_zero_lift_squarings_uncharged",
                u64::from(inst.n - 1),
            );
        }
        fb.cost.count("explicit_x_parse_uncharged", xs.len() as u64);
        fb.cost.wall_ns = start.elapsed().as_nanos() as u64;
        Ok(fb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn params(xs: &str) -> Params {
        Params([("xs".into(), xs.into())].into())
    }
    #[test]
    fn exact_order_and_canonical_bounds() {
        assert_eq!(coordinates(&params("0x20;1;0"), 128).unwrap(), [32, 1, 0]);
        for bad in [
            "",
            "1;",
            "1;;2",
            "-1",
            "1;0x1",
            "128",
            "0x",
            "18446744073709551616",
        ] {
            assert!(coordinates(&params(bad), 128).is_err(), "{bad}");
        }
        let mut p = params("1");
        p.0.insert("size".into(), "1".into());
        assert!(coordinates(&p, 128).is_err());
    }

    #[test]
    fn prime_lifts_keep_zero_and_self_negative_points_but_reject_missing_roots() {
        let g = PrimeCurve { p: 11, a: 0, b: 1 };
        let ctx = InstanceCtx {
            group: &g,
            generator: PrimePoint::affine(0, 1),
            target: PrimePoint::affine(0, 1),
            r: 3,
            cofactor: 4,
            group_order: 12,
            name: "explicit-x-fixture".into(),
            field_degree: None,
        };
        let fb = PrimeExplicitXBase
            .build(&ctx, &params("10;0"), &mut GroupOps::default())
            .unwrap();
        assert_eq!(fb.columns, 2);
        assert_eq!(fb.points.len(), 3);
        assert_eq!(fb.points[0], PrimePoint::affine(10, 0));
        assert_eq!(fb.neg_index[0], 0);
        assert_eq!(fb.col_of, [0, 1, 1]);
        assert_eq!(fb.coef_of, [1, 1, 2]);
        assert!(fb.subspace_basis.is_none());
        assert!(PrimeExplicitXBase
            .build(&ctx, &params("1"), &mut GroupOps::default())
            .is_err());
    }
}
