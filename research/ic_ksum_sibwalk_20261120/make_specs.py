#!/usr/bin/env python3
"""Spec generator (cwd = this directory); lays out JSON only, computes no result.
pilot:        python3 make_specs.py pilot           -> pilot/specs/<row>.json   (64 targets, 1 round; m in 4, 8, 16)
confirmatory: python3 make_specs.py confirm M.json  -> specs/<row>-r<rep>.json  (8 rows x 3 replicates; M.json = registered m per row)
Curves, max_trials, timeout and order come from the prefix-joint study's spec of the same name; seeds are fresh."""
import json, sys, os

ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]
PJ = "../ic_ksum_prefix_joint_20261118/specs"
# Incumbent best per row (research/ic_ksum_prefix_joint_20261118, registered best): prefix-joint where L1 held, joint elsewhere.
PREFIX_ROWS = {"P20", "B15", "B18", "B21", "B24"}
TPC = {"pilot": 64, "confirm": 48}


def base(row):
    s = json.load(open(f"{PJ}/{row}-r1.json"))
    inc = next(a for a in s["arms"] if a["name"] == ("prefix-joint" if row in PREFIX_ROWS else "joint"))["method"]["params"]
    assert inc["targets"] == "stride-renew-joint", inc
    assert inc["oracle"].endswith(",prefix=1") == (row in PREFIX_ROWS), inc
    return s, dict(inc)


def arm(name, params, extra="", role="candidate"):
    q = dict(params)
    q["oracle"] = params["oracle"] + extra
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": q}}


def write(path, spec):
    with open(path, "w") as f:
        json.dump(spec, f, indent=1)
        f.write("\n")


DESC = ("k-sum Table B arm (prune=1, split=1, stride-renew-joint, prefix=1 where it held): walk_sib=m forms the same "
        "target progression in blocks of 2m+1 around a centre, C+[i]S full and x(C-[i]S) from one shared inversion "
        "(one add + one sibling_second_points), x probed by one lookup. Generic decomposition track (bounded by "
        "sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.")
RHO = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]


def pilot():
    os.makedirs("pilot/specs", exist_ok=True)
    for i, row in enumerate(ROWS):
        s, inc = base(row)
        arms = [arm("inc", inc), arm("off", inc, ",walk_sib=0")] + [arm(f"sib{m}", inc, f",walk_sib={m}") for m in (4, 8, 16)]
        write(f"pilot/specs/{row}.json", {
            "schema": "ecbench.spec/v1", "label": f"ic_ksum_sibwalk_20261120 pilot {row}", "description": "Pilot (not data). " + DESC,
            "workloads": {**s["workloads"], "targets_per_curve": TPC["pilot"], "target_seed": 20261120500 + i},
            "arms": RHO + arms, "measurement": {**s["measurement"], "rounds": 1, "warmup": 0, "seed": 20261120550 + i}})


def confirm(mfile):
    m_of = json.load(open(mfile))["m"]
    os.makedirs("specs", exist_ok=True)
    for i, row in enumerate(ROWS):
        s, inc = base(row)
        m = int(m_of[row])
        arms = [arm("inc", inc), arm("off", inc, ",walk_sib=0"), arm("cand", inc, f",walk_sib={m}")]
        if row not in PREFIX_ROWS:
            # Fresh registered test of prefix=1 on the three rows where it narrowly failed (secondary, T3).
            arms.append(arm("inc-prefix", inc, ",prefix=1"))
        ref = RHO + [{"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}]
        for rep in (1, 2, 3):
            idx = 10 * i + rep
            write(f"specs/{row}-r{rep}.json", {
                "schema": "ecbench.spec/v1", "label": f"ic_ksum_sibwalk_20261120 {row}-r{rep}", "description": "Confirmatory. " + DESC,
                "workloads": {**s["workloads"], "targets_per_curve": TPC["confirm"], "target_seed": 20261120300 + idx},
                "arms": ref + arms, "measurement": {**s["measurement"], "seed": 20261120400 + idx}})


if sys.argv[1] == "pilot":
    pilot()
else:
    confirm(sys.argv[2])
