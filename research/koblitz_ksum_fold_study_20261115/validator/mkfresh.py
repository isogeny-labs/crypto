#!/usr/bin/env python3
"""Validator fresh-session specs for research/koblitz_ksum_fold_study_20261115 (K1 n23).
Seeds from the validator nonce (nonce.txt). Written by the validator, not the producer."""
import json, sys, pathlib
nonce = sys.argv[1]; out = pathlib.Path(sys.argv[2]); out.mkdir(parents=True, exist_ok=True)
seeds = [int(nonce[8*i:8*i+12], 16) % 10**12 for i in range(3)]
CAP = "raw_x_cap=100000000"
def ic(name, role, fb, oracle, targets):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": f"compact-orbit-scan:{fb}", "oracle": oracle,
        "max_trials": "1000000000", "targets": targets}}}
def rho(name, role, mid):
    return {"name": name, "role": role, "method": {"id": mid}}
arms = [
    rho("rho-strong", "reference", "rho.signed_frobenius_strong"),
    rho("rho-strong-aa", "control", "rho.signed_frobenius_strong"),
    rho("rho-strong-v2", "reference", "rho.signed_frobenius_strong_v2"),
    rho("rho-strong-escape", "reference", "rho.signed_frobenius_strong_escape"),
    rho("rho-frob", "reference", "rho.signed_frobenius"),
    rho("rho-frob-v2", "reference", "rho.signed_frobenius_v2"),
    rho("rho-frob-v3", "reference", "rho.signed_frobenius_v3"),
    rho("rho-frob-v4", "reference", "rho.signed_frobenius_v4"),
    rho("rho-frob-v5", "reference", "rho.signed_frobenius_v5"),
    # the registered incumbents, exactly as frozen
    ic("lazy-c3", "baseline", f"columns=3,{CAP}", "mitm-frobenius-lazy-counted:m=2", "stride-renew"),
    ic("lazy-c4", "baseline", f"columns=4,{CAP}", "mitm-frobenius-lazy-counted:m=2", "stride-renew"),
    ic("lazy-c5", "baseline", f"columns=5,{CAP}", "mitm-frobenius-lazy-counted:m=2", "stride-renew"),
    # the existing orbit-layout table (2026-10-22) without and with the generator orbit
    ic("layout-c4-sr", "baseline", f"columns=4,{CAP}", "mitm-frobenius-layout-counted:m=2", "stride-renew"),
    ic("gen-layout-c2", "baseline", f"columns=2,{CAP},generator_orbit=1", "mitm-frobenius-layout-counted:m=2", "stride-generator"),
    ic("gen-layout-c3", "baseline", f"columns=3,{CAP},generator_orbit=1", "mitm-frobenius-layout-counted:m=2", "stride-generator"),
    ic("gen-layout-c4", "baseline", f"columns=4,{CAP},generator_orbit=1", "mitm-frobenius-layout-counted:m=2", "stride-generator"),
    ic("gen-layout-c5", "baseline", f"columns=5,{CAP},generator_orbit=1", "mitm-frobenius-layout-counted:m=2", "stride-generator"),
    ic("gen-layout-c4-sr", "baseline", f"columns=4,{CAP},generator_orbit=1", "mitm-frobenius-layout-counted:m=2", "stride-renew"),
    ic("gen-lazy-c4-sr", "baseline", f"columns=4,{CAP},generator_orbit=1", "mitm-frobenius-lazy-counted:m=2", "stride-renew"),
    # the registered candidate, exactly as frozen, and its stride-generator variant (exploratory)
    ic("kf3-c1g-sr", "candidate", f"columns=1,{CAP},generator_orbit=1", "ksum-frobenius:m=3", "stride-renew"),
    ic("kf3-c1g-sg", "candidate", f"columns=1,{CAP},generator_orbit=1", "ksum-frobenius:m=3", "stride-generator"),
]
for i, ts in enumerate(seeds, 1):
    spec = {"schema": "ecbench.spec/v1",
            "label": f"Validator fresh session F{i}: Koblitz a=1 n=23, koblitz ksum fold study (VAL), nonce-derived seed",
            "description": "Validator re-measurement on new target seeds drawn from the validator nonce after the study's results were published. Registered candidate and incumbents unchanged; extra rho and IC arms to check reference strength and incumbent fairness. Engineering; no security claim.",
            "workloads": {"curves": [{"kind": "koblitz", "a": 1, "n": 23}], "targets_per_curve": 48,
                          "target_seed": ts, "target_kind": "public"},
            "arms": arms,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": ts + 1000,
                            "isolation_required": "L0", "timeout_seconds": 3600}}
    (out / f"F{i}.spec.json").write_text(json.dumps(spec, indent=1) + "\n")
    print(f"F{i}", ts)
