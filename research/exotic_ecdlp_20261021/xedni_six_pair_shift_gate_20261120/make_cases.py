"""Generate kernel-checkable arithmetic cases from frozen shift rows."""

import gzip
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    result = json.loads((HERE / "result.json").read_text())
    packed = (HERE / "rows.jsonl.gz").read_bytes()
    assert sha(packed) == result["rows_packed_sha256"]
    rows = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    assert len(rows) == 144
    lines = ["import Shift", "", "namespace XedniSixPairShift", "",
             "set_option maxRecDepth 20000", "set_option maxHeartbeats 0", "",
             "structure LocalRow where", "  x1 : ℤ", "  x2 : ℤ", "  y1 : ℤ",
             "  y2 : ℤ", "  k1 : ℤ", "  k2 : ℤ", "", "def localTable : List LocalRow := ["]
    for row in rows:
        (x1, y1), (x2, y2) = row["points"]
        k1, k2, _ = row["centered_k"]
        lines.append(f"  ⟨{x1}, {x2}, {y1}, {y2}, {k1}, {k2}⟩, -- source {row['source_index']}, local {row['local_indices']}")
    lines.extend([
        "]", "",
        "def admissible (r : LocalRow) : Prop :=",
        "  residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 = 0 ∨",
        "  residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 % (47*77) ≠ 0",
        "theorem exhaustive : ∀ r ∈ localTable, admissible r := by",
        "  dsimp only [admissible]",
        "  decide",
        "theorem localTable_length : localTable.length = 144 := by decide",
        "theorem centered_zero_count :",
        "    (localTable.filter (fun r => decide (residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 = 0))).length = 8 := by",
        "  decide",
        "theorem noncentered_shift_excluded (r : LocalRow) (hr : r ∈ localTable)",
        "    (hnz : residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 ≠ 0) :",
        "    residual r.k1 r.k2 r.x1 r.x2 r.y1 r.y2 % (47*77) ≠ 0 := by",
        "  rcases exhaustive r hr with hzero | hmod",
        "  · exact False.elim (hnz hzero)",
        "  · exact hmod",
        "#print axioms exhaustive", "#print axioms centered_zero_count",
        "#print axioms noncentered_shift_excluded", "", "end XedniSixPairShift",
    ])
    output = "\n".join(lines) + "\n"
    path = HERE / "formal/Cases.lean"
    if sys.argv[1:] == ["--check"]:
        assert path.read_text() == output
    else:
        path.write_text(output)
    print(json.dumps({"schema": "xedni-six-pair-lean-cases-v1", "status": "PASS",
                      "lean_sha256": sha(output.encode()), "rows": 144}, sort_keys=True))


if __name__ == "__main__":
    main()
