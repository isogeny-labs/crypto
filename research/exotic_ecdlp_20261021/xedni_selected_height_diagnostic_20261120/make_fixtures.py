#!/usr/bin/env python3
"""Freeze the three accepted rational curves from the prior 20-pair census."""

import gzip
import hashlib
import json
from collections import defaultdict
from pathlib import Path


SOURCE = Path("/Volumes/SSD990/cryptanalysis/experiments/xedni-varying-curve-selection-20261115/rows-v2.jsonl.gz")
SOURCE_SHA256 = "9a594d96f369fb14f1a4cf6270b1c811ceda515e1c2a7eccab4d011ca8f3f322"


def main() -> None:
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SOURCE_SHA256
    groups = defaultdict(lambda: {"pairs": [], "points": set()})
    rows = 0
    with gzip.open(SOURCE, "rt") as stream:
        for line in stream:
            row = json.loads(line)
            rows += 1
            if not row["accepted"]:
                continue
            key = (row["A"], row["B"])
            points = tuple(tuple(str(v) for v in p) for p in row["lifted_points"])
            groups[key]["pairs"].append({
                "points": points,
                "reduced_relation": row["reduced_relation"],
                "rational_relation": row["rational_relation"],
            })
            groups[key]["points"].update(points)
    assert len(groups) == 3 and sum(len(v["pairs"]) for v in groups.values()) == 20
    curves = []
    for index, ((A, B), data) in enumerate(sorted(groups.items())):
        assert len(data["points"]) == 4
        curves.append({
            "id": f"C{index}", "weierstrass_coefficients": ["0", "0", "0", A, B],
            "points": sorted(data["points"]), "pairs": data["pairs"],
        })
    print(json.dumps({
        "schema": "xedni-selected-height-fixtures-v1",
        "source_path": str(SOURCE), "source_sha256": SOURCE_SHA256,
        "source_rows": rows, "accepted_pairs": 20, "curves": curves,
    }, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
