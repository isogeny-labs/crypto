#!/usr/bin/env python3
"""Writes the frozen specs (one session per curve) into specs/. Run from anywhere."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs"); os.makedirs(OUT, exist_ok=True)
# (a, n) of every Koblitz curve with a leaderboard row below r = 2^38 (research/koblitz_rho_reference_20261115 README section 2)
CURVES = [(0, 13), (1, 17), (0, 19), (0, 23), (1, 23), (1, 29), (0, 31), (0, 37)]
ARMS = [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
        {"name": "rho-strong-aa", "role": "control", "method": {"id": "rho.signed_frobenius_strong"}},
        {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
        {"name": "rho-v5-aa", "role": "control", "method": {"id": "rho.signed_frobenius_v5"}}]
for i, (a, n) in enumerate(CURVES):
    seed = 20261119601 + i
    spec = {"schema": "ecbench.spec/v1", "label": f"Koblitz v5 (jump-index tally) vs strong: a={a} n={n}",
            "description": "Frozen in research/koblitz_v5_tally_remeasure_20261119/PROTOCOL.md before its data. Re-measurement of rho.signed_frobenius_v5 with its per-step jump-index tally against rho.signed_frobenius_strong; reference measurement, no IC change, no security claim.",
            "workloads": {"curves": [{"kind": "koblitz", "a": a, "n": n}], "targets_per_curve": 48, "target_seed": seed, "target_kind": "public"},
            "arms": ARMS,
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": seed + 1000, "isolation_required": "L0", "timeout_seconds": 3600}}
    p = os.path.join(OUT, f"a{a}-n{n}.json"); open(p, "w").write(json.dumps(spec, indent=1) + "\n"); print(p)
