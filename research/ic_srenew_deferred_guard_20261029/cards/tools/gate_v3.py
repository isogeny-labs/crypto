"""Mechanical facts for PROTOCOL.md (v3) section 6.1. Reads files; counts and filters only.

usage: gate_v3.py EKS_DIR ANALYSIS_DIR FROZEN_SPEC BASE WANT_REV WANT_ECBENCH_SHA
Prints facts and the lines GATE_COND1..6 and EXIT42_CLAUSE (true/false by the literal rule
of section 6.1), and writes ANALYSIS_DIR/gate_v3.json. Exits 0 whatever the values are.
It exits 1 with a line beginning STOP: only when an input file is absent or unreadable.
"""
import collections, json, os, re, sys

eks, an, frozen, base, want_rev, want_fb = sys.argv[1:7]
SESS = os.path.join(eks, "session")
WEIGHTS = ["0", "0.1", "1"]
RHO = ["rho-neg", "rho-neg-aa"]
PHRASE = "L2: solve too short to admit wall time"
TIME_RE = re.compile(r"^L2: solve too short to admit wall time: solve ([0-9.]+) (\S+) is under")


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def jload(p):
    try:
        with open(p) as f:
            return json.load(f)
    except Exception as e:
        stop("cannot read JSON %s: %s" % (p, type(e).__name__))


def b(x):
    return "true" if x else "false"


gate = jload(os.path.join(eks, "l2-gate.json"))
audit = jload(os.path.join(eks, "audit.json"))
jexit = jload(os.path.join(eks, "job-exit.json"))
binfo = jload(os.path.join(eks, "build-info.json"))
ispec = jload(os.path.join(eks, "input-spec.json"))
fspec = jload(frozen)
uids = [l.strip() for l in open(os.path.join(an, "pod-uid.txt")) if l.strip()]
try:
    recs = [json.loads(l) for l in open(os.path.join(SESS, "records.jsonl")) if l.strip()]
except Exception as e:
    stop("cannot read records.jsonl: " + type(e).__name__)

# arm classes, read from the frozen spec
spec_arms = fspec["arms"]
IC = [a["name"] for a in spec_arms if a["method"]["id"] == "ic.pipeline"]
RHO_SPEC = [a["name"] for a in spec_arms if a["method"]["id"] == "rho.negation"]
out = {"ic_arms": IC, "rho_arms_in_spec": RHO_SPEC, "rho_arms_rule": RHO}


def classify(level, blockers):
    """For a rho-arm execution: returns 'l2_or_l3', 'waived', 'not_waived' or 'below_l1'."""
    if level in ("L2", "L3"):
        return "l2_or_l3"
    if level != "L1":
        return "below_l1"
    l2b = [x for x in blockers if x.startswith("L2: ")]
    if all(x.startswith(PHRASE) for x in l2b):
        return "waived"
    return "not_waived"


def times(blockers):
    """Solve times printed in the 'solve too short' blocker: list of (number string, unit)."""
    r = []
    for x in blockers:
        m = TIME_RE.match(x)
        if m:
            r.append((m.group(1), m.group(2)))
    return r


# ---------------- records.jsonl ----------------
seqs = [r.get("seq") for r in recs]
out["records"] = {
    "count": len(recs),
    "distinct_seq": len(set(seqs)),
    "status_counts": dict(collections.Counter(str((r.get("outcome") or {}).get("status")) for r in recs)),
}
by_arm_level = collections.defaultdict(collections.Counter)
rho_class = collections.defaultdict(collections.Counter)
waived_us = []
waived_wall_ns = []
waived_units = collections.Counter()
waived_zero_l2_blockers = 0
waived_without_phrase_time = 0
not_waived = []
below_l1 = []
below_ic = []
rec_below = {}
other_arm = 0
for r in recs:
    arm = r.get("arm")
    iso = r.get("isolation") or {}
    level = iso.get("level", "MISSING")
    bl = iso.get("blockers") or []
    by_arm_level[arm][level] += 1
    if level not in ("L2", "L3"):
        rec_below[r.get("seq")] = (arm, iso)
    if arm in IC:
        if level not in ("L2", "L3"):
            below_ic.append([r.get("seq"), arm, level])
    elif arm in RHO:
        c = classify(level, bl)
        rho_class[arm][c] += 1
        if c == "waived":
            tl = times(bl)
            if not [x for x in bl if x.startswith("L2: ")]:
                waived_zero_l2_blockers += 1
            if not tl:
                waived_without_phrase_time += 1
            for num, unit in tl:
                waived_units[unit] += 1
                if unit == "µs":
                    waived_us.append(float(num))
            w = (r.get("online") or {}).get("wall_ns")
            if isinstance(w, int):
                waived_wall_ns.append(w)
        elif c == "not_waived":
            not_waived.append([r.get("seq"), arm, level])
        elif c == "below_l1":
            below_l1.append([r.get("seq"), arm, level])
    else:
        other_arm += 1
out["records"]["executions_by_arm_and_level"] = {a: dict(c) for a, c in sorted(by_arm_level.items())}
out["records"]["rho_execution_class_by_arm"] = {a: dict(c) for a, c in sorted(rho_class.items())}
out["records"]["records_of_arms_in_neither_class"] = other_arm
out["records"]["ic_executions_below_l2"] = len(below_ic)
out["records"]["ic_executions_below_l2_first5"] = below_ic[:5]
out["records"]["rho_waived_count"] = sum(c.get("waived", 0) for c in rho_class.values())
out["records"]["rho_waived_with_zero_l2_prefixed_blockers"] = waived_zero_l2_blockers
out["records"]["rho_waived_without_parsable_solve_time"] = waived_without_phrase_time
out["records"]["rho_not_waived_count"] = len(not_waived)
out["records"]["rho_not_waived_seq_arm_level"] = not_waived
out["records"]["rho_below_l1_count"] = len(below_l1)
out["records"]["rho_below_l1_seq_arm_level"] = below_l1
out["records"]["waived_solve_time_units_seen"] = dict(waived_units)
out["records"]["waived_solve_time_from_blocker_us_min"] = min(waived_us) if waived_us else None
out["records"]["waived_solve_time_from_blocker_us_max"] = max(waived_us) if waived_us else None
out["records"]["waived_online_wall_ns_min"] = min(waived_wall_ns) if waived_wall_ns else None
out["records"]["waived_online_wall_ns_max"] = max(waived_wall_ns) if waived_wall_ns else None
nw = [x for x in not_waived]
first_nw = None
if nw:
    sq = nw[0][0]
    for r in recs:
        if r.get("seq") == sq:
            first_nw = (r.get("isolation") or {}).get("blockers")
            break
out["records"]["first_not_waived_blockers"] = first_nw
out["records"]["not_waived_l2_blocker_texts"] = dict(collections.Counter(
    re.sub(r"[0-9.]+", "#", x)
    for r in recs if r.get("arm") in RHO and classify((r.get("isolation") or {}).get("level", "MISSING"), (r.get("isolation") or {}).get("blockers") or []) == "not_waived"
    for x in ((r.get("isolation") or {}).get("blockers") or []) if x.startswith("L2: ") and not x.startswith(PHRASE)
))
rec_c4 = len(below_ic) == 0
rec_c5 = len(not_waived) == 0 and len(below_l1) == 0 and other_arm == 0
rec_below_all_rho_waived = all(
    a in RHO and classify(i.get("level", "MISSING"), i.get("blockers") or []) == "waived"
    for (a, i) in rec_below.values()
)

# ---------------- l2-gate.json ----------------
gb = gate.get("below_l2") or []
g_by_arm_level = collections.defaultdict(collections.Counter)
g_rho_class = collections.defaultdict(collections.Counter)
g_waived_us = []
g_below_ic = []
g_not_waived = []
g_below_l1 = []
g_other = 0
for e in gb:
    arm = e.get("arm")
    iso = e.get("isolation") or {}
    level = iso.get("level", "MISSING")
    bl = iso.get("blockers") or []
    g_by_arm_level[arm][level] += 1
    if arm in IC:
        g_below_ic.append([e.get("seq"), arm, level])
    elif arm in RHO:
        c = classify(level, bl)
        g_rho_class[arm][c] += 1
        if c == "waived":
            for num, unit in times(bl):
                if unit == "µs":
                    g_waived_us.append(float(num))
        elif c == "not_waived":
            g_not_waived.append([e.get("seq"), arm, level])
        elif c == "below_l1":
            g_below_l1.append([e.get("seq"), arm, level])
    else:
        g_other += 1
out["gate"] = {
    "required": gate.get("required"),
    "accepted": gate.get("accepted"),
    "executions": gate.get("executions"),
    "grades": gate.get("grades"),
    "below_l2_len": len(gb),
    "invalid_outcomes_len": len(gate.get("invalid_outcomes") or []),
    "invalid_outcomes_key_present": "invalid_outcomes" in gate,
    "below_l2_by_arm_and_level": {a: dict(c) for a, c in sorted(g_by_arm_level.items())},
    "per_arm_counts_at_L2_or_above_available_in_gate": False,
    "rho_below_l2_class_by_arm": {a: dict(c) for a, c in sorted(g_rho_class.items())},
    "ic_entries_in_below_l2": len(g_below_ic),
    "ic_entries_in_below_l2_first5": g_below_ic[:5],
    "rho_waived_count": sum(c.get("waived", 0) for c in g_rho_class.values()),
    "rho_not_waived_count": len(g_not_waived),
    "rho_below_l1_count": len(g_below_l1),
    "entries_of_arms_in_neither_class": g_other,
    "waived_solve_time_from_blocker_us_min": min(g_waived_us) if g_waived_us else None,
    "waived_solve_time_from_blocker_us_max": max(g_waived_us) if g_waived_us else None,
}
g_c4 = len(g_below_ic) == 0
g_c5 = len(g_not_waived) == 0 and len(g_below_l1) == 0 and g_other == 0
g_below_all_rho_waived = all(
    e.get("arm") in RHO and classify((e.get("isolation") or {}).get("level", "MISSING"), (e.get("isolation") or {}).get("blockers") or []) == "waived"
    for e in gb
)

# ---------------- agreement of the two sources ----------------
gate_total = sum(g.get("count", 0) for g in (gate.get("grades") or []))
rec_levels = collections.Counter((r.get("isolation") or {}).get("level", "MISSING") for r in recs)
gate_levels = {g.get("level"): g.get("count") for g in (gate.get("grades") or [])}
gb_by_seq = {e.get("seq"): (e.get("arm"), e.get("isolation")) for e in gb}
agree_exec = gate.get("executions") == len(recs) == gate_total
agree_levels = dict(rec_levels) == gate_levels
agree_seqs = set(gb_by_seq) == set(rec_below)
agree_entries = agree_seqs and all(gb_by_seq[s] == rec_below[s] for s in gb_by_seq)
agree = agree_exec and agree_levels and agree_seqs and agree_entries and len(set(seqs)) == len(recs)
out["agreement"] = {
    "executions_equal": agree_exec,
    "grade_counts_equal": agree_levels,
    "below_l2_seq_sets_equal": agree_seqs,
    "below_l2_entries_equal_arm_and_isolation": agree_entries,
    "record_seq_all_distinct": len(set(seqs)) == len(recs),
    "gate_and_records_agree": agree,
}

# ---------------- conditions 1 to 3 ----------------
rp = audit.get("replays") or []
c1 = (
    audit.get("ok") is True
    and audit.get("session_status") == "complete"
    and len(audit.get("problems") or []) == 0
    and len(rp) > 0
    and all(x.get("detail") == "identical" and x.get("reproduced") is True for x in rp)
)
out["cond1_facts"] = {
    "audit_ok": audit.get("ok"),
    "session_status": audit.get("session_status"),
    "problems_len": len(audit.get("problems") or []),
    "replays_total": len(rp),
    "replays_identical_and_reproduced": sum(1 for x in rp if x.get("detail") == "identical" and x.get("reproduced") is True),
}
aa = []
c2 = True
c2r = True
pairs = [(base, base + "-aa"), ("rho-neg", "rho-neg-aa")]
for a, bb in pairs:
    for w in WEIGHTS:
        p = os.path.join(an, "compare-%s__%s-w%s.json" % (a, bb, w))
        cj = jload(p)
        ops = cj.get("ops") or {}
        r = ops.get("ratio_b_over_a")
        row = {
            "file": os.path.basename(p),
            "pricing_weight": cj.get("pricing_weight"),
            "a_arm": (cj.get("a") or {}).get("arm"),
            "b_arm": (cj.get("b") or {}).get("arm"),
            "ops_status": ops.get("status"),
            "ops_pairs": ops.get("pairs"),
            "ops_ratio_b_over_a_repr": repr(r),
            "ops_ci95": ops.get("ci95"),
            "ops_ratio_equals_1_exactly": (r == 1.0),
            "ops_ratio_formats_as_1.000": (isinstance(r, float) and ("%.3f" % r) == "1.000"),
        }
        aa.append(row)
        c2 = c2 and row["ops_ratio_equals_1_exactly"]
        c2r = c2r and row["ops_ratio_formats_as_1.000"]
out["aa_ratios"] = aa
c3 = (
    ispec == fspec
    and binfo.get("source_revision") == want_rev
    and binfo.get("ecbench_sha256") == want_fb
)
out["cond3_facts"] = {
    "input_spec_equals_frozen_spec": ispec == fspec,
    "build_info_source_revision": binfo.get("source_revision"),
    "build_info_source_revision_equals_want": binfo.get("source_revision") == want_rev,
    "build_info_ecbench_sha256": binfo.get("ecbench_sha256"),
    "build_info_ecbench_sha256_equals_want": binfo.get("ecbench_sha256") == want_fb,
}

# ---------------- conditions 4 to 6, exit 42 ----------------
c4 = rec_c4 and g_c4 and agree
c5 = rec_c5 and g_c5 and agree
c6 = out["gate"]["invalid_outcomes_len"] == 0 and out["gate"]["invalid_outcomes_key_present"]
ec = jexit.get("exit_code")
single = len(uids) == 1 and jexit.get("pod_uid") == uids[0].rstrip("/")
out["job_exit"] = {
    "exit_code": ec,
    "exit_code_is_0_or_42": ec in (0, 42),
    "pod_uid_prefix_count": len(uids),
    "job_exit_pod_uid_equals_prefix": jexit.get("pod_uid") == (uids[0].rstrip("/") if uids else None),
    "single_pod": single,
}
below_ok = rec_below_all_rho_waived and g_below_all_rho_waived
e42 = (ec == 42) and single and c1 and c2 and c3 and c4 and c5 and c6 and below_ok
out["exit42_parts"] = {
    "exit_code_is_42": ec == 42,
    "single_pod": single,
    "cond1": c1, "cond2": c2, "cond3": c3, "cond4": c4, "cond5": c5, "cond6": c6,
    "every_below_l2_entry_is_a_waived_rho_execution_records": rec_below_all_rho_waived,
    "every_below_l2_entry_is_a_waived_rho_execution_gate": g_below_all_rho_waived,
    "below_l2_nonempty": len(gb) > 0,
}
out["lines"] = {
    "GATE_COND1": c1, "GATE_COND2": c2, "GATE_COND2_ROUNDED3": c2r, "GATE_COND3": c3,
    "GATE_COND4": c4, "GATE_COND5": c5, "GATE_COND6": c6, "EXIT42_CLAUSE": e42,
}
with open(os.path.join(an, "gate_v3.json"), "w") as f:
    json.dump(out, f, indent=1, sort_keys=True)

# ---------------- printed projection (small) ----------------
def show(k, v):
    print(k + " " + json.dumps(v, sort_keys=True))

print("GATE_V3_FACTS (mechanical facts; the executor does not interpret them)")
show("ic_arms", IC)
show("records.count", out["records"]["count"])
show("records.executions_by_arm_and_level", out["records"]["executions_by_arm_and_level"])
show("gate.executions", out["gate"]["executions"])
show("gate.grades", out["gate"]["grades"])
show("gate.accepted", out["gate"]["accepted"])
show("gate.below_l2_len", out["gate"]["below_l2_len"])
show("gate.below_l2_by_arm_and_level", out["gate"]["below_l2_by_arm_and_level"])
show("records.rho_execution_class_by_arm", out["records"]["rho_execution_class_by_arm"])
show("gate.rho_below_l2_class_by_arm", out["gate"]["rho_below_l2_class_by_arm"])
show("records.rho_waived_count", out["records"]["rho_waived_count"])
show("records.rho_waived_with_zero_l2_prefixed_blockers", out["records"]["rho_waived_with_zero_l2_prefixed_blockers"])
show("records.rho_not_waived_count", out["records"]["rho_not_waived_count"])
show("records.rho_below_l1_count", out["records"]["rho_below_l1_count"])
show("records.not_waived_l2_blocker_texts", out["records"]["not_waived_l2_blocker_texts"])
show("records.waived_solve_time_units_seen", out["records"]["waived_solve_time_units_seen"])
show("records.waived_solve_time_from_blocker_us_min", out["records"]["waived_solve_time_from_blocker_us_min"])
show("records.waived_solve_time_from_blocker_us_max", out["records"]["waived_solve_time_from_blocker_us_max"])
show("gate.waived_solve_time_from_blocker_us_min", out["gate"]["waived_solve_time_from_blocker_us_min"])
show("gate.waived_solve_time_from_blocker_us_max", out["gate"]["waived_solve_time_from_blocker_us_max"])
show("records.waived_online_wall_ns_min", out["records"]["waived_online_wall_ns_min"])
show("records.waived_online_wall_ns_max", out["records"]["waived_online_wall_ns_max"])
show("records.ic_executions_below_l2", out["records"]["ic_executions_below_l2"])
show("gate.ic_entries_in_below_l2", out["gate"]["ic_entries_in_below_l2"])
show("gate.invalid_outcomes_len", out["gate"]["invalid_outcomes_len"])
show("records.status_counts", out["records"]["status_counts"])
show("job_exit", out["job_exit"])
show("agreement", out["agreement"])
show("cond1_facts", out["cond1_facts"])
show("cond3_facts", out["cond3_facts"])
for row in aa:
    print("AA " + json.dumps(row, sort_keys=True))
show("exit42_parts", out["exit42_parts"])
for k in ["GATE_COND1", "GATE_COND2", "GATE_COND2_ROUNDED3", "GATE_COND3", "GATE_COND4", "GATE_COND5", "GATE_COND6", "EXIT42_CLAUSE"]:
    print(k + "=" + b(out["lines"][k]))
print("GATE_V3_OK")
