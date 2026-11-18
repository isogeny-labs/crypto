//! New fixed-family paired protocol, without modifying older admission rules.
use super::sparse_capture::{fused_sample, paid_sum};
use super::*;
use compact_speed_analysis::{paired_statistics, Statistics};
const NAMES: [&str; 3] = [
    "f4-regeneration",
    "expanded-regeneration",
    "root-sparse-both",
];
const CYCLE: [[usize; 3]; 6] = [
    [0, 1, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
    [1, 0, 2],
    [0, 2, 1],
];
const INPUT_HASH: &str = "63f5d4bcaaf41f57ed81aba878bab41900c552c8ebbfc3956b67b0d016e4350d";
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    worker: PathBuf,
    legacy_cli: PathBuf,
    prior_fused: PathBuf,
    root_baseline: PathBuf,
    root_consumer: PathBuf,
    root_native: PathBuf,
    root_both: PathBuf,
    mature: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    cpu: String,
}
fn includes_one(s: &Statistics) -> bool {
    s.percentile_95[0] <= 1. && s.percentile_95[1] >= 1.
}
fn values(panel: &Value) -> Result<[[u64; 2]; 3]> {
    let samples = panel["samples"].as_array().ok_or("panel samples")?;
    if samples.len() != 3 {
        return Err("paired roster length".into());
    }
    let digest = samples[0]["basis_sha256"].as_str().ok_or("basis digest")?;
    if digest.len() != 64 {
        return Err("basis digest length".into());
    }
    let mut out = [[0; 2]; 3];
    for (i, s) in samples.iter().enumerate() {
        if s["pipeline"] != NAMES[i] || s["status"] != "complete" || s["basis_sha256"] != digest {
            return Err("incomplete/mismatched paired panel".into());
        }
        out[i] = [
            s["pipeline_work_ns"]
                .as_u64()
                .filter(|v| *v > 0)
                .ok_or("work cost")?,
            s["paid_wall_ns"]
                .as_u64()
                .filter(|v| *v > 0)
                .ok_or("paid cost")?,
        ];
        if out[i][1] < out[i][0] {
            return Err("paid cost below work".into());
        }
    }
    if samples[2]["answer"]["root_api"] != true
        || samples[2]["answer"]["native_product_criterion"] != true
        || samples[2]["answer"]["constructor"]["subset_succeeded"] != true
    {
        return Err("candidate is not the admitted fast path".into());
    }
    Ok(out)
}
fn aa_result(pairs: &[[Value; 2]]) -> Result<(Value, bool)> {
    if pairs.len() != 7 {
        return Err("exactly7 A/A pairs required".into());
    }
    let mut report = Vec::new();
    let mut all = true;
    for arm in 0..3 {
        let mut ratios = Vec::new();
        for [a, b] in pairs {
            let a = values(a)?;
            let b = values(b)?;
            ratios.push([
                a[arm][0] as f64 / b[arm][0] as f64,
                a[arm][1] as f64 / b[arm][1] as f64,
            ]);
        }
        let stats = paired_statistics(&ratios)?;
        let admitted = stats.iter().all(includes_one);
        all &= admitted;
        report.push(json!({"arm":NAMES[arm],"pipeline_work":stats[0],"full_paid":stats[1],"admitted":admitted}));
    }
    Ok((json!(report), all))
}
fn main_result(blocks: &[Value]) -> Result<Value> {
    if blocks.len() != 21 {
        return Err("exactly21 blocks; no replacement".into());
    }
    let mut ratios = Vec::new();
    for (b, panel) in blocks.iter().enumerate() {
        if panel["order"] != json!(CYCLE[b % 6]) {
            return Err("wrong counterbalanced order".into());
        }
        let v = values(panel)?;
        ratios.push([
            v[0][0] as f64 / v[2][0] as f64,
            v[1][0] as f64 / v[2][0] as f64,
            v[0][1] as f64 / v[2][1] as f64,
            v[1][1] as f64 / v[2][1] as f64,
        ]);
    }
    let s = paired_statistics(&ratios)?;
    Ok(
        json!({"comparisons":[{"control":NAMES[0],"pipeline_work":s[0],"full_paid":s[2]},{"control":NAMES[1],"pipeline_work":s[1],"full_paid":s[3]}],"paired_numeric_15x":s[2].adjusted_lower_1_over_720>15.&&s[3].adjusted_lower_1_over_720>15.,"paired_numeric_10x":s[2].adjusted_lower_1_over_720>10.&&s[3].adjusted_lower_1_over_720>10.,"family_confirmed":false,"execution_admission_verified":false,"scope":"named original paid workflows; not fastest-control/kernel claim"}),
    )
}
struct Panel<'a> {
    r: &'a Runner,
    config: &'a Config,
    input: &'a Path,
    item: &'a Value,
}
impl Panel<'_> {
    fn capture(&self, out: &Path, order: [usize; 3]) -> Result<Value> {
        fs::create_dir(out)?;
        let bytes = fs::read(self.input.join("system.json"))?;
        let input_hash = hash(&bytes);
        if self.item["input_sha256"] != input_hash {
            return Err("changed paired input".into());
        }
        let system: System = serde_json::from_slice(&bytes)?;
        let prep = self.item["preparation_ns"]
            .as_u64()
            .filter(|v| *v > 0)
            .ok_or("positive prep charge")?;
        let (basis, setup) = self.r.references(self.input, out, &system, prep)?;
        let mut samples = vec![Value::Null; 3];
        for arm in order {
            let start = Instant::now();
            let result = if arm == 2 {
                fused_sample(
                    self.r,
                    &self.config.root_both,
                    self.input,
                    &out.join(NAMES[arm]),
                    "compact-sparse",
                    system.variables.len(),
                    Some(&basis),
                )
            } else {
                self.r
                    .sample(
                        if arm == 0 { Arm::F4 } else { Arm::Certified },
                        self.input,
                        &out.join(NAMES[arm]),
                        system.variables.len(),
                        &basis,
                        &input_hash,
                    )
                    .map(|s| json!({"status":"complete","basis_sha256":s.basis_sha256,"sample":s}))
            };
            let work = ns(start).max(1);
            samples[arm] = match result {
                Ok(mut value) => {
                    value["pipeline"] = json!(NAMES[arm]);
                    value["pipeline_work_ns"] = json!(work);
                    value["paid_wall_ns"] =
                        json!(paid_sum(work, if arm == 2 { prep } else { setup }, 0)?);
                    value
                }
                Err(e) => {
                    json!({"pipeline":NAMES[arm],"status":"failed","reason":e.to_string(),"attempt_elapsed_ns":work})
                }
            };
        }
        let start = Instant::now();
        let equal = samples
            .iter()
            .all(|s| s["status"] == "complete" && s["basis_sha256"] == basis_hash(&basis));
        let equality = ns(start).max(1);
        for sample in &mut samples {
            if let Some(paid) = sample["paid_wall_ns"].as_u64() {
                sample["paid_wall_ns"] = json!(paid_sum(paid, equality, 0)?);
                sample["cross_arm_equality_ns"] = json!(equality);
            }
        }
        let panel = json!({"order":order,"input_sha256":input_hash,"preparation_ns":prep,"regeneration_setup_ns":setup,"all_complete_bases_equal":equal,"samples":samples});
        save(&out.join("panel.json"), &panel)?;
        values(&panel)?;
        Ok(panel)
    }
}
const LAST_INPUT_HASH: &str = "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3";
fn admitted_item(manifest: &Value, last: bool) -> Result<&Value> {
    let qualification = manifest["mode"] == "qualification";
    let (id, digest) = if last {
        ("star-last-n16-s7", LAST_INPUT_HASH)
    } else {
        ("star-first-n16-s7", INPUT_HASH)
    };
    let item = manifest["cases"]
        .as_array()
        .ok_or("roster")?
        .iter()
        .find(|v| {
            if qualification {
                v["id"] == "n3-s7"
            } else {
                v["id"] == id
            }
        })
        .ok_or("fixed admitted case missing")?;
    if !qualification
        && (manifest["mode"] != "rust-consumer-screen"
            || item["input_sha256"] != digest
            || item["variables"] != 16
            || item["seed"] != 7)
    {
        return Err("paired fixture outside admitted scope".into());
    }
    Ok(item)
}
pub(super) fn run(prepared: &Path, out: &Path, config_path: &Path) -> Result<()> {
    run_impl(prepared, out, config_path, false)
}
pub(super) fn run_last(prepared: &Path, out: &Path, config_path: &Path) -> Result<()> {
    run_impl(prepared, out, config_path, true)
}
fn run_impl(prepared: &Path, out: &Path, config_path: &Path, last: bool) -> Result<()> {
    let config_bytes = fs::read(config_path)?;
    let c: Config = serde_json::from_slice(&config_bytes)?;
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    let item = admitted_item(&manifest, last)?;
    let input = prepared.join(item["id"].as_str().ok_or("case id")?);
    let mut identities = serde_json::Map::new();
    for (name, path) in [
        ("adapter", std::env::current_exe()?),
        ("worker", c.worker.clone()),
        ("candidate", c.root_both.clone()),
        ("singular", c.singular.clone()),
        ("msolve", c.msolve.clone()),
        (
            "standard_library",
            PathBuf::from("/workspace/native-cas/share/singular/LIB/standard.lib"),
        ),
    ] {
        identities.insert(name.into(), json!({"path":path,"sha256":file_hash(&path)?}));
    }
    // Retain other config identities without using them as paired baselines.
    let _screen_only = (
        &c.legacy_cli,
        &c.prior_fused,
        &c.root_baseline,
        &c.root_consumer,
        &c.root_native,
        &c.mature,
    );
    fs::create_dir(out)?;
    write_new(&out.join("config.json"), &config_bytes)?;
    save(
        &out.join("pre-call.json"),
        &json!({"identities":identities,"input_sha256":item["input_sha256"],"preparation_charge_source":prepared.join("manifest.json"),"config_sha256":hash(&config_bytes),"qualification":qualification,"scope":"known-fixture matched paid calls; L0 unsigned"}),
    )?;
    let mut r = Runner {
        worker: c.worker.clone(),
        singular: c.singular.clone(),
        msolve: c.msolve.clone(),
        cpu: c.cpu.clone(),
        worker_hash: identities["worker"]["sha256"]
            .as_str()
            .ok_or("worker pin")?
            .into(),
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let mut warmup = Value::Null;
    let mut aa_pairs = Vec::new();
    let mut blocks = Vec::new();
    let mut failure = None;
    let mut aa = Value::Null;
    let mut result = Value::Null;
    let attempted = (|| -> Result<()> {
        r.case = Instant::now();
        warmup = Panel {
            r: &r,
            config: &c,
            input: &input,
            item,
        }
        .capture(&out.join("warmup"), CYCLE[0])?;
        if qualification {
            return Ok(());
        }
        for pair in 0..7 {
            let labels = if pair % 2 == 0 { [0, 1] } else { [1, 0] };
            let mut values = [Value::Null, Value::Null];
            for label in labels {
                r.case = Instant::now();
                values[label] = Panel {
                    r: &r,
                    config: &c,
                    input: &input,
                    item,
                }
                .capture(
                    &out.join(format!(
                        "aa-{pair:02}-{}",
                        if label == 0 { "a" } else { "b" }
                    )),
                    CYCLE[0],
                )?;
            }
            aa_pairs.push(values);
        }
        let (report, admitted) = aa_result(&aa_pairs)?;
        aa = report;
        if !admitted {
            return Err("A/A rejected; main blocks not run".into());
        }
        for block in 0..21 {
            r.case = Instant::now();
            blocks.push(
                Panel {
                    r: &r,
                    config: &c,
                    input: &input,
                    item,
                }
                .capture(&out.join(format!("block-{block:02}")), CYCLE[block % 6])?,
            );
            eprintln!("paired block{} retained", block + 1);
        }
        result = main_result(&blocks)?;
        Ok(())
    })();
    if let Err(e) = attempted {
        failure = Some(e.to_string());
    }
    let mut unchanged = true;
    for pin in identities.values() {
        unchanged &= file_hash(Path::new(pin["path"].as_str().ok_or("pin path")?))?
            == pin["sha256"].as_str().ok_or("pin hash")?;
    }
    save(
        &out.join("summary.json"),
        &json!({"schema":"actual-root-paired/v1","qualification":qualification,"input_sha256":item["input_sha256"],"identities_unchanged":unchanged,"warmup":warmup,"aa_pairs":aa_pairs,"aa":aa,"blocks":blocks,"result":result,"failure":failure,"scope":"unsigned L0, no independent/fresh-family confirmation"}),
    )?;
    if !unchanged {
        return Err("changed paired identity".into());
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_selection_rejects_wrong_hash_shape_seed_and_first_only_roster() {
        let first =
            json!({"id":"star-first-n16-s7","input_sha256":INPUT_HASH,"variables":16,"seed":7});
        let selected =
            json!({"id":"star-last-n16-s7","input_sha256":LAST_INPUT_HASH,"variables":16,"seed":7});
        let good = json!({"mode":"rust-consumer-screen","cases":[first,selected]});
        assert_eq!(
            admitted_item(&good, true).unwrap()["id"],
            "star-last-n16-s7"
        );
        assert_eq!(
            admitted_item(&good, false).unwrap()["id"],
            "star-first-n16-s7"
        );
        assert!(admitted_item(
            &json!({"mode":"rust-consumer-screen","cases":[first]}),
            true
        )
        .is_err());
        for (key, value) in [
            ("input_sha256", json!(INPUT_HASH)),
            ("variables", json!(12)),
            ("seed", json!(8)),
        ] {
            let mut bad = good.clone();
            bad["cases"][1][key] = value;
            assert!(admitted_item(&bad, true).is_err());
        }
        let mut bad = good;
        bad["mode"] = json!("other");
        assert!(admitted_item(&bad, true).is_err());
        assert!(admitted_item(
            &json!({"mode":"qualification","cases":[{"id":"n3-s7"}]}),
            true
        )
        .is_ok());
    }
    fn panel(work: [u64; 3], paid: [u64; 3]) -> Value {
        json!({"order":CYCLE[0],"samples":(0..3).map(|i|json!({"pipeline":NAMES[i],"status":"complete","basis_sha256":"a".repeat(64),"pipeline_work_ns":work[i],"paid_wall_ns":paid[i],"answer":{"root_api":true,"native_product_criterion":true,"constructor":{"subset_succeeded":true}}})).collect::<Vec<_>>()})
    }
    #[test]
    fn paid_boundary_threshold_and_order_are_not_native_only() {
        let good = panel([10, 10, 1], [20, 20, 1]);
        let mut blocks = vec![good.clone(); 21];
        for (i, b) in blocks.iter_mut().enumerate() {
            b["order"] = json!(CYCLE[i % 6]);
        }
        let r = main_result(&blocks).unwrap();
        assert_eq!(r["paired_numeric_15x"], true);
        let mut bad = blocks.clone();
        bad[5]["order"] = json!(CYCLE[0]);
        assert!(main_result(&bad).is_err());
        for b in &mut blocks {
            b["samples"][1]["paid_wall_ns"] = json!(15);
        }
        assert_eq!(main_result(&blocks).unwrap()["paired_numeric_15x"], false);
        assert_eq!(main_result(&blocks).unwrap()["paired_numeric_10x"], true);
    }
    #[test]
    fn aa_rejects_bias_and_incomplete_primary() {
        let good = panel([10, 10, 10], [20, 20, 20]);
        let pairs = vec![[good.clone(), good.clone()]; 7];
        assert!(aa_result(&pairs).unwrap().1);
        let biased = panel([20, 20, 20], [40, 40, 40]);
        assert!(!aa_result(&vec![[biased, good.clone()]; 7]).unwrap().1);
        let mut bad = good;
        bad["samples"][0]["status"] = json!("failed");
        assert!(values(&bad).is_err());
        assert!(paired_statistics(&[[0., 1.]; 7]).is_err());
    }
}
