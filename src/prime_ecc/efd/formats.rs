//! Affine <-> projective charts for the 33 prime-field representations.
//!
//! Quotient-only formats deliberately return only x, only y, only y^2, or the
//! three Jacobi-intersection coordinates; they do not reconstruct an affine
//! point or provide ordinary arbitrary point addition.
use super::program::Expr;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Projection {
    Full {
        x: FieldElement,
        y: FieldElement,
    },
    /// Jacobi intersections: `(s, c, d)`.
    Triple {
        s: FieldElement,
        c: FieldElement,
        d: FieldElement,
    },
    /// X-only charts (`shortw/xz`, `montgom/xz`).
    X(FieldElement),
    /// Y-only charts (`edwards/yz`).
    Y(FieldElement),
    /// Squared-Y-only charts (`edwards/yzsquared`).
    Y2(FieldElement),
}

impl Projection {
    pub fn x(&self) -> Option<&FieldElement> {
        match self {
            Self::Full { x, .. } | Self::X(x) => Some(x),
            _ => None,
        }
    }
    pub fn y(&self) -> Option<&FieldElement> {
        match self {
            Self::Full { y, .. } | Self::Y(y) => Some(y),
            _ => None,
        }
    }
}

fn div(a: &FieldElement, b: &FieldElement) -> Result<FieldElement> {
    // Public coordinates: variable-time inversion, see `program::Expr::eval`.
    let inverse = b.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
    Ok(a.mul(&inverse))
}

impl Representation {
    /// Encode a finite affine point with nonzero projective scale for the two
    /// coordinate models. The caller supplies a point on this representation's
    /// curve model; model changes are not implicit.
    pub fn encode_affine(
        &self,
        modulus: &BigUint,
        params: &Bindings,
        x: &FieldElement,
        y: &FieldElement,
        z: &FieldElement,
    ) -> Result<Bindings> {
        check_field(modulus, params)?;
        check_field(
            modulus,
            &Bindings::from([
                ("x".into(), x.clone()),
                ("y".into(), y.clone()),
                ("z".into(), z.clone()),
            ]),
        )?;
        if z.is_zero() {
            return Err(EfdError::OutsideChart);
        }
        if self.id.starts_with("jintersect/") {
            return Err(EfdError::Unsupported(format!(
                "{}: use encode_jintersection",
                self.id
            )));
        }
        let z2 = z.mul(z);
        let z3 = z2.mul(z);
        let mut v = params.clone();
        self.derive_chart_parameters(modulus, &mut v);
        let chart_r = v.get("r").cloned();
        let mut put = |name: &str, value: FieldElement| {
            v.insert(name.into(), value);
        };
        match self.id.as_str() {
            "shortw/jacobian" | "shortw/jacobian-0" | "shortw/jacobian-3" | "shortw/modified" => {
                put("X", x.mul(&z2));
                put("Y", y.mul(&z3));
                put("Z", z.clone());
            }
            "shortw/projective" | "shortw/projective-1" | "shortw/projective-3" => {
                put("X", x.mul(z));
                put("Y", y.mul(z));
                put("Z", z.clone());
            }
            "shortw/w12-0" => {
                put("X", x.mul(z));
                put("Y", y.mul(&z2));
                put("Z", z.clone());
            }
            "shortw/xyzz" | "shortw/xyzz-3" => {
                put("X", x.mul(&z2));
                put("Y", y.mul(&z3));
                put("ZZ", z2);
                put("ZZZ", z3);
            }
            "shortw/xz" | "montgom/xz" => {
                put("X", x.mul(z));
                put("Z", z.clone());
            }
            "2dik/standard" => {
                put("X", x.mul(z));
                put("Y", y.mul(&z2));
                put("Z", z.clone());
            }
            "3dik/standard" => {
                put("X", x.mul(&z2));
                put("Y", y.mul(&z3));
                put("Z", z.clone());
            }
            "edwards/projective"
            | "twisted/projective"
            | "twistedhessian/projective"
            | "hessian/standard"
            | "hessian/extended" => {
                put("X", x.mul(z));
                put("Y", y.mul(z));
                put("Z", z.clone());
            }
            "twisted/extended" | "twisted/extended-1" => {
                put("X", x.mul(z));
                put("Y", y.mul(z));
                put("T", x.mul(y).mul(z));
                put("Z", z.clone());
            }
            "edwards/inverted" | "twisted/inverted" => {
                if x.is_zero() || y.is_zero() {
                    return Err(EfdError::OutsideChart);
                }
                put("X", div(z, x)?);
                put("Y", div(z, y)?);
                put("Z", z.clone());
            }
            "edwards/yz" => {
                let r = chart_r
                    .clone()
                    .ok_or_else(|| EfdError::MissingInput("r".into()))?;
                put("Y", r.mul(y).mul(z));
                put("Z", z.clone());
            }
            "edwards/yzsquared" => {
                let r = chart_r
                    .clone()
                    .ok_or_else(|| EfdError::MissingInput("r".into()))?;
                put("Y", r.mul(&y.mul(y)).mul(z));
                put("Z", z.clone());
            }
            "jquartic/xyz" | "jquartic/2xyz" => {
                put("X", x.mul(z));
                put("Y", y.mul(&z2));
                put("Z", z.clone());
            }
            "jquartic/xxyzz" | "jquartic/xxyzzr" | "jquartic/2xxyzz" | "jquartic/2xxyzzr" => {
                put("X", x.mul(z));
                put("Y", y.mul(&z2));
                put("Z", z.clone());
            }
            // Legacy EFD chart: x=X/Z, y=Y/Z^2 with caches U,V,W,T.
            "jquartic/extended" => {
                put("X", x.mul(z));
                put("Y", y.mul(&z2));
                put("Z", z.clone());
            }
            _ => return Err(EfdError::Unsupported(self.id.clone())),
        }
        self.derive_relations(modulus, &mut v);
        Ok(v.into_iter()
            .filter(|(k, _)| self.variables.iter().any(|v| v == k))
            .collect())
    }

    /// Encode a Jacobi-intersection point `(s, c, d)` with nonzero scale.
    pub fn encode_jintersection(
        &self,
        modulus: &BigUint,
        params: &Bindings,
        s: &FieldElement,
        c: &FieldElement,
        d: &FieldElement,
        z: &FieldElement,
    ) -> Result<Bindings> {
        check_field(modulus, params)?;
        let point = Bindings::from([
            ("s".into(), s.clone()),
            ("c".into(), c.clone()),
            ("d".into(), d.clone()),
            ("z".into(), z.clone()),
        ]);
        check_field(modulus, &point)?;
        if !self.id.starts_with("jintersect/") {
            return Err(EfdError::Unsupported(self.id.clone()));
        }
        if z.is_zero() {
            return Err(EfdError::OutsideChart);
        }
        let mut v = params.clone();
        v.insert("S".into(), s.mul(z));
        v.insert("C".into(), c.mul(z));
        v.insert("D".into(), d.mul(z));
        v.insert("Z".into(), z.clone());
        self.derive_relations(modulus, &mut v);
        Ok(v.into_iter()
            .filter(|(k, _)| self.variables.iter().any(|v| v == k))
            .collect())
    }

    /// Derive chart parameters that are fixed by the model parameters:
    /// `r = sqrt(d)` for the `edwards/yz*` charts, and the missing member of
    /// `a^2 + c^2 = 1` for the doubling-oriented Jacobi quartic charts.
    fn derive_chart_parameters(&self, modulus: &BigUint, values: &mut Bindings) {
        if (self.id == "edwards/yz" || self.id == "edwards/yzsquared") && !values.contains_key("r")
        {
            if let Some(d) = values.get("d").cloned() {
                if let Some(r) = super::sqrt(&d) {
                    values.insert("r".into(), r);
                }
            }
        }
        if self.id.starts_with("jquartic/2") {
            let one = FieldElement::one(modulus.clone());
            match (values.get("a").cloned(), values.get("c").cloned()) {
                (Some(a), None) => {
                    if let Some(c) = super::sqrt(&one.sub(&a.mul(&a))) {
                        values.insert("c".into(), c);
                    }
                }
                (None, Some(c)) => {
                    if let Some(a) = super::sqrt(&one.sub(&c.mul(&c))) {
                        values.insert("a".into(), a);
                    }
                }
                _ => {}
            }
        }
    }

    /// Fill cached coordinate variables (XX, ZZ, R, SC, ...) from the
    /// representation's declared relations. Only relations whose left side is a
    /// bare cached variable are used; curve equations are left to validation.
    fn derive_relations(&self, modulus: &BigUint, values: &mut Bindings) {
        for _ in 0..self.relations.len() + 1 {
            let before = values.len();
            for relation in &self.relations {
                let Ok((left, right)) = program::equality(relation) else {
                    continue;
                };
                let Expr::Variable(name) = &left else {
                    continue;
                };
                if values.contains_key(name) || !self.variables.iter().any(|v| v == name) {
                    continue;
                }
                let Ok(value) = right.eval(modulus, values) else {
                    continue;
                };
                values.insert(name.clone(), value);
            }
            if values.len() == before {
                break;
            }
        }
    }

    /// Add an EFD point suffix (1,2,3) to encoded coordinates.
    pub fn bind_point(&self, values: &Bindings, index: u32) -> Bindings {
        values
            .iter()
            .map(|(k, v)| (format!("{k}{index}"), v.clone()))
            .collect()
    }

    /// Normalize an EFD output (3 for ADD/DBL/SCALE, 5 for DADD, and
    /// 4/5 for the doubled/summed points of a ladder step).
    pub fn project(&self, params: &Bindings, values: &Bindings, index: u32) -> Result<Projection> {
        let get = |s: &str| {
            values
                .get(&format!("{s}{index}"))
                .cloned()
                .ok_or_else(|| EfdError::MissingInput(format!("{s}{index}")))
        };
        if self.variables == ["X"] {
            return Ok(Projection::X(get("X")?));
        }
        if self.id == "edwards/yz" || self.id == "edwards/yzsquared" {
            let r = match params.get("r").cloned() {
                Some(r) => r,
                None => {
                    let d = params
                        .get("d")
                        .ok_or_else(|| EfdError::MissingInput("r or d".into()))?;
                    super::sqrt(d).ok_or_else(|| EfdError::MissingInput("r or d".into()))?
                }
            };
            let y = div(&get("Y")?, &r.mul(&get("Z")?))?;
            return Ok(if self.id == "edwards/yz" {
                Projection::Y(y)
            } else {
                Projection::Y2(y)
            });
        }
        if self.id.starts_with("jintersect/") {
            let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
            return Ok(Projection::Triple {
                s: get("S")?.mul(&zi),
                c: get("C")?.mul(&zi),
                d: get("D")?.mul(&zi),
            });
        }
        if self.id == "shortw/xz" || self.id == "montgom/xz" {
            let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
            return Ok(Projection::X(get("X")?.mul(&zi)));
        }
        if self.id == "edwards/inverted" || self.id == "twisted/inverted" {
            return Ok(Projection::Full {
                x: div(&get("Z")?, &get("X")?)?,
                y: div(&get("Z")?, &get("Y")?)?,
            });
        }
        let (x, y) = match self.id.as_str() {
            "shortw/jacobian" | "shortw/jacobian-0" | "shortw/jacobian-3" | "shortw/modified" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                let z2 = zi.mul(&zi);
                (get("X")?.mul(&z2), get("Y")?.mul(&z2.mul(&zi)))
            }
            "shortw/w12-0" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), get("Y")?.mul(&zi.mul(&zi)))
            }
            "shortw/xyzz" | "shortw/xyzz-3" => (
                div(&get("X")?, &get("ZZ")?)?,
                div(&get("Y")?, &get("ZZZ")?)?,
            ),
            "2dik/standard" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), div(&get("Y")?, &get("ZZ")?)?)
            }
            "3dik/standard" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                let z2 = zi.mul(&zi);
                (get("X")?.mul(&z2), get("Y")?.mul(&z2.mul(&zi)))
            }
            "jquartic/xyz" | "jquartic/2xyz" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), get("Y")?.mul(&zi.mul(&zi)))
            }
            "jquartic/xxyzz" | "jquartic/xxyzzr" | "jquartic/2xxyzz" | "jquartic/2xxyzzr" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), div(&get("Y")?, &get("ZZ")?)?)
            }
            "jquartic/extended" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), get("Y")?.mul(&zi.mul(&zi)))
            }
            "shortw/projective"
            | "shortw/projective-1"
            | "shortw/projective-3"
            | "edwards/projective"
            | "twisted/projective"
            | "twisted/extended"
            | "twisted/extended-1"
            | "hessian/standard"
            | "hessian/extended"
            | "twistedhessian/projective" => {
                let zi = get("Z")?.inv_vartime().ok_or(EfdError::ZeroDenominator)?;
                (get("X")?.mul(&zi), get("Y")?.mul(&zi))
            }
            _ => return Err(EfdError::Unsupported(self.id.clone())),
        };
        Ok(Projection::Full { x, y })
    }
}
