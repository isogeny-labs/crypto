//! Static public algebra only, after the exporter's sealed-cell admission.
//! The source reconstruction is mandatory even when a graph exceeds a cap.
//! This view does not read witnesses, outcomes or native decision streams.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Write};

use serde_json::{json, Value};

use crate::cryptanalysis::f4_fp::Poly;
use crate::cryptanalysis::groebner_f4::{cmp_monomial, Ordering};
use crate::cryptanalysis::residual_walk::is_prime_u64;

const SCHEMA: &str = "prime-chain-graph/v1";
const ENCODING: &str = "odd-prime-affine-s3-chain/v1";
const MAX_TERMS: usize = 4096;
const MAX_MONOMIALS: usize = 4096;
const MAX_POINTS: usize = 128;
const MAX_BYTES: usize = 8 * 1024 * 1024;

pub(super) fn capabilities() -> Value {
    json!({"schema":SCHEMA,"encoding":ENCODING,"families":["prime"],"m":[3,15],
        "static_inputs_only":true,"prefix_examples":false,"native_solving_required":false,
        "native_prime_modulus_limit_exclusive":"4294967296",
        "wide_prime_static_inputs_supported":true,"exact_decimal_field_values":true,
        "roundtrip_reconstruction_required":true,"max_terms":MAX_TERMS,
        "max_monomials":MAX_MONOMIALS,"max_signed_points":MAX_POINTS,"max_graph_bytes":MAX_BYTES,
        "oversize_policy":"exclude complete graph with restricted reason; never drop terms",
        "claim_limits":["affine infinity branches are not covered","no native trace replay or action labels","no exact first/last fall degree certificate"]})
}

fn array<'a>(value: &'a Value, name: &str) -> Result<&'a [Value], String> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| format!("graph {name} must be an array"))
}

fn uint(value: &Value, name: &str) -> Result<u32, String> {
    value
        .as_u64()
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| format!("graph {name} must be a u32 integer"))
}

fn decimal(value: &Value, name: &str) -> Result<u64, String> {
    let s = value
        .as_str()
        .ok_or_else(|| format!("graph {name} must be a decimal string"))?;
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) || (s.len() > 1 && s.starts_with('0'))
    {
        return Err(format!(
            "graph {name} is not a canonical unsigned decimal string"
        ));
    }
    s.parse()
        .map_err(|_| format!("graph {name} exceeds exact u64 range"))
}

fn field(value: &Value, p: u64, name: &str) -> Result<u64, String> {
    let value = decimal(value, name)?;
    if value >= p {
        return Err(format!("graph {name} is outside Fp"));
    }
    Ok(value)
}

fn point(value: &Value, p: u64) -> Result<(u64, u64), String> {
    Ok((
        field(&value["x"], p, "point.x")?,
        field(&value["y"], p, "point.y")?,
    ))
}

fn point_json(point: (u64, u64)) -> Value {
    json!({"x":point.0.to_string(),"y":point.1.to_string()})
}

fn increasing<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn exponents(value: &Value, variables: usize) -> Result<Vec<u32>, String> {
    let entries = array(value, "exponents")?;
    if entries.len() != variables {
        return Err("graph exponent dimension differs".into());
    }
    let result = entries
        .iter()
        .map(|v| uint(v, "exponent"))
        .collect::<Result<Vec<_>, _>>()?;
    // The current verified encoding has degree at most32. Check before the
    // native comparator, whose total-degree sum otherwise assumes no overflow.
    if result
        .iter()
        .try_fold(0u32, |sum, &v| sum.checked_add(v))
        .is_none_or(|degree| degree > 32)
    {
        return Err("graph exponent total degree exceeds encoding bound".into());
    }
    Ok(result)
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct System {
    m: usize,
    p: u64,
    a: u64,
    b: u64,
    target: (u64, u64),
    domains: Vec<Option<Vec<u64>>>,
    points: Vec<(u64, u64)>,
    polynomials: Vec<Poly>,
}

impl System {
    fn variables(&self) -> usize {
        2 * self.m - 2
    }

    fn check_points(&self) -> Result<(), String> {
        let mul = |a: u64, b: u64| ((u128::from(a) * u128::from(b)) % u128::from(self.p)) as u64;
        if (4u128 * u128::from(mul(mul(self.a, self.a), self.a))
            + 27u128 * u128::from(mul(self.b, self.b)))
        .is_multiple_of(u128::from(self.p))
        {
            return Err("graph curve is singular".into());
        }
        let on_curve = |(x, y): (u64, u64)| {
            x < self.p
                && y < self.p
                && mul(y, y)
                    == ((u128::from(mul(mul(x, x), x))
                        + u128::from(mul(self.a, x))
                        + u128::from(self.b))
                        % u128::from(self.p)) as u64
        };
        if !on_curve(self.target)
            || self.points.is_empty()
            || self.points.iter().any(|&p| !on_curve(p))
        {
            return Err("graph target or domain point is off curve".into());
        }
        let unique: BTreeSet<_> = self.points.iter().copied().collect();
        if unique.len() != self.points.len()
            || self
                .points
                .iter()
                .any(|&(x, y)| !unique.contains(&(x, if y == 0 { 0 } else { self.p - y })))
        {
            return Err("graph signed points are duplicated or not negation closed".into());
        }
        let xs: Vec<_> = self
            .points
            .iter()
            .map(|p| p.0)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        if self.domains[0].as_ref() != Some(&xs) {
            return Err("graph point abscissae differ from finite domain".into());
        }
        Ok(())
    }

    fn source(input: &Value) -> Result<Self, String> {
        let m = uint(&input["m"], "m")? as usize;
        if !(3..=15).contains(&m) {
            return Err("graph arity must be3..15".into());
        }
        let p = decimal(&input["modulus"], "modulus")?;
        if !(5..1u64 << 62).contains(&p) || !is_prime_u64(p) {
            return Err("graph modulus is not a supported odd prime".into());
        }
        let variables = 2 * m - 2;
        if uint(&input["n_vars"], "n_vars")? as usize != variables {
            return Err("graph source variable count differs".into());
        }
        let mut domains = Vec::new();
        for domain in array(&input["domains"], "source domains")? {
            domains.push(if domain.is_null() {
                None
            } else {
                Some(
                    array(domain, "source finite domain")?
                        .iter()
                        .map(|v| {
                            // serde_json's exact u64 is mandatory; no f64 conversion.
                            v.as_u64().filter(|&x| x < p).ok_or_else(|| {
                                "source domain must contain exact in-field u64 integers".to_owned()
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            });
        }
        check_domains(&domains, m)?;
        let points = array(&input["signed_factor_base"], "signed factor base")?
            .iter()
            .map(|v| point(v, p))
            .collect::<Result<Vec<_>, _>>()?;
        let mut polynomials = Vec::new();
        for row in array(&input["polynomials"], "polynomials")? {
            let mut polynomial = Vec::new();
            let mut seen = BTreeSet::new();
            for term in array(row, "polynomial")? {
                let e = exponents(&term["exponents"], variables)?;
                let c = field(&term["coefficient"], p, "coefficient")?;
                if c == 0 || !seen.insert(e.clone()) {
                    return Err("graph source has zero or duplicate terms".into());
                }
                polynomial.push((e, c));
            }
            if polynomial.windows(2).any(|w| {
                cmp_monomial(&w[0].0, &w[1].0, Ordering::Grevlex) != std::cmp::Ordering::Greater
            }) {
                return Err("graph source monomials are not in canonical order".into());
            }
            polynomials.push(polynomial);
        }
        if polynomials.len() != 2 * m - 1 {
            return Err("graph source equation count differs".into());
        }
        let system = Self {
            m,
            p,
            a: field(&input["curve"]["a"], p, "a")?,
            b: field(&input["curve"]["b"], p, "b")?,
            target: point(&input["target"], p)?,
            domains,
            points,
            polynomials,
        };
        system.check_points()?;
        Ok(system)
    }
}

fn check_domains(domains: &[Option<Vec<u64>>], m: usize) -> Result<(), String> {
    if domains.len() != 2 * m - 2 {
        return Err("graph domain dimension differs".into());
    }
    let finite = domains[0]
        .as_ref()
        .ok_or("graph leaf domain is not finite")?;
    if finite.is_empty()
        || finite.len() > 32
        || !increasing(finite)
        || domains[..m].iter().any(|d| d.as_ref() != Some(finite))
        || domains[m..].iter().any(Option::is_some)
    {
        return Err("graph leaf or auxiliary domain differs from encoding".into());
    }
    Ok(())
}

fn canonical_public(public: &Value, base: &Value, p: u64) -> Result<(), String> {
    if public["family"] != "prime" {
        return Err("prime graph on nonprime family".into());
    }
    for name in ["modulus", "characteristic"] {
        if decimal(&public["field"][name], name)? != p {
            return Err("graph public field differs".into());
        }
    }
    for name in ["a", "b"] {
        field(&public["curve"][name], p, name)?;
    }
    for name in ["group_order", "subgroup_order", "cofactor"] {
        decimal(&public["curve"][name], name)?;
    }
    for name in ["generator", "target"] {
        point(&public[name], p)?;
    }
    for row in array(&base["points"], "base points")? {
        point(row, p)?;
    }
    for orbit in array(&base["orbits"], "base orbits")? {
        for row in array(&orbit["points"], "orbit points")? {
            point(row, p)?;
        }
    }
    Ok(())
}

fn graph(system: &System) -> Value {
    let m = system.m;
    let n = system.variables();
    let finite = system.domains[0].as_ref().expect("checked finite domain");
    let monomials: Vec<Vec<u32>> = system
        .polynomials
        .iter()
        .flat_map(|p| p.iter().map(|t| t.0.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let ids: BTreeMap<_, _> = monomials
        .iter()
        .enumerate()
        .map(|(id, e)| (e.clone(), id))
        .collect();
    let variables: Vec<_> = (0..n).map(|id|json!({"id":id,"name":if id<m{format!("x{id}")}else{format!("u{}",id-m)},"role":if id<m{"factor_base_leaf"}else{"unrestricted_affine_auxiliary"},"domain":usize::from(id>=m)})).collect();
    let mut equations = Vec::new();
    for id in 0..m - 1 {
        let arguments = if id == 0 {
            vec![
                json!({"variable":0}),
                json!({"variable":1}),
                json!({"variable":m}),
            ]
        } else if id == m - 2 {
            vec![
                json!({"variable":2*m-3}),
                json!({"variable":m-1}),
                json!({"target_x":true}),
            ]
        } else {
            vec![
                json!({"variable":m+id-1}),
                json!({"variable":id+1}),
                json!({"variable":m+id}),
            ]
        };
        equations.push(json!({"id":id,"kind":"chain_s3","arguments":arguments}));
    }
    for leaf in 0..m {
        equations.push(json!({"id":m-1+leaf,"kind":"leaf_membership","variable":leaf}));
    }
    let mut terms = Vec::new();
    for (equation, polynomial) in system.polynomials.iter().enumerate() {
        let mut row: Vec<_> = polynomial.iter().map(|(e, c)| (ids[e], *c)).collect();
        row.sort_unstable();
        terms.extend(row.into_iter().map(|(monomial,c)|json!({"equation":equation,"monomial":monomial,"coefficient":c.to_string()})));
    }
    let mut powers = Vec::new();
    for (monomial, e) in monomials.iter().enumerate() {
        for (variable, &exponent) in e.iter().enumerate().filter(|(_, e)| **e > 0) {
            powers.push(json!({"monomial":monomial,"variable":variable,"exponent":exponent}));
        }
    }
    let point_ids: BTreeMap<_, _> = system
        .points
        .iter()
        .copied()
        .enumerate()
        .map(|(id, p)| (p, id))
        .collect();
    let point_nodes: Vec<_> = system
        .points
        .iter()
        .enumerate()
        .map(|(id, &p)| json!({"id":id,"x":p.0.to_string(),"y":p.1.to_string()}))
        .collect();
    let incidences: Vec<_> = system.points.iter().enumerate().map(|(point,p)|json!({"domain":0,"value":finite.binary_search(&p.0).expect("checked x incidence"),"point":point})).collect();
    let negation: Vec<_> = system
        .points
        .iter()
        .map(|&(x, y)| point_ids[&(x, if y == 0 { 0 } else { system.p - y })])
        .collect();
    let identity: Vec<_> = (0..system.points.len()).collect();
    let orbits: Vec<_> = identity
        .iter()
        .map(|&id| json!({"points":[id],"closed":true,"stabilizer_size":1}))
        .collect();
    json!({"schema":SCHEMA,"encoding":ENCODING,"pdp_domain":"cyclic-subgroup-factor-base",
        "arity":m,"variable_count":n,"monomial_order":"degrevlex","affine_infinity_branches_covered":false,
        "field":{"modulus":system.p.to_string(),"characteristic":system.p.to_string(),"degree":1},
        "curve":{"a":system.a.to_string(),"b":system.b.to_string()},"target":point_json(system.target),
        "variables":variables,"domains":[{"id":0,"kind":"finite_abscissae","values":finite.iter().map(ToString::to_string).collect::<Vec<_>>()},{"id":1,"kind":"all_Fp"}],
        "equations":equations,"monomials":monomials.iter().enumerate().map(|(id,e)|json!({"id":id,"exponents":e})).collect::<Vec<_>>(),
        "equation_monomial_edges":terms,"monomial_variable_edges":powers,"points":point_nodes,"x_point_incidence":incidences,
        "symmetry":{"negation_action":negation,"frobenius_action":identity,"frobenius_power":1,"orbits":orbits}})
}

/// Decode nodes and edges independently. This validates graph structure and
/// recovers polynomial rows; input reconstruction supplies the S3 semantics.
fn decode(value: &Value) -> Result<System, String> {
    if value["schema"] != SCHEMA
        || value["encoding"] != ENCODING
        || value["pdp_domain"] != "cyclic-subgroup-factor-base"
        || value["monomial_order"] != "degrevlex"
        || value["affine_infinity_branches_covered"] != false
    {
        return Err("graph encoding tags differ".into());
    }
    let m = uint(&value["arity"], "arity")? as usize;
    if !(3..=15).contains(&m) {
        return Err("graph arity outside3..15".into());
    }
    let n = 2 * m - 2;
    if uint(&value["variable_count"], "variable count")? as usize != n {
        return Err("graph variable count differs".into());
    }
    let p = decimal(&value["field"]["modulus"], "modulus")?;
    if !(5..1u64 << 62).contains(&p)
        || !is_prime_u64(p)
        || decimal(&value["field"]["characteristic"], "characteristic")? != p
        || value["field"]["degree"] != 1
    {
        return Err("graph field differs".into());
    }
    let domain_nodes = array(&value["domains"], "domains")?;
    if domain_nodes.len() != 2
        || domain_nodes[0]["id"] != 0
        || domain_nodes[1]["id"] != 1
        || domain_nodes[0]["kind"] != "finite_abscissae"
        || domain_nodes[1]["kind"] != "all_Fp"
        || domain_nodes[1].get("values").is_some()
    {
        return Err("graph shared domains differ".into());
    }
    let finite = array(&domain_nodes[0]["values"], "finite values")?
        .iter()
        .map(|v| field(v, p, "domain value"))
        .collect::<Result<Vec<_>, _>>()?;
    let mut domains = Vec::new();
    let variables = array(&value["variables"], "variables")?;
    if variables.len() != n {
        return Err("graph variable nodes differ".into());
    }
    for (id, node) in variables.iter().enumerate() {
        let leaf = id < m;
        let name = if leaf {
            format!("x{id}")
        } else {
            format!("u{}", id - m)
        };
        if uint(&node["id"], "variable id")? as usize != id
            || node["name"] != name
            || node["role"]
                != if leaf {
                    "factor_base_leaf"
                } else {
                    "unrestricted_affine_auxiliary"
                }
            || uint(&node["domain"], "domain id")? as usize != usize::from(!leaf)
        {
            return Err("graph variable role/domain differs".into());
        }
        domains.push(leaf.then(|| finite.clone()));
    }
    check_domains(&domains, m)?;
    let equations = array(&value["equations"], "equations")?;
    if equations.len() != 2 * m - 1 {
        return Err("graph equation nodes differ".into());
    }
    for (id, node) in equations.iter().enumerate() {
        if uint(&node["id"], "equation id")? as usize != id {
            return Err("graph equation id differs".into());
        }
        if id >= m - 1 {
            if node["kind"] != "leaf_membership"
                || uint(&node["variable"], "membership owner")? as usize != id - (m - 1)
            {
                return Err("graph membership incidence differs".into());
            }
        } else {
            if node["kind"] != "chain_s3" {
                return Err("graph chain kind differs".into());
            }
            let args = array(&node["arguments"], "chain arguments")?;
            if args.len() != 3 {
                return Err("graph chain argument count differs".into());
            }
            let expected = if id == 0 {
                [Some(0), Some(1), Some(m)]
            } else if id + 1 == m - 1 {
                [Some(n - 1), Some(m - 1), None]
            } else {
                [Some(m + id - 1), Some(id + 1), Some(m + id)]
            };
            for (arg, expected) in args.iter().zip(expected) {
                let fields = arg
                    .as_object()
                    .ok_or("graph chain argument is not an object")?;
                if fields.len() != 1 {
                    return Err("graph chain argument is ambiguous".into());
                }
                match expected {
                    Some(expected)
                        if uint(&arg["variable"], "chain variable")? as usize == expected => {}
                    None if arg["target_x"] == true => {}
                    _ => return Err("graph ordered chain incidence differs".into()),
                }
            }
        }
    }
    let mono_nodes = array(&value["monomials"], "monomials")?;
    if mono_nodes.len() > MAX_MONOMIALS {
        return Err("graph monomial cap exceeded".into());
    }
    let mut monomials = Vec::new();
    for (id, node) in mono_nodes.iter().enumerate() {
        if uint(&node["id"], "monomial id")? as usize != id {
            return Err("graph monomial id differs".into());
        }
        monomials.push(exponents(&node["exponents"], n)?);
    }
    if !increasing(&monomials) {
        return Err("graph monomials are duplicated or not lexicographically ordered".into());
    }
    let mut recovered_exponents = vec![vec![0u32; n]; monomials.len()];
    let mut last_power = None;
    for edge in array(&value["monomial_variable_edges"], "power edges")? {
        let monomial = uint(&edge["monomial"], "power monomial")? as usize;
        let variable = uint(&edge["variable"], "power variable")? as usize;
        let exponent = uint(&edge["exponent"], "power exponent")?;
        if monomial >= monomials.len()
            || variable >= n
            || exponent == 0
            || last_power.is_some_and(|old| old >= (monomial, variable))
        {
            return Err("graph power edge is invalid or duplicated".into());
        }
        last_power = Some((monomial, variable));
        recovered_exponents[monomial][variable] = exponent;
    }
    if recovered_exponents != monomials {
        return Err("graph power edges do not recover exponent vectors".into());
    }
    let term_edges = array(&value["equation_monomial_edges"], "term edges")?;
    if term_edges.len() > MAX_TERMS {
        return Err("graph term cap exceeded".into());
    }
    let mut polynomials = vec![Vec::new(); equations.len()];
    let mut used = BTreeSet::new();
    let mut last_term = None;
    for edge in term_edges {
        let equation = uint(&edge["equation"], "term equation")? as usize;
        let monomial = uint(&edge["monomial"], "term monomial")? as usize;
        let coefficient = field(&edge["coefficient"], p, "coefficient")?;
        if equation >= equations.len()
            || monomial >= monomials.len()
            || coefficient == 0
            || last_term.is_some_and(|old| old >= (equation, monomial))
        {
            return Err("graph term edge is invalid or duplicated".into());
        }
        last_term = Some((equation, monomial));
        used.insert(monomial);
        polynomials[equation].push((monomials[monomial].clone(), coefficient));
    }
    if used.len() != monomials.len() {
        return Err("graph contains unused monomials".into());
    }
    for row in &mut polynomials {
        row.sort_by(|a, b| cmp_monomial(&b.0, &a.0, Ordering::Grevlex));
    }
    let point_nodes = array(&value["points"], "points")?;
    if point_nodes.len() > MAX_POINTS {
        return Err("graph point cap exceeded".into());
    }
    let mut points = Vec::new();
    for (id, node) in point_nodes.iter().enumerate() {
        if uint(&node["id"], "point id")? as usize != id {
            return Err("graph point id differs".into());
        }
        points.push(point(node, p)?);
    }
    let incidence = array(&value["x_point_incidence"], "point incidence")?;
    if incidence.len() != points.len() {
        return Err("graph point incidence is incomplete".into());
    }
    for (id, edge) in incidence.iter().enumerate() {
        let x_id = uint(&edge["value"], "abscissa id")? as usize;
        if edge["domain"] != 0
            || uint(&edge["point"], "incidence point")? as usize != id
            || finite.get(x_id).copied() != Some(points[id].0)
        {
            return Err("graph point incidence differs".into());
        }
    }
    let system = System {
        m,
        p,
        a: field(&value["curve"]["a"], p, "a")?,
        b: field(&value["curve"]["b"], p, "b")?,
        target: point(&value["target"], p)?,
        domains,
        points,
        polynomials,
    };
    system.check_points()?;
    let symmetry = &value["symmetry"];
    let negation = array(&symmetry["negation_action"], "negation action")?;
    let frobenius = array(&symmetry["frobenius_action"], "Frobenius action")?;
    let orbits = array(&symmetry["orbits"], "orbits")?;
    if symmetry["frobenius_power"] != 1
        || [negation.len(), frobenius.len(), orbits.len()]
            .iter()
            .any(|&len| len != system.points.len())
    {
        return Err("graph symmetry dimensions differ".into());
    }
    for (id, &(x, y)) in system.points.iter().enumerate() {
        let destination = uint(&negation[id], "negation destination")? as usize;
        if system.points.get(destination).copied() != Some((x, if y == 0 { 0 } else { p - y }))
            || uint(&frobenius[id], "Frobenius destination")? as usize != id
            || orbits[id]["points"] != json!([id])
            || orbits[id]["closed"] != true
            || orbits[id]["stabilizer_size"] != 1
        {
            return Err("graph symmetry action differs from rational prime-field points".into());
        }
    }
    Ok(system)
}

struct SizeCounter {
    bytes: usize,
    limit: usize,
}
impl Write for SizeCounter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes) {
            return Err(io::Error::other("graph byte cap"));
        }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// None means only the explicit graph resource cap; invalid inputs are errors.
pub(super) fn project(
    public: &Value,
    base: &Value,
    input: &Value,
) -> Result<Option<Value>, String> {
    project_with_caps(public, base, input, MAX_TERMS, MAX_BYTES)
}

fn project_with_caps(
    public: &Value,
    base: &Value,
    input: &Value,
    terms_cap: usize,
    bytes_cap: usize,
) -> Result<Option<Value>, String> {
    super::super::solvers::verify_prime_f4_chain_input(public, base, input)?;
    let source = System::source(input)?;
    canonical_public(public, base, source.p)?;
    let terms = source
        .polynomials
        .iter()
        .try_fold(0usize, |n, row| n.checked_add(row.len()))
        .ok_or("graph term count overflow")?;
    if terms > terms_cap || source.points.len() > MAX_POINTS {
        return Ok(None);
    }
    let result = graph(&source);
    if array(&result["monomials"], "monomials")?.len() > MAX_MONOMIALS {
        return Ok(None);
    }
    let mut counter = SizeCounter {
        bytes: 0,
        limit: bytes_cap,
    };
    if serde_json::to_writer(&mut counter, &result).is_err() {
        return Ok(None);
    }
    if decode(&result)? != source {
        return Err("graph inverse differs from reconstructed public input".into());
    }
    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml_dataset::{curves, solvers, types::CaseConfig};

    fn check_equation_values_independently(system: &System) {
        use num_bigint::BigUint;
        let p = BigUint::from(system.p);
        let sub = |a: BigUint, b: BigUint| (a + &p - b % &p) % &p;
        let s3 = |x: &BigUint, y: &BigUint, z: &BigUint| {
            let a = BigUint::from(system.a);
            let b = BigUint::from(system.b);
            let positive =
                sub(x.clone(), y.clone()).pow(2) * z.pow(2) + sub(x * y, a.clone()).pow(2);
            let negative =
                BigUint::from(2u32) * ((x + y) * (x * y + a) + BigUint::from(2u32) * &b) * z
                    + BigUint::from(4u32) * b * (x + y);
            sub(positive % &p, negative % &p)
        };
        for mode in 0..3 {
            let values: Vec<BigUint> = (0..system.variables())
                .map(|i| {
                    BigUint::from(match mode {
                        0 => 0,
                        1 => (7 * i as u64 + 1) % system.p,
                        _ => system.p - 1,
                    })
                })
                .collect();
            for (row, polynomial) in system.polynomials.iter().enumerate() {
                let evaluated =
                    polynomial
                        .iter()
                        .fold(BigUint::from(0u32), |sum, (exponents, coefficient)| {
                            let term = exponents.iter().zip(&values).fold(
                                BigUint::from(*coefficient),
                                |term, (&power, value)| {
                                    term * value.modpow(&BigUint::from(power), &p) % &p
                                },
                            );
                            (sum + term) % &p
                        });
                let expected = if row == 0 {
                    s3(&values[0], &values[1], &values[system.m])
                } else if row + 1 == system.m - 1 {
                    s3(
                        &values[2 * system.m - 3],
                        &values[system.m - 1],
                        &BigUint::from(system.target.0),
                    )
                } else if row < system.m - 1 {
                    s3(
                        &values[system.m + row - 1],
                        &values[row + 1],
                        &values[system.m + row],
                    )
                } else {
                    let leaf = row - (system.m - 1);
                    system.domains[leaf]
                        .as_ref()
                        .unwrap()
                        .iter()
                        .fold(BigUint::from(1u32), |product, &x| {
                            product * sub(values[leaf].clone(), BigUint::from(x)) % &p
                        })
                };
                assert_eq!(
                    evaluated, expected,
                    "independent formula row {row}, assignment {mode}"
                );
            }
        }
    }

    fn input(n: u32, m: usize, twist: bool) -> (Value, Value, Value) {
        let config = CaseConfig {
            factor_base_policy: None,
            family: "prime".into(),
            n,
            m,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 2,
            work_limit: 0,
            solver: "f4".into(),
        };
        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        // Zero exposed-event budget obtains retained input without entering
        // native elimination. This is a pure codec fixture, not a plan cell.
        let result = solvers::solve(&config, &case).unwrap();
        assert_eq!(result.outcome["native_backend_invoked"], false);
        let artifact = result
            .artifacts
            .iter()
            .find(|a| a.path == "prime-f4-input.json")
            .unwrap();
        (
            case.public_instance,
            case.factor_base,
            serde_json::from_slice(&artifact.bytes).unwrap(),
        )
    }

    #[test]
    fn graph_roundtrip_preserves_m3_m4_m15_both_twists_and_wide_inputs() {
        for n in [7, 61] {
            for twist in [false, true] {
                for m in [3, 4, 15] {
                    let (public, base, input) = input(n, m, twist);
                    let projected = project(&public, &base, &input).unwrap().unwrap();
                    let roundtrip = decode(&projected).unwrap();
                    assert_eq!(roundtrip, System::source(&input).unwrap());
                    check_equation_values_independently(&roundtrip);
                    assert_eq!(projected["variables"].as_array().unwrap().len(), 2 * m - 2);
                    assert_eq!(projected["equations"].as_array().unwrap().len(), 2 * m - 1);
                    assert_eq!(projected["domains"][1], json!({"id":1,"kind":"all_Fp"}));
                    assert_eq!(roundtrip.points.len(), 2);
                    assert_eq!(roundtrip.points[0].0, roundtrip.points[1].0);
                    assert_ne!(roundtrip.points[0].1, roundtrip.points[1].1);
                    assert_eq!(projected["symmetry"]["negation_action"], json!([1, 0]));
                    assert_eq!(projected["symmetry"]["frobenius_action"], json!([0, 1]));
                    assert_eq!(
                        projected["x_point_incidence"],
                        json!([
                            {"domain":0,"value":0,"point":0},{"domain":0,"value":0,"point":1}
                        ])
                    );
                    let has_constant = roundtrip
                        .polynomials
                        .iter()
                        .flatten()
                        .any(|(exponents, _)| exponents.iter().all(|&e| e == 0));
                    assert_eq!(
                        projected["monomials"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|node| node["exponents"]
                                .as_array()
                                .unwrap()
                                .iter()
                                .all(|v| v == 0)),
                        has_constant
                    );
                    if n == 61 {
                        assert_eq!(projected["field"]["modulus"], "2305843009213693951");
                    }
                }
            }
        }
    }

    #[test]
    fn graph_inverse_rejects_forged_edges_ids_domains_and_actions() {
        let (public, base, input) = input(7, 3, false);
        let original = project(&public, &base, &input).unwrap().unwrap();
        for change in [
            "variable-id",
            "aux-domain",
            "chain-incidence",
            "membership-owner",
            "duplicate-term",
            "zero-term",
            "duplicate-power",
            "missing-power",
            "exponent-overflow",
            "missing-point-incidence",
            "point-x",
            "negation",
            "frobenius",
            "orbit",
            "monomial-id",
            "unused-monomial",
        ] {
            let mut bad = original.clone();
            match change {
                "variable-id" => bad["variables"][0]["id"] = json!(1),
                "aux-domain" => bad["variables"][3]["domain"] = json!(0),
                "chain-incidence" => bad["equations"][0]["arguments"][1]["variable"] = json!(2),
                "membership-owner" => bad["equations"][2]["variable"] = json!(1),
                "duplicate-term" => {
                    let edge = bad["equation_monomial_edges"][0].clone();
                    bad["equation_monomial_edges"]
                        .as_array_mut()
                        .unwrap()
                        .push(edge);
                }
                "zero-term" => bad["equation_monomial_edges"][0]["coefficient"] = json!("0"),
                "duplicate-power" => {
                    let edge = bad["monomial_variable_edges"][0].clone();
                    bad["monomial_variable_edges"]
                        .as_array_mut()
                        .unwrap()
                        .push(edge);
                }
                "missing-power" => {
                    bad["monomial_variable_edges"].as_array_mut().unwrap().pop();
                }
                "exponent-overflow" => {
                    bad["monomials"][0]["exponents"][0] = json!(u64::from(u32::MAX) + 1)
                }
                "missing-point-incidence" => {
                    bad["x_point_incidence"].as_array_mut().unwrap().pop();
                }
                "point-x" => bad["x_point_incidence"][0]["value"] = json!(1),
                "negation" => bad["symmetry"]["negation_action"][0] = json!(0),
                "frobenius" => bad["symmetry"]["frobenius_action"][0] = json!(1),
                "orbit" => bad["symmetry"]["orbits"][0]["points"] = json!([0, 1]),
                "monomial-id" => bad["monomials"][0]["id"] = json!(1),
                "unused-monomial" => {
                    bad["equation_monomial_edges"] = json!([]);
                }
                _ => unreachable!(),
            }
            assert!(decode(&bad).is_err(), "accepted {change}");
        }
    }

    #[test]
    fn graph_inverse_preserves_constants_empty_rows_and_large_decimal_values() {
        let (_, _, input) = input(7, 3, false);
        let mut system = System::source(&input).unwrap();
        // Exercise the inverse independently of the S3 source producer. The
        // decoder handles sparse rows; project separately requires exact S3
        // reconstruction, so this synthetic sparse system is never admitted.
        let p = 2_305_843_009_213_693_951u64;
        system.p = p;
        system.a = 0;
        system.b = 2;
        system.target = (p - 1, 1);
        system.points = vec![(p - 1, 1), (p - 1, p - 1)];
        system.domains = vec![
            Some(vec![p - 1]),
            Some(vec![p - 1]),
            Some(vec![p - 1]),
            None,
        ];
        system.polynomials = vec![
            vec![(vec![0; 4], p - 1)],
            vec![],
            vec![(vec![1, 0, 0, 0], 9_007_199_254_740_993)],
            vec![],
            vec![],
        ];
        let projected = graph(&system);
        let bytes = serde_json::to_vec(&projected).unwrap();
        let reloaded: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decode(&reloaded).unwrap(), system);
        assert_eq!(projected["domains"][0]["values"][0], "2305843009213693950");
        assert_eq!(
            projected["equation_monomial_edges"][0]["coefficient"],
            "2305843009213693950"
        );
        assert_eq!(
            projected["equation_monomial_edges"][1]["coefficient"],
            "9007199254740993"
        );
        assert_eq!(projected["monomials"][0]["exponents"], json!([0, 0, 0, 0]));
        assert!(projected["monomial_variable_edges"]
            .as_array()
            .unwrap()
            .iter()
            .all(|edge| edge["monomial"] != 0));
        for invalid in [
            json!(9_007_199_254_740_993u64),
            json!("09007199254740993"),
            json!("+1"),
            json!(" 1"),
            json!("1e3"),
            json!("1.0"),
            json!("2305843009213693951"),
        ] {
            let mut bad = projected.clone();
            bad["equation_monomial_edges"][0]["coefficient"] = invalid;
            assert!(decode(&bad).is_err());
        }
        // A valid field value changed to a rounded neighbor remains detectable
        // by the inverse comparison with the exact original source system.
        let mut rounded = projected;
        rounded["equation_monomial_edges"][1]["coefficient"] = json!("9007199254740992");
        assert_ne!(decode(&rounded).unwrap(), system);

        system.p = 7;
        system.a = 1;
        system.b = 0;
        system.target = (0, 0);
        system.points = vec![(0, 0)];
        system.domains = vec![Some(vec![0]), Some(vec![0]), Some(vec![0]), None];
        system.polynomials = vec![vec![]; 5];
        let self_negating = graph(&system);
        assert_eq!(decode(&self_negating).unwrap(), system);
        assert_eq!(self_negating["symmetry"]["negation_action"], json!([0]));
    }

    #[test]
    fn graph_projection_ignores_unknown_private_metadata_and_rejects_noncanonical_sources() {
        let (mut public, mut base, mut input) = input(7, 3, false);
        let clean = project(&public, &base, &input).unwrap().unwrap();
        let marker = "PRIVATE_SENTINEL_NOT_A_FEATURE";
        for key in ["field", "curve", "generator", "target", "symmetry"] {
            public[key]["private_scalar"] = json!(marker);
        }
        public["private_witness"] = json!({"seed":marker,"outcome":marker});
        base["points"][0]["log"] = json!(marker);
        base["orbits"][0]["points"][0]["seed"] = json!(marker);
        base["construction_trace"] = json!({"labels":marker});
        input["native_report"] = json!({"outcome":marker,"instructions":marker});
        let dirty = project(&public, &base, &input).unwrap().unwrap();
        assert_eq!(
            serde_json::to_vec(&clean).unwrap(),
            serde_json::to_vec(&dirty).unwrap()
        );
        assert!(!serde_json::to_string(&dirty).unwrap().contains(marker));
        for bad in [json!("00"), json!("+1"), json!(1)] {
            let mut malformed = public.clone();
            malformed["curve"]["group_order"] = bad;
            assert!(project(&malformed, &base, &input).is_err());
        }
        // Semantic nested fields are verified, not recursively copied. An
        // unknown key attached inside a polynomial term may reject admission.
        input["polynomials"][0][0]["private_scalar"] = json!(marker);
        assert!(project(&public, &base, &input).is_err());
    }

    #[test]
    fn graph_caps_exclude_whole_graph_without_hiding_invalid_inputs() {
        let (public, base, mut input) = input(7, 3, false);
        let clean = project(&public, &base, &input).unwrap().unwrap();
        let bytes = serde_json::to_vec(&clean).unwrap().len();
        assert!(project_with_caps(&public, &base, &input, MAX_TERMS, bytes)
            .unwrap()
            .is_some());
        assert!(
            project_with_caps(&public, &base, &input, MAX_TERMS, bytes - 1)
                .unwrap()
                .is_none()
        );
        assert!(project_with_caps(&public, &base, &input, 0, MAX_BYTES)
            .unwrap()
            .is_none());
        input["polynomials"][0][0]["coefficient"] = json!("0");
        assert!(project_with_caps(&public, &base, &input, 0, 0).is_err());
    }
}
