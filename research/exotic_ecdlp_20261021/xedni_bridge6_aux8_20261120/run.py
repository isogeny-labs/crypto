"""Enumerate two local coefficient boxes and construct their CRT bridge."""

import gzip
import hashlib
import importlib.util
import itertools
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
ARITH = ROOT / "xedni_bridge6_aux5_20261120/audit.py"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def pack(rows):
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n"
                  for row in rows).encode()
    return raw, gzip.compress(raw, mtime=0)


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(ARITH.read_bytes()) == protocol["arithmetic_source_sha256"]
    spec = importlib.util.spec_from_file_location("bridge6_arithmetic", ARITH)
    arithmetic = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(arithmetic)
    points = [json.loads(line)["lifted_points"] for line in gzip.decompress(source_bytes).splitlines()]
    pair = points[31]
    assert pair == [[2, 2], [-16, 17]]
    fixed = (0, 0, 0, 46, 45)
    P, Q = [tuple(v % 47 for v in xy) for xy in pair]
    assert arithmetic.add(P, arithmetic.mul(Q, 6, fixed, 47), fixed, 47) is None
    rows = []
    selected = {}
    counts = Counter()
    for prime, desired, bound in ((7, 4, 3), (11, 8, 5)):
        for k in itertools.product(range(-bound, bound+1), repeat=3):
            a = arithmetic.coefficients(pair, k)
            if a is None:
                status = "same_x"
                details = {"status": status}
            elif arithmetic.discriminant(a) == 0:
                status = "singular"
                details = {"status": status}
            else:
                details = arithmetic.local(a, pair, prime, desired)
                status = details["status"]
            rows.append({"prime": prime, "k": list(k), "local": details})
            counts[f"p{prime}_{status}"] += 1
            if status == "pass" and prime not in selected:
                selected[prime] = list(k)
    assert sorted(selected) == [7, 11]
    centered = []
    for k7, k11 in zip(selected[7], selected[11]):
        v = next(v for v in range(77) if v % 7 == k7 % 7 and v % 11 == k11 % 11)
        centered.append(v if v <= 38 else v-77)
    coeffs = arithmetic.coefficients(pair, centered)
    assert coeffs is not None and arithmetic.discriminant(coeffs) != 0
    local7 = arithmetic.local(coeffs, pair, 7, 4)
    local11 = arithmetic.local(coeffs, pair, 11, 8)
    assert local7["status"] == local11["status"] == "pass"
    from fractions import Fraction as F
    Pr, Qr = [(F(x), F(y)) for x, y in pair]
    rational_relation = arithmetic.add(Pr, arithmetic.mul(Qr, 6, coeffs), coeffs) is None
    raw, packed = pack(rows)
    (HERE / "local-rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-bridge6-aux8-result-v1", "status": "PASS",
                      "source_sha256": sha(source_bytes), "arithmetic_source_sha256": sha(ARITH.read_bytes()),
                      "source_pair_index": 31, "points": pair,
                      "local_attempts": len(rows), "counts": dict(counts),
                      "first_selected": {str(p): k for p, k in selected.items()},
                      "crt_k": centered, "crt_coefficients": list(map(str, coeffs)),
                      "crt_local_7": local7, "crt_local_11": local11,
                      "crt_rational_relation": rational_relation,
                      "local_rows_raw_sha256": sha(raw), "local_rows_packed_sha256": sha(packed)},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
