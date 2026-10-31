#!/usr/bin/env python3
import csv
import hashlib
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parent
INPUT = ROOT / "frozen-inputs.json"
RESULTS = ROOT / "results"
EXPECTED_INPUT_SHA256 = "b57fbb4c3be313842aa2d3024d1b3d221e8b380caf8fbb4c219c4222f55fe41a"
S_RATIOS = (0.0, 0.5, 1.0)
I_RATIOS = (1.0, 10.0, 100.0, 1000.0)
NATIVE_RATIOS = (0.0, 0.1, 1.0)


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    assert digest(INPUT) == EXPECTED_INPUT_SHA256
    frozen = json.loads(INPUT.read_text())
    assert frozen["schema"] == "kummer-fixed-offset-orbit-input/v1"
    RESULTS.mkdir(exist_ok=True)
    rows = []
    grid = []
    for model in frozen["models"]:
        bits, p, r = model["bits"], model["p"], model["r"]
        coordinate_bytes = 8  # current Rust state is [u64; 4]
        for jump in model["jumps"]:
            j = jump["jump_scalar"]
            orbit = r // math.gcd(r, j)
            row = {
                "schema": "kummer-fixed-offset-orbit-bound/v1",
                "bits": bits,
                "p": p,
                "r": r,
                "jump_index": jump["jump_index"],
                "jump_scalar": j,
                "gcd_r_jump": math.gcd(r, j),
                "orbit_size": orbit,
                "minimum_offset_labels": orbit,
                "current_coordinate_bytes": coordinate_bytes,
                "current_class_bytes": 4 * coordinate_bytes,
                "minimum_explicit_state_bytes_current_repr": orbit * 4 * coordinate_bytes,
                "minimum_regular_xadd_calls_per_full_state_update": orbit,
                "minimum_field_multiplications_current_cached_xadd": 24 * orbit,
                "minimum_field_additions_current_cached_xadd": 36 * orbit,
                "representative_lower_bound_gae": 24 * orbit,
                "bound_scope": "explicit_fixed_offset_one_xadd_per_entry_current_u64x4_representation",
            }
            rows.append(row)
            for s in S_RATIOS:
                for inv in I_RATIOS:
                    for native in NATIVE_RATIOS:
                        grid.append({
                            "bits": bits, "p": p, "r": r,
                            "jump_index": jump["jump_index"], "jump_scalar": j,
                            "S_per_M": s, "I_per_M": inv,
                            "native_GAE_per_logical": native,
                            "lower_bound_gae": 24 * orbit,
                            "note": "24M per regular cached xADD; additions reported but unpriced; no inversion or native charge",
                        })
    assert len(rows) == 336 and len(grid) == 336 * 36
    with (RESULTS / "bounds.csv").open("w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=rows[0].keys())
        w.writeheader(); w.writerows(rows)
    with (RESULTS / "price-grid.csv").open("w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=grid[0].keys())
        w.writeheader(); w.writerows(grid)
    summary = {
        "schema": "kummer-fixed-offset-orbit-summary/v1",
        "study_id": frozen["study_id"],
        "models": 21,
        "jumps_per_model": 16,
        "bound_rows": len(rows),
        "price_rows": len(grid),
        "prime_order_rows": sum(1 for x in rows if x["orbit_size"] == x["r"]),
        "minimum_labels_min": min(x["minimum_offset_labels"] for x in rows),
        "minimum_labels_max": max(x["minimum_offset_labels"] for x in rows),
        "explicit_state_bytes_min": min(x["minimum_explicit_state_bytes_current_repr"] for x in rows),
        "explicit_state_bytes_max": max(x["minimum_explicit_state_bytes_current_repr"] for x in rows),
        "representative_lower_bound_gae_min": min(x["representative_lower_bound_gae"] for x in rows),
        "representative_lower_bound_gae_max": max(x["representative_lower_bound_gae"] for x in rows),
        "input_sha256": digest(INPUT),
        "status": "proved_for_explicit_fixed_offset_state_model",
        "arbitrary_sidecar_status": "open",
    }
    (RESULTS / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")


if __name__ == "__main__":
    main()
