//! Representation-aware prime-field index calculus.
//!
//! Every curve model and coordinate chart in the EFD `g1p` catalogue can be
//! used as the input/output representation.  The relation engine itself works
//! on the short-Weierstrass model `y^2 = x^3 + A x + B` because the third
//! summation polynomial is expressed in that x-coordinate; this module
//! converts points through the EFD-pinned `toweierstrass` /
//! `fromweierstrass` maps and encodes results in the selected chart.
//!
//! The ten models covered by [`crate::prime_ecc::efd`] are short Weierstrass,
//! Montgomery, Edwards, twisted Edwards, twisted Hessian, Hessian, Jacobi
//! intersection, Jacobi quartic, 2-DIK and 3-DIK.  The 33 charts are those of
//! the catalogue; quotient charts (`X`, `Y`, `Y^2`, `(s,c,d)`) are decoded
//! with an explicit lift index where more than one point matches.

use num_bigint::BigUint;
use serde_json::{json, Value};

use super::prime_small_x::{Curve, PrimeField, Pt};
use crate::ecc::field::FieldElement;
use crate::prime_ecc::efd::{
    catalogue, sqrt, Bindings, Model, Projection, Representation, WeierstrassBridge,
};

/// A decoded EFD point before it is mapped to the short-Weierstrass curve.
#[derive(Clone, Debug)]
enum Decoded {
    /// Affine model coordinates (`x,y`, or `s,c,d` for Jacobi intersections).
    Model(Vec<FieldElement>),
    /// X-only chart (`shortw/xz`, `montgom/xz`).
    X(FieldElement),
    /// Y-only chart (`edwards/yz`).
    Y(FieldElement),
    /// Squared-Y-only chart (`edwards/yzsquared`).
    Y2(FieldElement),
}

/// A curve in an EFD model plus the chart used for input and output.
#[derive(Clone, Debug)]
pub struct EfdRepresentation {
    pub model_id: String,
    pub chart_id: String,
    pub modulus: BigUint,
    pub params: Bindings,
    pub bridge: WeierstrassBridge,
    chart: Option<&'static Representation>,
    model: &'static Model,
}

impl EfdRepresentation {
    /// Build a representation.  `chart_id` is an EFD chart id such as
    /// `montgom/xz`, `shortw/jacobian` or `edwards/yz`; the special value
    /// `affine` means the model's own affine coordinates.
    pub fn new(
        model_id: &str,
        chart_id: &str,
        modulus: &BigUint,
        params: &Bindings,
    ) -> Result<Self, String> {
        let catalogue = catalogue();
        let model = catalogue
            .model(model_id)
            .ok_or_else(|| format!("unknown EFD model {model_id}"))?;
        let chart_id = if chart_id == "affine" || chart_id == model_id || chart_id.contains('/') {
            chart_id.to_owned()
        } else {
            format!("{model_id}/{chart_id}")
        };
        let chart = if chart_id == "affine" || chart_id == model_id {
            None
        } else {
            let chart = catalogue
                .representation(&chart_id)
                .ok_or_else(|| format!("unknown EFD chart {chart_id}"))?;
            if !chart.id.starts_with(&format!("{model_id}/")) {
                return Err(format!(
                    "chart {chart_id} does not belong to model {model_id}"
                ));
            }
            Some(chart)
        };
        let bridge = model
            .weierstrass_bridge(modulus, params)
            .map_err(|e| format!("{e}"))?;
        Ok(Self {
            model_id: model_id.to_owned(),
            chart_id,
            modulus: modulus.clone(),
            params: params.clone(),
            bridge,
            chart,
            model,
        })
    }

    /// The short-Weierstrass `(A, B)` used by the relation engine.
    pub fn short_coefficients(&self) -> (BigUint, BigUint) {
        self.bridge.short_coefficients()
    }

    /// Build the index-calculus curve over the supplied field.
    pub fn curve<F: PrimeField>(
        &self,
        field: F,
        name: String,
        order: BigUint,
        order_is_prime: bool,
        cofactor: u64,
    ) -> Result<Curve<F>, String> {
        let (a, b) = self.short_coefficients();
        Ok(Curve {
            name,
            a: field.embed_big(&a)?,
            b: field.embed_big(&b)?,
            field,
            order,
            order_is_prime,
            cofactor,
        })
    }

    fn model_coordinates(&self) -> Vec<String> {
        self.model.coordinates()
    }

    fn one(&self) -> FieldElement {
        FieldElement::one(self.modulus.clone())
    }

    fn decode_model(&self, coords: &[FieldElement]) -> Result<Decoded, String> {
        if let Some(chart) = &self.chart {
            if coords.len() != chart.variables.len() {
                return Err(format!(
                    "{} takes {} coordinates, got {}",
                    chart.id,
                    chart.variables.len(),
                    coords.len()
                ));
            }
            let mut values = Bindings::new();
            for (name, value) in chart.variables.iter().zip(coords) {
                values.insert(format!("{name}1"), value.clone());
            }
            return match chart
                .project(&self.params, &values, 1)
                .map_err(|e| format!("{}: {e}", chart.id))?
            {
                Projection::Full { x, y } => Ok(Decoded::Model(vec![x, y])),
                Projection::Triple { s, c, d } => Ok(Decoded::Model(vec![s, c, d])),
                Projection::X(x) => Ok(Decoded::X(x)),
                Projection::Y(y) => Ok(Decoded::Y(y)),
                Projection::Y2(y) => Ok(Decoded::Y2(y)),
            };
        }
        let expected = self.model_coordinates();
        if coords.len() != expected.len() {
            return Err(format!(
                "{}/affine takes {} coordinates, got {}",
                self.model_id,
                expected.len(),
                coords.len()
            ));
        }
        Ok(Decoded::Model(coords.to_vec()))
    }

    /// Complete a quotient chart (x-only, y-only, y^2-only) to affine model
    /// coordinates.  `lift` selects among the points a quotient identifies.
    fn complete_model(&self, decoded: Decoded, lift: u32) -> Result<Vec<FieldElement>, String> {
        match decoded {
            Decoded::Model(coords) => Ok(coords),
            Decoded::Y(y) | Decoded::Y2(y) => {
                let y = match decoded_is_squared(&self.chart_id) {
                    true => {
                        let roots = sqrt(&y).ok_or("y^2 is not a square")?;
                        if lift & 1 == 0 {
                            roots
                        } else {
                            roots.neg()
                        }
                    }
                    false => y,
                };
                if self.model_id != "edwards" {
                    return Err(format!(
                        "{} cannot be completed from y alone",
                        self.model_id
                    ));
                }
                let c = self
                    .params
                    .get("c")
                    .cloned()
                    .ok_or("Edwards model needs c")?;
                let d = self
                    .params
                    .get("d")
                    .cloned()
                    .ok_or("Edwards model needs d")?;
                let one = self.one();
                let y2 = y.mul(&y);
                let c2 = c.mul(&c);
                let num = y2.sub(&c2);
                let den = c2.mul(&d).mul(&y2).sub(&one);
                let x2 = num.mul(&den.inv().ok_or("Edwards yz denominator is zero")?);
                let mut x = sqrt(&x2).ok_or("Edwards yz: x^2 is not a square")?;
                if (lift >> 1) & 1 == 1 {
                    x = x.neg();
                }
                Ok(vec![x, y])
            }
            Decoded::X(x) => {
                // Only x-only short-Weierstrass and Montgomery charts exist;
                // both have the identity toweierstrass map, so the short x is
                // an affine function of the model x.
                let xp = self.bridge.short_x(&x);
                Ok(vec![xp])
            }
        }
    }

    /// Decode a point in the selected chart to short-Weierstrass `(x, y)`.
    /// For x-only charts the ordinate is the canonical lift selected by
    /// `lift`.
    pub fn decode_short(
        &self,
        coords: &[FieldElement],
        lift: u32,
    ) -> Result<(FieldElement, FieldElement), String> {
        let decoded = self.decode_model(coords)?;
        if let Decoded::X(x) = decoded {
            let xp = self.bridge.short_x(&x);
            let rhs = xp
                .mul(&xp)
                .mul(&xp)
                .add(&self.bridge.short_a.mul(&xp))
                .add(&self.bridge.short_b);
            let mut y = sqrt(&rhs).ok_or("x is not on the short-Weierstrass curve")?;
            if lift & 1 == 1 {
                y = y.neg();
            }
            return Ok((xp, y));
        }
        let model_coords = self.complete_model(decoded, lift)?;
        self.model
            .to_short_weierstrass(&self.bridge, &model_coords)
            .map_err(|e| format!("to short Weierstrass: {e}"))
    }

    /// Encode a short-Weierstrass point in the selected chart.
    pub fn encode_short(
        &self,
        x: &FieldElement,
        y: &FieldElement,
    ) -> Result<Vec<FieldElement>, String> {
        let model_coords = self
            .model
            .from_short_weierstrass(&self.bridge, x, y)
            .map_err(|e| format!("from short Weierstrass: {e}"))?;
        let Some(chart) = &self.chart else {
            return Ok(model_coords);
        };
        let encoded = if chart.id.starts_with("jintersect/") {
            chart
                .encode_jintersection(
                    &self.modulus,
                    &self.params,
                    &model_coords[0],
                    &model_coords[1],
                    &model_coords[2],
                    &self.one(),
                )
                .map_err(|e| format!("{}: {e}", chart.id))?
        } else {
            chart
                .encode_affine(
                    &self.modulus,
                    &self.params,
                    &model_coords[0],
                    &model_coords[1],
                    &self.one(),
                )
                .map_err(|e| format!("{}: {e}", chart.id))?
        };
        chart
            .variables
            .iter()
            .map(|name| {
                encoded
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("{} did not produce {name}", chart.id))
            })
            .collect()
    }

    /// Decode chart coordinates into the generic field of the index-calculus
    /// curve.
    pub fn decode_point<F: PrimeField>(
        &self,
        field: &F,
        coords: &[FieldElement],
        lift: u32,
    ) -> Result<Pt<F::E>, String> {
        let (x, y) = self.decode_short(coords, lift)?;
        Ok(Pt {
            x: field.embed_big(&x.value)?,
            y: field.embed_big(&y.value)?,
        })
    }

    /// Encode an index-calculus point in the selected chart.
    pub fn encode_point<F: PrimeField>(
        &self,
        field: &F,
        point: &Pt<F::E>,
    ) -> Result<Vec<FieldElement>, String> {
        let x = FieldElement::new(field.to_big(point.x), self.modulus.clone());
        let y = FieldElement::new(field.to_big(point.y), self.modulus.clone());
        self.encode_short(&x, &y)
    }

    /// A JSON record suitable for an `ic prime` report.
    pub fn describe(&self) -> Value {
        let (a, b) = self.short_coefficients();
        let model_equation: Vec<&str> = self
            .model
            .source
            .text
            .lines()
            .filter(|line| line.starts_with("satisfying "))
            .collect();
        let chart_map: Vec<String> = self
            .chart
            .as_ref()
            .map(|chart| {
                chart
                    .relations
                    .iter()
                    .filter(|relation| {
                        relation.starts_with("x =")
                            || relation.starts_with("y =")
                            || relation.starts_with("s =")
                            || relation.starts_with("c =")
                            || relation.starts_with("d =")
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        json!({
            "model": self.model_id,
            "chart": self.chart_id,
            "chart_variables": self.chart.as_ref().map(|c| c.variables.clone()),
            "chart_affine_map": chart_map,
            "model_equations": model_equation,
            "parameters": self
                .params
                .iter()
                .map(|(name, value)| (name.clone(), value.value.to_string()))
                .collect::<std::collections::BTreeMap<_, _>>(),
            "short_weierstrass": {
                "a": a.to_string(),
                "b": b.to_string(),
                "equation": format!("y^2 = x^3 + {a}*x + {b}"),
            },
            "efd_model_source": self.model.source.url,
            "efd_chart_source": self.chart.as_ref().map(|c| c.page_url.clone()),
        })
    }
}

fn decoded_is_squared(chart_id: &str) -> bool {
    chart_id == "edwards/yzsquared"
}

/// The model's own coordinates and parameter names, for callers that need to
/// build a `Bindings` before constructing [`EfdRepresentation`].
pub fn model_parameters(model_id: &str) -> Result<Vec<String>, String> {
    catalogue()
        .model(model_id)
        .map(|model| model.parameter_names())
        .ok_or_else(|| format!("unknown EFD model {model_id}"))
}

/// All chart ids belonging to a model, plus `affine`.
pub fn chart_ids(model_id: &str) -> Vec<String> {
    let mut out = vec!["affine".to_owned()];
    out.extend(
        catalogue()
            .representations
            .iter()
            .filter(|rep| rep.id.starts_with(&format!("{model_id}/")))
            .map(|rep| rep.id.clone()),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::prime_small_x::{measure, Options, SmallPrime};

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
                v.insert("b".into(), fe(2));
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
                v.insert("a".into(), fe(1));
                v.insert("d".into(), fe(6));
            }
            "hessian" => {
                let y = fe(3);
                let num = y.mul(&y).mul(&y).add(&fe(2));
                let den = fe(3).mul(&y);
                v.insert("d".into(), num.mul(&den.inv().unwrap()));
            }
            "jintersect" => {
                v.insert("a".into(), fe(2));
            }
            "jquartic" | "2dik" | "3dik" => {
                v.insert("a".into(), fe(2));
            }
            other => unreachable!("unknown model {other}"),
        }
        v
    }

    fn short_points(repr: &EfdRepresentation, want: usize) -> Vec<(FieldElement, FieldElement)> {
        let mut out = Vec::new();
        for x in 0..P {
            let x = fe(x as i64);
            let rhs = x
                .mul(&x)
                .mul(&x)
                .add(&repr.bridge.short_a.mul(&x))
                .add(&repr.bridge.short_b);
            if let Some(y) = sqrt(&rhs) {
                out.push((x, y));
                if out.len() == want {
                    break;
                }
            }
        }
        out
    }

    fn round_trip(repr: &EfdRepresentation, x: &FieldElement, y: &FieldElement) -> Option<()> {
        let coords = repr.encode_short(x, y).ok()?;
        let (x2, y2) = repr.decode_short(&coords, 0).ok()?;
        match repr.chart_id.as_str() {
            "shortw/xz" | "montgom/xz" => {
                if x2 == *x {
                    Some(())
                } else {
                    None
                }
            }
            "edwards/yz" | "edwards/yzsquared" => {
                let coords2 = repr.encode_short(&x2, &y2).ok()?;
                let back = repr.decode_short(&coords2, 0).ok()?;
                if back.0 == x2 && back.1 == y2 {
                    Some(())
                } else {
                    None
                }
            }
            _ => {
                if x2 == *x && y2 == *y {
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    #[test]
    fn every_model_and_chart_round_trips_its_points() {
        let catalogue = catalogue();
        assert_eq!(catalogue.models.len(), 10);
        for model in &catalogue.models {
            let params = params_for(&model.id);
            for chart in chart_ids(&model.id) {
                let repr = EfdRepresentation::new(&model.id, &chart, &BigUint::from(P), &params)
                    .unwrap_or_else(|e| panic!("{}/{chart}: {e}", model.id));
                let mut checked = 0;
                for (x, y) in short_points(&repr, 12) {
                    if round_trip(&repr, &x, &y).is_some() {
                        checked += 1;
                    }
                }
                assert!(
                    checked > 0,
                    "{}/{} had no non-exceptional point",
                    model.id,
                    chart
                );
            }
        }
    }

    #[test]
    fn edwards_yz_lifts_select_both_x_roots() {
        let mut params = params_for("edwards");
        params.insert("r".into(), fe(8));
        let repr = EfdRepresentation::new("edwards", "yz", &BigUint::from(P), &params).unwrap();
        let mut seen = std::collections::BTreeSet::new();
        for lift in 0..4 {
            let (x, y) = repr.decode_short(&[fe(628), fe(1)], lift).unwrap();
            seen.insert((x.value.to_string(), y.value.to_string()));
        }
        // The y-only chart identifies x and -x; the lift index selects them.
        assert_eq!(seen.len(), 2);
    }

    #[test]
    fn montgomery_chart_feeds_the_index_calculus_engine() {
        let params = params_for("montgom");
        let repr = EfdRepresentation::new("montgom", "xz", &BigUint::from(P), &params).unwrap();
        let field = SmallPrime::new(P);
        let curve = repr
            .curve(
                field,
                "EFD Montgomery test".into(),
                BigUint::from(269u64),
                true,
                4,
            )
            .unwrap();

        // The published affine generator (604,211) in the xz chart is X/Z=604.
        let generator = repr
            .decode_point(&curve.field, &[fe(604), fe(1)], 0)
            .unwrap();
        assert!(curve.on_curve(&mut curve.field.clone(), &generator));

        let report = measure(
            &curve,
            &Options {
                size: 6,
                trials: 100,
                controls: 2,
                seed: 1,
            },
        )
        .unwrap();
        assert_eq!(report.controls_recovered, report.controls);
        assert!(report.distinct_sums > 0);

        let secret = BigUint::from(137u64);
        let target = curve
            .mul_big(&mut curve.field.clone(), &generator, &secret)
            .unwrap();
        let encoded = repr.encode_point(&curve.field, &target).unwrap();
        let decoded = repr.decode_point(&curve.field, &encoded, 0).unwrap();
        assert_eq!(decoded, target);
    }
}
