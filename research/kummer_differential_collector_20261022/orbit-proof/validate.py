#!/usr/bin/env python3
import csv
import hashlib
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parent
S_RATIOS = (0.0, 0.5, 1.0)
I_RATIOS = (1.0, 10.0, 100.0, 1000.0)
NATIVE_RATIOS = (0.0, 0.1, 1.0)


def prime(n: int) -> bool:
    if n < 2 or n % 2 == 0:
        return n == 2
    d = 3
    while d * d <= n:
        if n % d == 0:
            return False
        d += 2
    return True


def main() -> None:
    frozen = json.loads((ROOT / "frozen-inputs.json").read_text())
    rows = list(csv.DictReader((ROOT / "results/bounds.csv").open()))
    grid = list(csv.DictReader((ROOT / "results/price-grid.csv").open()))
    assert len(frozen["models"]) == 21 and len(rows) == 336 and len(grid) == 12096
    expected = {(m["p"], j["jump_index"]): (m, j) for m in frozen["models"] for j in m["jumps"]}
    assert len(expected) == 336
    for row in rows:
        p, idx = int(row["p"]), int(row["jump_index"])
        model, jump = expected.pop((p, idx))
        r, j = int(row["r"]), int(row["jump_scalar"])
        assert r == model["r"] and j == jump["jump_scalar"]
        assert prime(r) and 0 < j < r and math.gcd(r, j) == 1
        orbit = r // math.gcd(r, j)
        assert int(row["orbit_size"]) == orbit == r
        assert int(row["minimum_offset_labels"]) == orbit
        assert int(row["minimum_explicit_state_bytes_current_repr"]) == 32 * orbit
        assert int(row["minimum_regular_xadd_calls_per_full_state_update"]) == orbit
        assert int(row["minimum_field_multiplications_current_cached_xadd"]) == 24 * orbit
        assert int(row["minimum_field_additions_current_cached_xadd"]) == 36 * orbit
        assert int(row["representative_lower_bound_gae"]) == 24 * orbit
    assert not expected
    by_key = {}
    expected_cells = {
        (str(s), str(inv), str(native))
        for s in S_RATIOS
        for inv in I_RATIOS
        for native in NATIVE_RATIOS
    }
    for row in grid:
        key = (int(row["p"]), int(row["jump_index"]))
        by_key.setdefault(key, []).append(row)
        assert int(row["lower_bound_gae"]) == 24 * int(row["r"])
    assert len(by_key) == 336
    for cell_rows in by_key.values():
        cells = {
            (row["S_per_M"], row["I_per_M"], row["native_GAE_per_logical"])
            for row in cell_rows
        }
        assert len(cell_rows) == 36 and cells == expected_cells
    summary = json.loads((ROOT / "results/summary.json").read_text())
    assert summary["bound_rows"] == 336 and summary["price_rows"] == 12096
    receipt = {
        "schema": "kummer-fixed-offset-orbit-validation/v1",
        "input_sha256": hashlib.sha256((ROOT / "frozen-inputs.json").read_bytes()).hexdigest(),
        "bounds_sha256": hashlib.sha256((ROOT / "results/bounds.csv").read_bytes()).hexdigest(),
        "price_grid_sha256": hashlib.sha256((ROOT / "results/price-grid.csv").read_bytes()).hexdigest(),
        "models_checked": 21, "rows_checked": 336, "price_rows_checked": 12096,
        "prime_checks": 21, "gcd_checks": 336, "failures": 0,
    }
    (ROOT / "results/validation.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt, sort_keys=True))


if __name__ == "__main__":
    main()
