#!/usr/bin/env python3
"""Writes the frozen specs (one ecbench session per curve) into specs/. Run from anywhere."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
RX = "100000000"
LAYOUT = "mitm-frobenius-layout-counted:m=2"
def fb(c, k=None):
    s = f"compact-orbit-scan:columns={c},raw_x_cap={RX},generator_orbit=1"
    return s if k is None else s + f",generator_multiples={k}"
def ic(name, role, factor_base):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
        "factor_base": factor_base, "oracle": LAYOUT, "max_trials": "1000000000", "targets": "stride-generator"}}}
# (a, n): incumbent columns c0 (validated gen-orbit layout; n23 c4 is the validator's pre-declared test),
# primary known-log k*, secondary known-log k, extra incumbent columns
CURVES = {23: dict(a=1, c0=4, kp=3, ks=[4], extra=[3], seed=20261118401),
          29: dict(a=1, c0=1, kp=None, ks=[2], extra=[], seed=20261118402),
          37: dict(a=0, c0=8, kp=7, ks=[6, 8], extra=[], seed=20261118403)}
def arms(C):
    c0 = C["c0"]
    a = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
         {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
         ic(f"gen-c{c0}", "baseline", fb(c0)),
         ic(f"gen-c{c0}-aa", "control", fb(c0)),
         ic(f"gen-c{c0}-m1", "control", fb(c0, 1))]
    for c in C["extra"]:
        a.append(ic(f"gen-c{c}", "baseline", fb(c)))
    if C["kp"]:
        a.append(ic(f"kl-c{C['kp']}", "candidate", fb(C["kp"], C["kp"])))
    for k in C["ks"]:
        a.append(ic(f"kl-c{k}", "candidate", fb(k, k)))
    return a
for n, C in CURVES.items():
    spec = {"schema": "ecbench.spec/v1", "label": f"Koblitz known-log generator-multiple columns: a={C['a']} n={n}",
            "description": "Frozen in research/koblitz_known_log_columns_20261118/PROTOCOL.md before its data. Lever: compact-orbit-scan generator_multiples=k (columns seeded by the orbits of [1]G..[k]G) against the validated generator-orbit layout arm; rho strong and v5 references.",
            "workloads": {"curves": [{"kind": "koblitz", "a": C["a"], "n": n}], "targets_per_curve": 48, "target_seed": C["seed"], "target_kind": "public"},
            "arms": arms(C),
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": C["seed"] + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    p = os.path.join(OUT, f"n{n}.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
