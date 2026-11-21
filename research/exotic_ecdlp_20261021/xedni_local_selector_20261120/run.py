#!/usr/bin/env python3
"""Measure coefficient ambiguity of the frozen auxiliary-prime selector."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
sys.path.insert(0, str(PARENT / "xedni_full_law_crt_20261120"))
import run as crt
sys.path.insert(0, str(PARENT / "xedni_coefficient_choice_20261120"))
from audit import add

REDUCED = [0, 0, 0, 46, 45]
DOMAIN = tuple(range(1, 42))


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def local_residues(points, curve, p, counts):
    target, generator = points
    acc = None
    found = []
    for k in range(12):
        counts["local_relation_tests"] += 1
        if add(target, acc, curve, p) is None:
            found.append(k)
        if k != 11:
            counts["local_generator_steps"] += 1
            acc = add(acc, generator, curve, p)
    return set(found)


def reduced_coefficient(points, counts):
    target, generator = points
    acc = None
    found = []
    for k in range(43):
        counts["analysis_reduced_relation_tests"] += 1
        if add(target, acc, REDUCED, 47) is None:
            found.append(k)
        if k != 42:
            counts["analysis_reduced_generator_steps"] += 1
            acc = add(acc, generator, REDUCED, 47)
    assert len(found) == 1 and found[0] in DOMAIN
    return found[0]


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    pair_raw = gzip.decompress((PARENT / "xedni_full_law_crt_20261120/pair-rows.jsonl.gz").read_bytes())
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    assert sha(pair_raw) == protocol["full_law_pair_rows_raw_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    old_pairs = [json.loads(x) for x in pair_raw.splitlines()]
    assert len(source) == len(old_pairs) == 1722
    counts = Counter()
    row_bytes = []
    candidate_bytes = []
    pair_residue_hist = Counter()
    pair_candidate_count_hist = Counter()
    for index, (old, prior) in enumerate(zip(source, old_pairs)):
        assert prior["pair_index"] == index
        points = [tuple(P) for P in old["lifted_points"]]
        reduced_points = [tuple(P) for P in old["reduced_points"]]
        union = set()
        candidate_count = 0
        candidates_with_residue = 0
        if prior["status"] == "constructed":
            p7 = tuple(tuple(v % 7 for v in P) for P in points)
            p11 = tuple(tuple(v % 11 for v in P) for P in points)
            w7, _ = crt.local_gate(p7, 7, 4)
            w11, _ = crt.local_gate(p11, 11, 6)
            r7 = [local_residues(p7, a, 7, counts) for a in w7]
            r11 = [local_residues(p11, a, 11, counts) for a in w11]
            counts["local_curves_7"] += len(w7)
            counts["local_curves_11"] += len(w11)
            for i7, a in enumerate(r7):
                for i11, b in enumerate(r11):
                    residue_set = sorted(a & b)
                    union.update(residue_set)
                    candidate_count += 1
                    candidates_with_residue += bool(residue_set)
                    candidate_bytes.append(json.dumps({"pair_index": index,
                        "local_indices": [i7, i11], "residues": residue_set},
                        sort_keys=True) + "\n")
        assert candidate_count == prior["crt_candidates"]
        true_k = reduced_coefficient(reduced_points, counts)
        selected_k = [k for k in DOMAIN if k % 12 in union]
        row = {"pair_index": index, "status": prior["status"],
               "crt_candidates": candidate_count,
               "candidates_with_local_residue": candidates_with_residue,
               "local_residue_union": sorted(union),
               "selected_coefficient_count": len(selected_k),
               "true_coefficient": true_k,
               "true_coefficient_in_local_union": true_k in selected_k}
        row_bytes.append(json.dumps(row, sort_keys=True) + "\n")
        counts["pairs"] += 1
        counts["crt_candidates"] += candidate_count
        counts["candidates_with_local_residue"] += candidates_with_residue
        counts["pairs_with_local_residue"] += bool(union)
        counts["pairs_covering_true_coefficient"] += true_k in selected_k
        counts["total_selected_coefficients_across_pairs"] += len(selected_k)
        pair_residue_hist[len(union)] += 1
        pair_candidate_count_hist[len(selected_k)] += 1
    assert counts["crt_candidates"] == 32256
    assert counts["local_curves_7"] == 4032
    assert counts["local_curves_11"] == 10752
    raw_rows = "".join(row_bytes).encode()
    raw_candidates = "".join(candidate_bytes).encode()
    (HERE / "pair-rows.jsonl.gz").write_bytes(gzip.compress(raw_rows, mtime=0))
    (HERE / "candidate-rows.jsonl.gz").write_bytes(gzip.compress(raw_candidates, mtime=0))
    return {"schema": "xedni-local-selector-result-v1", "status": "PASS",
            "protocol_sha256": sha(protocol_bytes),
            "script_sha256": sha((HERE / "run.py").read_bytes()),
            "source_packed_sha256": sha(source_bytes),
            "full_law_pair_rows_raw_sha256": sha(pair_raw),
            "pair_rows_raw_sha256": sha(raw_rows),
            "candidate_rows_raw_sha256": sha(raw_candidates),
            "counts": dict(counts),
            "pair_residue_union_size_histogram": dict(sorted(pair_residue_hist.items())),
            "pair_selected_coefficient_count_histogram": dict(sorted(pair_candidate_count_hist.items()))}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
