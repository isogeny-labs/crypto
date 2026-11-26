#!/usr/bin/env python3
"""Writes the frozen confirmatory specs (K1 a=0 n=37, 3 replicate sessions) of koblitz_n37_sibwalk_k8_20261118.
Candidate rule fixed before data (PROTOCOL.md section 1): k = 8 (the incumbent's columns), walk_sib = 8,
ladder_verify = 1, orbit_key_reuse = 1. Arms and parameter strings as in koblitz_k1_sibwalk_20261118/make_specs.py."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
OR = "mitm-frobenius-layout-counted:m=2,sib=1"
FBX = "raw_x_cap=100000000,generator_orbit=1"
N, A, K, M = 37, 0, 8, 8          # curve; candidate columns (= incumbent) and block half-width (fixed rule)
SEED = 20261118491                # replicate r uses SEED + r - 1; measurement seed = target seed + 1000
def fb(k, extra=""):
    return f"compact-orbit-scan:columns={k},{FBX},generator_multiples={k},multiple_doublings=5,lean_layout=1" + extra
def ic(name, role, fbs, ors):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": fbs, "oracle": ors, "max_trials": "1000000000", "targets": "stride-generator"}}}
arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
        {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
        ic("ls", "baseline", fb(K), OR),
        ic("ls-aa", "control", fb(K), OR),
        ic("ls-off", "control", fb(K, ",orbit_key_reuse=0"), OR + ",walk_sib=0,ladder_verify=0"),
        ic("sw", "candidate", fb(K, ",orbit_key_reuse=1"), OR + f",walk_sib={M},ladder_verify=1")]
for rep in (1, 2, 3):
    ts = SEED + rep - 1
    s = {"schema": "ecbench.spec/v1",
         "label": f"Sibling walk k=8 m=8 + reuse of formed points vs lean+sib incumbent, K1 a={A} n={N}, replicate {rep}",
         "description": "Frozen in research/koblitz_n37_sibwalk_k8_20261118/PROTOCOL.md before its data. Candidate sw: k=8 (incumbent columns), walk_sib=8, ladder_verify=1, orbit_key_reuse=1; rho strong and v5 references in the same session.",
         "workloads": {"curves": [{"kind": "koblitz", "a": A, "n": N}], "targets_per_curve": 48, "target_seed": ts, "target_kind": "public"},
         "arms": arms,
         "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": ts + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    p = os.path.join(HERE, "specs", f"n37-r{rep}.json"); open(p, "w").write(json.dumps(s, indent=1) + "\n"); print(p)
