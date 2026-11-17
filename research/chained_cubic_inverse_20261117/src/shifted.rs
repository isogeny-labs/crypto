use super::{Balanced, Class, Result};
use crate::{
    fp2::{Fp2, Fp2Ctx},
    fp_poly::{fp_inv, FpPoly},
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use std::collections::{BTreeMap, BTreeSet};

const P: u64 = 101;
fn residue(a: u64) -> BigUint {
    BigUint::from(a)
}
fn value(a: &BigUint) -> u64 {
    a.to_u64().unwrap()
}
fn inv(a: u64) -> Result<u64> {
    fp_inv(&residue(a), &residue(P))
        .map(|x| value(&x))
        .ok_or("zero inverse".into())
}
fn neg(a: u64) -> u64 {
    (P - a) % P
}
fn power(poly: &FpPoly, n: usize) -> FpPoly {
    (0..n).fold(FpPoly::one(poly.p.clone()), |v, _| v.mul(poly))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Forward,
    Reverse,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Normal,
    OmitPositiveLost,
    SwapOldInfinities,
    OmitShift,
}
#[derive(Clone)]
struct Chart {
    source: Balanced,
    target: Balanced,
    a: u64,
    b: u64,
    bi: u64,
}
struct Mapped {
    class: Class,
    lost: usize,
}
impl Chart {
    /// Binary substitution. Forward A=1+aw,B=w; reverse A=1,B=x-a.
    fn substitute(poly: &FpPoly, degree: usize, direction: Direction, a: u64) -> Result<FpPoly> {
        if poly.degree().is_some_and(|d| d > degree) {
            return Err("substitution weight".into());
        }
        let make =
            |c: &[u64]| FpPoly::from_coeffs(c.iter().copied().map(residue).collect(), residue(P));
        let (upper, lower) = match direction {
            Direction::Forward => (make(&[1, a]), make(&[0, 1])),
            Direction::Reverse => (make(&[1]), make(&[neg(a), 1])),
        };
        let mut out = FpPoly::zero(residue(P));
        for i in 0..=degree {
            out = out.add(
                &power(&upper, i)
                    .mul(&power(&lower, degree - i))
                    .scalar_mul(&poly.coeff(i)),
            );
        }
        Ok(out)
    }
    fn new(source: &Balanced, a: u64, b: u64) -> Result<Self> {
        if source.p != P
            || a >= P
            || b == 0
            || b >= P
            || value(&source.f.eval(&residue(a))) != b * b % P
        {
            return Err("noncanonical/off-curve/ramified chart parameter".into());
        }
        let bi = inv(b)?;
        let output = Self::substitute(&source.f, 6, Direction::Forward, a)?
            .scalar_mul(&residue(bi * bi % P));
        let coefficients: Vec<_> = output.coeffs.iter().map(value).collect();
        let target = Balanced::new(P, &coefficients)?;
        let reverse =
            Self::substitute(&target.f, 6, Direction::Reverse, a)?.scalar_mul(&residue(b * b % P));
        if reverse != source.f {
            return Err("inverse model coefficient mismatch".into());
        }
        Ok(Self {
            source: source.clone(),
            target,
            a,
            b,
            bi,
        })
    }
    fn map_mode(&self, d: &Class, direction: Direction, mode: Mode) -> Result<Mapped> {
        let (source, target, scale) = match direction {
            Direction::Forward => (&self.source, &self.target, self.bi),
            Direction::Reverse => (&self.target, &self.source, self.b),
        };
        source.check(d)?;
        let (input_u, input_v, old_plus) = d.components();
        let m = d.degree();
        let a = if mode == Mode::OmitShift && direction == Direction::Forward {
            0
        } else {
            self.a
        };
        let raw = Self::substitute(input_u, m, direction, a)?;
        let remaining = raw.degree().ok_or("zero transformed support")?;
        let lost = m.checked_sub(remaining).ok_or("support degree increased")?;
        let u = raw.monic();
        let v = Self::substitute(input_v, 3, direction, a)?
            .scalar_mul(&residue(scale))
            .rem(&u);
        let mut out = target.reduce_support(u, v, 0)?;
        if lost != 0 {
            let at = if direction == Direction::Forward {
                self.a
            } else {
                0
            };
            let branch = value(&input_v.eval(&residue(at))) * scale % P;
            if branch == 1 {
                if mode != Mode::OmitPositiveLost {
                    out = target.add(&out, &target.mul(&target.delta(), lost as u64)?)?;
                }
            } else if branch != P - 1 {
                return Err("lost pole has invalid infinity branch".into());
            }
        }
        let qx = if direction == Direction::Forward {
            0
        } else {
            self.a
        };
        let mut qy = scale;
        if mode == Mode::SwapOldInfinities {
            qy = neg(qy);
        }
        let plus = target.point(qx, qy, 1)?;
        let minus = target.point(qx, neg(qy), 1)?;
        out = target.add(&out, &target.mul(&plus, u64::from(old_plus))?)?;
        out = target.add(
            &out,
            &target.mul(&minus, (2 - m) as u64 - u64::from(old_plus))?,
        )?;
        Ok(Mapped { class: out, lost })
    }
    fn map(&self, d: &Class, direction: Direction) -> Result<Mapped> {
        self.map_mode(d, direction, Mode::Normal)
    }
}

#[derive(Clone, Copy, Debug)]
enum Geometric {
    Finite(Fp2, Fp2),
    Infinity(u64),
}
#[derive(Clone)]
struct Witness {
    label: String,
    class: Class,
    effective: [Geometric; 2],
}
fn eval2(c: &Balanced, x: Fp2, k: &Fp2Ctx) -> Fp2 {
    let (f, _) = c.equation_and_infinity();
    f.coeffs
        .iter()
        .rev()
        .fold(k.zero(), |s, a| k.add(k.mul(s, x), k.from_fp(value(a))))
}
fn on_curve(c: &Balanced, point: Geometric, k: &Fp2Ctx) -> bool {
    match point {
        Geometric::Finite(x, y) => k.square(y) == eval2(c, x, k),
        Geometric::Infinity(t) => t == 1 || t == P - 1,
    }
}
fn pointwise(chart: &Chart, point: Geometric, k: &Fp2Ctx) -> Result<Geometric> {
    if !on_curve(&chart.source, point, k) {
        return Err("oracle source point off curve".into());
    }
    let image = match point {
        Geometric::Infinity(sign) => Geometric::Finite(k.zero(), k.from_fp(sign * chart.bi % P)),
        Geometric::Finite(x, y) => {
            let difference = k.sub(x, k.from_fp(chart.a));
            if k.is_zero(difference) {
                let ratio = k.mul(y, k.from_fp(chart.bi));
                if ratio.b != 0 || ![1, P - 1].contains(&ratio.a) {
                    return Err("oracle pole branch".into());
                }
                Geometric::Infinity(ratio.a)
            } else {
                let w = k.inv(difference).ok_or("oracle inverse")?;
                let z = k.mul(k.mul(y, k.pow(w, 3)), k.from_fp(chart.bi));
                Geometric::Finite(w, z)
            }
        }
    };
    if !on_curve(&chart.target, image, k) {
        return Err("oracle image off curve".into());
    }
    Ok(image)
}
fn interpolate_pair(c: &Balanced, p: Geometric, q: Geometric, k: &Fp2Ctx) -> Result<Class> {
    let (Geometric::Finite(x, y), Geometric::Finite(xx, yy)) = (p, q) else {
        return Err("finite pair required".into());
    };
    let slope = if x == xx {
        if y != yy || k.is_zero(y) {
            return Err("tangent requires nonzero same ordinate".into());
        }
        let mut derivative = k.zero();
        for (i, coefficient) in c.f.coeffs.iter().enumerate().skip(1) {
            derivative = k.add(
                derivative,
                k.mul(
                    k.from_fp(i as u64 * value(coefficient) % P),
                    k.pow(x, (i - 1) as u64),
                ),
            );
        }
        k.mul(derivative, k.inv(k.add(y, y)).ok_or("tangent inverse")?)
    } else {
        k.mul(
            k.sub(y, yy),
            k.inv(k.sub(x, xx)).ok_or("interpolation inverse")?,
        )
    };
    let constant = k.sub(y, k.mul(slope, x));
    let product = k.mul(x, xx);
    let sum = k.neg(k.add(x, xx));
    if [constant, slope, product, sum].iter().any(|z| z.b != 0) {
        return Err("quadratic support did not descend".into());
    }
    c.wrap(
        c.poly(&[product.a, sum.a, 1]),
        c.poly(&[constant.a, slope.a]),
        0,
    )
}
fn oracle(chart: &Chart, witness: &Witness, k: &Fp2Ctx) -> Result<Class> {
    let images = [
        pointwise(chart, witness.effective[0], k)?,
        pointwise(chart, witness.effective[1], k)?,
    ];
    if images
        .iter()
        .any(|p| matches!(p,Geometric::Finite(x,_)if x.b!=0))
    {
        return interpolate_pair(&chart.target, images[0], images[1], k);
    }
    let mut out = chart.target.zero();
    for point in images {
        let class = match point {
            Geometric::Infinity(1) => chart.target.delta(),
            Geometric::Infinity(t) if t == P - 1 => chart.target.zero(),
            Geometric::Finite(x, y) if x.b == 0 && y.b == 0 => chart.target.point(x.a, y.a, 1)?,
            _ => return Err("oracle mixed/nonrational point image".into()),
        };
        out = chart.target.add(&out, &class)?;
    }
    chart
        .target
        .add(&out, &chart.target.neg(&chart.target.delta())?)
}
fn rational_points(c: &Balanced) -> Vec<(u64, u64)> {
    let mut out = Vec::new();
    for x in 0..P {
        let rhs = value(&c.f.eval(&residue(x)));
        for y in 0..P {
            if y * y % P == rhs {
                out.push((x, y));
            }
        }
    }
    out
}
fn quadratic_square_table(k: &Fp2Ctx) -> BTreeMap<(u64, u64), Fp2> {
    let mut roots = BTreeMap::new();
    for a in 0..P {
        for b in 0..P {
            let y = k.element(a, b);
            let square = k.square(y);
            roots.entry((square.a, square.b)).or_insert(y);
        }
    }
    roots
}
fn witnesses(
    chart: &Chart,
    k: &Fp2Ctx,
    roots: &BTreeMap<(u64, u64), Fp2>,
    aux: bool,
) -> Result<Vec<Witness>> {
    let c = &chart.source;
    let finite = |x, y| Geometric::Finite(k.from_fp(x), k.from_fp(y));
    let mut out = vec![
        Witness {
            label: "identity".into(),
            class: c.zero(),
            effective: [Geometric::Infinity(1), Geometric::Infinity(P - 1)],
        },
        Witness {
            label: "delta".into(),
            class: c.delta(),
            effective: [Geometric::Infinity(1); 2],
        },
        Witness {
            label: "negative_delta".into(),
            class: c.neg(&c.delta())?,
            effective: [Geometric::Infinity(P - 1); 2],
        },
    ];
    let rational = rational_points(c);
    let q = *rational
        .iter()
        .find(|&&(x, y)| x != chart.a && y != 0)
        .ok_or("no Q")?;
    let r = *rational
        .iter()
        .find(|&&(x, y)| x != chart.a && x != q.0 && y != 0)
        .ok_or("no R")?;
    for (label, (x, y)) in [
        ("pole_positive", (chart.a, chart.b)),
        ("pole_negative", (chart.a, neg(chart.b))),
        ("ordinary_Q", q),
    ] {
        for plus in 0..=1 {
            out.push(Witness {
                label: format!("{label}_infinity{plus}"),
                class: c.point(x, y, plus)?,
                effective: [
                    finite(x, y),
                    Geometric::Infinity(if plus == 1 { 1 } else { P - 1 }),
                ],
            });
        }
    }
    for (label, p, q) in [
        ("pole_positive_plus_Q", (chart.a, chart.b), q),
        ("pole_negative_plus_Q", (chart.a, neg(chart.b)), q),
        (
            "double_positive_pole",
            (chart.a, chart.b),
            (chart.a, chart.b),
        ),
        (
            "double_negative_pole",
            (chart.a, neg(chart.b)),
            (chart.a, neg(chart.b)),
        ),
        ("double_Q", q, q),
        ("distinct_Q_R", q, r),
    ] {
        let effective = [finite(p.0, p.1), finite(q.0, q.1)];
        out.push(Witness {
            label: label.into(),
            class: interpolate_pair(c, effective[0], effective[1], k)?,
            effective,
        });
    }
    let mut closed = None;
    'search: for a in 0..P {
        for b in 1..P {
            let x = k.element(a, b);
            let rhs = eval2(c, x, k);
            if k.is_zero(rhs) {
                continue;
            }
            if let Some(&y) = roots.get(&(rhs.a, rhs.b)) {
                closed = Some((x, y));
                break 'search;
            }
        }
    }
    let (x, y) = closed.ok_or("no nonramified quadratic orbit")?;
    for (label, y) in [
        ("quadratic_orbit", y),
        ("opposite_quadratic_orbit", k.neg(y)),
    ] {
        let effective = [
            Geometric::Finite(x, y),
            Geometric::Finite(k.pow(x, P), k.pow(y, P)),
        ];
        out.push(Witness {
            label: label.into(),
            class: interpolate_pair(c, effective[0], effective[1], k)?,
            effective,
        });
    }
    if aux {
        for x in [10, 91] {
            for plus in 0..=1 {
                out.push(Witness {
                    label: format!("ramified_{x}_infinity{plus}"),
                    class: c.point(x, 0, plus)?,
                    effective: [
                        finite(x, 0),
                        Geometric::Infinity(if plus == 1 { 1 } else { P - 1 }),
                    ],
                });
            }
        }
        let effective = [finite(10, 0), finite(91, 0)];
        out.push(Witness {
            label: "two_ramified_points".into(),
            class: interpolate_pair(c, effective[0], effective[1], k)?,
            effective,
        });
    }
    let want = if aux { 22 } else { 17 };
    if out.len() != want {
        return Err(format!("witness coverage{} !={want}", out.len()));
    }
    for witness in &out {
        c.check(&witness.class)?;
        if witness
            .effective
            .iter()
            .any(|&point| !on_curve(c, point, k))
        {
            return Err("witness geometric support off curve".into());
        }
    }
    Ok(out)
}

fn fixtures() -> Result<Vec<(String, [u64; 7])>> {
    let mut lines = include_str!("../fixtures-12.csv").lines();
    if lines.next() != Some("name,f0,f1,f2,f3,f4,f5,f6") {
        return Err("fixture header".into());
    }
    let mut out = Vec::new();
    for line in lines {
        let columns: Vec<_> = line.split(',').collect();
        if columns.len() != 8 {
            return Err("fixture width".into());
        }
        let mut coefficients = [0; 7];
        for (slot, text) in coefficients.iter_mut().zip(&columns[1..]) {
            *slot = text.parse().map_err(|_| "fixture integer")?;
            if *slot >= P || slot.to_string() != *text {
                return Err("fixture canonical integer".into());
            }
        }
        out.push((columns[0].to_owned(), coefficients));
    }
    if out.len() != 13 {
        return Err("fixture population".into());
    }
    Ok(out)
}

pub fn run_unit() -> Result<()> {
    let k = Fp2Ctx::try_new(P).map_err(|e| e.to_string())?;
    let roots = quadratic_square_table(&k);
    let mut total_witnesses = 0;
    let mut total_pairs = 0;
    let mut total_controls = 0;
    for (name, coefficients) in fixtures()? {
        let source = Balanced::new(P, &coefficients)?;
        let aux = name.starts_with("auxiliary_");
        let points = rational_points(&source);
        let (a, b) = if aux {
            (2, 41)
        } else {
            *points
                .iter()
                .find(|&&(x, y)| x != 0 && y != 0)
                .ok_or("no nonzero shifted parameter")?
        };
        let chart = Chart::new(&source, a, b)?;
        let opposite = Chart::new(&source, a, neg(b))?;
        if opposite.target.f != chart.target.f {
            return Err("opposite sign changed target model".into());
        }
        let set = witnesses(&chart, &k, &roots, aux)?;
        let mut forward_losses = BTreeSet::new();
        let mut reverse_losses = BTreeSet::new();
        let mut shapes = BTreeSet::new();
        let mut images = Vec::new();
        for witness in &set {
            let image = chart.map(&witness.class, Direction::Forward)?;
            let recovered = chart.map(&image.class, Direction::Reverse)?;
            let expected = oracle(&chart, witness, &k)?;
            let negative = opposite.map(&witness.class, Direction::Forward)?;
            if image.class != expected
                || recovered.class != witness.class
                || negative.class != chart.target.neg(&image.class)?
            {
                return Err(format!(
                    "{name}/{}: oracle/inverse/opposite-sign mismatch",
                    witness.label
                ));
            }
            if opposite.map(&negative.class, Direction::Reverse)?.class != witness.class {
                return Err("negative chart inverse mismatch".into());
            }
            shapes.insert((witness.class.degree(), witness.class.plus));
            forward_losses.insert(image.lost);
            reverse_losses.insert(recovered.lost);
            println!("trace_balanced12,{name},{},input={:?},image={:?},inverse={:?},oracle={:?},forward_lost={},reverse_lost={},geometric={:?}",witness.label,witness.class.key(),image.class.key(),recovered.class.key(),expected.key(),image.lost,recovered.lost,witness.effective);
            images.push(image.class);
        }
        if shapes != BTreeSet::from([(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (2, 0)])
            || forward_losses != BTreeSet::from([0, 1, 2])
            || reverse_losses != BTreeSet::from([0, 1, 2])
        {
            return Err(format!(
                "{name}: shapes/loss coverage {shapes:?}/{forward_losses:?}/{reverse_losses:?}"
            ));
        }
        for (i, first) in set.iter().enumerate() {
            for (j, second) in set.iter().enumerate() {
                let sum = source.add(&first.class, &second.class)?;
                let mapped = chart.map(&sum, Direction::Forward)?.class;
                let expected = chart.target.add(&images[i], &images[j])?;
                let recovered = chart.map(&mapped, Direction::Reverse)?.class;
                if mapped != expected || recovered != sum {
                    return Err(format!("{name}: pair{i}/{j} addition/inverse mismatch"));
                }
                println!("trace_balanced12_pair,{name},{i},{j},source_sum={:?},mapped={:?},target_sum={:?},recovered={:?}",sum.key(),mapped.key(),expected.key(),recovered.key());
                total_pairs += 1;
            }
        }
        for mode in [
            Mode::OmitPositiveLost,
            Mode::SwapOldInfinities,
            Mode::OmitShift,
        ] {
            let mut detected = false;
            for (witness, expected) in set.iter().zip(&images) {
                match chart.map_mode(&witness.class, Direction::Forward, mode) {
                    Ok(bad) if bad.class != *expected => {
                        println!("control_balanced12,{name},{mode:?},witness={},actual_wrong={:?},expected={:?}",witness.label,bad.class.key(),expected.key());
                        detected = true;
                        break;
                    }
                    Err(reason) => {
                        println!("control_balanced12,{name},{mode:?},witness={},structured_rejection={reason},expected={:?}",witness.label,expected.key());
                        detected = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !detected {
                return Err(format!("{name}: mutant{mode:?} undetected"));
            }
            total_controls += 1;
        }
        let wrong_b = (1..P)
            .find(|c| c * c % P != value(&source.f.eval(&residue(a))))
            .ok_or("wrong parameter control unavailable")?;
        if Chart::new(&source, a, 0).is_ok() || Chart::new(&source, a, wrong_b).is_ok() {
            return Err("invalid parameter accepted".into());
        }
        println!("fixture_balanced12={name}; a={a}; b={b}; source={coefficients:?}; target={:?}; witnesses={}; pairs={}; forward_losses={forward_losses:?}; reverse_losses={reverse_losses:?}; shapes={shapes:?}; three_mutants=rejected; negative_sign=group_negation",chart.target.f.coeffs.iter().map(value).collect::<Vec<_>>(),set.len(),set.len()*set.len());
        total_witnesses += set.len();
    }
    if Balanced::new(P, &[0, 0, 0, 0, 0, 0, 1]).is_ok()
        || Balanced::new(P, &[P, 0, 0, 0, 0, 0, 1]).is_ok()
    {
        return Err("singular/noncanonical control accepted".into());
    }
    if (total_witnesses, total_pairs, total_controls) != (226, 3952, 39) {
        return Err("total unit coverage".into());
    }
    println!("balanced12=accepted; models=13; witnesses={total_witnesses}; ordered_pairs={total_pairs}; semantic_mutants={total_controls}; source_core=reused; curve_chart_only_no_Richelot_composition; unsigned");
    Ok(())
}

// New visibility-only adapter around the unchanged unit12 chart/witness code.
pub(crate) struct Bridge14(Chart);
impl Bridge14 {
    pub(crate) fn new(source: &Balanced, a: u64, b: u64) -> Result<Self> {
        Chart::new(source, a, b).map(Self)
    }
    pub(crate) fn target(&self) -> &Balanced {
        &self.0.target
    }
    pub(crate) fn forward(&self, d: &Class) -> Result<Class> {
        self.0.map(d, Direction::Forward).map(|m| m.class)
    }
    pub(crate) fn backward(&self, d: &Class) -> Result<Class> {
        self.0.map(d, Direction::Reverse).map(|m| m.class)
    }
}
pub(crate) fn witnesses14(source: &Balanced) -> Result<Vec<(String, Class)>> {
    let k = Fp2Ctx::try_new(P).map_err(|e| e.to_string())?;
    let chart = Chart::new(source, 0, 20)?;
    let roots = quadratic_square_table(&k);
    Ok(witnesses(&chart, &k, &roots, false)?
        .into_iter()
        .map(|w| (w.label, w.class))
        .collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_frozen_shifted_balanced_witnesses_pairs_and_controls() {
        super::run_unit().unwrap();
    }
}
