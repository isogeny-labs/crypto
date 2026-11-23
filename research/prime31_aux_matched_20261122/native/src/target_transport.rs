//! The certified degree-three route on the fixed ordinary Prime31 target.
//! Native codomain membership uses V. Transported membership uses the complete
//! finite image r(V), with collisions and kernel poles recorded explicitly.
//! Only the dense presentation is asserted for this transported image.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::bases::{prime31_fixture, CurveDefinition, GroupWork, PointDefinition};
use crate::collector::{from_replay, to_replay, PublicTarget};
use crate::kernel_fibers::KernelFiber;
use crate::oracle::TowerDescription;
use crate::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

pub const P: u64 = 2_147_483_647;
pub const N: u64 = 2_147_446_831;
pub const ALPHA: u64 = 533_591_203;
pub const MAP_T: u64 = 963_718_081;
pub const MAP_U: u64 = 1_037_789_400;

pub fn codomain() -> PrimeCurve {
    PrimeCurve {
        p: P,
        a: 323_891_735,
        b: 1_985_248_359,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetArm {
    Original,
    IsogenousNative,
    IsogenousTransported,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TransferWork {
    pub point_images: u64,
    pub x_images: u64,
    pub field_products: u64,
    pub field_additions: u64,
    pub inversion_calls: u64,
    pub square_root_products: u64,
    pub square_root_calls: u64,
    pub lift_checks: u64,
    pub costs_partial: bool,
}

fn add(a: u64, b: u64, w: &mut TransferWork) -> u64 {
    w.field_additions += 1;
    (a + b) % P
}
fn sub(a: u64, b: u64, w: &mut TransferWork) -> u64 {
    w.field_additions += 1;
    (a + P - b) % P
}
fn mul(a: u64, b: u64, w: &mut TransferWork) -> u64 {
    w.field_products += 1;
    a * b % P
}
fn power(mut a: u64, mut k: u64, w: &mut TransferWork) -> u64 {
    let mut v = 1;
    while k != 0 {
        if k & 1 != 0 {
            v = mul(v, a, w);
        }
        k >>= 1;
        if k != 0 {
            a = mul(a, a, w);
        }
    }
    v
}

/// Return r(x) and r'(x). A kernel abscissa maps to infinity.
fn image_coordinates(x: u64, w: &mut TransferWork) -> Result<Option<(u64, u64)>, String> {
    if x >= P {
        return Err("noncanonical input to target isogeny".into());
    }
    if x == ALPHA {
        return Ok(None);
    }
    let d = sub(x, ALPHA, w);
    w.inversion_calls += 1;
    let inverse = power(d, P - 2, w);
    let i2 = mul(inverse, inverse, w);
    let i3 = mul(i2, inverse, w);
    let t1 = mul(MAP_T, inverse, w);
    let u2 = mul(MAP_U, i2, w);
    let xt = add(x, t1, w);
    let xx = add(xt, u2, w);
    let t2 = mul(MAP_T, i2, w);
    let u3 = mul((2 * MAP_U) % P, i3, w);
    let z = sub(1, t2, w);
    let derivative = sub(z, u3, w);
    Ok(Some((xx, derivative)))
}

pub fn image_x(x: u64, w: &mut TransferWork) -> Result<Option<u64>, String> {
    w.x_images += 1;
    Ok(image_coordinates(x, w)?.map(|v| v.0))
}

pub fn image_point(q: PrimePoint, w: &mut TransferWork) -> Result<PrimePoint, String> {
    w.point_images += 1;
    if q.infinity {
        return Ok(codomain().identity());
    }
    let (e, _, _) = prime31_fixture();
    if q.x >= P || q.y >= P || !e.is_on_curve(q) {
        return Err("point is not on the source of the target isogeny".into());
    }
    let (x, derivative) = image_coordinates(q.x, w)?
        .ok_or("nonidentity rational source point cannot lie in this degree-three kernel")?;
    let y = mul(q.y, derivative, w);
    let out = PrimePoint::affine(x, y);
    if y == 0 || !codomain().is_on_curve(out) {
        return Err("target isogeny image fails codomain membership".into());
    }
    Ok(out)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnTransport {
    pub source_column: usize,
    pub destination_column: usize,
    /// psi(source canonical point) = sign * destination canonical point.
    pub sign: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedBase {
    pub arm: TargetArm,
    pub public_target: PublicTarget,
    pub curve: CurveDefinition,
    pub v: Vec<u64>,
    pub canonical_points: Vec<PointDefinition>,
    pub tower: Option<TowerDescription>,
    pub source_membership_size: usize,
    pub source_columns: Option<usize>,
    pub dropped_kernel_poles: Vec<u64>,
    pub duplicate_image_values: usize,
    pub column_transport: Vec<ColumnTransport>,
    pub group_work: GroupWork,
    pub transfer_work: TransferWork,
}

impl PreparedBase {
    pub fn factor_base(&self) -> Result<FactorBase<PrimePoint>, String> {
        let curve = self.curve.production();
        let order = self.public_target.order;
        let mut points = Vec::with_capacity(2 * self.canonical_points.len());
        let mut col_of = Vec::with_capacity(points.capacity());
        let mut coef_of = Vec::with_capacity(points.capacity());
        for (column, &q) in self.canonical_points.iter().enumerate() {
            let q = q.production();
            if q.infinity
                || q.y == 0
                || q.y >= P - q.y
                || !curve.is_on_curve(q)
                || self.v.binary_search(&q.x).is_err()
            {
                return Err("invalid canonical point in prepared target base".into());
            }
            points.extend([q, curve.neg(q)]);
            col_of.extend([column, column]);
            coef_of.extend([1, order - 1]);
        }
        FactorBase::from_column_map(
            format!(
                "Prime31 {:?}; {} complete membership values",
                self.arm,
                self.v.len()
            ),
            points,
            col_of,
            coef_of,
            self.canonical_points.len(),
            |q| curve.key(q),
            |q| curve.key(&curve.neg(*q)),
            |q| q.x,
        )
    }
}

fn lift_values(
    curve: &PrimeCurve,
    values: &[u64],
    ops: &mut GroupOps,
    work: &mut TransferWork,
) -> Result<Vec<PointDefinition>, String> {
    let mut out = Vec::new();
    for &x in values {
        work.lift_checks += 1;
        let x2 = mul(x, x, work);
        let x3 = mul(x2, x, work);
        let ax = mul(curve.a, x, work);
        let rhs0 = add(x3, ax, work);
        let rhs = add(rhs0, curve.b, work);
        work.square_root_calls += 1;
        if let Some(y) = curve.sqrt_counted(rhs, &mut work.square_root_products) {
            if y == 0 {
                return Err("unexpected rational two-torsion on odd-order target".into());
            }
            let q = PrimePoint::affine(x, y.min(P - y));
            if !curve.is_on_curve(q) || !curve.mul(ops, q, N).infinity {
                return Err("factor-base point failed subgroup check".into());
            }
            out.push(q.into());
        }
    }
    if out.is_empty() {
        return Err("target factor base is empty".into());
    }
    Ok(out)
}

pub fn prepare(
    fiber: &KernelFiber,
    source: &PublicTarget,
    arm: TargetArm,
) -> Result<PreparedBase, String> {
    let (e, n, g) = prime31_fixture();
    if (
        source.p,
        source.a,
        source.b,
        source.order,
        from_replay(source.generator),
    ) != (e.p, e.a, e.b, n, g)
    {
        return Err("isogeny comparison requires the pinned ordinary Prime31 source".into());
    }
    if fiber.v.len() != 1024 || fiber.v.windows(2).any(|w| w[0] >= w[1]) {
        return Err("source fiber must have 1024 sorted distinct values".into());
    }
    let mut ops = GroupOps::default();
    source.validate(&mut ops)?;
    let mut work = TransferWork {
        costs_partial: true,
        ..Default::default()
    };
    let mut public_target = source.clone();
    let curve = if arm == TargetArm::Original {
        e
    } else {
        let curve = codomain();
        public_target.a = curve.a;
        public_target.b = curve.b;
        public_target.generator = to_replay(image_point(g, &mut work)?);
        public_target.target = to_replay(image_point(from_replay(source.target), &mut work)?);
        public_target.validate(&mut ops)?;
        curve
    };
    let mut v = fiber.v.clone();
    let mut dropped_kernel_poles = Vec::new();
    let mut duplicate_image_values = 0;
    let mut column_transport = Vec::new();
    let mut source_columns = None;
    let mut source_points = Vec::new();
    let tower = if arm == TargetArm::IsogenousTransported {
        source_points = lift_values(&prime31_fixture().0, &fiber.v, &mut ops, &mut work)?;
        source_columns = Some(source_points.len());
        let mut images = BTreeSet::new();
        for &x in &fiber.v {
            match image_x(x, &mut work)? {
                None => dropped_kernel_poles.push(x),
                Some(y) => {
                    if !images.insert(y) {
                        duplicate_image_values += 1;
                    }
                }
            }
        }
        v = images.into_iter().collect();
        None
    } else {
        Some(fiber.tower.clone())
    };
    let canonical_points = lift_values(&curve, &v, &mut ops, &mut work)?;
    if arm == TargetArm::IsogenousTransported {
        let by_x: BTreeMap<_, _> = canonical_points
            .iter()
            .enumerate()
            .map(|(column, q)| (q.x, (column, q.production())))
            .collect();
        let mut used = BTreeSet::new();
        for (column, q) in source_points.iter().enumerate() {
            let image = image_point(q.production(), &mut work)?;
            let &(destination_column, canonical) = by_x
                .get(&image.x)
                .ok_or("transported rational point missing from codomain factor base")?;
            let sign = if image == canonical {
                1
            } else if image == curve.neg(canonical) {
                -1
            } else {
                return Err("transported point sign is inconsistent".into());
            };
            if !used.insert(destination_column) {
                return Err("target isogeny is not injective on factor base".into());
            }
            column_transport.push(ColumnTransport {
                source_column: column,
                destination_column,
                sign,
            });
        }
        if used.len() != canonical_points.len() || used.len() != source_points.len() {
            return Err(
                "nonlifting source x unexpectedly produced a rational codomain lift".into(),
            );
        }
    }
    Ok(PreparedBase {
        arm,
        public_target,
        curve: CurveDefinition::from(&curve),
        v,
        canonical_points,
        tower,
        source_membership_size: fiber.v.len(),
        source_columns,
        dropped_kernel_poles,
        duplicate_image_values,
        column_transport,
        group_work: GroupWork::from_ops(&ops),
        transfer_work: work,
    })
}

pub fn transport_row(row: &[i64], columns: &[ColumnTransport]) -> Result<Vec<i64>, String> {
    if row.len() != columns.len() {
        return Err("transport row has the wrong column count".into());
    }
    let mut output = vec![0; columns.len()];
    let mut source_seen = BTreeSet::new();
    let mut destination_seen = BTreeSet::new();
    for c in columns {
        if c.source_column >= row.len()
            || c.destination_column >= row.len()
            || !matches!(c.sign, -1 | 1)
            || !source_seen.insert(c.source_column)
            || !destination_seen.insert(c.destination_column)
        {
            return Err("column transport must be a signed permutation".into());
        }
        output[c.destination_column] = row[c.source_column]
            .checked_mul(c.sign)
            .ok_or("signed row overflow")?;
    }
    Ok(output)
}
