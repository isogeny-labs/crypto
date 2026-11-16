#!/usr/bin/env python3
"""Write the frozen ecbench specs for LEAD-ICB-tableB-binary-known-log-claw.

Curves: the five Table B random-binary instances, taken verbatim from
docs/curves/registry.json (binary_explicit). Table size per the lead:
T = s^2 near sqrt(r/2), i.e. s = round((r/2)^(1/4)).
"""
import json, math, os, sys
ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../.."))
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "specs")
SLUGS = ["icv1-f2m15-t131-ed7256f0", "icv1-f2m18-t393-d2c04152", "icv1-f2m21-t2591-de54d72e",
         "icv1-f2m24-tm3047-2d0c0e8c", "icv1-f2m27-t12543-569dca8b"]
reg = {c["slug"]: c for c in json.load(open(os.path.join(ROOT, "docs/curves/registry.json")))["curves"]}

def curve(slug):
    c = reg[slug]; rep = c["representations"][0]["curve"]; p = c["params"]
    return {"kind": "binary_explicit", "n": p["m"], "modulus": int(p["modulus"], 16),
            "a": int(p["a"], 16), "b": int(p["b"], 16), "group_order": int(c["order"]),
            "r": int(rep["subgroup_order"]), "gx": int(rep["generator"][0], 16),
            "gy": int(rep["generator"][1], 16)}

def kl(size, seed, r, max_trials=None):
    exp_q = r / (2 * size * size)
    mt = max_trials if max_trials is not None else min(r - 1, int(math.ceil(50 * exp_q)))
    return {"id": "ic.known_log_mitm2_v1", "params": {"size": str(size), "base_seed": str(seed),
            "base_multiplier": "1", "max_pair_work": str(size * (size + 1)), "max_trials": str(mt)}}

MEAS = {"rounds": 4, "warmup": 1, "order": "alternate", "seed": 2026111201,
        "isolation_required": "L0", "timeout_seconds": 120}
sizes = {}
for slug in SLUGS:
    cv = curve(slug); r = cv["r"]; s = round((r / 2) ** 0.25); sizes[slug] = s
    spec = {"schema": "ecbench.spec/v1",
            "label": f"LEAD-ICB-tableB-binary-known-log-claw main {slug}",
            "description": "BOUNDARY (BSGS-class, owner ruling 2026-11-08): known-log pair claw vs matched rho.negation on a Table B random-binary instance; A/A = second base seed. Counts only (L0 container).",
            "workloads": {"curves": [cv], "targets_per_curve": 16, "target_seed": 2026111202, "target_kind": "public"},
            "arms": [
                {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
                {"name": "known-log", "role": "candidate", "method": kl(s, 20261112, r)},
                {"name": "known-log-aa", "role": "control", "method": kl(s, 20261113, r)}],
            "measurement": MEAS}
    json.dump(spec, open(os.path.join(OUT, f"main-{slug.split('-')[1]}.json"), "w"), indent=2)
cv = curve(SLUGS[0]); s = sizes[SLUGS[0]]; r = cv["r"]
pos = {"schema": "ecbench.spec/v1", "label": "known-log claw positive control (planted, f2m15)",
       "description": "Planted targets: the harness checks the recovered scalar against the planted k.",
       "workloads": {"curves": [cv], "targets_per_curve": 8, "target_seed": 2026111203},
       "arms": [{"name": "known-log", "role": "candidate", "method": kl(s, 20261112, r)}],
       "measurement": dict(MEAS, rounds=1)}
json.dump(pos, open(os.path.join(OUT, "control-positive-f2m15.json"), "w"), indent=2)
neg = {"schema": "ecbench.spec/v1", "label": "known-log claw negative control (max_trials=1 < T, f2m15)",
       "description": "max_trials below T: runs must report exhaustion/failure, never an unverified log.",
       "workloads": {"curves": [cv], "targets_per_curve": 16, "target_seed": 2026111204, "target_kind": "public"},
       "arms": [{"name": "known-log-mt1", "role": "candidate", "method": kl(s, 20261112, r, max_trials=1)}],
       "measurement": dict(MEAS, rounds=1)}
json.dump(neg, open(os.path.join(OUT, "control-negative-f2m15.json"), "w"), indent=2)
print(json.dumps(sizes))
