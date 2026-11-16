#!/usr/bin/env python3
"""Add the frozen two-choice arm to each unchanged exact-order workload."""

import copy
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = ROOT / "research/three_leads_e2e_20261018/genus-order-fixtures-v1"
OUTPUT = Path(__file__).resolve().parent / "specs/genus2-choice2-v1"


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for bits in (16, 20, 24, 28, 32, 36, 40):
        source = ORIGINAL / f"order-genus-{bits}.json"
        spec = json.loads(source.read_text())
        incumbent = next(arm for arm in spec["arms"] if arm["name"] == "genus_ic")
        assert incumbent["method"]["id"] == "ic.three_leads_genus2_v2"
        assert spec["workloads"]["targets_per_curve"] == 60
        assert len(spec["workloads"]["curves"]) == 3
        choice = copy.deepcopy(incumbent)
        choice["name"] = "genus_choice2"
        choice["method"] = {
            "id": "ic.three_leads_genus2_choice2_v1",
            "params": {"trial_cap": incumbent["method"]["params"]["trial_cap"]},
        }
        spec["arms"].append(choice)
        spec["label"] = f"five-leads-choice2-order-genus-{bits}"
        spec["description"] = (
            "Frozen exact-order targets; incumbent v2, two-choice candidate, "
            "source rho controls; complete cold operation counts; L0 walls unqualified"
        )
        (OUTPUT / f"order-genus-{bits}.json").write_text(
            json.dumps(spec, indent=1, sort_keys=True) + "\n"
        )


if __name__ == "__main__":
    main()
