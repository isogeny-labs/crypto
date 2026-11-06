//! Replay sentinels: one cheap committed record per registered method id
//! that a pull request replays through the ordinary verifier.
//!
//! Pull requests audit every committed session with `--replay 0`
//! (integrity only), so a change to what a method counts merged green and
//! the committed records of that method stopped replaying unnoticed
//! (c653ede60b changed the native pencil point sieve's ledger and the
//! `ic.three_leads_genus2_pencil_symbolic_v2` records of
//! `research/ecbench_registry_optimization_20261101/execution/
//! smoke-three-leads-corrected` stopped reproducing for days).  The weekly
//! lane replays everything; this module freezes the *choice* of what a pull
//! request replays, in `docs/ecbench/replay-sentinels.json`:
//!
//! * `scan` lists the replayable records of one hydrated session
//!   (`audit::replay_candidates`, the audit's own predicate);
//! * `build` picks, for every method id that appears in the plan of a
//!   committed session, the record of least replay cost over all sessions
//!   (recorded solve wall plus a charge for the session's audit I/O), with
//!   ties broken by path and sequence, so the pick is a pure function of
//!   the committed data and never random (method ids come from each session's
//!   `spec.json`, plain text in git);
//! * `check` fails when a method id of a committed session has neither a
//!   sentinel nor a recorded reason it has none, when a sentinel names a
//!   session that changed or vanished, or when an entry no committed
//!   session needs is left behind.
//!
//! The replay itself is `ecbench verify --replay-seq N`, the same
//! comparator and the same legacy profiles as every other audit
//! (`.github/scripts/ecbench-replay-sentinels.sh`).  Nothing here compares
//! counts.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::cryptanalysis::ecbench::audit;

pub const MANIFEST_SCHEMA: &str = "ecbench.replay-sentinels/v1";

/// Nanoseconds of replay cost charged per byte of a session's
/// `records.jsonl`: every sentinel's session is integrity-audited
/// whole (hashes and seals of every record) before the record replays.
pub const AUDIT_NS_PER_RECORD_BYTE: u64 = 2_000;

/// Replay cost used for a record without a recorded solve wall.
const UNKNOWN_WALL_NS: u64 = 3_600_000_000_000;

pub const RULE: &str = "per method id (the id in a committed plan's arms), the replayable record \
     (audit::replay_candidates) minimising solve_wall_ns + 2000 ns per byte of its session's \
     records.jsonl, among the cheapest `per_method` distinct sessions; ties by session path, seq";

/// One replayable record of one session, as `scan` prints it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub method: String,
    pub session: String,
    pub session_id: String,
    pub records_sha256: Option<String>,
    pub records_bytes: u64,
    pub seq: u64,
    pub record_id: String,
    pub solve_wall_ns: Option<u64>,
    pub total_gae: Option<f64>,
}

impl Candidate {
    pub fn cost_ns(&self) -> u64 {
        self.solve_wall_ns
            .unwrap_or(UNKNOWN_WALL_NS)
            .saturating_add(self.records_bytes.saturating_mul(AUDIT_NS_PER_RECORD_BYTE))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sentinel {
    pub method: String,
    pub session: String,
    pub session_id: String,
    pub records_sha256: Option<String>,
    pub seq: u64,
    pub record_id: String,
    pub recorded_solve_wall_us: Option<u64>,
    pub recorded_total_gae: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Unreplayable {
    pub method: String,
    pub reason: String,
}

/// A committed record that already failed to replay when the manifest was
/// generated, for a reason older than the change under test.  It cannot
/// show a regression (it fails whatever a pull request does) and would
/// block every pull request, so `build` skips it and says so here; the
/// weekly lane and `audit-runs` still see it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownFailure {
    pub session: String,
    pub seq: u64,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: String,
    pub rule: String,
    pub per_method: usize,
    pub sentinels: Vec<Sentinel>,
    pub unreplayable: Vec<Unreplayable>,
    #[serde(default)]
    pub known_failures: Vec<KnownFailure>,
}

/// What a committed session's spec plans: its method ids.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanMethods {
    pub session: String,
    pub methods: BTreeSet<String>,
}

/// The method ids in `dir/spec.json`'s arms: the ids the plan resolves
/// and every record carries.  `spec.json` is committed as plain text in
/// every session, where `plan.json` may be compressed and stored in the
/// artifact store, so the coverage check needs no hydration.
pub fn plan_methods(dir: &Path) -> Result<BTreeSet<String>, String> {
    let path = dir.join("spec.json");
    let doc: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?,
    )
    .map_err(|e| format!("{}: {e}", path.display()))?;
    let arms = doc["arms"]
        .as_array()
        .ok_or_else(|| format!("{}: no arms", path.display()))?;
    arms.iter()
        .map(|a| {
            a["method"]["id"]
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("{}: an arm without method.id", path.display()))
        })
        .collect()
}

fn session_identity(dir: &Path) -> Result<(String, Option<String>), String> {
    let path = dir.join("session.json");
    let doc: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?,
    )
    .map_err(|e| format!("{}: {e}", path.display()))?;
    let id = doc["session_id"]
        .as_str()
        .ok_or_else(|| format!("{}: no session_id", path.display()))?
        .to_string();
    Ok((id, doc["records_sha256"].as_str().map(str::to_string)))
}

/// The cheapest `keep` replayable records per method of the hydrated
/// session in `dir`, labelled with its repository path `label`.
pub fn scan_session(dir: &Path, label: &str, keep: usize) -> Result<Vec<Candidate>, String> {
    let (session_id, records_sha256) = session_identity(dir)?;
    let records_bytes = std::fs::metadata(dir.join("records.jsonl"))
        .map_err(|e| format!("{}/records.jsonl: {e}", dir.display()))?
        .len();
    let mut by_method: BTreeMap<String, Vec<Candidate>> = BTreeMap::new();
    for c in audit::replay_candidates(dir)? {
        by_method
            .entry(c.method_id.clone())
            .or_default()
            .push(Candidate {
                method: c.method_id,
                session: label.to_string(),
                session_id: session_id.clone(),
                records_sha256: records_sha256.clone(),
                records_bytes,
                seq: c.seq,
                record_id: c.record_id,
                solve_wall_ns: c.solve_wall_ns,
                total_gae: c.total_gae,
            });
    }
    let mut out = Vec::new();
    for (_, mut v) in by_method {
        v.sort_by(|a, b| (a.cost_ns(), a.seq).cmp(&(b.cost_ns(), b.seq)));
        v.truncate(keep);
        out.extend(v);
    }
    Ok(out)
}

/// The manifest for `methods` (every method id of every committed plan)
/// from the scanned candidates.  Pure: the same inputs give the same
/// bytes.
pub fn build(
    methods: &BTreeSet<String>,
    candidates: &[Candidate],
    per_method: usize,
    known_failures: &[KnownFailure],
) -> Manifest {
    let per_method = per_method.max(1);
    let mut by_method: BTreeMap<&str, Vec<&Candidate>> = BTreeMap::new();
    for c in candidates {
        if known_failures
            .iter()
            .any(|k| k.session == c.session && k.seq == c.seq)
        {
            continue;
        }
        by_method.entry(&c.method).or_default().push(c);
    }
    let had_known_failure: BTreeSet<&str> = candidates
        .iter()
        .filter(|c| {
            known_failures
                .iter()
                .any(|k| k.session == c.session && k.seq == c.seq)
        })
        .map(|c| c.method.as_str())
        .collect();
    let mut sentinels = Vec::new();
    let mut unreplayable = Vec::new();
    for m in methods {
        let mut cs = by_method.remove(m.as_str()).unwrap_or_default();
        cs.sort_by(|a, b| (a.cost_ns(), &a.session, a.seq).cmp(&(b.cost_ns(), &b.session, b.seq)));
        let mut sessions = BTreeSet::new();
        let mut taken = 0;
        for c in cs {
            if taken == per_method {
                break;
            }
            if !sessions.insert(c.session.clone()) {
                continue;
            }
            taken += 1;
            sentinels.push(Sentinel {
                method: c.method.clone(),
                session: c.session.clone(),
                session_id: c.session_id.clone(),
                records_sha256: c.records_sha256.clone(),
                seq: c.seq,
                record_id: c.record_id.clone(),
                recorded_solve_wall_us: c.solve_wall_ns.map(|n| n / 1000),
                recorded_total_gae: c.total_gae,
            });
        }
        if taken == 0 && had_known_failure.contains(m.as_str()) {
            // Every replayable record of this method is a known failure:
            // leave it uncovered so `check` names it, rather than exempt it.
            continue;
        }
        if taken == 0 {
            unreplayable.push(Unreplayable {
                method: m.clone(),
                reason: "no committed record is replayable (audit::replayable: nondeterministic \
                         method, or only warm-ups, timeouts, crashes or budgeted solves)"
                    .into(),
            });
        }
    }
    Manifest {
        schema: MANIFEST_SCHEMA.into(),
        rule: RULE.into(),
        per_method,
        sentinels,
        unreplayable,
        known_failures: {
            let mut k = known_failures.to_vec();
            k.sort_by(|a, b| (&a.session, a.seq).cmp(&(&b.session, b.seq)));
            k.dedup();
            k
        },
    }
}

/// Problems in `manifest` against the committed sessions: `plans` (the
/// method ids of each) and `identities` (session path to session id and
/// records hash, as `session.json` says).  Empty means the manifest
/// covers every method id.
pub fn check(
    manifest: &Manifest,
    plans: &[PlanMethods],
    identities: &BTreeMap<String, (String, Option<String>)>,
) -> Vec<String> {
    let mut problems = Vec::new();
    if manifest.schema != MANIFEST_SCHEMA {
        problems.push(format!(
            "schema {} is not {MANIFEST_SCHEMA}",
            manifest.schema
        ));
    }
    let needed: BTreeSet<&str> = plans
        .iter()
        .flat_map(|p| p.methods.iter().map(String::as_str))
        .collect();
    let covered: BTreeSet<&str> = manifest
        .sentinels
        .iter()
        .map(|s| s.method.as_str())
        .collect();
    let exempt: BTreeSet<&str> = manifest
        .unreplayable
        .iter()
        .map(|u| u.method.as_str())
        .collect();
    for m in &needed {
        if !covered.contains(m) && !exempt.contains(m) {
            problems.push(format!(
                "method id {m} appears in a committed session and has no replay sentinel: \
                 regenerate docs/ecbench/replay-sentinels.json (docs/ecbench/README.md, \
                 \"Replay sentinels\")"
            ));
        }
    }
    for m in covered.intersection(&exempt) {
        problems.push(format!("method id {m} is both a sentinel and unreplayable"));
    }
    for m in covered.iter().chain(exempt.iter()) {
        if !needed.contains(m) {
            problems.push(format!(
                "manifest entry for {m}, which no committed session plans (stale)"
            ));
        }
    }
    let mut seen = BTreeSet::new();
    for s in &manifest.sentinels {
        if !seen.insert((s.method.clone(), s.session.clone())) {
            problems.push(format!(
                "duplicate sentinel for {} in {}",
                s.method, s.session
            ));
        }
        match identities.get(&s.session) {
            None => problems.push(format!(
                "sentinel {} names {}, which is not a committed ecbench session",
                s.method, s.session
            )),
            Some((id, sha)) => {
                if *id != s.session_id || *sha != s.records_sha256 {
                    problems.push(format!(
                        "sentinel {} names {}, whose session id or records hash changed",
                        s.method, s.session
                    ));
                }
            }
        }
        if manifest
            .known_failures
            .iter()
            .any(|k| k.session == s.session && k.seq == s.seq)
        {
            problems.push(format!(
                "sentinel {} ({} seq {}) is listed as a known failure",
                s.method, s.session, s.seq
            ));
        }
        let planned = plans
            .iter()
            .any(|p| p.session == s.session && p.methods.contains(&s.method));
        if !planned {
            problems.push(format!(
                "sentinel {} names {}, whose plan has no such method",
                s.method, s.session
            ));
        }
    }
    let mut sorted = manifest.sentinels.clone();
    sorted.sort_by(|a, b| (&a.method, &a.session).cmp(&(&b.method, &b.session)));
    if sorted != manifest.sentinels {
        problems.push("sentinels are not sorted by (method, session)".into());
    }
    problems
}

/// Plans and identities of the sessions listed (one directory per line)
/// in `list`, relative to `root`.  A session without a readable
/// `spec.json` (a hash-pinned scratch session) is returned in the second
/// vector and plans nothing.
#[allow(clippy::type_complexity)]
pub fn load_sessions(
    root: &Path,
    list: &str,
) -> Result<
    (
        Vec<PlanMethods>,
        BTreeMap<String, (String, Option<String>)>,
        Vec<String>,
    ),
    String,
> {
    let mut plans = Vec::new();
    let mut ids = BTreeMap::new();
    let mut skipped = Vec::new();
    for dir in list.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let path = root.join(dir);
        let Ok(identity) = session_identity(&path) else {
            skipped.push(dir.to_string());
            continue;
        };
        ids.insert(dir.to_string(), identity);
        match plan_methods(&path) {
            Ok(methods) => plans.push(PlanMethods {
                session: dir.to_string(),
                methods,
            }),
            Err(_) => skipped.push(dir.to_string()),
        }
    }
    Ok((plans, ids, skipped))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(method: &str, session: &str, seq: u64, wall_ms: u64, bytes: u64) -> Candidate {
        Candidate {
            method: method.into(),
            session: session.into(),
            session_id: format!("ID-{session}"),
            records_sha256: Some(format!("sha-{session}")),
            records_bytes: bytes,
            seq,
            record_id: format!("ECR-{session}-{seq}"),
            solve_wall_ns: Some(wall_ms * 1_000_000),
            total_gae: Some(1.0),
        }
    }

    fn set(ms: &[&str]) -> BTreeSet<String> {
        ms.iter().map(|m| m.to_string()).collect()
    }

    fn world(
        sessions: &[(&str, &[&str])],
    ) -> (Vec<PlanMethods>, BTreeMap<String, (String, Option<String>)>) {
        let plans = sessions
            .iter()
            .map(|(s, ms)| PlanMethods {
                session: s.to_string(),
                methods: set(ms),
            })
            .collect();
        let ids = sessions
            .iter()
            .map(|(s, _)| (s.to_string(), (format!("ID-{s}"), Some(format!("sha-{s}")))))
            .collect();
        (plans, ids)
    }

    #[test]
    fn the_cheapest_record_wins_and_the_audit_io_is_charged() {
        // b is 5 ms cheaper to solve but sits in a session 100 kB larger,
        // which costs 200 ms of audit.
        let cs = vec![
            cand("m", "a", 3, 10, 1_000),
            cand("m", "b", 0, 5, 101_000),
            cand("m", "a", 9, 12, 1_000),
        ];
        let man = build(&set(&["m"]), &cs, 1, &[]);
        assert_eq!(man.sentinels.len(), 1);
        assert_eq!(
            (man.sentinels[0].session.as_str(), man.sentinels[0].seq),
            ("a", 3)
        );
        assert!(man.unreplayable.is_empty());
    }

    #[test]
    fn build_is_deterministic_and_breaks_ties_by_path_then_seq() {
        let cs = vec![
            cand("m", "z", 1, 10, 0),
            cand("m", "a", 7, 10, 0),
            cand("m", "a", 2, 10, 0),
        ];
        let mut rev = cs.clone();
        rev.reverse();
        let one = build(&set(&["m"]), &cs, 1, &[]);
        assert_eq!(one, build(&set(&["m"]), &rev, 1, &[]));
        assert_eq!(
            (one.sentinels[0].session.as_str(), one.sentinels[0].seq),
            ("a", 2)
        );
    }

    #[test]
    fn per_method_takes_distinct_sessions() {
        let cs = vec![
            cand("m", "a", 1, 1, 0),
            cand("m", "a", 2, 2, 0),
            cand("m", "b", 5, 50, 0),
            cand("m", "c", 5, 90, 0),
        ];
        let man = build(&set(&["m"]), &cs, 2, &[]);
        let got: Vec<_> = man
            .sentinels
            .iter()
            .map(|s| (s.session.as_str(), s.seq))
            .collect();
        assert_eq!(got, vec![("a", 1), ("b", 5)]);
    }

    #[test]
    fn a_method_without_a_replayable_record_is_listed_with_a_reason() {
        let man = build(&set(&["m", "nd"]), &[cand("m", "a", 0, 1, 0)], 1, &[]);
        assert_eq!(man.unreplayable.len(), 1);
        assert_eq!(man.unreplayable[0].method, "nd");
        let (plans, ids) = world(&[("a", &["m", "nd"])]);
        assert_eq!(check(&man, &plans, &ids), Vec::<String>::new());
    }

    #[test]
    fn a_new_method_id_without_a_sentinel_fails_the_check() {
        let man = build(&set(&["m"]), &[cand("m", "a", 0, 1, 0)], 1, &[]);
        let (plans, ids) = world(&[("a", &["m", "brand.new_v1"])]);
        let problems = check(&man, &plans, &ids);
        assert_eq!(problems.len(), 1, "{problems:?}");
        assert!(problems[0].contains("brand.new_v1"));
    }

    #[test]
    fn a_changed_or_missing_session_and_a_stale_entry_fail_the_check() {
        let man = build(
            &set(&["m", "old"]),
            &[cand("m", "a", 0, 1, 0), cand("old", "gone", 0, 1, 0)],
            1,
            &[],
        );
        let (plans, mut ids) = world(&[("a", &["m"]), ("gone", &["old"])]);
        assert!(check(&man, &plans, &ids).is_empty());
        ids.get_mut("a").unwrap().1 = Some("other".into());
        ids.remove("gone");
        let problems = check(&man, &plans, &ids);
        assert!(
            problems.iter().any(|p| p.contains("changed")),
            "{problems:?}"
        );
        assert!(
            problems.iter().any(|p| p.contains("not a committed")),
            "{problems:?}"
        );
        let (plans, ids) = world(&[("a", &["m"])]);
        let problems = check(&man, &plans, &ids);
        assert!(problems.iter().any(|p| p.contains("stale")), "{problems:?}");
    }

    #[test]
    fn a_known_failure_is_skipped_for_the_next_cheapest_record() {
        let cs = vec![cand("m", "old", 0, 1, 0), cand("m", "new", 4, 50, 0)];
        let kf = vec![KnownFailure {
            session: "old".into(),
            seq: 0,
            reason: "counters changed after it was written".into(),
        }];
        let man = build(&set(&["m"]), &cs, 1, &kf);
        assert_eq!(
            (man.sentinels[0].session.as_str(), man.sentinels[0].seq),
            ("new", 4)
        );
        assert_eq!(man.known_failures, kf);
        let (plans, ids) = world(&[("old", &["m"]), ("new", &["m"])]);
        assert!(check(&man, &plans, &ids).is_empty());
        // A method whose only replayable record is a known failure stays
        // uncovered, so the check names it instead of exempting it.
        let man = build(&set(&["m"]), &cs[..1], 1, &kf);
        assert!(man.sentinels.is_empty() && man.unreplayable.is_empty());
        let (plans, ids) = world(&[("old", &["m"])]);
        assert_eq!(check(&man, &plans, &ids).len(), 1);
        // A sentinel that is also a known failure is a problem.
        let mut bad = build(&set(&["m"]), &cs, 1, &[]);
        bad.known_failures = vec![KnownFailure {
            session: "old".into(),
            seq: 0,
            reason: String::new(),
        }];
        let (plans, ids) = world(&[("old", &["m"]), ("new", &["m"])]);
        assert!(check(&bad, &plans, &ids)
            .iter()
            .any(|p| p.contains("known failure")));
    }

    #[test]
    fn manifest_round_trips_through_json() {
        let man = build(&set(&["m"]), &[cand("m", "a", 0, 1, 0)], 1, &[]);
        let text = serde_json::to_string_pretty(&man).unwrap();
        assert_eq!(serde_json::from_str::<Manifest>(&text).unwrap(), man);
    }

    #[test]
    fn plan_and_session_files_are_read_from_a_directory() {
        let dir = std::env::temp_dir().join(format!("sentinels-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("spec.json"),
            r#"{"arms":[{"method":{"id":"x.y"}},{"method":{"id":"a.b"}},{"method":{"id":"x.y"}}]}"#,
        )
        .unwrap();
        std::fs::write(
            dir.join("session.json"),
            r#"{"session_id":"S","records_sha256":"h"}"#,
        )
        .unwrap();
        assert_eq!(plan_methods(&dir).unwrap(), set(&["a.b", "x.y"]));
        assert_eq!(
            session_identity(&dir).unwrap(),
            ("S".into(), Some("h".into()))
        );
        let (plans, ids, skipped) = load_sessions(
            dir.parent().unwrap(),
            &format!(
                "{}\nmissing-dir\n",
                dir.file_name().unwrap().to_str().unwrap()
            ),
        )
        .unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(ids.len(), 1);
        assert_eq!(skipped, vec!["missing-dir".to_string()]);
        std::fs::remove_dir_all(&dir).ok();
    }
}
