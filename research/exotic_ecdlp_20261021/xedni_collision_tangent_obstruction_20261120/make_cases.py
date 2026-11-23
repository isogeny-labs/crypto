"""Generate the explicit Lean table for the frozen 336 collision source pairs."""

import gzip
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"
RESULT = HERE / "result.json"
SOURCE_SHA = "9a594d96f369fb14f1a4cf6270b1c811ceda515e1c2a7eccab4d011ca8f3f322"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    source_bytes = SOURCE.read_bytes()
    assert sha(source_bytes) == SOURCE_SHA
    source = [json.loads(line) for line in gzip.decompress(source_bytes).splitlines()]
    result = json.loads(RESULT.read_text())
    assert result["source_sha256"] == SOURCE_SHA
    rows = result["rows"]
    expected = []
    for index, row in enumerate(source):
        (x1, _), (x2, _) = row["lifted_points"]
        if x1 != x2 and ((x1-x2) % 7 == 0 or (x1-x2) % 11 == 0):
            expected.append(index)
    assert len(rows) == len(expected) == 336
    assert [r["pair_index"] for r in rows] == expected
    lines = [
        "import Tangent",
        "",
        "/-! Kernel-checked arithmetic certificate for all frozen auxiliary-x-collision pairs. -/",
        "",
        "namespace XedniCollisionTangent",
        "",
        "set_option maxRecDepth 20000",
        "set_option maxHeartbeats 0",
        "",
        "def sourceTable : List Coords := [",
    ]
    for position, row in enumerate(rows):
        index = row["pair_index"]
        (x1, y1), (x2, y2) = source[index]["lifted_points"]
        assert row["point1"] == [x1, y1] and row["point2"] == [x2, y2]
        lines.append(f"  ⟨{x1}, {x2}, {y1}, {y2}⟩, -- source {index}")
    lines.extend([
        "]",
        "",
        "theorem sourceTable_length : sourceTable.length = 336 := by decide",
        "",
        "def sourceCase (i : Fin 336) : Coords :=",
        "  sourceTable.get ⟨i.val, by rw [sourceTable_length]; exact i.isLt⟩",
        "",
        "def arithmeticBlocked (c : Coords) : Prop :=",
        "  (c.y1-c.y2)^2 % (c.x1-c.x2) ≠ 0 ∨",
        "    ((c.x1-c.x2)^2*(c.x1+2*c.x2)-(c.y1-c.y2)^2) % 47 ≠ 0",
        "",
        "theorem all_source_cases_blocked : ∀ i : Fin 336, arithmeticBlocked (sourceCase i) := by",
        "  dsimp only [arithmeticBlocked]",
        "  decide",
        "",
        "theorem no_relation_for_all_collision_cases (i : Fin 336)",
        "    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] (k1 k2 : ℤ)",
        "    (ha1 : W.a₁ = ((47*k1 : ℤ) : ℚ))",
        "    (ha2 : W.a₂ = ((47*k2 : ℤ) : ℚ))",
        "    (hP : W.Nonsingular ((sourceCase i).x1 : ℚ) ((sourceCase i).y1 : ℚ))",
        "    (hQ : W.Nonsingular ((sourceCase i).x2 : ℚ) ((sourceCase i).y2 : ℚ)) :",
        "    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x1 : ℚ)",
        "      ((sourceCase i).y1 : ℚ) hP) +",
        "    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)",
        "      ((sourceCase i).y2 : ℚ) hQ) +",
        "    (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)",
        "      ((sourceCase i).y2 : ℚ) hQ) ≠ 0 := by",
        "  rcases all_source_cases_blocked i with hrem | hmod",
        "  · have hdiv : ¬ ((sourceCase i).x1-(sourceCase i).x2) ∣",
        "        ((sourceCase i).y1-(sourceCase i).y2)^2 := by",
        "      intro hdvd",
        "      exact hrem (Int.emod_eq_zero_of_dvd hdvd)",
        "    exact no_relation_of_nondvd W (47*k1) (47*k2) _ _ _ _ ha1 ha2 hP hQ hdiv",
        "  · exact no_relation_of_mod47 W k1 k2 _ _ _ _ ha1 ha2 hP hQ hmod",
        "",
        "#print axioms all_source_cases_blocked",
        "#print axioms no_relation_for_all_collision_cases",
        "",
        "end XedniCollisionTangent",
    ])
    output = "\n".join(lines) + "\n"
    if len(sys.argv) == 2 and sys.argv[1] == "--check":
        assert (HERE / "formal/Cases.lean").read_text() == output
        print(sha(output.encode()))
    else:
        (HERE / "formal/Cases.lean").write_text(output)
        print(sha(output.encode()))


if __name__ == "__main__":
    main()
