//! Descriptive instruction profile of the frozen known-fixture candidate.
//! No speed ratio, sample selection, new target or wall-clock admission.
use super::*;
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    candidate: PathBuf,
    valgrind: PathBuf,
    library: PathBuf,
    debug_root: PathBuf,
    singular: PathBuf,
    msolve: PathBuf,
    baseline: PathBuf,
    qualification_baseline: PathBuf,
    cpu: String,
    runtime_files: Vec<PathBuf>,
    #[serde(default)]
    expected_kernel: Option<String>,
}
fn fixed_scope(qualification: bool, last: bool) -> (&'static str, &'static str, usize) {
    if qualification { ("n3-s7", "095ce289b11b87f45964e4b4d16b5dd890640241adaf528ac85fdfe9c6a07dfa", 3) }
    else if last { ("star-last-n16-s7", "e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3", 16) }
    else { ("star-first-n16-s7", "63f5d4bcaaf41f57ed81aba878bab41900c552c8ebbfc3956b67b0d016e4350d", 16) }
}
fn kernel_matches(expected: Option<&str>, answer: &Value, last: bool) -> bool {
    if last && !expected.is_some_and(|s| ["both_updates", "both_updates_fused"].contains(&s)) { return false; }
    expected.is_none_or(|s| answer["kernel_mode"] == s)
}
pub(super) fn run_fused(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, true)
}
pub(super) fn run(prepared: &Path, out: &Path, config: &Path) -> Result<()> {
    run_impl(prepared, out, config, false)
}
fn run_impl(prepared: &Path, out: &Path, config: &Path, last: bool) -> Result<()> {
    let config_bytes = fs::read(config)?;
    let c: Config = serde_json::from_slice(&config_bytes)?;
    let manifest: Value = serde_json::from_slice(&fs::read(prepared.join("manifest.json"))?)?;
    let qualification = manifest["mode"] == "qualification";
    if !qualification && manifest["mode"] != "rust-consumer-screen" {
        return Err("known profile manifest required".into());
    }
    let (id, expected, variables) = fixed_scope(qualification, last);
    let item = manifest["cases"]
        .as_array()
        .ok_or("cases")?
        .iter()
        .find(|v| v["id"] == id)
        .ok_or("fixed profile case")?;
    let input = prepared.join(id).join("system.json");
    let bytes = fs::read(&input)?;
    let s: System = serde_json::from_slice(&bytes)?;
    if hash(&bytes) != expected
        || item["input_sha256"] != expected
        || s.prime != 2147483647
        || s.variables.len() != variables
    {
        return Err("profile input outside fixed scope".into());
    }
    fs::create_dir(out)?;
    write_new(&out.join("config.json"), &config_bytes)?;
    let baseline = if qualification {
        &c.qualification_baseline
    } else {
        &c.baseline
    };
    let mut pins = serde_json::Map::new();
    for (name, path) in [
        ("capture", std::env::current_exe()?),
        ("candidate", c.candidate.clone()),
        ("launcher", c.valgrind.clone()),
        ("engine", c.library.join("callgrind-amd64-linux")),
        ("preload", c.library.join("vgpreload_core-amd64-linux.so")),
        ("baseline", baseline.clone()),
        ("env", PathBuf::from("/usr/bin/env")),
    ] {
        pins.insert(name.into(), json!({"path":path,"sha256":file_hash(&path)?}));
    }
    if c.runtime_files.is_empty() || c.runtime_files.len() > 64 {
        return Err("bounded runtime pin list required".into());
    }
    for (i, path) in c.runtime_files.iter().enumerate() {
        pins.insert(
            format!("runtime-{i}"),
            json!({"path":path,"sha256":file_hash(path)?}),
        );
    }
    save(
        &out.join("pre-call.json"),
        &json!({"pins":pins,"qualification":qualification,"fixed_last_entry":last,"expected_kernel":c.expected_kernel,"input_sha256":expected,"scope":"instruction diagnosis; generation excluded, no speed comparison"}),
    )?;
    let r = Runner {
        worker: c.candidate.clone(),
        singular: c.singular.clone(),
        msolve: c.msolve.clone(),
        cpu: c.cpu.clone(),
        worker_hash: file_hash(&c.candidate)?,
        global: Instant::now(),
        case: Instant::now(),
        global_limit: Duration::from_secs(600),
        case_limit: Duration::from_secs(180),
    };
    let result = (|| -> Result<Value> {
        let profile = out.join("callgrind.out");
        let proof = out.join("proof");
        let args = vec![
            format!("VALGRIND_LIB={}", c.library.display()),
            c.valgrind.display().to_string(),
            "--tool=callgrind".into(),
            format!("--extra-debuginfo-path={}", c.debug_root.display()),
            format!("--callgrind-out-file={}", profile.display()),
            format!("--log-file={}", out.join("valgrind.log").display()),
            "--dump-instr=no".into(),
            "--cache-sim=no".into(),
            "--branch-sim=no".into(),
            "--error-exitcode=86".into(),
            c.candidate.display().to_string(),
            input.display().to_string(),
            "compact-sparse".into(),
            r.singular.display().to_string(),
            r.msolve.display().to_string(),
            proof.display().to_string(),
        ];
        let (status, stdout, _) =
            r.process(Path::new("/usr/bin/env"), &args, None, &out.join("call"))?;
        if !status.success() || !fs::read(out.join("call.stderr"))?.is_empty() {
            return Err("instrumented invocation failed/child diagnostics".into());
        }
        let answer: Value = serde_json::from_slice(&stdout)?;
        if !kernel_matches(c.expected_kernel.as_deref(), &answer, last) { return Err("profile kernel differs from fixed protocol".into()); }
        let prior: Value = serde_json::from_slice(&fs::read(baseline)?)?;
        let basis = native_basis(&answer["basis"], s.variables.len())?;
        if basis != native_basis(&prior["basis"], s.variables.len())?
            || answer["root_api"] != true
            || answer["native_product_criterion"] != true
            || answer["completion_exports"] != 0
            || answer["constructor"]["subset_succeeded"] != true
        {
            return Err("profiled answer/mode differs from frozen baseline".into());
        }
        let text = fs::read_to_string(&profile)?;
        let counts = compact_speed_analysis::callgrind::parse(&text)?;
        if !counts
            .functions
            .iter()
            .any(|f| f.name.contains("certified_relations"))
        {
            return Err("profile lacks native function symbols".into());
        }
        Ok(
            json!({"basis_sha256":basis_hash(&basis),"complete_basis_equals_frozen":true,"answer":answer,"profile":counts,"qualification":qualification,"scope":"guest Ir counts including startup; inclusive functions overlap; not native cycles or wall speed"}),
        )
    })();
    let unchanged = pins
        .values()
        .map(|p| -> Result<bool> {
            Ok(file_hash(Path::new(p["path"].as_str().ok_or("pin path")?))?
                == p["sha256"].as_str().ok_or("pin hash")?)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .all(|v| v);
    let failed = result.is_err();
    let (value, failure) = match result {
        Ok(v) => (v, Value::Null),
        Err(e) => (Value::Null, json!(e.to_string())),
    };
    save(
        &out.join("summary.json"),
        &json!({"schema":"native-callgrind-profile/v1","qualification":qualification,"fixed_last_entry":last,"expected_kernel":c.expected_kernel,"identities_unchanged":unchanged,"input_sha256":expected,"result":value,"failure":failure,"scope":"single descriptive instrumented profile; no speedup or admission"}),
    )?;
    if failed || !unchanged {
        return Err("profile incomplete/changed pins; raw retained".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_last_profile_keeps_old_first_and_n3_scope() {
        assert_eq!(fixed_scope(false,true), ("star-last-n16-s7","e4d22ea93228f207b87640813fb925e15973d8ee43d5f966a4d98a63917cbaf3",16));
        assert_eq!(fixed_scope(false,false).0,"star-first-n16-s7");
        assert_eq!(fixed_scope(true,true),fixed_scope(true,false));
        assert_eq!(fixed_scope(true,true).2,3);
    }
    #[test]
    fn matched_profile_requires_exact_known_mode() {
        for mode in ["both_updates","both_updates_fused"] {
            assert!(kernel_matches(Some(mode),&json!({"kernel_mode":mode}),true));
            assert!(!kernel_matches(Some(mode),&json!({"kernel_mode":"other"}),true));
            assert!(!kernel_matches(Some(mode),&json!({}),true));
        }
        assert!(!kernel_matches(None,&json!({}),true));
        assert!(!kernel_matches(Some("other"),&json!({"kernel_mode":"other"}),true));
        assert!(kernel_matches(None,&json!({}),false));
    }
}
