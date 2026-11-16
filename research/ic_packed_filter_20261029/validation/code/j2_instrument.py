"""J2 (TASK-20261029-654f73): instrument checks, own code.

Wall session (PROTOCOL 6.1 conditions 1-7, read after the SHA256SUMS check):
exit, audit, counted A/A, spec and build, L2 gate (re-counted and re-graded
from each record's isolation block), native A/A admission file, decision
block facts recomputed from records, block positions and predecessors.
Counted session (XP-20261029-srenew-deferred-guard-v3 6.1 conditions 1-6 as
adapted by XP-20261029-dg-wall 6.1, and the exit-42 clause), read through
vcommon.load_counted: no wall field, no rho wall time.

usage: j2_instrument.py OUT.json
"""
import collections
import json
import math
import os
import sys

import vcommon as v

REV = "3271a25d02dcef6da148293fb8f9e2149ce59858"
ECB = "21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503"
BACKLOG = os.path.join(v.REPO, "backlog", "XP-20261029-packed-filter")
CURVES = {24: {"icv1-fp24-t217-e1fbffd2", "icv1-fp24-tm567-4accd0e8"},
          28: {"icv1-fp28-tm295-85243301", "icv1-fp28-tm8273-f1778659"},
          32: {"icv1-fp32-t89141-95879fe3", "icv1-fp32-tm68303-f902d2f0"}}
JOB = {("wall", 24): "ic-dgbf-p24-20261029", ("wall", 32): "ic-dgbf-p32-20261029",
       ("wall", 28): "ic-dgbf-p28-20261029",
       ("counted", 24): "ic-dgbf-rho-p24-20261029", ("counted", 32): "ic-dgbf-rho-p32-20261029",
       ("counted", 28): "ic-dgbf-rho-p28-20261029-r2"}
WEIGHTS = (0.0, 0.1, 1.0)
ORDER = {0: "L0", 1: "L1", 2: "L2", 3: "L3"}


def jload(p):
    with open(p) as f:
        return json.load(f)


def level_from_blockers(bl):
    if any(b.startswith("L1: ") for b in bl):
        return "L0"
    if any(b.startswith("L2: ") for b in bl):
        return "L1"
    if any(b.startswith("L3: ") for b in bl):
        return "L2"
    return "L3"


def common_file_checks(b, kind, records):
    eks = v.session_dir(b, kind)
    out = {}
    ok, n, bad = v.check_sums(eks)
    out["sha256sums"] = {"ok": ok, "files": n, "bad": bad}
    je = jload(os.path.join(eks, "job-exit.json"))
    out["job_exit"] = {"exit_code": je["exit_code"], "job_name": je["job_name"],
                       "job_name_expected": je["job_name"] == JOB[(kind, b)],
                       "single_pod_uid": isinstance(je.get("pod_uid"), str) and len(je["pod_uid"]) == 36}
    spec_name = f"spec-P{b}.json" if kind == "wall" else f"spec-P{b}-rho.json"
    frozen = jload(os.path.join(BACKLOG, spec_name))
    inspec = jload(os.path.join(eks, "input-spec.json"))
    sspec = jload(os.path.join(eks, "session", "spec.json"))
    bi = jload(os.path.join(eks, "build-info.json"))
    binsha = open(os.path.join(eks, "binary.sha256")).read().split()[0]
    sess = jload(os.path.join(eks, "session", "session.json"))
    out["spec_build"] = {
        "frozen_spec": spec_name, "frozen_spec_sha256": v.sha256_file(os.path.join(BACKLOG, spec_name)),
        "input_spec_json_equal_frozen": inspec == frozen,
        "session_spec_json_equal_frozen": sspec == frozen,
        "build_info_revision": bi["source_revision"] == REV,
        "build_info_ecbench_sha256": bi["ecbench_sha256"] == ECB,
        "binary_sha256_file": binsha == ECB,
        "session_binary_sha256": sess["binary_sha256"] == ECB,
        "records_binary_sha256_all": all(r["binary_sha256"] == ECB for r in records),
        "records_session_id_all": all(r["session_id"] == sess["session_id"] for r in records),
        "session_status": sess["status"], "grading_version": sess.get("grading_version"),
        "executions_planned": sess["executions_planned"], "records_written": sess["records_written"],
        "records_in_file": len(records),
        "session_records_sha256_matches_file": sess["records_sha256"] == v.sha256_file(os.path.join(eks, "session", "records.jsonl")),
        "session_plan_sha256_matches_file": sess["plan_sha256"] == v.sha256_file(os.path.join(eks, "session", "plan.json")),
        "curves_are_section3": {r["workload"]["curve"]["slug"] for r in records} == CURVES[b],
        "curve_seeds": sorted({r["workload"]["curve_spec"]["seed"] for r in records}) == [int(f"202610296{b}3"), int(f"202610296{b}4")],
        "target_seed": {r["workload"]["target_seed"] for r in records} == {int(f"202610296{b}6")},
        "order_seed_in_spec": frozen["measurement"]["seed"] == int(f"202610296{b}8"),
        "spec_rounds_warmup_order_level": [frozen["measurement"][k] for k in ("rounds", "warmup", "order", "isolation_required", "timeout_seconds")],
    }
    # algorithm seed equal across arms of each (curve, target, round)
    seeds = collections.defaultdict(set)
    for r in records:
        seeds[v.key(r)].add(r["algorithm_seed"])
    out["spec_build"]["algorithm_seed_shared_by_all_arms_per_key"] = all(len(s) == 1 for s in seeds.values())
    if os.path.exists(os.path.join(eks, "audit.json")):
        a = jload(os.path.join(eks, "audit.json"))
        sums = {l.split(None, 1)[1].strip(): l.split()[0] for l in open(os.path.join(eks, "SHA256SUMS")) if l.strip()}
        measured = sum(1 for r in records if not r["warmup"])
        out["audit"] = {
            "ok": a["ok"], "session_status": a["session_status"], "problems": len(a["problems"]),
            "records": a["records"], "verified_records": a["verified_records"],
            "replay_selection": a["replay_selection"], "replay_candidates": a["replay_candidates"],
            "measured_records": measured,
            "replays": len(a["replays"]),
            "replays_identical": sum(1 for x in a["replays"] if x["reproduced"] and x["detail"] == "identical"),
            "replay_seqs_are_all_measured": sorted(x["seq"] for x in a["replays"]) == sorted(r["seq"] for r in records if not r["warmup"]),
            "regraded": a.get("regraded"),
            "auditor_binary": a["auditor_binary_sha256"] == ECB,
            "audit_file_hashes_match_sums": all(sums.get("session/" + k) == h for k, h in a["files"].items()),
            "audit_stdout_equals_audit_json": sums.get("audit.json") == sums.get("audit.stdout"),
        }
    else:
        out["audit"] = None
    return out


def regrade_counts(records):
    lv = collections.Counter(r["isolation"]["level"] for r in records)
    mism = [r["record_id"] for r in records if level_from_blockers(r["isolation"]["blockers"]) != r["isolation"]["level"]]
    prefixes = collections.Counter()
    for r in records:
        for bl in r["isolation"]["blockers"]:
            prefixes[bl.split(": ")[0] + ": " + (bl.split(": ", 1)[1][:40] if bl.startswith("L3") else "<L2 or L1 blocker>")] += 1
    return dict(lv), mism, dict(prefixes)


def raw_regrade_wall(b, records):
    """Partial independent re-grade of the grading-v3 L1/L2 gates from each
    wall record's raw isolation and timing fields (wall session, IC arms only)."""
    sess = jload(os.path.join(v.session_dir(b, "wall"), "session", "session.json"))
    hz = sess["ticks_per_second"]
    th = sess["options"]["thresholds"]
    observed = sess["cpu_plan"]["observed"]
    flagged = collections.Counter()
    bad = []
    for r in records:
        iso, t = r["isolation"], r["time"]
        run = iso["run_cpu"]
        o = t.get("solve") or {}
        wall_s = t["process_wall_ns"] / 1e9
        cpu_s = (t["user_ns"] + t["sys_ns"]) / 1e9
        solve_ns = t["solve_wall_ns"]
        s = solve_ns / 1e9
        f = []
        # L1 gates
        if iso["placement"]["affinity"] != [run]: f.append("L1 affinity")
        if iso["placement"]["cpu_at_start"] != run or iso["placement"]["cpu_at_end"] != run: f.append("L1 cpu moved")
        if max(o.get("threads_before") or 1, o.get("threads_after") or 1) > 1: f.append("L1 threads")
        if wall_s > 0 and cpu_s / wall_s > 1.05: f.append("L1 cpu/wall")
        it = [x["intruders"]["user_threads"] for x in (iso["before"], iso["after"]) if x.get("intruders")]
        if not it or max(it) > sess["eviction"]["failed_user_threads"]: f.append("L1 intruders")
        # L2 gates
        if solve_ns < 1_000_000: f.append("L2 too short")
        a_, b_ = iso["before"]["reserved_ticks"].get(str(run)), iso["after"]["reserved_ticks"].get(str(run))
        busy = b_["busy"] - a_["busy"]
        foreign = round(busy - cpu_s * hz)
        if foreign != iso["foreign_busy_ticks"]: f.append("foreign ticks differ from record")
        if foreign > max(2.0, 0.01 * wall_s * hz): f.append("L2 foreign ticks")
        st = b_["steal"] - a_["steal"]
        if st > max(1.0, 0.01 * wall_s * hz): f.append("L2 steal")
        sb = 0
        for c in observed:
            if c != run:
                x0, x1 = iso["before"]["reserved_ticks"].get(str(c)), iso["after"]["reserved_ticks"].get(str(c))
                if x0 and x1:
                    sb += x1["busy"] - x0["busy"]
        if sb > max(2.0, 0.01 * wall_s * hz): f.append("L2 sibling busy")
        sch = o.get("schedstat_all") or t.get("schedstat_solve")
        if sch is None or sch["run_delay_ns"] > max(100_000, 0.005 * solve_ns): f.append("L2 run-queue")
        inv = o.get("involuntary_switches")
        if inv is not None and inv > max(math.ceil(th["max_preemptions_per_second"] * s), 1.0): f.append("L2 preemption")
        irq = o.get("irq")
        if irq is not None:
            nontick = irq.get("device", 0) + irq.get("ipi_other", 0) + irq.get("softirq_work", 0)
            if nontick > max(math.ceil(100.0 * s), 2): f.append("L2 irq (approx)")
        cs = o.get("cpu_sched")
        if cs is not None:
            own = (o.get("schedstat_all") or {}).get("timeslices", 0)
            fa = max(cs["pcount"] - own, 0)
            fns = cs["rq_cpu_ns"] if (o.get("involuntary_switches", 0) or 0) + (o.get("voluntary_switches", 0) or 0) == 0 else None
            long = fns is not None and fns > max(100_000, 0.01 * s * 1e9)
            if fa > 1 or long: f.append("L2 foreign slices")
        pa, pb = iso["before"]["psi_memory_some_total_us"], iso["after"]["psi_memory_some_total_us"]
        if (pb - pa) / 1e6 > max(0.005 * max(wall_s, 0.001), 0.001): f.append("L2 memory stall")
        for x in f:
            flagged[x] += 1
        rec_l2 = [x for x in iso["blockers"] if x.startswith("L2: ") or x.startswith("L1: ")]
        mine_l2 = [x for x in f if x.startswith("L2") or x.startswith("L1")]
        if bool(rec_l2) != bool(mine_l2) or "foreign ticks differ from record" in f:
            bad.append({"record_id": r["record_id"], "seq": r["seq"], "mine": f, "recorded": rec_l2})
    return {"ticks_per_second": hz, "thresholds": th, "records": len(records),
            "flag_counts": dict(flagged), "disagreements": bad,
            "note": "partial re-grade: L1/L2 gates that depend only on record fields; preflight, capsule and eviction gates are session-level and read from session.json"}


def positions(records):
    blocks = collections.defaultdict(list)
    for r in records:
        blocks[v.key(r)].append(r)
    pos = collections.defaultdict(collections.Counter)
    pred = collections.defaultdict(collections.Counter)
    orders = collections.Counter()
    consecutive = True
    for k, bl in blocks.items():
        bl.sort(key=lambda r: r["seq"])
        seqs = [r["seq"] for r in bl]
        consecutive &= seqs == list(range(seqs[0], seqs[0] + len(seqs)))
        if bl[0]["warmup"]:
            continue
        orders[tuple(r["arm"] for r in bl)] += 1
        for i, r in enumerate(bl):
            pos[r["arm"]][i] += 1
            pred[r["arm"]][bl[i - 1]["arm"] if i else "(block start)"] += 1
    return {"blocks": len(blocks), "block_sizes": dict(collections.Counter(len(b) for b in blocks.values())),
            "blocks_consecutive_seq": consecutive,
            "measured_orders": {" ".join(k): n for k, n in orders.items()},
            "positions": {a: {str(i): n for i, n in sorted(c.items())} for a, c in pos.items()},
            "predecessor_within_block": {a: dict(c) for a, c in pred.items()}}


def counted_aa(records, a, b):
    """ops A/A: per pair, priced GAE equal at each weight; ratio of sums of priced S."""
    arms = v.by_arm(records)
    A, B = arms[a], arms[b]
    out = {"pairs": len(set(A) & set(B)), "unpaired": len(set(A) ^ set(B))}
    for w in WEIGHTS:
        eq = all(v.priced_gae(A[k], w) == v.priced_gae(B[k], w) for k in A)
        sa = sum(v.priced_gae(A[k], w) / math.sqrt(A[k]["workload"]["curve"]["r"]) for k in A)
        sb = sum(v.priced_gae(B[k], w) / math.sqrt(B[k]["workload"]["curve"]["r"]) for k in A)
        out[f"w={w}"] = {"every_pair_equal": eq, "ratio_b_over_a": sb / sa}
    out["exactly_1_at_all_weights"] = all(out[f"w={w}"]["every_pair_equal"] and out[f"w={w}"]["ratio_b_over_a"] == 1.0 for w in WEIGHTS)
    return out


def wall_checks(b):
    recs = v.load_wall(b)
    out = common_file_checks(b, "wall", recs)
    eks = v.session_dir(b, "wall")
    g = jload(os.path.join(eks, "l2-gate.json"))
    lv, mism, pref = regrade_counts(recs)
    out["cond1_exit"] = out["job_exit"]["exit_code"] == 0 and out["job_exit"]["single_pod_uid"]
    a = out["audit"]
    out["cond2_audit"] = bool(a and a["ok"] and a["session_status"] == "complete" and a["problems"] == 0
                              and a["replays_identical"] == a["measured_records"] == a["replays"] and a["replay_seqs_are_all_measured"])
    out["cond3_counted_aa"] = {"dgb->dgb-aa": counted_aa(recs, "dgb", "dgb-aa"),
                               "dgbf-edge->dgbf": counted_aa(recs, "dgbf-edge", "dgbf")}
    out["cond3"] = all(x["exactly_1_at_all_weights"] for x in out["cond3_counted_aa"].values())
    sb = out["spec_build"]
    out["cond4"] = all(sb[k] for k in ("input_spec_json_equal_frozen", "build_info_revision", "build_info_ecbench_sha256", "binary_sha256_file"))
    out["l2_gate"] = {"accepted": g["accepted"], "invalid_outcomes": g["invalid_outcomes"], "below_l2": len(g["below_l2"]),
                      "grades": g["grades"], "executions": g["executions"],
                      "records_levels": lv, "gate_grades_equal_record_counts": {x["level"]: x["count"] for x in g["grades"]} == lv,
                      "level_from_blockers_mismatches": mism, "blocker_kinds": pref}
    out["cond5"] = (g["accepted"] is True and g["invalid_outcomes"] == [] and not g["below_l2"]
                    and set(lv) <= {"L2", "L3"} and out["l2_gate"]["gate_grades_equal_record_counts"] and not mism
                    and g["executions"] == len(recs))
    out["raw_regrade"] = raw_regrade_wall(b, recs)
    cc = jload(os.path.join(eks, "control-comparison.json"))
    w = cc["wall"]
    out["cond6_control_comparison"] = {
        "a": cc["a"]["arm"], "b": cc["b"]["arm"], "status": w["status"], "pairs": w["pairs"],
        "pairs_below_level": w["pairs_below_level"], "aa_pairs": w["aa_pairs"], "aa_ci95": w["aa_ci95"],
        "self_aa": w["self_aa"], "aa_arms": w["aa_arms"], "aa_selection": w["aa_selection"],
        "ratio_b_over_a": w["ratio_b_over_a"], "ci95": w["ci95"], "aa_ratio": w["aa_ratio"],
        "corrected_ratio": w["corrected_ratio"], "corrected_ci95": w["corrected_ci95"],
        "corrected_dof": w["corrected_dof"], "ci_method": w["ci_method"], "resample_seed": cc["resample_seed"],
        "ops_ratio": cc["ops"]["ratio_b_over_a"], "ops_band": cc["ops"]["band"]["ratio_b_over_a"]}
    c6 = out["cond6_control_comparison"]
    out["cond6"] = (c6["a"] == "dgbf-edge" and c6["b"] == "dgb-aa" and c6["status"] == "admitted" and c6["pairs"] == 288
                    and c6["pairs_below_level"] == 0 and c6["aa_pairs"] == 288 and c6["aa_ci95"] is not None
                    and c6["self_aa"] is False and set(c6["aa_arms"]) == {"dgbf", "dgbf-edge"}
                    and c6["aa_selection"] == "includes arm A")
    out["positions"] = positions(recs)
    return out


def counted_checks(b):
    recs = v.load_counted(b)
    out = common_file_checks(b, "counted", recs)
    eks = v.session_dir(b, "counted")
    graw = jload(os.path.join(eks, "l2-gate.json"))
    below = [{"seq": x["seq"], "arm": x["arm"], "level": x["isolation"]["level"],
              "blockers": v._mask_blockers(x["isolation"]["blockers"])} for x in graw["below_l2"]]
    g = {"accepted": graw["accepted"], "invalid_outcomes": graw["invalid_outcomes"], "grades": graw["grades"],
         "executions": graw["executions"]}
    del graw
    lv, mism, pref = regrade_counts(recs)
    a = out["audit"]
    rho = {"rho-neg", "rho-neg-aa"}

    def cond5_ok(arm, level, blockers):
        if level in ("L2", "L3"):
            return True
        if level != "L1":
            return False
        l2 = [x for x in blockers if x.startswith("L2: ")]
        return all(x == v.TOO_SHORT for x in l2) and not any(x.startswith("L1: ") for x in blockers)

    ic = [r for r in recs if r["method"]["id"] == "ic.pipeline"]
    rh = [r for r in recs if r["arm"] in rho]
    rec_below = sorted(r["seq"] for r in recs if r["isolation"]["level"] not in ("L2", "L3"))
    out["gate"] = {
        "cond1_audit": bool(a and a["ok"] and a["session_status"] == "complete" and a["problems"] == 0
                            and a["replays_identical"] == a["measured_records"] == a["replays"] and a["replay_seqs_are_all_measured"]),
        "cond2_rho_aa": counted_aa(recs, "rho-neg", "rho-neg-aa"),
        "cond3_spec_build": all(out["spec_build"][k] for k in ("input_spec_json_equal_frozen", "build_info_revision", "build_info_ecbench_sha256", "binary_sha256_file")),
        "cond4_ic_all_L2_L3": all(r["isolation"]["level"] in ("L2", "L3") for r in ic) and not any(x["arm"] not in rho for x in below),
        "cond5_rho": all(cond5_ok(r["arm"], r["isolation"]["level"], r["isolation"]["blockers"]) for r in rh),
        "cond6_invalid_outcomes_empty": g["invalid_outcomes"] == [],
        "below_l2_from_gate": {"count": len(below), "per_arm": dict(collections.Counter(x["arm"] for x in below)),
                               "levels": dict(collections.Counter(x["level"] for x in below)),
                               "all_too_short_only": all(cond5_ok(x["arm"], x["level"], x["blockers"]) and x["level"] == "L1" for x in below)},
        "below_l2_from_records_equal_gate": rec_below == sorted(x["seq"] for x in below),
        "records_levels": lv, "gate_grades_equal_record_counts": {x["level"]: x["count"] for x in g["grades"]} == lv,
        "level_from_blockers_mismatches": mism, "accepted": g["accepted"], "executions": g["executions"],
    }
    gt = out["gate"]
    gt["cond2"] = gt["cond2_rho_aa"]["exactly_1_at_all_weights"]
    c16 = gt["cond1_audit"] and gt["cond2"] and gt["cond3_spec_build"] and gt["cond4_ic_all_L2_L3"] and gt["cond5_rho"] and gt["cond6_invalid_outcomes_empty"]
    gt["conditions_1_to_6"] = c16
    ec = out["job_exit"]["exit_code"]
    gt["EXIT42_CLAUSE"] = (ec == 42 and out["job_exit"]["single_pod_uid"] and c16 and len(below) > 0
                           and all(x["arm"] in rho for x in below) and gt["below_l2_from_gate"]["all_too_short_only"]
                           and gt["below_l2_from_records_equal_gate"])
    gt["EXIT43_CLAUSE"] = False if ec != 43 else None
    gt["route_satisfied"] = (ec == 0 and c16) or gt["EXIT42_CLAUSE"]
    gt["note"] = "the solve-time range of the waived rho executions is not reported: it is rho wall time (PROTOCOL 10)"
    return out


def main():
    res = {}
    for b in (32, 24):
        res[f"P{b}-wall"] = wall_checks(b)
    for b in (32, 24, 28):
        res[f"P{b}-counted"] = counted_checks(b)
    v.dump(res, sys.argv[1])
    for k, r in res.items():
        if k.endswith("wall"):
            print(k, {c: r[c] for c in ("cond1_exit", "cond2_audit", "cond3", "cond4", "cond5", "cond6")},
                  "regrade disagreements", len(r["raw_regrade"]["disagreements"]), r["raw_regrade"]["flag_counts"])
            print("  positions", r["positions"]["positions"], r["positions"]["measured_orders"])
            print("  predecessors", r["positions"]["predecessor_within_block"])
        else:
            gt = r["gate"]
            print(k, "exit", r["job_exit"]["exit_code"], {c: gt[c] for c in ("cond1_audit", "cond2", "cond3_spec_build", "cond4_ic_all_L2_L3", "cond5_rho", "cond6_invalid_outcomes_empty", "EXIT42_CLAUSE", "route_satisfied")},
                  gt["below_l2_from_gate"])
        print("  sums", r["sha256sums"], "spec", {kk: vv for kk, vv in r["spec_build"].items() if vv is not True})


if __name__ == "__main__":
    main()
