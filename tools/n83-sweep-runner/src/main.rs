//! Native bounded diagnostic runner. Deliberately admits no scalar recovery or runtime winner.
use serde_json::{json, Value};
mod card;
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/ecbench/isolation.rs"]
mod native_isolation;
#[allow(dead_code)]
#[path = "../../../src/cryptanalysis/ecbench/signals.rs"]
pub mod native_signals;
mod cryptanalysis {
    pub mod ecbench {
        pub use crate::native_signals as signals;
    }
}
mod reservation;
mod xor_summary;
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    os::unix::fs::MetadataExt,
    path::Path,
    process::{Command, Output},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
type R<T> = Result<T, Box<dyn Error>>;
const SOURCES: [&str; 9] = [
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/koblitz_index_calculus.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
    "src/cryptanalysis/sat.rs",
    "examples/koblitz_n83_fixed_witness_control.rs",
    "examples/koblitz_n83_selector_witness_control.rs",
    "examples/koblitz_n83_free_selector_export_control.rs",
];
fn require(ok: bool, msg: &str) -> R<()> {
    if !ok {
        return Err(msg.into());
    }
    Ok(())
}
fn read(p: impl AsRef<Path>) -> R<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn write(p: impl AsRef<Path>, v: &Value) -> R<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(p)?;
    f.write_all(serde_json::to_string_pretty(v)?.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}
fn hash(p: impl AsRef<Path>) -> R<String> {
    Ok(blake3::hash(&fs::read(p)?).to_hex().to_string())
}
fn text<'a>(v: &'a Value, k: &str) -> R<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("missing string {k}").into())
}
fn number(v: &Value, k: &str) -> R<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing integer {k}").into())
}
// A single solve's completed boundaries or valid killed-worker prefix.
// Snapshot counters remain observations; only solve_return is terminal.
fn sat_trace(raw: &str) -> R<Value> {
    let mut events = Vec::new();
    let mut state = 0;
    let mut last_ms = 0.;
    let mut last_conflicts = 0;
    for line in raw.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if v["schema"] != "n83.sat-phase/v1" {
            continue;
        }
        let phase = text(&v, "phase")?;
        let next = match (state, phase) {
            (0, "solve_enter") => 1,
            (1, "initial_propagation_begin") => 2,
            (2, "initial_propagation_complete") => 3,
            (3, "cdcl_enter") => 4,
            (4, "conflict_snapshot") => 4,
            (1 | 3 | 4, "solve_return") => 5,
            _ => return Err("invalid SAT phase sequence".into()),
        };
        let ms = v["elapsed_ms"].as_f64().ok_or("missing SAT time")?;
        let conflicts = number(&v, "conflicts")?;
        require(
            ms.is_finite() && ms >= last_ms && conflicts >= last_conflicts,
            "SAT trace time/counter regressed",
        )?;
        require(
            v["counter_scope"] == "solver_instance_cumulative",
            "SAT counter scope differs",
        )?;
        if next == 5 {
            require(
                ["Sat", "Unsat", "Unknown"].contains(&text(&v, "outcome")?),
                "SAT outcome missing",
            )?;
        } else {
            require(v["outcome"].is_null(), "premature SAT outcome")?;
        }
        state = next;
        last_ms = ms;
        last_conflicts = conflicts;
        events.push(v);
    }
    require(!events.is_empty(), "no native SAT trace")?;
    Ok(
        json!({"schema":"n83.sat-trace-audit/v1","status":if state==5 {"COMPLETE"} else {"VALID_PREFIX_NONTERMINAL"},"events":events,"terminal_counters":if state==5 {events.last().cloned()} else {None}}),
    )
}
fn inventory(root: &Path) -> R<BTreeMap<String, String>> {
    fn walk(root: &Path, p: &Path, m: &mut BTreeMap<String, String>) -> R<()> {
        for item in fs::read_dir(p)? {
            let item = item?;
            let p = item.path();
            let t = item.file_type()?;
            require(!t.is_symlink(), "input symlinks forbidden")?;
            if t.is_dir() {
                walk(root, &p, m)?
            } else if t.is_file() {
                m.insert(
                    p.strip_prefix(root)?
                        .to_str()
                        .ok_or("non-UTF8 path")?
                        .to_owned(),
                    hash(&p)?,
                );
            }
        }
        Ok(())
    }
    let mut m = BTreeMap::new();
    walk(root, root, &mut m)?;
    Ok(m)
}
fn retained_panel(manifest: &Value, columns: u64) -> R<()> {
    let rows = manifest["bases"].as_array().ok_or("panel bases missing")?;
    require(
        rows.iter()
            .filter(|r| {
                r["a"] == 0
                    && r["columns"] == columns
                    && r["policy"] == "public_x_hash"
                    && r["seed"] == 2026100801u64
            })
            .count()
            == 1,
        "retained primary hash-policy row missing or ambiguous",
    )
}
fn docker(args: &[&str]) -> R<Output> {
    let out = Command::new("timeout")
        .args(["--kill-after=2s", "15s", "docker"])
        .args(args)
        .output()?;
    require(
        out.status.success(),
        &format!(
            "docker {:?}: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        ),
    )?;
    Ok(out)
}
fn inspect(name: &str) -> R<Value> {
    let out=docker(&["inspect","--format", "{\"host\":{{json .HostConfig}},\"mounts\":{{json .Mounts}},\"state\":{{json .State}},\"image\":{{json .Image}}}",name])?;
    Ok(serde_json::from_slice(&out.stdout)?)
}
fn isolation(v: &Value, image: &str) -> R<()> {
    let h = &v["host"];
    require(
        h["CapDrop"]
            .as_array()
            .is_some_and(|a| a.contains(&json!("ALL")))
            && h["SecurityOpt"]
                .as_array()
                .is_some_and(|a| a.contains(&json!("no-new-privileges"))),
        "capability/security isolation mismatch",
    )?;
    require(
        h["Memory"] == 4294967296u64
            && h["MemorySwap"] == 4294967296u64
            && h["NanoCpus"] == 1000000000u64
            && h["NetworkMode"] == "none"
            && h["ReadonlyRootfs"] == true
            && h["PidsLimit"] == 64
            && v["image"] == image,
        "isolation/image mismatch",
    )?;
    for m in v["mounts"].as_array().ok_or("missing mounts")? {
        let dst = m["Destination"].as_str().ok_or("mount destination")?;
        require(m["RW"] == (dst == "/out"), "unexpected writable mount")?;
    }
    Ok(())
}
// Independent polynomial-word arithmetic for the exact primary N83 curve.
const MOD: u128 = (1u128 << 83) | (1u128 << 45) | 7;
const ORDER: u128 = 2417851639230796216685689;
type Point = Option<(u128, u128)>;
fn mul(mut a: u128, mut b: u128) -> u128 {
    let mut r = 0;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a
        }
        b >>= 1;
        a <<= 1;
        if a & (1 << 83) != 0 {
            a ^= MOD
        }
    }
    r
}
fn inv(a: u128) -> R<u128> {
    require(a != 0, "zero field inverse")?;
    let mut e = (1u128 << 83) - 2;
    let (mut x, mut y) = (a, 1);
    while e != 0 {
        if e & 1 != 0 {
            y = mul(y, x)
        }
        x = mul(x, x);
        e >>= 1;
    }
    Ok(y)
}
fn add(a: Point, b: Point) -> R<Point> {
    let Some((x1, y1)) = a else { return Ok(b) };
    let Some((x2, y2)) = b else { return Ok(a) };
    if x1 == x2 {
        if y1 != y2 || x1 == 0 {
            return Ok(None);
        }
        let s = x1 ^ mul(y1, inv(x1)?);
        let x = mul(s, s) ^ s;
        return Ok(Some((x, mul(x1, x1) ^ mul(s ^ 1, x))));
    }
    let s = mul(y1 ^ y2, inv(x1 ^ x2)?);
    let x = mul(s, s) ^ s ^ x1 ^ x2;
    Ok(Some((x, mul(s, x1 ^ x) ^ x ^ y1)))
}
fn scale(mut p: Point, mut k: u128) -> R<Point> {
    let mut r = None;
    while k != 0 {
        if k & 1 != 0 {
            r = add(r, p)?
        }
        p = add(p, p)?;
        k >>= 1;
    }
    Ok(r)
}
fn decimal(v: &Value, cap: u128) -> R<u128> {
    let s = v.as_str().ok_or("decimal string")?;
    let n = s.parse::<u128>()?;
    require(
        n < cap && n.to_string() == s,
        "noncanonical/out-of-range decimal",
    )?;
    Ok(n)
}
fn point(v: &Value) -> R<Point> {
    if v.is_null() {
        return Ok(None);
    }
    let a = v.as_array().ok_or("point array")?;
    require(a.len() == 2, "point length")?;
    let (x, y) = (decimal(&a[0], 1 << 83)?, decimal(&a[1], 1 << 83)?);
    require(
        mul(y, y) ^ mul(x, y) == mul(mul(x, x), x) ^ 1,
        "point off curve",
    )?;
    Ok(Some((x, y)))
}
fn query(v: &Value, q: Point) -> R<()> {
    let g = Some((5402229379304697578719232, 3600185575789834070382019));
    let a = decimal(&v["a"], ORDER)?;
    let b = decimal(&v["b"], ORDER)?;
    require(b != 0, "zero query b")?;
    require(
        add(scale(g, a)?, scale(q, b)?)? == point(&v["target"])?,
        "independent public query replay mismatch",
    )
}
fn phases(raw: &str) -> R<Vec<Value>> {
    let mut records = Vec::new();
    let mut last = 0.0;
    let order = [
        "closure_begin",
        "closure_complete",
        "index_complete",
        "encoding_begin",
        "circuit_complete",
        "domain_complete",
        "solver_enter",
        "solver_return",
    ];
    let mut next = 0;
    for line in raw.lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v["schema"] == "n83.chain-phase/v1" {
                let t = v["elapsed_ms"].as_f64().ok_or("phase time")?;
                require(t.is_finite() && t >= 0., "invalid phase time")?;
                if next == order.len() {
                    next = if v["phase"] == "solver_enter" { 6 } else { 3 };
                }
                require(
                    v["phase"] == order[next],
                    "phase sequence is not a valid execution prefix",
                )?;
                next += 1;
                require(t >= last, "phase time regressed")?;
                last = t;
                records.push(v);
            }
        }
    }
    Ok(records)
}
fn zero_counters(r: &Value, conflicts: u64) -> R<()> {
    for key in [
        "relations",
        "sat_models",
        "terminal_matrix_rank",
        "verification_failures",
        "sat_invalid_models",
        "independent_relations",
    ] {
        require(
            r[key] == 0,
            &format!("nonzero/missing {key}: full relation replay required"),
        )?;
    }
    require(
        r["sat_calls"] == 1 && r["sat_unknowns"] == 1 && r["sat_conflicts"] == conflicts,
        "solver-cap counters disagree",
    )
}
fn audit(dir: &Path) -> R<Value> {
    let cfg = read(dir.join("config.json"))?;
    require(
        hash(dir.join("config.json"))? == text(&read(dir.join("custody.json"))?, "config_blake3")?,
        "config changed",
    )?;
    let out = read(dir.join("outer.json"))?;
    require(
        hash(dir.join("spec.json"))? == text(&cfg, "spec_blake3")?,
        "spec changed",
    )?;
    require(out["cleanup_verified"] == true, "cleanup not verified")?;
    require(
        hash(dir.join("stdout.log"))? == text(&out, "stdout_blake3")?
            && hash(dir.join("stderr.log"))? == text(&out, "stderr_blake3")?,
        "raw log custody mismatch",
    )?;
    isolation(&read(dir.join("inspect-live.json"))?, text(&cfg, "image")?)?;
    let final_state = read(dir.join("inspect-final.json"))?;
    require(
        final_state["state"]["OOMKilled"] == false,
        "OOM outcome cannot be admitted as search result",
    )?;
    require(
        inventory(Path::new(text(&cfg, "panel")?))?
            == serde_json::from_value(cfg["panel_blake3"].clone())?
            && inventory(Path::new(text(&cfg, "fixtures")?))?
                == serde_json::from_value(cfg["fixtures_blake3"].clone())?,
        "input custody mismatch",
    )?;
    require(
        hash(text(&cfg, "worker")?)? == text(&cfg, "worker_blake3")?,
        "worker changed",
    )?;
    require(
        inventory(&dir.join("source"))? == serde_json::from_value(cfg["source_blake3"].clone())?,
        "source custody mismatch",
    )?;
    let raw = fs::read_to_string(dir.join("stderr.log"))?;
    if cfg["columns"] == 0 {
        let external = cfg["external_solver_blake3"].is_string();
        let sat = if external {
            json!({"status":"EXTERNAL_NO_INTERNAL_TRACE"})
        } else {
            sat_trace(&raw)?
        };
        if out["wall_censored"] == true {
            return Ok(
                json!({"schema":"n83.native-audit/v1","status":"UNKNOWN_CONTROL_WALL_CAP","sat_trace":sat,"terminal_counters":null,"outer_wall_ms":out["outer_wall_ms"],"selected_best_total_runtime":null}),
            );
        }
        // The frozen reference worker retains raw CMS output and exits1 on
        // an unresolved backend. Preserve a typed unknown instead of trying
        // to parse its absent positive-control stdout as JSON.
        if external
            && out["exit_code"] == 1
            && fs::read_to_string(dir.join("cms.model"))?.trim() == "s INDETERMINATE"
        {
            let cms = fs::read_to_string(dir.join("cms.stdout"))?;
            let conflicts = cms_conflicts(&cms).ok_or("missing external conflict count")?;
            let cap = number(&cfg, "conflicts")?;
            require(
                (cap..=cap + 1).contains(&conflicts)
                    && raw.contains("external SAT reference did not return SAT: Some(15)"),
                "external unknown is not the declared conflict boundary",
            )?;
            return Ok(
                json!({"schema":"n83.native-audit/v1","status":"UNKNOWN_EXTERNAL_CONFLICT_CAP",
                "external_reported_conflicts":conflicts,"model_verified":null,"terminal_counters":null,
                "outer_wall_ms":out["outer_wall_ms"],"selected_best_total_runtime":null}),
            );
        }
        let result: Value = serde_json::from_slice(&fs::read(dir.join("stdout.log"))?)?;
        let external_model = if external {
            Some(external_replay(dir)?)
        } else {
            None
        };
        if cfg["control_variant"] == "fixed_coordinates_free_selectors"
            && result["status"] == "UNKNOWN_conflict_cap"
        {
            require(
                sat["status"] == "COMPLETE"
                    && sat["terminal_counters"]["outcome"] == "Unknown"
                    && result["conflicts"] == cfg["conflicts"]
                    && sat["terminal_counters"]["conflicts"] == cfg["conflicts"]
                    && out["exit_code"] == 0,
                "control cap/trace disagree",
            )?;
            return Ok(
                json!({"schema":"n83.native-audit/v1","status":"UNKNOWN_CONTROL_CONFLICT_CAP","sat_trace":sat,"control":result,"outer_wall_ms":out["outer_wall_ms"],"selected_best_total_runtime":null}),
            );
        }
        let panel = Path::new(text(&cfg, "panel")?);
        let fixture_bytes = fs::read(panel.join("fixture.json"))?;
        require(
            card::sha(&fixture_bytes)
                == "fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538",
            "known fixture SHA256 differs",
        )?;
        let fixture: Value = serde_json::from_slice(&fixture_bytes)?;
        let mut sum = None;
        for term in fixture["terms"].as_array().ok_or("witness terms")? {
            sum = add(sum, point(&term["point"])?)?;
        }
        require(
            sum == point(&fixture["equation"]["target"])?,
            "independent original-group witness sum differs",
        )?;
        require(
            result["status"] == "SAT_verified_witness_x"
                && result["model_verified"] == true
                && (external
                    || (sat["status"] == "COMPLETE"
                        && sat["terminal_counters"]["outcome"] == "Sat"
                        && sat["terminal_counters"]["conflicts"] == result["conflicts"]))
                && result["conflict_cap"] == cfg["conflicts"]
                && result["fixed_x_blocks"] == 8
                && result["identity_mask"] == 0
                && result["input_blake3"] == hash(panel.join("domains.json"))?
                && result["fixture_blake3"] == blake3::hash(&fixture_bytes).to_hex().as_str(),
            "positive control not verified",
        )?;
        if cfg["control_variant"] == "fixed_coordinates_representatives_and_phases"
            || cfg["control_variant"] == "fixed_coordinates_free_selectors"
        {
            if cfg["control_variant"] == "fixed_coordinates_free_selectors" {
                require(
                    result["selector_export"] == "decoded_from_sat_model_after_solve",
                    "missing actual model selector export",
                )?;
            }
            let input = read(panel.join("domains.json"))?;
            let assignments = result["selector_assignments"]
                .as_array()
                .ok_or("missing selector witness")?;
            require(
                assignments.len() == 5 && result["control_variant"] == cfg["control_variant"],
                "selector witness variant differs",
            )?;
            for (i, a) in assignments.iter().enumerate() {
                if let Some(model) = &external_model {
                    let first = number(a, "selector_first_wire")? as usize;
                    let phase_first = number(a, "phase_first_wire")? as usize;
                    let count = number(a, "domain_size")? as usize;
                    let index = number(a, "index")? as usize;
                    for k in 0..count {
                        require(
                            model.get(first + k - 1) == Some(&(k == index)),
                            "external selector bit differs",
                        )?;
                    }
                    let mut actual_phase = 0;
                    for k in 0..7 {
                        if *model
                            .get(phase_first + k - 1)
                            .ok_or("phase wire outside model")?
                        {
                            actual_phase |= 1 << k;
                        }
                    }
                    require(
                        actual_phase == number(a, "phase")?,
                        "external phase bits differ",
                    )?;
                }
                let source = if i < 3 { "small_reps" } else { "residual_reps" };
                let mut reps = input[source]
                    .as_array()
                    .ok_or("domain representatives")?
                    .iter()
                    .map(|r| decimal(r, 1 << 83))
                    .collect::<R<Vec<_>>>()?;
                reps.sort_unstable();
                let index = number(a, "index")? as usize;
                require(
                    index < reps.len() && a["domain_size"] == reps.len(),
                    "selector index/size differs",
                )?;
                let representative = decimal(&a["representative"], 1 << 83)?;
                require(
                    representative == reps[index],
                    "selector representative differs",
                )?;
                let phase = number(a, "phase")?;
                require(phase < 83, "phase outside83")?;
                let mut x = representative;
                for _ in 0..phase {
                    x = mul(x, x);
                }
                require(
                    x == decimal(&input["ordered_xs"][i], 1 << 83)?,
                    "independent selector/phase replay mismatch",
                )?;
            }
        }
        require(
            (external || number(&result, "conflicts")? <= number(&cfg, "conflicts")?)
                && out["exit_code"] == 0
                && out["wall_censored"] == false,
            "control capped or conflicts exceeded",
        )?;
        if external {
            require(
                result["external_solver"] == "CryptoMiniSat5.17.0"
                    && result["dimacs_blake3"] == hash(dir.join("witness.cnf"))?,
                "external formula custody differs",
            )?;
        }
        return Ok(
            json!({"schema":"n83.native-audit/v1","status":"PASS_KNOWN_WITNESS_MODEL_AND_INDEPENDENT_GROUP_SUM","sat_trace":sat,"control":result,"outer_wall_ms":out["outer_wall_ms"],"natural_relations":null,"selected_best_total_runtime":null}),
        );
    }
    let ps = phases(&raw)?;
    let corpus = read(Path::new(text(&cfg, "fixtures")?).join("probe-corpus.json"))?;
    let q = point(
        &corpus["targets"]
            .as_array()
            .ok_or("corpus targets")?
            .iter()
            .find(|t| t["a"] == 0 && t["fixture"] == 0)
            .ok_or("primary fixture")?["point"],
    )?;
    let mut queries = Vec::new();
    for line in raw.lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v["schema"] == "n83.chain-query/v1" {
                query(&v, q)?;
                queries.push(v);
            }
        }
    }
    require(
        queries.len() <= 1,
        "more than one query in one-trial diagnostic",
    )?;
    if !ps.is_empty() {
        require(
            queries.len() == 1
                && ps[0]["data"]["target"] == queries[0]["target"]
                && ps[0]["data"]["summands"] == 5
                && ps[0]["data"]["points"] == number(&cfg, "columns")? * 166,
            "phase target/base differs from independently replayed query",
        )?;
    }
    for p in &ps {
        if p["phase"] == "domain_complete" {
            require(
                number(&p["data"], "variables")? <= 150000
                    && number(&p["data"], "domain_clauses")? <= 2000000,
                "phase model exceeds frozen guard",
            )?;
        }
    }
    let terminal = dir.join("run/summary.json");
    if dir.join("run/config.json").exists() {
        let wc = read(dir.join("run/config.json"))?;
        require(
            wc["source_commit"] == cfg["source_commit"]
                && wc["policy"] == "public_x_hash"
                && wc["seed"] == 2026100801u64
                && wc["orbit_columns"] == cfg["columns"]
                && wc["strategy"] == "orbit-chain-s3"
                && wc["summands"] == 5
                && wc["allow_direct_relation"] == false
                && wc["max_trials"] == 1
                && wc["budget_seconds"] == cfg["wall_seconds"]
                && wc["memory_cgroup_limit_bytes"] == 4294967296u64
                && wc["memory_cgroup_swap_limit_bytes"] == 0,
            "worker config differs from frozen diagnostic",
        )?;
        require(
            wc["chain_s3_limits"]
                == json!({"conflict_budget":cfg["conflicts"],"max_variables":150000,"max_domain_clauses":2000000,"max_models":32}),
            "worker solver limits differ",
        )?;
    }
    let mut counters = Value::Null;
    let status;
    if terminal.exists() {
        require(
            dir.join("run/config.json").exists(),
            "terminal lacks worker configuration",
        )?;
        let s = read(&terminal)?;
        require(
            s["source_commit"] == cfg["source_commit"]
                && s["orbit_columns"] == cfg["columns"]
                && s["summands"] == 5
                && s["verified_log"].is_null()
                && s["selected_best_total_runtime"].is_null(),
            "terminal identity or unsupported scalar claim",
        )?;
        require(
            s["status"] == "UNKNOWN_solver_cap",
            "unsupported terminal disposition",
        )?;
        let r = &s["report"];
        zero_counters(r, number(&cfg, "conflicts")?)?;
        require(
            s["relation_matrix"] == json!([]) && r["trials"] == 1,
            "zero-relation receipt has matrix/trial mismatch",
        )?;
        for rank in s["rank_history"].as_array().ok_or("rank history")? {
            require(
                rank["rank"] == 0
                    && rank["rows"] == 0
                    && rank["columns"] == number(&cfg, "columns")? + 1
                    && rank["candidate_produced"] == false
                    && rank["candidate_verified"] == false,
                "zero-relation rank history differs",
            )?;
        }
        require(
            out["exit_code"] == 0,
            "terminal with nonzero container exit",
        )?;
        require(
            ps.last().is_some_and(|p| {
                p["phase"] == "solver_return"
                    && p["data"]["outcome"] == "Unknown"
                    && p["data"]["conflicts"] == r["sat_conflicts"]
            }),
            "terminal disagrees with solver-return marker",
        )?;
        let attempts = s["attempt_records"].as_array().ok_or("attempt records")?;
        require(
            attempts.len() == queries.len() && attempts.len() == 1,
            "query cardinality differs",
        )?;
        for (a, qv) in attempts.iter().zip(&queries) {
            query(a, q)?;
            for k in ["a", "b", "target"] {
                require(a[k] == qv[k], "terminal query differs from phase query")?;
            }
        }
        require(
            number(r, "sat_conflicts")? <= number(&cfg, "conflicts")?,
            "conflict cap exceeded",
        )?;
        counters = json!({"conflicts":r["sat_conflicts"],"models":0,"relations":0,"rank":0});
        status = "PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY";
    } else {
        require(
            out["wall_censored"] == true,
            "missing terminal without wall censoring",
        )?;
        status = "PASS_CENSORED_PHASE_OBSERVATION";
    }
    Ok(
        json!({"schema":"n83.native-audit/v1","status":status,"columns":cfg["columns"],"phases":ps,"queries":queries,"terminal_counters":counters,"outer_wall_ms":out["outer_wall_ms"],"selected_best_total_runtime":null}),
    )
}
fn cms_conflicts(raw: &str) -> Option<u64> {
    raw.lines().find_map(|line| {
        let rest = line.strip_prefix("c conflicts")?;
        let value = rest.trim().strip_prefix(':')?.split_whitespace().next()?;
        value.parse().ok()
    })
}

fn external_replay(dir: &Path) -> R<Vec<bool>> {
    let formula = fs::read_to_string(dir.join("witness.cnf"))?;
    let mut lines = formula.lines();
    let header: Vec<_> = lines
        .next()
        .ok_or("no DIMACS header")?
        .split_whitespace()
        .collect();
    require(
        header.len() == 4 && header[0] == "p" && header[1] == "cnf",
        "bad DIMACS header",
    )?;
    let n: usize = header[2].parse()?;
    let count: usize = header[3].parse()?;
    let mut assignments = vec![None; n];
    for line in fs::read_to_string(dir.join("cms.model"))?.lines() {
        if ["SAT", "s SATISFIABLE"].contains(&line) || line.starts_with('c') {
            continue;
        }
        for token in line.split_whitespace().filter(|&x| x != "v") {
            let l: i64 = token.parse()?;
            if l == 0 {
                continue;
            }
            let index = usize::try_from(l.unsigned_abs())?
                .checked_sub(1)
                .ok_or("zero literal")?;
            let slot = assignments.get_mut(index).ok_or("model out of bounds")?;
            require(slot.replace(l > 0).is_none(), "duplicate model literal")?;
        }
    }
    let model: Vec<bool> = assignments
        .into_iter()
        .map(|x| x.ok_or("incomplete model"))
        .collect::<Result<_, _>>()?;
    let mut checked = 0;
    for line in lines {
        if line.starts_with('c') || line.trim().is_empty() {
            continue;
        }
        let xor = line.starts_with('x');
        let mut satisfied = false;
        let mut terminated = false;
        for token in line.trim_start_matches('x').split_whitespace() {
            let l: i64 = token.parse()?;
            if l == 0 {
                terminated = true;
                break;
            }
            let index = usize::try_from(l.unsigned_abs())?
                .checked_sub(1)
                .ok_or("zero literal")?;
            let literal = *model.get(index).ok_or("formula out of bounds")? == (l > 0);
            satisfied = if xor {
                satisfied ^ literal
            } else {
                satisfied || literal
            };
        }
        require(
            terminated && satisfied,
            "independent external CNF/XOR replay fails",
        )?;
        checked += 1;
    }
    require(checked == count, "DIMACS constraint count differs")?;
    let cfg = read(dir.join("config.json"))?;
    let domains = read(Path::new(text(&cfg, "panel")?).join("domains.json"))?;
    for (block, x) in domains["ordered_xs"]
        .as_array()
        .ok_or("xs")?
        .iter()
        .chain(domains["partial_xs"].as_array().ok_or("partials")?)
        .enumerate()
    {
        let x = decimal(x, 1 << 83)?;
        for bit in 0..83 {
            require(
                model.get(block * 83 + bit) == Some(&((x >> bit) & 1 != 0)),
                "external fixed coordinate differs",
            )?;
        }
    }
    Ok(model)
}

fn validate_run_budget(columns: u64, external: bool, conflicts: u64, wall: u64) -> R<()> {
    let natural = columns != 0 && !external;
    let wall_ceiling = if natural { 2400 } else { 900 };
    let conflict_ceiling = if external {
        1_000_000
    } else if natural {
        400_000
    } else {
        100_000
    };
    require((1..=wall_ceiling).contains(&wall), "invalid wall budget")?;
    require(
        (1..=conflict_ceiling).contains(&conflicts),
        "invalid conflicts",
    )
}

fn run(spec: &Path, dir: &Path) -> R<()> {
    native_signals::install();
    let s = read(spec)?;
    let wall = number(&s, "wall_seconds")?;
    let columns = number(&s, "columns")?;
    require(
        [0, 2048, 4096].contains(&columns),
        "only retained diagnostic sizes admitted",
    )?;
    let conflicts = number(&s, "conflicts")?;
    validate_run_budget(columns, s["external_solver"].is_string(), conflicts, wall)?;
    let checkout = fs::canonicalize(text(&s, "checkout")?)?;
    let clean = Command::new("git")
        .args(["-c", &format!("safe.directory={}", checkout.display())])
        .args(["status", "--porcelain"])
        .current_dir(&checkout)
        .output()?;
    require(
        clean.status.success() && clean.stdout.is_empty(),
        "checkout not clean",
    )?;
    let commit = Command::new("git")
        .args(["-c", &format!("safe.directory={}", checkout.display())])
        .args(["rev-parse", "HEAD"])
        .current_dir(&checkout)
        .output()?;
    require(commit.status.success(), "git rev-parse failed")?;
    let commit = String::from_utf8(commit.stdout)?.trim().to_owned();
    require(
        s["source_commit"] == commit,
        "source differs from pre-run freeze",
    )?;
    let panel = fs::canonicalize(text(&s, "panel")?)?;
    let fixtures = fs::canonicalize(text(&s, "fixtures")?)?;
    let worker = fs::canonicalize(text(&s, "worker")?)?;
    if columns != 0 {
        retained_panel(&read(panel.join("manifest.json"))?, columns)?;
    }
    require(
        hash(&worker)? == text(&s, "worker_blake3")?
            && hash(std::env::current_exe()?)? == text(&s, "runner_blake3")?,
        "binary differs from pre-run freeze",
    )?;
    require(
        inventory(&panel)? == serde_json::from_value(s["panel_blake3"].clone())?
            && inventory(&fixtures)? == serde_json::from_value(s["fixtures_blake3"].clone())?,
        "input differs from pre-run freeze",
    )?;
    require(fs::read(&worker)?.starts_with(b"\x7fELF"), "worker not ELF")?;
    let image = text(&s, "image")?;
    require(
        image.starts_with("sha256:") && image.len() == 71,
        "immutable image required",
    )?;
    fs::create_dir(dir)?;
    let dir = fs::canonicalize(dir)?;
    let reservation = if s["isolate"] == true {
        Some(reservation::Reservation::acquire(
            &dir,
            s.get("cpu_reservation"),
        )?)
    } else {
        None
    };
    fs::copy(spec, dir.join("spec.json"))?;
    let source = dir.join("source");
    for p in SOURCES {
        let dst = source.join(p);
        fs::create_dir_all(dst.parent().unwrap())?;
        fs::copy(checkout.join(p), dst)?;
    }
    write(
        source.join("attestation.json"),
        &json!({"schema":"n83.chain-s3-source-attestation/v1","source_commit":commit,"status_clean":true}),
    )?;
    let name = format!(
        "n83-native-{}-{}",
        std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
    );
    let cfg = json!({"schema":"n83.native-run-config/v1","control_variant":s["control_variant"],"columns":columns,"conflicts":conflicts,"wall_seconds":wall,"panel":panel,"fixtures":fixtures,"worker":worker,"image":image,"source_commit":commit,"panel_blake3":inventory(&panel)?,"fixtures_blake3":inventory(&fixtures)?,"source_blake3":inventory(&source)?,"worker_blake3":hash(&worker)?,"runner_blake3":hash(std::env::current_exe()?)?,"spec_blake3":hash(spec)?,"container_name":name,"isolate":s["isolate"],"xor_engine":s["xor_engine"],"external_solver_blake3":s["external_solver_blake3"]});
    write(dir.join("config.json"), &cfg)?;
    write(
        dir.join("custody.json"),
        &json!({"config_blake3":hash(dir.join("config.json"))?}),
    )?;
    let mounts = [
        format!("type=bind,src={},dst=/panel,readonly", panel.display()),
        format!(
            "type=bind,src={},dst=/fixtures,readonly",
            fixtures.display()
        ),
        format!("type=bind,src={},dst=/worker,readonly", worker.display()),
        format!("type=bind,src={},dst=/source,readonly", source.display()),
        format!("type=bind,src={},dst=/out", dir.display()),
    ];
    let start = Instant::now();
    let mut cmd = vec![
        "run".to_owned(),
        "--detach".into(),
        "--pull=never".into(),
        "--name".into(),
        name.clone(),
        "--network=none".into(),
        "--memory=4g".into(),
        "--memory-swap=4g".into(),
        "--cpus=1".into(),
        "--pids-limit=64".into(),
        "--cap-drop=ALL".into(),
        "--security-opt=no-new-privileges".into(),
        "--read-only".into(),
        "--tmpfs=/tmp:rw,noexec,nosuid,size=64m".into(),
        "--env=ICV1_FROZEN_SOURCE_DIR=/source".into(),
        "--env=N83_CHAIN_PHASE_TRACE=1".into(),
        "--env=N83_SAT_PHASE_TRACE=1".into(),
    ];
    let owner = fs::metadata(&dir)?;
    if s["external_solver"].is_string() {
        let external = fs::canonicalize(text(&s, "external_solver")?)?;
        require(
            hash(&external)? == text(&s, "external_solver_blake3")?,
            "external solver differs",
        )?;
        cmd.extend([
            "--mount".into(),
            format!("type=bind,src={},dst=/cms,readonly", external.display()),
            "--env=N83_CMS_PATH=/cms".into(),
        ]);
    }
    if let Some(r) = &reservation {
        cmd.push(format!("--cpuset-cpus={}", r.plan.run_cpu));
    }
    if !s["xor_engine"].is_null() {
        let engine = text(&s, "xor_engine")?;
        require(
            ["dense", "indexed_words"].contains(&engine),
            "invalid XOR engine",
        )?;
        cmd.push(format!("--env=N83_XOR_ENGINE={engine}"));
    }
    cmd.push("--env=RAYON_NUM_THREADS=1".into());
    cmd.extend(["--user".into(), format!("{}:{}", owner.uid(), owner.gid())]);
    for m in &mounts {
        cmd.extend(["--mount".into(), m.clone()]);
    }
    cmd.extend([
        image.into(),
        "/usr/bin/timeout".into(),
        "--signal=KILL".into(),
        format!("{wall}s"),
        "/worker".into(),
    ]);
    if columns == 0 {
        cmd.extend(["/panel/domains.json".into(), conflicts.to_string()]);
    } else {
        cmd.extend([
            "primary-orbit-chain-cold".into(),
            "/panel".into(),
            "/fixtures".into(),
            columns.to_string(),
            "public_x_hash".into(),
            "2026100801".into(),
            "5".into(),
            "1".into(),
            "150000".into(),
            "2000000".into(),
            "32".into(),
            conflicts.to_string(),
            wall.to_string(),
            "4096".into(),
            "/out/run".into(),
        ]);
    }
    let attempt = (|| -> R<()> {
        docker(&cmd.iter().map(String::as_str).collect::<Vec<_>>())?;
        let live = inspect(&name)?;
        write(dir.join("inspect-live.json"), &live)?;
        isolation(&live, image)?;
        if let Some(r) = &reservation {
            require(
                live["host"]["CpusetCpus"] == r.plan.run_cpu.to_string(),
                "CPU pin differs",
            )?;
        }
        let mut censored = false;
        loop {
            require(
                native_signals::interrupted().is_none(),
                "interrupted; cleanup and affinity restoration required",
            )?;
            let v = inspect(&name)?;
            if v["state"]["Running"] == false {
                if start.elapsed() >= Duration::from_secs(wall) && v["state"]["ExitCode"] == 137 {
                    censored = true;
                }
                break;
            }
            if start.elapsed() >= Duration::from_secs(wall) {
                censored = true;
                docker(&["kill", &name])?;
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        let final_inspect = inspect(&name)?;
        write(dir.join("inspect-final.json"), &final_inspect)?;
        let logs = docker(&["logs", &name])?;
        fs::write(dir.join("stdout.log"), logs.stdout)?;
        fs::write(dir.join("stderr.log"), logs.stderr)?;
        docker(&["rm", &name])?;
        let absent = docker(&["ps", "-aq", "--filter", &format!("name=^{name}$")])?;
        require(absent.stdout.is_empty(), "container remains")?;
        write(
            dir.join("outer.json"),
            &json!({"schema":"n83.native-outer/v1","wall_censored":censored,"outer_wall_ms":start.elapsed().as_secs_f64()*1000.,"exit_code":final_inspect["state"]["ExitCode"],"cleanup_verified":true,"stdout_blake3":hash(dir.join("stdout.log"))?,"stderr_blake3":hash(dir.join("stderr.log"))?}),
        )?;
        Ok(())
    })();
    if attempt.is_err() {
        let _ = docker(&["kill", &name]);
        if let Ok(v) = inspect(&name) {
            write(dir.join("failure-inspect.json"), &v)?;
        }
        if let Ok(logs) = docker(&["logs", &name]) {
            fs::write(dir.join("failure-stdout.log"), logs.stdout)?;
            fs::write(dir.join("failure-stderr.log"), logs.stderr)?;
        }
        let _ = docker(&["rm", "--force", &name]);
        let removed = docker(&["ps", "-aq", "--filter", &format!("name=^{name}$")])
            .is_ok_and(|v| v.stdout.is_empty());
        write(
            dir.join("failure-cleanup.json"),
            &json!({"cleanup_verified":removed,"admitted_as_search_result":false}),
        )?;
        write(
            dir.join("launcher-error.json"),
            &json!({"error":format!("{}",attempt.as_ref().unwrap_err())}),
        )?;
    }
    attempt?;
    if let Some(r) = &reservation {
        r.finish(&dir)?;
    }
    let a = match audit(&dir) {
        Ok(a) => a,
        Err(e) => {
            write(
                dir.join("audit-failure.json"),
                &json!({"error":e.to_string(),"data_preserved":true}),
            )?;
            return Err(e);
        }
    };
    write(dir.join("audit.json"), &a)?;
    println!("{}", a);
    require(
        columns != 0 || a["status"] == "PASS_KNOWN_WITNESS_MODEL_AND_INDEPENDENT_GROUP_SUM",
        "positive-control gate not met",
    )?;
    Ok(())
}
fn main() -> R<()> {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("summarize-xor") if a.len() == 4 => {
            xor_summary::summarize(Path::new(&a[2]), Path::new(&a[3]))
        }
        Some("check-sat-trace") if a.len() == 3 => {
            println!("{}", sat_trace(&fs::read_to_string(&a[2])?)?);
            Ok(())
        }
        Some("check-positive-trace") if a.len() == 3 => {
            let raw = fs::read_to_string(&a[2])?;
            let mut group = String::new();
            let mut segments = 0;
            let mut sat = 0;
            for line in raw.lines() {
                if let Ok(v) = serde_json::from_str::<Value>(line) {
                    if v["schema"] == "n83.chain-phase/v1" {
                        if v["phase"] == "closure_begin" && !group.is_empty() {
                            phases(&group)?;
                            segments += 1;
                            group.clear();
                        }
                        if v["phase"] == "solver_return" && v["data"]["outcome"] == "Sat" {
                            sat += 1;
                        }
                        group.push_str(line);
                        group.push('\n');
                    }
                }
            }
            if !group.is_empty() {
                phases(&group)?;
                segments += 1;
            }
            require(
                sat >= 4,
                "missing four positive m5/m6 ordered/unordered SAT returns",
            )?;
            println!(
                "{}",
                json!({"status":"PASS_TRACED_SMALL_FIELD_POSITIVE_BRANCHES","segments":segments,"sat_returns":sat,"scope":"existing original-group assertion test; no natural yield claim"})
            );
            Ok(())
        }
        Some("card-run") | Some("card-prompt") | Some("card-report") if a.len() == 3 => {
            match card::dispatch(&a[1], Path::new(&a[2])) {
                Ok(()) => Ok(()),
                Err(e) => {
                    eprintln!("native card refused: {e}");
                    std::process::exit(2)
                }
            }
        }
        Some("card-check") if a.len() == 4 => card::check(Path::new(&a[2]), Path::new(&a[3])),
        Some("freeze") | Some("freeze-free-selector") if a.len() == 12 => {
            require(
                a[1] != "freeze-free-selector" || a[7] == "0",
                "free-selector freeze requires control columns=0",
            )?;
            let checkout = fs::canonicalize(&a[2])?;
            let commit = Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&checkout)
                .output()?;
            require(commit.status.success(), "freeze git failed")?;
            let panel = fs::canonicalize(&a[3])?;
            let fixtures = fs::canonicalize(&a[4])?;
            let worker = fs::canonicalize(&a[5])?;
            write(
                &a[11],
                &json!({"control_variant":if a[1]=="freeze-free-selector" {"fixed_coordinates_free_selectors"}else if a[7]=="0" {"fixed_coordinates_representatives_and_phases"}else{"retained_natural_query"},"checkout":checkout,"panel":panel,"fixtures":fixtures,"worker":worker,"image":a[6],"columns":a[7].parse::<u64>()?,"conflicts":a[8].parse::<u64>()?,"wall_seconds":a[9].parse::<u64>()?,"protocol_sha256":a[10],"source_commit":String::from_utf8(commit.stdout)?.trim(),"worker_blake3":hash(&worker)?,"runner_blake3":hash(std::env::current_exe()?)?,"panel_blake3":inventory(&panel)?,"fixtures_blake3":inventory(&fixtures)?}),
            )
        }
        Some("run") if a.len() == 4 => run(Path::new(&a[2]), Path::new(&a[3])),
        Some("audit") if a.len() == 3 => {
            println!("{}", audit(Path::new(&a[2]))?);
            Ok(())
        }
        _ => Err("usage: n83-sweep-runner run SPEC_JSON NEW_DIR | audit RUN_DIR".into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_conflict_parser_requires_exact_summary_label() {
        assert_eq!(
            cms_conflicts("c decisions/conflicts : 2.3\nc conflicts : 5001\n"),
            Some(5001)
        );
        assert_eq!(cms_conflicts("c conflicts/s : 5001\n"), None);
        assert_eq!(cms_conflicts("c conflicts : nope\n"), None);
    }
    #[test]
    fn retained_policy_preflight_rejects_wrong_or_ambiguous_rows() {
        let row = json!({"a":0,"columns":2048,"policy":"public_x_hash","seed":2026100801u64});
        retained_panel(&json!({"bases":[row.clone()]}), 2048).unwrap();
        assert!(retained_panel(&json!({"bases":[row.clone()]}), 4096).is_err());
        assert!(retained_panel(&json!({"bases":[row.clone(),row.clone()]}), 2048).is_err());
        let mut wrong = row;
        wrong["policy"] = json!("public_x_sequential");
        assert!(retained_panel(&json!({"bases":[wrong]}), 2048).is_err());
    }
    #[test]
    fn original_group_and_query_forgery() {
        let g = json!(["5402229379304697578719232", "3600185575789834070382019"]);
        let p = point(&g).unwrap();
        assert_eq!(scale(p, ORDER).unwrap(), None);
        assert_eq!(mul(12345, inv(12345).unwrap()), 1);
        let v = json!({"a":"0","b":"1","target":g});
        query(&v, p).unwrap();
        let mut forged = v.clone();
        forged["a"] = json!("1");
        assert!(query(&forged, p).is_err());
    }
    #[test]
    fn phase_regression_rejected() {
        assert!(phases("{\"schema\":\"n83.chain-phase/v1\",\"phase\":\"a\",\"elapsed_ms\":2}\n{\"schema\":\"n83.chain-phase/v1\",\"phase\":\"b\",\"elapsed_ms\":1}").is_err());
    }
    #[test]
    fn isolation_forgery_rejected() {
        assert!(isolation(&json!({"host":{"Memory":0}}), "sha256:x").is_err());
    }
    #[test]
    fn zero_relation_forgery_rejected() {
        let r = json!({"relations":0,"sat_models":0,"terminal_matrix_rank":0,"verification_failures":0,"sat_invalid_models":0,"independent_relations":0,"sat_calls":1,"sat_unknowns":1,"sat_conflicts":1000});
        zero_counters(&r, 1000).unwrap();
        for key in [
            "relations",
            "sat_models",
            "terminal_matrix_rank",
            "verification_failures",
            "independent_relations",
        ] {
            let mut forged = r.clone();
            forged[key] = json!(1);
            assert!(zero_counters(&forged, 1000).is_err());
        }
        assert!(zero_counters(&r, 999).is_err());
    }
}
#[test]
fn sat_trace_distinguishes_censoring_and_rejects_forged_order() {
    let event = |phase: &str, ms: f64, conflicts: u64, outcome: Value| {
        json!({"schema":"n83.sat-phase/v1","phase":phase,"elapsed_ms":ms,"conflicts":conflicts,"outcome":outcome,"counter_scope":"solver_instance_cumulative"}).to_string()
    };
    let enter = event("solve_enter", 0., 0, Value::Null);
    let begin = event("initial_propagation_begin", 1., 0, Value::Null);
    let complete = event("initial_propagation_complete", 2., 0, Value::Null);
    let cdcl = event("cdcl_enter", 3., 0, Value::Null);
    let snapshot = event("conflict_snapshot", 4., 1000, Value::Null);
    let returned = event("solve_return", 5., 1000, json!("Unknown"));
    let prefix = [
        enter.clone(),
        begin.clone(),
        complete.clone(),
        cdcl.clone(),
        snapshot.clone(),
    ]
    .join("\n");
    let audit = sat_trace(&prefix).unwrap();
    assert_eq!(audit["status"], "VALID_PREFIX_NONTERMINAL");
    assert!(audit["terminal_counters"].is_null());
    assert_eq!(
        sat_trace(&format!("{prefix}\n{returned}")).unwrap()["terminal_counters"]["conflicts"],
        1000
    );
    assert!(sat_trace(&[enter.clone(), cdcl.clone()].join("\n")).is_err());
    assert!(sat_trace(&format!(
        "{prefix}\n{}",
        event("conflict_snapshot", 5., 999, Value::Null)
    ))
    .is_err());
    assert!(sat_trace(
        &[
            enter,
            begin,
            complete,
            cdcl,
            event("solve_return", 1., 0, json!("Sat"))
        ]
        .join("\n")
    )
    .is_err());
    assert!(sat_trace("").is_err());
}

#[test]
fn natural_budget_admits_400k_but_stays_bounded() {
    for columns in [2048, 4096] {
        assert!(validate_run_budget(columns, false, 200_000, 1200).is_ok());
        assert!(validate_run_budget(columns, false, 400_000, 2400).is_ok());
        assert!(validate_run_budget(columns, false, 400_001, 2400).is_err());
        assert!(validate_run_budget(columns, false, 400_000, 2401).is_err());
        assert!(validate_run_budget(columns, false, 0, 2400).is_err());
        assert!(validate_run_budget(columns, false, 400_000, 0).is_err());
    }
}

#[test]
fn witness_and_external_budgets_keep_their_old_boundaries() {
    assert!(validate_run_budget(0, false, 100_000, 900).is_ok());
    assert!(validate_run_budget(0, false, 100_001, 900).is_err());
    assert!(validate_run_budget(0, false, 100_000, 901).is_err());
    assert!(validate_run_budget(0, true, 1_000_000, 900).is_ok());
    assert!(validate_run_budget(0, true, 1_000_001, 900).is_err());
    assert!(validate_run_budget(0, true, 1_000_000, 901).is_err());
}
