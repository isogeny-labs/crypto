"""Frozen 42-pair coefficient-six local policy screen."""

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


def chunk(label, rows):
    folder = HERE / label
    folder.mkdir(exist_ok=True)
    raw = b""
    part, block = 0, b""
    for row in rows:
        line = (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode()
        assert len(line) < 48000
        raw += line
        if len(block) + len(line) > 48000:
            (folder / f"{part:03d}.jsonl").write_bytes(block)
            part, block = part + 1, b""
        block += line
    if block:
        (folder / f"{part:03d}.jsonl").write_bytes(block)
        part += 1
    return sha(raw), len(raw), part


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes, arithmetic_bytes = SOURCE.read_bytes(), ARITH.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(arithmetic_bytes) == protocol["arithmetic_sha256"]
    spec = importlib.util.spec_from_file_location("bridge6_arithmetic", ARITH)
    arithmetic = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(arithmetic)
    pairs = [json.loads(x)["lifted_points"] for x in gzip.decompress(source_bytes).splitlines()]
    assert len(pairs) == 1722
    fixed = (0, 0, 0, 46, 45)
    selected = []
    for i, pair in enumerate(pairs):
        P, Q = [tuple(v % 47 for v in xy) for xy in pair]
        if arithmetic.add(P, arithmetic.mul(Q, 6, fixed, 47), fixed, 47) is None:
            selected.append(i)
    attempts = []
    outcomes = []
    status_counts = Counter()
    attempt_counts = Counter()
    for index in selected:
        pair = pairs[index]
        first = {}
        local_counts = Counter()
        for prime, order, bound in ((7, 4, 3), (11, 8, 5)):
            for k in itertools.product(range(-bound, bound + 1), repeat=3):
                coeffs = arithmetic.coefficients(pair, k)
                if coeffs is None:
                    result = {"status": "same_x"}
                elif arithmetic.discriminant(coeffs) == 0:
                    result = {"status": "singular"}
                else:
                    result = arithmetic.local(coeffs, pair, prime, order)
                attempts.append({"pair_index": index, "prime": prime, "k": list(k), "local": result})
                local_counts[f"p{prime}_{result['status']}"] += 1
                attempt_counts[f"p{prime}_{result['status']}"] += 1
                if result["status"] == "pass" and prime not in first:
                    first[prime] = list(k)
        row = {"pair_index": index, "points": pair,
               "first_pass": {str(p): k for p, k in first.items()},
               "local_counts": dict(sorted(local_counts.items()))}
        if sorted(first) == [7, 11]:
            centered = []
            for k7, k11 in zip(first[7], first[11]):
                x = next(x for x in range(77) if x % 7 == k7 % 7 and x % 11 == k11 % 11)
                centered.append(x if x <= 38 else x - 77)
            coeffs = arithmetic.coefficients(pair, centered)
            assert coeffs is not None and arithmetic.discriminant(coeffs) != 0
            p7 = arithmetic.local(coeffs, pair, 7, 4)
            p11 = arithmetic.local(coeffs, pair, 11, 8)
            assert p7["status"] == p11["status"] == "pass"
            from fractions import Fraction as F
            P, Q = [(F(x), F(y)) for x, y in pair]
            rational = arithmetic.add(P, arithmetic.mul(Q, 6, coeffs), coeffs) is None
            row.update({"status": "local_bridge", "crt_k": centered,
                        "crt_coefficients": list(map(str, coeffs)),
                        "crt_local_7": p7, "crt_local_11": p11,
                        "crt_rational_relation": rational})
        else:
            row["status"] = "no_local_bridge_in_boxes"
        outcomes.append(row)
        status_counts[row["status"]] += 1
    attempt_sha, attempt_bytes, attempt_parts = chunk("attempts", attempts)
    outcome_sha, outcome_bytes, outcome_parts = chunk("outcomes", outcomes)
    print(json.dumps({"schema": "xedni-bridge6-source-policy-result-v1",
                      "status": "PASS", "source_pairs": len(pairs),
                      "reduced_compatible": len(selected), "selected_indices": selected,
                      "attempts": len(attempts), "attempt_counts": dict(sorted(attempt_counts.items())),
                      "outcome_counts": dict(sorted(status_counts.items())),
                      "crt_rational_relations": sum(bool(x.get("crt_rational_relation")) for x in outcomes),
                      "attempts_raw_sha256": attempt_sha, "attempts_bytes": attempt_bytes,
                      "attempts_parts": attempt_parts,
                      "outcomes_raw_sha256": outcome_sha, "outcomes_bytes": outcome_bytes,
                      "outcomes_parts": outcome_parts,
                      "source_sha256": sha(source_bytes)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
