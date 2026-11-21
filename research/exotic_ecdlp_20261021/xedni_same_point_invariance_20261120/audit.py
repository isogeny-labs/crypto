"""Independent exact-rational replay of the tangent-compatibility census."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
OLD = ROOT / "xedni_local_collision_crt_20261120"
spec = importlib.util.spec_from_file_location("previous_run", OLD / "run.py")
previous = importlib.util.module_from_spec(spec)
spec.loader.exec_module(previous)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def mod_fraction(x, p):
    x = Fraction(x)
    assert x.denominator % p != 0
    return x.numerator * pow(x.denominator, -1, p) % p


def main():
    result = json.loads((HERE / "result.json").read_text())
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert digest((HERE / "screen.py").read_bytes()) == result["script_sha256"]
    assert digest((HERE / "protocol.json").read_bytes()) == result["protocol_sha256"]
    source_bytes = SOURCE.read_bytes()
    assert digest(source_bytes) == result["source_sha256"] == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    old_result = json.loads((OLD / "result.json").read_text())
    old_raw = gzip.decompress((OLD / "pair-rows.jsonl.gz").read_bytes())
    assert digest(old_raw) == old_result["pair_rows_raw_sha256"]
    old_rows = {r["pair_index"]: r for r in map(json.loads, old_raw.splitlines())}

    totals = Counter()
    for row in result["rows"]:
        index, p = row["pair_index"], row["prime"]
        points = [tuple(z) for z in source[index]["lifted_points"]]
        (x1, y1), (x2, y2) = points
        dx, dy = x1-x2, y1-y2
        assert dx != 0 and dx % p == dy % p == 0 and (dx//p) % p != 0
        local = {}
        for q, order in ((7, 4), (11, 6)):
            qpoints = tuple(tuple(z % q for z in P) for P in points)
            local[q] = (previous.collided_gate(qpoints, q, order)[0] if q == p
                        else previous.prior.local_gate(qpoints, q, order)[0])
        compatible = 0
        for a7 in local[7]:
            for a11 in local[11]:
                k = [previous.prior.centered_k(a7[i], a11[i]) for i in range(3)]
                rational_a4 = previous.prior.rational_curve(points, k)[3]
                collision_a4 = (a7 if p == 7 else a11)[3]
                compatible += mod_fraction(rational_a4, p) == collision_a4
        total = len(local[7]) * len(local[11])
        assert total == row["local_pairs"]
        assert compatible == row["compatible"]
        assert total-compatible == row["incompatible"]
        old = old_rows[index]
        assert old["collision_kind"] == "same_point"
        assert old["local_pairs"] == total
        assert old["constructed"] == compatible
        assert old["no_extra_digit"] == total-compatible
        totals["source_pairs"] += 1
        totals["local_pairs"] += total
        totals["compatible"] += compatible
        totals["incompatible"] += total-compatible
        totals[f"source_pairs_p{p}"] += 1
    assert dict(totals) == result["counts"]
    print(json.dumps({"status": "PASS", "counts": dict(totals),
                      "method": "exact rational a4 reduction and prior-row reconciliation"},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
