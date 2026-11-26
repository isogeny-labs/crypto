//! Direct original sparse equations and complete original assignment replay.
pub mod controls;
pub mod model;
pub mod process;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use torus_coset_circuit::{aggregation::TargetCircuit, index::IndexTable, Spec};
use torus_s4::{curve25519_prime, Curve, Field, Point, System};
pub type Result<T> = std::result::Result<T, String>;
pub struct Query {
    p: BigUint,
    variables: usize,
    script: String,
}
impl Query {
    pub fn validate(&self) -> Result<()> {
        if !(1..=4096).contains(&self.variables)
            || self.p.bits() > 256
            || self.p < BigUint::from(5u32)
            || !self.p.bit(0)
            || self.script.len() > 2 << 20
        {
            return Err("query shape/modulus/size admission".into());
        }
        Ok(())
    }
}
pub(crate) fn validate_system(s: &System) -> Result<()> {
    if !(1..=4096).contains(&s.variables.len())
        || s.equations.len() > 8192
        || s.p.bits() > 256
        || s.p < BigUint::from(5u32)
        || !s.p.bit(0)
    {
        return Err("system admission".into());
    }
    let mut terms = 0usize;
    for e in &s.equations {
        // Poly keeps its modulus private. Original constructors establish it;
        // the public solve API accepts Spec/R/Q, never a caller-built System.
        for (m, c) in &e.terms {
            if m.len() > 2
                || m.windows(2).any(|w| w[0] > w[1])
                || m.iter().any(|i| *i >= s.variables.len())
                || c >= &s.p
            {
                return Err("malformed sparse term".into());
            }
            terms = terms.checked_add(1).ok_or("term count overflow")?;
        }
    }
    if terms > 50_000 {
        return Err("term admission".into());
    }
    Ok(())
}
fn push(s: &mut String, part: &str) -> Result<()> {
    if part.len() > (2usize << 20).saturating_sub(s.len()) {
        return Err("SMT byte admission".into());
    }
    s.push_str(part);
    Ok(())
}
/// Preserve every original variable and polynomial row, without gate expansion.
pub(crate) fn export(s: &System) -> Result<Query> {
    export_internal(s, None)
}
/// Explicit extension control. Public solve never calls this with known values.
pub(crate) fn export_asserted(s: &System, known: &[BigUint]) -> Result<Query> {
    check_canonical(s, known)?;
    s.check(known)?;
    export_internal(s, Some(known))
}
fn export_internal(s: &System, known: Option<&[BigUint]>) -> Result<Query> {
    validate_system(s)?;
    let mut out=format!("(set-logic QF_FF)\n(set-option :produce-models true)\n(define-sort F () (_ FiniteField {}))\n",s.p);
    for i in 0..s.variables.len() {
        push(&mut out, &format!("(declare-const w{i} F)\n"))?;
    }
    for e in &s.equations {
        let mut terms = Vec::new();
        for (m, c) in &e.terms {
            let mut factors = Vec::new();
            if !c.is_one() || m.is_empty() {
                factors.push(format!("(as ff{c} F)"));
            }
            factors.extend(m.iter().map(|i| format!("w{i}")));
            terms.push(if factors.len() == 1 {
                factors.remove(0)
            } else {
                format!("(ff.mul {})", factors.join(" "))
            });
        }
        let expr = match terms.len() {
            0 => "(as ff0 F)".into(),
            1 => terms.remove(0),
            _ => format!("(ff.add {})", terms.join(" ")),
        };
        push(&mut out, &format!("(assert (= {expr} (as ff0 F)))\n"))?;
    }
    if let Some(v) = known {
        for (i, x) in v.iter().enumerate() {
            push(&mut out, &format!("(assert (= w{i} (as ff{x} F)))\n"))?;
        }
    }
    push(&mut out, "(check-sat)\n")?;
    let q = Query {
        p: s.p.clone(),
        variables: s.variables.len(),
        script: out,
    };
    q.validate()?;
    Ok(q)
}
pub(crate) fn check_canonical(s: &System, v: &[BigUint]) -> Result<()> {
    validate_system(s)?;
    if v.len() != s.variables.len() || v.iter().any(|x| x >= &s.p) {
        return Err("noncanonical or incomplete original assignment".into());
    }
    Ok(())
}
pub struct PointQuery {
    circuit: TargetCircuit,
    spec: Spec,
}
pub fn compile(spec: &Spec, r: Point, target: Point) -> Result<PointQuery> {
    let toy = spec.p == BigUint::from(101u32);
    let (a, q, ca) = if toy {
        (17, 3, 95)
    } else {
        (60824497, 213156431, 486662)
    };
    if (!toy && spec.p != curve25519_prime())
        || spec.order_factor != a
        || spec.q != q
        || spec.curve_a != Some(BigUint::from(ca as u64))
        || spec.k == 0
        || spec.k > 65536
        || spec.k * 2 >= a
        || spec.h.a >= spec.p
        || spec.h.b >= spec.p
    {
        return Err("unsupported original parameter contract".into());
    }
    let f = Field::new(spec.p.clone())?;
    if spec.h.norm(&f) != BigUint::one()
        || spec.h.pow(&(BigUint::from(a) * q), &f) != torus_s4_tree::Fp2::one()
        || spec.h.pow(&a.into(), &f) == torus_s4_tree::Fp2::one()
        || spec.h.pow(&q.into(), &f) == torus_s4_tree::Fp2::one()
    {
        return Err("original exact-order contract".into());
    }
    let curve = Curve {
        f,
        a: (ca as u64).into(),
    };
    if [&r, &target]
        .iter()
        .filter_map(|p| p.as_ref())
        .any(|(x, y)| x >= &spec.p || y >= &spec.p)
        || !curve.contains(&r)
        || !curve.contains(&target)
        || curve.times(&r, 8)? != target
        || r.as_ref().is_some_and(|(_, y)| y.is_zero())
    {
        return Err("public R/Q chart or [8] contract".into());
    }
    let circuit = TargetCircuit::compile(spec, 6, target.clone(), true)?;
    validate_system(&circuit.system)?;
    Ok(PointQuery {
        circuit,
        spec: spec.clone(),
    })
}
impl PointQuery {
    pub fn check_values(&self, values: &[BigUint]) -> Result<Value> {
        check_canonical(&self.circuit.system, values)?;
        self.circuit.system.check(values)?;
        let table = IndexTable::new(&self.spec)?;
        let row = self.circuit.decode_row(values, &table)?;
        Ok(
            json!({"kind":"checked_original_point_root","full_original_equations_checked":true,"indexed_group_replay_checked":true,"values":values.iter().map(ToString::to_string).collect::<Vec<_>>(),"row":row.iter().map(|t|json!({"j":t.j,"t":t.t,"coefficient":t.coefficient,"point":t.point.as_ref().map(|(x,y)|[x.to_string(),y.to_string()])})).collect::<Vec<_>>() }),
        )
    }
    pub fn check_model(&self, text: &str) -> Result<Value> {
        let n = self.circuit.system.variables.len();
        let layout: Vec<_> = (0..n).map(|i| (i, i)).collect();
        let v = model::parse(text, &self.spec.p, &layout).map_err(|e| format!("model: {e}"))?;
        self.check_values(&v)
    }
}
/// Original public problem only. A known assignment is not an API parameter.
pub fn solve(
    spec: &Spec,
    r: Point,
    target: Point,
    exe: &std::path::Path,
    limits: &process::Limits,
) -> Result<Value> {
    let public_problem = json!({"p":spec.p.to_string(),"q":spec.q,"k":spec.k,"order_factor":spec.order_factor,"R":r.as_ref().map(|(x,y)|[x.to_string(),y.to_string()]),"Q":target.as_ref().map(|(x,y)|[x.to_string(),y.to_string()])});
    let c = compile(spec, r, target)?;
    let query = export(&c.circuit.system)?;
    let run = process::run(exe, &query, limits)?;
    let mut result = match run.status.as_deref() {
        Some("sat") if run.refusal.is_none() => match c.check_model(&run.model) {
            Ok(x) => x,
            Err(reason) => json!({"kind":"model_or_projection_refusal","reason":reason}),
        },
        Some("unsat" | "unknown") if run.refusal.is_none() => {
            json!({"kind":"no_root_observation","scope":"unverified native status; not target absence or hardness"})
        }
        _ => json!({"kind":"resource_or_backend_refusal","reason":run.refusal}),
    };
    result["backend"] = run.json();
    result["original_system"] = c.circuit.stats();
    result["public_problem"] = public_problem;
    Ok(result)
}
