//! Checked odd-degree Vélu routes over the exact cubic field supplied to IC.

use crypto_lib::cryptanalysis::gaudry_cubic::{Curve3, Fp3, Pt3, E3};
use crypto_lib::hash::sha256::sha256;
use isogeny_algos::curve::{self, Curve, Isogeny, Pt};
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::kernel::velu::velu_from_point;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// A counter-free field adapter. `Fp3` uses `Cell` to charge IC arithmetic;
/// isogeny verification instead uses this `Send + Sync` field implementation.
#[derive(Clone)]
struct CubicField {
    p: u64,
    c: u64,
}

impl Field for CubicField {
    type E = E3;

    fn zero(&self) -> E3 {
        Fp3::ZERO
    }
    fn one(&self) -> E3 {
        Fp3::ONE
    }
    fn add(&self, a: E3, b: E3) -> E3 {
        E3(std::array::from_fn(|i| (a.0[i] + b.0[i]) % self.p))
    }
    fn sub(&self, a: E3, b: E3) -> E3 {
        E3(std::array::from_fn(|i| (a.0[i] + self.p - b.0[i]) % self.p))
    }
    fn neg(&self, a: E3) -> E3 {
        E3(std::array::from_fn(|i| (self.p - a.0[i]) % self.p))
    }
    fn mul(&self, a: E3, b: E3) -> E3 {
        let mut product = [0u64; 5];
        for i in 0..3 {
            for j in 0..3 {
                product[i + j] += a.0[i] * b.0[j];
            }
        }
        E3(std::array::from_fn(|i| {
            (product[i] + if i < 2 { self.c * product[i + 3] } else { 0 }) % self.p
        }))
    }
    fn inv(&self, a: E3) -> E3 {
        assert_ne!(a, Fp3::ZERO, "cannot invert zero in cubic route");
        self.pow(a, self.size() - 2)
    }
    fn from_u64(&self, n: u64) -> E3 {
        E3([n % self.p, 0, 0])
    }
    fn char(&self) -> u64 {
        self.p
    }
    fn size(&self) -> u128 {
        u128::from(self.p).pow(3)
    }
    fn random(&self, rng: &mut Rng) -> E3 {
        E3(std::array::from_fn(|_| rng.below(self.p)))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Graph {
    schema_version: u64,
    field_characteristic: u64,
    cubic_nonresidue: u64,
    curve_nodes: Vec<Node>,
    edges: Vec<Edge>,
    routes: Vec<Route>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Node {
    #[serde(rename = "ref")]
    reference: String,
    a: [u64; 3],
    b: [u64; 3],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Edge {
    id: String,
    source_curve_ref: String,
    target_curve_ref: String,
    degree: u64,
    kernel_generator: [[u64; 3]; 2],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Route {
    id: String,
    edge_ids: Vec<String>,
}

pub(super) struct Transport {
    pub curve: Curve3,
    pub target: Pt3,
    pub receipt: Value,
}

fn canonical(x: [u64; 3], p: u64) -> Result<E3, String> {
    if x.iter().any(|&coefficient| coefficient >= p) {
        Err("cubic route has a noncanonical field coefficient".into())
    } else {
        Ok(E3(x))
    }
}

fn model(node: &Node, f: &CubicField) -> Result<Curve<E3>, String> {
    let curve = Curve::new(canonical(node.a, f.p)?, canonical(node.b, f.p)?);
    if !curve::is_smooth(f, &curve) {
        return Err(format!("cubic route node {} is singular", node.reference));
    }
    Ok(curve)
}

fn from_pt(point: Pt<E3>) -> Result<Pt3, String> {
    match point {
        Pt::Inf => Err("cubic isogeny killed an input subgroup point".into()),
        Pt::Aff(x, y) => Ok(Pt3::affine(x, y)),
    }
}

fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    (2..)
        .take_while(|&d| d * d <= n)
        .all(|d| !n.is_multiple_of(d))
}

/// Verify every node and edge, then transport G and Q along a root-origin
/// route. The caller checks the recovered scalar again on its source curve.
pub(super) fn transport(
    source: &Curve3,
    target: Pt3,
    path: &Path,
    route_id: &str,
) -> Result<Transport, String> {
    let resolved = crypto_lib::store::resolve_path(path)?;
    let size = std::fs::metadata(&resolved)
        .map_err(|e| format!("{}: {e}", resolved.display()))?
        .len();
    if size > 32 * 1024 * 1024 {
        return Err("cubic route exceeds the 32 MiB verification limit".into());
    }
    let bytes = std::fs::read(&resolved).map_err(|e| format!("{}: {e}", resolved.display()))?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("cubic route exceeds the 32 MiB verification limit".into());
    }
    let graph: Graph =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", resolved.display()))?;
    if graph.schema_version != 1
        || graph.field_characteristic != source.field.p
        || graph.cubic_nonresidue != source.field.c
    {
        return Err("cubic route field identity or schema differs from the supplied input".into());
    }
    if graph.curve_nodes.is_empty()
        || graph.curve_nodes.len() > 256
        || graph.edges.len() > 1024
        || graph.routes.len() > 2048
    {
        return Err("cubic route graph exceeds the bounded verification envelope".into());
    }
    let f = CubicField {
        p: source.field.p,
        c: source.field.c,
    };
    let mut nodes = HashMap::new();
    for node in &graph.curve_nodes {
        if node.reference.is_empty()
            || nodes
                .insert(node.reference.as_str(), model(node, &f)?)
                .is_some()
        {
            return Err("cubic route has an empty or duplicate node reference".into());
        }
    }
    let root = &graph.curve_nodes[0];
    if nodes[&root.reference.as_str()] != Curve::new(source.a, source.b) {
        return Err("cubic route root differs from the supplied curve".into());
    }
    let mut edge_by_id = HashMap::new();
    for edge in &graph.edges {
        if edge.id.is_empty() || edge_by_id.insert(edge.id.as_str(), edge).is_some() {
            return Err("cubic route has an empty or duplicate edge id".into());
        }
        if !(3..=31).contains(&edge.degree) || edge.degree.is_multiple_of(2) || !prime(edge.degree)
        {
            return Err("cubic route degree must be an odd prime in 3..=31".into());
        }
        if edge.degree == source.n {
            return Err("cubic route degree equals the DLP subgroup order".into());
        }
        let domain = nodes
            .get(edge.source_curve_ref.as_str())
            .ok_or("cubic route edge has an unknown source")?;
        let codomain = nodes
            .get(edge.target_curve_ref.as_str())
            .ok_or("cubic route edge has an unknown target")?;
        let kernel = Pt::Aff(
            canonical(edge.kernel_generator[0], f.p)?,
            canonical(edge.kernel_generator[1], f.p)?,
        );
        if !curve::on_curve(&f, domain, &kernel)
            || curve::pmul(&f, domain, &kernel, u128::from(edge.degree)) != Pt::Inf
        {
            return Err(format!(
                "cubic route edge {} has an invalid kernel",
                edge.id
            ));
        }
        let isogeny = velu_from_point(&f, domain, &kernel, edge.degree);
        if &isogeny.cod != codomain {
            return Err(format!("cubic route edge {} has a wrong codomain", edge.id));
        }
    }
    let mut route_ids = HashSet::new();
    for route in &graph.routes {
        if route.id.is_empty() || !route_ids.insert(route.id.as_str()) {
            return Err("cubic route has an empty or duplicate route id".into());
        }
        if route.edge_ids.is_empty() || route.edge_ids.len() > 64 {
            return Err(format!("cubic route {} must have 1..=64 edges", route.id));
        }
        let mut cursor = root.reference.as_str();
        for edge_id in &route.edge_ids {
            let edge = edge_by_id
                .get(edge_id.as_str())
                .ok_or_else(|| format!("cubic route edge {edge_id} is absent"))?;
            if edge.source_curve_ref != cursor {
                return Err(format!("cubic route {} is disconnected", route.id));
            }
            cursor = edge.target_curve_ref.as_str();
        }
    }
    let route = graph
        .routes
        .iter()
        .find(|route| route.id == route_id)
        .ok_or_else(|| format!("cubic route id {route_id} is absent"))?;
    let mut current_ref = root.reference.as_str();
    let mut g = Pt::Aff(source.g.x, source.g.y);
    let mut q = Pt::Aff(target.x, target.y);
    for edge_id in &route.edge_ids {
        let edge = edge_by_id
            .get(edge_id.as_str())
            .ok_or_else(|| format!("cubic route edge {edge_id} is absent"))?;
        if edge.source_curve_ref != current_ref {
            return Err(format!("cubic route edge {edge_id} is disconnected"));
        }
        let domain = nodes[&current_ref];
        let codomain = nodes[&edge.target_curve_ref.as_str()];
        let kernel = Pt::Aff(E3(edge.kernel_generator[0]), E3(edge.kernel_generator[1]));
        let isogeny = velu_from_point(&f, &domain, &kernel, edge.degree);
        for point in [g, q] {
            let before = curve::padd(&f, &domain, &point, &point);
            let after = isogeny.eval(&f, &point);
            if isogeny.eval(&f, &before) != curve::padd(&f, &codomain, &after, &after) {
                return Err(format!(
                    "cubic route edge {edge_id} failed its point-map check"
                ));
            }
        }
        g = isogeny.eval(&f, &g);
        q = isogeny.eval(&f, &q);
        if g == Pt::Inf
            || q == Pt::Inf
            || !curve::on_curve(&f, &codomain, &g)
            || !curve::on_curve(&f, &codomain, &q)
            || curve::pmul(&f, &codomain, &g, u128::from(source.n)) != Pt::Inf
            || curve::pmul(&f, &codomain, &q, u128::from(source.n)) != Pt::Inf
        {
            return Err(format!("cubic route edge {edge_id} lost the DLP subgroup"));
        }
        current_ref = edge.target_curve_ref.as_str();
    }
    let destination = nodes[&current_ref];
    let generator = from_pt(g)?;
    let target = from_pt(q)?;
    let curve = Curve3::new(
        source.field.clone(),
        destination.a,
        destination.b,
        source.n,
        generator,
    );
    Ok(Transport {
        curve,
        target,
        receipt: json!({
            "route_id": route_id,
            "route_file_sha256": hex::encode(sha256(&bytes)),
            "nodes_verified": graph.curve_nodes.len(),
            "edges_verified": graph.edges.len(),
            "hops": route.edge_ids.len(),
            "destination_ref": current_ref,
            "destination_curve": {"a": destination.a.0, "b": destination.b.0},
            "transported_generator": [generator.x.0, generator.y.0],
            "transported_target": [target.x.0, target.y.0],
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_f7_route_transports_the_same_scalar() {
        let field = Fp3::with_cube_nonresidue(7, 2).unwrap();
        let generator = Pt3::affine(E3([1, 2, 3]), E3([1, 1, 6]));
        let target = Pt3::affine(E3([3, 4, 6]), E3([1, 3, 5]));
        let source = Curve3::new(field, E3([3, 6, 4]), E3([4, 4, 6]), 29, generator);
        let path = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ic/fixtures/cubic_f7_cofactor12_isogeny_route.json"
        ));
        let result = transport(&source, target, path, "root-to-velu3").unwrap();
        assert_eq!(result.curve.a, E3([3, 5, 6]));
        assert_eq!(result.curve.b, E3([4, 4, 6]));
        assert_eq!(result.curve.g, Pt3::affine(E3([0, 5, 2]), E3([2, 6, 4])));
        assert_eq!(result.target, Pt3::affine(E3([5, 2, 0]), E3([1, 3, 3])));
        assert_eq!(source.mul(&generator, 5), target);
        assert_eq!(result.curve.mul(&result.curve.g, 5), result.target);
        assert_eq!(result.receipt["nodes_verified"], 2);
        assert_eq!(result.receipt["edges_verified"], 1);
    }
}
