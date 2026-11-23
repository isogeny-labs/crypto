import Mathlib.NumberTheory.Zsqrtd.Basic
import Mathlib.Tactic.NormNum

-- Existing-law assembly, without exporting a new generic law.
example (n : Zsqrtd (-3)) : n.norm = n.re * n.re - (-3) * n.im * n.im :=
  Zsqrtd.norm_def n

example (n m : Zsqrtd (-3)) : (n * m).norm = n.norm * m.norm :=
  Zsqrtd.norm_mul n m

example : (⟨1, 5⟩ : Zsqrtd (-3)).norm = 76 := by
  rw [Zsqrtd.norm_def]
  norm_num

example : (⟨2, -5⟩ : Zsqrtd (-3)).norm = 79 := by
  rw [Zsqrtd.norm_def]
  norm_num
