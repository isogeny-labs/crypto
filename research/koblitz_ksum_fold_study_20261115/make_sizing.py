#!/usr/bin/env python3
"""Sizing-pilot specs (not study data). Usage: python3 make_sizing.py <outdir>"""
import json, sys, os
out = sys.argv[1]; os.makedirs(out, exist_ok=True)
RX = "100000000"
def ic(name, role, fb, oracle, targets="walk", full=False):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline_full" if full else "ic.pipeline",
        "params": {"factor_base": fb, "oracle": oracle, "max_trials": "1000000000", "targets": targets}}}
def cos(c, g=False): return f"compact-orbit-scan:columns={c},raw_x_cap={RX}" + (",generator_orbit=1" if g else "")
def arms(lazy_cs, kf_cs):
    a = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}}]
    for c in lazy_cs:
        a.append(ic(f"lazy-c{c}", "baseline", cos(c), "mitm-frobenius-lazy-counted:m=2", "stride-renew"))
    for m in (3, 4):
        for c in kf_cs:
            for g in (False, True):
                for t in ("walk", "stride-renew"):
                    if m == 4 and t == "stride-renew": continue
                    a.append(ic(f"kf{m}-c{c}{'g' if g else ''}-{'w' if t=='walk' else 'sr'}", "candidate", cos(c, g), f"ksum-frobenius:m={m}", t))
    return a
for a, n, seed, lazy in ((1, 29, 20261115901, (1, 2)), (1, 23, 20261115903, (1, 2, 3, 4))):
    spec = {"schema": "ecbench.spec/v1", "label": f"SIZING PILOT a={a} n={n}", "description": "sizing pilot, not data",
            "workloads": {"curves": [{"kind": "koblitz", "a": a, "n": n}], "targets_per_curve": 16, "target_seed": seed, "target_kind": "public"},
            "arms": arms(lazy, (1, 2)),
            "measurement": {"rounds": 2, "warmup": 1, "order": "alternate", "seed": seed + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    json.dump(spec, open(f"{out}/sizing-a{a}-n{n}.json", "w"), indent=1)
