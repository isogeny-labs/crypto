#!/usr/bin/env python3
"""Independent arithmetic and artifact checks for the N11 derivation."""
import csv
import hashlib
import json
import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "orbit-proof" / "frozen-inputs.json"
WALK_SOURCE = HERE.parent / "src" / "walk_impl.rs"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def kummer(x, r):
    return min(x % r, -x % r)


def pair(x, g, r):
    return kummer(x, r), kummer(x + g, r)


def check_small_groups():
    # Exact finite checks illustrate the general proof, including identity.
    for r in (3, 5, 7, 11, 13, 17, 19, 23, 29, 31):
        for g in range(1, r):
            assert len({pair(x, g, r) for x in range(r)}) == r
            for t in range(r):
                for word in range(256):
                    a, d = 0, t
                    for n in range(8):
                        c = (word >> (7 - n)) & 1
                        a = (2 * a + c) % r
                        d = (2 * d + c * g) % r
                        assert d == (a * g + pow(2, n + 1, r) * t) % r
    # The exact obstruction when 2G=0: adding G does not break the sign.
    assert pair(1, 2, 4) == pair(3, 2, 4)


def main():
    frozen = json.loads(SOURCE.read_text())
    packet = json.loads((HERE / "results/bounds.json").read_text())
    assert packet["input_sha256"] == sha(SOURCE)
    assert packet["walk_source_sha256"] == sha(WALK_SOURCE)
    rows = list(csv.DictReader((HERE / "results/bounds.csv").open(newline="")))
    assert len(rows) == len(packet["rows"]) == 336
    assert len(frozen["models"]) == 21
    for i, (row, source) in enumerate(zip(rows, packet["rows"])):
        model, jump = frozen["models"][i // 16], frozen["models"][i // 16]["jumps"][i % 16]
        r = model["r"]
        assert r & 1 and math.gcd(r, jump["jump_scalar"]) == 1
        assert int(row["bits"]) == source["bits"] == model["bits"]
        assert int(row["p"]) == source["p"] == model["p"]
        assert int(row["r"]) == source["r"] == r
        assert int(row["jump_scalar"]) == source["jump_scalar"] == jump["jump_scalar"]
        assert int(row["fixed_offset_explicit_bytes"]) == source["fixed_offset_explicit_bytes"] == 32 * r
        assert int(row["pair_plus_label_bytes_external_step"]) == source["pair_plus_label_bytes_external_step"] == 72
        assert int(row["pair_plus_label_and_step_bytes"]) == source["pair_plus_label_and_step_bytes"] == 80
        assert int(row["minimum_label_bits_for_all_histories"]) == source["minimum_label_bits_for_all_histories"] == (r - 1).bit_length()
        assert int(row["regular_pair_step_field_m"]) == source["regular_pair_step_field_m"] == 3 * 24
        assert int(row["regular_pair_step_field_a"]) == source["regular_pair_step_field_a"] == 3 * 36
        assert int(row["pair_sign_ambiguities"]) == 0
        assert row["coordinate_xadd_exception_bound"] == ""
        assert row["cohort_eligible"] == "False"
    assert packet["coordinate_xadd_exception_bound"] is None
    assert packet["cohort_eligible"] is False
    check_small_groups()
    for line in (HERE / "SHA256SUMS").read_text().splitlines():
        expected, name = line.split("  ", 1)
        assert sha(HERE / name) == expected, name
    print(json.dumps({"schema": "kummer-compressed-sidecar-validation/v1", "rows": len(rows),
                      "model_count": len(frozen["models"]), "small_group_checks": "pass",
                      "source_digest": sha(SOURCE), "checksums": "pass", "cohort_eligible": False}))


if __name__ == "__main__":
    main()
