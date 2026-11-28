#!/usr/bin/env python3
"""Addendum A1 (PROTOCOL.md): frozen specs for the spread known-log seeds (multiple_doublings) into a1/specs/."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
def ic(name, role, c, k=None, d=None):
    fb = f"compact-orbit-scan:columns={c},raw_x_cap=100000000,generator_orbit=1"
    if k: fb += f",generator_multiples={k}"
    if d: fb += f",multiple_doublings={d}"
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {"factor_base": fb, "oracle": "mitm-frobenius-layout-counted:m=2", "max_trials": "1000000000", "targets": "stride-generator"}}}
CURVES = {23: dict(a=1, c0=4, seed=20261118411), 37: dict(a=0, c0=8, seed=20261118413)}
for n, C in CURVES.items():
    c0 = C["c0"]
    arms = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
            {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
            ic(f"gen-c{c0}", "baseline", c0), ic(f"gen-c{c0}-aa", "control", c0),
            ic(f"kl-c{c0}", "baseline", c0, c0),
            ic(f"sp-c{c0}-d5", "candidate", c0, c0, 5),
            ic(f"sp-c{c0}-d3", "candidate", c0, c0, 3),
            ic(f"sp-c{c0 + 1}-d5", "candidate", c0 + 1, c0 + 1, 5)]
    spec = {"schema": "ecbench.spec/v1", "label": f"Koblitz known-log columns, addendum A1 (spread seeds): a={C['a']} n={n}",
            "description": "Frozen in research/koblitz_known_log_columns_20261118/PROTOCOL.md addendum A1 before its data. multiple_doublings=d: known seeds [2^(d(j-1))]G.",
            "workloads": {"curves": [{"kind": "koblitz", "a": C["a"], "n": n}], "targets_per_curve": 48, "target_seed": C["seed"], "target_kind": "public"},
            "arms": arms,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": C["seed"] + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    p = os.path.join(OUT, f"n{n}.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
