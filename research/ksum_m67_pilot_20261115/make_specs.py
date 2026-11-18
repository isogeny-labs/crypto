#!/usr/bin/env python3
"""Pilot specs for ksum m = 6, 7 (sizing pilot, not data).

Curves are copied from the frozen study specs; targets use fresh seeds.
  python3 make_specs.py   # writes specs/B27.json and specs/P24.json
"""
import json, pathlib

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[1]
STUDIES = {
    "B27": ("research/ic_tableb_binary_ksum_20261115/specs/B27-r1.json", "binary-multiplier:size={n},mult=1",
            [(5, 12), (6, 10), (6, 11), (7, 9), (7, 10), (7, 11)], 2026111567),
    "P24": ("research/ic_tableb_ksum_20261115/specs/P24-r1.json", "prime-abscissa:size={n}",
            [(5, 11), (6, 9), (6, 10), (6, 11), (7, 9), (7, 10)], 2026111568),
}
for row, (src, base, arms, seed) in STUDIES.items():
    study = json.loads((ROOT / src).read_text())
    spec = {
        "schema": "ecbench.spec/v1",
        "label": f"ksum_m67_pilot_20261115 {row}",
        "description": "Sizing pilot, not data: ksum m = 6, 7 against the frozen ksum5 size. Public synthetic curve and targets; no security claim.",
        "workloads": {
            "curves": study["workloads"]["curves"],
            "targets_per_curve": 32,
            "target_seed": seed,
            "target_kind": "public",
        },
        "arms": [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]
        + [
            {
                "name": f"ksum{m}-n{n}",
                "role": "baseline" if m == 5 else "candidate",
                "method": {"id": "ic.pipeline", "params": {"factor_base": base.format(n=n), "oracle": f"ksum:m={m}", "max_trials": "13528399621"}},
            }
            for m, n in arms
        ],
        "measurement": {"rounds": 1, "warmup": 0, "order": "alternate", "seed": seed + 1, "isolation_required": "L0", "timeout_seconds": 1800},
    }
    (HERE / "specs" / f"{row}.json").write_text(json.dumps(spec, indent=1) + "\n")
