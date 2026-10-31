#!/usr/bin/env python3
"""Freeze the original 1,260 exact-order workloads with the target-rank arm."""

import json
from pathlib import Path


BASE = Path(__file__).resolve().parents[1] / "five_leads_e2e_20261019" / "specs" / "genus2-projective-chart-v1"
OUT = Path(__file__).resolve().parent / "specs"
for bits in (16, 20, 24, 28, 32, 36, 40):
    old = json.loads((BASE / f"order-genus-{bits}.json").read_text())
    assert len(old["workloads"]["curves"]) == 3
    assert old["workloads"]["targets_per_curve"] == 60
    assert old["measurement"]["rounds"] == 1
    rank = {
        "name": "genus_target_rank",
        "role": "candidate",
        "method": {
            "id": "ic.three_leads_genus2_target_rank_v1",
            "params": {"trial_cap": old["arms"][2]["method"]["params"]["trial_cap"]},
        },
    }
    old["arms"] = [rank]
    old["label"] = f"target-rank-projective-order-genus-{bits}"
    old["description"] = "Original 1,260-target exact-order cohort; target-coefficient rank continuation; unchanged projective backend; all candidate work charged; L0 operation counts"
    OUT.mkdir(exist_ok=True)
    (OUT / f"order-genus-{bits}.json").write_text(json.dumps(old, indent=2, sort_keys=True) + "\n")
