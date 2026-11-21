#!/usr/bin/env python3
"""Independent Sage point-group replay of bounded coefficient local gates."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_full_law_crt_20261120"))
import run as crt


def on_curve_relation(points, coefficients, p, k):
    curve = EllipticCurve(GF(p), list(coefficients))
    first, second = [curve([int(x), int(y)]) for x, y in points]
    return first + k * second == curve(0)


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    raw = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == protocol["source_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(raw).splitlines()]
    by_k = {str(k): Counter() for k in protocol["coefficients"]}
    counts = Counter()
    for old in source:
        reduced = old["reduced_points"]
        points = old["lifted_points"]
        compatible = {}
        for k in protocol["coefficients"]:
            compatible[k] = on_curve_relation(reduced, [0, 0, 0, 46, 45], 47, k)
            by_k[str(k)]["reduced_compatible_pairs"] += compatible[k]
            counts["reduced_coefficient_checks"] += 1
        (x1, _), (x2, _) = points
        if x1 == x2 or (x1-x2)%7 == 0 or (x1-x2)%11 == 0:
            continue
        p7 = tuple(tuple(v%7 for v in P) for P in points)
        p11 = tuple(tuple(v%11 for v in P) for P in points)
        if crt.point_rank(p7, 7) < 2 or crt.point_rank(p11, 11) < 2:
            continue
        w7, _ = crt.local_gate(p7, 7, 4)
        w11, _ = crt.local_gate(p11, 11, 6)
        for k in protocol["coefficients"]:
            if not compatible[k]:
                continue
            row = by_k[str(k)]
            row["compatible_crt_candidates"] += len(w7)*len(w11)
            for a7 in w7:
                counts["local_checks_7"] += len(w11)
                if not on_curve_relation(p7, a7, 7, k):
                    continue
                row["local_7_pass"] += len(w11)
                for a11 in w11:
                    counts["local_checks_11"] += 1
                    if on_curve_relation(p11, a11, 11, k):
                        row["local_11_pass"] += 1
    for k, saved in result["by_coefficient"].items():
        for key in ("reduced_compatible_pairs", "compatible_crt_candidates",
                    "local_7_pass", "local_11_pass"):
            assert by_k[k][key] == saved.get(key, 0), (k, key)
    for key in ("reduced_coefficient_checks", "local_checks_7", "local_checks_11"):
        assert counts[key] == result["counts"][key], key
    return {"status": "PASS", "checks": dict(counts),
            "by_coefficient": {k: dict(v) for k, v in by_k.items()}}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
