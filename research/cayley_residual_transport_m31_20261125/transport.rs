//! Exact transport of trace-zero chord-function labels on one residual pencil.
//! Supplied-prefix/label qualification component; no prefix or label finder.
use crate::original_chord::{add, inv, mul, roots, sub, Diagnostics, P};
use crate::upstream::{Arithmetic, Curve, Point};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Label {
    Identity,
    Finite(u64),
}

impl Label {
    pub fn coordinates(self) -> Result<(u64, u64), String> {
        match self {
            Self::Identity => Ok((1, 0)),
            Self::Finite(x) if x < P => Ok((x, 1)),
            _ => Err("noncanonical label parameter".into()),
        }
    }
    pub fn from_coordinates(x: u64, z: u64) -> Result<Self, String> {
        if x >= P || z >= P || (x == 0 && z == 0) {
            return Err("invalid projective parameter".into());
        }
        Ok(if z == 0 {
            Self::Identity
        } else {
            Self::Finite(mul(x, inv(z)))
        })
    }
    pub fn product(self, other: Self) -> Result<Self, String> {
        let (x, z) = self.coordinates()?;
        let (y, w) = other.coordinates()?;
        Self::from_coordinates(sub(mul(x, y), mul(z, w)), add(mul(x, w), mul(z, y)))
    }
    pub fn inverse(self) -> Result<Self, String> {
        let (x, z) = self.coordinates()?;
        Self::from_coordinates(x, sub(0, z))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    entries: [[u64; 2]; 2],
}

impl Matrix {
    pub fn new(entries: [[u64; 2]; 2]) -> Result<Self, String> {
        if entries.iter().flatten().any(|x| *x >= P) {
            return Err("noncanonical matrix".into());
        }
        let m = Self { entries };
        if m.determinant() == 0 {
            return Err("singular projective matrix".into());
        }
        Ok(m)
    }
    pub fn determinant(self) -> u64 {
        let [[a, b], [c, d]] = self.entries;
        sub(mul(a, d), mul(b, c))
    }
    pub fn inverse(self) -> Self {
        let [[a, b], [c, d]] = self.entries;
        Self {
            entries: [[d, sub(0, b)], [sub(0, c), a]],
        }
    }
    pub fn product(self, rhs: Self) -> Self {
        let mut out = [[0; 2]; 2];
        for (i, row) in out.iter_mut().enumerate() {
            for (j, x) in row.iter_mut().enumerate() {
                *x = add(
                    mul(self.entries[i][0], rhs.entries[0][j]),
                    mul(self.entries[i][1], rhs.entries[1][j]),
                );
            }
        }
        Self { entries: out }
    }
    fn raw(self, label: Label) -> Result<(u64, u64), String> {
        let (x, z) = label.coordinates()?;
        let [[a, b], [c, d]] = self.entries;
        Ok((add(mul(a, x), mul(b, z)), add(mul(c, x), mul(d, z))))
    }
    pub fn apply(self, label: Label) -> Result<Label, String> {
        let (x, z) = self.raw(label)?;
        Label::from_coordinates(x, z)
    }
    /// Exact compatibility without normalizing a projective image or inverting.
    pub fn compatible(self, left: Label, right: Label) -> Result<bool, String> {
        let (x, z) = self.raw(left)?;
        let (u, v) = right.coordinates()?;
        Ok(mul(x, v) == mul(u, z))
    }
    fn torus_multiplier(label: Label) -> Result<Self, String> {
        let (e, f) = label.coordinates()?;
        Self::new([[e, sub(0, f)], [f, e]])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Anchor {
    pub s: u64,
    pub delta: u64,
}

fn validate_curve(e: Curve) -> Result<(), String> {
    if e.p != P || e.a >= P || e.b >= P {
        return Err("requires canonical M31 curve coefficients".into());
    }
    if add(mul(4, mul(mul(e.a, e.a), e.a)), mul(27, mul(e.b, e.b))) == 0 {
        return Err("singular curve".into());
    }
    Ok(())
}

fn validate_anchor(e: Curve, anchor: Anchor) -> Result<(), String> {
    if anchor.s >= P || anchor.delta == 0 || anchor.delta >= P {
        return Err("invalid trace-zero anchor coordinates".into());
    }
    if Arithmetic::default().rhs(e, anchor.s) != sub(0, mul(anchor.delta, anchor.delta)) {
        return Err("anchor is not a trace-zero point on this curve".into());
    }
    Ok(())
}

/// Inputs and stored state contain public coordinates and supplied labels only.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    curve: Curve,
    residual: (u64, u64),
    anchors: [Anchor; 2],
    factors: [Label; 2],
    transport: Matrix,
    slope_to_label: [Matrix; 2],
}

impl Frame {
    pub fn curve(self) -> Curve {
        self.curve
    }
    pub fn residual(self) -> (u64, u64) {
        self.residual
    }
    pub fn anchors(self) -> [Anchor; 2] {
        self.anchors
    }
    pub fn factors(self) -> [Label; 2] {
        self.factors
    }
    pub fn transport(self) -> Matrix {
        self.transport
    }
    pub fn new(
        e: Curve,
        k: Point,
        anchors: [Anchor; 2],
        factors: [Label; 2],
    ) -> Result<Self, String> {
        validate_curve(e)?;
        let Some((w, v)) = k else {
            return Err("identity residual unsupported".into());
        };
        if v == 0 || !Arithmetic::default().on_curve(e, k) {
            return Err("requires canonical on-curve residual with nonzero ordinate".into());
        }
        for anchor in anchors {
            validate_anchor(e, anchor)?;
        }
        let h1 = sub(anchors[0].s, w);
        let h2 = sub(anchors[1].s, w);
        if h1 == 0 || h2 == 0 {
            return Err("rational residual intersects trace-zero support".into());
        }
        let m1 = Matrix::torus_multiplier(factors[0])?;
        let m2 = Matrix::torus_multiplier(factors[1])?;
        let normal = Matrix::new([
            [mul(h2, anchors[0].delta), mul(sub(h1, h2), v)],
            [0, mul(h1, anchors[1].delta)],
        ])?;
        let transport = m2.product(normal).product(m1.inverse());
        let slope_to_label = [
            m1.product(Matrix::new([[sub(0, h1), v], [0, anchors[0].delta]])?),
            m2.product(Matrix::new([[sub(0, h2), v], [0, anchors[1].delta]])?),
        ];
        Ok(Self {
            curve: e,
            residual: (w, v),
            anchors,
            factors,
            transport,
            slope_to_label,
        })
    }

    /// Build genuine four-point divisor prefix factors from public points.
    pub fn from_prefix(
        e: Curve,
        r: Point,
        p1: Point,
        p2: Point,
        anchors: [Anchor; 2],
    ) -> Result<Self, String> {
        validate_curve(e)?;
        for anchor in anchors {
            validate_anchor(e, anchor)?;
        }
        let mut ar = Arithmetic::default();
        for p in [r, p1, p2] {
            if !ar.on_curve(e, p) {
                return Err("invalid public prefix/target point".into());
            }
        }
        let a = ar.add(e, p1, p2);
        let negative_a = ar.neg(e, a);
        let k = ar.add(e, r, negative_a);
        let mut factors = [Label::Identity; 2];
        for j in 0..2 {
            factors[j] =
                chord_ratio(e, p1, p2, anchors[j])?.product(chord_ratio(e, a, k, anchors[j])?)?;
        }
        Self::new(e, k, anchors, factors)
    }

    pub fn label_from_slope(self, side: usize, slope: Option<u64>) -> Result<Label, String> {
        let Some(matrix) = self.slope_to_label.get(side) else {
            return Err("invalid label side".into());
        };
        matrix.apply(slope.map_or(Label::Identity, Label::Finite))
    }
    pub fn slope_from_label(self, side: usize, label: Label) -> Result<Option<u64>, String> {
        let Some(matrix) = self.slope_to_label.get(side) else {
            return Err("invalid label side".into());
        };
        Ok(match matrix.inverse().apply(label)? {
            Label::Identity => None,
            Label::Finite(m) => Some(m),
        })
    }

    /// Recover all ordered rational residual pairs for this supplied left label.
    /// Infinity is the pair {O,K}; finite labels require only the chord quadratic.
    pub fn extract(
        self,
        left_label: Label,
        d: &mut Diagnostics,
    ) -> Result<Vec<(Point, Point)>, String> {
        let Some(m) = self.slope_from_label(0, left_label)? else {
            return Ok(vec![
                (None, Some(self.residual)),
                (Some(self.residual), None),
            ]);
        };
        let (w, v) = self.residual;
        let m2 = mul(m, m);
        let s = sub(m2, w);
        let q = add(
            add(add(mul(w, m2), mul(w, w)), self.curve.a),
            mul(add(v, v), m),
        );
        let mut pairs = Vec::new();
        for x in roots(1, sub(0, s), q, d, false) {
            let z = sub(s, x);
            let p1 = Some((x, sub(mul(m, sub(x, w)), v)));
            let p2 = Some((z, sub(mul(m, sub(z, w)), v)));
            pairs.push((p1, p2));
        }
        pairs.sort_unstable();
        pairs.dedup();
        Ok(pairs)
    }

    /// None means incompatible labels; Some(empty) means a compatible nonsplit
    /// divisor. Preserve the distinction and reject incompatibility before roots.
    pub fn extract_compatible(
        self,
        left: Label,
        right: Label,
        d: &mut Diagnostics,
    ) -> Result<Option<Vec<(Point, Point)>>, String> {
        if !self.transport.compatible(left, right)? {
            return Ok(None);
        }
        Ok(Some(self.extract(left, d)?))
    }
}

/// Projective line evaluation avoids a slope inversion. Vertical factors cancel.
pub fn chord_ratio(e: Curve, left: Point, right: Point, anchor: Anchor) -> Result<Label, String> {
    validate_curve(e)?;
    validate_anchor(e, anchor)?;
    let mut ar = Arithmetic::default();
    if !ar.on_curve(e, left) || !ar.on_curve(e, right) {
        return Err("invalid chord point".into());
    }
    let (Some((x, y)), Some((z, t))) = (left, right) else {
        return Ok(Label::Identity);
    };
    if x == z && add(y, t) == 0 {
        return Ok(Label::Identity);
    }
    let (n, d) = if x == z {
        (add(mul(3, mul(x, x)), e.a), add(y, y))
    } else {
        (sub(t, y), sub(z, x))
    };
    let c = sub(mul(n, x), mul(d, y));
    Label::from_coordinates(sub(c, mul(n, anchor.s)), mul(d, anchor.delta))
}

#[derive(Debug)]
pub struct LabelSet(BTreeSet<Label>);

impl LabelSet {
    pub fn new(labels: &[Label]) -> Result<Self, String> {
        for label in labels {
            label.coordinates()?;
        }
        Ok(Self(labels.iter().copied().collect()))
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default, Debug)]
pub struct QueryStats {
    pub visited: usize,
    pub membership_tests: usize,
}

/// Exact intersection of two supplied sets under the residual Mobius map.
/// Sorted sets make replay independent of randomized hashing. This is a linear
/// scan of the smaller set, with logarithmic membership, not a sublinear index.
pub fn intersect(
    frame: Frame,
    left: &LabelSet,
    right: &LabelSet,
) -> Result<(Vec<(Label, Label)>, QueryStats), String> {
    let mut out = Vec::new();
    let mut stats = QueryStats::default();
    if left.len() <= right.len() {
        for label in &left.0 {
            let image = frame.transport.apply(*label)?;
            stats.visited += 1;
            stats.membership_tests += 1;
            if right.0.contains(&image) {
                out.push((*label, image));
            }
        }
    } else {
        let reverse = frame.transport.inverse();
        for label in &right.0 {
            let preimage = reverse.apply(*label)?;
            stats.visited += 1;
            stats.membership_tests += 1;
            if left.0.contains(&preimage) {
                out.push((preimage, *label));
            }
        }
    }
    out.sort_unstable();
    Ok((out, stats))
}
