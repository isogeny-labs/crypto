#!/usr/bin/env python3
"""Centered one-generator control paired with the frozen two-generator box."""

import hashlib
import json
import statistics
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def height(q):
    return max(abs(int(q.numerator())).bit_length(), int(q.denominator()).bit_length())


def red(q):
    return int(q.numerator())*pow(int(q.denominator()), -1, 47) % 47


def main():
    protocol_raw = (HERE / "centered-control-protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    assert sha((HERE / "protocol.json").read_bytes()) == protocol["parent_protocol_sha256"]
    box_raw = (HERE / "box-rows.jsonl").read_bytes()
    box_result = json.loads((HERE / "result.json").read_text())
    assert sha(box_raw) == box_result["box_rows_raw_sha256"]
    box = [json.loads(line) for line in box_raw.splitlines()]
    min_box = {}
    for row in box:
        if row["reduced_point"] is None:
            continue
        point = tuple(row["reduced_point"])
        min_box[point] = min(min_box.get(point, 10**9), row["height_bits"])
    E = EllipticCurve(QQ, protocol["curve_coefficients"])
    Q = E(protocol["Q"])
    Ep = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    G = Ep([red(Q[0]), red(Q[1])])
    rows = []
    for k in range(-21, 22):
        if k == 0:
            continue
        P = k*Q
        point = Ep([red(P[0]), red(P[1])])
        assert point == k*G
        point_key = (int(point[0]), int(point[1]))
        own_height = max(height(P[0]), height(P[1]))
        rows.append({"k": k, "reduced_point": list(point_key),
                     "centered_height_bits": own_height,
                     "box_min_height_bits": min_box[point_key]})
    assert len(rows) == 42 and len({tuple(r["reduced_point"]) for r in rows}) == 42
    raw = "".join(json.dumps(row, sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "centered-comparison-rows.jsonl").write_bytes(raw)
    return {"schema": "xedni-centered-single-generator-control-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_raw), "script_sha256": sha((HERE / "run_centered_sage.py").read_bytes()),
            "box_rows_raw_sha256": sha(box_raw), "comparison_rows_raw_sha256": sha(raw),
            "centered_table_entries": 42, "two_generator_table_entries": 81,
            "centered_median_height_bits": statistics.median(r["centered_height_bits"] for r in rows),
            "centered_max_height_bits": max(r["centered_height_bits"] for r in rows),
            "box_median_height_bits": statistics.median(r["box_min_height_bits"] for r in rows),
            "box_max_height_bits": max(r["box_min_height_bits"] for r in rows),
            "box_strictly_smaller_targets": sum(r["box_min_height_bits"] < r["centered_height_bits"] for r in rows)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
