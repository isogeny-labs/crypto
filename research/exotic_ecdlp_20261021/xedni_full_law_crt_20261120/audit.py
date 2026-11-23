"""Independent complete replay of local gates, CRT lifts, and rational sums."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from fractions import Fraction as F
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
LOCAL = HERE.parent / "xedni_local_residue_20261120"


def load_local_audit():
    spec = importlib.util.spec_from_file_location("local_residue_audit", LOCAL / "audit.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CHECK = load_local_audit()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def rank(points, p):
    rows = [tuple(v%p for v in (y*y, x*y, y, x**3, x*x, x, 1)) for x, y in points]
    return 1 if rows[0] == rows[1] else 2


@lru_cache(maxsize=None)
def gate(points, p, order):
    candidates = tuple(CHECK.candidates(points, p))
    smooth = [a for a in candidates if CHECK.smooth(a, p)]
    selected = tuple(a for a in smooth if CHECK.order_character_sum(a, p) == order)
    return selected, {"residues": len(candidates), "singular": len(candidates)-len(smooth),
                      "smooth": len(smooth), "gate": len(selected)}


def centered(a7, a11):
    hits = [k for k in range(-38, 39) if (47*k-a7)%7 == 0 and (47*k-a11)%11 == 0]
    assert len(hits) == 1
    return hits[0]


def on_curve(a, point):
    x, y = point
    a1, a2, a3, a4, a6 = a
    return y*y+a1*x*y+a3*y == x**3+a2*x*x+a4*x+a6


def rational_relation(points, a):
    (x1, y1), (x, y) = points
    a1, a2, a3, a4, a6 = a
    den = 2*y+a1*x+a3
    if den == 0:
        return False
    lam = (3*x*x+2*a2*x+a4-a1*y)/den
    nu = (-x**3+a4*x+2*a6-a3*y)/den
    xd = lam*lam+a1*lam-a2-2*x
    yd = -(lam+a1)*xd-nu-a3
    return (xd, yd) == (x1, -y1-a1*x1-a3)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    pair_packed = (HERE / "pair-rows.jsonl.gz").read_bytes()
    pair_raw = gzip.decompress(pair_packed)
    positive_raw = gzip.decompress((HERE / "positive-rows.jsonl.gz").read_bytes())
    assert result["protocol_sha256"] == sha(protocol_bytes)
    assert result["source_sha256"] == protocol["source_sha256"] == sha(source_bytes)
    assert result["script_sha256"] == sha((HERE / "run.py").read_bytes())
    assert result["arithmetic_sha256"] == sha((PRIOR / "audit.py").read_bytes())
    assert result["pair_rows_packed_sha256"] == sha(pair_packed)
    assert result["pair_rows_raw_sha256"] == sha(pair_raw)
    assert result["positive_rows_raw_sha256"] == sha(positive_raw)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    saved = [json.loads(x) for x in pair_raw.splitlines()]
    assert len(source) == len(saved) == 1722
    totals = Counter()
    stream = hashlib.sha256()
    positives = []
    for index, (old, row) in enumerate(zip(source, saved)):
        points = [tuple(P) for P in old["lifted_points"]]
        (x1, y1), (x2, y2) = points
        expected = {"pair_index": index, "source_status": old["status"],
                    "reduced_relation": old["reduced_relation"],
                    "status": None, "local_counts": {}, "crt_candidates": 0,
                    "rational_relations": 0, "first_rational_relation": None}
        if x1 == x2:
            expected["status"] = "same_x_q"
        elif (x1-x2)%7 == 0:
            expected["status"] = "x_collision_7"
        elif (x1-x2)%11 == 0:
            expected["status"] = "x_collision_11"
        else:
            p7 = tuple(tuple(v%7 for v in P) for P in points)
            p11 = tuple(tuple(v%11 for v in P) for P in points)
            if rank(p7, 7) < 2:
                expected["status"] = "rank_lt2_7"
            elif rank(p11, 11) < 2:
                expected["status"] = "rank_lt2_11"
            else:
                w7, c7 = gate(p7, 7, 4)
                w11, c11 = gate(p11, 11, 6)
                assert len(w7) == c7["gate"] == 3
                assert len(w11) == c11["gate"] == 8
                expected["local_counts"] = {"7": c7, "11": c11}
                if not w7:
                    expected["status"] = "no_gate_7"
                elif not w11:
                    expected["status"] = "no_gate_11"
                else:
                    expected["status"] = "constructed"
                    Fpoints = [tuple(F(v) for v in P) for P in points]
                    for i7, a7 in enumerate(w7):
                        for i11, a11 in enumerate(w11):
                            ks = [centered(a7[i], a11[i]) for i in range(3)]
                            (u, v), (w, z) = Fpoints
                            a1, a2, a3 = [F(47*k) for k in ks]
                            d1 = v*v+a1*u*v+a3*v-u**3-a2*u*u
                            d2 = z*z+a1*w*z+a3*z-w**3-a2*w*w
                            a4 = (d1-d2)/(u-w)
                            a = [a1, a2, a3, a4, d1-a4*u]
                            assert all(on_curve(a, P) for P in Fpoints)
                            for p, target in ((7, a7), (11, a11)):
                                assert all(c.denominator%p for c in a)
                                assert tuple(c.numerator*pow(c.denominator, -1, p)%p for c in a) == target
                            assert [c.numerator*pow(c.denominator, -1, 47)%47 for c in a] == [0, 0, 0, 46, 45]
                            related = rational_relation(Fpoints, a)
                            candidate = {"pair_index": index, "local_indices": [i7, i11],
                                         "k": ks, "coefficients": list(map(str, a)),
                                         "rational_relation": related}
                            stream.update((json.dumps(candidate, sort_keys=True, separators=(",", ":"))+"\n").encode())
                            expected["crt_candidates"] += 1
                            expected["rational_relations"] += int(related)
                            if related:
                                if expected["first_rational_relation"] is None:
                                    expected["first_rational_relation"] = candidate
                                positives.append(candidate)
        assert row == expected
        totals["pairs"] += 1
        totals["status_"+expected["status"]] += 1
        totals["local_residues_7"] += expected["local_counts"].get("7", {}).get("residues", 0)
        totals["local_residues_11"] += expected["local_counts"].get("11", {}).get("residues", 0)
        totals["local_gate_7"] += expected["local_counts"].get("7", {}).get("gate", 0)
        totals["local_gate_11"] += expected["local_counts"].get("11", {}).get("gate", 0)
        totals["crt_candidates"] += expected["crt_candidates"]
        totals["rational_relations"] += expected["rational_relations"]
        totals["reduced_event_pairs"] += int(old["reduced_relation"])
        totals["reduced_event_crt_candidates"] += expected["crt_candidates"] if old["reduced_relation"] else 0
        totals["reduced_event_rational_relations"] += expected["rational_relations"] if old["reduced_relation"] else 0
    assert dict(totals) == result["counts"]
    assert stream.hexdigest() == result["candidate_stream_sha256"]
    rerendered = "".join(json.dumps(x, sort_keys=True)+"\n" for x in positives).encode()
    assert rerendered == positive_raw
    by_pair = Counter(x["pair_index"] for x in positives)
    assert by_pair == {425: 4, 467: 4}
    print(json.dumps({"status": "PASS", "pairs_replayed": len(saved),
                      "crt_candidates_replayed": totals["crt_candidates"],
                      "positive_by_pair": dict(by_pair),
                      "candidate_stream_sha256": stream.hexdigest()}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
