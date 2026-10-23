//! Finite helper elimination with exact projected and whole-extension witnesses.
use super::coupled::{public_target, Image, Linear};
use crate::{digest, toy_field::*, trace::canonical, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::Instant,
};
type Rows = BTreeSet<Vec<i16>>;
fn rational_index(e: Quad, base: &[P], m: usize) -> (BTreeMap<P, Rows>, u64) {
    let signed = base
        .iter()
        .enumerate()
        .flat_map(|(i, p)| [(*p, i, 1i16), (e.pn(*p), i, -1i16)])
        .collect::<Vec<_>>();
    let mut index = BTreeMap::<P, Rows>::new();
    let mut adds = 0;
    for &(p, i, a) in &signed {
        for &(q, j, b) in &signed {
            let s = e.pa(p, q);
            adds += 1;
            let endings = if m == 2 {
                vec![(None, 0, 0)]
            } else {
                signed.clone()
            };
            for (z, k, c) in endings {
                let s = if m == 2 {
                    s
                } else {
                    adds += 1;
                    e.pa(s, z)
                };
                let mut row = vec![0; base.len()];
                row[i] += a;
                row[j] += b;
                if m == 3 {
                    row[k] += c;
                }
                index.entry(s).or_default().insert(row);
            }
        }
    }
    (index, adds)
}
fn scalar_adds(n: u64) -> u64 {
    (64 - n.leading_zeros() + n.count_ones()) as u64
}
fn embed(e: Quad, p: P) -> P {
    p.map(|(x, y)| {
        (
            Q(x.0 * inv(e.d, e.p) % e.p, 0),
            Q(0, y.0 * inv(e.d * e.d % e.p, e.p) % e.p),
        )
    })
}
fn projected(e: Quad, p: P, n: u64, nt: u64) -> P {
    let ni = (1..n)
        .find(|x| nt * x % n == 1)
        .expect("noncoprime projector");
    e.pm(p, (nt * ni) as u128)
}
fn verify_row(e: Quad, points: &[P], g: P, row: &[u64]) -> u64 {
    let mut p = None;
    let mut additions = 0;
    for (a, q) in row[..points.len()].iter().zip(points) {
        p = e.pa(p, e.pm(*q, *a as u128));
        additions += 1 + scalar_adds(*a);
    }
    assert_eq!(p, e.pm(g, *row.last().unwrap() as u128));
    additions + scalar_adds(*row.last().unwrap())
}
fn collect(
    f: &Fixture,
    base: &[P],
    helpers: &[P],
    u: P,
    index: &BTreeMap<P, Rows>,
    target: P,
    mode: &str,
    out: &Path,
) -> Result<Value> {
    let e = f.curve();
    let start = Instant::now();
    let before = reference_calls();
    let mut additions = 0u64;
    let general = mode == "general-extension" || mode == "direct-rational";
    let cols = base.len() + if general { helpers.len() } else { 0 };
    let mut columns = base.to_vec();
    if general {
        columns.extend(helpers);
    }
    columns.push(target);
    let mut l = Linear::new(f.order);
    let mut raw = vec![];
    let mut graph = BTreeSet::new();
    let mut status = "exhausted-partial-row-search";
    let mut k = None;
    let query = [(None, 0u64, 0u64), (f.group(), 0, 1), (target, 1, 0)];
    let pairs = if general {
        (0..helpers.len())
            .flat_map(|i| (i..helpers.len()).map(move |j| (i, j)))
            .collect::<Vec<_>>()
    } else {
        vec![(0, 0)]
    };
    'outer: for &(i, j) in &pairs {
        let (s, h1, h2) = if general {
            let h1 = if mode == "general-extension" {
                e.pa(helpers[i], u)
            } else {
                helpers[i]
            };
            let h2 = if mode == "general-extension" {
                e.pa(helpers[j], e.pn(u))
            } else {
                helpers[j]
            };
            additions += 3;
            let s = e.pa(h1, h2);
            assert_eq!(s, e.pa(helpers[i], helpers[j]));
            (s, h1, h2)
        } else if mode == "pure-twist" {
            (None, u, e.pn(u))
        } else {
            (None, None, None)
        };
        for (r, beta, rhs) in query {
            let need = e.pa(r, e.pn(s));
            additions += 1;
            if let Some(rs) = index.get(&need) {
                for row in rs {
                    if raw.len() >= 65536 {
                        status = "stored-partial-row-cap";
                        break 'outer;
                    }
                    if additions >= (1 << 20) {
                        status = "explicit-group-operation-cap";
                        break 'outer;
                    }
                    if start.elapsed().as_secs() >= 120 {
                        status = "wall-failsafe-cap-unranked";
                        break 'outer;
                    }
                    let sum = row.iter().zip(base).fold(None, |p, (c, q)| {
                        e.pa(
                            p,
                            e.pm(if *c < 0 { e.pn(*q) } else { *q }, c.unsigned_abs() as u128),
                        )
                    });
                    additions += base.len() as u64
                        + 2
                        + row
                            .iter()
                            .map(|c| scalar_adds(c.unsigned_abs() as u64))
                            .sum::<u64>();
                    assert_eq!(
                        e.pa(e.pa(sum, h1), h2),
                        r,
                        "whole extension relation failed"
                    );
                    let mut v = vec![0; cols + 2];
                    for (v, c) in v.iter_mut().zip(row) {
                        *v = (*c as i64).rem_euclid(f.order as i64) as u64;
                    }
                    if general {
                        v[base.len() + i] = (v[base.len() + i] + 1) % f.order;
                        v[base.len() + j] = (v[base.len() + j] + 1) % f.order;
                        graph.insert((i, j));
                    }
                    v[cols] = (f.order - beta) % f.order;
                    v[cols + 1] = rhs;
                    raw.push(v.clone());
                    l.insert(v);
                    k = l.value(cols, cols + 1);
                    if k.is_some() {
                        status = "verified-target-span-recovery";
                        break 'outer;
                    }
                }
            }
        }
    }
    if let Some(k) = k {
        assert_eq!(e.pm(f.group(), k as u128), target);
    }
    let mut replay_adds = 0;
    for row in l.rows.values() {
        replay_adds += verify_row(e, &columns, f.group(), row);
    }
    let mut components = (0..helpers.len()).collect::<Vec<_>>();
    for &(i, j) in &graph {
        let ci = components[i];
        let cj = components[j];
        for c in &mut components {
            if *c == cj {
                *c = ci;
            }
        }
    }
    let file = format!(
        "{}-{}.json",
        digest(&serde_json::to_vec(&(f.id.clone(), base, helpers, target))?),
        mode
    );
    let witness = json!({"fixture":f.id,"mode":mode,"base":base,"helper_projections":if general{helpers}else{&[]},"common_anti_point":u,"target":target,
      "columns_including_target":columns,"raw_projected_equations":raw,"raw_row_sha256":digest(&serde_json::to_vec(&raw)?),"eliminated_echelon_rows":l.rows,
      "graph_edges":graph,"graph_component_labels":components,"all_input_extension_relations_verified":true,"all_eliminated_rows_replayed_in_source_group":true,
      "scalar":k,"status":status,"rank":l.rows.len(),"explicit_collection_group_additions":additions,"eliminated_row_replay_additions":replay_adds,
      "unpriced_reference_calls_including_verification":reference_calls()-before,"matrix_modular_row_operations":l.ops});
    write_json(out.join(&file), &witness)?;
    Ok(
        json!({"file":file,"mode":mode,"status":status,"scalar_verified":k.is_some(),"rank":l.rows.len(),"raw_rows":raw.len(),"row_sha256":digest(&serde_json::to_vec(&raw)?),"matrix_row_operations":l.ops,"explicit_collection_group_additions":additions,"unpriced_reference_calls":reference_calls()-before}),
    )
}
pub fn replay(fixtures: &[Fixture], input: &Path, out: &Path) -> Result<Vec<Value>> {
    let selected = ["linear-l1-h4", "linear-l3-h4"];
    let mut images = vec![];
    for entry in fs::read_dir(input)? {
        let p = entry?.path();
        if p.extension().is_some_and(|x| x == "json") && p.file_name().unwrap() != "SUMMARY.json" {
            let im: Image = serde_json::from_slice(&fs::read(p)?)?;
            if selected.contains(&im.config.as_str()) {
                images.push(im);
            }
        }
    }
    images.sort_by(|a, b| (&a.fixture, &a.config).cmp(&(&b.fixture, &b.config)));
    let mut rows = vec![];
    let mut visited = BTreeSet::new();
    for im in images {
        let f = fixtures.iter().find(|f| f.id == im.fixture).unwrap();
        let e = f.curve();
        let nt = 2 * (f.p + 1) - f.order;
        if !(1..f.order).any(|x| nt * x % f.order == 1) {
            rows.push(json!({"experiment":"E23","fixture":f.id,"status":"unsupported-noncoprime-projector"}));
            continue;
        }
        let et = Quad::new(
            f.p,
            f.a * f.d % f.p * f.d % f.p,
            f.b * f.d % f.p * f.d % f.p * f.d % f.p,
        );
        assert_eq!(et.rational().len() as u64, nt);
        let u = et
            .rational()
            .into_iter()
            .map(|p| embed(e, p))
            .find(|p| p.is_some())
            .unwrap();
        assert!(e.on(u));
        assert_eq!(e.frob(u), e.pn(u));
        assert_eq!(projected(e, u, f.order, nt), None);
        // A fake nonzero twist-log column is inadmissible, even if its label differs.
        let fake_column_rejected = projected(e, u, f.order, nt).is_none();
        assert!(fake_column_rejected);
        for b in [8usize, 12, 16] {
            for m in [2usize, 3] {
                for h in [16usize, 32, 64] {
                    if im.images.len() < b {
                        rows.push(json!({"experiment":"E23","fixture":f.id,"config":im.config,"B":b,"arity":m,"H":h,"status":"unsupported-undersized-base"}));
                        continue;
                    }
                    let base = im.images[..b].iter().map(|x| x.0).collect::<Vec<_>>();
                    let key = digest(&serde_json::to_vec(&(f.id.clone(), &base, m, h))?);
                    if !visited.insert(key.clone()) {
                        rows.push(json!({"experiment":"E23","fixture":f.id,"config":im.config,"B":b,"arity":m,"H":h,"status":"identical-projected-profile-shared","profile":key}));
                        continue;
                    }
                    let mut available = e
                        .rational()
                        .into_iter()
                        .filter(|p| p.is_some() && canonical(e, *p) == *p && !base.contains(p))
                        .collect::<Vec<_>>();
                    available.sort();
                    if available.len() < h {
                        rows.push(json!({"experiment":"E23","fixture":f.id,"B":b,"arity":m,"H":h,"status":"unsupported-insufficient-distinct-helper-projections"}));
                        continue;
                    }
                    let helpers = &available[..h];
                    for p in helpers {
                        assert_eq!(projected(e, e.pa(*p, u), f.order, nt), *p);
                    }
                    let before = reference_calls();
                    let (index, index_adds) = rational_index(e, &base, m);
                    let index_calls = reference_calls() - before;
                    let mut targets = vec![];
                    for label in if f.role.ends_with("-1") {
                        16..32
                    } else {
                        0..16
                    } {
                        let q = public_target(f, "E23", label);
                        let mut arms = vec![];
                        for mode in [
                            "zero-helper",
                            "pure-twist",
                            "general-extension",
                            "direct-rational",
                        ] {
                            arms.push(collect(f, &base, helpers, u, &index, q, mode, out)?);
                        }
                        assert_eq!(
                            arms[0]["row_sha256"], arms[1]["row_sha256"],
                            "pure twist changed useful rows"
                        );
                        assert_eq!(
                            arms[2]["row_sha256"], arms[3]["row_sha256"],
                            "extension/rational helper graphs differ"
                        );
                        assert_eq!(arms[2]["scalar_verified"], arms[3]["scalar_verified"]);
                        targets.push(json!({"label":label,"arms":arms,"same_projected_graph_and_stopping_rule":true}));
                    }
                    let record = json!({"experiment":"E23","fixture":f.id,"config":im.config,"profile":key,"B":b,"arity":m,"H":h,"index_additions_cold":index_adds,"index_unpriced_reference_calls_cold":index_calls,
              "helper_construction_and_projector_costs_complete":false,"fake_twist_log_column_rejected":fake_column_rejected,"targets":targets,"status":"bounded-partial-helper-controls-verified","full_E23_complete":false,"performance_claim":false});
                    write_json(out.join(format!("profile-{key}.json")), &record)?;
                    rows.push(json!({"experiment":"E23","fixture":f.id,"B":b,"arity":m,"H":h,"profile":key,"targets":16,"status":"bounded-partial-helper-controls-verified"}));
                }
            }
        }
        println!(
            "{}",
            json!({"experiment":"E23","fixture":f.id,"config":im.config,"status":"helper-profiles-finished"})
        );
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_extension_helper_rows_have_identical_source_graph_and_elimination() {
        let v: Value =
            serde_json::from_str(include_str!("../../../execution-v1/fixtures.json")).unwrap();
        let fixtures: Vec<Fixture> = serde_json::from_value(v["fixtures"].clone()).unwrap();
        let f = fixtures.iter().find(|f| f.p == 59).unwrap();
        let e = f.curve();
        let canonical_points = e
            .rational()
            .into_iter()
            .filter(|p| p.is_some() && canonical(e, *p) == *p)
            .collect::<Vec<_>>();
        let base = &canonical_points[..8];
        let helpers = &canonical_points[8..24];
        let et = Quad::new(59, 4, 45);
        let u = et
            .rational()
            .into_iter()
            .map(|p| embed(e, p))
            .find(|p| p.is_some())
            .unwrap();
        assert_eq!(projected(e, u, 67, 53), None);
        let (index, _) = rational_index(e, base, 2);
        let q = e.pm(f.group(), 13);
        let dir = std::env::temp_dir().join(format!("twist-helper-support-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let a = collect(f, base, helpers, u, &index, q, "general-extension", &dir).unwrap();
        let b = collect(f, base, helpers, u, &index, q, "direct-rational", &dir).unwrap();
        assert_eq!(a["row_sha256"], b["row_sha256"]);
        assert_eq!(a["scalar_verified"], true);
        assert_eq!(a["rank"], b["rank"]);
        fs::remove_dir_all(dir).unwrap();
    }
}
