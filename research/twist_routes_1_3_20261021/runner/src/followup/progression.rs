//! Target-blind progression constructors; all discovery is charged by the caller.
use crate::{solver, toy_field::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

pub fn census(input: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh progression census required".into());
    }
    fs::create_dir_all(out.join("bases"))?;
    let old: Value = serde_json::from_slice(&fs::read(input)?)?;
    let mut fixtures = BTreeMap::<String, Fixture>::new();
    for cell in old["cells"].as_array().ok_or("prior cells")? {
        let f: Fixture = serde_json::from_value(cell["task"]["fixture"].clone())?;
        fixtures.insert(f.id.clone(), f);
    }
    assert_eq!(fixtures.len(), 10);
    let mut rows = vec![];
    for f in fixtures.values() {
        for family in ["twist-orbit", "curve-orbit", "field-ap", "uniform"] {
            for shift in [0, 1, 3] {
                for size in [8, 16] {
                    let id = format!("{}-{family}-c{shift}-b{size}", f.id);
                    let base = construct(f, family, shift, size)?;
                    let xs: Vec<u64> = serde_json::from_value(base["base"].clone())?;
                    let e = f.curve();
                    let points = xs
                        .iter()
                        .map(|x| {
                            (
                                *x,
                                Some((Q(*x, 0), Q(sqrt(e.rhs(Q(*x, 0)).0, f.p).unwrap(), 0))),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                    let mut queries = vec![];
                    for arity in [2, 3] {
                        for label in 0..16 {
                            let target = solver::target(f, label);
                            let count = if base["filled"] == true {
                                Some(super::block::group_mitm(e, &xs, &points, target, arity).len())
                            } else {
                                None
                            };
                            queries.push(json!({"arity":arity,"label":label,"target":target,"canonical_relation_tuples":count}));
                        }
                    }
                    let row = json!({"id":id,"fixture":f,"construction":base,"queries":queries});
                    write_json(out.join("bases").join(format!("{id}.json")), &row)?;
                    rows.push(row);
                }
            }
        }
    }
    write_json(out.join("ROWS.json"), &rows)?;
    write_json(
        out.join("SUMMARY.json"),
        &json!({"schema":"twist.progression-census/v1","bases":rows.len(),
        "filled":rows.iter().filter(|r|r["construction"]["filled"]==true).count(),"queries_requested":rows.len()*32,
        "labels":[0,15],"final_E28_labels_opened":false,"cost_measured":false}),
    )
}

pub fn construct(f: &Fixture, family: &str, shift: u64, size: usize) -> Result<Value> {
    let source = f.curve();
    let p = f.p;
    if shift >= p
        || size == 0
        || !["twist-orbit", "curve-orbit", "field-ap", "uniform"].contains(&family)
    {
        return Err("invalid progression constructor".into());
    }
    let twist = family == "twist-orbit";
    let orbit = family.ends_with("orbit");
    let model = if twist {
        Quad {
            p,
            d: f.d,
            a: f.a * f.d % p * f.d % p,
            b: f.b * f.d % p * f.d % p * f.d % p,
        }
    } else {
        source
    };
    let mut seed_trials = 0u64;
    let generator = if twist {
        (0..p)
            .find_map(|x| {
                seed_trials += 1;
                sqrt(model.rhs(Q(x, 0)).0, p)
                    .filter(|y| *y != 0)
                    .map(|y| Some((Q(x, 0), Q(y.min(p - y), 0))))
            })
            .ok_or("twist has no nonzero-ordinate seed")?
    } else if orbit {
        f.group()
    } else {
        None
    };
    let inverse_d = if twist { inv(f.d, p) } else { 1 };
    let cycle_cap = if twist {
        2 * (p + 1) - f.order
    } else {
        f.order
    };
    let limit = if orbit {
        cycle_cap
    } else if family == "field-ap" {
        p
    } else {
        8 * p
    };
    let mut state = 202610260001u64 ^ p ^ (f.b << 16) ^ shift;
    let mut point = generator;
    let mut accepted = BTreeSet::new();
    let mut attempts = vec![];
    let mut reason = "candidate-cap";
    for i in 0..limit {
        if orbit && point.is_none() {
            reason = "orbit-identity";
            break;
        }
        let raw = if orbit {
            point.unwrap().0 .0
        } else if family == "field-ap" {
            i
        } else {
            crate::next(&mut state) % p
        };
        let u = raw * inverse_d % p;
        let x = (u + shift) % p;
        let y = sqrt(source.rhs(Q(x, 0)).0, p).filter(|y| *y != 0);
        let fresh = if y.is_some() {
            accepted.insert(x)
        } else {
            false
        };
        attempts.push(json!({"term":i+1,"raw_x":raw,"u":u,"x":x,"source_lift":y.is_some(),"new_coordinate":fresh}));
        if accepted.len() == size {
            reason = "filled";
            break;
        }
        if orbit {
            point = model.pa(point, generator);
        }
    }
    let xs = accepted.into_iter().collect::<Vec<_>>();
    Ok(
        json!({"schema":"twist.progression-base/v1","fixture":f.id,"source_identity":f.id,
        "twist_identity":f.twist_id,"family":family,"shift":shift,"B":size,"base":xs,
        "filled":xs.len()==size,"stop_reason":reason,"seed_trials":seed_trials,
        "orbit_generator":generator,"orbit_model":{"p":p,"a":model.a,"b":model.b},
        "inverse_d":inverse_d,"candidate_cap":limit,"attempts":attempts,
        "target_used_by_constructor":false}),
    )
}

pub fn worker(input: &Path, out: &Path, budget: u64) -> Result<()> {
    let mut task: Value = serde_json::from_slice(&fs::read(input)?)?;
    let f: Fixture = serde_json::from_value(task["fixture"].clone())?;
    let family = task["family"].as_str().ok_or("family")?.to_owned();
    let shift = task["shift"].as_u64().ok_or("shift")?;
    let provenance = construct(&f, &family, shift, task["B"].as_u64().ok_or("B")? as usize)?;
    if provenance["filled"] != true {
        return write_json(
            out,
            &json!({"schema":"twist.progression-worker/v1","base_provenance":provenance,
            "status":"unsupported-underfilled-base","complete_projection":false}),
        );
    }
    task["base"] = provenance["base"].clone();
    task["base_provenance"] = provenance;
    if task["linear_roots"].is_null() {
        task["linear_roots"] = json!(true);
    }
    if task["arm"] == "companion-curve" {
        task["companion"] = json!({"shift":shift,"delta":if family=="twist-orbit" {f.d} else {1}});
    }
    let derived = out.with_extension("derived-task.json");
    write_json(&derived, &task)?;
    super::block::decomposition_worker(&derived, out, budget)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Fixture {
        Fixture {
            id: "test".into(),
            p: 59,
            a: 1,
            b: 13,
            order: 67,
            d: 2,
            generator: [1, 29],
            role: "test".into(),
            extension_id: "test".into(),
            twist_id: "test-twist".into(),
            cubic_extension_id: None,
            sextic_extension_id: None,
            sextic_modulus: None,
            sextic_flat_generator: None,
        }
    }
    #[test]
    fn unshifted_twist_cannot_supply_nonzero_source_lifts() {
        let v = construct(&fixture(), "twist-orbit", 0, 8).unwrap();
        assert_eq!(v["base"], json!([]));
        assert_eq!(v["filled"], false);
        assert_eq!(v["stop_reason"], "orbit-identity");
    }
    #[test]
    fn constructors_produce_sorted_distinct_valid_source_coordinates() {
        let f = fixture();
        for family in ["twist-orbit", "curve-orbit", "field-ap", "uniform"] {
            for shift in [1, 3] {
                let v = construct(&f, family, shift, 8).unwrap();
                let xs: Vec<u64> = serde_json::from_value(v["base"].clone()).unwrap();
                assert!(xs.windows(2).all(|w| w[0] < w[1]));
                for x in xs {
                    assert!(sqrt(f.curve().rhs(Q(x, 0)).0, f.p).is_some_and(|y| y != 0));
                }
            }
        }
    }
    #[test]
    fn planted_relations_survive_all_four_complete_worker_paths() {
        let f = fixture();
        let e = f.curve();
        let root =
            std::env::temp_dir().join(format!("twist-progression-positive-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let linear = std::env::var("F4_FP_LINEAR_ROOTS").is_ok_and(|v| v == "1");
        for family in ["twist-orbit", "curve-orbit", "field-ap", "uniform"] {
            let construction = construct(&f, family, 1, 8).unwrap();
            assert_eq!(construction["filled"], true);
            let xs: Vec<u64> = serde_json::from_value(construction["base"].clone()).unwrap();
            let point = |x: u64| Some((Q(x, 0), Q(sqrt(e.rhs(Q(x, 0)).0, f.p).unwrap(), 0)));
            for arity in [2, 3] {
                let mut tuple = vec![xs[0], xs[1]];
                let pair = e.pa(point(xs[0]), point(xs[1]));
                if arity == 3 {
                    tuple.push(
                        *xs.iter()
                            .find(|x| e.pa(pair, point(**x)).is_some())
                            .unwrap(),
                    );
                }
                let target = tuple.iter().fold(None, |s, x| e.pa(s, point(*x)));
                for arm in [
                    "direct-chain",
                    "hybrid-eight",
                    "group-mitm",
                    "companion-curve",
                ] {
                    if arm == "companion-curve" && !family.ends_with("orbit") {
                        continue;
                    }
                    let name = format!("{family}-{arity}-{arm}");
                    let input = root.join(format!("{name}.input.json"));
                    let out = root.join(format!("{name}.output.json"));
                    write_json(&input,&json!({"fixture":f,"family":family,"shift":1,"B":8,"arity":arity,"target":target,"arm":arm,"linear_roots":linear})).unwrap();
                    worker(&input, &out, 15000).unwrap();
                    let result: Value = serde_json::from_slice(&fs::read(out).unwrap()).unwrap();
                    assert_eq!(result["complete_projection"], true, "{name}");
                    assert!(
                        result["signed_witnesses"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .any(|r| r["x"] == json!(tuple) && r["sign_mask"] == 0),
                        "planted row missing: {name}"
                    );
                }
            }
        }
        fs::remove_dir_all(root).unwrap();
    }
}
