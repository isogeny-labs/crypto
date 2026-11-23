#!/usr/bin/env python3
"""Full-law xedni local selector with order-5 and order-11 gates."""

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
from audit import add, discriminant

REDUCED = [0, 0, 0, 46, 45]
DOMAIN = range(1, 42)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def local_log(points, curve, p, order, counts):
    target, generator = points
    acc = None
    found = []
    for k in range(order):
        counts["local_relation_tests"] += 1
        if add(target, acc, curve, p) is None:
            found.append(k)
        if k != order-1:
            counts["local_generator_steps"] += 1
            acc = add(acc, generator, curve, p)
    assert len(found) == 1
    return found[0]


def reduced_log(points, counts):
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


def local_crt(k7, k11):
    k = (k7 + 5 * ((k11-k7)*pow(5, -1, 11) % 11)) % 55
    assert k % 5 == k7 and k % 11 == k11
    return k


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_packed_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    counts = Counter()
    status_counts = Counter()
    rows = []
    candidates = []
    union_hist = Counter()
    for index, old in enumerate(source):
        points = [tuple(P) for P in old["lifted_points"]]
        (x1, _), (x2, _) = points
        status = None
        union = set()
        pair_candidates = 0
        selected_candidates = 0
        local_counts = {}
        if x1 == x2:
            status = "same_x_q"
        elif (x1-x2)%7 == 0:
            status = "x_collision_7"
        elif (x1-x2)%11 == 0:
            status = "x_collision_11"
        else:
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            if crt.point_rank(p7, 7) < 2:
                status = "rank_lt2_7"
            elif crt.point_rank(p11, 11) < 2:
                status = "rank_lt2_11"
            else:
                w7, c7 = crt.local_gate(p7, 7, 5)
                w11, c11 = crt.local_gate(p11, 11, 11)
                local_counts = {"7": c7, "11": c11}
                counts["local_residues_7"] += c7["residues"]
                counts["local_residues_11"] += c11["residues"]
                counts["local_gate_witnesses_7"] += len(w7)
                counts["local_gate_witnesses_11"] += len(w11)
                if not w7:
                    status = "no_gate_7"
                elif not w11:
                    status = "no_gate_11"
                else:
                    status = "constructed"
                    r7 = [local_log(p7, a, 7, 5, counts) for a in w7]
                    r11 = [local_log(p11, a, 11, 11, counts) for a in w11]
                    counts["local_curves_tested_7"] += len(w7)
                    counts["local_curves_tested_11"] += len(w11)
                    for i7, a7 in enumerate(w7):
                        for i11, a11 in enumerate(w11):
                            shifts = [crt.centered_k(a7[j], a11[j]) for j in range(3)]
                            a = crt.rational_curve(points, shifts)
                            assert discriminant(a) != 0
                            k = local_crt(r7[i7], r11[i11])
                            selected = k in DOMAIN
                            pair_candidates += 1
                            selected_candidates += selected
                            counts["crt_candidates"] += 1
                            counts["selected_crt_candidates"] += selected
                            if selected:
                                union.add(k)
                            candidates.append(json.dumps({"pair_index": index,
                                "local_indices": [i7, i11], "crt_shifts": shifts,
                                "local_coefficient_mod_55": k,
                                "selected_coefficient": k if selected else None}, sort_keys=True) + "\n")
        true_k = reduced_log([tuple(P) for P in old["reduced_points"]], counts)
        status_counts[status] += 1
        union_hist[len(union)] += 1
        counts["pairs"] += 1
        counts["pairs_with_selected_coefficient"] += bool(union)
        counts["pairs_covering_true_coefficient"] += true_k in union
        counts["selected_coefficient_incidence"] += len(union)
        rows.append(json.dumps({"pair_index": index, "status": status,
            "local_counts": local_counts, "crt_candidates": pair_candidates,
            "selected_crt_candidates": selected_candidates,
            "selected_coefficients": sorted(union),
            "true_coefficient": true_k,
            "true_coefficient_selected": true_k in union}, sort_keys=True) + "\n")
    raw_rows = "".join(rows).encode()
    raw_candidates = "".join(candidates).encode()
    (HERE / "pair-rows.jsonl.gz").write_bytes(gzip.compress(raw_rows, mtime=0))
    (HERE / "candidate-rows.jsonl.gz").write_bytes(gzip.compress(raw_candidates, mtime=0))
    return {"schema": "xedni-wide-period-selector-result-v1", "status": "PASS",
        "protocol_sha256": sha(protocol_bytes),
        "script_sha256": sha((HERE / "run.py").read_bytes()),
        "source_packed_sha256": sha(source_bytes),
        "pair_rows_raw_sha256": sha(raw_rows),
        "candidate_rows_raw_sha256": sha(raw_candidates),
        "counts": dict(counts), "status_counts": dict(status_counts),
        "selected_coefficient_union_size_histogram": dict(sorted(union_hist.items()))}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
