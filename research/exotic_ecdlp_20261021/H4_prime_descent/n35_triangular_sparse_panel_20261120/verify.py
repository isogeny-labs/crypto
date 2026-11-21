#!/usr/bin/env python3
"""Independent finite-instance replay of all five local n35 IC/rho pairs."""
from sage.all import GF, PolynomialRing, EllipticCurve
import hashlib
import json
from pathlib import Path
import random
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
FIX = json.loads((HERE / "fixtures.json").read_text())
RESULT = json.loads((HERE / "result.json").read_text())
CANDIDATE = json.loads((ROOT / SPEC["candidate"]).read_text())
SOURCE = json.loads((ROOT / SPEC["full_result"]).read_text())


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def digest(obj):
    return hashlib.sha256(json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()


assert RESULT["status"] == "complete"
assert sha(HERE / "run.py") == SPEC["runner_sha256"]
assert sha(HERE / "fixtures.json") == SPEC["fixtures_sha256"] == RESULT["fixtures_sha256"]
assert sha(HERE / "protocol.json") == RESULT["protocol_sha256"]
for name, expected in SPEC["source_hashes"].items():
    assert sha(ROOT / name) == expected
record = CANDIDATE["record"]
candidate_id = CANDIDATE["candidate_id"]
assert candidate_id == SPEC["candidate_id"] == FIX["candidate_id"] == RESULT["candidate_id"]
assert candidate_id.endswith("h" + digest(record)[:12])
field = record["field"]
n = field["n"]
poly = field["defining_polynomial_int"]
F2 = GF(2)
PR = PolynomialRing(F2, "z")
z = PR.gen()
mod = sum(F2((poly >> i) & 1) * z**i for i in range(n + 1))
K = GF(1 << n, "z", modulus=mod)


def elt(v):
    return K([(v >> i) & 1 for i in range(n)])


coeff = record["curve"]["coefficients"]
E = EllipticCurve(K, [1, elt(coeff["a2"]), 0, 0, elt(coeff["a6"])])
r = record["curve"]["r"]
assert r == 520604191 and int(E.cardinality()) == record["curve"]["curve_order"]


def point(pair):
    return E(elt(pair[0]), elt(pair[1]))


def rec(P):
    return [int(P[0].to_integer()), int(P[1].to_integer())]


G = point(record["curve"]["G"])
assert r * G == E(0)
base = [point(pair) for pair in record["factor_base"]["encoded_points"]]
assert len(base) == 4096 and digest([rec(P) for P in base]) == record["factor_base"]["enumerated_set_digest"]
reps = sorted({min((P, -P), key=rec) for P in base}, key=rec)
assert len(reps) == 2048
col_of = {P: i for i, P in enumerate(reps)}
logs = SOURCE["factor_logs"]
assert len(logs) == len(reps) and digest(logs) == RESULT["shared_setup"]["factor_logs_sha256"] == SOURCE["factor_logs_sha256"]
assert all(logs[i] * G == reps[i] for i in range(len(reps)))


def key(P):
    if P == E(0):
        return 1 << (2 * n)
    return int(P[0].to_integer()) | (int(P[1].to_integer()) << n)


# Column-major traversal and explicit minimum payload differ from the producer.
index = {}
B = len(base)
for j in range(B):
    for i in range(j + 1):
        k = key(base[i] + base[j])
        packed = i * B + j
        previous = index.get(k)
        if previous is None or packed < previous:
            index[k] = packed
assert len(index) == SPEC["distinct_pair_sums"] == RESULT["shared_setup"]["distinct_pair_sums"]


def row_for(indices):
    row = [0] * len(reps)
    signed = {}
    for i in indices:
        P = base[i]
        rep = min((P, -P), key=rec)
        col = col_of[rep]
        sign = 1 if P == rep else -1
        row[col] = (row[col] + sign) % r
        signed[col] = signed.get(col, 0) + sign
    return row, signed


assert len(RESULT["rows"]) == len(FIX["targets"]) == 5
ratios = []
points = set()
for fixture, pair in zip(FIX["targets"], RESULT["rows"]):
    j = fixture["index"]
    assert pair["index"] == j and pair["order"] == fixture["order"]
    assert pair["tag"] == fixture["tag"] and pair["target"] == fixture["point"]
    scalar = int.from_bytes(hashlib.sha256(fixture["tag"].encode()).digest(), "big") % (r - 1) + 1
    assert scalar == fixture["fixture_scalar"] == pair["fixture_scalar"]
    Q = point(fixture["point"])
    assert scalar * G == Q
    points.add(tuple(fixture["point"]))
    workload = {"curve_id": FIX["curve_id"], "subgroup_order": r,
                "G": rec(G), "target": fixture["point"], "target_count": 1,
                "target_law": "sha256_nonzero_scalar_times_G", "target_label": fixture["tag"],
                "precomputation_state": "ready"}
    workload_id = "W" + digest(workload)[:12]
    assert pair["workload_id"] == workload_id
    assert pair["run_id"] == candidate_id + workload_id + "R1"
    assert pair["status"] == "verified_pair" and not pair["resource_cap_exceeded"]
    assert pair["peak_rss_bytes"] < SPEC["rss_cap_bytes"]
    assert pair["calibration_before_ns"] > 0 and pair["calibration_after_ns"] > 0
    assert pair["calibration_ratio"] == max(pair["calibration_before_ns"], pair["calibration_after_ns"]) / min(pair["calibration_before_ns"], pair["calibration_after_ns"])
    ic, rho = pair["arms"]["IC"], pair["arms"]["rho"]
    assert ic["status"] == rho["status"] == "verified"
    assert ic["scalar_replay"] and rho["scalar_replay"]
    assert ic["scalar"] == rho["scalar"] == scalar
    assert ic["scalar"] * G == rho["scalar"] * G == Q
    assert sum(ic["phase_wall_ns"].values()) == ic["wall_ns"]
    rng = random.Random(73035)
    for shift_number, shift_row in enumerate(ic["target_shifts"]):
        shift = 0 if shift_number == 0 else rng.randrange(1, r)
        assert shift_row["shift_number"] == shift_number and shift_row["shift_scalar"] == shift
        shifted = Q + shift * G if shift else Q
        start = rng.randrange(B)
        first = next((offset for offset in range(B)
                      if key(shifted - base[(start + offset) % B]) in index), None)
        if first is None:
            assert shift_row["status"] == "exhausted"
            assert shift_row["third_point_attempts"] == B
        else:
            assert shift_row["status"] == "verified"
            assert shift_row["third_point_attempts"] == first + 1
            indices = ic["witness_indices"]
            assert indices[2] == (start + first) % B
            assert indices[0] * B + indices[1] == index[key(shifted - base[indices[2]])]
            assert sum((base[i] for i in indices), E(0)) == shifted
            assert [rec(base[i]) for i in indices] == ic["witness"]
            row, signed = row_for(indices)
            assert digest(row) == ic["row_sha256"]
            assert [[col, signed[col]] for col in sorted(signed)] == ic["signed_coefficients"]
            assert sum(v != 0 for v in signed.values()) == ic["nonzero_signed_columns"]
            assert sum((v * reps[col] for col, v in signed.items()), E(0)) == shifted
            assert (sum(v * logs[col] for col, v in signed.items()) - shift) % r == scalar
            assert shift_number == len(ic["target_shifts"]) - 1
    ratios.append(rho["wall_ns"] / ic["wall_ns"])
assert len(points) == 5
summary = {"schema": "n35-triangular-sparse-local-panel-validation-v1", "status": "passed",
           "result_sha256": sha(HERE / "result.json"), "candidate_id": candidate_id,
           "verified_pairs": len(ratios), "distinct_targets": len(points),
           "ratio_by_pair": ratios, "median_paired_ratio": statistics.median(ratios),
           "median_ic_online_ns": statistics.median(row["arms"]["IC"]["wall_ns"] for row in RESULT["rows"]),
           "median_rho_online_ns": statistics.median(row["arms"]["rho"]["wall_ns"] for row in RESULT["rows"]),
           "calibration_ratio_max": max(row["calibration_ratio"] for row in RESULT["rows"]),
           "host_tier": "ordinary_exploratory"}
(HERE / "validation.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
