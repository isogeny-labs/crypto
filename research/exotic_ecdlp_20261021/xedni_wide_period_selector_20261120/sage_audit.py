#!/usr/bin/env python3
"""Independent local gate enumeration and Sage selector replay."""

import gzip
import hashlib
import importlib.util
import itertools
import json
from collections import Counter
from functools import lru_cache
from pathlib import Path

from sage.all import EllipticCurve, GF

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
CHECK_PATH = PARENT / "xedni_local_residue_20261120/audit.py"
spec = importlib.util.spec_from_file_location("local_arithmetic_audit", CHECK_PATH)
CHECK = importlib.util.module_from_spec(spec)
spec.loader.exec_module(CHECK)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


@lru_cache(maxsize=None)
def gate(points, p, wanted):
    (x1, y1), (x2, y2) = points
    kept = []
    counts = Counter()
    inv = pow((x1-x2)%p, -1, p)
    for a1, a2, a3 in itertools.product(range(p), repeat=3):
        d1 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1) % p
        d2 = (y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2) % p
        a4 = (d1-d2)*inv % p
        a6 = (d1-a4*x1) % p
        a = (a1, a2, a3, a4, a6)
        counts["residues"] += 1
        if not CHECK.smooth(a, p):
            counts["singular"] += 1
            continue
        counts["smooth"] += 1
        if CHECK.order_character_sum(a, p) == wanted:
            kept.append(a)
            counts["gate"] += 1
    return tuple(kept), dict(counts)


def local_log(points, a, p, order):
    E = EllipticCurve(GF(p), list(a))
    P, Q = [E([int(x), int(y)]) for x, y in points]
    acc = E(0)
    hits = []
    for k in range(order):
        if P + acc == E(0):
            hits.append(k)
        acc += Q
    assert len(hits) == 1
    return hits[0]


def true_log(points):
    E = EllipticCurve(GF(47), [0, 0, 0, 46, 45])
    T, G = [E([int(x), int(y)]) for x, y in points]
    hits = [k for k in range(1, 42) if T + k*G == E(0)]
    assert len(hits) == 1
    return hits[0]


def centered(a7, a11):
    u = a7 * pow(47, -1, 7) % 7
    v = a11 * pow(47, -1, 11) % 11
    shift = (u + 7*((v-u)*pow(7, -1, 11)%11)) % 77
    return shift if shift < 39 else shift-77


def main():
    result = json.loads((HERE / "result.json").read_text())
    source = [json.loads(x) for x in gzip.decompress(
        (PARENT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz").read_bytes()
    ).splitlines()]
    pair_raw = gzip.decompress((HERE / "pair-rows.jsonl.gz").read_bytes())
    candidate_raw = gzip.decompress((HERE / "candidate-rows.jsonl.gz").read_bytes())
    assert sha(pair_raw) == result["pair_rows_raw_sha256"]
    assert sha(candidate_raw) == result["candidate_rows_raw_sha256"]
    rows = [json.loads(x) for x in pair_raw.splitlines()]
    candidates = [json.loads(x) for x in candidate_raw.splitlines()]
    assert len(source) == len(rows) == 1722
    counts = Counter()
    cursor = 0
    for index, (old, saved) in enumerate(zip(source, rows)):
        points = old["lifted_points"]
        (x1, _), (x2, _) = points
        status = None
        local_counts = {}
        union = set()
        selected = 0
        candidate_count = 0
        if x1 == x2:
            status = "same_x_q"
        elif (x1-x2)%7 == 0:
            status = "x_collision_7"
        elif (x1-x2)%11 == 0:
            status = "x_collision_11"
        else:
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            if CHECK.incidence_rank(p7, 7) < 2:
                status = "rank_lt2_7"
            elif CHECK.incidence_rank(p11, 11) < 2:
                status = "rank_lt2_11"
            else:
                w7, c7 = gate(p7, 7, 5)
                w11, c11 = gate(p11, 11, 11)
                local_counts = {"7": c7, "11": c11}
                if not w7:
                    status = "no_gate_7"
                elif not w11:
                    status = "no_gate_11"
                else:
                    status = "constructed"
                    r7 = [local_log(p7, a, 7, 5) for a in w7]
                    r11 = [local_log(p11, a, 11, 11) for a in w11]
                    for i7, a7 in enumerate(w7):
                        for i11, a11 in enumerate(w11):
                            k7, k11 = r7[i7], r11[i11]
                            k = (k7 + 5*((k11-k7)*pow(5,-1,11)%11))%55
                            shifts = [centered(a7[j], a11[j]) for j in range(3)]
                            expected = {"pair_index": index,
                                "local_indices": [i7, i11], "crt_shifts": shifts,
                                "local_coefficient_mod_55": k,
                                "selected_coefficient": k if 1 <= k <= 41 else None}
                            assert candidates[cursor] == expected, cursor
                            cursor += 1
                            candidate_count += 1
                            if 1 <= k <= 41:
                                selected += 1
                                union.add(k)
        assert saved["pair_index"] == index
        assert saved["status"] == status
        assert saved["local_counts"] == local_counts
        assert saved["crt_candidates"] == candidate_count
        assert saved["selected_crt_candidates"] == selected
        assert saved["selected_coefficients"] == sorted(union)
        true_k = true_log(old["reduced_points"])
        assert saved["true_coefficient"] == true_k
        assert saved["true_coefficient_selected"] == (true_k in union)
        counts["pairs"] += 1
        counts["crt_candidates"] += candidate_count
        counts["selected_crt_candidates"] += selected
        counts["pairs_covering_true_coefficient"] += true_k in union
    assert cursor == len(candidates) == 215040
    for key in counts:
        assert counts[key] == result["counts"][key], key
    return {"status": "PASS", "counts": dict(counts),
        "pair_rows_raw_sha256": sha(pair_raw),
        "candidate_rows_raw_sha256": sha(candidate_raw)}


if __name__ == "__main__":
    print(json.dumps(main(), sort_keys=True, indent=2))
