#!/usr/bin/env python3
"""Independent Sage replay of every local selector row."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_full_law_crt_20261120"))
import run as crt


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def residues(points, a, p):
    E = EllipticCurve(GF(p), list(a))
    P, Q = [E([int(x), int(y)]) for x, y in points]
    acc = E(0)
    found = []
    for k in range(12):
        if P + acc == E(0):
            found.append(k)
        acc += Q
    return set(found)


def true_coefficient(points):
    E = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    T, G = [E([int(x), int(y)]) for x, y in points]
    hits = [k for k in range(1, 42) if T + k*G == E(0)]
    assert len(hits) == 1
    return hits[0]


def main():
    result = json.loads((HERE / "result.json").read_text())
    source_raw = gzip.decompress((PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes())
    prior_raw = gzip.decompress((PARENT / "xedni_full_law_crt_20261120/pair-rows.jsonl.gz").read_bytes())
    saved_pair_raw = gzip.decompress((HERE / "pair-rows.jsonl.gz").read_bytes())
    saved_candidate_raw = gzip.decompress((HERE / "candidate-rows.jsonl.gz").read_bytes())
    assert sha(saved_pair_raw) == result["pair_rows_raw_sha256"]
    assert sha(saved_candidate_raw) == result["candidate_rows_raw_sha256"]
    source = [json.loads(x) for x in source_raw.splitlines()]
    prior = [json.loads(x) for x in prior_raw.splitlines()]
    saved_pairs = [json.loads(x) for x in saved_pair_raw.splitlines()]
    saved_candidates = [json.loads(x) for x in saved_candidate_raw.splitlines()]
    assert len(source) == len(prior) == len(saved_pairs) == 1722
    counts = Counter()
    cursor = 0
    for index, (old, prior_row, row) in enumerate(zip(source, prior, saved_pairs)):
        points = old["lifted_points"]
        union = set()
        candidate_count = 0
        positive_count = 0
        if prior_row["status"] == "constructed":
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            w7, _ = crt.local_gate(p7, 7, 4)
            w11, _ = crt.local_gate(p11, 11, 6)
            r7 = [residues(p7, a, 7) for a in w7]
            r11 = [residues(p11, a, 11) for a in w11]
            for i7, a in enumerate(r7):
                for i11, b in enumerate(r11):
                    expected = sorted(a & b)
                    saved = saved_candidates[cursor]
                    assert saved == {"pair_index": index, "local_indices": [i7, i11],
                                     "residues": expected}, cursor
                    cursor += 1
                    candidate_count += 1
                    positive_count += bool(expected)
                    union.update(expected)
        assert candidate_count == prior_row["crt_candidates"] == row["crt_candidates"]
        assert positive_count == row["candidates_with_local_residue"]
        assert sorted(union) == row["local_residue_union"]
        true_k = true_coefficient(old["reduced_points"])
        assert true_k == row["true_coefficient"]
        selected = [k for k in range(1, 42) if k%12 in union]
        assert len(selected) == row["selected_coefficient_count"]
        assert (true_k in selected) == row["true_coefficient_in_local_union"]
        counts["pairs"] += 1
        counts["candidates"] += candidate_count
        counts["candidates_with_local_residue"] += positive_count
        counts["pairs_covering_true_coefficient"] += true_k in selected
    assert cursor == len(saved_candidates) == 32256
    assert counts["pairs_covering_true_coefficient"] == result["counts"]["pairs_covering_true_coefficient"]
    return {"status": "PASS", "counts": dict(counts),
            "pair_rows_raw_sha256": sha(saved_pair_raw),
            "candidate_rows_raw_sha256": sha(saved_candidate_raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
