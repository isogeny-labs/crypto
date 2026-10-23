//! Checked prime-degree Vélu routes over the exact cubic field supplied to IC.

use clap::Args;
use crypto_lib::cryptanalysis::gaudry_cubic::{Curve3, E3, Fp3, Pt3};
use crypto_lib::hash::sha256::sha256;
use isogeny_algos::curve::{self, Curve, Isogeny, Pt};
use isogeny_algos::field::{Field, Rng, Zp};
use isogeny_algos::kernel::velu::{VeluIso, velu_cyclic, velu_from_point};
use isogeny_algos::path::graph::{self, PhiCache};
use isogeny_algos::poly;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Args, Clone)]
pub(super) struct GraphArgs {
    /// Public cubic target JSON, local or a checked store reference; supplies p, c, a, and b.
    #[arg(long)]
    input: Option<std::path::PathBuf>,
    /// Prime characteristic in 7..=101 with p congruent to 1 modulo 3; use with --c, --a, --b.
    #[arg(long)]
    p: Option<u64>,
    /// A non-cube in F_p defining t^3 = c.
    #[arg(long)]
    c: Option<u64>,
    /// Three coefficients of the source curve's a parameter.
    #[arg(long, num_args = 3)]
    a: Vec<u64>,
    /// Three coefficients of the source curve's b parameter.
    #[arg(long, num_args = 3)]
    b: Vec<u64>,
    /// Include supported isogeny degrees through this value (2..=7).
    #[arg(long, default_value_t = 5)]
    max_ell: usize,
    /// Include degree-2 j-neighbours alongside the supported odd degrees.
    #[arg(long)]
    include_degree_two: bool,
    /// Include each j-neighbour's root multiplicity in the modular polynomial.
    #[arg(long)]
    include_multiplicity: bool,
    /// Maximum distinct j-invariants to retain (1..=64).
    #[arg(long, default_value_t = 32)]
    max_nodes: usize,
    /// Deterministic polynomial-root search seed.
    #[arg(long, default_value_t = 1)]
    seed: u64,
}

#[derive(Args, Clone)]
pub(super) struct GenerateArgs {
    /// Public cubic target JSON, local or a checked store reference.
    #[arg(long)]
    input: std::path::PathBuf,
    /// Prime degree for the rational Vélu kernel (2..=31).
    #[arg(long)]
    degree: u64,
    /// Optional target j-invariant as three canonical cubic-field coefficients.
    #[arg(long, num_args = 3)]
    target_j: Option<Vec<u64>>,
    /// Checked cubic j-graph JSON, local or a checked store reference; use with --target-node.
    #[arg(long)]
    graph_file: Option<std::path::PathBuf>,
    /// Numbered destination j-node connected directly to graph root by --degree.
    #[arg(long)]
    target_node: Option<usize>,
    /// Deterministic point-sampling seed.
    #[arg(long, default_value_t = 1)]
    seed: u64,
    /// Bound on rational kernel candidates to sample.
    #[arg(long, default_value_t = 256)]
    max_samples: usize,
    /// Number of consecutive checked isogeny edges to emit (1..=4).
    #[arg(long, default_value_t = 1)]
    max_depth: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublicTarget {
    p: u64,
    c: u64,
    a: [u64; 3],
    b: [u64; 3],
    n: u64,
    gx: [u64; 3],
    gy: [u64; 3],
    qx: [u64; 3],
    qy: [u64; 3],
}

fn graph_source(args: &GraphArgs) -> Result<(u64, u64, [u64; 3], [u64; 3], Option<Value>), String> {
    if let Some(path) = &args.input {
        if args.p.is_some() || args.c.is_some() || !args.a.is_empty() || !args.b.is_empty() {
            return Err("--input cannot be combined with --p, --c, --a, or --b".into());
        }
        let resolved = crypto_lib::store::resolve_path(path)?;
        if std::fs::metadata(&resolved)
            .map_err(|e| e.to_string())?
            .len()
            > 64 * 1024
        {
            return Err("cubic graph input exceeds 64 KiB".into());
        }
        let bytes = std::fs::read(&resolved).map_err(|e| e.to_string())?;
        if bytes.len() > 64 * 1024 {
            return Err("cubic graph input exceeds 64 KiB".into());
        }
        let target: PublicTarget = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        let artifact = json!({
            "reference": path.to_string_lossy(),
            "sha256": hex::encode(sha256(&bytes)),
            "bytes": bytes.len(),
        });
        Ok((target.p, target.c, target.a, target.b, Some(artifact)))
    } else {
        let p = args
            .p
            .ok_or("cubic graph needs --input or --p --c --a --b")?;
        let c = args
            .c
            .ok_or("cubic graph needs --input or --p --c --a --b")?;
        let a = args
            .a
            .clone()
            .try_into()
            .map_err(|_| "cubic graph needs three --a coefficients")?;
        let b = args
            .b
            .clone()
            .try_into()
            .map_err(|_| "cubic graph needs three --b coefficients")?;
        Ok((p, c, a, b, None))
    }
}

fn cubic_order(f: &CubicField, curve: &Curve<E3>) -> u64 {
    let p = f.p;
    let q = (p * p * p) as usize;
    let offset = |v: E3| ((v.0[2] * p + v.0[1]) * p + v.0[0]) as usize;
    let mut roots = vec![0u8; q];
    for z in 0..p {
        for y in 0..p {
            for x in 0..p {
                let e = E3([x, y, z]);
                roots[offset(f.sq(e))] += 1;
            }
        }
    }
    let mut order = 1u64;
    for z in 0..p {
        for y in 0..p {
            for x in 0..p {
                order += u64::from(roots[offset(curve::rhs(f, curve, E3([x, y, z])))]);
            }
        }
    }
    order
}

fn kernel_isogeny(
    f: &CubicField,
    curve: &Curve<E3>,
    kernel: &Pt<E3>,
    degree: u64,
) -> VeluIso<CubicField> {
    if degree == 2 {
        velu_cyclic(f, curve, kernel, degree)
    } else {
        velu_from_point(f, curve, kernel, degree)
    }
}

fn selected_graph_node(
    args: &GenerateArgs,
    input: &PublicTarget,
    input_bytes: &[u8],
    field: &CubicField,
    source: &Curve<E3>,
) -> Result<(E3, Value), String> {
    let path = args.graph_file.as_ref().ok_or("--graph-file is required")?;
    let node_id = args.target_node.ok_or("--target-node is required")?;
    if node_id == 0 {
        return Err("--target-node must select a non-root graph node".into());
    }
    if args.target_j.is_some() || args.max_depth != 1 {
        return Err("--graph-file --target-node requires one hop and cannot use --target-j".into());
    }
    let resolved = crypto_lib::store::resolve_path(path)?;
    if std::fs::metadata(&resolved)
        .map_err(|e| e.to_string())?
        .len()
        > 1024 * 1024
    {
        return Err("cubic graph file exceeds 1 MiB".into());
    }
    let bytes = std::fs::read(&resolved).map_err(|e| e.to_string())?;
    if bytes.len() > 1024 * 1024 {
        return Err("cubic graph file exceeds 1 MiB".into());
    }
    let graph: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if graph["schema_version"].as_u64() != Some(1)
        || graph["graph_kind"].as_str() != Some("cubic-rational-j-neighbours")
        || graph["field_characteristic"].as_u64() != Some(input.p)
        || graph["cubic_nonresidue"].as_u64() != Some(input.c)
        || graph["source_curve"]["a"] != json!(input.a)
        || graph["source_curve"]["b"] != json!(input.b)
    {
        return Err("cubic graph does not match the target field and source curve".into());
    }
    let target_sha = hex::encode(sha256(input_bytes));
    if !graph["input_artifact"].is_null()
        && graph["input_artifact"]["sha256"].as_str() != Some(target_sha.as_str())
    {
        return Err("cubic graph was made from a different target artifact".into());
    }
    let nodes = graph["nodes"]
        .as_array()
        .ok_or("cubic graph has no nodes")?;
    let roots: Vec<&Value> = nodes
        .iter()
        .filter(|node| node["id"].as_u64() == Some(0))
        .collect();
    if roots.len() != 1 {
        return Err("cubic graph must have exactly one root node".into());
    }
    let root = roots[0];
    if root["j"] != json!(curve::jinv(field, source).0) {
        return Err("cubic graph root j does not match the target curve".into());
    }
    let selected: Vec<&Value> = nodes
        .iter()
        .filter(|node| node["id"].as_u64() == Some(node_id as u64))
        .collect();
    if selected.len() != 1 {
        return Err("cubic graph target node is missing or duplicated".into());
    }
    let coords: [u64; 3] = serde_json::from_value(selected[0]["j"].clone())
        .map_err(|_| "cubic graph target j is not a coefficient triple")?;
    if coords.iter().any(|&v| v >= input.p) {
        return Err("cubic graph target j is not canonical".into());
    }
    let edges = graph["edges"]
        .as_array()
        .ok_or("cubic graph has no edges")?;
    if !edges.iter().any(|edge| {
        edge["source"].as_u64() == Some(0)
            && edge["target"].as_u64() == Some(node_id as u64)
            && edge["degree"].as_u64() == Some(args.degree)
    }) {
        return Err("selected graph node has no root edge of the requested degree".into());
    }
    let degree = args.degree as usize;
    if ![2, 3, 5, 7].contains(&degree) || degree + 1 >= input.p as usize {
        return Err("graph-selected route degree is outside the supported graph range".into());
    }
    let cache = PhiCache::new(&Zp::new(input.p), &[degree]).lift(|v| E3([v, 0, 0]));
    let j = E3(coords);
    if cache.get(degree).eval(field, curve::jinv(field, source), j) != field.zero() {
        return Err("selected graph edge failed independent modular-polynomial evaluation".into());
    }
    Ok((
        j,
        json!({
            "reference":path.to_string_lossy(),
            "sha256":hex::encode(sha256(&bytes)),
            "bytes":bytes.len(),
            "target_node":node_id,
            "target_j":coords,
            "degree":args.degree,
            "target_input_sha256":target_sha,
        }),
    ))
}

/// Construct a bounded rational-kernel route in the exact schema accepted by
/// `solve-cubic`. A j-neighbour without a rational kernel may be unreachable
/// by this constructor; the bounded search reports that outcome explicitly.
pub(super) fn generate(args: &GenerateArgs) -> Result<Value, String> {
    if !(2..=31).contains(&args.degree)
        || !prime(args.degree)
        || !(1..=10_000).contains(&args.max_samples)
        || !(1..=4).contains(&args.max_depth)
    {
        return Err(
            "degree must be prime in 2..=31, max-samples in 1..=10000, and max-depth in 1..=4"
                .into(),
        );
    }
    if args.graph_file.is_some() != args.target_node.is_some() {
        return Err("--graph-file and --target-node must be supplied together".into());
    }
    let resolved = crypto_lib::store::resolve_path(&args.input)?;
    if std::fs::metadata(&resolved)
        .map_err(|e| e.to_string())?
        .len()
        > 32 * 1024 * 1024
    {
        return Err("cubic target input exceeds 32 MiB".into());
    }
    let bytes = std::fs::read(&resolved).map_err(|e| e.to_string())?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err("cubic target input exceeds 32 MiB".into());
    }
    let input: PublicTarget = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if !(7..=101).contains(&input.p)
        || !prime(input.p)
        || args.degree == input.p
        || !(3..=2_000_000).contains(&input.n)
        || !prime(input.n)
        || args.degree == input.n
    {
        return Err("invalid cubic field, subgroup, or separable route degree".into());
    }
    let all = [
        &input.a, &input.b, &input.gx, &input.gy, &input.qx, &input.qy,
    ];
    if input.c >= input.p || all.into_iter().flatten().any(|&v| v >= input.p) {
        return Err("cubic target field coefficients must be canonical".into());
    }
    let _checked_field = Fp3::with_cube_nonresidue(input.p, input.c)?;
    let f = CubicField {
        p: input.p,
        c: input.c,
    };
    let curve = Curve::new(E3(input.a), E3(input.b));
    if !curve::is_smooth(&f, &curve) {
        return Err("cubic target source curve is singular".into());
    }
    let g = Pt::Aff(E3(input.gx), E3(input.gy));
    let q = Pt::Aff(E3(input.qx), E3(input.qy));
    if !curve::on_curve(&f, &curve, &g)
        || !curve::on_curve(&f, &curve, &q)
        || curve::pmul(&f, &curve, &g, u128::from(input.n)) != Pt::Inf
        || curve::pmul(&f, &curve, &q, u128::from(input.n)) != Pt::Inf
    {
        return Err("cubic target points fail source subgroup checks".into());
    }
    let (requested_j, graph_selection) = if args.graph_file.is_some() {
        let (j, selection) = selected_graph_node(args, &input, &bytes, &f, &curve)?;
        (Some(j), Some(selection))
    } else {
        let j = match &args.target_j {
            None => None,
            Some(values) if values.len() == 3 && values.iter().all(|&v| v < input.p) => {
                Some(E3([values[0], values[1], values[2]]))
            }
            _ => return Err("target-j must be three canonical field coefficients".into()),
        };
        (j, None)
    };
    let order = cubic_order(&f, &curve);
    if !order.is_multiple_of(args.degree) || !order.is_multiple_of(input.n) {
        return Err(
            "route degree or DLP subgroup order does not divide counted curve order".into(),
        );
    }
    let mut cofactor = order;
    let mut valuation = 0;
    while cofactor.is_multiple_of(args.degree) {
        cofactor /= args.degree;
        valuation += 1;
    }
    let mut rng = Rng::new(args.seed);
    let mut current_curve = curve;
    let mut current_g = g;
    let mut current_q = q;
    let mut current_ref = "root".to_string();
    let mut seen_models = vec![curve];
    let mut nodes = vec![json!({"ref":"root","a":input.a,"b":input.b})];
    let mut edges = Vec::with_capacity(args.max_depth);
    let mut edge_ids = Vec::with_capacity(args.max_depth);
    for depth in 0..args.max_depth {
        let final_hop = depth + 1 == args.max_depth;
        let mut found = None;
        for _ in 0..args.max_samples {
            let sampled = curve::random_point_f(&f, &current_curve, &mut rng);
            let mut kernel = curve::pmul(&f, &current_curve, &sampled, u128::from(cofactor));
            for _ in 1..valuation {
                if kernel == Pt::Inf
                    || curve::pmul(&f, &current_curve, &kernel, u128::from(args.degree)) == Pt::Inf
                {
                    break;
                }
                kernel = curve::pmul(&f, &current_curve, &kernel, u128::from(args.degree));
            }
            if kernel == Pt::Inf
                || curve::pmul(&f, &current_curve, &kernel, u128::from(args.degree)) != Pt::Inf
            {
                continue;
            }
            let iso = kernel_isogeny(&f, &current_curve, &kernel, args.degree);
            if !curve::is_smooth(&f, &iso.cod)
                || (args.max_depth > 1 && seen_models.contains(&iso.cod))
                || (final_hop
                    && requested_j.is_some_and(|target| curve::jinv(&f, &iso.cod) != target))
            {
                continue;
            }
            let mapped_g = iso.eval(&f, &current_g);
            let mapped_q = iso.eval(&f, &current_q);
            if mapped_g == Pt::Inf
                || mapped_q == Pt::Inf
                || !curve::on_curve(&f, &iso.cod, &mapped_g)
                || !curve::on_curve(&f, &iso.cod, &mapped_q)
                || curve::pmul(&f, &iso.cod, &mapped_g, u128::from(input.n)) != Pt::Inf
                || curve::pmul(&f, &iso.cod, &mapped_q, u128::from(input.n)) != Pt::Inf
                || iso.eval(&f, &curve::padd(&f, &current_curve, &current_g, &current_g))
                    != curve::padd(&f, &iso.cod, &mapped_g, &mapped_g)
                || iso.eval(&f, &curve::padd(&f, &current_curve, &current_q, &current_q))
                    != curve::padd(&f, &iso.cod, &mapped_q, &mapped_q)
            {
                continue;
            }
            let Pt::Aff(kx, ky) = kernel else {
                unreachable!()
            };
            found = Some((iso.cod, mapped_g, mapped_q, kx, ky));
            break;
        }
        let (codomain, mapped_g, mapped_q, kx, ky) = found.ok_or_else(|| format!(
            "bounded route search exhausted {} samples at hop {} without a fresh rational degree-{} kernel satisfying the route constraints",
            args.max_samples, depth + 1, args.degree
        ))?;
        let next_ref = if final_hop {
            "destination".to_string()
        } else {
            format!("hop{}", depth + 1)
        };
        let edge_id = if args.max_depth == 1 {
            format!("velu{}", args.degree)
        } else {
            format!("velu{}-step{}", args.degree, depth + 1)
        };
        nodes.push(json!({"ref":next_ref,"a":codomain.a.0,"b":codomain.b.0}));
        edges.push(json!({"id":edge_id,"source_curve_ref":current_ref,"target_curve_ref":next_ref,"degree":args.degree,"kernel_generator":[kx.0,ky.0]}));
        edge_ids.push(edge_id);
        seen_models.push(codomain);
        current_curve = codomain;
        current_g = mapped_g;
        current_q = mapped_q;
        current_ref = next_ref;
    }
    let route_id = if args.max_depth == 1 {
        format!("root-to-velu{}", args.degree)
    } else {
        format!("root-to-velu{}-depth{}", args.degree, args.max_depth)
    };
    let mut report = json!({
        "schema_version":1,
        "field_characteristic":input.p,
        "cubic_nonresidue":input.c,
        "curve_nodes":nodes,
        "edges":edges,
        "routes":[{"id":route_id,"edge_ids":edge_ids}]
    });
    if let Some(selection) = graph_selection {
        report["graph_selection"] = selection;
    }
    Ok(report)
}

/// Enumerate distinct F_{p^3}-rational j-neighbours by modular polynomial.
/// This is a graph of j-invariants; explicit point transport requires a
/// separately certified route of curve models and kernel generators.
pub(super) fn enumerate_j(args: &GraphArgs) -> Result<Value, String> {
    let (p, c, a, b, input_artifact) = graph_source(args)?;
    if !(7..=101).contains(&p)
        || !prime(p)
        || !(2..=7).contains(&args.max_ell)
        || !(1..=64).contains(&args.max_nodes)
    {
        return Err(
            "cubic graph requires prime p in 7..=101, max-ell in 2..=7, max-nodes in 1..=64".into(),
        );
    }
    if c >= p || a.iter().chain(&b).any(|&v| v >= p) {
        return Err("cubic graph field coefficients must be canonical triples".into());
    }
    let _checked_field = Fp3::with_cube_nonresidue(p, c)?;
    let f = CubicField { p, c };
    let source = Curve::new(E3(a), E3(b));
    if !curve::is_smooth(&f, &source) {
        return Err("cubic graph source curve is singular".into());
    }
    let degrees: Vec<usize> = [2, 3, 5, 7]
        .into_iter()
        .filter(|&ell| {
            ell <= args.max_ell && ell + 1 < p as usize && (ell != 2 || args.include_degree_two)
        })
        .collect();
    if degrees.is_empty() {
        return Err("no selected degree below the field characteristic; use --include-degree-two for max-ell 2".into());
    }
    let prime_cache = PhiCache::new(&Zp::new(p), &degrees);
    let cache = prime_cache.lift(|v| E3([v, 0, 0]));
    let mut nodes = vec![curve::jinv(&f, &source)];
    let mut index = HashMap::new();
    index.insert(nodes[0], 0usize);
    let mut edges = Vec::new();
    let mut rng = Rng::new(args.seed);
    let mut expanded = 0usize;
    let mut truncated = false;
    while expanded < nodes.len() {
        let source_id = expanded;
        let j = nodes[source_id];
        let mut neighbors = graph::neighbors(&f, &cache, &degrees, j, &mut rng);
        neighbors.sort_unstable_by_key(|(ell, jn)| (*ell, jn.0));
        neighbors.dedup();
        for (ell, next_j) in neighbors {
            if cache.get(ell).eval(&f, j, next_j) != f.zero() {
                return Err("modular-polynomial neighbour failed independent evaluation".into());
            }
            let target_id = if let Some(&id) = index.get(&next_j) {
                id
            } else {
                if nodes.len() == args.max_nodes {
                    truncated = true;
                    continue;
                }
                let id = nodes.len();
                nodes.push(next_j);
                index.insert(next_j, id);
                id
            };
            let mut edge = json!({"source":source_id,"target":target_id,"degree":ell});
            if args.include_multiplicity {
                let mut polynomial = cache.get(ell).y_poly(&f, j);
                let linear = vec![f.neg(next_j), f.one()];
                let mut multiplicity = 0;
                while polynomial.len() > 1 {
                    let (quotient, remainder) = poly::divrem(&f, &polynomial, &linear);
                    if !remainder.is_empty() {
                        break;
                    }
                    polynomial = quotient;
                    multiplicity += 1;
                }
                if multiplicity == 0 || multiplicity > ell + 1 {
                    return Err("modular-polynomial neighbour has invalid multiplicity".into());
                }
                edge["multiplicity"] = json!(multiplicity);
            }
            edges.push(edge);
        }
        expanded += 1;
    }
    let mut report = json!({
        "schema_version":1,
        "graph_kind":"cubic-rational-j-neighbours",
        "field_characteristic":p,
        "cubic_nonresidue":c,
        "source_curve":{"a":a,"b":b},
        "degrees":degrees,
        "nodes":nodes.iter().enumerate().map(|(id,j)| json!({"id":id,"j":j.0})).collect::<Vec<_>>(),
        "edges":edges,
        "nodes_expanded":expanded,
        "truncated":truncated,
        "seed":args.seed
    });
    if let Some(artifact) = input_artifact {
        report["input_artifact"] = artifact;
    }
    Ok(report)
}

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
    #[serde(default)]
    graph_selection: Option<GraphSelection>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GraphSelection {
    reference: String,
    sha256: String,
    bytes: usize,
    target_node: usize,
    target_j: [u64; 3],
    degree: u64,
    target_input_sha256: String,
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
    input_sha256: Option<&str>,
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
        if !(2..=31).contains(&edge.degree) || !prime(edge.degree) {
            return Err("cubic route degree must be prime in 2..=31".into());
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
        let isogeny = kernel_isogeny(&f, domain, &kernel, edge.degree);
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
    if let Some(selection) = &graph.graph_selection {
        let first_edge = edge_by_id[route.edge_ids[0].as_str()];
        let last_edge = edge_by_id[route.edge_ids.last().unwrap().as_str()];
        let destination = nodes[&last_edge.target_curve_ref.as_str()];
        let canonical_hash =
            |value: &str| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit());
        if selection.reference.is_empty()
            || selection.bytes == 0
            || selection.target_node == 0
            || !canonical_hash(&selection.sha256)
            || !canonical_hash(&selection.target_input_sha256)
            || input_sha256.is_some_and(|digest| digest != selection.target_input_sha256)
            || route.edge_ids.len() != 1
            || first_edge.source_curve_ref != root.reference
            || first_edge.degree != selection.degree
            || curve::jinv(&f, &destination).0 != selection.target_j
        {
            return Err("cubic route graph selection disagrees with the checked route".into());
        }
    }
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
        let isogeny = kernel_isogeny(&f, &domain, &kernel, edge.degree);
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
    fn generated_public_f7_route_is_consumable_and_in_j_graph() {
        let input = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ic/fixtures/cubic_f7_cofactor12_public_target.json"
        ));
        let args = GenerateArgs {
            input: input.to_path_buf(),
            degree: 3,
            target_j: None,
            graph_file: None,
            target_node: None,
            seed: 1,
            max_samples: 256,
            max_depth: 1,
        };
        let graph = generate(&args).unwrap();
        let path = std::env::temp_dir().join(format!(
            "cubic-generated-route-{}-{}.json",
            std::process::id(),
            line!()
        ));
        std::fs::write(&path, serde_json::to_vec(&graph).unwrap()).unwrap();
        let field = Fp3::with_cube_nonresidue(7, 2).unwrap();
        let generator = Pt3::affine(E3([1, 2, 3]), E3([1, 1, 6]));
        let target = Pt3::affine(E3([3, 4, 6]), E3([1, 3, 5]));
        let source = Curve3::new(field, E3([3, 6, 4]), E3([4, 4, 6]), 29, generator);
        let result = transport(&source, target, &path, "root-to-velu3", None).unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(result.curve.mul(&result.curve.g, 5), result.target);
        let f = CubicField { p: 7, c: 2 };
        let destination_j = curve::jinv(&f, &Curve::new(result.curve.a, result.curve.b));
        let j_graph = enumerate_j(&GraphArgs {
            input: None,
            p: Some(7),
            c: Some(2),
            a: vec![3, 6, 4],
            b: vec![4, 4, 6],
            max_ell: 5,
            include_degree_two: false,
            include_multiplicity: false,
            max_nodes: 32,
            seed: 1,
        })
        .unwrap();
        let node = j_graph["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .find(|node| node["j"] == json!(destination_j.0))
            .unwrap();
        let id = node["id"].as_u64().unwrap();
        assert!(
            j_graph["edges"]
                .as_array()
                .unwrap()
                .iter()
                .any(|edge| edge["source"] == 0 && edge["target"] == id && edge["degree"] == 3)
        );
        let mut requested = args;
        requested.target_j = Some(destination_j.0.to_vec());
        assert_eq!(
            generate(&requested).unwrap()["curve_nodes"][1]["a"],
            graph["curve_nodes"][1]["a"]
        );
    }

    #[test]
    fn public_f7_j_graph_matches_exhaustive_field_roots() {
        let args = GraphArgs {
            input: None,
            p: Some(7),
            c: Some(2),
            a: vec![3, 6, 4],
            b: vec![4, 4, 6],
            max_ell: 5,
            include_degree_two: false,
            include_multiplicity: false,
            max_nodes: 32,
            seed: 1,
        };
        let report = enumerate_j(&args).unwrap();
        assert_eq!(report["truncated"], false);
        assert_eq!(report["nodes"].as_array().unwrap().len(), 6);
        assert_eq!(report["edges"].as_array().unwrap().len(), 18);
        let nodes: Vec<E3> = report["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| E3(serde_json::from_value(n["j"].clone()).unwrap()))
            .collect();
        let actual: HashSet<_> = report["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|edge| {
                let source = edge["source"].as_u64().unwrap() as usize;
                let target = edge["target"].as_u64().unwrap() as usize;
                (
                    source,
                    edge["degree"].as_u64().unwrap() as usize,
                    nodes[target],
                )
            })
            .collect();
        let f = CubicField { p: 7, c: 2 };
        let cache = PhiCache::new(&Zp::new(7), &[3, 5]).lift(|v| E3([v, 0, 0]));
        let mut expected = HashSet::new();
        for (source, &j) in nodes.iter().enumerate() {
            for ell in [3, 5] {
                for x2 in 0..7 {
                    for x1 in 0..7 {
                        for x0 in 0..7 {
                            let candidate = E3([x0, x1, x2]);
                            if cache.get(ell).eval(&f, j, candidate) == Fp3::ZERO {
                                expected.insert((source, ell, candidate));
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn degree_two_j_graph_matches_exhaustive_field_roots() {
        let args = GraphArgs {
            input: None,
            p: Some(7),
            c: Some(2),
            a: vec![3, 6, 4],
            b: vec![4, 4, 6],
            max_ell: 2,
            include_degree_two: true,
            include_multiplicity: false,
            max_nodes: 64,
            seed: 1,
        };
        let report = enumerate_j(&args).unwrap();
        assert_eq!(report["degrees"], json!([2]));
        let nodes: Vec<E3> = report["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|node| E3(serde_json::from_value(node["j"].clone()).unwrap()))
            .collect();
        let retained: HashSet<_> = nodes.iter().copied().collect();
        let actual: HashSet<_> = report["edges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|edge| {
                assert_eq!(edge["degree"], 2);
                let source = edge["source"].as_u64().unwrap() as usize;
                let target = edge["target"].as_u64().unwrap() as usize;
                (source, nodes[target])
            })
            .collect();
        let f = CubicField { p: 7, c: 2 };
        let phi = PhiCache::new(&Zp::new(7), &[2]).lift(|v| E3([v, 0, 0]));
        let mut expected = HashSet::new();
        let mut omitted = false;
        for (source, &j) in nodes.iter().enumerate() {
            for x2 in 0..7 {
                for x1 in 0..7 {
                    for x0 in 0..7 {
                        let candidate = E3([x0, x1, x2]);
                        if phi.get(2).eval(&f, j, candidate) == Fp3::ZERO {
                            if retained.contains(&candidate) {
                                expected.insert((source, candidate));
                            } else {
                                omitted = true;
                            }
                        }
                    }
                }
            }
        }
        assert!(!actual.is_empty());
        assert_eq!(actual, expected);
        assert_eq!(report["truncated"], omitted);
    }

    #[test]
    fn degree_two_j_graph_counts_repeated_root() {
        let report = enumerate_j(&GraphArgs {
            input: None,
            p: Some(7),
            c: Some(2),
            a: vec![0, 0, 0],
            b: vec![1, 0, 0],
            max_ell: 2,
            include_degree_two: true,
            include_multiplicity: true,
            max_nodes: 8,
            seed: 1,
        })
        .unwrap();
        assert_eq!(report["nodes"][0]["j"], json!([0, 0, 0]));
        let edge = report["edges"]
            .as_array()
            .unwrap()
            .iter()
            .find(|edge| edge["source"] == 0)
            .unwrap();
        let target = edge["target"].as_u64().unwrap() as usize;
        assert_eq!(report["nodes"][target]["j"], json!([2, 0, 0]));
        assert_eq!(edge["multiplicity"], 3);
    }

    #[test]
    fn generated_degree_two_route_transports_public_scalar() {
        let input = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ic/fixtures/cubic_f7_cofactor12_public_target.json"
        ));
        let graph = generate(&GenerateArgs {
            input: input.to_path_buf(),
            degree: 2,
            target_j: None,
            graph_file: None,
            target_node: None,
            seed: 1,
            max_samples: 256,
            max_depth: 1,
        })
        .unwrap();
        let path = std::env::temp_dir().join(format!(
            "cubic-degree-two-route-{}-{}.json",
            std::process::id(),
            line!()
        ));
        std::fs::write(&path, serde_json::to_vec(&graph).unwrap()).unwrap();
        let field = Fp3::with_cube_nonresidue(7, 2).unwrap();
        let generator = Pt3::affine(E3([1, 2, 3]), E3([1, 1, 6]));
        let target = Pt3::affine(E3([3, 4, 6]), E3([1, 3, 5]));
        let source = Curve3::new(field, E3([3, 6, 4]), E3([4, 4, 6]), 29, generator);
        let mapped = transport(&source, target, &path, "root-to-velu2", None).unwrap();
        std::fs::remove_file(path).unwrap();
        assert_eq!(mapped.curve.mul(&mapped.curve.g, 5), mapped.target);
        assert_eq!(mapped.receipt["edges_verified"], 1);
        let f = CubicField { p: 7, c: 2 };
        let source_j = curve::jinv(&f, &Curve::new(source.a, source.b));
        let destination_j = curve::jinv(&f, &Curve::new(mapped.curve.a, mapped.curve.b));
        let phi = PhiCache::new(&Zp::new(7), &[2]).lift(|v| E3([v, 0, 0]));
        assert_eq!(phi.get(2).eval(&f, source_j, destination_j), Fp3::ZERO);
    }

    #[test]
    fn generated_two_hop_degree_two_route_replays_on_each_edge() {
        let input = Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/ic/fixtures/cubic_f7_cofactor12_public_target.json"
        ));
        let graph = generate(&GenerateArgs {
            input: input.to_path_buf(),
            degree: 2,
            target_j: None,
            graph_file: None,
            target_node: None,
            seed: 1,
            max_samples: 256,
            max_depth: 2,
        })
        .unwrap();
        assert_eq!(graph["curve_nodes"].as_array().unwrap().len(), 3);
        assert_eq!(graph["edges"].as_array().unwrap().len(), 2);
        let path = std::env::temp_dir().join(format!(
            "cubic-degree-two-two-hop-{}-{}.json",
            std::process::id(),
            line!()
        ));
        std::fs::write(&path, serde_json::to_vec(&graph).unwrap()).unwrap();
        let field = Fp3::with_cube_nonresidue(7, 2).unwrap();
        let generator = Pt3::affine(E3([1, 2, 3]), E3([1, 1, 6]));
        let target = Pt3::affine(E3([3, 4, 6]), E3([1, 3, 5]));
        let source = Curve3::new(field, E3([3, 6, 4]), E3([4, 4, 6]), 29, generator);
        let mapped = transport(&source, target, &path, "root-to-velu2-depth2", None).unwrap();
        let mut tampered = graph;
        tampered["edges"][1]["kernel_generator"][0] = json!([0, 0, 0]);
        std::fs::write(&path, serde_json::to_vec(&tampered).unwrap()).unwrap();
        assert!(
            transport(&source, target, &path, "root-to-velu2-depth2", None)
                .err()
                .expect("tampered second edge must fail")
                .contains("invalid kernel")
        );
        std::fs::remove_file(path).unwrap();
        assert_eq!(mapped.receipt["hops"], 2);
        assert_eq!(mapped.receipt["edges_verified"], 2);
        assert_eq!(mapped.curve.mul(&mapped.curve.g, 5), mapped.target);
    }

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
        let result = transport(&source, target, path, "root-to-velu3", None).unwrap();
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
