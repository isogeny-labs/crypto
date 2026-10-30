//! Adapter for the frozen native CM experiment. No validation scalar is an input.
//! Each invocation owns its table; the Rust replay uses separate affine arithmetic.
use super::{
    canonical::sha256_hex,
    methods::{unpriced_of, OnlineWindow, PhaseRecord, ResolvedMethod, SolveReport},
};
use crate::cryptanalysis::{
    ic_boundary::{CountedGroup, GroupOps, IncrementalGauss, PrimeInstance, PrimePoint, RowStatus},
    ic_framework::cm_orbit::{action, derive_map},
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
static NEXT: AtomicU64 = AtomicU64::new(0);
#[cfg(unix)]
fn monotonic_ns() -> Result<u64, String> {
    let mut stamp = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut stamp) } != 0 {
        return Err("monotonic clock unavailable".into());
    }
    Ok(stamp.tv_sec as u64 * 1_000_000_000 + stamp.tv_nsec as u64)
}
#[cfg(not(unix))]
fn monotonic_ns() -> Result<u64, String> {
    Err("native CM backend requires the Unix monotonic-clock domain".into())
}
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn u(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64()
        .ok_or_else(|| format!("backend missing integer {k}"))
}
fn parse_row(s: &str) -> Result<Vec<u64>, String> {
    s.split(',')
        .map(|x| x.parse().map_err(|_| "malformed native CSV".into()))
        .collect()
}
/// The `_v2` ids charge the Rust replay's counted operations and name the
/// native counters they leave unpriced; `_v1` keeps the accounting its
/// committed records were written under (`total_gae` 0, prose `unpriced`).
pub fn is_v2(id: &str) -> bool {
    id.starts_with("ic.cm_folded_") && id.ends_with("_v2")
}

/// The recurrence control, either version.
fn is_recurrence(id: &str) -> bool {
    id == "ic.cm_folded_recurrence_v1" || id == "ic.cm_folded_recurrence_v2"
}

/// The `D = −43` control, either version.
fn is_d43(id: &str) -> bool {
    id == "ic.cm_folded_native_d43_v1" || id == "ic.cm_folded_native_d43_v2"
}

/// The phases, total and unpriced list of a report.
///
/// `_v1` (frozen): `native_cold_backend` and `independent_replay` with
/// `gae` 0, `total_gae` 0 (the report's default) and one prose `unpriced`
/// entry; every group operation of the replay sits in the phase's `adds`,
/// `doubles` and `scalar_mults` only.
///
/// `_v2`: the replay is the counted work, so `independent_replay.gae` is its
/// additions plus doublings (the base scalar replays, the relation sums, the
/// `[α]G` of each relation and the final `[d]G = Q`) and `total_gae` is the
/// phase sum.  Native work stays unpriced, named by its `*_uncharged`
/// counters, as every other method names it; the Vélu map derivation's group
/// operations (`prep_ops`), which no phase carries, join them as
/// `cm_map_derivation_group_ops_uncharged`.
pub(super) fn account(
    v2: bool,
    audit_ops: GroupOps,
    prep_ops: GroupOps,
    backend_wall_ns: Option<u64>,
    replay_ns: u64,
    counters: &mut BTreeMap<String, u64>,
) -> (Vec<PhaseRecord>, f64, Vec<String>) {
    let phases = vec![
        PhaseRecord {
            name: "native_cold_backend".into(),
            wall_ns: backend_wall_ns,
            ..Default::default()
        },
        PhaseRecord {
            name: "independent_replay".into(),
            adds: audit_ops.adds,
            doubles: audit_ops.doubles,
            scalar_mults: audit_ops.scalar_mults,
            gae: if v2 { audit_ops.gae() } else { 0.0 },
            wall_ns: Some(replay_ns),
            ..Default::default()
        },
    ];
    if !v2 {
        let unpriced = vec!["native field/group operations, hash probes, table scans, modular exponentiation, elimination and CM derivation lack a complete common price".into()];
        return (phases, 0.0, unpriced);
    }
    if prep_ops.adds + prep_ops.doubles > 0 {
        counters.insert(
            "cm_map_derivation_group_ops_uncharged".into(),
            prep_ops.adds + prep_ops.doubles,
        );
    }
    let total = phases.iter().map(|p| p.gae).sum();
    (phases, total, unpriced_of(counters))
}

/// What `solve` refuses on every execution, decided at plan time: a curve
/// that is not of cofactor one and prime order, a base size outside
/// `4..=16384` or not a multiple of four, a query cap not below `r`, a
/// watchdog that is not a positive number, a mode other than `scalar` /
/// `velu` (`velu` is refused for the recurrence control), and a curve with
/// no CM orbit action for the control's discriminant (`D = −19`, or `−43`
/// for the degree-11 control).  The backend binary itself is not checked
/// here: it lives on the host that runs the arm.
pub fn applicable(m: &ResolvedMethod, inst: &PrimeInstance) -> Result<(), String> {
    if inst.cofactor != 1 || inst.group_order != inst.r {
        return Err("CM folded controls require a cofactor-one prime-order curve".into());
    }
    let size: usize = m.params["size"].parse().map_err(|_| "bad base size")?;
    let limit: u64 = m.params["query_limit"]
        .parse()
        .map_err(|_| "bad query cap")?;
    if !(4..=16384).contains(&size) || !size.is_multiple_of(4) || limit >= inst.r {
        return Err(format!(
            "unsupported CM base or noninjective query cap: size {size} (a multiple of 4 in 4..=16384), query_limit {limit} must be below r = {}",
            inst.r
        ));
    }
    let seconds: f64 = m.params["seconds"]
        .parse()
        .map_err(|_| "bad wall watchdog")?;
    if !seconds.is_finite() || seconds <= 0.0 {
        return Err("watchdog must be finite and positive".into());
    }
    match (m.params["mode"].as_str(), is_recurrence(&m.id)) {
        ("scalar", _) | ("velu", false) => {}
        _ => return Err("CM mode must be velu or scalar".into()),
    }
    let abs_d = if is_d43(&m.id) { 43 } else { 19 };
    action(inst, abs_d)
        .map(|_| ())
        .map_err(|e| format!("no CM orbit action for D = -{abs_d} on this curve: {e}"))
}

pub fn solve(
    m: &ResolvedMethod,
    inst: &PrimeInstance,
    q: PrimePoint,
    seed: u64,
) -> Result<SolveReport, String> {
    monotonic_ns()?;
    if inst.cofactor != 1 || inst.group_order != inst.r {
        return Err("CM folded controls require a cofactor-one prime-order curve".into());
    }
    let recurrence = is_recurrence(&m.id);
    let start = Instant::now();
    let bytes = fs::read(&m.params["backend"]).map_err(|e| format!("native backend: {e}"))?;
    if sha256_hex(&bytes) != m.params["backend_sha256"] {
        return Err("native backend hash mismatch".into());
    }
    let size: usize = m.params["size"].parse().map_err(|_| "bad base size")?;
    let limit: u64 = m.params["query_limit"]
        .parse()
        .map_err(|_| "bad query cap")?;
    if !(4..=16384).contains(&size) || !size.is_multiple_of(4) || limit >= inst.r {
        return Err("unsupported CM base or noninjective query cap".into());
    }
    let seconds: f64 = m.params["seconds"]
        .parse()
        .map_err(|_| "bad wall watchdog")?;
    if !seconds.is_finite() || seconds <= 0.0 {
        return Err("watchdog must be finite and positive".into());
    }
    let abs_d = if is_d43(&m.id) { 43 } else { 19 };
    let (_, lambda, degree) = action(inst, abs_d)?;
    let mut prep_ops = GroupOps::default();
    let map = match m.params["mode"].as_str() {
        "velu" if !recurrence => Some(derive_map(inst, abs_d, &mut prep_ops)?),
        "scalar" => None,
        _ => return Err("CM mode must be velu or scalar".into()),
    };
    let dir = Scratch(std::env::temp_dir().join(format!(
        "crypto-cm-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::SeqCst)
    )));
    fs::create_dir(&dir.0).map_err(|e| e.to_string())?;
    for d in ["factor-bases", "relations"] {
        fs::create_dir(dir.0.join(d)).map_err(|e| e.to_string())?;
    }
    let (xs, ts, ys, ic, icu) = map
        .as_ref()
        .map(|v| {
            (
                v.kernel_x.clone(),
                v.kernel_t.clone(),
                v.kernel_rhs.clone(),
                v.inv_c,
                v.inv_cu,
            )
        })
        .unwrap_or((
            vec![0; ((degree - 1) / 2) as usize],
            vec![0; ((degree - 1) / 2) as usize],
            vec![0; ((degree - 1) / 2) as usize],
            1,
            1,
        ));
    let mut values = vec![inst.curve.p, inst.r, 0, lambda, ic, icu];
    values.extend(xs);
    values.extend(ts);
    values.extend(ys);
    let params = format!(
        "{}\n",
        values
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
    fs::write(dir.0.join("params"), params).map_err(|e| e.to_string())?;
    fs::write(
        dir.0.join("work.csv"),
        format!(
            "job,pair,role,p,r,a,b,Gx,Gy,Qx,Qy,seed,repeat\n0,0,{},{},{},{},{},{},{},{},{},{},0\n",
            if recurrence {
                4
            } else {
                u8::from(map.is_none())
            },
            inst.curve.p,
            inst.r,
            inst.curve.a,
            inst.curve.b,
            inst.generator.0,
            inst.generator.1,
            q.x,
            q.y,
            seed
        ),
    )
    .map_err(|e| e.to_string())?;
    let out = Command::new(&m.params["backend"])
        .arg(dir.0.join("work.csv"))
        .arg(size.to_string())
        .arg(limit.to_string())
        .arg(&m.params["seconds"])
        .arg("1")
        .arg(&dir.0)
        .arg(dir.0.join("params"))
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!(
            "native backend failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let raw: Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("native result: {e}"))?;
    let replay_start = Instant::now();
    let fb = fs::read_to_string(dir.0.join("factor-bases/job-0.csv"))
        .map_err(|e| format!("{}: {e}", dir.0.join("factor-bases/job-0.csv").display()))?;
    let rel = fs::read_to_string(dir.0.join("relations/job-0.jsonl"))
        .map_err(|e| format!("{}: {e}", dir.0.join("relations/job-0.jsonl").display()))?;
    let base: Vec<Vec<u64>> = fb
        .lines()
        .skip(1)
        .map(parse_row)
        .collect::<Result<_, _>>()?;
    if base.len() != size {
        return Err("truncated base".into());
    }
    let c = &inst.curve;
    let mut audit_ops = GroupOps::default();
    // Reconstruct the public seed law independently, then check each claimed scalar.
    let mut seeds = Vec::new();
    let mut x = 1;
    while seeds.len() < 4 {
        let cand = x;
        x += 1;
        if cand == inst.generator.0 {
            continue;
        }
        if let Some(y) = c.sqrt(c.rhs(cand)) {
            if y != 0 {
                seeds.push(PrimePoint::affine(cand, y.min(c.p - y)));
            }
        }
    }
    for (i, row) in base.iter().enumerate() {
        if row.len() != 5
            || row[0] != i as u64
            || row[1] as usize != i / (size / 4)
            || row[2] >= inst.r
        {
            return Err("invalid base row".into());
        }
        let p = PrimePoint::affine(row[3], row[4]);
        if !c.is_on_curve(p) || c.mul(&mut audit_ops, seeds[row[1] as usize], row[2]) != p {
            return Err("independent base scalar replay failed".into());
        }
    }
    let mut matrix = IncrementalGauss::new(5, inst.r);
    let mut relations = Vec::new();
    for line in rel.lines() {
        let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let i = u(&v, "i")? as usize;
        let j = u(&v, "j")? as usize;
        if i >= size || j >= size {
            return Err("relation index out of bounds".into());
        }
        let si = v["si"].as_i64().ok_or("bad relation sign")?;
        let sj = v["sj"].as_i64().ok_or("bad relation sign")?;
        if ![-1, 1].contains(&si) || ![-1, 1].contains(&sj) {
            return Err("invalid relation sign".into());
        }
        let mut row = vec![0; 5];
        let mut sum = PrimePoint::INFINITY;
        for (k, sign) in [(i, si), (j, sj)] {
            let b = &base[k];
            let p = PrimePoint::affine(b[3], b[4]);
            sum = c.add(&mut audit_ops, sum, if sign == 1 { p } else { c.neg(p) });
            let coef = if sign == 1 { b[2] } else { inst.r - b[2] };
            row[b[1] as usize] =
                ((row[b[1] as usize] as u128 + coef as u128) % inst.r as u128) as u64;
        }
        let alpha = u(&v, "alpha")?;
        let ag = c.mul(&mut audit_ops, inst.generator_point(), alpha);
        let expected = c.add(&mut audit_ops, ag, q);
        if sum != expected || sum != PrimePoint::affine(u(&v, "Rx")?, u(&v, "Ry")?) {
            return Err("independent relation replay failed".into());
        }
        row[4] = inst.r - 1;
        let status = matrix.add_row(row, alpha);
        if matches!(status, RowStatus::Inconsistent) || matrix.rank() as u64 != u(&v, "rank")? {
            return Err("independent rank mismatch".into());
        }
        relations.push(v);
    }
    let recovered = if raw["verified"] == true {
        let d = u(&raw, "recovered")?;
        if matrix.pinned(4) != Some(d) || c.mul(&mut audit_ops, inst.generator_point(), d) != q {
            return Err("independent recovery check failed".into());
        }
        Some(d as u128)
    } else {
        None
    };
    let queries = u(&raw, "queries")?;
    let origin = ((seed % inst.r) as u128 + recovered.unwrap_or(0)) % inst.r as u128;
    let opposite_sum = (inst.r as u128 - 2 * origin % inst.r as u128) % inst.r as u128;
    let opposite_pair_absent = recovered.is_some()
        && queries < (inst.r / 2)
        && (queries < 2 || !(1..=2 * queries - 3).contains(&(opposite_sum as u64)));
    let replay_ns = replay_start.elapsed().as_nanos() as u64;
    let online_ns = monotonic_ns()?
        .checked_sub(u(&raw, "online_start_monotonic_ns")?)
        .ok_or("invalid online clock")?;
    let mut phases_ns = BTreeMap::new();
    for phase in [
        "target_query",
        "target_PDP",
        "target_relation_check",
        "target_descent",
        "target_recovery_check",
    ] {
        let val = raw[format!("{phase}_wall_seconds")]
            .as_f64()
            .ok_or("missing native phase")?;
        if !val.is_finite() || val < 0.0 {
            return Err("invalid native exclusive phase".into());
        }
        phases_ns.insert(phase.into(), (val * 1e9) as u64);
    }
    let subtotal: u64 = phases_ns.values().sum();
    let residual = online_ns
        .checked_sub(subtotal)
        .ok_or("online phases exceed parent clock")?;
    *phases_ns.get_mut("target_recovery_check").unwrap() += residual;
    let online=OnlineWindow{wall_ns:online_ns,phases_ns,zero_phases:BTreeMap::new(),start_event:"backend begins public target query setup after base/table setup".into(),stop_event:"Rust independent scalar/base/relation/rank replay and final recovery check completed".into(),included_stages:vec!["native query/PDP/check/elimination/recovery".into(),"child teardown, output parsing, complete independent Rust replay".into()],mapping:"Native exclusive diagnostic phases plus all interprocess, parsing, independent base and relation replay overhead charged to target_recovery_check; no gaps removed from online interval. Map derivation and reusable base/table setup are supplementary cold costs.".into()};
    let receipt = json!({"schema":"cm-folded.native-replay/v1","base_sha256":sha256_hex(fb.as_bytes()),"relations_sha256":sha256_hex(rel.as_bytes()),"base_points_checked":base.len(),"relations_checked":relations.len(),"rank":matrix.rank(),"recovery":recovered.map(|d|d.to_string()),"no_equal_query_points":queries<inst.r,"no_opposite_query_points":opposite_pair_absent,"query_stream":"Q+[seed mod r + j]G, j=0..queries-1"});
    let receipt_sha256 = sha256_hex(serde_json::to_string(&receipt).unwrap().as_bytes());
    let mut counters = BTreeMap::new();
    for k in [
        "queries",
        "hits",
        "rank",
        "rejected_duplicate_columns",
        "pair_table_keys",
        "table_bytes",
        "factor_base_scans",
        "field_muls",
        "field_sqrs",
        "group_doubles",
        "group_mixed",
        "scalar_calls",
    ] {
        counters.insert(format!("native_{k}_uncharged"), u(&raw, k)?);
    }
    counters.insert("independent_replay_wall_ns".into(), replay_ns);
    let (phases, total_gae, unpriced) = account(
        is_v2(&m.id),
        audit_ops,
        prep_ops,
        raw["wall_seconds"].as_f64().map(|x| (x * 1e9) as u64),
        replay_ns,
        &mut counters,
    );
    Ok(SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        total_gae,
        counters,
        unpriced,
        automorphisms_used: 2,
        deterministic: false,
        nondeterminism: vec!["cold wall watchdog".into()],
        solve_wall_ns: start.elapsed().as_nanos() as u64,
        online: Some(online),
        detail: json!({"native":raw,"lambda":lambda,"backend_sha256":m.params["backend_sha256"],"relations":relations,"replay":receipt,"replay_sha256":receipt_sha256}),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::methods::{resolve, MethodSpec};
    use crate::cryptanalysis::ic_boundary::PrimeCurve;

    /// The public D = −19 control of `ic_framework::cm_orbit`'s tests.
    fn control() -> PrimeInstance {
        PrimeInstance {
            name: "public control".into(),
            curve: PrimeCurve {
                p: 343073,
                a: 36086,
                b: 72172,
            },
            group_order: 342833,
            r: 342833,
            cofactor: 1,
            generator: (0, 2882),
        }
    }

    fn method(id: &str, extra: &[(&str, &str)]) -> ResolvedMethod {
        let mut params: BTreeMap<String, String> = [
            ("backend".to_string(), "/nonexistent/folded".to_string()),
            ("backend_sha256".to_string(), "00".to_string()),
            ("query_limit".to_string(), "1000".to_string()),
        ]
        .into_iter()
        .collect();
        params.extend(extra.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        resolve(&MethodSpec {
            id: id.into(),
            params,
        })
        .unwrap()
    }

    fn counters() -> BTreeMap<String, u64> {
        [
            ("native_field_muls_uncharged", 1000u64),
            ("native_hits_uncharged", 0),
            ("native_queries_uncharged", 77),
            ("independent_replay_wall_ns", 5),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }

    const AUDIT: GroupOps = GroupOps {
        adds: 40,
        doubles: 25,
        scalar_mults: 9,
        field_ops: crate::cryptanalysis::ic_boundary::FieldOps {
            muls: 0,
            sqrs: 0,
            invs: 0,
        },
        montgomery_reductions: 0,
    };

    /// The frozen `_v1` report is byte for byte what its committed records
    /// carry: zero gae everywhere, the prose entry, no extra counter.
    #[test]
    fn v1_accounting_is_unchanged() {
        let mut c = counters();
        let (phases, total, unpriced) =
            account(false, AUDIT, GroupOps::default(), Some(11), 22, &mut c);
        assert_eq!(total, 0.0);
        assert!(phases.iter().all(|p| p.gae == 0.0));
        assert_eq!(phases[1].name, "independent_replay");
        assert_eq!(
            (phases[1].adds, phases[1].doubles, phases[1].scalar_mults),
            (40, 25, 9)
        );
        assert_eq!(unpriced.len(), 1);
        assert!(unpriced[0].starts_with("native field/group operations"));
        assert_eq!(c, counters());
    }

    /// `_v2` charges the replay's counted operations, and only them: the
    /// total is the phase sum and `independent_replay.gae` is adds + doubles
    /// (the `[d]G = Q` check included, it is in `audit_ops`).
    #[test]
    fn v2_charges_the_replay_and_names_the_unpriced_counters() {
        let mut c = counters();
        let (v1, t1, _) = account(
            false,
            AUDIT,
            GroupOps::default(),
            Some(11),
            22,
            &mut counters(),
        );
        let (phases, total, unpriced) =
            account(true, AUDIT, GroupOps::default(), Some(11), 22, &mut c);
        assert_eq!(total, 65.0);
        assert!(total > 0.0);
        assert_eq!(total, phases.iter().map(|p| p.gae).sum::<f64>());
        assert_eq!(phases[0].gae, 0.0);
        assert_eq!(phases[1].gae, 65.0);
        // v2 − v1 is exactly the replay's gae; everything else is equal.
        assert_eq!(total - t1, 65.0);
        for (a, b) in v1.iter().zip(&phases) {
            assert_eq!(
                (&a.name, a.adds, a.doubles, a.scalar_mults, a.wall_ns),
                (&b.name, b.adds, b.doubles, b.scalar_mults, b.wall_ns)
            );
        }
        // The prose entry is replaced by the counters themselves, nonzero only.
        assert_eq!(
            unpriced,
            ["native_field_muls_uncharged", "native_queries_uncharged"]
        );
        assert_eq!(c, counters(), "no derivation ops, no extra counter");
        // A Vélu derivation's group operations are named, not charged.
        let prep = GroupOps {
            adds: 3,
            doubles: 4,
            ..GroupOps::default()
        };
        let (_, total_prep, unpriced) = account(true, AUDIT, prep, Some(11), 22, &mut c);
        assert_eq!(total_prep, total);
        assert_eq!(c["cm_map_derivation_group_ops_uncharged"], 7);
        assert!(unpriced.contains(&"cm_map_derivation_group_ops_uncharged".to_string()));
    }

    #[test]
    fn versions_are_told_apart_by_id() {
        assert!(!is_v2("ic.cm_folded_native_v1"));
        assert!(!is_v2("ic.cm_folded_recurrence_v1"));
        assert!(!is_v2("ic.cm_folded_native_d43_v1"));
        assert!(is_v2("ic.cm_folded_native_v2"));
        assert!(is_v2("ic.cm_folded_recurrence_v2"));
        assert!(is_v2("ic.cm_folded_native_d43_v2"));
        assert!(is_recurrence("ic.cm_folded_recurrence_v2"));
        assert!(is_d43("ic.cm_folded_native_d43_v2"));
        assert!(!is_d43("ic.cm_folded_native_v2"));
    }

    /// The plan-time check is what the adapter refuses on every run.
    #[test]
    fn applicable_refuses_what_solve_refuses() {
        let i = control();
        for id in [
            "ic.cm_folded_native_v1",
            "ic.cm_folded_native_v2",
            "ic.cm_folded_recurrence_v2",
        ] {
            applicable(&method(id, &[]), &i).unwrap_or_else(|e| panic!("{id}: {e}"));
        }
        let m = method("ic.cm_folded_native_v2", &[]);
        // Cofactor not one.
        let mut c = i.clone();
        c.cofactor = 2;
        assert!(applicable(&m, &c).unwrap_err().contains("cofactor-one"));
        // Query cap not below r; base not a multiple of four, or too big.
        let r = i.r.to_string();
        for (k, v) in [
            ("query_limit", r.as_str()),
            ("size", "6"),
            ("size", "32768"),
        ] {
            let err = applicable(&method("ic.cm_folded_native_v2", &[(k, v)]), &i).unwrap_err();
            assert!(
                err.contains("unsupported CM base or noninjective"),
                "{k}={v}: {err}"
            );
        }
        // Watchdog and mode.
        assert!(applicable(&method("ic.cm_folded_native_v2", &[("seconds", "0")]), &i).is_err());
        assert!(applicable(&method("ic.cm_folded_native_v2", &[("mode", "x")]), &i).is_err());
        assert!(applicable(&method("ic.cm_folded_native_v2", &[("mode", "velu")]), &i).is_ok());
        assert!(applicable(
            &method("ic.cm_folded_recurrence_v2", &[("mode", "velu")]),
            &i
        )
        .is_err());
        // No CM orbit action: the D = −43 control on a D = −19 curve.
        let err = applicable(&method("ic.cm_folded_native_d43_v2", &[]), &i).unwrap_err();
        assert!(err.contains("no CM orbit action for D = -43"), "{err}");
    }
}
