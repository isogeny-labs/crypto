//! Arm (b) and its negative control (b0): carry a floor-curve instance by
//! explicit 2-isogenies (Vélu) and charge every native operation the carrier
//! spends finding the path, as an attacker without the volcano would.
//!
//! Heights count up from the floor (floor 0, crater `depth`). A floor curve
//! of an ℓ = 2 volcano has one rational 2-torsion point; every other vertex
//! has three. Vélu's map with kernel `(x₀, 0)` is
//!
//! ```text
//!   v = 3x₀² + a,  E′ : y² = x³ + (a − 5v)x + (b − 7x₀v),
//!   φ(x, y) = (x + v/(x − x₀),  y·(1 − v/(x − x₀)²)),
//! ```
//!
//! and the dual's kernel on `E′` has abscissa `−2x₀` (both other 2-torsion
//! points map there), so the backtracking edge is known for free.
//!
//! **Charged path finding.** Height 0: the one rational root of the cubic
//! from `gcd(x^p − x, f)`. Height `h ≥ 1`: the two non-backtracking kernels
//! are the roots of `x² + x_b x + x_b² + a` (one square root). Candidate `c₁`
//! descends iff a non-backtracking walk of `h − 1` steps from its codomain
//! lands on a floor curve (a Legendre symbol: the floor's quadratic factor is
//! irreducible); an ascending candidate cannot reach the floor in fewer than
//! `h + 1` steps (Fouquet–Morain, ANTS 2002). Mapping `G` is target
//! independent setup; mapping `Q` is the online part. Both are charged.
//!
//! **Uncharged verification.** Every image is on its codomain, has order `r`
//! and respects `φ(G + 2G) = φ(G) + φ(2G)`; each vertex's height is
//! recomputed by breadth-first search to the nearest floor.

use std::collections::{HashSet, VecDeque};

use serde_json::{json, Value};

use crate::fp::{all_roots, poly, rational_part, Curve, Fp, Point, Tally};

/// One Vélu step: the codomain and the constants its point map needs.
#[derive(Clone, Copy, Debug)]
pub struct Velu {
    pub x0: u64,
    pub v: u64,
    pub cod: Curve,
}

pub fn velu(f: &Fp, c: &Curve, x0: u64, t: &mut Tally) -> Velu {
    let x02 = f.mul(x0, x0, t);
    let v = f.add(f.small(3, x02), c.a);
    let w = f.mul(x0, v, t);
    let cod = Curve {
        a: f.sub(c.a, f.small(5, v)),
        b: f.sub(c.b, f.small(7, w)),
    };
    Velu { x0, v, cod }
}

/// `φ(P)`: one inversion and four multiplications.
pub fn map(f: &Fp, s: &Velu, pt: Point, t: &mut Tally) -> Point {
    let (x, y) = pt?;
    if x == s.x0 {
        return None;
    }
    let inv = f.inv(f.sub(x, s.x0), t);
    let xn = f.add(x, f.mul(s.v, inv, t));
    let inv2 = f.mul(inv, inv, t);
    let yn = f.mul(y, f.sub(1, f.mul(s.v, inv2, t)), t);
    Some((xn, yn))
}

fn back_of(f: &Fp, x0: u64) -> u64 {
    f.neg(f.small(2, x0))
}

/// The two roots other than `xb` of `x³ + ax + b`, or `None` when the curve
/// is on the floor (the quadratic factor is irreducible).
fn other_roots(f: &Fp, c: &Curve, xb: u64, t: &mut Tally) -> Option<(u64, u64)> {
    let xb2 = f.mul(xb, xb, t);
    let disc = f.neg(f.add(f.small(3, xb2), f.small(4, c.a)));
    let s = f.sqrt(disc, t)?;
    let inv2 = f.p.div_ceil(2);
    let nxb = f.neg(xb);
    Some((f.mul(f.add(nxb, s), inv2, t), f.mul(f.sub(nxb, s), inv2, t)))
}

fn is_floor(f: &Fp, c: &Curve, xb: u64, t: &mut Tally) -> bool {
    let xb2 = f.mul(xb, xb, t);
    let disc = f.neg(f.add(f.small(3, xb2), f.small(4, c.a)));
    disc != 0 && !f.is_square(disc, t)
}

/// Whether the edge with kernel `c1` from a height-`h` vertex descends.
fn descends(f: &Fp, w: &Curve, c1: u64, h: u32, t: &mut Tally) -> (bool, Velu) {
    let e1 = velu(f, w, c1, t);
    let mut cur = e1.cod;
    let mut back = back_of(f, c1);
    for _ in 0..h.saturating_sub(1) {
        match other_roots(f, &cur, back, t) {
            None => return (true, e1), // reached the floor early: only a descent can
            Some((r, _)) => {
                cur = velu(f, &cur, r, t).cod;
                back = back_of(f, r);
            }
        }
    }
    (is_floor(f, &cur, back, t), e1)
}

/// The result of carrying one instance along a path.
#[derive(Clone, Debug)]
pub struct Carried {
    pub curve: Curve,
    pub g: (u64, u64),
    pub path_j: Vec<u64>,
    /// Path finding and mapping `G`: target independent.
    pub setup: Tally,
    /// Mapping `Q`: the per-target part.
    pub online: Tally,
    pub steps: Vec<Velu>,
}

fn floor_root(f: &Fp, c: &Curve, t: &mut Tally) -> Result<u64, String> {
    let g = rational_part(f, c, t);
    if poly::deg(&g) != 1 {
        return Err(format!(
            "not a floor curve: {} rational 2-torsion roots",
            poly::deg(&g)
        ));
    }
    Ok(f.neg(g[0]))
}

fn take(f: &Fp, s: Velu, g: Point, out: &mut Carried) -> Point {
    let img = map(f, &s, g, &mut out.setup);
    // The target's image costs the same operations; charged to the online tally.
    let _ = map(f, &s, g, &mut out.online);
    out.curve = s.cod;
    out.path_j.push(s.cod.j(f));
    out.steps.push(s);
    img
}

/// Ascend `depth` steps from a floor curve to the crater.
pub fn ascend(f: &Fp, c0: Curve, g0: (u64, u64), depth: u32) -> Result<Carried, String> {
    let mut out = Carried {
        curve: c0,
        g: g0,
        path_j: vec![c0.j(f)],
        setup: Tally::default(),
        online: Tally::default(),
        steps: Vec::new(),
    };
    let mut t = Tally::default();
    let x0 = floor_root(f, &c0, &mut t)?;
    let s = velu(f, &c0, x0, &mut t);
    out.setup.absorb(&t);
    let mut g = take(f, s, Some(g0), &mut out);
    let mut back = back_of(f, x0);
    for h in 1..depth {
        let mut t = Tally::default();
        let w = out.curve;
        let (c1, c2) = other_roots(f, &w, back, &mut t)
            .ok_or_else(|| format!("height {h} vertex has a single rational root"))?;
        let (down, e1) = descends(f, &w, c1, h, &mut t);
        let s = if down { velu(f, &w, c2, &mut t) } else { e1 };
        out.setup.absorb(&t);
        g = take(f, s, g, &mut out);
        back = back_of(f, s.x0);
    }
    out.g = g.ok_or("the generator mapped to the identity")?;
    Ok(out)
}

/// Floor → the parent → the parent's other floor child (the sibling).
pub fn sibling(f: &Fp, c0: Curve, g0: (u64, u64)) -> Result<Carried, String> {
    let mut out = Carried {
        curve: c0,
        g: g0,
        path_j: vec![c0.j(f)],
        setup: Tally::default(),
        online: Tally::default(),
        steps: Vec::new(),
    };
    let mut t = Tally::default();
    let x0 = floor_root(f, &c0, &mut t)?;
    let s = velu(f, &c0, x0, &mut t);
    out.setup.absorb(&t);
    let g = take(f, s, Some(g0), &mut out);
    let back = back_of(f, x0);
    let mut t = Tally::default();
    let w = out.curve;
    let (c1, c2) = other_roots(f, &w, back, &mut t).ok_or("the parent is a floor curve")?;
    let (down, e1) = descends(f, &w, c1, 1, &mut t);
    let s = if down { e1 } else { velu(f, &w, c2, &mut t) };
    out.setup.absorb(&t);
    let g = take(f, s, g, &mut out);
    out.g = g.ok_or("the generator mapped to the identity")?;
    Ok(out)
}

/// Distance to the nearest floor vertex, by breadth-first search over
/// 2-isogenies keyed by `j` (uncharged); `limit + 1` when none is within `limit`.
pub fn height(f: &Fp, c: Curve, limit: u32) -> u32 {
    let mut scratch = Tally::default();
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    seen.insert(c.j(f));
    queue.push_back((c, 0u32));
    while let Some((e, d)) = queue.pop_front() {
        let roots = all_roots(f, &e, &mut scratch);
        if roots.len() == 1 {
            return d;
        }
        if d == limit {
            continue;
        }
        for r in roots {
            let s = velu(f, &e, r, &mut scratch);
            if seen.insert(s.cod.j(f)) {
                queue.push_back((s.cod, d + 1));
            }
        }
    }
    limit + 1
}

/// Uncharged checks of a carried instance: on-curve images of order `r` and
/// additivity at every step, and the endpoint heights along the path.
pub fn verify(
    f: &Fp,
    c0: Curve,
    g0: (u64, u64),
    r: u64,
    carried: &Carried,
) -> Result<Vec<u32>, String> {
    let mut cur = c0;
    let mut g: Point = Some(g0);
    let mut scratch = Tally::default();
    let mut heights = vec![height(f, c0, 8)];
    for s in &carried.steps {
        let g2 = cur.add(f, g, g);
        let g3 = cur.add(f, g, g2);
        let (ig, ig2, ig3) = (
            map(f, s, g, &mut scratch),
            map(f, s, g2, &mut scratch),
            map(f, s, g3, &mut scratch),
        );
        let cod = s.cod;
        if !(cod.on(f, ig) && cod.on(f, ig2) && cod.on(f, ig3)) {
            return Err("an image is not on the codomain".into());
        }
        if cod.add(f, ig, ig2) != ig3 {
            return Err("phi(G + 2G) != phi(G) + phi(2G)".into());
        }
        if ig.is_none() || cod.mul(f, ig, r).is_some() {
            return Err("the image of G does not have order r".into());
        }
        cur = cod;
        g = ig;
        heights.push(height(f, cur, 8));
    }
    if g != Some(carried.g) || cur != carried.curve {
        return Err("the carried endpoint does not replay".into());
    }
    Ok(heights)
}

fn u(v: &Value, k: &str) -> Result<u64, String> {
    v.get(k)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing integer field {k}"))
}

fn tally_json(t: &Tally) -> Value {
    json!({"muls": t.muls, "invs": t.invs, "ops": t.ops()})
}

/// One `transport.jsonl` row for a floor curve of `curves.jsonl`.
pub fn transport_row(
    f: &Fp,
    row: &Value,
    r: u64,
    group_order: u64,
    depth: u32,
    j_index: &[(u64, u64, u64)],
) -> Result<Value, String> {
    let c0 = Curve {
        a: u(row, "a")?,
        b: u(row, "b")?,
    };
    let g0 = (u(row, "gx")?, u(row, "gy")?);
    let lookup = |j: u64, level: u64| {
        j_index
            .iter()
            .find(|(jj, lv, _)| *jj == j && *lv == level)
            .map(|x| x.2)
    };
    let arm = |carried: Carried, want_end: u32, end_level: u64| -> Result<Value, String> {
        let heights = verify(f, c0, g0, r, &carried)?;
        if *heights.last().unwrap() != want_end {
            return Err(format!(
                "endpoint height {:?}, expected {want_end}",
                heights
            ));
        }
        let end_j = *carried.path_j.last().unwrap();
        Ok(json!({
            "a": carried.curve.a, "b": carried.curve.b, "gx": carried.g.0, "gy": carried.g.1,
            "j": end_j, "path_j": carried.path_j, "path_heights_verified": heights,
            "kernels": carried.steps.iter().map(|s| s.x0).collect::<Vec<_>>(),
            "end_vertex_in_export": lookup(end_j, end_level),
            "ops_setup": tally_json(&carried.setup),
            "ops_online": tally_json(&carried.online),
            "ops_total": carried.setup.ops() + carried.online.ops(),
        }))
    };
    let b = arm(ascend(f, c0, g0, depth)?, depth, 0)?;
    let sib = sibling(f, c0, g0)?;
    if *sib.path_j.last().unwrap() == c0.j(f) {
        return Err("the sibling path returned to its start".into());
    }
    let b0 = arm(sib, 0, depth as u64)?;
    Ok(json!({
        "schema": "repsel.transport/v1",
        "p": f.p, "group_order": group_order, "r": r, "depth": depth,
        "floor_vertex": u(row, "vertex")?, "floor_j": c0.j(f),
        "source": {"a": c0.a, "b": c0.b, "gx": g0.0, "gy": g0.1},
        "B": b, "B0": b0,
        "charge": "every native operation in ops_total is added to each run of the arm at the band weight w (AGENTS.md 5a)",
    }))
}

/// `repsel ascend`: write `transport.jsonl` beside a class's `curves.jsonl`
/// for every floor curve (model 0); returns the rows.
pub fn write_transport(dir: &std::path::Path) -> Result<Vec<Value>, String> {
    let ci = crate::specs::load_class(dir)?;
    let f = Fp::new(ci.p);
    let idx: Vec<(u64, u64, u64)> = ci
        .rows
        .iter()
        .filter(|x| x["model"].as_u64() == Some(0))
        .map(|x| Ok((u(x, "j")?, u(x, "level")?, u(x, "vertex")?)))
        .collect::<Result<_, String>>()?;
    let mut rows = Vec::new();
    let mut text = String::new();
    for row in ci
        .rows
        .iter()
        .filter(|x| x["model"].as_u64() == Some(0) && x["level"].as_u64() == Some(ci.depth))
    {
        let t = transport_row(&f, row, ci.r, ci.order, ci.depth as u32, &idx)
            .map_err(|e| format!("vertex {}: {e}", row["vertex"]))?;
        text += &t.to_string();
        text.push('\n');
        rows.push(t);
    }
    std::fs::write(dir.join("transport.jsonl"), text).map_err(|e| e.to_string())?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../testdata/class-l2-s1001/curves.jsonl");
    const P: u64 = 15808601;
    const R: u64 = 1317583;

    fn rows() -> Vec<Value> {
        FIXTURE
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect()
    }

    fn index(rows: &[Value]) -> Vec<(u64, u64, u64)> {
        rows.iter()
            .map(|x| {
                (
                    u(x, "j").unwrap(),
                    u(x, "level").unwrap(),
                    u(x, "vertex").unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn every_floor_curve_ascends_to_an_exported_crater_vertex_along_its_bfs_parents() {
        let f = Fp::new(P);
        let rows = rows();
        let idx = index(&rows);
        let by_vertex = |v: u64| rows.iter().find(|x| u(x, "vertex").unwrap() == v);
        let floors: Vec<&Value> = rows
            .iter()
            .filter(|x| u(x, "level").unwrap() == 3)
            .collect();
        assert_eq!(floors.len(), 12);
        for fl in floors {
            let row = transport_row(&f, fl, R, 15810996, 3, &idx).unwrap();
            let b = &row["B"];
            assert_eq!(b["path_heights_verified"], json!([0, 1, 2, 3]));
            assert!(
                b["end_vertex_in_export"].is_u64(),
                "crater endpoint not in the export: {row}"
            );
            // The first step goes to the BFS parent (the floor vertex's only neighbour).
            let parent = fl["parent"].as_u64().unwrap();
            if let Some(pr) = by_vertex(parent) {
                assert_eq!(b["path_j"][1].as_u64().unwrap(), u(pr, "j").unwrap());
            }
            let b0 = &row["B0"];
            assert_eq!(b0["path_heights_verified"], json!([0, 1, 0]));
            assert_ne!(b0["j"], row["floor_j"]);
            // Charged work is a few hundred field operations, far below one IC solve.
            let ops = b["ops_total"].as_u64().unwrap();
            assert!(ops > 100 && ops < 2000, "B ops {ops}");
            assert!(b0["ops_total"].as_u64().unwrap() < ops);
        }
    }

    #[test]
    fn ascending_is_deterministic_and_counted() {
        let f = Fp::new(P);
        let fl = rows()
            .into_iter()
            .find(|x| u(x, "level").unwrap() == 3)
            .unwrap();
        let c0 = Curve {
            a: u(&fl, "a").unwrap(),
            b: u(&fl, "b").unwrap(),
        };
        let g0 = (u(&fl, "gx").unwrap(), u(&fl, "gy").unwrap());
        let a = ascend(&f, c0, g0, 3).unwrap();
        let b = ascend(&f, c0, g0, 3).unwrap();
        assert_eq!(a.setup, b.setup);
        assert_eq!((a.curve, a.g), (b.curve, b.g));
        assert_eq!(a.online, Tally { muls: 12, invs: 3 });
    }

    #[test]
    fn a_non_floor_curve_is_refused() {
        let f = Fp::new(P);
        let crater = rows()
            .into_iter()
            .find(|x| u(x, "level").unwrap() == 0)
            .unwrap();
        let c0 = Curve {
            a: u(&crater, "a").unwrap(),
            b: u(&crater, "b").unwrap(),
        };
        let g0 = (u(&crater, "gx").unwrap(), u(&crater, "gy").unwrap());
        assert!(ascend(&f, c0, g0, 3).is_err());
        assert_eq!(height(&f, c0, 8), 3);
    }
}
