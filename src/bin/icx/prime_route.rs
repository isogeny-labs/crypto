//! Certified prime-field isogeny route transport for an external IC target.

use super::PrimeArgs;
use crypto_lib::cryptanalysis::isogeny_walk::walk::{self, StartCurve};
use crypto_lib::ecc::{curve::CurveParams, field::FieldElement, point::Point};
use crypto_lib::hash::sha256::sha256;
use isogeny_algos::curve::{self, Curve, Isogeny, Pt};
use isogeny_algos::field::{Field as _, Zp};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::Path;

pub(super) struct Transport {
    pub curve: CurveParams,
    pub generator: Point,
    pub target: Point,
    pub receipt: Value,
}

fn integer(value: &Value, what: &str) -> Result<u64, String> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
        .ok_or_else(|| format!("route {what} is not a nonnegative u64"))
}

fn model(node: &Value, p: u64) -> Result<Curve<u64>, String> {
    let coefficients = node["coefficients_a1_a2_a3_a4_a6"]
        .as_array()
        .ok_or("route node lacks curve coefficients")?;
    let a = integer(coefficients.get(3).ok_or("route node lacks a4")?, "a4")?;
    let b = integer(coefficients.get(4).ok_or("route node lacks a6")?, "a6")?;
    if a >= p || b >= p {
        return Err("route curve coefficients are not canonical".into());
    }
    Ok(Curve::new(a, b))
}

fn pair(values: &Value, p: u64, what: &str) -> Result<(u64, u64), String> {
    let arr = values
        .as_array()
        .ok_or_else(|| format!("route {what} is not a pair"))?;
    let x = integer(arr.first().ok_or(format!("route {what} lacks x"))?, "x")?;
    let y = integer(arr.get(1).ok_or(format!("route {what} lacks y"))?, "y")?;
    if x >= p || y >= p {
        return Err(format!("route {what} coordinates are not canonical"));
    }
    Ok((x, y))
}

fn ec_point(p: u64, point: Pt<u64>) -> Result<Point, String> {
    let Pt::Aff(x, y) = point else {
        return Err("isogeny killed an input subgroup point".into());
    };
    let modulus = BigUint::from(p);
    Ok(Point::Affine {
        x: FieldElement::new(BigUint::from(x), modulus.clone()),
        y: FieldElement::new(BigUint::from(y), modulus),
    })
}

/// Reverify the entire graph record and transport both points along one
/// root-origin route. The original ECDLP scalar is replayed by the caller.
pub(super) fn transport(
    args: &PrimeArgs,
    path: &Path,
    route_id: &str,
) -> Result<Transport, String> {
    let resolved = crypto_lib::store::resolve_path(path)?;
    let size = std::fs::metadata(&resolved)
        .map_err(|e| format!("{}: {e}", resolved.display()))?
        .len();
    if size > 32 * 1024 * 1024 {
        return Err("route file exceeds the 32 MiB verification limit".into());
    }
    let bytes = std::fs::read(&resolved).map_err(|e| format!("{}: {e}", resolved.display()))?;
    let data: Value =
        serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", resolved.display()))?;
    let nodes = data["curve_nodes"]
        .as_array()
        .ok_or("route lacks curve_nodes")?;
    let edges = data["edges"].as_array().ok_or("route lacks edges")?;
    let routes = data["routes"].as_array().ok_or("route lacks routes")?;
    if nodes.is_empty() || nodes.len() > 256 || edges.len() > 1024 || routes.len() > 2048 {
        return Err("route graph exceeds the bounded verification envelope".into());
    }
    for edge in edges {
        let ell = integer(&edge["degree"], "degree")?;
        if !(3..=31).contains(&ell) || !super::prime(ell) {
            return Err("route degrees must be odd primes in 3..=31".into());
        }
    }
    let root = &nodes[0];
    let root_model = model(root, args.p)?;
    if root_model != Curve::new(args.a, args.b) {
        return Err("route root curve differs from the supplied curve".into());
    }
    let order = super::count_curve_order(args.p, args.a, args.b);
    if !order.is_multiple_of(args.n) {
        return Err("supplied subgroup order does not divide the counted curve order".into());
    }
    let cofactor = order / args.n;
    let (rgx, rgy) = pair(&root["generator_G"], args.p, "root generator")?;
    let start = StartCurve {
        name: "external-prime".into(),
        tag: "fp".into(),
        p: BigUint::from(args.p),
        a: BigUint::from(args.a),
        b: BigUint::from(args.b),
        order: BigUint::from(order),
        subgroup_order: BigUint::from(args.n),
        cofactor: BigUint::from(cofactor),
        gx: BigUint::from(rgx),
        gy: BigUint::from(rgy),
    };
    let (nodes_verified, edges_verified) = walk::verify_routes(&data, &start, 2)?;
    let chosen = routes
        .iter()
        .find(|route| route["id"].as_str() == Some(route_id))
        .ok_or_else(|| format!("route id {route_id} is absent"))?;
    let edge_ids = chosen["edge_ids"]
        .as_array()
        .ok_or("route lacks edge_ids")?;
    if edge_ids.is_empty() || edge_ids.len() > 64 {
        return Err("route must have 1..=64 edges".into());
    }
    let by_ref: HashMap<&str, &Value> = nodes
        .iter()
        .filter_map(|n| n["ref"].as_str().map(|r| (r, n)))
        .collect();
    let by_id: HashMap<&str, &Value> = edges
        .iter()
        .filter_map(|e| e["id"].as_str().map(|id| (id, e)))
        .collect();
    let fp = Zp::new(args.p);
    let mut current_ref = root["ref"].as_str().ok_or("route root lacks ref")?;
    let mut current = root_model;
    let mut g = Pt::Aff(args.gx, args.gy);
    let mut q = Pt::Aff(args.qx, args.qy);
    for id in edge_ids {
        let id = id.as_str().ok_or("route edge id is not text")?;
        let edge = by_id
            .get(id)
            .ok_or_else(|| format!("route edge {id} is absent"))?;
        if edge["source_curve_ref"].as_str() != Some(current_ref) {
            return Err(format!(
                "route edge {id} does not start at the current curve"
            ));
        }
        let target_ref = edge["target_curve_ref"]
            .as_str()
            .ok_or("route edge lacks target ref")?;
        let target_node = by_ref
            .get(target_ref)
            .ok_or("route target node is absent")?;
        let target_model = model(target_node, args.p)?;
        let ell = integer(&edge["degree"], "degree")?;
        if ell == args.n {
            return Err("isogeny degree equals the DLP subgroup order".into());
        }
        let coeffs = edge["kernel_polynomial"]
            .as_array()
            .ok_or("route edge lacks kernel polynomial")?
            .iter()
            .map(|v| integer(v, "kernel coefficient"))
            .collect::<Result<Vec<_>, _>>()?;
        if coeffs.iter().any(|&v| v >= args.p) {
            return Err("route kernel coefficients are not canonical".into());
        }
        let iso = isogeny_algos::kernel::kohel::kohel(&fp, &current, &coeffs, ell);
        let recorded_codomain = &edge["velu_codomain_a4_a6"];
        let codomain = Curve::new(
            integer(&recorded_codomain[0], "Vélu a4")?,
            integer(&recorded_codomain[1], "Vélu a6")?,
        );
        if iso.cod != codomain {
            return Err(format!(
                "route edge {id} disagrees with the independent Kohel map"
            ));
        }
        let u2 = integer(&edge["isomorphism_u2"], "isomorphism u2")?;
        if u2 >= args.p {
            return Err("route isomorphism u2 is not canonical".into());
        }
        let mut u = fp.sqrt(u2).ok_or("route isomorphism u2 is not a square")?;
        u = u.min(fp.neg(u));
        if fp.mul(fp.mul(u2, u2), iso.cod.a) != target_model.a
            || fp.mul(fp.mul(fp.mul(u2, u2), u2), iso.cod.b) != target_model.b
        {
            return Err(format!(
                "route edge {id} isomorphism misses its target model"
            ));
        }
        let map = |point: Pt<u64>| -> Pt<u64> {
            match iso.eval(&fp, &point) {
                Pt::Inf => Pt::Inf,
                Pt::Aff(x, y) => Pt::Aff(fp.mul(u2, x), fp.mul(fp.mul(u2, u), y)),
            }
        };
        for point in [g, q] {
            let doubled_before = curve::pmul(&fp, &current, &point, 2);
            let mapped = map(point);
            let doubled_after = curve::pmul(&fp, &target_model, &mapped, 2);
            if map(doubled_before) != doubled_after {
                return Err(format!("route edge {id} failed its homomorphism check"));
            }
        }
        g = map(g);
        q = map(q);
        if g == Pt::Inf
            || q == Pt::Inf
            || !curve::on_curve(&fp, &target_model, &g)
            || !curve::on_curve(&fp, &target_model, &q)
        {
            return Err(format!(
                "route edge {id} produced an invalid subgroup point"
            ));
        }
        current_ref = target_ref;
        current = target_model;
    }
    let generator = ec_point(args.p, g)?;
    let target = ec_point(args.p, q)?;
    let curve = CurveParams {
        name: "transported-prime",
        p: BigUint::from(args.p),
        a: BigUint::from(current.a),
        b: BigUint::from(current.b),
        gx: match g {
            Pt::Aff(x, _) => BigUint::from(x),
            Pt::Inf => unreachable!(),
        },
        gy: match g {
            Pt::Aff(_, y) => BigUint::from(y),
            Pt::Inf => unreachable!(),
        },
        n: BigUint::from(args.n),
        h: cofactor as u32,
    };
    let n = BigUint::from(args.n);
    let a_fe = curve.a_fe();
    if generator.scalar_mul(&n, &a_fe) != Point::Infinity
        || target.scalar_mul(&n, &a_fe) != Point::Infinity
    {
        return Err("transported points failed the original subgroup-order check".into());
    }
    let point_coords = |pt: Pt<u64>| -> Vec<u64> {
        match pt {
            Pt::Aff(x, y) => vec![x, y],
            Pt::Inf => unreachable!(),
        }
    };
    let receipt = json!({
        "route_id": route_id,
        "route_file_sha256": hex::encode(sha256(&bytes)),
        "nodes_verified": nodes_verified,
        "edges_verified": edges_verified,
        "hops": edge_ids.len(),
        "counted_curve_order": order,
        "destination": {"a": current.a, "b": current.b, "generator": point_coords(g), "target": point_coords(q)},
        "homomorphism_checks": 2 * edge_ids.len(),
        "subgroup_verified": true,
    });
    Ok(Transport {
        curve,
        generator,
        target,
        receipt,
    })
}
