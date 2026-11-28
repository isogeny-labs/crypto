#!/usr/bin/env python3
"""Sage k=14 relation screen on the frozen full-law CRT policy."""

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


def point_relation(points, coeffs, field, k):
    E = EllipticCurve(field, list(coeffs))
    P, Q = [E([field(x), field(y)]) for x, y in points]
    return P + k * Q == E(0)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    pair_bytes = gzip.decompress((PARENT / "xedni_full_law_crt_20261120/pair-rows.jsonl.gz").read_bytes())
    assert hashlib.sha256(source_bytes).hexdigest() == protocol["source_packed_sha256"]
    assert hashlib.sha256(pair_bytes).hexdigest() == protocol["full_law_pair_rows_raw_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    pairs = [json.loads(x) for x in pair_bytes.splitlines()]
    assert len(source) == len(pairs) == 1722
    k = protocol["coefficient"]
    counts = Counter()
    checked = []
    for index, (old, prior) in enumerate(zip(source, pairs)):
        assert prior["pair_index"] == index
        counts["reduced_checks"] += 1
        reduced = old["reduced_points"]
        if not point_relation(reduced, [0, 0, 0, 46, 45], GF(47), k):
            continue
        counts["reduced_compatible_pairs"] += 1
        points = old["lifted_points"]
        if prior["status"] != "constructed":
            counts["compatible_skipped_pairs"] += 1
            continue
        (x1, _), (x2, _) = points
        assert x1 != x2 and (x1-x2)%7 and (x1-x2)%11
        p7 = tuple(tuple(v%7 for v in P) for P in points)
        p11 = tuple(tuple(v%11 for v in P) for P in points)
        w7, _ = crt.local_gate(p7, 7, 4)
        w11, _ = crt.local_gate(p11, 11, 6)
        assert len(w7)*len(w11) == prior["crt_candidates"]
        for i7, a7 in enumerate(w7):
            for i11, a11 in enumerate(w11):
                counts["compatible_crt_candidates"] += 1
                counts["local_checks_7"] += 1
                if not point_relation(p7, a7, GF(7), k):
                    continue
                counts["local_7_pass"] += 1
                counts["local_checks_11"] += 1
                if not point_relation(p11, a11, GF(11), k):
                    continue
                counts["local_11_pass"] += 1
                shifts = [crt.centered_k(a7[j], a11[j]) for j in range(3)]
                coeffs = crt.rational_curve(points, shifts)
                qa = [QQ(v.numerator)/QQ(v.denominator) for v in coeffs]
                counts["rational_checks"] += 1
                good = point_relation(points, qa, QQ, k)
                counts["rational_relations"] += good
                checked.append({"pair_index": index, "local_indices": [i7, i11],
                                "coefficient": k, "crt_shifts": shifts,
                                "curve_coefficients": list(map(str, coeffs)),
                                "rational_relation": bool(good)})
    assert counts["reduced_compatible_pairs"] == 42
    raw = "".join(json.dumps(x, sort_keys=True) + "\n" for x in checked).encode()
    (HERE / "rational-checks.jsonl.gz").write_bytes(gzip.compress(raw, mtime=0))
    return {"schema": "xedni-periodic-coefficient-result-v1", "status": "PASS",
            "protocol_sha256": hashlib.sha256(protocol_bytes).hexdigest(),
            "script_sha256": hashlib.sha256((HERE / "run_sage.py").read_bytes()).hexdigest(),
            "source_packed_sha256": hashlib.sha256(source_bytes).hexdigest(),
            "full_law_pair_rows_raw_sha256": hashlib.sha256(pair_bytes).hexdigest(),
            "counts": dict(counts), "rational_checks_raw_sha256": hashlib.sha256(raw).hexdigest()}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
