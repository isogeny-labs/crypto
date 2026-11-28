#!/usr/bin/env python3
"""Writes the frozen spec (one ecbench session, K1 a=1 n=29) into specs/. Run from anywhere."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
FB = "compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1"
OR = "mitm-frobenius-layout-counted:m=2"
def ic(name, role, fb_extra="", or_extra=""):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": FB + fb_extra, "oracle": OR + or_extra, "max_trials": "1000000000", "targets": "stride-generator"}}}
arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
        {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
        ic("gen-c1", "baseline"),
        ic("gen-c1-aa", "control"),
        ic("gen-c1-off", "control", ",lean_layout=0", ",sib=0,tau_verify=0"),
        ic("lv-all", "candidate", ",lean_layout=1", ",sib=1,tau_verify=1"),
        ic("lv-lean", "candidate", ",lean_layout=1"),
        ic("lv-sib", "candidate", "", ",sib=1"),
        ic("lv-tau", "candidate", "", ",tau_verify=1")]
spec = {"schema": "ecbench.spec/v1", "label": "Generator-orbit IC levers on K1 a=1 n=29: lean base, sibling pair table, tau-adic check",
        "description": "Frozen in research/koblitz_n29_orbit_levers_20261118/PROTOCOL.md before its data. Levers (default off): compact-orbit-scan lean_layout=1, mitm-frobenius-layout-counted sib=1 and tau_verify=1, against the validated generator-orbit arm gen-c1; rho strong and v5 references.",
        "workloads": {"curves": [{"kind": "koblitz", "a": 1, "n": 29}], "targets_per_curve": 48, "target_seed": 20261118404, "target_kind": "public"},
        "arms": arms,
        "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": 20261119404, "isolation_required": "L0", "timeout_seconds": 3600}}
p = os.path.join(OUT, "n29.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
