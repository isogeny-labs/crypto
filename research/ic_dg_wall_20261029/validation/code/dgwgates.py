#!/usr/bin/env python3
"""Validator re-check of PROTOCOL.md section 6.1 (both sessions), 6.2 and the 28-bit E6
conditions 1-4, from the job's own files and records.  Stdlib only; own code.

Usage: python3 -I dgwgates.py <evidence_dir> <frozen_dir> <recompute.json> <out.json>
"""
import collections
import hashlib
import json
import math
import os
import sys

EV, FZ, RC, OUTP = sys.argv[1:5]
rc = json.load(open(RC))
LEVEL = {"L0": 0, "L1": 1, "L2": 2, "L3": 3}
SRC = "be2969cebf30a0cfb79088fd10094324c3efb3bb"
BIN = "88070375e77648aa12a561ff9ee59e34d469c55e1da56987234811b0956e412c"


def J(*p):
    return json.load(open(os.path.join(EV, *p)))


def sums_ok(d):
    bad = []
    for line in open(os.path.join(d, "SHA256SUMS")):
        h, f = line.split(None, 1)
        f = f.strip()
        if hashlib.sha256(open(os.path.join(d, f), "rb").read()).hexdigest() != h:
            bad.append(f)
    return bad


def records(b, s):
    out = []
    for line in open(os.path.join(EV, "P%d" % b, s, "eks-results/session/records.jsonl")):
        out.append(json.loads(line))
    return out


def audit_check(a, n_measured):
    reps = a["replays"]
    return {"ok": a["ok"] is True, "session_status": a["session_status"], "problems": len(a["problems"]),
            "records": a["records"], "verified_records": a["verified_records"], "replays": len(reps),
            "replays_all_identical": all(r["reproduced"] is True and r["detail"] == "identical" for r in reps),
            "replays_cover_all_measured": len(reps) == n_measured, "checks_all_true": all(a["checks"].values()),
            "auditor_binary": a["auditor_binary_sha256"] == BIN,
            "pass": a["ok"] is True and a["session_status"] == "complete" and len(a["problems"]) == 0
            and all(r["reproduced"] is True and r["detail"] == "identical" for r in reps) and len(reps) == n_measured}


def build_check(b, s):
    bi = J("P%d" % b, s, "eks-results/build-info.json")
    return {"source_revision": bi["source_revision"], "ecbench_sha256": bi["ecbench_sha256"],
            "pass": bi["source_revision"] == SRC and bi["ecbench_sha256"] == BIN}


def spec_check(b, s):
    fz = json.load(open(os.path.join(FZ, "spec-P%d%s.json" % (b, "" if s == "wall" else "-rho"))))
    inp = J("P%d" % b, s, "eks-results/input-spec.json")
    return {"input_spec_json_equal_frozen": inp == fz, "pass": inp == fz}


out = {}
for b in (24, 28, 32):
    R = rc["sizes"]["P%d" % b]
    res = {}
    # ------------------------------------------------------------ wall session
    s = "wall"
    d = os.path.join(EV, "P%d" % b, s, "eks-results")
    W = {"sha256sums_mismatch": sums_ok(d)}
    je = J("P%d" % b, s, "eks-results/job-exit.json")
    W["c1_exit"] = {"exit_code": je["exit_code"], "pod_uid": je["pod_uid"],
                    "pod_uid_matches_result_receipt": je["pod_uid"] == J("P%d" % b, s, "launch/receipts/result.json")["pod_uid"],
                    "pass": je["exit_code"] == 0}
    recs = records(b, s)
    nmeas = sum(1 for r in recs if not r["warmup"])
    W["c2_audit"] = audit_check(J("P%d" % b, s, "eks-results/audit.json"), nmeas)
    aa = R["aa_counted_wall_session"]
    W["c3_counted_aa"] = {"ratios": aa["ratio"], "pairwise_identical": aa["pairwise_identical_counts_and_seeds"],
                          "pass": aa["exactly_1_at_all_weights"]}
    # also as written in the committed srenew__srenew-aa files
    cf = {wk: J("P%d" % b, s, "analysis/compare-srenew__srenew-aa-w%s.json" % wk)["ops"]["ratio_b_over_a"] for wk in ("0", "0.1", "1")}
    W["c3_counted_aa"]["committed_files_ratio"] = cf
    W["c3_counted_aa"]["pass"] &= all(v == 1.0 for v in cf.values())
    sp, bu = spec_check(b, s), build_check(b, s)
    W["c4_spec_build"] = {"spec": sp, "build": bu, "pass": sp["pass"] and bu["pass"]}
    g = J("P%d" % b, s, "eks-results/l2-gate.json")
    lv = collections.Counter(r["isolation"]["level"] for r in recs)
    W["c5_gate"] = {"accepted": g["accepted"], "grades": g["grades"], "invalid_outcomes": len(g["invalid_outcomes"]),
                    "below_l2": len(g["below_l2"]), "records_levels": dict(lv),
                    "records_all_verified": all(r["outcome"]["status"] == "verified" for r in recs),
                    "executions": g["executions"], "records": len(recs),
                    "pass": g["accepted"] is True and not g["invalid_outcomes"] and not g["below_l2"]
                    and all(LEVEL[r["isolation"]["level"]] >= 2 for r in recs) and g["executions"] == len(recs)}
    cc = J("P%d" % b, s, "eks-results/control-comparison.json")
    ccw = cc["wall"]
    W["c6_native_aa"] = {"arms": [cc["a"]["arm"], cc["b"]["arm"]], "status": ccw["status"],
                         "pairs_below_level": ccw["pairs_below_level"], "aa_pairs": ccw["aa_pairs"],
                         "aa_ci95": ccw["aa_ci95"], "aa_arms": ccw["aa_arms"], "aa_selection": ccw["aa_selection"],
                         "pass": ccw["status"] == "admitted" and ccw["pairs_below_level"] == 0 and ccw["aa_pairs"] >= 5
                         and ccw["aa_ci95"] is not None and ccw["aa_arms"] == ["srenew", "srenew-aa"]}
    files = {wk: J("P%d" % b, s, "analysis/compare-srenew__dg-w%s.json" % wk) for wk in ("0", "0.1", "1")}
    w0 = files["0"]["wall"]
    same = all(json.dumps(files[wk]["wall"], sort_keys=True) == json.dumps(w0, sort_keys=True) for wk in ("0.1", "1"))
    mine = R["wall_decision"]
    W["c7_decision_block"] = {"status": w0["status"], "pairs": w0["pairs"], "pairs_below_level": w0["pairs_below_level"],
                              "self_aa": w0["self_aa"], "aa_arms": w0["aa_arms"], "aa_selection": w0["aa_selection"],
                              "aa_pairs": w0["aa_pairs"], "corrected_ci95": w0["corrected_ci95"],
                              "wall_block_identical_across_weight_files": same,
                              "validator_recompute": {"pairs": mine["pairs"], "admitted": mine["admitted"],
                                                      "pairs_below_level": mine["pairs_below_level"],
                                                      "aa_pairs": mine["aa_pairs"], "aa_arms": mine["aa_arms"],
                                                      "aa_selection": mine["aa_selection"], "self_aa": mine["self_aa"]},
                              "pass": w0["status"] == "admitted" and w0["pairs"] == 288 and w0["pairs_below_level"] == 0
                              and w0["self_aa"] is False and w0["aa_arms"] == ["srenew", "srenew-aa"]
                              and w0["aa_selection"] == "includes arm A" and w0["aa_pairs"] == 288
                              and w0["corrected_ci95"] is not None and same
                              and mine["admitted"] == 288 and mine["aa_pairs"] == 288}
    ops = R["ops_wall_session"]["srenew__dg"]
    W["c8_same_charged_work"] = {"ops_ratio_w0": ops["0"]["ratio_b_over_a"],
                                 "abs_ratio_minus_1_w0": abs(ops["0"]["ratio_b_over_a"] - 1.0),
                                 "ops_ci95_upper_w0.1": ops["0.1"]["ci95"][1], "ops_ci95_upper_w1": ops["1"]["ci95"][1],
                                 "committed_ops_ratio_w0": files["0"]["ops"]["ratio_b_over_a"],
                                 "pass": abs(ops["0"]["ratio_b_over_a"] - 1.0) < 0.0005 and ops["0.1"]["ci95"][1] < 1.0
                                 and ops["1"]["ci95"][1] < 1.0}
    W["all_1_to_8"] = all(W[k]["pass"] for k in W if k.startswith("c")) and not W["sha256sums_mismatch"]
    up = mine["corrected_ci95"][1]
    lo = mine["corrected_ci95"][0]
    W["section_6_2"] = {"corrected_ratio": mine["corrected_ratio"], "corrected_ci95": mine["corrected_ci95"],
                        "reading": ("improvement" if up < 1 else "regression" if lo > 1 else "null") if W["all_1_to_8"] else "undecidable"}
    res["wall"] = W

    # ------------------------------------------------------------ counted session
    s = "counted"
    d = os.path.join(EV, "P%d" % b, s, "eks-results")
    C = {"sha256sums_mismatch": sums_ok(d)}
    recs = records(b, s)
    nmeas = sum(1 for r in recs if not r["warmup"])
    C["v3c1_audit"] = audit_check(J("P%d" % b, s, "eks-results/audit.json"), nmeas)
    aa = R["aa_counted_counted_session"]
    cf = {wk: J("P%d" % b, s, "analysis/compare-rho-neg__rho-neg-aa-w%s.json" % wk)["ops"]["ratio_b_over_a"] for wk in ("0", "0.1", "1")}
    C["v3c2_aa_rho"] = {"ratios": aa["ratio"], "committed_files_ratio": cf,
                        "pairwise_identical": aa["pairwise_identical_counts_and_seeds"],
                        "pass": aa["exactly_1_at_all_weights"] and all(v == 1.0 for v in cf.values())}
    sp, bu = spec_check(b, s), build_check(b, s)
    C["v3c3_spec_build"] = {"spec": sp, "build": bu, "pass": sp["pass"] and bu["pass"]}
    ic = [r for r in recs if r["method"]["id"] == "ic.pipeline"]
    C["v3c4_ic_levels"] = {"executions": len(ic), "levels": dict(collections.Counter(r["isolation"]["level"] for r in ic)),
                           "pass": all(LEVEL[r["isolation"]["level"]] >= 2 for r in ic)}
    rho = [r for r in recs if r["method"]["id"] == "rho.negation"]
    rb = [r for r in rho if LEVEL[r["isolation"]["level"]] < 2]

    def c5_ok(r):
        lvl = r["isolation"]["level"]
        if LEVEL[lvl] >= 2:
            return True
        l2 = [x for x in r["isolation"]["blockers"] if x.startswith("L2: ")]
        return lvl == "L1" and l2 and all(x.startswith("L2: solve too short to admit wall time") for x in l2)
    C["v3c5_rho_levels"] = {"executions": len(rho), "below_l2_by_arm": dict(collections.Counter(r["arm"] for r in rb)),
                            "below_l2_measured_by_arm": dict(collections.Counter(r["arm"] for r in rb if not r["warmup"])),
                            "all_satisfy_condition_5": all(c5_ok(r) for r in rho),
                            "note": "counts only; the solve-time range of these executions is not used (dg-wall section 10); "
                                    "the validator checked only that each one is under the 1 ms grading floor",
                            "every_below_l2_solve_under_1ms": all(r["time"]["solve_wall_ns"] < 1_000_000 for r in rb),
                            "pass": all(c5_ok(r) for r in rho)}
    g = J("P%d" % b, s, "eks-results/l2-gate.json")
    C["v3c6_outcomes"] = {"invalid_outcomes": len(g["invalid_outcomes"]),
                          "records_all_verified": all(r["outcome"]["status"] == "verified" for r in recs),
                          "pass": not g["invalid_outcomes"]}
    je = J("P%d" % b, s, "eks-results/job-exit.json")
    res_rc = J("P%d" % b, s, "launch/receipts/result.json")
    gate_arms = collections.Counter(x["arm"] for x in g["below_l2"])
    seqs_gate = sorted(x["seq"] for x in g["below_l2"])
    seqs_rec = sorted(r["seq"] for r in rb)
    gate_entries_ok = all(x["arm"] in ("rho-neg", "rho-neg-aa") for x in g["below_l2"]) and seqs_gate == seqs_rec
    C["exit_42_clause"] = {"exit_code": je["exit_code"], "launcher_state": res_rc["state"], "launcher_exit": res_rc["exit_code"],
                           "l2_gate_accepted": g["accepted"], "below_l2_entries": len(g["below_l2"]),
                           "below_l2_arms": dict(gate_arms), "below_l2_seqs_equal_records_recompute": seqs_gate == seqs_rec,
                           "control_comparison_present": os.path.exists(os.path.join(d, "control-comparison.json")),
                           "pass": je["exit_code"] == 42 and res_rc["state"] == "failed" and res_rc["exit_code"] == 23
                           and gate_entries_ok and all(C[k]["pass"] for k in C if k.startswith("v3c"))}
    C["route_satisfied"] = C["exit_42_clause"]["pass"] and not C["sha256sums_mismatch"]
    res["counted"] = C

    # ------------------------------------------------------------ 6.2 E6 at 28 bits
    if b == 28:
        E = {"cond1_improvement": W["section_6_2"]["reading"] == "improvement"}
        recs_w = records(b, "wall")
        per = collections.defaultdict(lambda: collections.Counter())
        for r in recs_w:
            if r["warmup"] or r["arm"] not in ("srenew", "dg"):
                continue
            k = r["workload"]["curve"]["slug"]
            per[k][r["arm"] + ".gae"] += r["cost"]["total_gae"]
            per[k][r["arm"] + ".probes"] += sum((p.get("native") or {}).get("target_guard_probes", 0) for p in r["phases"])
        c2, c3, c4 = {}, {}, {}
        for c0 in R["ops_wall_session"]["srenew__dg"]["0"]["curves"]:
            sl = c0["slug"]
            c2[sl] = {"ratio_b_over_a_w0": c0["ratio_b_over_a"], "sum_total_gae_srenew": per[sl]["srenew.gae"],
                      "sum_total_gae_dg": per[sl]["dg.gae"],
                      "pass": c0["ratio_b_over_a"] <= 1.0 and per[sl]["dg.gae"] <= per[sl]["srenew.gae"]}
            ups = {wk: [c for c in R["ops_wall_session"]["srenew__dg"][wk]["curves"] if c["slug"] == sl][0]["ci95"][1]
                   for wk in ("0.1", "1")}
            c3[sl] = {"ci95_upper": ups, "pass": all(v < 1 for v in ups.values())}
            c4[sl] = {"probes_srenew": per[sl]["srenew.probes"], "probes_dg": per[sl]["dg.probes"],
                      "pass": per[sl]["dg.probes"] < per[sl]["srenew.probes"]}
        E.update(cond2_per_curve=c2, cond3_per_curve=c3, cond4_per_curve=c4,
                 cond2=all(v["pass"] for v in c2.values()), cond3=all(v["pass"] for v in c3.values()),
                 cond4=all(v["pass"] for v in c4.values()))
        E["all_four"] = E["cond1_improvement"] and E["cond2"] and E["cond3"] and E["cond4"]
        E["reading"] = "dg replaces srenew as the 28-bit baseline" if E["all_four"] else "srenew stays the 28-bit baseline"
        res["e6_28bit"] = E

    # ------------------------------------------------------------ descriptive: dose-response and per-round stability
    recs_w = records(b, "wall")
    by = collections.defaultdict(dict)
    for r in recs_w:
        if r["warmup"]:
            continue
        by[(r["workload"]["workload_id"], r["round"])][r["arm"]] = r

    def probes(r):
        return sum((p.get("native") or {}).get("target_guard_probes", 0) for p in r["phases"])

    def regress(xs, ys):
        n = len(xs)
        mx, my = sum(xs) / n, sum(ys) / n
        sxx = sum((x - mx) ** 2 for x in xs)
        sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
        syy = sum((y - my) ** 2 for y in ys)
        return {"slope_ns_per_probe": sxy / sxx, "pearson_r": sxy / math.sqrt(sxx * syy), "n": n}
    xs, ys, yaa = [], [], []
    for k, v in by.items():
        removed = probes(v["srenew"]) - probes(v["dg"])
        xs.append(removed)
        ys.append(v["srenew"]["time"]["solve_wall_ns"] - v["dg"]["time"]["solve_wall_ns"])
        yaa.append(v["srenew"]["time"]["solve_wall_ns"] - v["srenew-aa"]["time"]["solve_wall_ns"])
    rounds = {}
    for rd in range(1, 7):
        t = collections.Counter()
        for (wid, r0), v in by.items():
            if r0 != rd:
                continue
            for arm in ("srenew", "dg", "srenew-aa"):
                t[arm + ".w"] += v[arm]["time"]["solve_wall_ns"]
                t[arm + ".g"] += v[arm]["cost"]["total_gae"]
        rounds[str(rd)] = {"corrected_ratio": (t["dg.w"] / t["dg.g"]) / (t["srenew-aa.w"] / t["srenew-aa.g"]),
                           "aa_ratio": (t["srenew-aa.w"] / t["srenew-aa.g"]) / (t["srenew.w"] / t["srenew.g"])}
    res["descriptive"] = {
        "dose_response_wall_saving_vs_removed_probes": regress(xs, ys),
        "null_object_srenew_minus_srenew_aa_vs_removed_probes": regress(xs, yaa),
        "per_round": rounds,
    }
    out["P%d" % b] = res
json.dump(out, open(OUTP, "w"), indent=1)
for b, res in out.items():
    W, C = res["wall"], res["counted"]
    print(b, "wall 1-8:", {k: W[k]["pass"] for k in W if k.startswith("c")}, "all", W["all_1_to_8"], "6.2:", W["section_6_2"]["reading"])
    print(b, "counted:", {k: C[k]["pass"] for k in C if k.startswith(("v3c", "exit"))}, "route", C["route_satisfied"])
    if "e6_28bit" in res:
        E = res["e6_28bit"]
        print(b, "E6:", {k: E[k] for k in ("cond1_improvement", "cond2", "cond3", "cond4", "all_four", "reading")})
    print(b, "descriptive:", json.dumps(res["descriptive"]))
