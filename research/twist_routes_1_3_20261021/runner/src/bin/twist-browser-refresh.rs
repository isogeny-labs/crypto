//! Incremental, native catalog projection from a frozen browser and new records.
//! Retains all old measurements; performs no new curve or solver computation.
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
    process::Command,
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn load(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn word(v: &Value) -> String {
    v.as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| v.to_string())
}
fn bits(v: &Value) -> Value {
    if v.is_null() {
        return Value::Null;
    }
    json!(word(v)
        .parse::<BigUint>()
        .expect("recorded nonnegative integer")
        .bits())
}
fn sha(p: impl AsRef<Path>) -> Result<String> {
    let r = Command::new("sha256sum").arg(p.as_ref()).output()?;
    if !r.status.success() {
        return Err("metadata hash failed".into());
    }
    Ok(String::from_utf8(r.stdout)?
        .split_whitespace()
        .next()
        .ok_or("hash")?
        .to_string())
}
fn text(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or("").into()
}
fn n(v: &Value, key: &str) -> u64 {
    v[key].as_u64().unwrap_or(0)
}
fn unique_push(v: &mut Value, x: Value) {
    let a = v.as_array_mut().expect("array");
    if !a.contains(&x) {
        a.push(x);
    }
}
fn coefficient(v: &Value) -> Value {
    if v.is_null() {
        Value::Null
    } else if let Some(a) = v.as_array() {
        json!(format!(
            "[{}]",
            a.iter().map(word).collect::<Vec<_>>().join(", ")
        ))
    } else {
        json!(word(v))
    }
}
fn curve(c: &Value) -> Value {
    let p = &c["params"];
    let degree = if c["family"] == "extension" {
        n(p, "k").max(word(&p["k"]).parse().unwrap_or(0))
    } else {
        1
    };
    let field_bits = word(&p["p"])
        .parse::<BigUint>()
        .unwrap()
        .pow(degree as u32)
        .bits();
    let reps = c["representations"].as_array().cloned().unwrap_or_default();
    let subgroup = reps.first().map(|r| &r["curve"]).unwrap_or(&Value::Null);
    let model: Value = serde_json::from_str(c["model_json"].as_str().unwrap()).unwrap();
    json!({"slug":c["slug"],"icv1":c["icv1"],"family":c["family"],"standard_names":c["standard_names"],
        "field":{"characteristic":"p","p":word(&p["p"]),"bits":field_bits,"degree":degree,"modulus":p["modulus"]},
        "a":coefficient(if p["a"].is_null(){&model["a"]}else{&p["a"]}),
        "b":coefficient(if p["b"].is_null(){&model["b"]}else{&p["b"]}),
        "construction":p["generator_call"],"trace":if c["trace"].is_null(){Value::Null}else{json!(word(&c["trace"]))},
        "order":c["order"],"order_bits":bits(&c["order"]),"r":subgroup["subgroup_order"],"r_bits":bits(&subgroup["subgroup_order"]),
        "cofactor":subgroup["cofactor"],"generator":subgroup["generator"],"target_group":subgroup["target_group"],"j":c["j"],
        "endomorphism_discriminant":if c["end"]=="unk"{Value::Null}else{c["end"].clone()},"model_json":c["model_json"],
        "ec1":reps.iter().map(|r|r["ec1"].clone()).collect::<Vec<_>>(),"curve_uid":reps.iter().map(|r|r["curve_uid"].clone()).collect::<Vec<_>>(),
        "ec1_unresolved":c["ec1_unresolved"],"retired_names":c["aliases"],"sources":c["sources"],"on_leaderboard":false,
        "leaderboard":[],"ecbench":[],"factor_bases":[],"tournament_cells":[]})
}
fn yield_row(r: &Value, session: &Value) -> Value {
    let phases: BTreeMap<_, _> = r["phases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (text(p, "name"), p))
        .collect();
    let null = Value::Null;
    let rel = &phases.get("relations").copied().unwrap_or(&null)["native"];
    let la = &phases.get("linear_algebra").copied().unwrap_or(&null)["native"];
    let fb = &r["factor_base"];
    let dec = &r["detail"]["decomposition"];
    let params = &r["method"]["params"];
    let cost = &r["cost"];
    let workload = &r["workload"];
    let c = &workload["curve"];
    let relations = rel.get("relations").unwrap_or(&rel["hits"]);
    let trials = n(rel, "trials");
    let hits = relations.as_u64().unwrap_or(0);
    let oracle = if params["oracle"].is_null() {
        r["method"]["id"].clone()
    } else {
        params["oracle"].clone()
    };
    let mut v = json!({"run_id":r["run_id"],"record_id":r["record_id"],"session_id":session,"arm":r["arm"],"round":r["round"],
        "status":r["outcome"]["status"],"curve":c["slug"],"log2_r":bits(&c["r"]),"target_index":workload["target_index"],"target":workload["target"],
        "method_id":r["method"]["method_id"],"fb_id":fb["fb_id"],"fb_family":fb["family"],"fb_params":fb["params"],"fb_columns":fb["columns"],"fb_signed_points":fb["signed_points"],
        "oracle":oracle,"solver_name":params.get("solver").unwrap_or(&dec["solver"]),"summands":dec["summands"],"hit_rate":dec["hit_rate"],
        "trials":rel["trials"],"relations":relations,"yield":if trials>0&&!relations.is_null(){json!(hits as f64/trials as f64)}else{Value::Null},
        "lookups":rel["lookups"],"lookups_per_relation":if hits>0&&!rel["lookups"].is_null(){json!(n(rel,"lookups") as f64/hits as f64)}else{Value::Null},
        "walk_steps":rel["walk_steps"],"lift_failures":rel["lift_failures"],"unliftable_systems":rel["unliftable_systems"],"frobfold_mismatches":rel["frobfold_mismatches"],
        "matrix_rows":la["rows"],"matrix_columns":la["columns"],"matrix_rank":la["rank"],"row_ops":la["row_ops"],
        "gae":{},"total_gae":cost["total_gae"],"s":cost["s"],"deterministic":cost["deterministic"],"solver":r["solver"]});
    for key in [
        "factor_base",
        "oracle_setup",
        "relations",
        "linear_algebra",
        "verify",
    ] {
        v["gae"][key] = phases.get(key).copied().unwrap_or(&null)["gae"].clone();
    }
    if cost["total_gae"] == 0 && cost["unpriced"].as_array().is_some_and(|a| !a.is_empty()) {
        v["total_gae"] = Value::Null;
        v["s"] = Value::Null;
        v["charged_gae_lower_bound"] = json!(0);
        v["charged_s_lower_bound"] = json!(0);
        v["score_status"] = json!("unknown complete price; zero charged lower bound");
        v["unpriced"] = cost["unpriced"].clone();
    }
    v
}
#[derive(Default)]
struct Totals {
    n: u64,
    verified: u64,
    s: f64,
    floor: f64,
    zero: u64,
    method: Value,
    id: Value,
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let base = Path::new(args.get(1).ok_or("frozen browser JSON")?);
    let root = Path::new(args.get(2).ok_or("repository root")?);
    let output = Path::new(args.get(3).ok_or("fresh output JSON")?);
    if output.exists() {
        return Err("fresh output required".into());
    }
    let mut b = load(base)?;
    let registry = load(root.join("docs/curves/registry.json"))?;
    let old: BTreeMap<_, _> = b["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| (text(c, "slug"), c.clone()))
        .collect();
    let mut curves = vec![];
    let mut added = 0;
    for c in registry["curves"].as_array().unwrap() {
        let row = if let Some(row) = old.get(&text(c, "slug")) {
            assert_eq!(row["model_json"], c["model_json"]);
            row.clone()
        } else {
            assert!(c["family"] == "prime" || c["family"] == "extension");
            added += 1;
            curve(c)
        };
        curves.push(row);
    }
    assert_eq!(old.len() + added, curves.len());
    assert_eq!(added, 65);
    let covers = load(root.join("docs/curves/covers.json"))?;
    let graph = load(root.join("docs/curves/cover-links.yaml"))?;
    let hash = sha(root.join("docs/curves/registry.json"))?;
    assert_eq!(covers["registry_sha256"], hash);
    assert_eq!(graph["registry_sha256"], hash);
    let findings: BTreeMap<_, _> = covers["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (text(v, "slug"), v))
        .collect();
    let nodes: BTreeMap<_, _> = graph["curves"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (text(&v["icv1_identity"], "slug"), v))
        .collect();
    assert_eq!(findings.len(), curves.len());
    assert_eq!(nodes.len(), curves.len());
    for c in &mut curves {
        let slug = text(c, "slug");
        c["hyperelliptic_cover"] = findings[&slug].clone();
        let node = nodes[&slug];
        assert_eq!(node["model_json"], c["model_json"]);
        c["cover_links"] = json!(node["cover_links"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                let mut v = v.clone();
                v["cover_id"] = graph["covers"][text(&v, "cover_uid")]["cover_id"].clone();
                v["map_id"] = graph["maps"][text(&v, "map_uid")]["map_id"].clone();
                v
            })
            .collect::<Vec<_>>());
        c["standards_provenance"] = node
            .get("standards_provenance")
            .cloned()
            .filter(|v| !v.is_null())
            .unwrap_or(json!([]));
    }
    let run = "research/twist_routes_1_3_20261021/execution-v1/runs/ecbench-20261022T0012Z";
    let session = load(root.join(run).join("session.json"))?;
    let spec = load(root.join(run).join("spec.json"))?;
    let sid = &session["session_id"];
    assert!(!b["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| &s["session_id"] == sid));
    let records: Vec<Value> = fs::read_to_string(root.join(run).join("records.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    assert_eq!(records.len(), 1920);
    let mut methods: BTreeMap<_, _> = b["methods"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (text(v, "method_id"), v.clone()))
        .collect();
    let mut fbs: BTreeMap<_, _> = b["factor_bases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (text(v, "fb_id"), v.clone()))
        .collect();
    let mut agg = BTreeMap::<(String, String), Totals>::new();
    let mut statuses = BTreeMap::<String, u64>::new();
    let mut slugs = BTreeSet::new();
    let mut arms = BTreeMap::new();
    for r in &records {
        let m = &r["method"];
        let slug = text(&r["workload"]["curve"], "slug");
        let arm = text(r, "arm");
        let verified = r["outcome"]["status"] == "verified";
        *statuses.entry(text(&r["outcome"], "status")).or_default() += 1;
        slugs.insert(slug.clone());
        arms.insert(
            arm.clone(),
            json!({"arm":arm,"role":r["role"],"method":m["id"],"method_id":m["method_id"]}),
        );
        let method=methods.entry(text(m,"method_id")).or_insert_with(||json!({"method_id":m["method_id"],"method_sha256":m["method_sha256"],"method":m["id"],"family":m["family"],"params":m["params"],"sessions":[],"curves":[],"runs":0,"verified":0}));
        unique_push(&mut method["sessions"], sid.clone());
        unique_push(&mut method["curves"], json!(slug));
        method["runs"] = json!(n(method, "runs") + 1);
        method["verified"] = json!(n(method, "verified") + u64::from(verified));
        if let Some(fb) = r.get("factor_base").filter(|v| !v.is_null()) {
            let f = fbs.entry(text(fb, "fb_id")).or_insert_with(|| {
                let mut f = json!({});
                for k in [
                    "fb_id",
                    "fb_sha256",
                    "family",
                    "params",
                    "description",
                    "signed_points",
                    "abscissae",
                    "columns",
                    "dimension",
                    "points_sha256",
                ] {
                    f[k] = fb[k].clone();
                }
                f["curve"] = json!(slug);
                f["sessions"] = json!([]);
                f["methods"] = json!([]);
                f
            });
            unique_push(&mut f["sessions"], sid.clone());
            unique_push(&mut f["methods"], m["method_id"].clone());
        }
        if r["warmup"] == true {
            continue;
        }
        if m["family"] == "ic" {
            b["yields"].as_array_mut().unwrap().push(yield_row(r, sid));
        }
        let total = agg.entry((slug, arm)).or_default();
        total.n += 1;
        total.method = m["id"].clone();
        total.id = m["method_id"].clone();
        if verified {
            total.verified += 1;
            total.s += r["cost"]["s"].as_f64().unwrap();
            total.floor += r["boundaries"]["floor_s"].as_f64().unwrap();
            if r["cost"]["total_gae"] == 0
                && r["cost"]["unpriced"]
                    .as_array()
                    .is_some_and(|v| !v.is_empty())
            {
                total.zero += 1;
            }
        }
    }
    for c in &mut curves {
        for ((slug, arm), a) in &agg {
            if c["slug"] != *slug {
                continue;
            }
            let mut v = json!({"session_id":sid,"arm":arm,"method":a.method,"method_id":a.id,"runs":a.n,"verified":a.verified,
                "mean_s":if a.verified>0&&a.zero==0{json!((a.s/a.verified as f64*1e4).round_ties_even()/1e4)}else{Value::Null},
                "mean_ratio_to_floor":if a.floor>0.&&a.zero==0{json!((a.s/a.floor*1e3).round_ties_even()/1e3)}else{Value::Null}});
            if a.zero > 0 {
                v["unpriced_zero_runs"] = json!(a.zero);
                v["score_status"] = json!("unknown complete price; zero charged lower bound");
            }
            c["ecbench"].as_array_mut().unwrap().push(v);
        }
        c["ecbench"]
            .as_array_mut()
            .unwrap()
            .sort_by_key(|v| (text(v, "session_id"), text(v, "arm")));
        for f in fbs.values() {
            if f["curve"] == c["slug"] {
                unique_push(&mut c["factor_bases"], f["fb_id"].clone());
            }
        }
    }
    let mut method_rows: Vec<_> = methods.into_values().collect();
    for m in &mut method_rows {
        for k in ["sessions", "curves"] {
            m[k].as_array_mut().unwrap().sort_by_key(|v| word(v));
        }
    }
    method_rows.sort_by_key(|v| (text(v, "family"), text(v, "method"), text(v, "method_id")));
    let mut fb_rows: Vec<_> = fbs.into_values().collect();
    for f in &mut fb_rows {
        for k in ["sessions", "methods"] {
            f[k].as_array_mut().unwrap().sort_by_key(|v| word(v));
        }
    }
    fb_rows.sort_by_key(|v| (text(v, "family"), text(v, "curve"), text(v, "fb_id")));
    b["curves"] = json!(curves);
    b["methods"] = json!(method_rows);
    b["factor_bases"] = json!(fb_rows);
    b["sessions"].as_array_mut().unwrap().push(json!({"session_id":sid,"dir":run,"label":session.get("label").unwrap_or(&spec["label"]),"status":session["status"],"spec_id":session["spec_id"],"env_class_id":session["env_class_id"],"binary_sha256":session["binary_sha256"],"git_commit":session["git_commit"],"records":records.len(),"status_counts":statuses,"started_unix_ms":session["started_unix_ms"],"curves":slugs,"arms":arms.into_values().collect::<Vec<_>>()}));
    b["sessions"]
        .as_array_mut()
        .unwrap()
        .sort_by_key(|v| text(v, "dir"));
    b["yields"].as_array_mut().unwrap().sort_by_key(|v| {
        (
            text(v, "curve"),
            text(v, "fb_id"),
            text(v, "oracle"),
            text(v, "session_id"),
            text(v, "arm"),
            v["target_index"].as_i64().unwrap_or(-1),
            n(v, "round"),
        )
    });
    for key in ["curves", "methods", "factor_bases", "sessions", "yields"] {
        b["counts"][key] = json!(b[key].as_array().unwrap().len());
    }
    for p in [
        "docs/curves/registry.json",
        "docs/curves/covers.json",
        "docs/curves/cover-links.yaml",
    ] {
        b["sources"][p] = json!(sha(root.join(p))?);
    }
    b["sources"][format!("{run}/records.jsonl")] =
        json!(sha(root.join(run).join("records.jsonl"))?);
    let file = fs::File::create(output)?;
    let mut serializer = serde_json::Serializer::with_formatter(
        file,
        serde_json::ser::PrettyFormatter::with_indent(b" "),
    );
    serde::Serialize::serialize(&b, &mut serializer)?;
    use std::io::Write;
    serializer.into_inner().write_all(b"\n")?;
    println!(
        "{}",
        json!({"base_sha256":sha(base)?,"output_sha256":sha(output)?,"added_curves":added,"added_records":records.len(),"counts":b["counts"],"scientific_computations":"none; index of recorded metadata and counted-run diagnostics","legacy_generated_by":"retained for canonical site schema compatibility"})
    );
    Ok(())
}
