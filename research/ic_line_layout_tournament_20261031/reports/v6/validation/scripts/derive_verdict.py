#!/usr/bin/env python3
"""Re-derive per-session gates and the combined V6 verdict from the frozen rule text.

Sources of the rule: PROTOCOL-V6.md section 6, protocol-v6.json decision_rule.text, and
PROTOCOL-V6-AMENDMENT-1.md sections 2.3, 2.4 and 3 (three-session combination, host-epoch
condition, s1 determinism). Inputs are the validator's own recompute.json and replay
receipts plus each wall job's job-exit.json; producer outputs are read only at the end, to
compare. Run with `python3 -I`.

usage: derive_verdict.py OUT_JSON RECOMPUTE_JSON REPLAY_DIR JOBEXIT_S1 JOBEXIT_S2 JOBEXIT_S3 \
                         PRODUCER_VERDICT_RUN_I PRODUCER_VERDICT_RUN_II S1DIR S2DIR S3DIR
"""
import json
import sys

WALL_RECORDS, WALL_PAIRS = 576, 128


def J(p):
    with open(p) as f:
        return json.load(f)


def replay_gate(receipt, exit_code, records):
    return (exit_code == 0 and receipt.get("ok") is True and receipt.get("regraded") is True
            and receipt.get("records_bound") is True and receipt.get("records", 0) > 0
            and receipt.get("verified_records") == receipt.get("records")
            and len(receipt.get("replays", [])) > 0
            and all(r.get("reproduced") is True for r in receipt["replays"])
            and records == WALL_RECORDS)


def aa_ok(ci):
    return ci is not None and ci[0] <= 1 <= ci[1] and ci[0] >= 0.9 and ci[1] <= 1.1


def session_gates(w, receipt, rexit, jobexit):
    c, aa, cc, pos = w["candidate"], w["aa"], w["candidate_corrected"], w["positive"]
    g = {}
    g["replay_and_audit"] = replay_gate(receipt, rexit, w["records"])
    # exact counted parity: every measured pair has identical GAE (so the ratio is exactly 1
    # and the interval is [1, 1]), same algorithm seeds, no unpaired run
    g["counted_parity_ops"] = (c["ops_S_ratio_w0"] == 1.0 and c["gae_equal_every_pair"]
                               and c["same_algorithm_seed"] and c["unpaired_runs"] == 0)
    g["counted_parity_records"] = (w["record_parity"]["mismatched_cells"] == 0
                                   and w["record_parity"]["cells_incumbent"] == 144
                                   and w["record_parity"]["cells_candidate"] == 144)
    # admission: 128 pairs, none below L2, job exited 0 (status 'admitted' is ecbench's own
    # label and is cross-read from the producer below)
    g["admission"] = (c["n"] == WALL_PAIRS and c["pairs_below_L2"] == 0 and jobexit["exit_code"] == 0)
    # A/A: incumbent-aa / incumbent interval (the pair both the candidate comparison and
    # control-comparison.json use) non-null, contains 1, inside [0.9, 1.1]
    g["aa_inside_0_9_1_1"] = aa_ok(aa["ci95_log"])
    g["positive_control_detected"] = (pos["ops_S_ratio_w0"] > 1 and pos["ratio"] > 1
                                      and pos["ci95_log"][0] > 1 and pos["pairs_below_L2"] == 0)
    g["total_and_corrected_ci_below_1"] = (c["ratio"] < 1 and c["ci95_log"][1] < 1
                                           and cc["ratio"] < 1 and cc["ci95_log"][1] < 1)
    # outside_noise: two readings, both evaluated; the verdict is reported under each
    disjoint = c["ci95_log"][0] > aa["ci95_log"][1] or c["ci95_log"][1] < aa["ci95_log"][0]
    corr_excl_1 = cc["ci95_log"][0] > 1 or cc["ci95_log"][1] < 1
    g["outside_noise"] = disjoint and corr_excl_1
    g["_outside_noise_readings"] = {"candidate_ci_disjoint_from_aa_ci": disjoint,
                                    "corrected_ci_excludes_1": corr_excl_1}
    return g


def verdict(g, jobexit):
    if jobexit["exit_code"] != 0:
        return "invalid_measurement"
    valid = all(g[k] for k in ("replay_and_audit", "counted_parity_ops", "counted_parity_records",
                               "admission", "aa_inside_0_9_1_1", "positive_control_detected"))
    if not valid:
        return "invalid_measurement"
    if g["total_and_corrected_ci_below_1"] and g["outside_noise"]:
        return "supported_within_scope"
    if not g["outside_noise"]:
        return "within_control_distribution"
    return "inconclusive"


def combine(vs, host_epoch_holds, determinism_ok):
    if not determinism_ok:
        return "invalid_measurement"
    if all(v == "supported_within_scope" for v in vs):
        out = "supported_within_scope"
    elif any(v == "invalid_measurement" for v in vs):
        out = "invalid_measurement"
    elif any(v == "inconclusive" for v in vs):
        out = "inconclusive"
    else:
        out = "within_control_distribution"
    if not host_epoch_holds and out == "supported_within_scope":
        out = "inconclusive"
    return out


def main(a):
    out_path, rc_path, replay_dir = a[1], a[2], a[3]
    je = {"s1": J(a[4]), "s2": J(a[5]), "s3": J(a[6])}
    prod_i, prod_ii = J(a[7]), J(a[8])
    dirs = {"s1": a[9], "s2": a[10], "s3": a[11]}
    rc = J(rc_path)
    res = {"schema": "validator.verdict/v1", "sessions": {}}
    for s in ("s1", "s2", "s3"):
        receipt = J(f"{replay_dir}/{s}-replay-all.json")
        with open(f"{replay_dir}/{s}-replay-all.exit") as f:
            rexit = int(f.read().strip())
        g = session_gates(rc["wall"][s], receipt, rexit, je[s])
        v = verdict(g, je[s])
        # alternative outside_noise reading cannot change the label here unless it flips
        alt = {}
        for name, val in g["_outside_noise_readings"].items():
            g2 = dict(g, outside_noise=val)
            alt[name] = verdict(g2, je[s])
        res["sessions"][s] = {"job_exit": je[s], "gates": g, "verdict": v,
                              "verdict_under_each_outside_noise_reading": alt}
    s1, s2, s3 = je["s1"], je["s2"], je["s3"]
    host = (s3["exit_code"] == 0 and s3["node_name"] != s1["node_name"] and s3["node_name"] != s2["node_name"]
            and s3["pod_uid"] != s1["pod_uid"] and s3["pod_uid"] != s2["pod_uid"])
    h1 = s1["node_name"] == s2["node_name"]

    def det(p):
        for x in p["sessions"]:
            if x["dir"] == dirs["s1"]:
                return json.dumps({k: x[k] for k in ("verify_exit", "verdict", "gates")}, sort_keys=True,
                                  separators=(",", ":"))
        return None
    da, db = det(prod_i), det(prod_ii)
    determinism = da is not None and da == db
    vs = [res["sessions"][s]["verdict"] for s in ("s1", "s2", "s3")]
    res["host_epoch"] = {"h1_same_node_s1_s2": h1, "h2_condition_holds": host,
                         "h1_node": s1["node_name"], "h2_node": s3["node_name"]}
    res["s1_determinism"] = {"run_i": da, "run_ii": db, "byte_equal": determinism}
    res["combined_verdict"] = combine(vs, host, determinism)
    # cross-read producer per-session verdicts and gates
    prod = {}
    for s, p, key in (("s1", prod_i, dirs["s1"]), ("s2", prod_i, dirs["s2"]), ("s3", prod_ii, dirs["s3"])):
        x = next(x for x in p["sessions"] if x["dir"] == key)
        mine = {k: v for k, v in res["sessions"][s]["gates"].items() if not k.startswith("_")}
        prod[s] = {"producer_verdict": x["verdict"], "producer_gates": x["gates"],
                   "verdict_agrees": x["verdict"] == res["sessions"][s]["verdict"],
                   "gates_agree": x["gates"] == mine,
                   "gate_disagreements": sorted(k for k in mine if mine[k] != x["gates"].get(k))}
    res["producer_crosscheck"] = prod
    res["producer_overall_fields"] = {"run_i": prod_i["verdict"], "run_ii": prod_ii["verdict"],
                                      "note": "neither overall field is the verdict (amendment 2.4)"}
    with open(out_path, "w") as f:
        json.dump(res, f, indent=1, sort_keys=True)
    print(json.dumps({"per_session": dict(zip(("s1", "s2", "s3"), vs)), "host_epoch": host,
                      "determinism": determinism, "combined": res["combined_verdict"],
                      "producer_agrees": {s: (prod[s]["verdict_agrees"], prod[s]["gates_agree"]) for s in prod}}))


if __name__ == "__main__":
    main(sys.argv)
