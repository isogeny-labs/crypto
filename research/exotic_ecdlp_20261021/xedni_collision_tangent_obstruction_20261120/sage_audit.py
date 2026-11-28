"""Independent source and F47 replay of the eight Lean certificates."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
REDUCTION = ROOT / "xedni_collision_reduction_screen_20261120" / "result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == result["source_sha256"] == protocol["source_sha256"]
    assert sha(REDUCTION.read_bytes()) == result["reduction_screen_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    E = EllipticCurve(GF(47), [0, 0, 0, -1, -2])
    assert E.cardinality() == 43 and E.discriminant() != 0
    formal = {r[0]: r[1:] for r in protocol["formal_cases"]}
    counts = Counter()
    for row in result["rows"]:
        index = row["pair_index"]
        (x1, y1), (x2, y2) = source[index]["lifted_points"]
        assert x1 != x2
        assert (x1-x2) % 7 == 0 or (x1-x2) % 11 == 0
        P = E([x1 % 47, y1 % 47])
        Q = E([x2 % 47, y2 % 47])
        reduced = bool(P+2*Q == E(0))
        dx, dy = x1-x2, y1-y2
        remainder = dy*dy % abs(dx)
        assert row["point1"] == [x1, y1] and row["point2"] == [x2, y2]
        assert row["dx"] == dx and row["dy"] == dy
        assert row["dy2_mod_abs_dx"] == remainder
        assert row["reduced_relation"] == reduced
        if reduced:
            assert formal[index] == [x1, x2, y1, y2]
            assert remainder != 0 and row["status"] == "tangent_divisibility"
        else:
            assert row["status"] == "fixed_prime_reduction"
        counts[row["status"]] += 1
    assert dict(counts) == result["counts"]
    assert len(formal) == counts["tangent_divisibility"] == 8
    print(json.dumps({"status": "PASS", "source_pairs": len(source),
                      "collision_pairs": sum(counts.values()), "counts": dict(counts),
                      "fixed_curve_order": int(E.cardinality())}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
