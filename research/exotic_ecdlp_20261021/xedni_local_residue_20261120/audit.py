"""Independent local enumeration by four free coefficients and character sums."""

import gzip
import hashlib
import itertools
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def incidence_rank(points, p):
    x, y = points[0]
    u, v = points[1]
    row1 = tuple(z % p for z in (y*y, x*y, y, x**3, x*x, x, 1))
    row2 = tuple(z % p for z in (v*v, u*v, v, u**3, u*u, u, 1))
    return 1 if row1 == row2 else 2


def eval_curve(a, point, p):
    x, y = point
    a1, a2, a3, a4, a6 = a
    return (y*y+a1*x*y+a3*y-x**3-a2*x*x-a4*x-a6) % p


def smooth(a, p):
    a1, a2, a3, a4, a6 = a
    for x in range(p):
        y = -(a1*x+a3)*pow(2, -1, p) % p
        if eval_curve(a, (x, y), p) == 0 and (a1*y-3*x*x-2*a2*x-a4) % p == 0:
            return False
    return True


def order_character_sum(a, p):
    a1, a2, a3, a4, a6 = a
    squares = {x*x % p for x in range(p)}
    total = p+1
    for x in range(p):
        d = ((a1*x+a3)**2 + 4*(x**3+a2*x*x+a4*x+a6)) % p
        total += 0 if d == 0 else (1 if d in squares else -1)
    return total


def relation_by_double(points, a, p):
    (x1, y1), (x, y) = points
    a1, a2, a3, a4, a6 = a
    den = (2*y+a1*x+a3) % p
    if den == 0:
        return False
    inv = pow(den, -1, p)
    lam = (3*x*x+2*a2*x+a4-a1*y)*inv % p
    nu = (-x**3+a4*x+2*a6-a3*y)*inv % p
    xdouble = (lam*lam+a1*lam-a2-2*x) % p
    ydouble = (-(lam+a1)*xdouble-nu-a3) % p
    return (xdouble, ydouble) == (x1, (-y1-a1*x1-a3) % p)


def candidates(points, p):
    (x1, y1), (x2, y2) = points
    for a1, a2, a3, a4 in itertools.product(range(p), repeat=4):
        a6 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1-a4*x1) % p
        a = (a1, a2, a3, a4, a6)
        if eval_curve(a, (x2, y2), p) == 0:
            yield a


def main():
    result = json.loads((HERE / "result.json").read_text())
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    packed = (HERE / "pair-rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert result["protocol_sha256"] == sha(protocol_bytes)
    assert result["source_sha256"] == sha(source_bytes)
    assert result["script_sha256"] == sha((HERE / "run.py").read_bytes())
    assert result["arithmetic_sha256"] == sha((PRIOR / "audit.py").read_bytes())
    assert result["pair_rows_packed_sha256"] == sha(packed)
    assert result["pair_rows_raw_sha256"] == sha(raw)
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    saved = [json.loads(x) for x in raw.splitlines()]
    assert len(saved) == 42
    stream = hashlib.sha256()
    totals = {"7_4": Counter(), "11_6": Counter(), "11_7": Counter()}
    for row in saved:
        index = row["pair_index"]
        original = source[index]
        assert original["reduced_relation"]
        assert row["lifted_points"] == original["lifted_points"]
        for p in (7, 11):
            points = [tuple(v % p for v in P) for P in original["lifted_points"]]
            rank = incidence_rank(points, p)
            arms = {target: {"counts": Counter(), "first_gate": None, "first_gate_relation": None}
                    for target in ((4,) if p == 7 else (6, 7))}
            for a in candidates(points, p):
                good = smooth(a, p)
                order = order_character_sum(a, p) if good else None
                rel = relation_by_double(points, a, p) if good else None
                for target, rec in arms.items():
                    c = rec["counts"]
                    c["candidate_residues"] += 1
                    c["singular"] += int(not good)
                    if good:
                        c["good_reduction"] += 1
                        c["rank_lt_2"] += int(rank < 2)
                        c["rank2"] += int(rank == 2)
                        if rank == 2 and order == target:
                            c["gate"] += 1
                            c["gate_relation"] += int(rel)
                            if rec["first_gate"] is None:
                                rec["first_gate"] = list(a)
                            if rel and rec["first_gate_relation"] is None:
                                rec["first_gate_relation"] = list(a)
                stream.update((json.dumps([index, p, a, good, order, rel], separators=(",", ":"))+"\n").encode())
            for target, rec in arms.items():
                key = f"{p}_{target}"
                assert row["arms"][key]["incidence_rank"] == rank
                assert row["arms"][key]["counts"] == dict(rec["counts"])
                assert row["arms"][key]["first_gate"] == rec["first_gate"]
                assert row["arms"][key]["first_gate_relation"] == rec["first_gate_relation"]
                totals[key].update(rec["counts"])
                totals[key]["pairs_with_gate"] += int(rec["first_gate"] is not None)
                totals[key]["pairs_with_gate_relation"] += int(rec["first_gate_relation"] is not None)
    assert {k: dict(v) for k, v in totals.items()} == result["counts"]
    assert stream.hexdigest() == result["candidate_stream_sha256"]
    for index in (353, 395):
        row = next(x for x in saved if x["pair_index"] == index)
        assert row["arms"]["7_4"]["counts"]["gate_relation"] == 0
        assert row["arms"]["11_6"]["counts"]["gate_relation"] == 0
    joint = {arm: sum(row["arms"]["7_4"]["first_gate_relation"] is not None and
                      row["arms"][arm]["first_gate_relation"] is not None for row in saved)
             for arm in ("11_6", "11_7")}
    assert joint == {"11_6": 14, "11_7": 22}
    print(json.dumps({"status": "PASS", "replayed_pairs": len(saved),
                      "candidate_stream_sha256": stream.hexdigest(),
                      "selected_pair_local_obstructions": [353, 395],
                      "joint_relation_compatible_pairs": joint}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
