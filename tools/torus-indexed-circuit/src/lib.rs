//! Redundant indexed augmentation of the unchanged original point equations.
//! This compiler is not a root-discovery solver. Primality remains external.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use serde_json::{json, Value};
use torus_coset_circuit::{aggregation::shifted, aggregation::TargetCircuit, Circuit, Spec};
use torus_s4::{Field, Point, Poly, System};
use torus_s4_tree::Fp2;

pub mod controls;
pub type Result<T> = std::result::Result<T, String>;
type Pair = (Poly, Poly);
const MAX_VARIABLES: usize = 16384;
const MAX_EQUATIONS: usize = 32768;

#[derive(Clone, Serialize)]
pub struct MemberRoles {
    pub parameter: [usize; 2],
    pub selector_bits: Vec<usize>,
    pub original_slack_bits: Vec<usize>,
    pub index_bits: Vec<usize>,
    pub branch_bit: Option<usize>,
    pub raw_v: Option<usize>,
}
#[derive(Clone)]
enum Op {
    Bit { member: usize, bit: u32 },
    Expr(Poly),
}
struct Builder {
    system: System,
    ops: Vec<Op>,
}
impl Builder {
    fn c(&self, n: u64) -> Poly {
        Poly::small(&self.system.p, n)
    }
    fn constant(&self, n: BigUint) -> Poly {
        Poly::constant(&self.system.p, n)
    }
    fn equation(&mut self, p: Poly) -> Result<()> {
        if self.system.equations.len() == MAX_EQUATIONS {
            return Err("construction equation cap".into());
        }
        self.system.equations.push(p);
        Ok(())
    }
    fn alloc(&mut self, name: &str, op: Op) -> Result<Poly> {
        if self.system.variables.len() == MAX_VARIABLES {
            return Err("construction variable cap".into());
        }
        let i = self.system.variables.len();
        let v = Poly::var(&self.system.p, i);
        if let Op::Expr(p) = &op {
            self.equation(&v - p)?;
        }
        self.system.variables.push(format!("indexed_{name}_{i}"));
        self.ops.push(op);
        Ok(v)
    }
    fn product(&mut self, a: &Pair, b: &Pair) -> Result<Pair> {
        let real = &(&a.0 * &b.0) + &(&a.1 * &b.1).scale(2);
        let imag = &(&a.0 * &b.1) + &(&a.1 * &b.0);
        Ok((
            self.alloc("product_a", Op::Expr(real))?,
            self.alloc("product_b", Op::Expr(imag))?,
        ))
    }
    fn selected_power(
        &mut self,
        initial: Pair,
        base: Fp2,
        bits: &[Poly],
        f: &Field,
    ) -> Result<Pair> {
        let mut selected = initial;
        let mut power = base;
        for bit in bits {
            let factor = (
                &self.c(1) + &(bit * &self.constant(f.sub(&power.a, &BigUint::one()))),
                bit * &self.constant(power.b.clone()),
            );
            selected = self.product(&selected, &factor)?;
            power = power.mul(&power, f);
        }
        Ok(selected)
    }
    fn borrow(&mut self, x: &[Poly], slack: &[Poly], cap: u64) -> Result<()> {
        let mut incoming = self.c(0);
        for (i, (digit, slack_digit)) in x.iter().zip(slack).enumerate() {
            let one = (cap >> i) & 1;
            let expr = if one == 0 {
                &(digit + &incoming) - &(digit * &incoming)
            } else {
                digit * &incoming
            };
            let outgoing = if i + 1 < x.len() {
                self.alloc("borrow", Op::Expr(expr))?
            } else {
                self.equation(expr)?;
                self.c(0)
            };
            let expected = &(&(&self.c(one) - digit) - &incoming) + &outgoing.scale(2);
            self.equation(slack_digit - &expected)?;
            incoming = outgoing;
        }
        Ok(())
    }
    fn range(&mut self, bits: &[Poly], cap: u64) -> Result<()> {
        let mut equal = self.c(1);
        for (i, bit) in bits.iter().enumerate().rev() {
            let one = (cap >> i) & 1;
            if one == 0 {
                self.equation(&equal * bit)?;
            }
            if i != 0 {
                let same = if one == 1 {
                    bit.clone()
                } else {
                    &self.c(1) - bit
                };
                equal = self.alloc("prefix_equal", Op::Expr(&equal * &same))?;
            }
        }
        Ok(())
    }
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut out = vec![];
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d = if d == 2 { 3 } else { d + 2 };
    }
    if n != 1 {
        out.push(n);
    }
    out
}
fn admit(s: &Spec) -> Result<Field> {
    if !(5..=512).contains(&s.p.bits()) || !s.p.bit(0) {
        return Err("odd5..512bit field required; primality external".into());
    }
    if s.q == 0 || s.order_factor == 0 || s.q > u32::MAX.into() || s.order_factor > u32::MAX.into()
    {
        return Err("construction a/q positive32bit cap".into());
    }
    let f = Field::new(s.p.clone())?;
    if BigUint::from(2u32).modpow(&((&s.p - 1u32) >> 1), &s.p) != &s.p - 1u32 {
        return Err("nonsquare2 field contract failed".into());
    }
    if s.h.a >= s.p || s.h.b >= s.p || s.h.norm(&f) != BigUint::one() {
        return Err("noncanonical/nonunit torus generator".into());
    }
    let m = s
        .order_factor
        .checked_mul(s.q)
        .ok_or("order product overflow")?;
    if s.h.pow(&m.into(), &f) != Fp2::one() {
        return Err("generator order closure failed".into());
    }
    let mut primes = factors(s.order_factor);
    primes.extend(factors(s.q));
    primes.sort_unstable();
    primes.dedup();
    for r in primes {
        if s.h.pow(&(m / r).into(), &f) == Fp2::one() {
            return Err("generator order is a proper divisor".into());
        }
    }
    Ok(f)
}
fn variable_index(p: &Poly) -> Result<usize> {
    let mut terms = p.terms.iter();
    let (m, c) = terms.next().ok_or("missing symbolic variable")?;
    if m.len() != 1 || !c.is_one() || terms.next().is_some() {
        return Err("role is not a single variable".into());
    }
    Ok(m[0])
}
fn named_input(c: &Circuit, prefix: &str, offset: usize) -> Result<Option<usize>> {
    let ids: Vec<_> = c
        .system
        .variables
        .iter()
        .enumerate()
        .filter(|(_, n)| n.starts_with(prefix))
        .map(|(i, _)| i + offset)
        .collect();
    if ids.len() > 1 {
        return Err("ambiguous version-bound original role".into());
    }
    Ok(ids.first().copied())
}

pub struct Augmented {
    pub system: System,
    pub original: System,
    pub roles: Vec<MemberRoles>,
    ops: Vec<Op>,
    index_width: u32,
}
impl Augmented {
    pub fn member(s: &Spec) -> Result<(Circuit, Self)> {
        let f = admit(s)?;
        let c = Circuit::compile(s)?;
        let a = Self::append(s, &c.system, &c, 1, &f)?;
        Ok((c, a))
    }
    pub fn target(
        s: &Spec,
        arity: usize,
        target: Point,
        distinct: bool,
    ) -> Result<(TargetCircuit, Self)> {
        let f = admit(s)?;
        let c = TargetCircuit::compile(s, arity, target, distinct)?;
        let member = Circuit::compile(s)?;
        let a = Self::append(s, &c.system, &member, arity, &f)?;
        Ok((c, a))
    }
    fn append(
        s: &Spec,
        original: &System,
        member: &Circuit,
        arity: usize,
        f: &Field,
    ) -> Result<Self> {
        if original.variables.len() > MAX_VARIABLES || original.equations.len() > MAX_EQUATIONS {
            return Err("original construction dimension cap".into());
        }
        let n = member.system.variables.len();
        let e = member.system.equations.len();
        for leaf in 0..arity {
            let actual = original
                .equations
                .get(leaf * e..(leaf + 1) * e)
                .ok_or("original member prefix absent")?;
            if !actual
                .iter()
                .zip(&member.system.equations)
                .all(|(a, b)| *a == shifted(b, leaf * n))
            {
                return Err("version-bound member layout changed".into());
            }
        }
        let mut b = Builder {
            system: original.clone(),
            ops: vec![],
        };
        let width = 64 - (s.q - 1).leading_zeros();
        let mut roles = vec![];
        for leaf in 0..arity {
            let first = member.first_selector();
            let mut selectors = vec![];
            let mut slack = vec![];
            for bit in 0..member.width as usize {
                let at = first.ok_or("selector role missing")? + 2 * bit + leaf * n;
                if at + 1 >= (leaf + 1) * n {
                    return Err("selector/slack layout outside member".into());
                }
                selectors.push(Poly::var(&s.p, at));
                slack.push(Poly::var(&s.p, at + 1));
            }
            let mut bits = vec![];
            for bit in 0..width {
                let v = b.alloc("t_bit", Op::Bit { member: leaf, bit })?;
                b.equation(&v * &(&v - &b.c(1)))?;
                bits.push(v);
            }
            b.borrow(&selectors, &slack, s.k - 1)?;
            b.range(&bits, s.q - 1)?;
            let initial = (b.constant(s.h.a.clone()), b.constant(s.h.b.clone()));
            let j = b.selected_power(initial, s.h.clone(), &selectors, f)?;
            let z = b.selected_power(j, s.h.pow(&s.order_factor.into(), f), &bits, f)?;
            let (pa, pb) = member.parameter_polys();
            let parameter = (shifted(&pa, leaf * n), shifted(&pb, leaf * n));
            b.equation(&parameter.0 - &z.0)?;
            b.equation(&parameter.1 - &z.1)?;
            roles.push(MemberRoles {
                parameter: [variable_index(&parameter.0)?, variable_index(&parameter.1)?],
                selector_bits: selectors
                    .iter()
                    .map(variable_index)
                    .collect::<Result<_>>()?,
                original_slack_bits: slack.iter().map(variable_index).collect::<Result<_>>()?,
                index_bits: bits.iter().map(variable_index).collect::<Result<_>>()?,
                branch_bit: named_input(member, "branch_", leaf * n)?,
                raw_v: named_input(member, "raw_v_", leaf * n)?,
            });
        }
        Ok(Self {
            system: b.system,
            original: original.clone(),
            roles,
            ops: b.ops,
            index_width: width,
        })
    }
    /// Qualification helper only: t labels are NOT inputs to a discovery solver.
    /// Returns an assignment candidate; callers must still check both systems.
    /// Representable overflow t is permitted so the range constraints can reject it.
    pub fn extension_candidate(&self, values: &[BigUint], ts: &[u64]) -> Result<Vec<BigUint>> {
        if values.len() != self.original.variables.len()
            || ts.len() != self.roles.len()
            || values.iter().any(|x| x >= &self.system.p)
        {
            return Err("noncanonical original assignment/label shape".into());
        }
        if ts.iter().any(|t| *t >= 1u64 << self.index_width) {
            return Err("t cannot be represented by declared bits".into());
        }
        let mut out = values.to_vec();
        for op in &self.ops {
            let x = match op {
                Op::Bit { member, bit } => ((ts[*member] >> bit) & 1).into(),
                Op::Expr(p) => p.evaluate(&out)?,
            };
            out.push(x);
        }
        Ok(out)
    }
    pub fn project_and_check(&self, values: &[BigUint]) -> Result<Vec<BigUint>> {
        if values.iter().any(|x| x >= &self.system.p) {
            return Err("noncanonical augmented assignment".into());
        }
        self.system.check(values)?;
        let prefix = &values[..self.original.variables.len()];
        self.original.check(prefix)?;
        Ok(prefix.to_vec())
    }
    pub fn indices(&self, values: &[BigUint]) -> Result<Vec<u64>> {
        self.project_and_check(values)?;
        self.roles
            .iter()
            .map(|r| {
                let mut t = 0;
                for (i, at) in r.index_bits.iter().enumerate() {
                    if values[*at].is_one() {
                        t |= 1u64 << i;
                    } else if !values[*at].is_zero() {
                        return Err("nonboolean index role".into());
                    }
                }
                Ok(t)
            })
            .collect()
    }
    pub fn prefix_retained(&self) -> bool {
        self.system.variables.starts_with(&self.original.variables)
            && self.system.equations.starts_with(&self.original.equations)
            && self.system.p == self.original.p
    }
    pub fn stats(&self) -> Value {
        json!({"original_variables":self.original.variables.len(),
            "original_equations":self.original.equations.len(),
            "variables":self.system.variables.len(),"equations":self.system.equations.len(),
            "terms":self.system.equations.iter().map(|p|p.terms.len()).sum::<usize>(),
            "max_degree":self.system.equations.iter().map(Poly::degree).max().unwrap_or(0),
            "original_prefix_retained":self.prefix_retained(),"index_width":self.index_width,
            "index_input_bits":self.roles.iter().map(|r|r.index_bits.len()).sum::<usize>(),
            "derived_new_variables":self.ops.iter().filter(|x|matches!(x,Op::Expr(_))).count(),
            "roles":self.roles,"role_binding":"version-bound original layout plus checked member equations",
            "root_discovery_implemented":false,"primality_contract":"external",
            "complete_cost_accounting":false})
    }
}
