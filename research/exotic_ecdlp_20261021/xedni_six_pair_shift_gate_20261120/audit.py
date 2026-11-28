"""Independent gate replay using the prior alternate local enumerator."""

import gzip
import hashlib
import importlib.util
import json
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PRIOR = ROOT / "xedni_full_law_crt_20261120"
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
spec = importlib.util.spec_from_file_location("independent_full_law_audit", PRIOR / "audit.py")
independent = importlib.util.module_from_spec(spec)
spec.loader.exec_module(independent)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    p = json.loads((HERE / "protocol.json").read_text())
    result = json.loads((HERE / "result.json").read_text())
    source_bytes = SOURCE.read_bytes()
    packed = (HERE / "rows.jsonl.gz").read_bytes()
    raw = gzip.decompress(packed)
    assert sha(source_bytes) == p["source_sha256"] == result["source_sha256"]
    assert sha(packed) == result["rows_packed_sha256"]
    assert sha(raw) == result["rows_raw_sha256"]
    source = [json.loads(x) for x in gzip.decompress(source_bytes).splitlines()]
    saved = [json.loads(x) for x in raw.splitlines()]
    counts = Counter()
    index = 0
    for i in p["input_indices"]:
        points = source[i]["lifted_points"]
        (x1, y1), (x2, y2) = points
        d, y = x1-x2, y1-y2
        w7, _ = independent.gate(tuple((x % 7, yy % 7) for x, yy in points), 7, 4)
        w11, _ = independent.gate(tuple((x % 11, yy % 11) for x, yy in points), 11, 6)
        assert len(w7) == 3 and len(w11) == 8
        for j, a7 in enumerate(w7):
            for k, a11 in enumerate(w11):
                row = saved[index]
                index += 1
                assert row["source_index"] == i and row["local_indices"] == [j, k]
                assert row["points"] == points
                assert tuple(row["a7"]) == a7 and tuple(row["a11"]) == a11
                ks = [independent.centered(a7[t], a11[t]) for t in range(3)]
                assert row["centered_k"] == ks
                z = d*d*(47*ks[1]+x1+2*x2)-47*ks[0]*d*y-y*y
                assert row["residual"] == z and row["residue_mod3619"] == z % 3619
                expected = ("centered_relation_necessary" if z == 0 else
                            "shift_excluded_mod3619" if z % 3619 else
                            "shift_unresolved_mod3619")
                assert row["status"] == expected
                counts[expected] += 1
    assert index == len(saved) == 144 and dict(counts) == result["counts"]
    print(json.dumps({"schema": "xedni-six-pair-shift-audit-v1", "status": "PASS",
                      "rows": index, "counts": dict(counts), "rows_raw_sha256": sha(raw)},
                     indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
