"""Independent arithmetic and archive validator for the signed-recovery gate."""
import json
from collections import Counter
from pathlib import Path

B = Path(__file__).resolve().parent
R = B / "results"


def rows(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def curve(theta, p):
    poly = [0, 1]
    for root in [1, theta["lambda"], theta["mu"], theta["nu"]]:
        nxt = [0] * (len(poly) + 1)
        for i, c in enumerate(poly):
            nxt[i] = (nxt[i] - root * c) % p
            nxt[i + 1] = (nxt[i + 1] + c) % p
        poly = nxt
    return poly + [0]


def modquad(poly, a0, a1, p):
    x0, x1 = 1, 0
    r0, r1 = 0, 0
    for coeff in poly:
        r0 = (r0 + coeff * x0) % p
        r1 = (r1 + coeff * x1) % p
        x0, x1 = (-a0 * x1) % p, (x0 - a1 * x1) % p
    return r0, r1


theta = {r["p"]: r["selected"]["constants"] for r in rows(
    B.parent / "kummer_theta_20261022/results/theta-models.jsonl"
)}
totals = Counter()
by_bits = []
seen = set()
for bits in [16, 20, 24, 28, 32, 36, 40]:
    rr = rows(R / f"recover-target-{bits}.jsonl")
    old = rows(R / f"literal-recover-failures/recover-target-{bits}.jsonl")
    source = rows(B.parent / f"kummer_coordinates_20261022/results/target-gate-{bits}.jsonl")
    assert len(rr) == len(old) == len(source) == 180
    counts = Counter()
    for row, lit, src in zip(rr, old, source):
        key = (bits, row["p"], row["target_index"])
        assert key not in seen
        seen.add(key)
        assert row["bits"] == src["bits"] == bits
        assert row["p"] == src["p"]
        assert row["workload_id"] == src["workload_id"]
        assert row["source_scalar_verified"] and src["all_source_checks"]
        assert row["computed"] == row["expected"]
        assert row["correct"] and row["general_conversion_ok"]
        assert row["expected"] == src["cases"][2]["coordinate_witness"]["jacobian_expected"]
        assert not lit["correct"]
        p = row["p"]
        div = row["computed"]
        a0, a1 = div["u"][:2]
        b0, b1 = div["v"]
        f = curve(theta[p], p)
        assert modquad([b0*b0, 2*b0*b1, b1*b1], a0, a1, p) == modquad(f, a0, a1, p)
        xi4 = (b1*b1 + (a1*a1-a0)*(f[5]*a1 - f[6]*(a1*a1-a0)) + a1*(f[3]-f[4]*a1)-f[2]) % p
        assert row["general_target"] == [1, (-a1) % p, a0, xi4]
        counts[row["status"]] += 1
        totals[row["status"]] += 1
    by_bits.append({"bits": bits, "targets": len(rr), **dict(counts)})

zero = rows(R / "zero-shift.jsonl")
assert len(zero) == 175 and all(r["repair_verified"] for r in zero)
w = json.loads((R / "equation-witness.json").read_text())
assert (2 * w["f5"] * w["xi3"] * w["Z2"] * w["Z3"]) % w["p"] == 423
assert (w["equation_11_printed_V"] + 423) % w["p"] == w["equation_11_derived_V"]
assert w["derived_expected_v"] == rows(R / "recover-target-16.jsonl")[0]["expected"]["v"]
assert w["literal_recovered_v"] == rows(R / "literal-recover-failures/recover-target-16.jsonl")[0]["computed"]["v"]
assert len(seen) == 1260 and totals["computed"] == 1257
assert totals["enumerated_exception_lift"] == 3
summary = {
    "schema": "kummer-recover-summary/v1",
    "study_id": "KUMMER-RECOVER-20261022",
    "source_verified_targets": 1260,
    "correct_signed_recoveries": 1260,
    "correct_fast_generic_recoveries": 1257,
    "correct_enumerated_exception_lifts": 3,
    "wrong_or_inapplicable": 0,
    "literal_printed_equation_wrong": 1257,
    "literal_printed_equation_D_zero": 3,
    "zero_output_repaired": 175,
    "closed_labeled_collector_completed": False,
    "candidate_cost": None,
    "rho_cost": None,
    "candidate_to_rho_ratio": None,
    "by_bits": by_bits,
}
(R / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
print(json.dumps(summary, sort_keys=True))
