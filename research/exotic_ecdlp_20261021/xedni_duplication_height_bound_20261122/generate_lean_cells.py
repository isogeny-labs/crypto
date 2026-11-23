#!/usr/bin/env python3
"""Emit kernel-checkable Lean obligations for the frozen rational cells."""

import json
import sys
from fractions import Fraction
from pathlib import Path

from audit_certificate import F, G, bernstein_coefficients

HERE = Path(__file__).resolve().parent


def lit(q):
    q = Fraction(q)
    if q.denominator == 1:
        return f"({q.numerator} : ℝ)"
    return f"({q.numerator} / {q.denominator} : ℝ)"


def emit():
    data = json.loads((HERE / "certificate.json").read_text())
    assert len(data["leaves"]) == 91 and data["target_denominator"] == 1024
    out = [
        "import Bounds",
        "",
        "namespace XedniDuplicationCells",
        "",
        "def F_real (t : ℝ) : ℝ := t ^ 4 - 1502 * t ^ 2 + 31976 * t - 187435",
        "def G_real (t : ℝ) : ℝ := 4 * t ^ 3 - 188 * t ^ 2 + 3004 * t - 15988",
        "",
        "private theorem from_cell_F (t K N : ℝ) (hN : max 1 |t| ≤ N)",
        "    (hK : N ^ 4 / 1024 = K) (hF : K ≤ F_real t) :",
        "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
        "  calc",
        "    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr",
        "    _ = K := hK",
        "    _ ≤ |F_real t| := hF.trans (le_abs_self _)",
        "    _ ≤ max |F_real t| |G_real t| := le_max_left _ _",
        "",
        "private theorem from_cell_negF (t K N : ℝ) (hN : max 1 |t| ≤ N)",
        "    (hK : N ^ 4 / 1024 = K) (hF : K ≤ -F_real t) :",
        "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
        "  calc",
        "    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr",
        "    _ = K := hK",
        "    _ ≤ |F_real t| := hF.trans (neg_le_abs _)",
        "    _ ≤ max |F_real t| |G_real t| := le_max_left _ _",
        "",
        "private theorem from_cell_G (t K N : ℝ) (hN : max 1 |t| ≤ N)",
        "    (hK : N ^ 4 / 1024 = K) (hG : K ≤ G_real t) :",
        "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
        "  calc",
        "    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr",
        "    _ = K := hK",
        "    _ ≤ |G_real t| := hG.trans (le_abs_self _)",
        "    _ ≤ max |F_real t| |G_real t| := le_max_right _ _",
        "",
        "private theorem from_cell_negG (t K N : ℝ) (hN : max 1 |t| ≤ N)",
        "    (hK : N ^ 4 / 1024 = K) (hG : K ≤ -G_real t) :",
        "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
        "  calc",
        "    (max 1 |t|) ^ 4 / 1024 ≤ N ^ 4 / 1024 := by gcongr",
        "    _ = K := hK",
        "    _ ≤ |G_real t| := hG.trans (neg_le_abs _)",
        "    _ ≤ max |F_real t| |G_real t| := le_max_right _ _",
        "",
    ]
    for i, row in enumerate(data["leaves"]):
        left, right = Fraction(*row["lo"]), Fraction(*row["hi"])
        name, sign = row["witness"]
        coeffs = bernstein_coefficients(F if name == "F" else (0,) + G, left, right)
        if sign == "-":
            coeffs = [-b for b in coeffs]
        bound_n = max(1, abs(left), abs(right))
        k = bound_n**4 / 1024
        assert all(b >= k for b in coeffs)
        inv_width = 1 / (right - left)
        offset = -left * inv_width
        polynomial = ("-" if sign == "-" else "") + f"{name}_real t"
        helper = f"from_cell_{'neg' if sign == '-' else ''}{name}"
        out += [
            f"private theorem cell_{i} (t : ℝ)",
            f"    (hl : {lit(left)} ≤ t) (hu : t ≤ {lit(right)}) :",
            f"    {lit(k)} ≤ {polynomial} := by",
            f"  let s : ℝ := {lit(inv_width)} * t + {lit(offset)}",
            "  have hs₀ : 0 ≤ s := by dsimp [s]; linarith [hl]",
            "  have hs₁ : s ≤ 1 := by dsimp [s]; linarith [hu]",
            "  have hb := XedniDuplicationBound.bernstein_quartic_lower",
            f"    {' '.join(lit(b) for b in coeffs)}",
            f"    {lit(k)} s hs₀ hs₁",
            "    (by norm_num) (by norm_num) (by norm_num)",
            "    (by norm_num) (by norm_num)",
            f"  convert hb using 1 <;> dsimp [s, {name}_real] <;> ring",
            "",
            f"private theorem cell_{i}_global (t : ℝ)",
            f"    (hl : {lit(left)} ≤ t) (hu : t ≤ {lit(right)}) :",
            "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
            f"  have hN : max (1 : ℝ) |t| ≤ {lit(bound_n)} := by",
            "    apply max_le",
            "    · norm_num",
            "    · apply abs_le.mpr",
            "      constructor <;> linarith [hl, hu]",
            f"  have hK : {lit(bound_n)} ^ 4 / 1024 = {lit(k)} := by norm_num",
            f"  exact {helper} t {lit(k)} {lit(bound_n)} hN hK (cell_{i} t hl hu)",
            "",
        ]
    out += [
        "set_option maxHeartbeats 5000000 in",
        "theorem finite_archimedean_lower (t : ℝ) (hl : (-128 : ℝ) ≤ t)",
        "    (hu : t ≤ (128 : ℝ)) :",
        "    (max 1 |t|) ^ 4 / 1024 ≤ max |F_real t| |G_real t| := by",
    ]
    for i, row in enumerate(data["leaves"][:-1]):
        right = Fraction(*row["hi"])
        out.append(f"  by_cases h{i} : t ≤ {lit(right)}")
        lower_proof = "hl" if i == 0 else f"(le_of_lt (lt_of_not_ge h{i-1}))"
        out.append(f"  · exact cell_{i}_global t {lower_proof} h{i}")
    out.append("  exact cell_90_global t (le_of_lt (lt_of_not_ge h89)) hu")
    out += ["", "#print axioms finite_archimedean_lower", "", "end XedniDuplicationCells", ""]
    return "\n".join(out)


if __name__ == "__main__":
    target = HERE / "Cells.lean"
    source = emit()
    if sys.argv[1:] == ["--check"]:
        assert target.read_text() == source, "Cells.lean differs from frozen certificate"
        print("PASS generated Lean source matches 91 exact cells")
    else:
        assert not sys.argv[1:]
        target.write_text(source)
        print(f"Wrote {target.name} from 91 exact cells")
