"""Enumerate all local Weierstrass coefficient residues through reduced-event pairs."""

import gzip
import hashlib
import itertools
import json
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "xedni_coefficient_choice_20261120"
sys.path.insert(0, str(PRIOR))
from audit import curve_order, discriminant, relation


def sha(data):
    return hashlib.sha256(data).hexdigest()


def incidence_rank(points, p):
    rows = [tuple(v % p for v in (y*y, x*y, y, x**3, x*x, x, 1)) for x, y in points]
    return 1 if rows[0] == rows[1] else 2


def coeffs(points, p):
    (x1, y1), (x2, y2) = points
    for a1, a2, a3 in itertools.product(range(p), repeat=3):
        d1 = (y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1) % p
        d2 = (y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2) % p
        if x1 != x2:
            a4 = (d1-d2)*pow((x1-x2)%p, -1, p) % p
            yield (a1, a2, a3, a4, (d1-a4*x1)%p)
        elif d1 == d2:
            for a4 in range(p):
                yield (a1, a2, a3, a4, (d1-a4*x1)%p)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = (PRIOR / "source-rows-v2.jsonl.gz").read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    reduced = [(i, row) for i, row in enumerate(source) if row["reduced_relation"]]
    assert len(reduced) == 42
    global_counts = {"7_4": Counter(), "11_6": Counter(), "11_7": Counter()}
    output = []
    stream = hashlib.sha256()
    for pair_index, old in reduced:
        arms = {}
        for p in (7, 11):
            points = [tuple(v % p for v in P) for P in old["lifted_points"]]
            rank = incidence_rank(points, p)
            local = {order: {"counts": Counter(), "first_gate": None, "first_gate_relation": None}
                     for order in ((4,) if p == 7 else (6, 7))}
            for a in coeffs(points, p):
                nonsingular = discriminant(a) % p != 0
                order = curve_order(a, p) if nonsingular else None
                rel = relation(points, a, p) if nonsingular else None
                for target, data in local.items():
                    c = data["counts"]
                    c["candidate_residues"] += 1
                    c["singular"] += int(not nonsingular)
                    if nonsingular:
                        c["good_reduction"] += 1
                        c["rank_lt_2"] += int(rank < 2)
                        c["rank2"] += int(rank == 2)
                        if rank == 2 and order == target:
                            c["gate"] += 1
                            c["gate_relation"] += int(rel)
                            if data["first_gate"] is None:
                                data["first_gate"] = list(a)
                            if rel and data["first_gate_relation"] is None:
                                data["first_gate_relation"] = list(a)
                stream.update((json.dumps([pair_index, p, a, nonsingular, order, rel], separators=(",", ":"))+"\n").encode())
            for target, data in local.items():
                key = f"{p}_{target}"
                arms[key] = {"incidence_rank": rank, "counts": dict(data["counts"]),
                             "first_gate": data["first_gate"],
                             "first_gate_relation": data["first_gate_relation"]}
                global_counts[key].update(data["counts"])
                global_counts[key]["pairs_with_gate"] += int(data["first_gate"] is not None)
                global_counts[key]["pairs_with_gate_relation"] += int(data["first_gate_relation"] is not None)
        output.append(json.dumps({"pair_index": pair_index, "lifted_points": old["lifted_points"], "arms": arms}, sort_keys=True)+"\n")
    raw = "".join(output).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "pair-rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-local-residue-result-v1", "status": "PASS",
                      "protocol_sha256": sha(protocol_bytes), "source_sha256": sha(source_bytes),
                      "script_sha256": sha((HERE / "run.py").read_bytes()),
                      "arithmetic_sha256": sha((PRIOR / "audit.py").read_bytes()),
                      "pair_rows_raw_sha256": sha(raw), "pair_rows_packed_sha256": sha(packed),
                      "candidate_stream_sha256": stream.hexdigest(),
                      "reduced_event_pairs": len(reduced),
                      "counts": {k: dict(v) for k, v in global_counts.items()}}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
