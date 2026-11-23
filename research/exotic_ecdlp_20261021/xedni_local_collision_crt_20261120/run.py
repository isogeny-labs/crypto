"""Frozen one-extra-digit CRT census of auxiliary x-coordinate collisions."""

import gzip
import hashlib
import importlib.util
import itertools
import json
import sys
import time
from collections import Counter
from fractions import Fraction
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PRIOR = ROOT / "xedni_full_law_crt_20261120"
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
spec = importlib.util.spec_from_file_location("prior_full_law", PRIOR / "run.py")
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def packed_lines(rows):
    raw = b"".join((json.dumps(r, sort_keys=True, separators=(",", ":")) + "\n").encode() for r in rows)
    return raw, gzip.compress(raw, mtime=0)


def residue(value, p):
    value = Fraction(value)
    if value.denominator % p == 0:
        return None
    return value.numerator * pow(value.denominator, -1, p) % p


@lru_cache(maxsize=None)
def collided_gate(points, p, target):
    (x1, y1), (x2, y2) = points
    assert x1 == x2
    kept = []
    counts = Counter()
    if y1 != y2:
        iterator = ((a1, a2, (-y1-y2-a1*x1) % p, a4)
                    for a1, a2, a4 in itertools.product(range(p), repeat=3))
    else:
        iterator = itertools.product(range(p), repeat=4)
    for a1, a2, a3, a4 in iterator:
        a6 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1-a4*x1) % p
        a = (a1, a2, a3, a4, a6)
        counts["residues"] += 1
        for x, y in points:
            assert (y*y+a1*x*y+a3*y-x**3-a2*x*x-a4*x-a6) % p == 0
        if prior.discriminant(a) % p == 0:
            counts["singular"] += 1
            continue
        counts["smooth"] += 1
        if prior.curve_order(a, p) == target:
            kept.append(a)
            counts["gate"] += 1
    return tuple(kept), dict(counts)


def corrected_candidate(points, a7, a11, p):
    k = [prior.centered_k(a7[i], a11[i]) for i in range(3)]
    base = prior.rational_curve(points, k)
    base_a4 = residue(base[3], p)
    assert base_a4 is not None
    expected = (a7 if p == 7 else a11)[3]
    pivot, shift = None, 0
    if base_a4 != expected:
        for i in range(3):
            trial = k.copy()
            trial[i] += 77
            new_a4 = residue(prior.rational_curve(points, trial)[3], p)
            assert new_a4 is not None
            slope = (new_a4-base_a4) % p
            if slope:
                pivot = i
                shift = ((expected-base_a4)*pow(slope, -1, p)) % p
                if shift > p//2:
                    shift -= p
                k[i] += 77*shift
                break
        if pivot is None:
            return None
    a = prior.rational_curve(points, k)
    assert prior.discriminant(a) != 0
    for pp, local in ((7, a7), (11, a11)):
        assert tuple(residue(v, pp) for v in a) == local
    assert tuple(residue(v, 47) for v in a) == (0, 0, 0, 46, 45)
    return k, a, pivot, shift


def main():
    started = time.monotonic()
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    caps = protocol["caps"]
    rows, candidates, positives = [], [], []
    totals = Counter()
    selected = 0
    for index, old in enumerate(source):
        points = [tuple(P) for P in old["lifted_points"]]
        (x1, y1), (x2, y2) = points
        if x1 == x2 or ((x1-x2)%7 != 0 and (x1-x2)%11 != 0):
            continue
        selected += 1
        p = 7 if (x1-x2)%7 == 0 else 11
        row = {"pair_index": index, "collision_prime": p,
               "collision_kind": "same_point" if (y1-y2)%p == 0 else "opposite_y",
               "reduced_relation": bool(old["reduced_relation"]),
               "status": None, "local_counts": {}, "local_pairs": 0,
               "processed_local_pairs": 0, "constructed": 0, "no_extra_digit": 0,
               "rational_relations": 0, "first_rational_relation": None}
        if time.monotonic()-started > caps["wall_seconds"]:
            row["status"] = "censored_wall"
        elif totals["processed_local_pairs"] >= caps["crt_local_pairs_total"]:
            row["status"] = "censored_total_cap"
        else:
            local = {}
            for pp, order in ((7, 4), (11, 6)):
                pp_points = tuple(tuple(v%pp for v in P) for P in points)
                local[pp], count = (collided_gate(pp_points, pp, order)
                                    if pp == p else prior.local_gate(pp_points, pp, order))
                row["local_counts"][str(pp)] = count
            w7, w11 = local[7], local[11]
            row["local_pairs"] = len(w7)*len(w11)
            if not row["local_pairs"]:
                row["status"] = "no_local_gate"
            else:
                row["status"] = "constructed"
                for i7, a7 in enumerate(w7):
                    for i11, a11 in enumerate(w11):
                        if row["processed_local_pairs"] >= caps["crt_local_pairs_per_source_pair"]:
                            row["status"] = "censored_pair_cap"
                            break
                        if totals["processed_local_pairs"] >= caps["crt_local_pairs_total"]:
                            row["status"] = "censored_total_cap"
                            break
                        if time.monotonic()-started > caps["wall_seconds"]:
                            row["status"] = "censored_wall"
                            break
                        row["processed_local_pairs"] += 1
                        totals["processed_local_pairs"] += 1
                        built = corrected_candidate(points, a7, a11, p)
                        if built is None:
                            row["no_extra_digit"] += 1
                            continue
                        k, a, pivot, shift = built
                        rational = bool(prior.relation([tuple(map(Fraction, P)) for P in points], a))
                        candidate = {"pair_index": index, "local_indices": [i7, i11],
                                     "k": k, "pivot": pivot, "shift": shift,
                                     "local_7": a7, "local_11": a11,
                                     "coefficients": list(map(str, a)),
                                     "rational_relation": rational}
                        candidates.append(candidate)
                        row["constructed"] += 1
                        row["rational_relations"] += int(rational)
                        if rational:
                            positives.append(candidate)
                            if row["first_rational_relation"] is None:
                                row["first_rational_relation"] = candidate
                    if row["status"] != "constructed":
                        break
        totals["pairs"] += 1
        totals["status_"+row["status"]] += 1
        totals["constructed"] += row["constructed"]
        totals["no_extra_digit"] += row["no_extra_digit"]
        totals["rational_relations"] += row["rational_relations"]
        totals["reduced_event_pairs"] += int(old["reduced_relation"])
        rows.append(row)
        if selected%50 == 0:
            print(f"processed {selected}/336 collision pairs", file=sys.stderr, flush=True)
    assert selected == 336
    raw, packed = packed_lines(rows)
    craw, cpacked = packed_lines(candidates)
    praw, ppacked = packed_lines(positives)
    for name, data in (("pair-rows.jsonl.gz", packed),
                       ("candidate-rows.jsonl.gz", cpacked),
                       ("positive-rows.jsonl.gz", ppacked)):
        (HERE / name).write_bytes(data)
    print(json.dumps({"schema": "xedni-local-collision-crt-result-v1",
                      "protocol_sha256": sha(protocol_bytes), "source_sha256": sha(source_bytes),
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "pair_rows_raw_sha256": sha(raw), "pair_rows_packed_sha256": sha(packed),
                      "candidate_rows_raw_sha256": sha(craw), "candidate_rows_packed_sha256": sha(cpacked),
                      "positive_rows_raw_sha256": sha(praw), "positive_rows_packed_sha256": sha(ppacked),
                      "counts": dict(totals),
                      "status": "censored" if any(r["status"].startswith("censored") for r in rows) else "complete",
                      "elapsed_s": round(time.monotonic()-started, 3)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
