//! Native ownership accounting and comparison with a retained SAT trace prefix.
use super::{number, read, require, sat_trace, write, R};
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};

const COMPONENTS: &[&str] = &[
    "clause_arena",
    "clause_offsets",
    "detached_flags",
    "watch_headers",
    "watch_entries",
    "xor_row_headers",
    "xor_row_masks",
    "xor_matrix",
    "xor_matrix_rhs",
    "xor_support_headers",
    "xor_support_words",
    "xor_support_scratch",
    "xor_reason_headers",
    "xor_reason_literals",
    "xor_conflict",
    "xor_pivots",
    "xor_row_scratch",
    "literal_values",
    "assignment_bitsets",
    "levels_and_reasons",
    "phase_and_activity",
    "trail",
    "analysis_scratch",
    "branch_heap",
];

fn bytes(v: &Value, key: &str) -> R<u64> {
    number(v, key)
}
fn status_bytes(v: &Value, prefix: &str) -> R<Option<u64>> {
    let Some(lines) = v.as_array() else {
        return Ok(None);
    };
    for line in lines {
        let line = line.as_str().ok_or("worker status line")?;
        if let Some(rest) = line.strip_prefix(prefix) {
            let parts: Vec<_> = rest.split_whitespace().collect();
            require(parts.len() == 2 && parts[1] == "kB", "worker status unit")?;
            return Ok(Some(
                parts[0]
                    .parse::<u64>()?
                    .checked_mul(1024)
                    .ok_or("status overflow")?,
            ));
        }
    }
    Ok(None)
}
fn validate_storage(v: &Value) -> R<Value> {
    let reclamation = match (v.get("arena_reclamations"), v.get("arena_reclaimed_words")) {
        (None, None) => None,
        (Some(calls), Some(words)) => {
            let calls = calls.as_u64().ok_or("reclamation count integer")?;
            let words = words.as_u64().ok_or("reclaimed words integer")?;
            require(
                (calls == 0) == (words == 0),
                "reclamation counters disagree",
            )?;
            Some((calls, words))
        }
        _ => return Err("incomplete reclamation counters".into()),
    };
    require(v["schema"] == "n83.sat-storage/v1", "storage schema")?;
    let parts = v["components_capacity_bytes"]
        .as_object()
        .ok_or("storage components")?;
    require(
        parts.len() == COMPONENTS.len() && COMPONENTS.iter().all(|key| parts.contains_key(*key)),
        "storage component coverage differs",
    )?;
    let sum = parts.values().try_fold(0u64, |n, x| {
        n.checked_add(x.as_u64().ok_or("storage component integer")?)
            .ok_or("storage overflow")
    })?;
    let owned = bytes(v, "owned_capacity_bytes")?;
    require(sum == owned && owned > 0, "owned capacity total mismatch")?;
    let used = bytes(v, "arena_used_words")?;
    let cap = bytes(v, "arena_capacity_words")?;
    let word = bytes(v, "literal_word_bytes")?;
    require(word == 4 && cap >= used, "arena layout/capacity mismatch")?;
    require(
        cap.checked_mul(word) == parts.get("clause_arena").and_then(Value::as_u64),
        "arena byte capacity mismatch",
    )?;
    let original = bytes(v, "original_clause_words")?;
    let active = bytes(v, "active_learnt_clause_words")?;
    let dead = bytes(v, "detached_learnt_clause_words")?;
    require(
        original
            .checked_add(active)
            .and_then(|n| n.checked_add(dead))
            == Some(used),
        "arena partition mismatch",
    )?;
    let n = bytes(v, "stored_clauses")?;
    let n_orig = bytes(v, "original_clauses")?;
    let n_dead = bytes(v, "detached_learnt_clauses")?;
    require(
        n_orig <= n && n_dead <= n - n_orig && original >= n_orig && dead >= n_dead,
        "clause count partition mismatch",
    )?;
    let allocator = &v["allocator"];
    let allocator_live = if allocator.is_null() {
        None
    } else {
        require(
            allocator["backend"] == "glibc_mallinfo2",
            "allocator backend",
        )?;
        Some(
            bytes(allocator, "in_use_arena_bytes")?
                .checked_add(bytes(allocator, "mmap_bytes")?)
                .ok_or("allocator overflow")?,
        )
    };
    require(
        bytes(v, "worker_pid_in_namespace")? > 0,
        "worker PID missing",
    )?;
    let rss = status_bytes(&v["worker_process_status"], "VmRSS:")?.ok_or("worker RSS missing")?;
    let hwm = status_bytes(&v["worker_process_status"], "VmHWM:")?.ok_or("worker HWM missing")?;
    require(hwm >= rss, "worker high water below RSS")?;
    Ok(
        json!({"owned_capacity_bytes":owned,"components_capacity_bytes":parts,
        "arena_used_bytes":used.checked_mul(word).ok_or("arena overflow")?,
        "arena_capacity_bytes":cap.checked_mul(word).ok_or("arena overflow")?,
        "arena_reclamations":reclamation.map(|v|v.0),
        "arena_reclaimed_words":reclamation.map(|v|v.1),
        "original_clause_bytes":original*word,"active_learnt_clause_bytes":active*word,
        "detached_learnt_clause_bytes":dead*word,"detached_learnt_clauses":n_dead,
        "arena_capacity_fraction_of_owned":cap as f64*word as f64/owned as f64,
        "detached_fraction_of_used_words":if used>0 {Some(dead as f64/used as f64)}else{None},
        "allocator_in_use_and_mmap_bytes":allocator_live,
        "worker_rss_bytes":rss,
        "worker_hwm_bytes":hwm,
        "worker_pid_in_namespace":v["worker_pid_in_namespace"]}),
    )
}
fn signature(v: &Value) -> R<Value> {
    let counters: BTreeMap<_, _> = [
        "conflicts",
        "decisions",
        "propagations",
        "xor_passes",
        "xor_propagations",
    ]
    .into_iter()
    .map(|key| Ok((key, number(v, key)?)))
    .collect::<R<_>>()?;
    Ok(json!(counters))
}
fn compare_snapshots(events: &[Value], old: &[Value]) -> R<usize> {
    let mut reference = BTreeMap::new();
    for event in old.iter().filter(|e| e["phase"] == "conflict_snapshot") {
        require(
            reference
                .insert(number(event, "conflicts")?, signature(event)?)
                .is_none(),
            "duplicate reference snapshot",
        )?;
    }
    let mut matched = 0;
    let mut previous = None;
    for event in events.iter().filter(|e| e["phase"] == "conflict_snapshot") {
        let count = number(event, "conflicts")?;
        require(
            previous.is_none_or(|last| count > last),
            "duplicate or regressed snapshot",
        )?;
        require(
            reference.get(&count) == Some(&signature(event)?),
            "SAT snapshot signature differs or reference coverage missing",
        )?;
        previous = Some(count);
        matched += 1;
    }
    require(matched > 0, "no conflict snapshots to compare")?;
    Ok(matched)
}
fn storage_rows(events: &[Value]) -> R<Vec<Value>> {
    let mut rows = Vec::new();
    for event in events {
        let phase = super::text(event, "phase")?;
        let due = matches!(phase, "solve_enter" | "solve_return")
            || (phase == "conflict_snapshot" && number(event, "conflicts")?.is_multiple_of(10_000));
        require(
            due != event["storage"].is_null(),
            "storage sampling coverage differs",
        )?;
        if due {
            let mut row = validate_storage(&event["storage"])?;
            row["phase"] = event["phase"].clone();
            row["conflicts"] = event["conflicts"].clone();
            row["elapsed_ms"] = event["elapsed_ms"].clone();
            rows.push(row);
        }
    }
    Ok(rows)
}
fn compare_complete_1m_prefix(events: &[Value], old: &[Value]) -> R<usize> {
    let mut previous = None;
    let mut prefix = Vec::new();
    for event in events.iter().filter(|v| v["phase"] == "conflict_snapshot") {
        let count = number(event, "conflicts")?;
        require(
            previous.is_none_or(|n| count > n),
            "duplicate or regressed snapshot",
        )?;
        signature(event)?;
        if count <= 1_000_000 {
            prefix.push(event.clone());
        }
        previous = Some(count);
    }
    let matched = compare_snapshots(&prefix, old)?;
    require(matched == 1001, "full 1M prefix coverage required")?;
    Ok(matched)
}
pub fn summarize(dir: &Path, reference: &Path, out: &Path) -> R<()> {
    summarize_impl(dir, reference, out, false)
}

/// The candidate must show actual reclamation and match every reference
/// snapshot through the complete retained 1M prefix. Later storage samples
/// remain validated, but later search signatures are explicitly unpaired.
pub fn summarize_reclamation(dir: &Path, reference: &Path, out: &Path) -> R<()> {
    summarize_impl(dir, reference, out, true)
}
fn summarize_impl(dir: &Path, reference: &Path, out: &Path, reclamation: bool) -> R<()> {
    let audit = super::audit(dir)?;
    let reference_audit = super::audit(reference)?;
    require(
        audit["status"] == "PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY"
            && reference_audit["status"] == "PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY",
        "natural full audits required",
    )?;
    let cfg = read(dir.join("config.json"))?;
    let ref_cfg = read(reference.join("config.json"))?;
    require(
        cfg["columns"] == ref_cfg["columns"]
            && cfg["panel_blake3"] == ref_cfg["panel_blake3"]
            && cfg["fixtures_blake3"] == ref_cfg["fixtures_blake3"],
        "reference inputs differ",
    )?;
    let trace = sat_trace(&std::fs::read_to_string(dir.join("stderr.log"))?)?;
    let retained = sat_trace(&std::fs::read_to_string(reference.join("stderr.log"))?)?;
    require(
        trace["status"] == "COMPLETE" && retained["status"] == "COMPLETE",
        "completed traces required",
    )?;
    let events = trace["events"].as_array().ok_or("events")?;
    let old = retained["events"].as_array().ok_or("reference events")?;
    let reference_limit = number(&ref_cfg, "conflicts")?;
    let matched = if reclamation {
        require(
            reference_limit == 1_000_000,
            "complete retained 1M reference required",
        )?;
        let matched = compare_complete_1m_prefix(events, old)?;
        require(
            matched == 1001 && number(&cfg, "conflicts")? >= reference_limit,
            "full 1M prefix coverage required",
        )?;
        matched
    } else {
        compare_snapshots(events, old)?
    };
    let snapshots = storage_rows(events)?;
    if reclamation {
        let mut previous = (0, 0);
        for row in &snapshots {
            let current = (
                number(row, "arena_reclamations")?,
                number(row, "arena_reclaimed_words")?,
            );
            require(
                current.0 >= previous.0 && current.1 >= previous.1,
                "reclamation counters regressed",
            )?;
            previous = current;
        }
        require(
            previous.0 > 0 && previous.1 > 0,
            "no arena reclamation observed",
        )?;
    }
    require(
        matched > 0 && snapshots.len() >= 2,
        "missing storage profile/snapshot comparison",
    )?;
    require(
        snapshots.first().unwrap()["phase"] == "solve_enter"
            && snapshots.last().unwrap()["phase"] == "solve_return",
        "storage boundary coverage missing",
    )?;
    let mut terminal_components: Vec<_> = snapshots.last().unwrap()["components_capacity_bytes"]
        .as_object()
        .ok_or("terminal components")?
        .iter()
        .map(|(name, value)| Ok((name, value.as_u64().ok_or("component integer")?)))
        .collect::<R<_>>()?;
    terminal_components.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    let result = json!({"schema":"n83.sat-storage-summary/v1","status":"PASS_OWNERSHIP_COUNTERS_AND_RETAINED_SNAPSHOT_SIGNATURES",
        "dir":dir,"reference_dir":reference,"columns":cfg["columns"],
        "source_commit":cfg["source_commit"],"worker_blake3":cfg["worker_blake3"],
        "terminal_counters":audit["terminal_counters"],"matched_conflict_snapshots":matched,
        "reclamation_required":reclamation,
        "signature_comparison_limit_conflicts":if reclamation {Some(reference_limit)}else{None},
        "later_search_signatures_compared":false,
        "n_storage_snapshots":snapshots.len(),"snapshots":snapshots,
        "terminal_components_descending":terminal_components,
        "timing_admission":"diagnostic instrumented worker; no matched clock or speed claim",
        "scope":"requested Vec capacities by owner; independent process allocator/RSS snapshots; no allocation stack trace or per-buffer residency attribution"});
    write(out, &result)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Value {
        let mut components: BTreeMap<_, u64> = COMPONENTS.iter().map(|&key| (key, 0)).collect();
        components.insert("clause_arena", 64);
        components.insert("watch_entries", 16);
        json!({"schema":"n83.sat-storage/v1","components_capacity_bytes":components,"owned_capacity_bytes":80,
 "arena_used_words":10,"arena_capacity_words":16,"literal_word_bytes":4,"original_clause_words":4,"active_learnt_clause_words":3,"detached_learnt_clause_words":3,
 "stored_clauses":3,"original_clauses":1,"detached_learnt_clauses":1,"allocator":null,"worker_pid_in_namespace":2,"worker_process_status":["VmRSS:\t12 kB","VmHWM:\t16 kB"]})
    }
    #[test]
    fn inventory_sums_and_worker_units_are_checked() {
        let v = validate_storage(&fixture()).unwrap();
        assert_eq!(v["worker_rss_bytes"], 12288);
        assert_eq!(v["arena_used_bytes"], 40);
        assert_eq!(v["detached_learnt_clause_bytes"], 12);
    }
    #[test]
    fn reclamation_counters_fail_closed() {
        let mut v = fixture();
        v["arena_reclamations"] = json!(1);
        assert!(validate_storage(&v).is_err());
        v["arena_reclaimed_words"] = json!(0);
        assert!(validate_storage(&v).is_err());
        v["arena_reclaimed_words"] = json!(30);
        assert_eq!(validate_storage(&v).unwrap()["arena_reclaimed_words"], 30);
        v["arena_reclamations"] = json!(-1);
        assert!(validate_storage(&v).is_err());
    }
    #[test]
    fn continuation_requires_the_entire_reference_prefix() {
        let old: Vec<_> = (0..=1000)
            .map(|i| {
                json!({"phase":"conflict_snapshot",
            "conflicts":i*1000,"decisions":i,"propagations":i,
            "xor_passes":i,"xor_propagations":i})
            })
            .collect();
        let mut candidate = old.clone();
        let mut later = old[1000].clone();
        later["conflicts"] = json!(1_001_000);
        candidate.push(later.clone());
        assert_eq!(compare_complete_1m_prefix(&candidate, &old).unwrap(), 1001);
        candidate.push(later);
        assert!(compare_complete_1m_prefix(&candidate, &old).is_err());
        candidate.pop();
        candidate.remove(10);
        assert!(compare_complete_1m_prefix(&candidate, &old).is_err());
        let mut changed = old.clone();
        changed[500]["decisions"] = json!(9);
        assert!(compare_complete_1m_prefix(&changed, &old).is_err());
        assert!(compare_complete_1m_prefix(&old[..1000], &old).is_err());
    }
    #[test]
    fn forged_capacity_and_partition_are_refused() {
        let mut v = fixture();
        v["owned_capacity_bytes"] = json!(79);
        assert!(validate_storage(&v).is_err());
        let mut v = fixture();
        v["detached_learnt_clause_words"] = json!(4);
        assert!(validate_storage(&v).is_err());
    }
    #[test]
    fn worker_status_requires_explicit_kib_units() {
        let mut v = fixture();
        v["worker_process_status"] = json!(["VmRSS: 12 MB"]);
        assert!(validate_storage(&v).is_err());
    }
    #[test]
    fn incomplete_component_and_sampling_coverage_is_refused() {
        let mut v = fixture();
        v["components_capacity_bytes"]
            .as_object_mut()
            .unwrap()
            .remove("watch_entries");
        v["owned_capacity_bytes"] = json!(64);
        assert!(validate_storage(&v).is_err());
        assert!(storage_rows(&[json!({"phase":"conflict_snapshot","conflicts":10_000})]).is_err());
        assert!(storage_rows(&[
            json!({"phase":"conflict_snapshot","conflicts":1_000,"storage":fixture()})
        ])
        .is_err());
    }
    #[test]
    fn prefix_comparison_refuses_changed_missing_or_duplicate_counters() {
        let event = json!({"phase":"conflict_snapshot","conflicts":1000,"decisions":2000,
            "propagations":3000,"xor_passes":4000,"xor_propagations":5000});
        assert_eq!(
            compare_snapshots(std::slice::from_ref(&event), std::slice::from_ref(&event)).unwrap(),
            1
        );
        let mut missing = event.clone();
        missing.as_object_mut().unwrap().remove("decisions");
        assert!(compare_snapshots(
            std::slice::from_ref(&missing),
            std::slice::from_ref(&missing)
        )
        .is_err());
        let mut forged = event.clone();
        forged["decisions"] = json!(2001);
        assert!(compare_snapshots(&[forged], std::slice::from_ref(&event)).is_err());
        assert!(compare_snapshots(std::slice::from_ref(&event), &[]).is_err());
        assert!(compare_snapshots(
            &[event.clone(), event.clone()],
            std::slice::from_ref(&event)
        )
        .is_err());
    }
}
