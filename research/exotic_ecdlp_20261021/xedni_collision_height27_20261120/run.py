"""Frozen 27-representative CRT height census on six eligible collision pairs."""

import gzip
import hashlib
import importlib.util
import itertools
import json
import sys
import time
from collections import Counter
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
BASE = ROOT / "xedni_local_collision_crt_20261120" / "candidate-rows.jsonl.gz"
PRIOR = ROOT / "xedni_full_law_crt_20261120" / "run.py"
spec = importlib.util.spec_from_file_location("full_law", PRIOR)
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def packed_lines(rows):
    raw = b"".join((json.dumps(r, sort_keys=True, separators=(",", ":")) + "\n").encode()
                   for r in rows)
    return raw, gzip.compress(raw, mtime=0)


def residue(x, p):
    x = Fraction(x)
    return None if x.denominator % p == 0 else x.numerator * pow(x.denominator, -1, p) % p


def height_bits(a):
    return max(max(abs(x.numerator).bit_length(), x.denominator.bit_length())
               for x in map(Fraction, a))


def on_curve(point, a):
    x, y = map(Fraction, point)
    a1, a2, a3, a4, a6 = a
    return y*y+a1*x*y+a3*y == x*x*x+a2*x*x+a4*x+a6


def main():
    started = time.monotonic()
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = SOURCE.read_bytes()
    base_bytes = BASE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(base_bytes) == protocol["base_candidates_packed_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    bases = [json.loads(line) for line in gzip.decompress(base_bytes).splitlines()
             if json.loads(line)["pair_index"] in protocol["selected_source_indices"]]
    assert len(source) == 1722 and len(bases) == protocol["caps"]["base_rows"] == 600
    assert sorted({b["pair_index"] for b in bases}) == protocol["selected_source_indices"]
    rows, positives, counts = [], [], Counter()
    per_pair = Counter()
    for base_index, base in enumerate(bases):
        index = base["pair_index"]
        points = [tuple(P) for P in source[index]["lifted_points"]]
        (x1, _), (x2, _) = points
        p = 7 if (x1-x2) % 7 == 0 else 11
        assert (x1-x2) % p == 0 and (x1-x2) % (p*p) != 0
        assert base["rational_relation"] is False
        step = 77*p
        for shift in itertools.product((-1, 0, 1), repeat=3):
            if len(rows) >= protocol["caps"]["total_candidates"]:
                counts["censored_total"] += 1
                break
            if time.monotonic()-started > protocol["caps"]["wall_seconds"]:
                counts["censored_wall"] += 1
                break
            k = [base["k"][i]+step*shift[i] for i in range(3)]
            a = prior.rational_curve(points, k)
            assert all(on_curve(P, a) for P in points)
            status = "constructed"
            if prior.discriminant(a) == 0:
                status = "invalid_discriminant"
            elif any(tuple(residue(z, q) for z in a) != tuple(base[f"local_{q}"])
                     for q in (7, 11)) or tuple(residue(z, 47) for z in a) != (0, 0, 0, 46, 45):
                status = "invalid_reduction"
            rational = bool(prior.relation([tuple(map(Fraction, P)) for P in points], a)) if status == "constructed" else None
            row = {"base_index": base_index, "pair_index": index, "collision_prime": p,
                   "local_indices": base["local_indices"], "shift": shift, "step": step,
                   "k": k, "coefficients": list(map(str, a)), "height_bits": height_bits(a),
                   "status": status, "rational_relation": rational}
            rows.append(row)
            counts[status] += 1
            counts["relations"] += int(rational is True)
            per_pair[str(index)] += 1
            if rational:
                positives.append(row)
            if shift == (0, 0, 0):
                assert row["coefficients"] == base["coefficients"]
                assert rational is False
        if counts["censored_total"] or counts["censored_wall"]:
            break
        if (base_index+1) % 100 == 0:
            print(f"processed {base_index+1}/600 base rows", file=sys.stderr, flush=True)
    raw, packed = packed_lines(rows)
    praw, ppacked = packed_lines(positives)
    (HERE / "candidate-rows.jsonl.gz").write_bytes(packed)
    (HERE / "positive-rows.jsonl.gz").write_bytes(ppacked)
    print(json.dumps({"schema": "xedni-collision-height27-result-v1",
                      "protocol_sha256": sha(protocol_bytes),
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "source_sha256": sha(source_bytes),
                      "base_candidates_packed_sha256": sha(base_bytes),
                      "candidate_rows_raw_sha256": sha(raw),
                      "candidate_rows_packed_sha256": sha(packed),
                      "positive_rows_raw_sha256": sha(praw),
                      "positive_rows_packed_sha256": sha(ppacked),
                      "counts": dict(counts), "per_pair_candidates": dict(per_pair),
                      "max_height_bits": max((r["height_bits"] for r in rows), default=None),
                      "status": "censored" if counts["censored_wall"] or counts["censored_total"] else "complete",
                      "elapsed_s": round(time.monotonic()-started, 3)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
