"""Recompute the fixed-difference quotient and signed-label gate from raw rows."""
import json
from collections import Counter
from pathlib import Path

B = Path(__file__).resolve().parent


def rows(path):
    return [json.loads(x) for x in path.read_text().splitlines()]


def same(x, y, p):
    return all((x[i] * y[j] - x[j] * y[i]) % p == 0 for i in range(4) for j in range(4))


by = []
total = Counter()
for bits in [16, 20, 24, 28, 32, 36, 40]:
    rr = rows(B / f"results/transition-{bits}.jsonl")
    src = rows(B.parent / f"kummer_coordinates_20261022/results/target-gate-{bits}.jsonl")
    assert len(rr) == 540 and len(src) == 180
    counts = Counter()
    for i, row in enumerate(rr):
        s = src[i // 3]
        assert row["step"] == i % 3
        assert row["bits"] == bits and row["p"] == s["p"] and row["r"] == s["r"]
        assert row["workload_id"] == s["workload_id"] and row["source_scalar_verified"]
        assert row["status"] == "pass" and row["expected_signed"] == row["label_signed"]
        assert same(row["output_k"], row["expected_k"], row["p"])
        assert same(row["output_k_plus_g"], row["expected_k_plus_g"], row["p"])
        a, b = row["input_label"]
        choice = row["branch"]
        assert row["output_label"] == [(2*a + choice) % row["r"], (2*b) % row["r"]]
        if i % 3:
            prev = rr[i - 1]
            assert row["input_label"] == prev["output_label"]
            assert same(row["input_k"], prev["output_k"], row["p"])
            assert same(row["input_k_plus_g"], prev["output_k_plus_g"], row["p"])
        counts[f"branch_{choice}"] += 1
        counts["repair_calls"] += sum(x is not None for x in row["torsion_repair_roots"])
    by.append({"bits": bits, "targets": 180, "transitions": 540, **counts})
    total.update(counts)

summary = {
    "schema": "kummer-differential-walk-summary/v1",
    "study_id": "KUMMER-DIFFERENTIAL-WALK-20261022",
    "source_verified_targets": 1260,
    "verified_transitions": 3780,
    "wrong_or_inapplicable": 0,
    "branch_0": total["branch_0"],
    "branch_1": total["branch_1"],
    "torsion_repair_calls": total["repair_calls"],
    "collector_completed": False,
    "candidate_cost": None,
    "rho_cost": None,
    "candidate_to_rho_ratio": None,
    "by_bits": by,
}
(B / "results/summary.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, sort_keys=True))
