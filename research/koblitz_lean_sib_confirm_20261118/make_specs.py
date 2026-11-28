#!/usr/bin/env python3
"""Writes the frozen specs (3 curves x 3 replicate sessions) into specs/. Run from anywhere."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
OR = "mitm-frobenius-layout-counted:m=2"
# incumbent per curve (validated): n23 sp-c4-d5, n29 gen-c1, n37 sp-c8-d5 (VAL-20261118-4ae594, VAL-20261118-43658a)
CURVES = {23: dict(a=1, inc="sp-c4-d5", fb="compact-orbit-scan:columns=4,raw_x_cap=100000000,generator_orbit=1,generator_multiples=4,multiple_doublings=5", seed=20261118431),
          29: dict(a=1, inc="gen-c1", fb="compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1", seed=20261118434),
          37: dict(a=0, inc="sp-c8-d5", fb="compact-orbit-scan:columns=8,raw_x_cap=100000000,generator_orbit=1,generator_multiples=8,multiple_doublings=5", seed=20261118437)}
def ic(name, role, fb, orx=""):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": fb, "oracle": OR + orx, "max_trials": "1000000000", "targets": "stride-generator"}}}
for n, C in CURVES.items():
    inc, fb = C["inc"], C["fb"]
    arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
            {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
            ic(inc, "baseline", fb),
            ic(inc + "-aa", "control", fb),
            ic(inc + "-off", "control", fb + ",lean_layout=0", ",sib=0,tau_verify=0"),
            ic("ls", "candidate", fb + ",lean_layout=1", ",sib=1")]
    for rep in (1, 2, 3):
        ts = C["seed"] + rep - 1
        spec = {"schema": "ecbench.spec/v1", "label": f"Lean base + sibling pair table (no tau check) vs incumbent, K1 a={C['a']} n={n}, replicate {rep}",
                "description": "Frozen in research/koblitz_lean_sib_confirm_20261118/PROTOCOL.md before its data. Candidate ls = incumbent + compact-orbit-scan lean_layout=1 + mitm-frobenius-layout-counted sib=1 (tau_verify off); rho strong and v5 references in the same session.",
                "workloads": {"curves": [{"kind": "koblitz", "a": C["a"], "n": n}], "targets_per_curve": 48, "target_seed": ts, "target_kind": "public"},
                "arms": arms,
                "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": ts + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
        p = os.path.join(OUT, f"n{n}-r{rep}.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
