#!/usr/bin/env python3
"""Pre-flight runtime and cost model for the P4 confirmatory jobs.

research/prime_medium_rho_p4_confirm_20261108, PROTOCOL.md section 11.
Standard library only. Run from the repository root:

    python3 -I research/prime_medium_rho_p4_confirm_20261108/analysis/cost_model.py \
        --out research/prime_medium_rho_p4_confirm_20261108/analysis/cost.json

Model (derived, not measured): one job = settle + solve + replay + overhead,
  solve   = (rounds + warmup) * targets * sum_curves sqrt(r) * sum_arms S_arm * ns_arm,
  replay  = rounds * targets * ... (the pod's verify --replay-all replays measured runs only),
  overhead= 2 * executions * 5 ms (fit to the old J1 session),
  settle  = 60 s; provisioning and launcher time is added separately.
S_arm and ns_arm are the 2026-11-07 c8a.2xlarge L2 values (largest per-arm ns/GAE seen in
any of its jobs); bsgs.textbook's ns/GAE is taken equal to bsgs.negation's (the local
development preflight saw them within 10 % of each other). The model is checked against
the six old session walls (solve part only).
"""
import argparse
import json
import math
import os

D = "research/prime_medium_rho_p4_confirm_20261108"
S = {"rho-neg": 0.969, "rho-neg-aa": 0.969, "rho-plain": 1.270, "rho-neg-v2": 0.947,
     "bsgs-neg": 1.0, "bsgs-neg-aa": 1.0, "bsgs-tb": 1.5}
NS = {"rho-neg": 122.8, "rho-neg-aa": 122.8, "rho-plain": 116.6, "rho-neg-v2": 123.3,
      "bsgs-neg": 155.8, "bsgs-neg-aa": 155.8, "bsgs-tb": 155.8}
PER_EXEC = 0.005
SETTLE = 60.0
PROVISION = 300.0          # assumption: node start + image pull + launcher, per job
CAP = 3600.0
PRICE_SPOT = (0.0988, 0.1577)   # USD/h c8a.2xlarge us-east-2, describe-spot-price-history 2026-11-07T20Z..11-08T03Z


def job_model(spec):
    arms = [a["name"] for a in spec["arms"]]
    roots = [math.sqrt(c["r"]) for c in spec["workloads"]["curves"]]
    t = spec["workloads"]["targets_per_curve"]
    m = spec["measurement"]
    per_round = sum(roots) * sum(S[a] * NS[a] for a in arms) * 1e-9 * t
    execs = (m["rounds"] + m["warmup"]) * t * len(roots) * len(arms)
    solve = (m["rounds"] + m["warmup"]) * per_round
    replay = m["rounds"] * per_round
    over = 2 * execs * PER_EXEC
    pod = SETTLE + solve + replay + over
    return {"executions": execs, "solve_s": solve, "replay_s": replay, "overhead_s": over,
            "pod_s": pod, "with_provisioning_s": pod + PROVISION, "margin_under_cap": 1 - pod / CAP}


def old_check():
    old = "research/prime_medium_tier_rho_20261107"
    out = {}
    for j in ["J1", "J2", "J3", "J4", "J5", "J6"]:
        spec = json.load(open(os.path.join(old, "specs", j + ".json")))
        sess = json.load(open(os.path.join(old, "sessions", j, "session.json")))
        # old arms include bsgs-neg; same S/ns tables
        arms = [a["name"] for a in spec["arms"]]
        roots = [math.sqrt(c["r"]) for c in spec["workloads"]["curves"]]
        t = spec["workloads"]["targets_per_curve"]
        m = spec["measurement"]
        s_old = dict(S, **{"bsgs-neg": 0.984})
        solve = (m["rounds"] + m["warmup"]) * sum(roots) * t * sum(s_old[a] * NS[a] for a in arms) * 1e-9
        solve += (m["rounds"] + m["warmup"]) * t * len(roots) * len(arms) * PER_EXEC
        actual = (sess["finished_unix_ms"] - sess["started_unix_ms"]) / 1000
        out[j] = {"model_session_s": solve, "actual_session_s": actual, "ratio_actual_over_model": actual / solve}
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    jobs = {}
    for j in ["M1", "M2", "M3", "M4", "M5", "M6", "M7", "M8", "M9", "PC"]:
        jobs[j] = job_model(json.load(open(os.path.join(D, "specs", j + ".json"))))
    tot_pod = sum(v["pod_s"] for v in jobs.values())
    tot = sum(v["with_provisioning_s"] for v in jobs.values())
    doc = {"schema": "prime_medium_rho_p4_confirm.cost/v1", "evidence_class": "derived (model) from [pcc:unverified] counts and walls",
           "assumptions": {"S": S, "ns_per_gae": NS, "per_execution_s": PER_EXEC, "settle_s": SETTLE,
                           "provisioning_s_per_job": PROVISION, "spot_usd_per_hour": PRICE_SPOT},
           "jobs": jobs, "total_pod_s": tot_pod, "total_with_provisioning_s": tot,
           "instance_hours": tot / 3600,
           "compute_usd_range": [tot / 3600 * PRICE_SPOT[0], tot / 3600 * PRICE_SPOT[1]],
           "old_session_check": old_check()}
    with open(a.out, "w") as fh:
        json.dump(doc, fh, indent=1, sort_keys=True)
        fh.write("\n")
    for j, v in jobs.items():
        print("%-3s execs %5d solve %6.0f replay %6.0f pod %6.0f s  margin %.0f%%" % (
            j, v["executions"], v["solve_s"], v["replay_s"], v["pod_s"], 100 * v["margin_under_cap"]))
    print("total pod %.0f s, with provisioning %.0f s = %.2f instance-hours, USD %.2f-%.2f" % (
        tot_pod, tot, tot / 3600, *doc["compute_usd_range"]))
    for j, v in doc["old_session_check"].items():
        print("old %s model %.0f s actual %.0f s ratio %.2f" % (j, v["model_session_s"], v["actual_session_s"], v["ratio_actual_over_model"]))


if __name__ == "__main__":
    main()
