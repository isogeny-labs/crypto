//! Numeric bridges between every EFD curve model and short Weierstrass.
//!
//! The catalogue's `toweierstrass` / `fromweierstrass` lines are the pinned
//! EFD maps; the `a0..a6` lines are the model's long-Weierstrass invariants.
//! This module evaluates those records over `F_p` and composes them with the
//! standard long-to-short Weierstrass transformation
//!
//! ```text
//! a0 y^2 + a1 x y + a3 y = x^3 + a2 x^2 + a4 x + a6
//! ```
//!
//! so every one of the ten models can feed the prime-field index-calculus
//! engine, which works on `y^2 = x^3 + A x + B`.
//!
//! The caller supplies a point on the model's own curve.  These functions do
//! not validate curve membership; use [`Model::on_model`] for that.

use super::program::Expr;
use super::*;

/// The long-Weierstrass invariants and the short-Weierstrass curve they
/// determine, together with the constants needed for the point maps.
#[derive(Clone, Debug)]
pub struct WeierstrassBridge {
    pub modulus: BigUint,
    /// Model parameters used by the pinned `toweierstrass` maps.
    pub params: Bindings,
    /// `a0 y^2 + a1 x y + a3 y = x^3 + a2 x^2 + a4 x + a6`.
    pub a0: FieldElement,
    pub a1: FieldElement,
    pub a2: FieldElement,
    pub a3: FieldElement,
    pub a4: FieldElement,
    pub a6: FieldElement,
    /// `B2 = a0 a2 + a1^2/4`, the completed-square `X^2` coefficient before
    /// the final shift, in the scaled coordinates `X = a0 x`, `W = a0^2 y`.
    pub b2: FieldElement,
    pub short_a: FieldElement,
    pub short_b: FieldElement,
    inv2: FieldElement,
    inv3: FieldElement,
    inv_a0: FieldElement,
}

impl WeierstrassBridge {
    /// The short-Weierstrass coefficients as canonical integers.
    pub fn short_coefficients(&self) -> (BigUint, BigUint) {
        (self.short_a.value.clone(), self.short_b.value.clone())
    }

    /// The short-Weierstrass x of a model point whose `toweierstrass` map is
    /// the identity (`shortw`, `montgom`), from the model x alone.
    pub fn short_x(&self, x: &FieldElement) -> FieldElement {
        self.a0.mul(x).add(&self.b2.mul(&self.inv3))
    }
}

fn field_line(text: &str, name: &str, modulus: &BigUint, scope: &Bindings) -> Result<FieldElement> {
    let prefix = format!("{name} ");
    let line = text
        .lines()
        .find(|line| line.starts_with(&prefix) || line.starts_with(&format!("{name}=")))
        .ok_or_else(|| EfdError::MissingInput(name.into()))?;
    let (_, expression) = line
        .split_once('=')
        .ok_or_else(|| EfdError::Syntax(line.into()))?;
    Expr::parse(expression)?.eval(modulus, scope)
}

fn relation_values(
    text: &str,
    prefix: &str,
    modulus: &BigUint,
    scope: &mut Bindings,
) -> Result<Vec<FieldElement>> {
    let mut out = Vec::new();
    for line in text.lines() {
        let Some(body) = line.strip_prefix(prefix) else {
            continue;
        };
        let (name, expression) = body
            .split_once('=')
            .ok_or_else(|| EfdError::Syntax(line.into()))?;
        let value = Expr::parse(expression)?.eval(modulus, scope)?;
        scope.insert(name.trim().into(), value.clone());
        out.push(value);
    }
    if out.is_empty() {
        return Err(EfdError::MissingInput(prefix.trim().into()));
    }
    Ok(out)
}

impl Model {
    /// Evaluate this model's long-Weierstrass invariants and derive the
    /// equivalent short-Weierstrass curve.  `a0` must be invertible and the
    /// characteristic must not be 2 or 3.
    pub fn weierstrass_bridge(
        &self,
        modulus: &BigUint,
        params: &Bindings,
    ) -> Result<WeierstrassBridge> {
        check_field(modulus, params)?;
        let two = FieldElement::new(BigUint::from(2u32), modulus.clone());
        let three = FieldElement::new(BigUint::from(3u32), modulus.clone());
        let four = FieldElement::new(BigUint::from(4u32), modulus.clone());
        let twenty_seven = FieldElement::new(BigUint::from(27u32), modulus.clone());
        let inv2 = two.inv().ok_or(EfdError::ZeroDenominator)?;
        let inv3 = three.inv().ok_or(EfdError::ZeroDenominator)?;
        let inv4 = four.inv().ok_or(EfdError::ZeroDenominator)?;
        let inv27 = twenty_seven.inv().ok_or(EfdError::ZeroDenominator)?;

        let a0 = field_line(&self.source.text, "a0", modulus, params)?;
        let a1 = field_line(&self.source.text, "a1", modulus, params)?;
        let a2 = field_line(&self.source.text, "a2", modulus, params)?;
        let a3 = field_line(&self.source.text, "a3", modulus, params)?;
        let a4 = field_line(&self.source.text, "a4", modulus, params)?;
        let a6 = field_line(&self.source.text, "a6", modulus, params)?;
        let inv_a0 = a0.inv().ok_or(EfdError::ZeroDenominator)?;

        // Scale y by a0 and x by a0, then complete the square and the cube.
        let a0_sq = a0.mul(&a0);
        let a0_cu = a0_sq.mul(&a0);
        let big_a1 = a1.clone();
        let big_a2 = a0.mul(&a2);
        let big_a3 = a0.mul(&a3);
        let big_a4 = a0_sq.mul(&a4);
        let big_a6 = a0_cu.mul(&a6);
        let b2 = big_a2.add(&big_a1.mul(&big_a1).mul(&inv4));
        let b4 = big_a4.add(&big_a1.mul(&big_a3).mul(&inv2));
        let b6 = big_a6.add(&big_a3.mul(&big_a3).mul(&inv4));
        let short_a = b4.sub(&b2.mul(&b2).mul(&inv3));
        let short_b = b6
            .sub(&b2.mul(&b4).mul(&inv3))
            .add(&b2.mul(&b2).mul(&b2).mul(&inv27).mul(&two));

        Ok(WeierstrassBridge {
            modulus: modulus.clone(),
            params: params.clone(),
            a0,
            a1,
            a2,
            a3,
            a4,
            a6,
            b2,
            short_a,
            short_b,
            inv2,
            inv3,
            inv_a0,
        })
    }

    /// Map affine model coordinates to the long-Weierstrass coordinates
    /// `(u, v)` given by this model's `toweierstrass` record.
    pub fn to_long_weierstrass(
        &self,
        modulus: &BigUint,
        params: &Bindings,
        point: &[FieldElement],
    ) -> Result<(FieldElement, FieldElement)> {
        let coordinates = self.coordinates();
        if coordinates.len() != point.len() {
            return Err(EfdError::InvalidPoint);
        }
        let mut scope = params.clone();
        for (name, value) in coordinates.iter().zip(point) {
            scope.insert(name.clone(), value.clone());
        }
        let values = relation_values(&self.source.text, "toweierstrass ", modulus, &mut scope)?;
        if values.len() < 2 {
            return Err(EfdError::Unsupported(format!(
                "{}: toweierstrass does not give two coordinates",
                self.id
            )));
        }
        Ok((values[0].clone(), values[1].clone()))
    }

    /// Map long-Weierstrass coordinates `(u, v)` back to this model's affine
    /// coordinates (two coordinates, or three for Jacobi intersections).
    pub fn from_long_weierstrass(
        &self,
        modulus: &BigUint,
        params: &Bindings,
        u: &FieldElement,
        v: &FieldElement,
    ) -> Result<Vec<FieldElement>> {
        let mut scope = params.clone();
        // The pinned coordinate names differ by model (`u`/`v`, `weierx`/
        // `weiery`, or `x`/`y` for the Jacobi intersection).  Bind the two
        // names the model's own `toweierstrass` record uses.
        let names: Vec<String> = self
            .source
            .text
            .lines()
            .filter_map(|line| line.strip_prefix("toweierstrass "))
            .filter_map(|body| body.split_once('=').map(|(name, _)| name.trim().to_owned()))
            .collect();
        for alias in ["u", "weierx", "x"] {
            scope.insert(alias.into(), u.clone());
        }
        for alias in ["v", "weiery", "y"] {
            scope.insert(alias.into(), v.clone());
        }
        if names.len() >= 2 {
            scope.insert(names[0].clone(), u.clone());
            scope.insert(names[1].clone(), v.clone());
        }
        relation_values(&self.source.text, "fromweierstrass ", modulus, &mut scope)
    }

    /// Map an affine model point to the short-Weierstrass curve in `bridge`.
    pub fn to_short_weierstrass(
        &self,
        bridge: &WeierstrassBridge,
        point: &[FieldElement],
    ) -> Result<(FieldElement, FieldElement)> {
        let (u, v) = self.to_long_weierstrass(&bridge.modulus, &bridge.params, point)?;
        let x = bridge.a0.mul(&u);
        let w = bridge.a0.mul(&bridge.a0).mul(&v);
        let xp = x.add(&bridge.b2.mul(&bridge.inv3));
        let y = w.add(&bridge.a1.mul(&x).add(&bridge.a3).mul(&bridge.inv2));
        Ok((xp, y))
    }

    /// Map a short-Weierstrass point back to this model's affine coordinates.
    pub fn from_short_weierstrass(
        &self,
        bridge: &WeierstrassBridge,
        xp: &FieldElement,
        y: &FieldElement,
    ) -> Result<Vec<FieldElement>> {
        let x = xp.sub(&bridge.b2.mul(&bridge.inv3));
        let w = y.sub(&bridge.a1.mul(&x).add(&bridge.a3).mul(&bridge.inv2));
        let u = x.mul(&bridge.inv_a0);
        let v = w.mul(&bridge.inv_a0).mul(&bridge.inv_a0);
        self.from_long_weierstrass(&bridge.modulus, &bridge.params, &u, &v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 1019;

    fn fe(v: i64) -> FieldElement {
        let modulus = BigUint::from(P);
        if v < 0 {
            FieldElement::new(BigUint::from((-v) as u64), modulus.clone()).neg()
        } else {
            FieldElement::new(BigUint::from(v as u64), modulus)
        }
    }

    fn params_for(model: &str) -> Bindings {
        let mut v = Bindings::new();
        match model {
            "shortw" => {
                v.insert("a".into(), fe(2));
                v.insert("b".into(), fe(3));
            }
            "montgom" => {
                v.insert("a".into(), fe(131));
                v.insert("b".into(), fe(1));
            }
            "edwards" => {
                v.insert("c".into(), fe(1));
                v.insert("d".into(), fe(64));
            }
            "twisted" => {
                v.insert("a".into(), fe(2));
                v.insert("d".into(), fe(12));
            }
            "twistedhessian" => {
                // `a0` has denominator `a - (d/3)^3`; keep it nonzero.
                v.insert("a".into(), fe(1));
                v.insert("d".into(), fe(6));
            }
            "hessian" => {
                // x = 1 with y = 3: d = (y^3 + 2)/(3y).
                let y = fe(3);
                let num = y.mul(&y).mul(&y).add(&fe(2));
                let den = fe(3).mul(&y);
                v.insert("d".into(), num.mul(&den.inv().unwrap()));
            }
            "jintersect" => {
                v.insert("a".into(), fe(2));
            }
            "jquartic" => {
                v.insert("a".into(), fe(2));
            }
            "2dik" | "3dik" => {
                v.insert("a".into(), fe(2));
            }
            other => unreachable!("unknown model {other}"),
        }
        v
    }

    fn short_points(bridge: &WeierstrassBridge, want: usize) -> Vec<(FieldElement, FieldElement)> {
        let mut out = Vec::new();
        let one = FieldElement::one(BigUint::from(P));
        for x in 0..P {
            let x = fe(x as i64);
            let rhs = x
                .mul(&x)
                .mul(&x)
                .add(&bridge.short_a.mul(&x))
                .add(&bridge.short_b);
            if let Some(y) = super::super::sqrt(&rhs) {
                out.push((x, y));
                if out.len() == want {
                    break;
                }
            }
            let _ = &one;
        }
        out
    }

    #[test]
    fn all_ten_models_round_trip_through_short_weierstrass() {
        let catalogue = super::super::catalogue();
        assert_eq!(catalogue.models.len(), 10);
        for model in &catalogue.models {
            let params = params_for(&model.id);
            let bridge = model
                .weierstrass_bridge(&BigUint::from(P), &params)
                .unwrap_or_else(|e| panic!("{}: bridge: {e}", model.id));
            assert!(
                !bridge.short_a.value.is_zero() || !bridge.short_b.value.is_zero(),
                "{}: degenerate short curve",
                model.id
            );
            for (x, y) in short_points(&bridge, 8) {
                let coords = match model.from_short_weierstrass(&bridge, &x, &y) {
                    Ok(coords) => coords,
                    // Exceptional points of the rational map (identity,
                    // 2-torsion, chart boundary) are not in its domain.
                    Err(EfdError::ZeroDenominator) => continue,
                    Err(e) => panic!("{}: from short: {e}", model.id),
                };
                let on_model = model
                    .on_model(&BigUint::from(P), &params, &coords)
                    .unwrap_or_else(|e| panic!("{}: model check: {e}", model.id));
                assert!(on_model, "{}: image is not on the model", model.id);
                let (x2, y2) = match model.to_short_weierstrass(&bridge, &coords) {
                    Ok(point) => point,
                    Err(EfdError::ZeroDenominator) => continue,
                    Err(e) => panic!("{}: to short: {e}", model.id),
                };
                assert_eq!((x2.value, y2.value), (x.value, y.value), "{}", model.id);
            }
        }
    }
}
