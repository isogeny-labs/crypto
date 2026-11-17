use crate::polynomial::{mulm, s3, Pol};
use serde_json::{json, Value};
#[derive(Clone, Debug)]
pub struct Case {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub g: u64,
    pub zeta: u64,
    pub m: usize,
    pub t: usize,
    pub xr: u64,
    pub numerator: Vec<u64>,
    pub recorded_degrees: Vec<u32>,
    pub recorded_vars: usize,
    pub recorded_equations: usize,
}
fn uint(v: &Value, name: &str) -> Result<u64, String> {
    v.as_u64().ok_or_else(|| format!("missing {name}"))
}
impl Case {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
        let rows = text
            .lines()
            .map(serde_json::from_str::<Value>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        let cases: Vec<_> = rows.iter().filter(|r| r["m"].is_u64()).collect();
        if cases.len() != 1 {
            return Err("expected exactly one saved case".into());
        }
        let r = cases[0];
        let p = uint(&r["p"], "p")?;
        let m = uint(&r["m"], "m")? as usize;
        let t = uint(&r["t"], "t")? as usize;
        if p != 2013265921 || m != 4 || t != 4 || r["kind"] != "kummer" || r["control"] != "tower" {
            return Err("outside frozen m4t4 Kummer polynomial-cover scope".into());
        }
        let numerator = if r["cover"].is_null() {
            vec![0, 1]
        } else {
            if r["cover"]["map"] != "cubic" || r["cover"]["den"] != json!([1]) {
                return Err("unsupported cover".into());
            }
            r["cover"]["num"]
                .as_array()
                .ok_or("missing cover coefficients")?
                .iter()
                .map(|v| uint(v, "cover coefficient"))
                .collect::<Result<Vec<_>, _>>()?
        };
        if numerator.len() != 2 && numerator.len() != 4 {
            return Err("unsupported map degree".into());
        }
        let recorded_degrees = r["input_degrees"]
            .as_array()
            .ok_or("missing recorded degrees")?
            .iter()
            .map(|v| {
                let d = uint(v, "input degree")?;
                u32::try_from(d).map_err(|e| e.to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;
        let c = Self {
            p,
            a: uint(&r["curve"]["a"], "a")?,
            b: uint(&r["curve"]["b"], "b")?,
            g: uint(&r["tower"]["g"], "g")?,
            zeta: uint(&r["tower"]["zeta"], "zeta")?,
            m,
            t,
            xr: uint(&r["x_r"], "x_r")?,
            numerator,
            recorded_degrees,
            recorded_vars: uint(&r["n_vars"], "n_vars")? as usize,
            recorded_equations: uint(&r["n_equations"], "n_equations")? as usize,
        };
        if [c.a, c.b, c.g, c.zeta, c.xr]
            .iter()
            .chain(c.numerator.iter())
            .any(|&x| x >= p)
        {
            return Err("noncanonical field parameter".into());
        }
        Ok(c)
    }
    pub fn original_n(&self) -> usize {
        self.m * self.t + self.m - 2
    }
    pub fn image_poly(&self, i: usize, n: usize) -> Pol {
        let u = Pol::var(n, i * self.t).scale(self.g, self.p);
        self.numerator.iter().rev().fold(Pol::zero(n), |v, &c| {
            v.mul(&u, self.p).add(&Pol::constant(n, c, self.p), self.p)
        })
    }
    pub fn image_value(&self, u: u64) -> u64 {
        let scaled = mulm(self.g, u, self.p);
        self.numerator
            .iter()
            .rev()
            .fold(0, |v, &c| (mulm(v, scaled, self.p) + c) % self.p)
    }
    pub fn canonical_lift(&self, w: &[u64]) -> Vec<u64> {
        assert_eq!(w.len(), self.original_n());
        let mut z = w.to_vec();
        z.extend((0..self.m).map(|i| self.image_value(w[i * self.t])));
        z
    }
    pub fn metadata(&self) -> Value {
        json!({"p":self.p,"curve":{"a":self.a,"b":self.b},"g":self.g,"zeta":self.zeta,"m":self.m,"t":self.t,"x_r":self.xr,"numerator":self.numerator,"denominator":[1]})
    }
}
#[derive(Clone, Debug)]
pub struct System {
    pub n: usize,
    pub equations: Vec<Pol>,
    pub graph: bool,
}
impl System {
    pub fn degrees(&self) -> Vec<u32> {
        self.equations.iter().map(Pol::degree).collect()
    }
    pub fn names(&self) -> Vec<String> {
        (0..self.n).map(|i| format!("v{i}")).collect()
    }
    pub fn json(&self, c: &Case, input_sha: &str) -> Value {
        json!({
            "schema":"cover.polynomial-system/v1","representation":if self.graph {"graph"}else{"direct"},
            "input_sha256":input_sha,"case":c.metadata(),"variables":self.names(),"n_vars":self.n,
            "n_equations":self.equations.len(),"input_degrees":self.degrees(),
            "polynomial_term_counts":self.equations.iter().map(|f|f.terms.len()).collect::<Vec<_>>(),
            "polynomials":self.equations.iter().map(Pol::terms_vec).collect::<Vec<_>>(),
            "storage_order":"ascending lex exponent vectors; consumer must normalize its chosen monomial order",
            "layout":{"tower_count":c.m*c.t,"chain_base":c.m*c.t,"chain_count":c.m-2,
                "image_base":if self.graph {Some(c.original_n())}else{None},"image_count":if self.graph {c.m}else{0}},
            "scope":"construction and exact substitution only; no solver, witness or timing result"
        })
    }
    pub fn msolve_text(&self, p: u64) -> String {
        let names = self.names();
        format!(
            "{}\n{p}\n{}\n",
            names.join(","),
            self.equations
                .iter()
                .map(|f| f.expression(&names))
                .collect::<Vec<_>>()
                .join(",\n")
        )
    }
}
pub fn build(c: &Case, graph: bool) -> System {
    assert!(c.m >= 2 && c.m <= 8 && c.t >= 1 && c.t <= 8 && c.p > 3 && c.p <= u32::MAX as u64);
    let old = c.original_n();
    let n = old + if graph { c.m } else { 0 };
    let p = c.p;
    let images: Vec<_> = (0..c.m)
        .map(|i| {
            if graph {
                Pol::var(n, old + i)
            } else {
                c.image_poly(i, n)
            }
        })
        .collect();
    let target = Pol::constant(n, c.xr, p);
    let z = |i| Pol::var(n, c.m * c.t + i);
    let mut eqs = if c.m == 2 {
        vec![s3(&images[0], &images[1], &target, c.a, c.b, p)]
    } else {
        let mut out = vec![s3(&images[0], &images[1], &z(0), c.a, c.b, p)];
        for k in 1..c.m - 2 {
            out.push(s3(&z(k - 1), &images[k + 1], &z(k), c.a, c.b, p));
        }
        out.push(s3(&z(c.m - 3), &images[c.m - 1], &target, c.a, c.b, p));
        out
    };
    for i in 0..c.m {
        for j in 0..c.t {
            let here = Pol::var(n, i * c.t + j);
            let next = if j + 1 < c.t {
                Pol::var(n, i * c.t + j + 1)
            } else {
                Pol::constant(n, 1, p)
            };
            eqs.push(next.sub(&here.pow(2, p), p));
        }
    }
    if graph {
        for i in 0..c.m {
            eqs.push(images[i].sub(&c.image_poly(i, n), p));
        }
    }
    System {
        n,
        equations: eqs,
        graph,
    }
}
pub fn verify_pair(c: &Case, direct: &System, graph: &System) -> Result<(), String> {
    let old = c.original_n();
    if direct.n != old
        || graph.n != old + c.m
        || graph.equations.len() != direct.equations.len() + c.m
    {
        return Err("layout mismatch".into());
    }
    let mut subst: Vec<_> = (0..old).map(|i| Pol::var(old, i)).collect();
    subst.extend((0..c.m).map(|i| c.image_poly(i, old)));
    for (i, f) in graph.equations.iter().enumerate() {
        let projected = f.substitute(&subst, old, c.p);
        let expected = if i < direct.equations.len() {
            direct.equations[i].clone()
        } else {
            Pol::zero(old)
        };
        if projected != expected {
            return Err(format!("coefficient projection mismatch in equation{i}"));
        }
    }
    Ok(())
}
pub fn verify_saved_dimensions(c: &Case, direct: &System) -> Result<(), String> {
    if direct.n != c.recorded_vars
        || direct.equations.len() != c.recorded_equations
        || direct.degrees() != c.recorded_degrees
    {
        return Err("direct layout/degrees differ from saved producer row".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomial::{addm, s3_scalar};
    const REFERENCE: &str =
        include_str!("../../cover_image_fb_r5_m4t4_20261112/runs/m4t4-none.jsonl");
    const CUBIC: &str = include_str!("../../cover_image_fb_r5_m4t4_20261112/runs/m4t4-cubic.jsonl");
    #[test]
    fn scalar_s3_matches_independent_symmetric_formula() {
        for (p, a, b) in [(17, 2, 2), (97, 3, 11)] {
            let form = s3(&Pol::var(3, 0), &Pol::var(3, 1), &Pol::var(3, 2), a, b, p);
            for x in 0..7 {
                for y in 0..7 {
                    for z in 0..7 {
                        assert_eq!(form.eval(&[x, y, z], p), s3_scalar(x, y, z, a, b, p));
                    }
                }
            }
        }
    }
    #[test]
    fn both_saved_systems_project_coefficient_for_coefficient() {
        for bytes in [REFERENCE.as_bytes(), CUBIC.as_bytes()] {
            let c = Case::parse(bytes).unwrap();
            let direct = build(&c, false);
            let graph = build(&c, true);
            verify_saved_dimensions(&c, &direct).unwrap();
            verify_pair(&c, &direct, &graph).unwrap();
            assert_eq!((direct.n, direct.equations.len()), (18, 19));
            assert_eq!((graph.n, graph.equations.len()), (22, 23));
            assert_eq!(graph.degrees().into_iter().max(), Some(4));
            for seed in 0..12 {
                let w: Vec<_> = (0..direct.n)
                    .map(|i| (seed * 17 + i as u64 * 31 + 3) % c.p)
                    .collect();
                let lifted = c.canonical_lift(&w);
                for (i, f) in direct.equations.iter().enumerate() {
                    assert_eq!(f.eval(&w, c.p), graph.equations[i].eval(&lifted, c.p));
                }
                assert!(graph.equations[direct.equations.len()..]
                    .iter()
                    .all(|f| f.eval(&lifted, c.p) == 0));
                let mut bad = lifted.clone();
                bad[direct.n] = addm(bad[direct.n], 1, c.p);
                assert_eq!(graph.equations[direct.equations.len()].eval(&bad, c.p), 1);
            }
        }
    }
    #[test]
    fn changed_coefficient_fails_projection() {
        let c = Case::parse(CUBIC.as_bytes()).unwrap();
        let direct = build(&c, false);
        let mut graph = build(&c, true);
        graph.equations[0] = graph.equations[0].add(&Pol::constant(graph.n, 1, c.p), c.p);
        assert!(verify_pair(&c, &direct, &graph).is_err());
    }
    #[test]
    fn two_and_three_summand_chain_boundaries() {
        let base = Case::parse(CUBIC.as_bytes()).unwrap();
        for m in [2, 3] {
            let mut c = base.clone();
            c.m = m;
            c.t = 1;
            let direct = build(&c, false);
            let graph = build(&c, true);
            verify_pair(&c, &direct, &graph).unwrap();
            assert_eq!(direct.equations.len(), m - 1 + m);
            assert_eq!(graph.equations.len(), m - 1 + 2 * m);
        }
    }
}
