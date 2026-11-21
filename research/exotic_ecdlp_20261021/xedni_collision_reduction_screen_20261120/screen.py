"""Independent F47 group-law screen for auxiliary-x-collision pairs."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
LOCAL = ROOT / "xedni_local_collision_crt_20261120"
P = 47


def sha(data):
    return hashlib.sha256(data).hexdigest()


def on_curve(q):
    x, y = q
    return (y*y-x*x*x+x+2) % P == 0


def add(a, b):
    if a is None:
        return b
    if b is None:
        return a
    x1, y1 = a
    x2, y2 = b
    if x1 == x2 and (y1+y2) % P == 0:
        return None
    if x1 == x2:
        slope = (3*x1*x1-1) * pow(2*y1, -1, P) % P
    else:
        slope = (y2-y1) * pow(x2-x1, -1, P) % P
    x3 = (slope*slope-x1-x2) % P
    return (x3, (slope*(x1-x3)-y1) % P)


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    local_result = json.loads((LOCAL / "result.json").read_text())
    local_raw = gzip.decompress((LOCAL / "pair-rows.jsonl.gz").read_bytes())
    assert sha(local_raw) == local_result["pair_rows_raw_sha256"]
    local = {r["pair_index"]: r for r in map(json.loads, local_raw.splitlines())}
    assert len(local) == 336
    rows, counts = [], Counter()
    for index, old in enumerate(source):
        q1, q2 = [tuple(v % P for v in Q) for Q in old["lifted_points"]]
        x1, x2 = old["lifted_points"][0][0], old["lifted_points"][1][0]
        if x1 == x2 or ((x1-x2) % 7 != 0 and (x1-x2) % 11 != 0):
            continue
        assert on_curve(q1) and on_curve(q2)
        relation = add(q1, add(q2, q2)) is None
        assert relation == bool(old["reduced_relation"])
        prior = local[index]
        row = {"pair_index": index, "points_mod47": [q1, q2],
               "collision_prime": prior["collision_prime"],
               "collision_kind": prior["collision_kind"],
               "reduced_relation": relation,
               "local_status": prior["status"], "local_pairs": prior["local_pairs"],
               "constructed": prior["constructed"]}
        rows.append(row)
        counts["selected_pairs"] += 1
        counts["reduced_relation_pairs"] += int(relation)
        counts["constructed_reduced_relation_pairs"] += int(relation and prior["constructed"] > 0)
        counts["constructed_on_reduced_relation_pairs"] += prior["constructed"] if relation else 0
        counts["same_point_pairs"] += prior["collision_kind"] == "same_point"
        counts["same_point_reduced_relation_pairs"] += relation and prior["collision_kind"] == "same_point"
    assert counts["selected_pairs"] == 336
    print(json.dumps({"schema": "xedni-collision-reduction-screen-result-v1",
                      "source_sha256": sha(source_bytes),
                      "protocol_sha256": sha(protocol_bytes),
                      "script_sha256": sha((HERE / "screen.py").read_bytes()),
                      "prior_pair_rows_raw_sha256": sha(local_raw),
                      "counts": dict(counts), "rows": rows}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
