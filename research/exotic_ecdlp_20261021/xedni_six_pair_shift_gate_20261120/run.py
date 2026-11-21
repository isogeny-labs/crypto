"""Screen every six-pair local witness class against all 77-shifts."""

import gzip
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PRIOR = ROOT / "xedni_full_law_crt_20261120"
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
sys.path.insert(0, str(PRIOR))
from run import centered_k, local_gate  # noqa: E402


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    p = json.loads((HERE / "protocol.json").read_text())
    assert sha(SOURCE.read_bytes()) == p["source_sha256"]
    assert sha((PRIOR / "run.py").read_bytes()) == p["prior_runner_sha256"]
    assert sha((PRIOR / "result.json").read_bytes()) == p["prior_result_sha256"]
    source = [json.loads(line) for line in gzip.open(SOURCE, "rt")]
    rows = []
    counts = Counter()
    for i in p["input_indices"]:
        points = source[i]["lifted_points"]
        (x1, y1), (x2, y2) = points
        d, y = x1-x2, y1-y2
        w7, _ = local_gate(tuple((x % 7, yy % 7) for x, yy in points), 7, 4)
        w11, _ = local_gate(tuple((x % 11, yy % 11) for x, yy in points), 11, 6)
        assert len(w7) == 3 and len(w11) == 8
        for j, a7 in enumerate(w7):
            for k, a11 in enumerate(w11):
                ks = [centered_k(a7[t], a11[t]) for t in range(3)]
                k1, k2, _ = ks
                residual = d*d*(47*k2+x1+2*x2)-47*k1*d*y-y*y
                residue = residual % (47*77)
                if residual == 0:
                    status = "centered_relation_necessary"
                elif residue != 0:
                    status = "shift_excluded_mod3619"
                else:
                    status = "shift_unresolved_mod3619"
                rows.append({"source_index": i, "local_indices": [j, k],
                             "points": points, "a7": a7, "a11": a11,
                             "centered_k": ks, "residual": residual,
                             "residue_mod3619": residue, "status": status})
                counts[status] += 1
    assert len(rows) == p["expected_rows"]
    raw = "".join(json.dumps(row, sort_keys=True, separators=(",", ":"))+"\n"
                  for row in rows).encode()
    packed = gzip.compress(raw, mtime=0)
    (HERE / "rows.jsonl.gz").write_bytes(packed)
    print(json.dumps({"schema": "xedni-six-pair-shift-result-v1", "status": "PASS",
                      "protocol_sha256": sha((HERE / "protocol.json").read_bytes()),
                      "source_sha256": sha(SOURCE.read_bytes()),
                      "rows_raw_sha256": sha(raw), "rows_packed_sha256": sha(packed),
                      "rows": len(rows), "counts": dict(counts)}, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
