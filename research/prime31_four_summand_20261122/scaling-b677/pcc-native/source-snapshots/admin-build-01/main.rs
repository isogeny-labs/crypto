//! Administrative metadata only: no science, signature, or ledger append.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    process::Command,
};

include!(concat!(env!("OUT_DIR"), "/canonical_utf8.rs"));

type Result<T, E = String> = std::result::Result<T, E>;
const STUDY: &str = "research/prime31_four_summand_20261122/scaling-b677";
const TOOL: &str = "research/prime31_four_summand_20261122/scaling-b677/pcc-native";

fn require(ok: bool, message: impl Into<String>) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing string {key}"))
}

fn hex(s: &str, bytes: usize) -> bool {
    s.len() == bytes * 2
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn text(path: &Path) -> Result<String> {
    String::from_utf8(read(path)?).map_err(|e| format!("{}: {e}", path.display()))
}

fn json_file(path: &Path) -> Result<Value> {
    serde_json::from_slice(&read(path)?).map_err(|e| format!("{}: {e}", path.display()))
}

fn relative(path: &str) -> Result<()> {
    require(
        !path.is_empty()
            && !path.contains('\\')
            && Path::new(path)
                .components()
                .all(|c| matches!(c, Component::Normal(_))),
        format!("invalid relative path: {path}"),
    )
}

fn contained(repo: &Path, rel: &str) -> Result<PathBuf> {
    relative(rel)?;
    let path = repo
        .join(rel)
        .canonicalize()
        .map_err(|e| format!("{rel}: {e}"))?;
    require(
        path.starts_with(repo),
        format!("path escapes repository: {rel}"),
    )?;
    Ok(path)
}

fn pin(repo: &Path, rel: &str) -> Result<Value> {
    let path = contained(repo, rel)?;
    require(path.is_file(), format!("not a regular file: {rel}"))?;
    Ok(json!({"path":rel, "sha256":sha(&read(&path)?)}))
}

fn check_pin(repo: &Path, value: &Value) -> Result<()> {
    let path = string(value, "path")?;
    let digest = string(value, "sha256")?;
    require(hex(digest, 32), format!("invalid SHA256 pin: {path}"))?;
    require(
        pin(repo, path)?["sha256"] == digest,
        format!("SHA256 mismatch: {path}"),
    )
}

fn canonical_digest(value: &Value) -> Result<String> {
    let mut body = value.clone();
    if let Some(object) = body.as_object_mut() {
        object.remove("standing");
    }
    Ok(sha(canonical_utf8(&body)?.as_bytes()))
}

fn check_claim(repo: &Path, claim: &Value) -> Result<Value> {
    require(claim["schema"] == "pcc.claim/v1", "wrong claim schema")?;
    let id = string(claim, "id")?;
    let parts: Vec<_> = id.split('-').collect();
    require(
        parts.len() == 3
            && parts[0] == "PCC"
            && parts[1].len() == 8
            && parts[1].bytes().all(|c| c.is_ascii_digit())
            && hex(parts[2], 3),
        "invalid claim ID",
    )?;
    require(
        claim["kind"] == "statement" && claim["claim_type"] == "data",
        "this native checker supports only unsigned statement/data records",
    )?;
    require(
        !string(claim, "statement")?.trim().is_empty(),
        "empty statement",
    )?;
    require(!string(claim, "scope")?.trim().is_empty(), "empty scope")?;
    require(
        hex(string(&claim["producer"], "commit")?, 20),
        "invalid producer commit",
    )?;
    string(&claim["producer"], "session")?;
    string(&claim["producer"], "model")?;
    string(claim, "created_at")?;
    require(
        claim.get("signature").is_none()
            && claim.get("sig").is_none()
            && claim.get("standing").is_none(),
        "unsigned record must not carry signature or standing",
    )?;
    let proofs = claim["proofs"].as_array().ok_or("proofs is not an array")?;
    require(!proofs.is_empty(), "no artifact pins")?;
    let mut seen = BTreeSet::new();
    for proof in proofs {
        require(
            proof.as_object().is_some_and(|o| o.len() == 2) && proof["kind"] == "attestation",
            "only unsigned subject attestations are supported",
        )?;
        check_pin(repo, &proof["subject"])?;
        require(
            seen.insert(string(&proof["subject"], "path")?),
            "duplicate artifact pin",
        )?;
    }
    Ok(json!({
        "schema":"rr5.pcc-native-metadata-check/v1", "result":"metadata_consistent",
        "claim_id":id, "claim_sha256":canonical_digest(claim)?, "artifact_pins_checked":proofs.len(),
        "check_scope":"supported unsigned statement schema and exact artifact byte hashes only",
        "trust_status":"unsigned producer metadata; no PCC verdict or execution verification"
    }))
}

fn command(repo: &Path, program: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    require(
        output.status.success(),
        format!("{program} returned {}", output.status),
    )?;
    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_owned())
        .map_err(|e| e.to_string())
}

fn write_new(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())
}

fn check_inventory(
    repo: &Path,
    rel: &str,
    substitutions: &Value,
    git_sources: &Value,
) -> Result<Value> {
    let mut count = 0usize;
    let mut replacements = Vec::new();
    let mut historical = Vec::new();
    let mut seen = BTreeSet::new();
    for line in text(&contained(repo, rel)?)?.lines() {
        let (digest, original) = line
            .split_once("  ")
            .ok_or_else(|| format!("bad inventory line in {rel}"))?;
        require(hex(digest, 32), format!("bad inventory digest in {rel}"))?;
        require(
            seen.insert(original.to_owned()),
            format!("duplicate source in {rel}: {original}"),
        )?;
        if let Some(commit) = git_sources.get(original).and_then(Value::as_str) {
            require(hex(commit, 20), "invalid historical source commit")?;
            relative(original)?;
            let output = Command::new("git")
                .args(["show", &format!("{commit}:{original}")])
                .current_dir(repo)
                .output()
                .map_err(|e| e.to_string())?;
            require(
                output.status.success() && sha(&output.stdout) == digest,
                format!("historical Git source differs: {commit}:{original}"),
            )?;
            historical.push(
                json!({"recorded_path":original,"commit":commit,"sha256":digest,
                "resolution":"exact raw Git blob bytes; not a direct run-time source snapshot"}),
            );
        } else {
            let actual = substitutions
                .get(original)
                .and_then(Value::as_str)
                .unwrap_or(original);
            check_pin(repo, &json!({"path":actual, "sha256":digest}))?;
            if actual != original {
                replacements.push(
                    json!({"recorded_path":original,"preserved_path":actual,"sha256":digest}),
                );
            }
        }
        count += 1;
    }
    require(count > 0, format!("empty source inventory: {rel}"))?;
    Ok(
        json!({"inventory":pin(repo, rel)?, "source_paths_checked":count,
        "original_source_copies_used":replacements,"historical_git_sources_used":historical}),
    )
}

fn prepare(repo: &Path, out_rel: &str) -> Result<Value> {
    relative(out_rel)?;
    require(
        out_rel.starts_with(&format!("{TOOL}/")),
        "output must be under pcc-native",
    )?;
    let out = repo.join(out_rel);
    require(
        !out.exists(),
        "output directory already exists; preserve it and choose a new one",
    )?;
    let parent = out
        .parent()
        .ok_or("missing output parent")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    require(
        parent.starts_with(contained(repo, TOOL)?),
        "output parent escapes pcc-native",
    )?;
    let input_rel = format!("{TOOL}/input.json");
    let input = json_file(&contained(repo, &input_rel)?)?;
    require(
        input["schema"] == "rr5.pcc-metadata-input/v1",
        "wrong preparation input schema",
    )?;
    check_pin(repo, &input["specification"])?;
    let reference = json_file(&contained(
        repo,
        string(&input["reference_claim"], "path")?,
    )?)?;
    let reference_digest = canonical_digest(&reference)?;
    require(
        reference_digest == string(&input["reference_claim"], "canonical_sha256")?,
        "native canonicalization differs from the existing registered RR5 claim",
    )?;
    let ledger_before = pin(repo, "pcc/ledger.jsonl")?;
    let created_at = command(repo, "date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"])?;
    require(created_at.len() == 20, "unexpected UTC date format")?;
    let head_commit = command(repo, "git", &["rev-parse", "HEAD"])?;
    require(hex(&head_commit, 20), "Git HEAD is not a 40-hex commit")?;
    let head_tree = command(repo, "git", &["rev-parse", "HEAD^{tree}"])?;
    let commit = string(&input["published_core"], "commit")?;
    require(
        hex(commit, 20) && input["published_core"]["tree"] == head_tree,
        "declared publication core tree differs from the local core tree",
    )?;

    // Bind the compiled administrative tool to its actual source files too.
    for (rel, compiled) in [
        (
            format!("{TOOL}/src/main.rs"),
            include_bytes!("main.rs").as_slice(),
        ),
        (
            format!("{TOOL}/Cargo.toml"),
            include_bytes!("../Cargo.toml").as_slice(),
        ),
        (
            format!("{TOOL}/Cargo.lock"),
            include_bytes!("../Cargo.lock").as_slice(),
        ),
        (
            format!("{TOOL}/build.rs"),
            include_bytes!("../build.rs").as_slice(),
        ),
    ] {
        require(
            pin(repo, &rel)?["sha256"] == sha(compiled),
            format!("compiled tool source changed: {rel}"),
        )?;
    }
    check_pin(
        repo,
        &json!({"path":"src/cryptanalysis/ecbench/claim.rs",
        "sha256":env!("PCC_CANONICAL_WHOLE_SHA256")}),
    )?;

    let mut artifacts = BTreeMap::<String, Value>::new();
    let mut phase_records = Vec::new();
    let phases = input["phases"].as_array().ok_or("phases is not an array")?;
    for phase in phases {
        let name = string(phase, "name")?;
        relative(name)?;
        require(!name.contains('/'), "phase name must have one component")?;
        let dir_rel = format!("{STUDY}/evidence/{name}");
        let dir = contained(repo, &dir_rel)?;
        let exit: i64 = text(&dir.join("exit-status.txt"))?
            .trim()
            .parse()
            .map_err(|_| "invalid exit status")?;
        require(
            phase["exit_status"].as_i64() == Some(exit),
            format!("unexpected exit status for {name}"),
        )?;
        let stdout = text(&dir.join("stdout.txt"))?;
        let stderr = text(&dir.join("stderr.txt"))?;
        let mut checks = Vec::new();
        for field in ["source-check-exit.txt", "binary-check-exit.txt"] {
            if dir.join(field).exists() {
                require(
                    text(&dir.join(field))?.trim() == "0",
                    format!("failed {field} in {name}"),
                )?;
                checks.push(field);
            }
        }
        let summary = if let Some(expected) = phase.get("test_summary") {
            let lines: Vec<_> = stdout
                .lines()
                .filter(|s| s.starts_with("test result:"))
                .collect();
            require(
                lines.len() == 1,
                format!("expected one test summary in {name}"),
            )?;
            let prefix = format!(
                "test result: {}. {} passed; {} failed; 0 ignored; 0 measured; 0 filtered out;",
                string(expected, "status")?,
                expected["passed"],
                expected["failed"]
            );
            require(
                lines[0].starts_with(&prefix),
                format!("test summary differs in {name}"),
            )?;
            Some(lines[0].to_owned())
        } else {
            None
        };
        if name == "admission-01" {
            require(
                stderr.contains("checkpoint: cannot upload, so nothing was checkpointed"),
                "missing recorded admission failure",
            )?;
        }
        if name == "root-library-01" {
            require(
                stderr.contains("Cannot allocate memory (os error 12)")
                    && stderr.contains("could not compile `aws-sdk-s3`")
                    && !stdout.contains("test result:")
                    && !stdout.contains("running "),
                "root gate record no longer matches the scoped build limitation",
            )?;
        }
        let inventory_rel = format!("{dir_rel}/pre-command-sources.sha256");
        let inventory = if dir.join("pre-command-sources.sha256").exists() {
            let old = name == "build-01" || name == "native-tests-01";
            let substitutions = if old {
                &input["old_source_substitutions"]
            } else {
                &Value::Null
            };
            let git_sources = if name == "root-library-01" {
                &input["root_library_historical_git_sources"]
            } else {
                &Value::Null
            };
            Some(check_inventory(
                repo,
                &inventory_rel,
                substitutions,
                git_sources,
            )?)
        } else {
            None
        };
        let mut files = Vec::new();
        for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.file_type().map_err(|e| e.to_string())?.is_file() {
                let rel = format!(
                    "{dir_rel}/{}",
                    entry.file_name().to_str().ok_or("non-UTF8 artifact name")?
                );
                artifacts.insert(rel.clone(), pin(repo, &rel)?);
                files.push(rel);
            }
        }
        files.sort();
        phase_records.push(json!({"phase":name, "recorded_exit_status":exit,
            "recorded_test_summary":summary, "zero_status_receipts":checks,
            "source_inventory_check":inventory, "artifact_paths":files}));
    }
    let scalar_rel = format!("{STUDY}/evidence/coset7-check-01/stdout.txt");
    let scalar = json_file(&contained(repo, &scalar_rel)?)?;
    require(
        scalar
            == json!({"p":2147483647u64,"y0":136769428u64,"exponent":7,
        "repeated_product":1852957275u64,"addition_chain":1852957275u64,
        "later_spelling":1852955275u64,"earlier_spelling":1852957275u64,
        "matches_later":false,"matches_earlier":true}),
        "scalar diagnostic record differs",
    )?;

    for value in input["extra_artifacts"]
        .as_array()
        .ok_or("extra_artifacts is not an array")?
    {
        let rel = value.as_str().ok_or("artifact path is not a string")?;
        artifacts.insert(rel.to_owned(), pin(repo, rel)?);
    }
    for rel in [
        input_rel.clone(),
        format!("{TOOL}/src/main.rs"),
        format!("{TOOL}/Cargo.toml"),
        format!("{TOOL}/Cargo.lock"),
        format!("{TOOL}/build.rs"),
        format!("{TOOL}/README.md"),
        "src/cryptanalysis/ecbench/claim.rs".to_owned(),
        "tools/pcc/pcc.py".to_owned(),
    ] {
        artifacts.insert(rel.clone(), pin(repo, &rel)?);
    }
    let executable = env::current_exe().map_err(|e| e.to_string())?;
    let preparation = json!({
        "schema":"rr5.pcc-native-preparation/v1", "created_at":created_at,
        "scope":"unsigned administrative observations from recorded files; no science or execution-proof verification",
        "local_head_at_preparation":head_commit,"local_head_tree":head_tree,
        "published_core_supplied_by_parent":input["published_core"],
        "tested_source_identity":"byte inventories with explicit direct original-source copies for build-01/native-tests-01 and two historical Git blobs for root-library-01",
        "pcc_specification":input["specification"],
        "native_canonical_reference_check":{"path":input["reference_claim"]["path"],"canonical_sha256":reference_digest},
        "canonical_json_source":{"path":"src/cryptanalysis/ecbench/claim.rs",
            "whole_source_sha256":env!("PCC_CANONICAL_WHOLE_SHA256"),
            "byte_start_inclusive":env!("PCC_CANONICAL_RANGE_START"),
            "byte_end_exclusive":env!("PCC_CANONICAL_RANGE_END"),
            "selected_sha256":env!("PCC_CANONICAL_SELECTED_SHA256"),"rewritten":false},
        "generator":{"binary_sha256":sha(&read(&executable)?),
            "command_arguments":env::args().collect::<Vec<_>>(),"version":env!("CARGO_PKG_VERSION")},
        "canonical_ledger_observed_before":ledger_before,
        "phases":phase_records,"recorded_scalar_diagnostic":scalar,
        "artifacts":artifacts.values().collect::<Vec<_>>(),
        "registration_state":"prepared only; no canonical ledger append",
        "limitations":["No signed PCC verdict or execution proof is produced",
            "The metadata checker does not rerun the scientific tests or scalar arithmetic",
            "External executable provenance is represented only by the retained binary-hash receipts",
            "The full root command timed out after an explicit dependency allocation error; no root test result",
            "The pinned set contains qualification/forced-cap controls, not a registered B677 extraction result"]
    });
    fs::create_dir(&out).map_err(|e| e.to_string())?;
    write_new(&out.join("preparation.json"), &preparation)?;
    let preparation_rel = format!("{out_rel}/preparation.json");
    artifacts.insert(preparation_rel.clone(), pin(repo, &preparation_rel)?);
    let mut random = [0u8; 3];
    fs::File::open("/dev/urandom")
        .and_then(|mut f| f.read_exact(&mut random))
        .map_err(|e| e.to_string())?;
    let id = format!(
        "PCC-{}-{:02x}{:02x}{:02x}",
        created_at[..10].replace('-', ""),
        random[0],
        random[1],
        random[2]
    );
    let claim = json!({"schema":"pcc.claim/v1","id":id,"kind":"statement","claim_type":"data",
        "statement":input["statement"],"scope":input["scope"],"created_at":created_at,
        "producer":{"commit":commit,"session":input["session"],"model":input["model"]},
        "proofs":artifacts.values().map(|subject| json!({"kind":"attestation","subject":subject})).collect::<Vec<_>>()});
    let mut checked = check_claim(repo, &claim)?;
    let claim_name = format!("{id}.json");
    write_new(&out.join(&claim_name), &claim)?;
    let reread = json_file(&out.join(&claim_name))?;
    require(
        canonical_digest(&reread)? == canonical_digest(&claim)?,
        "claim readback changed its canonical digest",
    )?;
    let body = json!({"claim_id":id,"claim_sha256":canonical_digest(&claim)?,"path":format!("pcc/claims/{id}.json")});
    write_new(&out.join("registration-body.json"), &body)?;
    let ledger_after = pin(repo, "pcc/ledger.jsonl")?;
    require(
        ledger_before == ledger_after,
        "canonical ledger changed during preparation; stop before registration",
    )?;
    checked["claim_file"] = pin(repo, &format!("{out_rel}/{claim_name}"))?;
    checked["canonical_ledger_after"] = ledger_after;
    checked["canonical_ledger_unchanged"] = json!(true);
    checked["pending_registration_body"] = pin(repo, &format!("{out_rel}/registration-body.json"))?;
    write_new(&out.join("native-check.json"), &checked)?;
    Ok(checked)
}

fn run() -> Result<Value> {
    let args: Vec<_> = env::args().collect();
    require(args.len() == 4 || args.len() == 5,
        "usage: rr5-b677-pcc-metadata prepare REPO OUT_REL | check REPO CLAIM_REL [EXPECTED_CANONICAL_SHA256]")?;
    let repo = Path::new(&args[2])
        .canonicalize()
        .map_err(|e| e.to_string())?;
    match args[1].as_str() {
        "prepare" if args.len() == 4 => prepare(&repo, &args[3]),
        "check" => {
            let claim = json_file(&contained(&repo, &args[3])?)?;
            let mut checked = check_claim(&repo, &claim)?;
            if let Some(expected) = args.get(4) {
                require(
                    hex(expected, 32) && checked["claim_sha256"] == *expected,
                    "claim canonical digest differs from the expected digest",
                )?;
            }
            checked["claim_file"] = pin(&repo, &args[3])?;
            Ok(checked)
        }
        _ => Err("unsupported command; this tool cannot append the ledger".into()),
    }
}

fn main() {
    match run() {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => {
            eprintln!("native PCC metadata preparation refused: {error}");
            std::process::exit(1);
        }
    }
}
