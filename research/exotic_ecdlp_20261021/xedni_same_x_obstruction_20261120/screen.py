"""Exact source-law census for the same-rational-x obstruction."""

import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    assert len(source) == 1722
    selected = []
    for index, row in enumerate(source):
        p, q = row["lifted_points"]
        if p[0] != q[0]:
            continue
        selected.append({"source_index": index, "points": [p, q],
                         "same_y": p[1] == q[1], "opposite_y": p[1] == -q[1]})
    out = {
        "schema": "xedni-same-x-screen-result-v1",
        "source_sha256": sha(source_bytes),
        "protocol_sha256": sha((HERE / "protocol.json").read_bytes()),
        "script_sha256": sha((HERE / "screen.py").read_bytes()),
        "source_pairs": len(source),
        "same_x_pairs": len(selected),
        "same_y_pairs": sum(r["same_y"] for r in selected),
        "distinct_y_pairs": sum(not r["same_y"] for r in selected),
        "opposite_y_pairs": sum(r["opposite_y"] for r in selected),
        "source_rows": selected,
    }
    print(json.dumps(out, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
