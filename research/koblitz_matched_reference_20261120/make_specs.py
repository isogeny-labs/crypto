#!/usr/bin/env python3
"""EXP-KOBLITZ-REF-002 specs: the audit's curves, targets, seeds and measurement
(research/koblitz_rho_reference_20261115/make_specs.py), minus its two context arms and n41/n53."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
RX = "100000000"
def cos(c): return f"compact-orbit-scan:columns={c},raw_x_cap={RX},generator_orbit=1"
def ic(name, role, fb, oracle, targets, mt="1000000000"):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {"factor_base": fb, "oracle": oracle, "max_trials": mt, "targets": targets}}}
LAYOUT = "mitm-frobenius-layout-counted:m=2"
CURVES = {17: dict(a=1, gen=[1], k1=None, seed=20261115301),
          23: dict(a=1, gen=[3, 4], k1=(9, "13528399621"), seed=20261115302),
          29: dict(a=1, gen=[1], k1=(10, "20000000"), seed=20261115303),
          31: dict(a=0, gen=[2], k1=None, seed=20261115304),
          37: dict(a=0, gen=[8], k1=None, seed=20261115305)}
def arms(C):
    a = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
         {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
         {"name": "rho-v5-aa", "role": "control", "method": {"id": "rho.signed_frobenius_v5"}}]
    for c in C["gen"]:
        a.append(ic(f"gen-c{c}", "baseline", cos(c), LAYOUT, "stride-generator"))
    if C["k1"]:
        d, mt = C["k1"]
        a.append(ic(f"k1-l{d}", "baseline", f"koblitz-standard-subspace:dimension={d}", "pdp3-koblitz:m=3,engine=rootfind-negfold", "walk", mt=mt))
    return a
for n, C in CURVES.items():
    spec = {"schema": "ecbench.spec/v1", "label": f"Koblitz matched rho reference, v5 tallied: a={C['a']} n={n}",
            "description": "Frozen in research/koblitz_matched_reference_20261120/PROTOCOL.md before its data. v5 with its jump-index selection tallied vs strong, on the audit's targets; no IC change, no security claim.",
            "workloads": {"curves": [{"kind": "koblitz", "a": C["a"], "n": n}], "targets_per_curve": 48, "target_seed": C["seed"], "target_kind": "public"},
            "arms": arms(C),
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": C["seed"] + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    p = os.path.join(OUT, f"n{n}.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
