"""Generate a Lean source table and proof target from the frozen point law."""

import gzip
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "xedni_coefficient_choice_20261120/source-rows-v2.jsonl.gz"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    packed = SOURCE.read_bytes()
    assert sha(packed) == protocol["source_sha256"]
    source = [json.loads(x) for x in gzip.decompress(packed).splitlines()]
    assert len(source) == 1722
    counts = {name: 0 for name in protocol["expected_classification_counts"]}
    survivors = []
    for i, row in enumerate(source):
        (x1, y1), (x2, y2) = row["lifted_points"]
        d, y = x1-x2, y1-y2
        if d == 0:
            assert y != 0
            counts["same_x"] += 1
        elif (d*d*(x1+2*x2)-y*y) % 47:
            counts["mod47"] += 1
        elif y*y % d:
            counts["divisibility"] += 1
        else:
            counts["surviving_pair"] += 1
            survivors.append(i)
    assert counts == protocol["expected_classification_counts"]
    assert survivors == protocol["expected_surviving_indices"]

    lines = [
        "import Tangent",
        "import SameX",
        "",
        "/-! Complete source-law necessary condition for the specified relation. -/",
        "namespace XedniFullSource",
        "open XedniCollisionTangent",
        "set_option maxRecDepth 100000",
        "set_option maxHeartbeats 0",
        "",
        "def sourceTable : List Coords := [",
    ]
    for i, row in enumerate(source):
        (x1, y1), (x2, y2) = row["lifted_points"]
        lines.append(f"  ⟨{x1}, {x2}, {y1}, {y2}⟩, -- source {i}")
    lines.extend([
        "]",
        "theorem sourceTable_length : sourceTable.length = 1722 := by decide",
        "def sourceCase (i : Fin 1722) : Coords :=",
        "  sourceTable.get ⟨i.val, by rw [sourceTable_length]; exact i.isLt⟩",
        "def survivor (i : Fin 1722) : Prop :=",
        "  i.val ∈ ([201, 243, 425, 467, 511, 553] : List ℕ)",
        "def blocked (i : Fin 1722) : Prop :=",
        "  ((sourceCase i).x1 = (sourceCase i).x2 ∧",
        "    (sourceCase i).y1 ≠ (sourceCase i).y2) ∨",
        "  (((sourceCase i).x1-(sourceCase i).x2)^2*",
        "    ((sourceCase i).x1+2*(sourceCase i).x2)-",
        "    ((sourceCase i).y1-(sourceCase i).y2)^2) % 47 ≠ 0 ∨",
        "  ((sourceCase i).y1-(sourceCase i).y2)^2 %",
        "    ((sourceCase i).x1-(sourceCase i).x2) ≠ 0 ∨",
        "  survivor i",
        "theorem exhaustive : ∀ i : Fin 1722, blocked i := by",
        "  dsimp only [blocked, survivor]",
        "  decide",
        "",
        "theorem relation_implies_survivor (i : Fin 1722)",
        "    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] (k1 k2 : ℤ)",
        "    (ha1 : W.a₁ = ((47*k1 : ℤ) : ℚ))",
        "    (ha2 : W.a₂ = ((47*k2 : ℤ) : ℚ))",
        "    (hP : W.Nonsingular ((sourceCase i).x1 : ℚ) ((sourceCase i).y1 : ℚ))",
        "    (hQ : W.Nonsingular ((sourceCase i).x2 : ℚ) ((sourceCase i).y2 : ℚ))",
        "    (hrel : (WeierstrassCurve.Affine.Point.some ((sourceCase i).x1 : ℚ)",
        "      ((sourceCase i).y1 : ℚ) hP) +",
        "      (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)",
        "      ((sourceCase i).y2 : ℚ) hQ) +",
        "      (WeierstrassCurve.Affine.Point.some ((sourceCase i).x2 : ℚ)",
        "      ((sourceCase i).y2 : ℚ) hQ) = 0) : survivor i := by",
        "  rcases exhaustive i with hsame | hmod | hrem | hs",
        "  · rcases hsame with ⟨hx, hy⟩",
        "    have hxq : ((sourceCase i).x1 : ℚ) = ((sourceCase i).x2 : ℚ) := by exact_mod_cast hx",
        "    have hyq : ((sourceCase i).y1 : ℚ) ≠ ((sourceCase i).y2 : ℚ) := by exact_mod_cast hy",
        "    have hQ' : W.Nonsingular ((sourceCase i).x1 : ℚ) ((sourceCase i).y2 : ℚ) := by",
        "      simpa only [hxq] using hQ",
        "    have hno := XedniSameX.no_relation_of_same_x_distinct_y hP hQ' hyq",
        "    exact False.elim (hno (by simpa only [hxq] using hrel))",
        "  · have hmod' : (((sourceCase i).x1-(sourceCase i).x2)^2*",
        "        ((sourceCase i).x1+2*(sourceCase i).x2)-",
        "        ((sourceCase i).y1-(sourceCase i).y2)^2) % 47 ≠ 0 := hmod",
        "    exact False.elim ((no_relation_of_mod47 W k1 k2 _ _ _ _ ha1 ha2 hP hQ hmod') hrel)",
        "  · have hdiv : ¬ ((sourceCase i).x1-(sourceCase i).x2) ∣",
        "        ((sourceCase i).y1-(sourceCase i).y2)^2 := by",
        "      intro hdvd",
        "      exact hrem (Int.emod_eq_zero_of_dvd hdvd)",
        "    exact False.elim ((no_relation_of_nondvd W (47*k1) (47*k2) _ _ _ _ ha1 ha2 hP hQ hdiv) hrel)",
        "  · exact hs",
        "#print axioms exhaustive",
        "#print axioms relation_implies_survivor",
        "end XedniFullSource",
    ])
    output = "\n".join(lines) + "\n"
    path = HERE / "formal/FullSource.lean"
    if sys.argv[1:] == ["--check"]:
        assert path.read_text() == output
    else:
        path.parent.mkdir(exist_ok=True)
        path.write_text(output)
    print(json.dumps({"status": "PASS", "counts": counts, "survivors": survivors,
                      "lean_sha256": sha(output.encode())}, sort_keys=True))


if __name__ == "__main__":
    main()
