#!/usr/bin/env python3
"""Writes the six frozen study specs (3 sessions per curve) into specs/.
Usage: python3 make_specs.py   (run from anywhere; writes next to this file)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
RX = "100000000"
def cos(c, g=False): return f"compact-orbit-scan:columns={c},raw_x_cap={RX}" + (",generator_orbit=1" if g else "")
def ic(name, role, fb, oracle, targets="walk", full=False, mt="1000000000"):
    return {"name": name, "role": role, "method": {"id": "ic.pipeline_full" if full else "ic.pipeline",
            "params": {"factor_base": fb, "oracle": oracle, "max_trials": mt, "targets": targets}}}
LAZY = "mitm-frobenius-lazy-counted:m=2"; KF = "ksum-frobenius:m=3"
# curve: (a, n, lazy columns (frozen from sizing), K1 dimension, K1 max_trials, target seeds)
CURVES = {23: dict(a=1, lazy=(3, 4), k1=9, k1mt="13528399621", seeds=(20261115101, 20261115102, 20261115103), c_primary=4),
          29: dict(a=1, lazy=(1, 2), k1=10, k1mt="20000000", seeds=(20261115201, 20261115202, 20261115203), c_primary=1)}
def arms(n):
    C = CURVES[n]; cp = C["c_primary"]
    a = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
         {"name": "rho-strong-aa", "role": "control", "method": {"id": "rho.signed_frobenius_strong"}},
         {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]
    for c in C["lazy"]:
        a.append(ic(f"lazy-c{c}", "baseline", cos(c), LAZY, "stride-renew"))
    a.append(ic(f"k1-l{C['k1']}", "baseline", f"koblitz-standard-subspace:dimension={C['k1']}",
                "pdp3-koblitz:m=3,engine=rootfind-negfold", "walk", mt=C["k1mt"]))
    a.append(ic("kf3-c1g-sr", "candidate", cos(1, True), KF, "stride-renew"))
    a.append(ic("kf3-c1g-w", "candidate", cos(1, True), KF, "walk"))
    # secondary: the lazy studies' own pricing (*_full) for the primary pair
    a.append({"name": "rho-strong-full", "role": "reference", "method": {"id": "rho.signed_frobenius_strong_full"}})
    a.append(ic(f"lazy-c{cp}-full", "baseline", cos(cp), LAZY, "stride-renew", full=True))
    a.append(ic("kf3-c1g-sr-full", "candidate", cos(1, True), KF, "stride-renew", full=True))
    return a
for n, C in CURVES.items():
    for i, seed in enumerate(C["seeds"], 1):
        spec = {"schema": "ecbench.spec/v1", "label": f"Frobenius-folded k-sum vs lazy orbit-member IC: Koblitz a={C['a']} n={n} replicate {i}",
                "description": "Frozen in research/koblitz_ksum_fold_study_20261115/PROTOCOL.md before its data. Same factor base and targets strategy per method as stated there; engineering comparison, no security claim.",
                "workloads": {"curves": [{"kind": "koblitz", "a": C["a"], "n": n}], "targets_per_curve": 48, "target_seed": seed, "target_kind": "public"},
                "arms": arms(n),
                "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": seed + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
        p = os.path.join(OUT, f"n{n}-r{i}.json"); json.dump(spec, open(p, "w"), indent=1, sort_keys=False); open(p, "a").write("\n")
        print(p)
