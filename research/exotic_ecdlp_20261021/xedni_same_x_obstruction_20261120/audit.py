"""Independent streaming check of the source-law screen and its output."""

import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"


def main():
    observed = json.loads((HERE / "result.json").read_text())
    protocol = json.loads((HERE / "protocol.json").read_text())
    source_bytes = SOURCE.read_bytes()
    source_hash = hashlib.sha256(source_bytes).hexdigest()
    assert observed["source_sha256"] == protocol["source_sha256"] == source_hash
    assert observed["protocol_sha256"] == hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest()
    assert observed["script_sha256"] == hashlib.sha256((HERE / "screen.py").read_bytes()).hexdigest()

    expected = {}
    total = 0
    with gzip.open(SOURCE, "rt") as stream:
        for index, line in enumerate(stream):
            total += 1
            points = json.loads(line)["lifted_points"]
            if points[0][0] == points[1][0]:
                expected[index] = points
    rows = observed["source_rows"]
    assert total == observed["source_pairs"] == 1722
    assert len(rows) == len(expected) == observed["same_x_pairs"]
    assert len({row["source_index"] for row in rows}) == len(rows)
    for row in rows:
        p, q = expected[row["source_index"]]
        assert row["points"] == [p, q]
        assert row["same_y"] == (p[1] == q[1])
        assert row["opposite_y"] == (p[1] == -q[1])
    assert observed["same_y_pairs"] == sum(p[0][1] == p[1][1] for p in expected.values())
    assert observed["distinct_y_pairs"] == len(expected) - observed["same_y_pairs"]
    assert observed["opposite_y_pairs"] == sum(p[0][1] == -p[1][1] for p in expected.values())
    print(json.dumps({"status": "PASS", "source_pairs": total,
                      "same_x_pairs": len(expected),
                      "distinct_y_pairs": observed["distinct_y_pairs"]}, sort_keys=True))


if __name__ == "__main__":
    main()
