"""Independent exact-fraction replay of the Sage family census."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
GATE = ROOT / "xedni_six_pair_shift_gate_20261120/rows.jsonl.gz"
spec = importlib.util.spec_from_file_location("full_law_fraction_audit", ROOT / "xedni_full_law_crt_20261120/audit.py")
independent = importlib.util.module_from_spec(spec)
spec.loader.exec_module(independent)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def curve_invariants(a):
    a1, a2, a3, a4, a6 = a
    b2 = a1*a1+4*a2
    b4 = a1*a3+2*a4
    b6 = a3*a3+4*a6
    b8 = a1*a1*a6+4*a2*a6-a1*a3*a4+a2*a3*a3-a4*a4
    c4 = b2*b2-24*b4
    delta = -b2*b2*b8-8*b4**3-27*b6*b6+9*b2*b4*b6
    assert delta
    return delta, c4**3/delta


def main():
    p = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    gate_bytes = GATE.read_bytes()
    assert sha(gate_bytes) == p["shift_gate_rows_sha256"]
    gates = [json.loads(x) for x in gzip.decompress(gate_bytes).splitlines()]
    selected = {(r["source_index"], tuple(r["local_indices"])): r for r in gates
                if r["status"] == "centered_relation_necessary"}
    packed = (HERE / "rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(packed) == result["rows_packed_sha256"]
    assert sha(raw) == result["rows_raw_sha256"]
    rows = [json.loads(x) for x in raw.splitlines()]
    assert len(rows) == 8*33
    counts = Counter()
    j_values = set()
    for row in rows:
        key = (row["source_index"], tuple(row["local_indices"]))
        gate = selected[key]
        t = row["shift_t"]
        assert -16 <= t <= 16
        k = gate["centered_k"].copy()
        k[2] += 77*t
        assert row["k"] == k
        a = [F(x) for x in row["coefficients"]]
        assert a[:3] == [47*F(z) for z in k]
        pts = [tuple(F(z) for z in xy) for xy in gate["points"]]
        assert all(independent.on_curve(a, pt) for pt in pts)
        assert independent.rational_relation(pts, a) and row["relation"]
        delta, j = curve_invariants(a)
        assert str(delta) == row["discriminant"] and str(j) == row["j"]
        j_values.add(j)
        for prime, wanted in ((47, [0, 0, 0, 46, 45]),
                              (7, gate["a7"]), (11, gate["a11"])):
            assert all(c.denominator % prime for c in a)
            reduced = [c.numerator*pow(c.denominator, -1, prime) % prime for c in a]
            assert reduced == wanted == row["reductions"][str(prime)]["coefficients"]
        counts[key] += 1
    assert len(counts) == 8 and set(counts.values()) == {33}
    assert len(j_values) == result["distinct_j"] == 132
    print(json.dumps({"schema": "xedni-positive-shift-family-audit-v1",
                      "status": "PASS", "verified_rows": len(rows),
                      "classes": len(counts), "distinct_j": len(j_values),
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
