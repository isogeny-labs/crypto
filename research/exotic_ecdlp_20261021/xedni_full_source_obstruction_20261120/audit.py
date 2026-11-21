"""Independent source/Lean-table audit for the full-law obstruction."""

import gzip
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
LEAN = HERE / "formal/FullSource.lean"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    packed = SOURCE.read_bytes()
    assert sha(packed) == protocol["source_sha256"]
    rows = [json.loads(line) for line in gzip.decompress(packed).splitlines()]
    text = LEAN.read_text()
    pattern = r"⟨(-?\d+), (-?\d+), (-?\d+), (-?\d+)⟩, -- source (\d+)"
    table = [tuple(map(int, m.groups())) for m in re.finditer(pattern, text)]
    assert len(rows) == len(table) == 1722
    counts = Counter()
    surviving = []
    for position, (x1, x2, y1, y2, index) in enumerate(table):
        assert index == position
        assert ((x1, y1), (x2, y2)) == tuple(map(tuple, rows[index]["lifted_points"]))
        d, y = x1-x2, y1-y2
        if d == 0:
            assert y != 0
            counts["same_x"] += 1
        elif (d*d*(x1+2*x2)-y*y) % 47:
            counts["mod47"] += 1
        elif y*y % abs(d):
            counts["divisibility"] += 1
        else:
            counts["surviving_pair"] += 1
            surviving.append(index)
    assert dict(counts) == protocol["expected_classification_counts"]
    assert surviving == protocol["expected_surviving_indices"]
    positives = [json.loads(line) for line in gzip.open(
        HERE.parent / "xedni_full_law_crt_20261120/positive-rows.jsonl.gz", "rt")]
    assert len(positives) == 8
    assert {row["pair_index"] for row in positives} == {425, 467}
    print(json.dumps({"schema": "xedni-full-source-audit-v1", "status": "PASS",
                      "source_sha256": sha(packed), "lean_sha256": sha(text.encode()),
                      "counts": dict(counts), "surviving_indices": surviving,
                      "positive_pair_indices": [425, 467], "positive_records": 8},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
