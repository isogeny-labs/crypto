#!/usr/bin/env python3
"""Spec generator (cwd = this directory); lays out JSON only, computes no result.
pilot:        python3 make_specs.py pilot    -> pilot/specs/<row>.json   (8 specs, 64 targets, 1 round, rho-neg + the 4 cells)
confirmatory: python3 make_specs.py confirm  -> specs/<row>-r<rep>.json  (8 rows x 3 replicates)
Curves, max_trials, timeout and order come from the split study's spec of the same name; seeds are fresh."""
import json, sys, os

ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]
SPLIT = "../ic_ksum_split_20261118/specs"
# The incumbent: the split study's cheaper -psr arm at w = 0 (README "best after"), producer N (unchanged by the
# retune null, research/ic_ksum_retune_20261118).
INCUMBENT_K = {"P20": 5, "P22": 5, "P24": 5, "B15": 4, "B18": 4, "B21": 5, "B24": 5, "B27": 5}
# 2x2: L1 = prefix inserts (oracle prefix=1), L2 = joint set-up (targets=stride-renew-joint).
CELLS = [("inc", False, False), ("prefix", True, False), ("joint", False, True), ("prefix-joint", True, True)]
TPC = {"pilot": 64, "confirm": 128}


def arms_for(row):
    s = json.load(open(f"{SPLIT}/{row}-r1.json"))
    k = INCUMBENT_K[row]
    p = next(a for a in s["arms"] if a["name"] == f"ksum{k}-psr")["method"]["params"]
    assert p["oracle"] == f"ksum:m={k},prune=1,split=1" and p["targets"] == "stride-renew", p
    out = []
    for name, prefix, joint in CELLS:
        q = dict(p)
        if prefix:
            q["oracle"] = p["oracle"] + ",prefix=1"
        if joint:
            q["targets"] = "stride-renew-joint"
        out.append({"name": name, "role": "candidate", "method": {"id": "ic.pipeline", "params": q}})
    return s, out


def write(path, spec):
    with open(path, "w") as f:
        json.dump(spec, f, indent=1)
        f.write("\n")


DESC = ("2x2 on the k-sum -psr arm (prune=1, split=1, stride-renew): L1 prefix=1 (key the partial sums the build forms "
        "anyway), L2 stride-renew-joint (Shamir joint double-and-add for the three set-up [a]G+[b]Q). Generic "
        "decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no "
        "security claim.")


def pilot():
    os.makedirs("pilot/specs", exist_ok=True)
    for i, row in enumerate(ROWS):
        s, arms = arms_for(row)
        write(f"pilot/specs/{row}.json", {
            "schema": "ecbench.spec/v1", "label": f"ic_ksum_prefix_joint_20261118 pilot {row}", "description": "Pilot (not data). " + DESC,
            "workloads": {**s["workloads"], "targets_per_curve": TPC["pilot"], "target_seed": 20261118400 + i},
            "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}] + arms,
            "measurement": {**s["measurement"], "rounds": 1, "warmup": 0, "seed": 20261118450 + i}})


def confirm():
    os.makedirs("specs", exist_ok=True)
    for i, row in enumerate(ROWS):
        s, arms = arms_for(row)
        ref = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
               {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}]
        for rep in (1, 2, 3):
            idx = 10 * i + rep
            write(f"specs/{row}-r{rep}.json", {
                "schema": "ecbench.spec/v1", "label": f"ic_ksum_prefix_joint_20261118 {row}-r{rep}", "description": "Confirmatory. " + DESC,
                "workloads": {**s["workloads"], "targets_per_curve": TPC["confirm"], "target_seed": 20261118800 + idx},
                "arms": ref + arms, "measurement": {**s["measurement"], "seed": 20261118900 + idx}})


{"pilot": pilot, "confirm": confirm}[sys.argv[1]]()
