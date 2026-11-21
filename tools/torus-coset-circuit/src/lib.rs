//! A lifted membership circuit, not a decomposition algorithm.
//! Exact-order/prime-field and map contracts remain external assumptions.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use torus_s4::{Field, Point, Poly, System};
use torus_s4_tree::Fp2;

pub mod aggregation;
pub mod aggregation_controls;
pub mod controls;
pub mod index;
pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
pub struct Spec {
    pub p: BigUint,
    pub h: Fp2,
    pub order_factor: u64,
    pub q: u64,
    pub k: u64,
    pub curve_a: Option<BigUint>,
}
#[derive(Clone)]
enum Input {
    Real,
    Imag,
    Selector(u32),
    Slack(u32),
    Branch,
    RawV,
}
#[derive(Clone)]
enum Op {
    Input(Input),
    Expr(Poly),
    Inverse(Poly),
}
type Pair = (Poly, Poly);
struct Builder {
    system: System,
    ops: Vec<Op>,
    pair_muls: usize,
}
impl Builder {
    fn alloc(&mut self, name: &str, op: Op) -> Poly {
        let index = self.system.variables.len();
        let v = Poly::var(&self.system.p, index);
        match &op {
            Op::Expr(p) => self.system.equations.push(&v - p),
            Op::Inverse(p) => self.system.equations.push(&(&v * p) - &self.c(1)),
            Op::Input(_) => {}
        }
        self.system.variables.push(format!("{name}_{index}"));
        self.ops.push(op);
        v
    }
    fn c(&self, n: u64) -> Poly {
        Poly::small(&self.system.p, n)
    }
    fn constant(&self, n: BigUint) -> Poly {
        Poly::constant(&self.system.p, n)
    }
    fn boolean(&mut self, name: &str, kind: Input) -> Poly {
        let v = self.alloc(name, Op::Input(kind));
        self.system.equations.push(&v * &(&v - &self.c(1)));
        v
    }
    fn pair_mul(&mut self, l: &Pair, r: &Pair) -> Pair {
        let real = &(&l.0 * &r.0) + &(&l.1 * &r.1).scale(2);
        let imag = &(&l.0 * &r.1) + &(&l.1 * &r.0);
        self.pair_muls += 1;
        (
            self.alloc("mul_a", Op::Expr(real)),
            self.alloc("mul_b", Op::Expr(imag)),
        )
    }
    fn power(&mut self, z: &Pair, n: u64) -> Pair {
        let mut a = z.clone();
        let mut r: Option<Pair> = None;
        for i in 0..(64 - n.leading_zeros()) {
            if (n >> i) & 1 != 0 {
                r = Some(match r {
                    None => a.clone(), // Fold the first multiplication by one.
                    Some(ref v) => self.pair_mul(v, &a),
                });
            }
            if i + 1 < 64 - n.leading_zeros() {
                a = self.pair_mul(&a, &a);
            }
        }
        r.expect("positive exponent checked by constructor")
    }
    fn double(&mut self, u: &Poly, v: &Poly, square: Option<Poly>, a: &Poly) -> Pair {
        let w = square.unwrap_or_else(|| self.alloc("u_square", Op::Expr(u * u)));
        let inv = self.alloc("inverse_2v", Op::Inverse(v.scale(2)));
        let numerator = &(&w.scale(3) + &(a * &u.scale(2))) + &self.c(1);
        let slope = self.alloc("slope", Op::Expr(&numerator * &inv));
        let next_u = self.alloc(
            "double_u",
            Op::Expr(&(&(&slope * &slope) - a) - &u.scale(2)),
        );
        let next_v = self.alloc("double_v", Op::Expr(&(&slope * &(u - &next_u)) - v));
        (next_u, next_v)
    }
}

pub struct Circuit {
    pub system: System,
    ops: Vec<Op>,
    image: Option<Pair>,
    raw_point: Option<Pair>,
    parameter: Pair,
    pub width: u32,
    power_gates: usize,
    selector_gates: usize,
}
#[derive(Clone)]
pub struct WitnessInput {
    pub z: Fp2,
    /// Zero-based selector x=j-1; may exceed k-1 for rejection controls.
    pub selector: u64,
    pub slack: u64,
    pub branch: BigUint,
    pub raw_v: BigUint,
}
impl Circuit {
    pub fn compile(s: &Spec) -> Result<Self> {
        if s.k == 0 || s.q == 0 || s.k.checked_mul(2).is_none_or(|v| v >= s.order_factor) {
            return Err("invalid exponent/coset bounds".into());
        }
        let width = 64 - (s.k - 1).leading_zeros();
        if width >= 64 || s.p <= (BigUint::one() << (width + 1)) {
            return Err("selector modulus does not prevent wraparound".into());
        }
        let f = Field::new(s.p.clone())?;
        if s.h.a >= f.p || s.h.b >= f.p || s.h.norm(&f) != BigUint::one() {
            return Err("generator coordinates/norm invalid".into());
        }
        let mut b = Builder {
            system: System {
                p: s.p.clone(),
                variables: vec![],
                equations: vec![],
            },
            ops: vec![],
            pair_muls: 0,
        };
        let z = (
            b.alloc("z_a", Op::Input(Input::Real)),
            b.alloc("z_b", Op::Input(Input::Imag)),
        );
        b.system
            .equations
            .push(&(&(&z.0 * &z.0) - &(&z.1 * &z.1).scale(2)) - &b.c(1));
        let mut bits = vec![];
        let mut cap = b.c(0);
        for i in 0..width {
            let bit = b.boolean("selector", Input::Selector(i));
            let slack = b.boolean("slack", Input::Slack(i));
            cap = &cap + &(&bit + &slack).scale(1u64 << i);
            bits.push(bit);
        }
        b.system.equations.push(&cap - &b.c(s.k - 1));
        let power = b.power(&z, s.q);
        let power_gates = b.pair_muls;
        let omega = s.h.pow(&s.q.into(), &f);
        let mut constant_power = omega.clone();
        let mut selected = (b.constant(omega.a.clone()), b.constant(omega.b.clone()));
        for bit in &bits {
            let factor = (
                &b.c(1) + &(bit * &b.constant(f.sub(&constant_power.a, &BigUint::one()))),
                bit * &b.constant(constant_power.b.clone()),
            );
            selected = b.pair_mul(&selected, &factor);
            constant_power = constant_power.mul(&constant_power, &f);
        }
        b.system.equations.push(&power.0 - &selected.0);
        b.system.equations.push(&power.1 - &selected.1);
        let selector_gates = b.pair_muls - power_gates;
        let mut raw_point = None;
        let image = if let Some(a) = &s.curve_a {
            if a.is_zero() || a >= &f.p {
                return Err("invalid curve coefficient".into());
            }
            let branch = b.boolean("branch", Input::Branch);
            let v = b.alloc("raw_v", Op::Input(Input::RawV));
            let inverse = b.alloc("inverse_z_a", Op::Inverse(z.0.clone()));
            let half_a = f.mul(a, &f.inv(&2u32.into())?);
            let center = b.constant(f.sub(&0u32.into(), &half_a));
            let delta = &b.constant(half_a) * &inverse;
            let u = b.alloc(
                "raw_u",
                Op::Expr(&center + &(&(&b.c(1) - &branch.scale(2)) * &delta)),
            );
            let square = b.alloc("raw_u_square", Op::Expr(&u * &u));
            raw_point = Some((u.clone(), v.clone()));
            let ca = b.constant(a.clone());
            b.system
                .equations
                .push(&(&(&(&v * &v) - &(&u * &square)) - &(&ca * &square)) - &u);
            let p2 = b.double(&u, &v, Some(square), &ca);
            let p4 = b.double(&p2.0, &p2.1, None, &ca);
            Some(b.double(&p4.0, &p4.1, None, &ca))
        } else {
            None
        };
        Ok(Self {
            system: b.system,
            ops: b.ops,
            image,
            raw_point,
            parameter: z,
            width,
            power_gates,
            selector_gates,
        })
    }
    pub fn witness(&self, input: &WitnessInput) -> Result<Vec<BigUint>> {
        if input.selector >= 1u64 << self.width || input.slack >= 1u64 << self.width {
            return Err("selector/slack cannot be represented by supplied bits".into());
        }
        let f = Field::new(self.system.p.clone())?;
        let mut values = Vec::with_capacity(self.ops.len());
        for op in &self.ops {
            let value = match op {
                Op::Input(kind) => match kind {
                    Input::Real => input.z.a.clone(),
                    Input::Imag => input.z.b.clone(),
                    Input::Selector(i) => ((input.selector >> i) & 1).into(),
                    Input::Slack(i) => ((input.slack >> i) & 1).into(),
                    Input::Branch => input.branch.clone(),
                    Input::RawV => input.raw_v.clone(),
                },
                Op::Expr(expr) => expr.evaluate(&values)?,
                Op::Inverse(expr) => f.inv(&expr.evaluate(&values)?)?,
            };
            values.push(value);
        }
        Ok(values)
    }
    pub fn image(&self, values: &[BigUint]) -> Result<Point> {
        match &self.image {
            None => Ok(None),
            Some((u, v)) => Ok(Some((u.evaluate(values)?, v.evaluate(values)?))),
        }
    }
    /// Symbolic roles, independent of generated variable names or slots.
    pub fn image_polys(&self) -> Option<(Poly, Poly)> {
        self.image.clone()
    }
    pub fn raw_point_polys(&self) -> Option<(Poly, Poly)> {
        self.raw_point.clone()
    }
    pub fn parameter_polys(&self) -> (Poly, Poly) {
        self.parameter.clone()
    }
    pub fn selector_value(&self, values: &[BigUint]) -> Result<u64> {
        self.system.check(values)?;
        let mut x = 0u64;
        for (i, op) in self.ops.iter().enumerate() {
            if let Op::Input(Input::Selector(bit)) = op {
                if values[i] == BigUint::one() {
                    x |= 1u64 << bit;
                } else if !values[i].is_zero() {
                    return Err("nonbinary selector".into());
                }
            }
        }
        Ok(x + 1)
    }
    pub fn first_selector(&self) -> Option<usize> {
        self.ops
            .iter()
            .position(|x| matches!(x, Op::Input(Input::Selector(0))))
    }
    pub fn stats(&self) -> Value {
        json!({"variables":self.system.variables.len(),"equations":self.system.equations.len(),
            "max_degree":self.system.equations.iter().map(Poly::degree).max().unwrap_or(0),
            "total_terms":self.system.equations.iter().map(|p|p.terms.len()).sum::<usize>(),
            "selector_bits":self.width,"slack_bits":self.width,
            "fp2_power_gates":self.power_gates,"fp2_selector_gates":self.selector_gates,
            "has_nonidentity_cofactor_image":self.image.is_some(),
            "solver_cost_established":false})
    }
}
