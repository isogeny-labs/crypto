#!/usr/bin/env python3
"""Exact point incidence and reduced-row rank of wide-period witnesses."""

import gzip
import hashlib
import json
from collections import Counter, defaultdict
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def on_curve(a, point):
    x, y = point
    a1, a2, a3, a4, a6 = a
    return y*y+a1*x*y+a3*y == x**3+a2*x*x+a4*x+a6


def rank_mod(rows, p):
    if not rows:
        return 0
    work = [list(map(lambda v: v%p, row)) for row in rows]
    n = len(work[0])
    assert all(len(row) == n for row in work)
    pivot_row = 0
    for col in range(n):
        pivot = next((i for i in range(pivot_row, len(work)) if work[i][col]), None)
        if pivot is None:
            continue
        work[pivot_row], work[pivot] = work[pivot], work[pivot_row]
        inv = pow(work[pivot_row][col], -1, p)
        work[pivot_row] = [(v*inv)%p for v in work[pivot_row]]
        for i in range(len(work)):
            if i != pivot_row and work[i][col]:
                scalar = work[i][col]
                work[i] = [(x-scalar*y)%p for x,y in zip(work[i], work[pivot_row])]
        pivot_row += 1
        if pivot_row == len(work):
            break
    return pivot_row


def orbit(point):
    x, y = point
    neg_y = (-y)%47
    assert y != neg_y
    return (x, min(y, neg_y)), (1 if y < neg_y else -1)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    part_dir = PARENT / "xedni_wide_period_relation_20261120"
    parts = sorted(part_dir.glob("rational-checks.part-*.jsonl.gz"))
    assert len(parts) == 7
    raw = b"".join(gzip.decompress(path.read_bytes()) for path in parts)
    assert sha(raw) == protocol["rational_checks_raw_sha256"]
    positives = [json.loads(x) for x in raw.splitlines() if json.loads(x)["rational_relation"]]
    assert len(positives) == 40

    point_lifts = {}
    for record in source:
        for reduced, lifted in zip(record["reduced_points"], record["lifted_points"]):
            reduced = tuple(reduced)
            lifted = tuple(F(v) for v in lifted)
            if reduced in point_lifts:
                assert point_lifts[reduced] == lifted
            point_lifts[reduced] = lifted
    assert len(point_lifts) == 42
    orbits = sorted({orbit(point)[0] for point in point_lifts})
    assert len(orbits) == 21
    orbit_index = {point: i for i, point in enumerate(orbits)}

    curve_groups = defaultdict(list)
    global_rows = []
    records = []
    for positive in positives:
        a = tuple(F(v) for v in positive["curve_coefficients"])
        index = positive["pair_index"]
        designated = [tuple(P) for P in source[index]["reduced_points"]]
        assert all(on_curve(a, point_lifts[P]) for P in designated)
        incident = sorted(P for P, lift in point_lifts.items() if on_curve(a, lift))
        row = [0]*len(orbits)
        for coefficient, point in zip((1, 2), designated):
            key, sign = orbit(point)
            row[orbit_index[key]] = (row[orbit_index[key]]+coefficient*sign)%43
        record = {"pair_index": index, "local_indices": positive["local_indices"],
                  "curve_coefficients": positive["curve_coefficients"],
                  "designated_points": [list(P) for P in designated],
                  "incident_points": [list(P) for P in incident],
                  "extra_incident_points": len(incident)-len(set(designated)),
                  "signed_relation_row": row}
        records.append(record)
        curve_groups[a].append(row)
        global_rows.append(row)
    per_curve_rank = Counter(rank_mod(rows, 43) for rows in curve_groups.values())
    incident_hist = Counter(len(record["incident_points"]) for record in records)
    exact_curve_pair_count = Counter(len(rows) for rows in curve_groups.values())
    raw_rows = "".join(json.dumps(x, sort_keys=True)+"\n" for x in records).encode()
    (HERE / "curve-rows.jsonl").write_bytes(raw_rows)
    return {"schema": "xedni-witness-incidence-result-v1", "status": "PASS",
        "protocol_sha256": sha(protocol_bytes),
        "script_sha256": sha((HERE / "run.py").read_bytes()),
        "source_packed_sha256": sha(source_bytes),
        "rational_checks_raw_sha256": sha(raw),
        "curve_rows_raw_sha256": sha(raw_rows),
        "positive_records": len(records), "distinct_exact_curves": len(curve_groups),
        "source_points": len(point_lifts), "sign_orbits": len(orbits),
        "total_extra_incident_point_records": sum(x["extra_incident_points"] for x in records),
        "global_signed_relation_rank_mod_43": rank_mod(global_rows, 43),
        "per_curve_rank_histogram": dict(sorted(per_curve_rank.items())),
        "incident_point_count_histogram": dict(sorted(incident_hist.items())),
        "positive_records_per_exact_curve_histogram": dict(sorted(exact_curve_pair_count.items()))}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
