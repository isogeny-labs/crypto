#!/usr/bin/env python3
"""Select a fixed confirmation set from an independently replayed grid."""

import argparse
import json
import math
from pathlib import Path

from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_nearby_grid_tier1_20261024 import verify as replay

TOP_PER_METRIC = 3


def choose(analysis):
    admitted = (analysis.get("status") == "PASS_TIER1_GRID_SCREEN" and
                analysis.get("source_verified") is True and
                analysis.get("host_verified") is True and
                analysis.get("noise_gate_passed") is True and
                analysis.get("point_index") == 0)
    result = {
        "schema": "n83.nearby-grid-confirm16-selection/v1",
        "screen_workload_id": screen.WORKLOAD_ID,
        "screen_catalog_sha256": screen.CATALOG_SHA256,
        "selection_rule": "top3_paired_online_union_top3_paired_cold_plus_k1107",
        "status": "BOUNDED_UNKNOWN", "selected_ks": None,
        "selected_candidate_ids": None,
    }
    if not admitted:
        return result
    ks = set(screen.CANDIDATES)
    metrics = []
    for field in ("median_paired_online_ratio_to_baseline",
                  "median_paired_cold_ratio_to_baseline"):
        values = analysis.get(field)
        if not isinstance(values, dict) or set(values) != {str(k) for k in ks}:
            raise ValueError(f"{field} lacks exact candidate coverage")
        if any(not isinstance(value, (int, float)) or
               isinstance(value, bool) or not math.isfinite(value) or
               value <= 0 for value in values.values()):
            raise ValueError(f"{field} has an invalid ratio")
        metrics.append(sorted(ks, key=lambda k: (values[str(k)], k))[:TOP_PER_METRIC])
    selected = sorted({screen.BASELINE_K, *metrics[0], *metrics[1]})
    result.update({
        "status": "PASS_FROZEN_SCREEN_SELECTION",
        "selected_ks": selected,
        "selected_candidate_ids": {str(k): screen.CANDIDATES[k][0]
                                   for k in selected},
        "top_online_ks": metrics[0], "top_cold_ks": metrics[1],
    })
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("raw_screen_output", type=Path)
    args = parser.parse_args()
    analysis = replay.verify(args.raw_screen_output)
    print(json.dumps(choose(analysis), sort_keys=True))
    return 0 if analysis["status"] == "PASS_TIER1_GRID_SCREEN" else 2


if __name__ == "__main__":
    raise SystemExit(main())
