#!/usr/bin/env python3
"""Exact rational k=10 checks on every locally selected CRT candidate."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_full_law_crt_20261120"))
import run as crt


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    candidate_raw = gzip.decompress((PARENT / "xedni_local_selector_20261120/candidate-rows.jsonl.gz").read_bytes())
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    assert sha(candidate_raw) == protocol["local_selector_candidate_rows_raw_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    candidates = [json.loads(x) for x in candidate_raw.splitlines()]
    assert len(source) == 1722 and len(candidates) == 32256
    selected = [row for row in candidates if 10 in row["residues"]]
    assert len(selected) == 1488
    counts = Counter()
    rows = []
    cache = {}
    for candidate in selected:
        index = candidate["pair_index"]
        if index not in cache:
            points = source[index]["lifted_points"]
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            w7, _ = crt.local_gate(p7, 7, 4)
            w11, _ = crt.local_gate(p11, 11, 6)
            cache[index] = (points, w7, w11)
        points, w7, w11 = cache[index]
        i7, i11 = candidate["local_indices"]
        a7, a11 = w7[i7], w11[i11]
        shifts = [crt.centered_k(a7[j], a11[j]) for j in range(3)]
        coeffs = crt.rational_curve(points, shifts)
        a = [QQ(v.numerator)/QQ(v.denominator) for v in coeffs]
        E = EllipticCurve(QQ, a)
        P, Q = [E([QQ(x), QQ(y)]) for x, y in points]
        related = P + 10*Q == E(0)
        counts["rational_checks"] += 1
        counts["rational_relations"] += related
        if related:
            e47 = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
            T, G = [e47([int(x), int(y)]) for x, y in source[index]["reduced_points"]]
            assert T + 10*G == e47(0)
            counts["positive_reduced_postchecks"] += 1
        rows.append({"pair_index": index, "local_indices": [i7, i11],
                     "coefficient": 10, "crt_shifts": shifts,
                     "curve_coefficients": list(map(str, coeffs)),
                     "rational_relation": bool(related)})
    raw = "".join(json.dumps(row, sort_keys=True) + "\n" for row in rows).encode()
    (HERE / "rational-checks.jsonl.gz").write_bytes(gzip.compress(raw, mtime=0))
    return {"schema": "xedni-residue10-rational-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_bytes),
            "script_sha256": sha((HERE / "run_sage.py").read_bytes()),
            "source_packed_sha256": sha(source_bytes),
            "selector_candidate_rows_raw_sha256": sha(candidate_raw),
            "counts": dict(counts),
            "rational_checks_raw_sha256": sha(raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
