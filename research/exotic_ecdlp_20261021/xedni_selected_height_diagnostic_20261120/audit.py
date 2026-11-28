#!/usr/bin/env python3
"""Independent source, arithmetic, and status audit of the selected panel."""

import gzip
import hashlib
import json
import math
from collections import defaultdict
from decimal import Decimal
from fractions import Fraction
from pathlib import Path


ROOT = Path(__file__).resolve().parent


def load(name):
    return json.loads((ROOT / name).read_text())


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    protocol = load("protocol.json")
    runtime = load("sage-runtime-info.json")
    assert runtime["status"] == "verified"
    source = Path(protocol["source"]["path"])
    assert digest(source) == protocol["source"]["sha256"]
    assert digest(ROOT / "fixtures.json") == protocol["fixtures_sha256"]
    for name, expected in protocol["source_sha256"].items():
        assert digest(ROOT / name) == expected
    fixtures = load("fixtures.json")
    seen = defaultdict(lambda: {"points": set(), "pairs": []})
    row_count = 0
    with gzip.open(source, "rt") as stream:
        for line in stream:
            row = json.loads(line)
            row_count += 1
            if row["accepted"]:
                key = (row["A"], row["B"])
                pts = tuple(tuple(str(v) for v in p) for p in row["lifted_points"])
                seen[key]["points"].update(pts)
                seen[key]["pairs"].append({
                    "points": pts, "reduced_relation": row["reduced_relation"],
                    "rational_relation": row["rational_relation"]})
    assert row_count == fixtures["source_rows"] == 1722
    assert sum(len(v["pairs"]) for v in seen.values()) == fixtures["accepted_pairs"] == 20
    assert len(seen) == len(fixtures["curves"]) == 3
    for item in fixtures["curves"]:
        A, B = item["weierstrass_coefficients"][3:]
        assert set(map(tuple, item["points"])) == seen[(A, B)]["points"]
        assert len(item["pairs"]) == len(seen[(A, B)]["pairs"])
        for saved, replayed in zip(item["pairs"], seen[(A, B)]["pairs"]):
            assert tuple(map(tuple, saved["points"])) == replayed["points"]
            assert saved["reduced_relation"] == replayed["reduced_relation"]
            assert saved["rational_relation"] == replayed["rational_relation"]

    heights = load("height-result.json")
    ranks = load("rank-result.json")
    proofs = load("rank-proof-result.json")
    effort = load("c2-effort-result.json")
    perturb = load("perturb-result.json")
    torsion_protocol = load("torsion-protocol.json")
    assert digest(ROOT / "torsion_count.py") == torsion_protocol["source_sha256"]
    torsion = load("torsion-result.json")
    certificate_protocol = load("torsion-certificate-protocol.json")
    assert digest(ROOT / "torsion_certificate.py") == certificate_protocol["source_sha256"]
    assert digest(ROOT / "torsion-result.json") == certificate_protocol["point_counts_sha256"]
    certificate = load("torsion-certificate.json")
    assert [x["id"] for x in heights["curves"]] == [x["id"] for x in fixtures["curves"]]
    summaries = []
    for fixture, measured in zip(fixtures["curves"], heights["curves"]):
        A, B = (Fraction(x) for x in fixture["weierstrass_coefficients"][3:])
        delta = -16 * (4 * A**3 + 27 * B**2)
        assert Fraction(measured["discriminant"]) == delta != 0
        assert len(measured["points"]) == len(fixture["points"]) == 4
        assert len(measured["pairs"]) == len(fixture["pairs"])
        errors = []
        heights_this = []
        for row in measured["points"]:
            assert row["coordinates"] in fixture["points"]
            p, s = Decimal(row["height_pari"]), Decimal(row["height_sage"])
            e = Decimal(row["crosscheck_abs_error"])
            assert p > 0 and s > 0 and e == abs(p - s) and e < Decimal("1e-30")
            heights_this.append(p)
            errors.append(e)
        residuals = [Decimal(x["parallelogram_residual"]) for x in measured["pairs"]]
        assert all(Decimal(0) <= e < Decimal("1e-30") for e in residuals)
        for actual, expected in zip(measured["pairs"], fixture["pairs"]):
            assert actual["points"] == expected["points"]
            assert actual["reduced_relation"] == expected["reduced_relation"]
            assert actual["rational_relation"] == expected["rational_relation"]
        assert 1 <= measured["torsion_order"] <= 16
        summaries.append({"id": measured["id"], "torsion_order": measured["torsion_order"],
                          "minimum_selected_height": str(min(heights_this)),
                          "maximum_selected_height": str(max(heights_this)),
                          "max_algorithm_error": str(max(errors)),
                          "max_parallelogram_residual": str(max(residuals))})
    for fixture, row in zip(fixtures["curves"], torsion["rows"]):
        assert fixture["id"] == row["id"]
        A, B = (Fraction(x) for x in fixture["weierstrass_coefficients"][3:])
        assert math.gcd(*row["orders"]) == row["gcd"] == 1
        assert [x["group_order"] for x in row["good_prime_counts"]
                if x["p"] in row["first_coprime_pair"]] == row["orders"]
        for count in row["good_prime_counts"]:
            p = count["p"]
            a = A.numerator * pow(A.denominator, -1, p) % p
            b = B.numerator * pow(B.denominator, -1, p) % p
            assert [a, b] == [count["a_mod_p"], count["b_mod_p"]]
            replay = 1
            for x in range(p):
                y2 = (x**3 + a*x + b) % p
                replay += 1 if y2 == 0 else (2 if pow(y2, (p-1)//2, p) == 1 else 0)
            assert replay == count["group_order"]
    for count_row, proof_row in zip(torsion["rows"], certificate["rows"]):
        assert count_row["id"] == proof_row["id"]
        p0 = count_row["good_prime_counts"][0]
        assert proof_row["initial_good_prime"] == p0["p"]
        assert proof_row["initial_group_order"] == p0["group_order"]
        candidates = {p0["p"]}
        for n in range(2, p0["group_order"] + 1):
            if p0["group_order"] % n == 0 and all(n % d for d in range(2, math.isqrt(n)+1)):
                candidates.add(n)
        assert set(proof_row["candidate_torsion_primes"]) == candidates
        for exclusion in proof_row["exclusions"]:
            ell = exclusion["torsion_prime"]
            assert ell in candidates and ell != exclusion["reduction_prime"]
            recorded, = (x for x in count_row["good_prime_counts"]
                         if x["p"] == exclusion["reduction_prime"])
            assert exclusion["reduced_group_order"] == recorded["group_order"]
            assert recorded["group_order"] % ell != 0
        assert {x["torsion_prime"] for x in proof_row["exclusions"]} == candidates
    assert [x["id"] for x in ranks["rows"]] == protocol["curve_ids"]
    assert all(x["status"] in ("complete", "timeout", "error") for x in ranks["rows"])
    proof_protocol = load("proof-protocol.json")
    for name, expected in proof_protocol["source_sha256"].items():
        assert digest(ROOT / name) == expected
    assert digest(ROOT / "rank-result.json") == proof_protocol["first_rank_result_sha256"]
    assert digest(ROOT / "height-result.json") == proof_protocol["first_height_result_sha256"]
    assert [(x["id"], x["algorithm"]) for x in proofs["rows"]] == [tuple(x) for x in proof_protocol["arms"]]
    effort_protocol = load("effort-protocol.json")
    for name, expected in effort_protocol["source_sha256"].items():
        assert digest(ROOT / name) == expected
    assert digest(ROOT / "rank-proof-result.json") == effort_protocol["rank_proof_result_sha256"]
    simon_protocol = load("simon-protocol.json")
    assert digest(ROOT / "simon_c2_witness_sage.py") == simon_protocol["source_sha256"]
    assert digest(ROOT / "c2-effort-result.json") == simon_protocol["effort_result_sha256"]
    perturb_protocol = load("perturb-protocol.json")
    assert digest(ROOT / "simon_perturb_sage.py") == perturb_protocol["source_sha256"]
    assert digest(ROOT / "simon-c2-result.json") == perturb_protocol["simon_witness_result_sha256"]
    assert [x["arm"] for x in perturb["rows"]] == ["none", "one", "two", "four_with_inverses"]
    print(json.dumps({"status": "PASS", "schema": "xedni-selected-height-audit-v1",
                      "source_rows": row_count, "accepted_pairs": 20,
                      "curves": summaries,
                      "rank_intervals": [{"id": x["id"], "status": x["status"],
                                          "result": x["result"]} for x in ranks["rows"]],
                      "proof_arm_statuses": [{"id": x["id"], "algorithm": x["algorithm"],
                                              "status": x["status"]} for x in proofs["rows"]],
                      "c2_effort_status": effort["row"]["status"],
                      "torsion_coprime_pairs": [
                          {"id": x["id"], "primes": x["first_coprime_pair"],
                           "orders": x["orders"]} for x in torsion["rows"]],
                      "torsion_exclusion_status": [
                          {"id": x["id"], "candidate_primes": x["candidate_torsion_primes"],
                           "excluded": len(x["exclusions"])} for x in certificate["rows"]],
                      "c2_simon_perturb_intervals": [
                          [x["arm"], x.get("rank_lower"), x.get("rank_upper")]
                          for x in perturb["rows"]]}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
