#!/usr/bin/env python3
"""POST HOC supplement (not frozen before the main data): the reference the validated gen-layout section used,
rho.signed_frobenius_full ("rho-frob", the tuned walk), beside strong and v5 on the main sessions' targets (same seeds)."""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "specs_supp"); os.makedirs(OUT, exist_ok=True)
for n, a, seed in [(17, 1, 20261115301), (23, 1, 20261115302), (29, 1, 20261115303), (31, 0, 20261115304), (37, 0, 20261115305)]:
    spec = {"schema": "ecbench.spec/v1", "label": f"POST HOC supplement: tuned rho-frob vs strong vs v5, Koblitz a={a} n={n}",
            "description": "Post hoc, written after the main sessions; see research/koblitz_rho_reference_20261115/README.md. No IC arm.",
            "workloads": {"curves": [{"kind": "koblitz", "a": a, "n": n}], "targets_per_curve": 48, "target_seed": seed, "target_kind": "public"},
            "arms": [{"name": "rho-strong", "role": "reference", "method": {"id": "rho.signed_frobenius_strong"}},
                     {"name": "rho-v5", "role": "reference", "method": {"id": "rho.signed_frobenius_v5"}},
                     {"name": "rho-frob", "role": "reference", "method": {"id": "rho.signed_frobenius_full"}},
                     {"name": "rho-frob-plain", "role": "reference", "method": {"id": "rho.signed_frobenius"}},
                     {"name": "rho-frob-aa", "role": "control", "method": {"id": "rho.signed_frobenius_full"}}],
            "measurement": {"rounds": 3, "warmup": 1, "order": "alternate", "seed": seed + 2000, "isolation_required": "L0", "timeout_seconds": 3600}}
    open(os.path.join(OUT, f"n{n}.json"), "w").write(json.dumps(spec, indent=1) + "\n")
