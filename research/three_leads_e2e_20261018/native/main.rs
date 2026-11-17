mod analysis;
mod certificates;
mod coverage;
mod exhaustions;
mod order_cohort;
mod report;
use serde_json::{json, Value};
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};
use three_leads_engine::*;
fn write(p: impl AsRef<Path>, v: &Value) {
    fs::write(
        p,
        {
            let mut buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b" ");
            let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
            serde::Serialize::serialize(v, &mut serializer).unwrap();
            String::from_utf8(buf).unwrap()
        } + "\n",
    )
    .unwrap();
}
fn curve(i: &StudyInstance) -> Value {
    json!({"kind":"three_leads","p":i.curve.f.p,"n":i.curve.f.n,"modulus":i.curve.f.modulus,"a":if i.curve.binary{i.curve.c[2]}else{i.curve.c[1]},"b":i.curve.c[0],"group_order":i.order,"r":i.r,"gx":i.generator.x,"gy":i.generator.y,"family":i.family,"parameter":i.parameter})
}
fn arm(name: &str, id: &str, params: Value) -> Value {
    json!({"name":name,"role":if name=="rho_neg"{"reference"}else{"candidate"},"method":{"id":id,"params":params}})
}
fn spec(label: &str, curves: Vec<Value>, targets: u64, arms: Vec<Value>) -> Value {
    json!({"schema":"ecbench.spec/v1","label":label,"description":"Fresh three-leads counts-only experiment; all cold phases; walls unqualified","workloads":{"curves":curves,"targets_per_curve":targets,"target_seed":202610181260u64},"arms":arms,"measurement":{"rounds":1,"warmup":0,"order":"shuffle","seed":2026101801u64,"isolation_required":"L2","timeout_seconds":1200}})
}
fn generate_genus(dir: &str) {
    let dir = Path::new(dir);
    fs::create_dir(dir).unwrap();
    let mut all = vec![];
    let mut rejects = vec![];
    for bits in [16, 20, 24, 28, 32, 36, 40] {
        let mut fixtures = vec![];
        let start = 3u64 << (bits / 2 - 2);
        let mut p = start + ((3 + 4 - start % 4) % 4);
        while fixtures.len() < 3 {
            if is_prime(p) {
                if let Some(i) = scholten_instance(p, 20261018 + p) {
                    eprintln!("genus bits={bits} p={p} r={}", i.r);
                    let s = Scholten::new(p);
                    let mut rng = Rng(p);
                    for _ in 0..32 {
                        let a = rng.next() % i.r;
                        let b = rng.next() % i.r;
                        let x = i.curve.mul(i.generator, a);
                        let y = i.curve.mul(i.generator, b);
                        let tx = s.tau(s.from_short(x));
                        assert_eq!(s.norm(tx), s.e.mul(s.from_short(x), 2));
                        assert_eq!(
                            s.tau(s.from_short(i.curve.add(x, y))),
                            s.j.add(tx, s.tau(s.from_short(y)))
                        );
                    }
                    fixtures.push(i);
                } else {
                    rejects.push(json!({"bits":bits,"p":p,"reason":"no Hasse-unique usable odd-prime subgroup witness"}));
                }
            }
            p += 4;
        }
        let cap = (100 * fixtures.iter().map(|i| i.curve.f.p).max().unwrap() + 10000).to_string();
        let curves: Vec<_> = fixtures.iter().map(curve).collect();
        let arms = vec![
            arm(
                "rho_plain",
                "rho.three_leads_plain_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "rho_neg",
                "rho.three_leads_negation_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "jac_rho",
                "rho.three_leads_jacobian_v1",
                json!({"trial_cap":cap}),
            ),
            arm(
                "genus_ic",
                "ic.three_leads_genus2_v1",
                json!({"trial_cap":cap,"independent":"0","unique_relations":"1"}),
            ),
        ];
        write(
            dir.join(format!("genus-{bits}.json")),
            &spec(
                &format!("three-leads-genus-{bits}"),
                curves.clone(),
                60,
                arms,
            ),
        );
        for (name, ind, unique) in [("independent", "1", "1"), ("unfiltered", "0", "0")] {
            write(
                dir.join(format!("genus-{bits}-{name}.json")),
                &spec(
                    &format!("three-leads-genus-{bits}-{name}"),
                    curves.clone(),
                    1,
                    vec![arm(
                        name,
                        "ic.three_leads_genus2_v1",
                        json!({"trial_cap":cap,"independent":ind,"unique_relations":unique}),
                    )],
                ),
            );
        }
        all.extend(fixtures);
    }
    write(
        dir.join("fixtures.json"),
        &serde_json::to_value(all).unwrap(),
    );
    write(dir.join("rejected.json"), &json!(rejects));
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str).unwrap_or("smoke") {
        "report" => report::generate(&args[2]),
        "publish-dashboard" => report::publish(&args[2]),
        "random-x-controls" => coverage::random_x_controls(&args[2], &args[3]),
        "supplementary-controls" => coverage::supplementary_controls(&args[2]),
        "replay-exhaustions" => exhaustions::replay(&args[2], &args[3]),
        "instruction-AA" => analysis::instruction_aa(&args[2]),
        "certificates" => certificates::generate(&args[2]),
        "generate-order-genus" => order_cohort::generate(&args[2]),
        "verify-oracles" => coverage::verify_oracles(&args[2], &args[3]),
        "operation-counts" => {
            let fixtures: Vec<StudyInstance> =
                serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
            let mut rows = vec![];
            for i in fixtures {
                let s = Scholten::new(i.curve.f.p);
                let mut rng = Rng(20261018 + i.curve.f.p);
                let mut jac = vec![];
                let mut ec = vec![];
                for _ in 0..100 {
                    let p = i.curve.mul(i.generator, 1 + rng.next() % (i.r - 1));
                    let q = i.curve.mul(i.generator, 1 + rng.next() % (i.r - 1));
                    let a = s.tau(s.from_short(p));
                    let b = s.tau(s.from_short(q));
                    s.j.f.reset();
                    let d = s.j.add(a, b);
                    let jc = s.j.f.counts();
                    assert!(s.j.valid(d));
                    i.curve.f.reset();
                    let r = i.curve.add(p, q);
                    let count = i.curve.f.counts();
                    assert!(i.curve.valid(r));
                    jac.push(jc);
                    ec.push(count);
                }
                rows.push(json!({"p":i.curve.f.p,"r":i.r,"jacobian":jac,"source":ec}));
            }
            write(&args[3], &json!(rows));
        }
        "merge-registry" => {
            let original = fs::read_to_string(&args[2]).unwrap();
            let old: Value = serde_json::from_str(&original).unwrap();
            let new: Value = serde_json::from_str(&fs::read_to_string(&args[3]).unwrap()).unwrap();
            let boundary = original.rfind("\n ]").unwrap();
            let mut result = original[..boundary].to_owned();
            let mut added = 0;
            for row in new["curves"].as_array().unwrap() {
                if !old["curves"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|v| v["slug"] == row["slug"])
                {
                    let mut bytes = vec![];
                    let mut ser = serde_json::Serializer::with_formatter(
                        &mut bytes,
                        serde_json::ser::PrettyFormatter::with_indent(b" "),
                    );
                    serde::Serialize::serialize(row, &mut ser).unwrap();
                    let row = String::from_utf8(bytes).unwrap();
                    result.push_str(",\n");
                    for (n, line) in row.lines().enumerate() {
                        if n > 0 {
                            result.push('\n');
                        }
                        result.push_str("  ");
                        result.push_str(line);
                    }
                    added += 1;
                }
            }
            result.push_str(&original[boundary..]);
            let check: Value = serde_json::from_str(&result).unwrap();
            assert_eq!(
                check["curves"].as_array().unwrap().len(),
                old["curves"].as_array().unwrap().len() + added
            );
            fs::write(&args[3], result).unwrap();
        }
        "format-json" => write(
            &args[2],
            &serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap(),
        ),
        "analyze-coverage" => analysis::coverage(&args[2], &args[3]),
        "analyze-sessions" => analysis::sessions(&args[2], &args[3]),
        "generate-coverage" => coverage::generate(&args[2]),
        "generate-genus" => generate_genus(&args[2]),
        "smoke" => {
            for p in [19, 23, 31, 43, 59, 67, 83, 103] {
                let s = Scholten::new(p);
                println!(
                    "{}",
                    json!({"p":p,"order":exhaustive_order(&s.e),"f":s.j.polynomial})
                );
            }
        }
        _ => panic!("unknown study command"),
    }
}
mod hash {
    pub mod sha256 {
        pub fn sha256(bytes: &[u8]) -> [u8; 32] {
            use sha2::Digest;
            sha2::Sha256::digest(bytes).into()
        }
    }
}
