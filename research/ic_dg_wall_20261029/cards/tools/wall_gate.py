"""Mechanical facts for PROTOCOL.md section 6.1 conditions 1 to 8 (wall session) and the two comparisons of section 6.2.
Reads files; counts and compares only. It decides nothing and exits 0 whatever the values are.
usage: wall_gate.py EKS_DIR ANALYSIS_DIR FROZEN_SPEC WANT_REV WANT_ECBENCH_SHA
Needs ANALYSIS_DIR/pod-uid.txt and the compare files compare-srenew__dg-w{0,0.1,1}.json and compare-srenew__srenew-aa-w{0,0.1,1}.json.
Exits 1 with a line beginning STOP: only when records.jsonl, a compare file or the frozen spec is absent or unreadable."""
import collections, json, os, sys

eks, an, frozen, want_rev, want_fb = sys.argv[1:6]
SESS = os.path.join(eks, "session")
WEIGHTS = ["0", "0.1", "1"]
AA_ARMS = ["srenew", "srenew-aa"]


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def jload(p, required=True):
    if not os.path.isfile(p):
        if required:
            stop("absent: " + p)
        return None
    try:
        with open(p) as f:
            return json.load(f)
    except Exception as e:
        if required:
            stop("cannot read JSON %s: %s" % (p, type(e).__name__))
        return None


def tf(x):
    return "true" if x else "false"


def show(k, v):
    print(k + " " + json.dumps(v, sort_keys=True))


gate = jload(os.path.join(eks, "l2-gate.json"), False) or {}
audit = jload(os.path.join(eks, "audit.json"), False) or {}
jexit = jload(os.path.join(eks, "job-exit.json"), False) or {}
binfo = jload(os.path.join(eks, "build-info.json"), False) or {}
ispec = jload(os.path.join(eks, "input-spec.json"), False)
cc = jload(os.path.join(eks, "control-comparison.json"), False)
fspec = jload(frozen)
uids = [l.strip() for l in open(os.path.join(an, "pod-uid.txt")) if l.strip()]
try:
    recs = [json.loads(l) for l in open(os.path.join(SESS, "records.jsonl")) if l.strip()]
except Exception as e:
    stop("cannot read records.jsonl: " + type(e).__name__)
cmp = {}
for pair in ("srenew__dg", "srenew__srenew-aa"):
    for w in WEIGHTS:
        cmp[(pair, w)] = jload(os.path.join(an, "compare-%s-w%s.json" % (pair, w)))

# ---- facts about the records (pair counts) ----
per_arm = collections.defaultdict(lambda: collections.Counter())
per_arm_curve = collections.defaultdict(lambda: collections.Counter())
levels = collections.Counter()
for r in recs:
    arm = r.get("arm")
    per_arm[arm]["records"] += 1
    if r.get("warmup"):
        per_arm[arm]["warmup"] += 1
    else:
        per_arm[arm]["measured"] += 1
        if (r.get("outcome") or {}).get("status") == "verified":
            per_arm[arm]["measured_verified"] += 1
            per_arm_curve[arm][((r.get("workload") or {}).get("curve") or {}).get("slug")] += 1
    levels[(r.get("isolation") or {}).get("level", "MISSING")] += 1
print("WALL_GATE_FACTS (mechanical facts; the executor does not interpret them)")
show("records.count", len(recs))
show("records.by_arm", {a: dict(c) for a, c in sorted(per_arm.items())})
show("records.measured_verified_by_arm_and_curve", {a: dict(c) for a, c in sorted(per_arm_curve.items())})
show("records.isolation_levels", dict(levels))
show("gate.executions", gate.get("executions"))
show("gate.grades", gate.get("grades"))
show("gate.accepted", gate.get("accepted"))
show("gate.below_l2_len", len(gate.get("below_l2") or []))
show("gate.invalid_outcomes_len", len(gate.get("invalid_outcomes") or []))
exec_1680 = (gate.get("executions") == 1680 and len(recs) == 1680)
per_arm_336 = (sorted(per_arm) == sorted(["walk", "srenew", "dg", "srenew-aa", "dg-small"]) and all(c["records"] == 336 and c["measured"] == 288 and c["measured_verified"] == 288 for c in per_arm.values()))
print("FACT_EXECUTIONS_1680=" + tf(exec_1680))
print("FACT_EACH_OF_5_ARMS_336_RECORDS_288_MEASURED_288_VERIFIED=" + tf(per_arm_336))

# ---- condition 1 ----
ec = jexit.get("exit_code")
single = len(uids) == 1 and jexit.get("pod_uid") == (uids[0].rstrip("/") if uids else None)
c1 = (ec == 0) and single
show("cond1_facts", {"exit_code": ec, "pod_uid_prefix_count": len(uids), "job_exit_pod_uid_equals_prefix": jexit.get("pod_uid") == (uids[0].rstrip("/") if uids else None), "single_pod": single})

# ---- condition 2 ----
rp = audit.get("replays") or []
c2 = (
    audit.get("ok") is True
    and audit.get("session_status") == "complete"
    and len(audit.get("problems") or []) == 0
    and len(rp) > 0
    and all(x.get("detail") == "identical" and x.get("reproduced") is True for x in rp)
)
show("cond2_facts", {"audit_ok": audit.get("ok"), "session_status": audit.get("session_status"), "problems_len": len(audit.get("problems") or []), "replays_total": len(rp), "replays_identical_and_reproduced": sum(1 for x in rp if x.get("detail") == "identical" and x.get("reproduced") is True)})

# ---- condition 3 ----
c3 = True
c3r = True
for w in WEIGHTS:
    d = cmp[("srenew__srenew-aa", w)]
    ops = d.get("ops") or {}
    r = ops.get("ratio_b_over_a")
    row = {
        "file": "compare-srenew__srenew-aa-w%s.json" % w,
        "pricing_weight": d.get("pricing_weight"),
        "a_arm": (d.get("a") or {}).get("arm"),
        "b_arm": (d.get("b") or {}).get("arm"),
        "ops_status": ops.get("status"),
        "ops_pairs": ops.get("pairs"),
        "ops_ratio_b_over_a_repr": repr(r),
        "ops_ci95": ops.get("ci95"),
        "ops_ratio_equals_1_exactly": (r == 1.0),
        "ops_ratio_formats_as_1.000": (isinstance(r, float) and ("%.3f" % r) == "1.000"),
    }
    print("AA " + json.dumps(row, sort_keys=True))
    c3 = c3 and row["ops_ratio_equals_1_exactly"]
    c3r = c3r and row["ops_ratio_formats_as_1.000"]

# ---- condition 4 ----
c4 = (ispec is not None) and (ispec == fspec) and binfo.get("source_revision") == want_rev and binfo.get("ecbench_sha256") == want_fb
show("cond4_facts", {"input_spec_present": ispec is not None, "input_spec_equals_frozen_spec": (ispec is not None) and ispec == fspec, "build_info_source_revision": binfo.get("source_revision"), "build_info_source_revision_equals_want": binfo.get("source_revision") == want_rev, "build_info_ecbench_sha256": binfo.get("ecbench_sha256"), "build_info_ecbench_sha256_equals_want": binfo.get("ecbench_sha256") == want_fb})

# ---- condition 5 ----
grades = gate.get("grades") or []
grade_total = sum(g.get("count", 0) for g in grades)
grades_ok = len(grades) > 0 and all(g.get("level") in ("L2", "L3") for g in grades) and grade_total == gate.get("executions")
records_ok = len(recs) > 0 and all((r.get("isolation") or {}).get("level") in ("L2", "L3") for r in recs)
inv_ok = ("invalid_outcomes" in gate) and len(gate.get("invalid_outcomes") or []) == 0
c5 = (gate.get("accepted") is True) and grades_ok and records_ok and inv_ok
show("cond5_facts", {"accepted": gate.get("accepted"), "grades_all_L2_or_L3_and_sum_equals_executions": grades_ok, "records_all_L2_or_L3": records_ok, "invalid_outcomes_key_present_and_empty": inv_ok})

# ---- condition 6 ----
ccw = (cc or {}).get("wall") or {}
c6 = (
    cc is not None
    and ccw.get("status") == "admitted"
    and ccw.get("pairs_below_level") == 0
    and isinstance(ccw.get("aa_pairs"), int) and ccw.get("aa_pairs") >= 5
    and ccw.get("aa_ci95") is not None
    and ccw.get("aa_arms") == AA_ARMS
)
show("cond6_facts", {"control_comparison_present": cc is not None, "a_arm": ((cc or {}).get("a") or {}).get("arm"), "b_arm": ((cc or {}).get("b") or {}).get("arm"), "wall.status": ccw.get("status"), "wall.pairs": ccw.get("pairs"), "wall.pairs_below_level": ccw.get("pairs_below_level"), "wall.aa_pairs": ccw.get("aa_pairs"), "wall.aa_ci95_is_null": ccw.get("aa_ci95") is None, "wall.aa_arms": ccw.get("aa_arms"), "wall.self_aa": ccw.get("self_aa"), "wall.aa_selection": ccw.get("aa_selection")})

# ---- condition 7 ----
d0 = cmp[("srenew__dg", "0")]
w0 = d0.get("wall") or {}
ident = {w: (cmp[("srenew__dg", w)].get("wall") == d0.get("wall")) for w in ("0.1", "1")}
c7 = (
    w0.get("status") == "admitted"
    and w0.get("pairs") == 288
    and w0.get("pairs_below_level") == 0
    and w0.get("self_aa") is False
    and w0.get("aa_arms") == AA_ARMS
    and w0.get("aa_selection") == "includes arm A"
    and w0.get("aa_pairs") == 288
    and w0.get("corrected_ci95") is not None
    and ident["0.1"] and ident["1"]
)
show("cond7_facts", {"wall.status": w0.get("status"), "wall.pairs": w0.get("pairs"), "wall.pairs_below_level": w0.get("pairs_below_level"), "wall.self_aa": w0.get("self_aa"), "wall.aa_arms": w0.get("aa_arms"), "wall.aa_selection": w0.get("aa_selection"), "wall.aa_pairs": w0.get("aa_pairs"), "wall.corrected_ci95_is_null": w0.get("corrected_ci95") is None, "wall_block_identical_at_w0.1": ident["0.1"], "wall_block_identical_at_w1": ident["1"]})

# ---- condition 8 ----
o0 = d0.get("ops") or {}
r0 = o0.get("ratio_b_over_a")
c8a = isinstance(r0, float) and abs(r0 - 1.0) < 0.0005
ub = {}
c8b = True
for w in ("0.1", "1"):
    ci = (cmp[("srenew__dg", w)].get("ops") or {}).get("ci95")
    ub[w] = ci[1] if ci else None
    c8b = c8b and (ci is not None) and ci[1] < 1
c8 = c8a and c8b
show("cond8_facts", {"ops.ratio_b_over_a_w0_repr": repr(r0), "abs_ratio_minus_1_lt_0.0005": c8a, "ops.ci95_upper_w0.1_repr": repr(ub["0.1"]), "ops.ci95_upper_w1_repr": repr(ub["1"]), "ops.ci95_upper_lt_1_at_both": c8b})

# ---- section 6.2 comparisons on the decision block ----
cci = w0.get("corrected_ci95")
upper_lt1 = bool(cci) and cci[1] < 1
lower_gt1 = bool(cci) and cci[0] > 1
show("decision_block_w0", {k: w0.get(k) for k in ["ns_per_gae_a", "ns_per_gae_b", "ratio_b_over_a", "ci95", "ci_method", "aa_ratio", "corrected_ratio", "corrected_ci95", "corrected_dof", "outside_noise", "total_wall_ratio", "median_ratio"]})
show("decision_resample_seed", d0.get("resample_seed"))

lines = {
    "W_COND1": c1, "W_COND2": c2, "W_COND3": c3, "W_COND3_ROUNDED3": c3r, "W_COND4": c4, "W_COND5": c5, "W_COND6": c6, "W_COND7": c7, "W_COND8": c8,
}
all8 = c1 and c2 and c3 and c4 and c5 and c6 and c7 and c8
all8r = c1 and c2 and c3r and c4 and c5 and c6 and c7 and c8
lines["W_ALL_1_TO_8"] = all8
lines["W_ALL_1_TO_8_ROUNDED3"] = all8r
lines["CORRECTED_CI95_UPPER_LT_1"] = upper_lt1
lines["CORRECTED_CI95_LOWER_GT_1"] = lower_gt1
lines["FACT_EXECUTIONS_1680"] = exec_1680
lines["FACT_EACH_OF_5_ARMS_336_RECORDS_288_MEASURED_288_VERIFIED"] = per_arm_336
with open(os.path.join(an, "wall_gate.json"), "w") as f:
    json.dump({"lines": lines}, f, indent=1, sort_keys=True)
for k in ["W_COND1", "W_COND2", "W_COND3", "W_COND3_ROUNDED3", "W_COND4", "W_COND5", "W_COND6", "W_COND7", "W_COND8", "W_ALL_1_TO_8", "W_ALL_1_TO_8_ROUNDED3", "CORRECTED_CI95_UPPER_LT_1", "CORRECTED_CI95_LOWER_GT_1"]:
    print(k + "=" + tf(lines[k]))
print("WALL_GATE_OK")
