use crate::*;
use serde_json::Map;

fn optional_text(path: &Path) -> Value {
    fs::read_to_string(path)
        .ok()
        .map(|v| json!(v.trim()))
        .unwrap_or(Value::Null)
}

pub fn map_row(
    index: usize,
    case: &Case,
    arm: &str,
    native: &Value,
    status: &Value,
) -> Result<Value> {
    require(
        native["arm"] == arm
            && native["draw"] == case.draw
            && native["guesses"] == case.guesses
            && native["branch"] == case.branch,
        "native row identity mismatch",
    )?;
    let kind = native["classification"]
        .as_str()
        .ok_or("missing native classification")?;
    let outcome = if kind == "verified" {
        "succeeded"
    } else if kind == "timeout" {
        "timeout"
    } else if status["interrupted_by_restore"] == true {
        "interrupted"
    } else {
        "failed"
    };
    let completed = matches!(outcome, "succeeded" | "failed");
    let mut counted = Map::new();
    if kind == "verified" {
        for name in ["closure_rank", "full_columns", "plain_rank", "hky_rounds"] {
            let value = native["replay"][name]
                .as_u64()
                .ok_or("verified native row lacks an exact metric")?;
            counted.insert(name.to_owned(), json!(value));
        }
    }
    // The canonical seed table uses numeric instance indices, not backend tuple labels.
    // Preserve the actual tuple in run_id and backend_ref, with the frozen index map.
    Ok(json!({
        "run_id":format!("{}-{arm}",case.id),"arm":arm,"instance_id":index.to_string(),"seed":0,
        "replicate":0,"warmup":false,"status":outcome,"outcome_class":if completed {"completed"} else {"not_completed"},
        "attempts":1,"resumed":0,"checkpoint":null,
        "measurements":{"counted":counted,"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},
        "isolation":{"tier":"L0","contended":null,"blockers":[]},
        "verification":{"status":if kind=="verified" {"verified"} else {"not_run"},"verifier":if kind=="verified" {json!("frozen native ic_guess_completed_audit replay-summary")} else {Value::Null}},
        "exclusion":{"excluded":false,"rule":null},"artifacts":[],"started_at":null,"finished_at":null,"worker":null,
        "backend_ref":case.id
    }))
}

pub fn flow(rows: &[Value]) -> Value {
    json!({"planned":rows.len(),"attempted":rows.iter().filter(|r| r["attempts"].as_u64().unwrap_or(0)>=1).count(),
        "completed":rows.iter().filter(|r|r["outcome_class"]=="completed").count(),
        "not_completed":rows.iter().filter(|r|r["outcome_class"]=="not_completed").count(),
        "interrupted":rows.iter().filter(|r|r["status"]=="interrupted").count(),
        "resumed":rows.iter().filter(|r|r["resumed"].as_u64().unwrap_or(0)>0).count(),"excluded":[]})
}

pub fn validate_grid(rows: &[Value], cases: &[Case], complete: bool) -> Result<()> {
    if complete {
        require(
            rows.len() == cases.len() * 2,
            "incomplete Cartesian execution grid",
        )?;
    }
    let mut seen = BTreeSet::new();
    for row in rows {
        let arm = row["arm"].as_str().ok_or("missing row arm")?;
        require(matches!(arm, "hky" | "hky-aa"), "unexpected row arm")?;
        let index = row["instance_id"]
            .as_str()
            .ok_or("missing numeric instance identity")?
            .parse::<usize>()?;
        let case = cases
            .get(index)
            .ok_or("row outside the frozen instance map")?;
        require(
            row["backend_ref"] == case.id,
            "backend tuple differs from the frozen numeric index map",
        )?;
        require(
            row["run_id"] == format!("{}-{arm}", case.id),
            "run identity differs from the tuple map",
        )?;
        require(
            row["replicate"] == 0 && row["seed"] == 0 && row["warmup"] == false,
            "row differs from frozen seed/replicate policy",
        )?;
        require(
            row["measurements"]["wall_valid"] == false,
            "clock cannot be promoted",
        )?;
        require(
            seen.insert((arm.to_owned(), index)),
            "duplicate execution cell",
        )?;
        let succeeded = row["status"] == "succeeded";
        if !succeeded {
            require(
                row["measurements"]["counted"].as_object().map(Map::len) == Some(0),
                "censored row contains invented exact metrics",
            )?;
        }
        let completed = matches!(row["status"].as_str(), Some("succeeded" | "failed"));
        require(
            row["outcome_class"]
                == if completed {
                    "completed"
                } else {
                    "not_completed"
                },
            "outcome class differs from canonical status semantics",
        )?;
    }
    Ok(())
}

impl Context {
    pub fn export_directory(&self) -> Result<PathBuf> {
        let directory = self.study.join(relative_path(
            self.native()["export_directory"]
                .as_str()
                .ok_or("no export directory")?,
        )?);
        require(
            directory.parent() == Some(self.study.join("exports").as_path()),
            "export directory has an unexpected depth",
        )?;
        Ok(directory)
    }

    pub fn validate_export(&self, export: &Value, directory: &Path) -> Result<()> {
        require(
            read_json(&self.protocol_path)? == self.protocol,
            "protocol file changed before final export validation",
        )?;
        validate_schema(
            export,
            Path::new(
                self.native()["export_schema"]["path"]
                    .as_str()
                    .ok_or("no export schema")?,
            ),
        )?;
        require(
            export["protocol"]["sha256"] == self.protocol["freeze"]["protocol_sha256"]
                && export["protocol"]["protocol_id"] == self.protocol["protocol_id"]
                && export["protocol"]["version"] == self.protocol["version"],
            "export does not bind this frozen protocol",
        )?;
        let rows = export["runs"].as_array().ok_or("export lacks runs")?;
        validate_grid(rows, &self.cases, true)?;
        require(
            export["flow"] == flow(rows),
            "export flow differs from its execution rows",
        )?;
        require(
            export["analysis"]["script"]["sha256"]
                == self.protocol["analysis_plan"]["script"]["sha256"],
            "export analysis hash differs from frozen analysis",
        )?;
        require(
            export["arms"].as_array().map(Vec::len) == Some(2),
            "export arm count changed",
        )?;
        for (index, arm) in export["arms"]
            .as_array()
            .ok_or("export lacks arms")?
            .iter()
            .enumerate()
        {
            let declared = &self.protocol["arms"][index];
            for field in ["id", "role"] {
                require(
                    arm[field] == declared[field],
                    "export arm identity or role changed",
                )?;
            }
            for field in ["commit", "binary_sha256"] {
                require(
                    arm[field] == declared["instrument"][field],
                    "export instrument differs from the frozen scientific binary",
                )?;
            }
        }
        let mut entries: Vec<&Value> = export["files"]
            .as_array()
            .ok_or("export lacks file index")?
            .iter()
            .collect();
        entries.extend(
            export["analysis"]["outputs"]
                .as_array()
                .ok_or("export lacks analysis outputs")?,
        );
        entries.push(&export["analysis"]["script"]);
        for row in rows {
            entries.extend(
                row["artifacts"]
                    .as_array()
                    .ok_or("row lacks artifact list")?,
            );
        }
        for entry in entries {
            let path = directory
                .join(entry["path"].as_str().ok_or("file entry lacks path")?)
                .canonicalize()?;
            require(
                path.starts_with(&self.root),
                "export file lies outside the research repository",
            )?;
            require(
                sha_file(&path)? == entry["sha256"].as_str().ok_or("file entry lacks digest")?,
                "export file changed since hashing",
            )?;
            require(
                fs::metadata(path)?.len()
                    == entry["bytes"].as_u64().ok_or("file entry lacks size")?,
                "export file size changed",
            )?;
        }
        let host = directory
            .join(export["host"]["path"].as_str().ok_or("host path missing")?)
            .canonicalize()?;
        require(
            host.starts_with(&self.root) && sha_file(&host)? == export["host"]["sha256"],
            "export host manifest changed",
        )?;
        require(
            read_json(&host)?["summary"] == export["host"]["summary"],
            "export host summary differs from its source record",
        )?;
        Ok(())
    }
    pub fn host(&self) -> Result<()> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let cpu = cpuinfo.lines().find_map(|s| {
            s.strip_prefix("model name")
                .and_then(|s| s.split_once(':').map(|(_, v)| v.trim()))
        });
        let logical = cpuinfo
            .lines()
            .filter(|s| s.starts_with("processor") && s.contains(':'))
            .count();
        let memory = fs::read_to_string("/proc/meminfo").ok().and_then(|s| {
            s.lines().find_map(|line| {
                line.strip_prefix("MemTotal:")
                    .and_then(|s| s.split_whitespace().next()?.parse::<u64>().ok())
                    .map(|k| k * 1024)
            })
        });
        let kernel = Command::new("uname").arg("-r").output()?;
        require(kernel.status.success(), "could not read kernel metadata")?;
        let summary = json!({"cpu_model":cpu,"logical_cpus":logical,"memory_bytes":memory,"kernel":String::from_utf8(kernel.stdout)?.trim(),"hypervisor":if cpuinfo.lines().any(|s|s.starts_with("flags")&&s.split_whitespace().any(|w|w=="hypervisor")){json!("cpuinfo hypervisor flag present")}else{Value::Null},"isolation_tier_max":"L0"});
        let mut pressures = Map::new();
        for path in [
            "/proc/loadavg",
            "/proc/pressure/cpu",
            "/proc/pressure/memory",
        ] {
            if let Ok(value) = fs::read_to_string(path) {
                pressures.insert(path.to_owned(), json!(value));
            }
        }
        let dirty = Command::new("git")
            .args(["status", "--porcelain", "--untracked-files=no"])
            .current_dir(&self.root)
            .output()?;
        require(
            dirty.status.success(),
            "could not read tracked source state",
        )?;
        let document = json!({"summary":summary,"repository_context":git_head(&self.root)?,"tracked_source_dirty":!dirty.stdout.is_empty(),
            "isolation_tier":"L0","wall_valid":false,"threads":1,"caps":self.native()["caps"],
            "scientific_binary_sha256":self.protocol["arms"][0]["instrument"]["binary_sha256"],
            "scientific_source_commit":self.protocol["arms"][0]["instrument"]["commit"],
            "native_controller_sha256":self.native()["binary_sha256"],"pressure":pressures,
            "environment_recorded":{"RAYON_NUM_THREADS":"1","OMP_NUM_THREADS":"1","HKY_CHUNK":"1024"},"credential_values_recorded":false});
        let name = if self.run.join("host.json").exists() {
            format!(
                "host-resume-{}.json",
                SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
            )
        } else {
            "host.json".to_owned()
        };
        atomic_json(&self.run.join(&name), &document)?;
        atomic_bytes(
            &self.run.join("current-host.txt"),
            format!("{name}\n").as_bytes(),
        )
    }

    pub fn refresh(&self) -> Result<Vec<Value>> {
        self.guard()?;
        let out = Command::new(
            self.protocol["extensions"]["frozen_binary_path"]
                .as_str()
                .ok_or("no scientific executable")?,
        )
        .arg("replay-summary")
        .arg(&self.study)
        .arg(&self.run)
        .stdin(Stdio::null())
        .output()?;
        fs::write(self.run.join("native-summary.stderr"), &out.stderr)?;
        require(out.status.success(), "native scientific aggregate failed")?;
        let native: Value = serde_json::from_slice(&out.stdout)?;
        require(
            native["original_producer_timeout_and_cost_unchanged"] == true
                && native["independent_solver_words"].is_null(),
            "native source-cost contract changed",
        )?;
        atomic_json(&self.run.join("native-summary.json"), &native)?;
        let all = native["rows"]
            .as_array()
            .ok_or("native aggregate lacks rows")?;
        let mut rows = Vec::new();
        for (index, case) in self.cases.iter().enumerate() {
            for arm in ["hky", "hky-aa"] {
                let folder = self.run.join(arm).join(&case.id);
                if !folder.join("ended.txt").exists() {
                    continue;
                }
                let found: Vec<_> = all
                    .iter()
                    .filter(|r| {
                        r["arm"] == arm
                            && r["draw"] == case.draw
                            && r["guesses"] == case.guesses
                            && r["branch"] == case.branch
                    })
                    .collect();
                require(
                    found.len() == 1,
                    "native aggregate contains missing or duplicate case identity",
                )?;
                let status = read_json(&folder.join("status.json"))?;
                let mut row = map_row(index, case, arm, found[0], &status)?;
                row["started_at"] = optional_text(&folder.join("started.txt"));
                row["finished_at"] = optional_text(&folder.join("ended.txt"));
                rows.push(row);
            }
        }
        validate_grid(&rows, &self.cases, false)?;
        let mut lines = String::new();
        for row in &rows {
            lines.push_str(&serde_json::to_string(row)?);
            lines.push('\n');
        }
        atomic_bytes(&self.run.join("rows.jsonl"), lines.as_bytes())?;
        atomic_json(&self.run.join("native-flow.json"), &flow(&rows))?;
        Ok(rows)
    }

    pub fn final_export(&self, rows: &[Value]) -> Result<PathBuf> {
        validate_grid(rows, &self.cases, true)?;
        require(
            self.case_paths()
                .iter()
                .all(|p| p.join("ended.txt").is_file() && p.join("status.json").is_file()),
            "final export requires every registered raw completion/status record",
        )?;
        require(
            self.protocol["randomization"]["blinding"]["analysis_labels_masked"] == false,
            "native exact replay does not support unregistered blinding changes",
        )?;
        let directory = self.export_directory()?;
        fs::create_dir_all(&directory)?;
        let prefix = format!("../../{}", self.native()["run_directory"].as_str().unwrap());
        let file_entry = |relative: &str, kind: &str| -> Result<Value> {
            let path = self.run.join(relative_path(relative)?);
            Ok(
                json!({"path":format!("{prefix}/{relative}"),"sha256":sha_file(&path)?,"bytes":fs::metadata(path)?.len(),"kind":kind}),
            )
        };
        let files: Vec<_> = raw_files(&self.run)?
            .iter()
            .map(|r| file_entry(r, if r == "host.json" { "host" } else { "raw" }))
            .collect::<Result<_>>()?;
        let host = read_json(&self.run.join("host.json"))?;
        let arms:Vec<_>=self.protocol["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect();
        let analysis_path = self.root.join("examples/ic_guess_completed_audit.rs");
        let export = json!({"schema":"experiment.export/v1","export_id":"IC-balanced-guess-HKY-native-continuation-20261026","created":now()?,
            "protocol":{"path":format!("../../{}",self.protocol_path.file_name().ok_or("no protocol filename")?.to_str().ok_or("non-UTF8 protocol path")?),"sha256":self.protocol["freeze"]["protocol_sha256"],"protocol_id":self.protocol["protocol_id"],"version":self.protocol["version"]},
            "producer":{"runtime":"codex_cli","model":null,"session":"ic-native-hky-20261026","role":"executor"},
            "source":{"repo":self.protocol["repository"],"commit":host["repository_context"],"dirty":host["tracked_source_dirty"]},"backend":"local",
            "host":{"path":format!("{prefix}/host.json"),"sha256":sha_file(&self.run.join("host.json"))?,"class_id":"managed-linux-shared-L0","summary":host["summary"]},
            "arms":arms,"runs":rows,"flow":flow(rows),"files":files,
            "analysis":{"script":{"path":"../../../../examples/ic_guess_completed_audit.rs","sha256":sha_file(&analysis_path)?,"bytes":fs::metadata(analysis_path)?.len(),"kind":"analysis"},"blinded":false,"blinding_key_sha256":null,"outputs":[file_entry("native-summary.json","analysis")?,file_entry("native-flow.json","summary")?],"unmasked_at":null},
            "notes":[{"kind":"observation","text":"Every wall clock is invalid. Capped and interrupted attempts retain no imputed ranks or word cost. Four earlier legacy timeouts remain outside this prospective export.","by":"native replay executor"}],"reviews_dir":"reviews",
            "backend_record":{"scope":"native Boolean closure replication and A/A determinism only","native_controller_sha256":self.native()["binary_sha256"],"legacy_archive":self.native()["legacy_archive"],"legacy_processes_excluded_from_this_primary_export":4,"producer_unknown_cost_unchanged":true}});
        require(
            export["analysis"]["script"]["sha256"]
                == self.protocol["analysis_plan"]["script"]["sha256"],
            "export analysis hash differs from frozen analysis",
        )?;
        validate_schema(
            &export,
            Path::new(self.native()["export_schema"]["path"].as_str().unwrap()),
        )?;
        self.validate_export(&export, &directory)?;
        let path = directory.join("export.json");
        write_once_json(&path, &export)?;
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_export_checks_real_file_bindings_and_cannot_be_overwritten() {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let root = std::env::temp_dir().join(format!(
            "ic-native-export-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let study = root.join(STUDY);
        let run = study.join("pipeline-receipts/native-export-fixture");
        fs::create_dir_all(root.join("examples")).unwrap();
        fs::create_dir_all(&run).unwrap();
        let script = root.join("examples/ic_guess_completed_audit.rs");
        fs::write(
            &script,
            "// software export fixture; no scientific computation\n",
        )
        .unwrap();
        let mut protocol = read_json(&Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../research/ic_algebraic_crossover_20261023/protocol-hky-restart-20261026.json",
        ))
        .unwrap();
        protocol["analysis_plan"]["script"]["sha256"] = json!(sha_file(&script).unwrap());
        protocol["extensions"]["native_resume"] = json!({"run_directory":"pipeline-receipts/native-export-fixture","export_directory":"exports/native-export-fixture","binary_sha256":"0".repeat(64),"legacy_archive":{"path":"software-fixture-only","sha256":"0".repeat(64)},"export_schema":{"path":workspace.join("crypto-autoresearcher/schemas/experiment-export.v1.json")}});
        let cases: Vec<_> = all_cases().into_iter().skip(2).collect();
        let context = Context {
            root: root.clone(),
            study: study.clone(),
            protocol_path: study.join("fixture-protocol.json"),
            protocol,
            run: run.clone(),
            cases,
        };
        atomic_json(&context.protocol_path, &context.protocol).unwrap();
        let host = json!({"repository_context":"0".repeat(40),"tracked_source_dirty":false,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":"L0"}});
        atomic_json(&run.join("host.json"), &host).unwrap();
        atomic_json(
            &run.join("native-summary.json"),
            &json!({"software_fixture":true}),
        )
        .unwrap();
        let mut rows = Vec::new();
        for (i, case) in context.cases.iter().enumerate() {
            for arm in ["hky", "hky-aa"] {
                let path = run.join(arm).join(&case.id);
                fs::create_dir_all(&path).unwrap();
                fs::write(path.join("ended.txt"), "software-fixture\n").unwrap();
                atomic_json(
                    &path.join("status.json"),
                    &json!({"exit_code":124,"software_fixture":true}),
                )
                .unwrap();
                rows.push(map_row(i,case,arm,&json!({"arm":arm,"draw":case.draw,"guesses":case.guesses,"branch":case.branch,"classification":"timeout"}),&json!({"exit_code":124})).unwrap());
            }
        }
        atomic_json(&run.join("native-flow.json"), &flow(&rows)).unwrap();
        let export = context.final_export(&rows).unwrap();
        let before = sha_file(&export).unwrap();
        context
            .validate_export(&read_json(&export).unwrap(), export.parent().unwrap())
            .unwrap();
        assert!(context.final_export(&rows).is_err());
        assert_eq!(sha_file(&export).unwrap(), before);
        fs::write(run.join("host.json"), "changed software fixture\n").unwrap();
        assert!(context
            .validate_export(&read_json(&export).unwrap(), export.parent().unwrap())
            .is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn censored_rows_keep_metrics_absent_and_match_canonical_seed_indices() {
        let case = all_cases()[2].clone();
        let native = json!({"arm":"hky","draw":case.draw,"guesses":case.guesses,"branch":case.branch,"classification":"timeout","replay":null});
        let row = map_row(0, &case, "hky", &native, &json!({"exit_code":124})).unwrap();
        assert_eq!(row["instance_id"], "0");
        assert_eq!(row["backend_ref"], "d0-k1-b1");
        assert_eq!(row["measurements"]["counted"], json!({}));
        assert_eq!(row["outcome_class"], "not_completed");
        validate_grid(std::slice::from_ref(&row), &[case], false).unwrap();
        assert_eq!(flow(&[row])["not_completed"], 1);
    }
    #[test]
    fn row_mapping_rejects_mismatched_tuple_and_fabricated_success_metrics() {
        let case = all_cases()[2].clone();
        let native = json!({"arm":"hky","draw":case.draw,"guesses":case.guesses,"branch":case.branch,"classification":"verified","replay":{}});
        assert!(map_row(0, &case, "hky", &native, &json!({"exit_code":0})).is_err());
        let mut wrong = native;
        wrong["draw"] = json!(6);
        assert!(map_row(0, &case, "hky", &wrong, &json!({"exit_code":0})).is_err());
    }
    #[test]
    fn complete_grid_rejects_missing_duplicate_and_nonnumeric_seed_cells() {
        let cases: Vec<_> = all_cases().into_iter().skip(2).collect();
        let mut rows = Vec::new();
        for (i, case) in cases.iter().enumerate() {
            for arm in ["hky", "hky-aa"] {
                rows.push(map_row(i,case,arm,&json!({"arm":arm,"draw":case.draw,"guesses":case.guesses,"branch":case.branch,"classification":"timeout"}),&json!({"exit_code":124})).unwrap());
            }
        }
        validate_grid(&rows, &cases, true).unwrap();
        assert_eq!(flow(&rows)["planned"], 244);
        let mut bad = rows.clone();
        bad.pop();
        assert!(validate_grid(&bad, &cases, true).is_err());
        let mut bad = rows.clone();
        bad[1] = bad[0].clone();
        assert!(validate_grid(&bad, &cases, true).is_err());
        let mut bad = rows;
        bad[0]["instance_id"] = json!("d0-k1-b1");
        assert!(validate_grid(&bad, &cases, true).is_err());
    }
}
