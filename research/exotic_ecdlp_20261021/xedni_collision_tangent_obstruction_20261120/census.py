"""Bind the eight Lean point cases to the frozen auxiliary-collision source law."""

import gzip
import hashlib
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120" / "source-rows-v2.jsonl.gz"
REDUCTION = ROOT / "xedni_collision_reduction_screen_20261120" / "result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol_bytes = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_bytes)
    source_bytes = SOURCE.read_bytes()
    reduction_bytes = REDUCTION.read_bytes()
    assert sha(source_bytes) == protocol["source_sha256"]
    assert sha(reduction_bytes) == protocol["reduction_screen_sha256"]
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    reduction = json.loads(reduction_bytes)
    assert len(source) == 1722 and len(reduction["rows"]) == 336
    formal_cases = {case[0]: case[1:] for case in protocol["formal_cases"]}
    rows, counts = [], Counter()
    for item in reduction["rows"]:
        index = item["pair_index"]
        (x1, y1), (x2, y2) = source[index]["lifted_points"]
        assert x1 != x2
        dx, dy = x1-x2, y1-y2
        if item["reduced_relation"]:
            assert formal_cases[index] == [x1, x2, y1, y2]
            residue = dy*dy % abs(dx)
            status = "tangent_divisibility" if residue else "unclassified"
        else:
            residue = dy*dy % abs(dx)
            status = "fixed_prime_reduction"
        rows.append({"pair_index": index, "point1": [x1, y1], "point2": [x2, y2],
                     "dx": dx, "dy": dy, "dy2_mod_abs_dx": residue,
                     "reduced_relation": item["reduced_relation"],
                     "local_status": item["local_status"], "status": status})
        counts[status] += 1
    assert len(formal_cases) == counts["tangent_divisibility"] + counts["unclassified"]
    print(json.dumps({"schema": "xedni-collision-tangent-obstruction-result-v1",
                      "source_sha256": sha(source_bytes),
                      "reduction_screen_sha256": sha(reduction_bytes),
                      "protocol_sha256": sha(protocol_bytes),
                      "script_sha256": sha((HERE / "census.py").read_bytes()),
                      "counts": dict(counts), "rows": rows}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
