#!/usr/bin/env python3
"""Writes the frozen confirmatory specs of koblitz_k1_setup_w1_20261118 (PROTOCOL.md): K1 n23, n29, n37, three
replicate sessions each, against the current best arm of each curve. Candidate = incumbent + lazy_neg=1 (base) +
orbit_ids=1 (table). No tuning parameter: both levers are on/off."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OR = "mitm-frobenius-layout-counted:m=2,sib=1"
FBX = "raw_x_cap=100000000,generator_orbit=1"
A = {23: 1, 29: 1, 37: 0}
# Current best arm per curve (research/KOBLITZ_K1_SPEEDUP_LOG.md): (columns k, walk/ladder/key-reuse levers on?)
INC = {23: (4, True), 29: (1, False), 37: (8, True)}
SEED = {23: 20261118771, 29: 20261118774, 37: 20261118777}  # replicate r: SEED + r - 1; measurement seed +1000
def arms_for(n):
    k, sw = INC[n]
    fb = f"compact-orbit-scan:columns={k},{FBX}" + (f",generator_multiples={k},multiple_doublings=5" if k > 1 else "") + ",lean_layout=1"
    fb += ",orbit_key_reuse=1" if sw else ""
    orc = OR + (",walk_sib=8,ladder_verify=1" if sw else "")
    def ic(name, role, f, o):
        return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
            "factor_base": f, "oracle": o, "max_trials": "1000000000", "targets": "stride-generator"}}}
    return [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
            {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
            ic("inc", "baseline", fb, orc),
            ic("inc-aa", "control", fb, orc),
            ic("inc-off", "control", fb + ",lazy_neg=0", orc + ",orbit_ids=0"),
            ic("w1", "candidate", fb + ",lazy_neg=1", orc + ",orbit_ids=1"),
            ic("w1-neg", "control", fb + ",lazy_neg=1", orc),
            ic("w1-ids", "control", fb, orc + ",orbit_ids=1")]
for n in (23, 29, 37):
    for rep in (1, 2, 3):
        ts = SEED[n] + rep - 1
        s = {"schema": "ecbench.spec/v1",
             "label": f"K1 set-up levers lazy_neg + orbit_ids vs current best arm, K1 a={A[n]} n={n}, replicate {rep}",
             "description": "Frozen in research/koblitz_k1_setup_w1_20261118/PROTOCOL.md before its data. Candidate w1 = incumbent + lazy_neg=1 + orbit_ids=1; w1-neg, w1-ids each lever alone (secondary); rho strong and v5 references in the same session.",
             "workloads": {"curves": [{"kind": "koblitz", "a": A[n], "n": n}], "targets_per_curve": 48, "target_seed": ts, "target_kind": "public"},
             "arms": arms_for(n),
             "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": ts + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
        p = os.path.join(HERE, "specs", f"n{n}-r{rep}.json"); open(p, "w").write(json.dumps(s, indent=1) + "\n"); print(p)
