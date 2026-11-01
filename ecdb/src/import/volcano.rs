//! ℓ-isogeny volcanoes derived from the stored isogeny graphs (Kohel;
//! Fouquet–Morain). For an ordinary isogeny class over F_p with
//! t^2 - 4p = v^2 D_K, the ℓ-volcano has depth d = v_ℓ(v). Its floor is the
//! set of curves with exactly one rational ℓ-isogeny, and a curve's level is
//! d minus its distance to the floor in the ℓ-graph. The level is v_ℓ of the
//! conductor of End(E): 0 on the crater, d on the floor.
//!
//! Only `census-fp` graphs are used: they hold every curve over F_p (p < 2^10)
//! and every rational ℓ-isogeny between them (ℓ = 2, 3, 5), so each component
//! is complete. Before anything is written, a class and ℓ must pass every
//! check below; otherwise it is reported and skipped:
//! - every out-degree is 1 + (D_K/ℓ) when d = 0, and ℓ + 1 or 1 when d ≥ 1;
//! - every curve is at distance ≤ d from the floor;
//! - for D_K ∉ {-3, -4}, each component has level sizes
//!   n_k = n_0 (ℓ - (D_K/ℓ)) ℓ^(k-1) (automorphisms change the counts at j = 0, 1728).
//!
//! It writes:
//! - `isogeny_volcano` components and their `volcano_vertex` rows;
//! - `endomorphism_level`;
//! - each edge's direction;
//! - `endomorphism_ring` when v is {2, 3, 5}-smooth, which pins f_E = ∏ ℓ^level.
//!
//! Every other curve with a known conductor gets level 0 for each ℓ ∤ v: its
//! ℓ-volcano is a bare crater.

use super::{bulk, bulk1, Ctx, Report};
use anyhow::Result;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};
use serde_json::json;
use std::collections::{BTreeMap, VecDeque};

const ELLS: [u64; 3] = [2, 3, 5];

const VERTICES_SQL: &str = "
SELECT c.id, ic.id, ic.p::text, fg.conductor_v::text, fg.cm_discriminant::text, fg.trace::text,
       coalesce(fg.is_supersingular, false)
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = ic.id
WHERE c.family = 'census-fp' AND fg.conductor_v IS NOT NULL AND fg.cm_discriminant IS NOT NULL";

const EDGES_SQL: &str = "
SELECT i.id, i.source_curve_id, i.target_curve_id, i.ell::int8
FROM ec.isogeny i JOIN ec.curve c ON c.id = i.source_curve_id
WHERE c.family = 'census-fp' AND i.formula = 'velu' AND i.ell IN (2, 3, 5)";

const VOLCANO_SQL: &str = "
INSERT INTO ec.isogeny_volcano (isogeny_class_id, ell, depth, crater_size, crater_shape, vertex_count, is_complete,
       status, provenance_id, component_key)
SELECT r.class_id, r.ell, r.depth, r.crater, r.shape, r.n, true, 'proved', $2::bigint, r.key
FROM jsonb_to_recordset($1::jsonb) AS r(class_id bigint, ell numeric, depth int, crater int, shape text, n int, key text)
ON CONFLICT (component_key) DO NOTHING";

const VERTEX_SQL: &str = "
INSERT INTO ec.volcano_vertex (volcano_id, curve_id, level)
SELECT v.id, r.curve_id, r.level
FROM jsonb_to_recordset($1::jsonb) AS r(key text, curve_id bigint, level int)
JOIN ec.isogeny_volcano v ON v.component_key = r.key
ON CONFLICT DO NOTHING";

const LEVEL_SQL: &str = "
INSERT INTO ec.endomorphism_level (curve_id, ell, depth, level, method, status, provenance_id)
SELECT r.curve_id, r.ell, r.depth, r.level, r.method, 'proved', $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, ell numeric, depth int, level int, method text)
ON CONFLICT (curve_id, ell) DO NOTHING";

const DIRECTION_SQL: &str = "
UPDATE ec.isogeny i SET direction = r.direction::ec.volcano_direction
FROM jsonb_to_recordset($1::jsonb) AS r(id bigint, direction text)
WHERE i.id = r.id AND i.direction IS NULL";

const END_SQL: &str = "
INSERT INTO ec.endomorphism_ring (curve_id, over_closure, kind, discriminant, conductor_f, conductor_divides,
       is_maximal, contains, status, method, provenance_id)
SELECT r.curve_id, false, 'imaginary_quadratic_order', r.disc, r.f, r.v, r.f = 1, ARRAY['Z[pi]'], 'proved',
       r.method, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(curve_id bigint, disc numeric, f numeric, v numeric, method text)
ON CONFLICT (curve_id) DO NOTHING";

/// Crater-only levels for every other curve with a known conductor: ℓ ∤ v means depth 0.
const CRATER_SQL: &str = "
INSERT INTO ec.endomorphism_level (curve_id, ell, depth, level, method, status, provenance_id)
SELECT c.id, l.ell, 0, 0, 'v_l(v) = 0: the l-volcano is a bare crater', 'proved', $1::bigint
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.frobenius_genus1 fg ON fg.isogeny_class_id = av.isogeny_class_id
CROSS JOIN (VALUES (2::numeric), (3::numeric), (5::numeric)) AS l(ell)
WHERE c.genus = 1 AND fg.conductor_v IS NOT NULL AND NOT coalesce(fg.is_supersingular, false)
  AND mod(fg.conductor_v, l.ell) <> 0
ON CONFLICT (curve_id, ell) DO NOTHING";

/// The Kronecker symbol (D/ℓ) for a fundamental discriminant D and ℓ ∈ {2, 3, 5}.
pub fn kronecker(d: &BigInt, l: u64) -> i64 {
    if l == 2 {
        if d.is_even() {
            return 0;
        }
        return if d.mod_floor(&BigInt::from(8)) == BigInt::from(1) { 1 } else { -1 };
    }
    let r = d.mod_floor(&BigInt::from(l)).to_u64().unwrap();
    if r == 0 {
        return 0;
    }
    // Euler's criterion for small odd ℓ
    let e = (1..=((l - 1) / 2)).fold(1u64, |acc, _| acc * r % l);
    if e == 1 {
        1
    } else {
        -1
    }
}

fn val(n: &BigInt, l: u64) -> u32 {
    let l = BigInt::from(l);
    let mut n = n.clone();
    let mut k = 0;
    while !n.is_zero() && (&n % &l).is_zero() {
        n /= &l;
        k += 1;
    }
    k
}

struct Vertex {
    class_id: i64,
    v: BigInt,
    d_k: BigInt,
}

/// The volcano of one class and ℓ: levels by vertex, or why it was refused.
#[allow(clippy::type_complexity)]
pub fn levels(
    l: u64,
    depth: u32,
    chi: i64,
    special: bool,
    verts: &[i64],
    out: &BTreeMap<i64, Vec<i64>>,
) -> Result<(BTreeMap<i64, u32>, Vec<Vec<i64>>), String> {
    let deg = |v: &i64| out.get(v).map_or(0, |e| e.len()) as i64;
    if depth == 0 {
        for v in verts {
            if deg(v) != 1 + chi {
                return Err(format!("depth 0 but out-degree {} != 1 + (D_K/{l}) = {}", deg(v), 1 + chi));
            }
        }
    } else {
        for v in verts {
            let d = deg(v);
            if d != l as i64 + 1 && d != 1 {
                return Err(format!("depth {depth} but out-degree {d} is neither {} nor 1", l + 1));
            }
        }
    }
    // undirected adjacency within the class
    let set: std::collections::BTreeSet<i64> = verts.iter().copied().collect();
    let mut adj: BTreeMap<i64, Vec<i64>> = verts.iter().map(|&v| (v, Vec::new())).collect();
    for v in verts {
        for t in out.get(v).into_iter().flatten() {
            if !set.contains(t) {
                return Err(format!("an edge leaves the isogeny class ({v} -> {t})"));
            }
            adj.get_mut(v).unwrap().push(*t);
            adj.get_mut(t).unwrap().push(*v);
        }
    }
    // components
    let mut comp_of: BTreeMap<i64, usize> = BTreeMap::new();
    let mut comps: Vec<Vec<i64>> = Vec::new();
    for &s in verts {
        if comp_of.contains_key(&s) {
            continue;
        }
        let id = comps.len();
        let mut q = VecDeque::from([s]);
        comp_of.insert(s, id);
        let mut members = vec![s];
        while let Some(x) = q.pop_front() {
            for &y in &adj[&x] {
                if let std::collections::btree_map::Entry::Vacant(e) = comp_of.entry(y) {
                    e.insert(id);
                    members.push(y);
                    q.push_back(y);
                }
            }
        }
        members.sort();
        comps.push(members);
    }
    // levels: depth - distance to the floor
    let mut level: BTreeMap<i64, u32> = BTreeMap::new();
    if depth == 0 {
        for v in verts {
            level.insert(*v, 0);
        }
    } else {
        let mut dist: BTreeMap<i64, u32> = BTreeMap::new();
        let mut q = VecDeque::new();
        for v in verts {
            if deg(v) == 1 {
                dist.insert(*v, 0);
                q.push_back(*v);
            }
        }
        while let Some(x) = q.pop_front() {
            let dx = dist[&x];
            for &y in &adj[&x] {
                if let std::collections::btree_map::Entry::Vacant(e) = dist.entry(y) {
                    e.insert(dx + 1);
                    q.push_back(y);
                }
            }
        }
        for v in verts {
            let Some(&dv) = dist.get(v) else { return Err(format!("curve {v} cannot reach the floor")) };
            if dv > depth {
                return Err(format!("curve {v} is {dv} steps from the floor, deeper than depth {depth}"));
            }
            level.insert(*v, depth - dv);
        }
        if !special {
            for c in &comps {
                let mut n = vec![0u64; depth as usize + 1];
                for v in c {
                    n[level[v] as usize] += 1;
                }
                for k in 1..=depth as usize {
                    let want = n[0] * (l as i64 - chi) as u64 * l.pow(k as u32 - 1);
                    if n[k] != want {
                        return Err(format!("component level sizes {n:?} violate n_k = n_0 ({l} - ({chi})) {l}^(k-1)"));
                    }
                }
            }
        }
    }
    Ok((level, comps))
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report { input: "census-fp isogeny graphs (stored ec.isogeny)".into(), ..Default::default() };
    let client = ctx.client;
    let src = ctx
        .source(
            "ecdb/src/import/volcano.rs",
            &crate::math::sha256_hex(include_str!("volcano.rs")),
            "l-volcano levels derived by ecdb from stored isogenies",
        )
        .await?;
    let prov = ctx
        .provenance(src, "Kohel volcano levels: depth v_l(v), level = depth - distance to the floor; checked", None)
        .await?;

    let mut vertices: BTreeMap<i64, Vertex> = BTreeMap::new();
    let mut by_class: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for r in client.query(VERTICES_SQL, &[]).await? {
        let id: i64 = r.get(0);
        if r.get::<_, bool>(6) {
            continue; // supersingular: no volcano
        }
        let class_id: i64 = r.get(1);
        let v: BigInt = r.get::<_, String>(3).parse()?;
        let d_k: BigInt = r.get::<_, String>(4).parse()?;
        vertices.insert(id, Vertex { class_id, v, d_k });
        by_class.entry(class_id).or_default().push(id);
    }
    let mut out: BTreeMap<u64, BTreeMap<i64, Vec<i64>>> = BTreeMap::new();
    let mut edge_ids: BTreeMap<u64, Vec<(i64, i64, i64)>> = BTreeMap::new();
    for r in client.query(EDGES_SQL, &[]).await? {
        let (id, s, t, l): (i64, i64, i64, i64) = (r.get(0), r.get(1), r.get(2), r.get(3));
        if !vertices.contains_key(&s) {
            continue;
        }
        out.entry(l as u64).or_default().entry(s).or_default().push(t);
        edge_ids.entry(l as u64).or_default().push((id, s, t));
    }
    report.count("census_vertices", vertices.len() as u64);

    let (mut volcanoes, mut vrows, mut lrows) = (Vec::new(), Vec::new(), Vec::new());
    let mut levels_by: BTreeMap<(u64, i64), u32> = BTreeMap::new();
    let empty = BTreeMap::new();
    for (class_id, verts) in &by_class {
        let first = &vertices[&verts[0]];
        let special = first.d_k == BigInt::from(-3) || first.d_k == BigInt::from(-4);
        for l in ELLS {
            let depth = val(&first.v, l);
            let chi = kronecker(&first.d_k, l);
            let o = out.get(&l).unwrap_or(&empty);
            match levels(l, depth, chi, special, verts, o) {
                Ok((level, comps)) => {
                    report.count(&format!("classes_ok_l{l}"), 1);
                    for c in comps {
                        let key = format!("ell:{l}:min_curve:{}", c[0]);
                        let crater = c.iter().filter(|v| level[v] == 0).count();
                        let shape = match crater {
                            1 => "single",
                            2 => "pair",
                            _ => "cycle",
                        };
                        volcanoes.push(json!({ "class_id": class_id, "ell": l, "depth": depth, "crater": crater,
                            "shape": shape, "n": c.len(), "key": key }));
                        for v in &c {
                            vrows.push(json!({ "key": key, "curve_id": v, "level": level[v] }));
                        }
                    }
                    for (v, lv) in level {
                        levels_by.insert((l, v), lv);
                        lrows.push(json!({ "curve_id": v, "ell": l, "depth": depth, "level": lv,
                            "method": "depth v_l(v); level = depth - distance to the floor in the complete stored l-graph" }));
                    }
                }
                Err(why) => {
                    report.count(&format!("classes_refused_l{l}"), 1);
                    report.problem(format!("class {class_id}, l = {l}: {why}"));
                }
            }
        }
    }
    report.count("volcanoes", bulk(client, VOLCANO_SQL, &volcanoes, prov).await?);
    report.count("volcano_vertices", bulk1(client, VERTEX_SQL, &vrows).await?);
    report.count("endomorphism_levels", bulk(client, LEVEL_SQL, &lrows, prov).await?);

    // directions
    let mut dirs = Vec::new();
    for (l, edges) in &edge_ids {
        for (id, s, t) in edges {
            if let (Some(ls), Some(lt)) = (levels_by.get(&(*l, *s)), levels_by.get(&(*l, *t))) {
                let d = match lt.cmp(ls) {
                    std::cmp::Ordering::Equal => "horizontal",
                    std::cmp::Ordering::Greater => "descending",
                    std::cmp::Ordering::Less => "ascending",
                };
                dirs.push(json!({ "id": id, "direction": d }));
            }
        }
    }
    report.count("isogeny_directions", bulk1(client, DIRECTION_SQL, &dirs).await?);

    // End(E) where v is {2,3,5}-smooth and every level is known
    let mut ends = Vec::new();
    for (id, vx) in &vertices {
        let mut rest = vx.v.clone();
        let mut f = BigInt::from(1);
        let mut known = true;
        for l in ELLS {
            let e = val(&rest, l);
            rest /= BigInt::from(l).pow(e);
            if e > 0 {
                match levels_by.get(&(l, *id)) {
                    Some(&lv) => f *= BigInt::from(l).pow(lv),
                    None => known = false,
                }
            }
        }
        if known && rest == BigInt::from(1) {
            ends.push(json!({ "curve_id": id, "disc": (&f * &f * &vx.d_k).to_string(), "f": f.to_string(),
                "v": vx.v.to_string(),
                "method": "f = prod over l | v of l^level (volcano levels), v {2,3,5}-smooth" }));
            let _ = vx.class_id;
        }
    }
    report.count("endomorphism_rings", bulk(client, END_SQL, &ends, prov).await?);
    report.count("crater_levels_other_curves", client.execute(CRATER_SQL, &[&prov]).await?);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kronecker_small() {
        assert_eq!(kronecker(&BigInt::from(-7), 2), 1); // -7 = 1 mod 8: 2 splits in Q(sqrt -7)
        assert_eq!(kronecker(&BigInt::from(-3), 2), -1);
        assert_eq!(kronecker(&BigInt::from(-4), 2), 0);
        assert_eq!(kronecker(&BigInt::from(-23), 3), 1); // -23 = 1 mod 3
        assert_eq!(kronecker(&BigInt::from(-7), 5), -1); // -7 = 3 mod 5, a non-residue
    }

    #[test]
    fn levels_of_a_depth_one_volcano() {
        // l = 2, D_K/2 = 0 (ramified): crater of one vertex with a loop, 2 descending to 2 floor vertices
        // crater a: out-edges a->a (horizontal), a->b, a->c ; floors b->a, c->a
        let mut out: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
        out.insert(1, vec![1, 2, 3]);
        out.insert(2, vec![1]);
        out.insert(3, vec![1]);
        let (lv, comps) = levels(2, 1, 0, false, &[1, 2, 3], &out).unwrap();
        assert_eq!(lv[&1], 0);
        assert_eq!(lv[&2], 1);
        assert_eq!(comps.len(), 1);
        // wrong degree is refused
        out.insert(2, vec![1, 3]);
        assert!(levels(2, 1, 0, false, &[1, 2, 3], &out).is_err());
    }
}
