//! The linked EFD genus-1 large-characteristic catalogue: 471 programs,
//! 33 representations on 10 curve shapes.
//!
//! These are opt-in reference operations over the existing prime field
//! [`FieldElement`]. They do not replace the affine implementation, a scalar
//! multiplication or a rho walk. A formula's mixed-input restrictions and
//! cached-parameter assumptions are checked; callers of the low-level evaluator
//! still supply valid model points and, for differential operations, the stated
//! point difference.
//!
//! See `docs/efd/README.md` for sources, charts, exceptional cases and coverage.
use crate::ecc::field::FieldElement;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

mod checked;
mod formats;
mod program;
#[cfg(test)]
mod tests;

pub use checked::{weierstrass_operation, CheckedPoint, ExecutionPath};
pub use formats::Projection;
pub use program::{OperationCounts, Program};

pub type Bindings = BTreeMap<String, FieldElement>;
pub type Result<T> = std::result::Result<T, EfdError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EfdError {
    Syntax(String),
    MissingInput(String),
    Assumption(String),
    FieldMismatch,
    ZeroDenominator,
    OutsideChart,
    InvalidPoint,
    Unsupported(String),
}
impl std::fmt::Display for EfdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for EfdError {}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub url: String,
    pub sha256: String,
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct Model {
    pub id: String,
    pub source: Source,
}
#[derive(Debug, Deserialize)]
pub struct Representation {
    pub id: String,
    pub page_url: String,
    pub variables: Vec<String>,
    pub relations: Vec<String>,
    pub assumptions: Vec<String>,
    pub source: Source,
}
#[derive(Debug, Deserialize)]
pub struct Formula {
    pub id: String,
    pub representation: String,
    pub operation: String,
    pub name: String,
    pub assumptions: Vec<String>,
    pub record: Source,
    pub program: Source,
    pub upstream_verification_url: String,
}
#[derive(Debug, Deserialize)]
pub struct Catalogue {
    pub schema: String,
    #[serde(default)]
    pub section: Option<String>,
    pub source_url: String,
    /// The EFD section marker. Characteristic two catalogues use `2`; this one
    /// uses the string `"p"`.
    pub characteristic: String,
    pub models: Vec<Model>,
    pub representations: Vec<Representation>,
    pub formulas: Vec<Formula>,
}
impl Catalogue {
    pub fn formula(&self, id: &str) -> Option<&Formula> {
        self.formulas.iter().find(|f| f.id == id)
    }
    pub fn representation(&self, id: &str) -> Option<&Representation> {
        self.representations.iter().find(|r| r.id == id)
    }
    pub fn model(&self, id: &str) -> Option<&Model> {
        self.models.iter().find(|m| m.id == id)
    }
    /// Model owning a representation (`shortw/jacobian` -> `shortw`).
    pub fn representation_model(&self, representation: &str) -> Option<&Model> {
        let model = representation.split('/').next()?;
        self.model(model)
    }
}

impl Model {
    /// Coordinate names in model order (`x`, `y`; or `s`, `c`, `d`).
    pub fn coordinates(&self) -> Vec<String> {
        self.source
            .text
            .lines()
            .filter_map(|l| l.strip_prefix("coordinate "))
            .map(str::to_owned)
            .collect()
    }
    /// Model parameter names (`a`, `b`, `d`, ...).
    pub fn parameter_names(&self) -> Vec<String> {
        self.source
            .text
            .lines()
            .filter_map(|l| l.strip_prefix("parameter "))
            .map(str::to_owned)
            .collect()
    }
    /// `toweierstrass` relations of this model, as `(target, expression)`.
    pub fn to_weierstrass(&self) -> Vec<(String, String)> {
        relations_with_prefix(&self.source.text, "toweierstrass ")
    }
    /// `fromweierstrass` relations of this model, as `(target, expression)`.
    pub fn from_weierstrass(&self) -> Vec<(String, String)> {
        relations_with_prefix(&self.source.text, "fromweierstrass ")
    }
    /// True when the Weierstrass bridge is the identity map (`u=x`, `v=y`).
    pub fn is_identity_weierstrass(&self) -> bool {
        let map = self.to_weierstrass();
        map.len() == 2 && map[0].1 == "x" && map[1].1 == "y"
    }
    /// Check a point against every `satisfying` equation of this curve model.
    /// Covers all ten imported shapes without model-specific code.
    pub fn on_model(
        &self,
        modulus: &BigUint,
        params: &Bindings,
        point: &[FieldElement],
    ) -> Result<bool> {
        let coordinates = self.coordinates();
        if coordinates.len() != point.len() {
            return Err(EfdError::InvalidPoint);
        }
        let mut values = params.clone();
        for (name, value) in coordinates.iter().zip(point) {
            values.insert(name.clone(), value.clone());
        }
        check_field(modulus, &values)?;
        for line in self.source.text.lines() {
            let Some(relation) = line.strip_prefix("satisfying ") else {
                continue;
            };
            let (left, right) = program::equality(relation)?;
            if left.eval(modulus, &values)? != right.eval(modulus, &values)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

fn relations_with_prefix(text: &str, prefix: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|l| l.strip_prefix(prefix))
        .filter_map(|r| r.split_once('='))
        .map(|(a, b)| (a.trim().to_owned(), b.trim().to_owned()))
        .collect()
}

/// Pinned mathematical records and three-operand code, with per-source hashes.
pub fn catalogue() -> &'static Catalogue {
    static C: OnceLock<Catalogue> = OnceLock::new();
    C.get_or_init(|| {
        serde_json::from_str(include_str!("catalogue.json")).expect("checked EFD catalogue")
    })
}

impl Formula {
    pub fn compile(&self) -> Result<Program> {
        Program::compile(self)
    }
    /// Evaluate a reference program. No strings are evaluated as Rust or shell.
    /// Cache parameters with defining assumptions are derived if absent and
    /// verified if supplied. Point and differential-relation validity are the
    /// caller's responsibility. Use `weierstrass_operation` for checked ADD/DBL.
    pub fn evaluate(&self, modulus: &BigUint, inputs: &Bindings) -> Result<Bindings> {
        self.compile()?.evaluate(modulus, inputs)
    }
}

pub(super) fn check_field(modulus: &BigUint, inputs: &Bindings) -> Result<()> {
    if modulus <= &BigUint::from(2u32) || inputs.values().any(|v| &v.modulus != modulus) {
        return Err(EfdError::FieldMismatch);
    }
    // Like the existing field API, this trusts the supplied modulus to be
    // prime. It does not infer field identity from the element count.
    Ok(())
}

/// Square root in F_p, if one exists. Uses the `p = 3 mod 4` shortcut when it
/// applies and Tonelli--Shanks otherwise. The input modulus must be prime.
pub(super) fn sqrt(a: &FieldElement) -> Option<FieldElement> {
    let p = &a.modulus;
    if a.is_zero() {
        return Some(a.clone());
    }
    if !is_quadratic_residue(a) {
        return None;
    }
    let three = BigUint::from(3u32);
    if p % BigUint::from(4u32) == three {
        let exp = (p + BigUint::one()) / BigUint::from(4u32);
        return Some(a.pow(&exp));
    }
    // Tonelli--Shanks.
    let one = FieldElement::one(p.clone());
    let p_minus_one = p - BigUint::one();
    let mut q = p_minus_one.clone();
    let mut s = 0u32;
    while &q % BigUint::from(2u32) == BigUint::zero() {
        q /= BigUint::from(2u32);
        s += 1;
    }
    // Find a non-residue z.
    let mut z = FieldElement::new(BigUint::from(2u32), p.clone());
    while is_quadratic_residue(&z) {
        z = z.add(&one);
    }
    let mut m = s;
    let mut c = z.pow(&q);
    let mut t = a.pow(&q);
    let mut r = a.pow(&((&q + BigUint::one()) / BigUint::from(2u32)));
    while t != one {
        let mut i = 0u32;
        let mut t2 = t.clone();
        while t2 != one {
            t2 = t2.mul(&t2);
            i += 1;
            if i == m {
                return None;
            }
        }
        let exp = BigUint::one() << (m - i - 1);
        let b = c.pow(&exp);
        r = r.mul(&b);
        let b2 = b.mul(&b);
        t = t.mul(&b2);
        c = b2;
        m = i;
    }
    Some(r)
}

fn is_quadratic_residue(a: &FieldElement) -> bool {
    let p = &a.modulus;
    let exp = (p - BigUint::one()) / BigUint::from(2u32);
    a.pow(&exp) == FieldElement::one(p.clone())
}
