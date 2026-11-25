"""Independent all-base rank control on the frozen B677 generic relation stream.

Every signed equation is replayed before FLINT eliminates all 677 support
columns directly. This tests whether the LP graph is needed for this solve.
"""

import hashlib
import json
from pathlib import Path

from flint import nmod_mat

from verify_mitm_graph import A, B, B677_INPUT, G, N, P, Q, ROOT, add, scalar

RECEIPT = ROOT / "evidence/mitm-b677-control-01/forty-cap-full-cost.json"
OUT = ROOT / "evidence/mitm-b677-control-01/all-base-control.json"


def check_prefix(relations, queries, points, count):
    rows = []
    for relation in relations:
        label = relation["label"]
        if label >= count:
            continue
        row = [0] * 679
        total = None
        for index, sign in relation["terms"]:
            x, y = points[index]
            total = add(total, (x, y if sign == 1 else (-y) % P))
            row[index] = sign % N
        assert total == tuple(queries[label]["query"])
        row[677] = (-queries[label]["b"]) % N
        row[678] = queries[label]["a"]
        rows.append(row)
    reduced, rank = nmod_mat(rows, N).rref()
    target_only = []
    contradictions = 0
    for i in range(len(rows)):
        support_zero = all(int(reduced[i, j]) == 0 for j in range(677))
        if support_zero and int(reduced[i, 677]) == 1:
            target_only.append(int(reduced[i, 678]))
        if support_zero and int(reduced[i, 677]) == 0 and int(reduced[i, 678]) != 0:
            contradictions += 1
    assert contradictions == 0
    assert all(scalar(d, G) == Q for d in target_only)
    return {"queries": count, "relations_replayed": len(rows),
            "augmented_rank": rank, "target_only_rows": target_only,
            "contradictions": contradictions}


def main():
    receipt = json.loads(RECEIPT.read_text())
    fixture_line = next(line for line in B677_INPUT.read_text().splitlines()
                        if line.startswith("B677_INPUT "))
    fixture = json.loads(fixture_line.split(" ", 1)[1])
    points = fixture["public_base"]
    assert len(points) == 677 and len({point[0] for point in points}) == 677
    assert receipt["curve"] == {"p": P, "a": A, "b": B, "n": N, "g": list(G)}
    assert tuple(receipt["public_target"]) == Q
    assert len(receipt["queries"]) == 11
    results = [check_prefix(receipt["relation_rows"], receipt["queries"], points, k)
               for k in (10, 11)]
    assert results[0]["target_only_rows"] == []
    assert results[1]["target_only_rows"] == [receipt["recovered_scalar"]]
    output = {"source_receipt_sha256": hashlib.sha256(RECEIPT.read_bytes()).hexdigest(),
              "curve": receipt["curve"], "public_target": receipt["public_target"],
              "all_677_points_treated_as_base": True, "prefixes": results,
              "interpretation": "same generic source recovers target without LP projection"}
    OUT.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output, indent=2))


if __name__ == "__main__":
    main()
