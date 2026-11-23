#!/usr/bin/env python3
"""Compare matched auto and forced-generic reducer benchmark CSVs."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


def rows(path: Path) -> dict[int, dict[str, str]]:
    with path.open(newline="") as handle:
        return {int(row["degree"]): row for row in csv.DictReader(handle)}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("results", type=Path, nargs="?", default=Path("results"))
    args = parser.parse_args()
    output = args.results / "fixed_field_comparison.csv"
    columns = (
        "p", "degree", "cases", "repeats", "auto_mitm_ns", "generic_mitm_ns",
        "mitm_speedup", "auto_factor_ns", "generic_factor_ns", "factor_speedup",
    )
    with output.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=columns)
        writer.writeheader()
        for auto_path in sorted(args.results.glob("fixed_field_auto_p*.csv")):
            p = int(auto_path.stem.removeprefix("fixed_field_auto_p"))
            generic_path = args.results / f"fixed_field_generic_p{p}.csv"
            auto, generic = rows(auto_path), rows(generic_path)
            if auto.keys() != generic.keys():
                raise SystemExit(f"degree mismatch for p={p}")
            for degree in sorted(auto):
                a, g = auto[degree], generic[degree]
                if (a["cases"], a["structure_repeats"]) != (g["cases"], g["structure_repeats"]):
                    raise SystemExit(f"workload mismatch for p={p}, degree={degree}")
                am, gm = float(a["mitm_batch_ns"]), float(g["mitm_batch_ns"])
                af, gf = float(a["generic_ns"]), float(g["generic_ns"])
                writer.writerow({
                    "p": p,
                    "degree": degree,
                    "cases": a["cases"],
                    "repeats": a["structure_repeats"],
                    "auto_mitm_ns": f"{am:.3f}",
                    "generic_mitm_ns": f"{gm:.3f}",
                    "mitm_speedup": f"{gm / am:.6f}",
                    "auto_factor_ns": f"{af:.3f}",
                    "generic_factor_ns": f"{gf:.3f}",
                    "factor_speedup": f"{gf / af:.6f}",
                })
    print(output)


if __name__ == "__main__":
    main()
