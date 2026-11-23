module

public import FairKernel
public import Mathlib.Data.ZMod.Basic

@[expose] public section

namespace CycleCouplingAudit

variable {n : ℕ} [NeZero n]

noncomputable def potential (d : ZMod n) : ℝ := (d.val : ℝ) * ((n : ℝ) - d.val)

theorem potential_nonneg (d : ZMod n) : 0 ≤ potential d := by
  unfold potential
  have hv : (d.val : ℝ) ≤ n := by exact_mod_cast (ZMod.val_lt d).le
  positivity

omit [NeZero n] in
theorem potential_max (d : ZMod n) : potential d ≤ (n : ℝ)^2 / 4 := by
  unfold potential
  nlinarith [sq_nonneg ((d.val : ℝ) - (n : ℝ) / 2)]

theorem val_plus (hn : 2 ≤ n) (d : ZMod n) :
    (d + 1).val = if d.val + 1 < n then d.val + 1 else 0 := by
  rw [ZMod.val_add, ZMod.val_one'' (by omega)]
  split_ifs with h
  · exact Nat.mod_eq_of_lt h
  · have he : d.val + 1 = n := by have := ZMod.val_lt d; omega
    simp [he]

theorem val_minus (hn : 2 ≤ n) (d : ZMod n) (hd : d ≠ 0) :
    (d - 1).val = d.val - 1 := by
  have hpos : 0 < d.val := Nat.pos_of_ne_zero (mt (ZMod.val_eq_zero d).mp hd)
  rw [ZMod.val_sub (by rw [ZMod.val_one'' (by omega)]; omega), ZMod.val_one'' (by omega)]

theorem potential_neighbors (hn : 2 ≤ n) (d : ZMod n) (hd : d ≠ 0) :
    (potential (d + 1) + potential (d - 1)) / 2 = potential d - 1 := by
  have hpos : 0 < d.val := Nat.pos_of_ne_zero (mt (ZMod.val_eq_zero d).mp hd)
  have hlt := ZMod.val_lt d
  unfold potential
  rw [val_plus hn, val_minus hn d hd, Nat.cast_sub (by omega : 1 ≤ d.val)]
  split_ifs with h
  · push_cast
    ring
  · have he : d.val + 1 = n := by omega
    have he' : (d.val : ℝ) + 1 = n := by exact_mod_cast he
    push_cast
    nlinarith

def coupleStep (b : Bool) (s : ZMod n × ZMod n) : ZMod n × ZMod n :=
  if s.1 = s.2 then
    if b then (s.1 + 1, s.2 + 1) else s
  else if b then (s.1 + 1, s.2) else (s.1, s.2 + 1)

noncomputable def mismatch (s : ZMod n × ZMod n) : ℝ := if s.1 = s.2 then 0 else 1

noncomputable def pairPotential (s : ZMod n × ZMod n) : ℝ := potential (s.1 - s.2)

omit [NeZero n] in
theorem mismatch_nonneg (s : ZMod n × ZMod n) : 0 ≤ mismatch s := by
  unfold mismatch
  split_ifs <;> norm_num

omit [NeZero n] in
theorem mismatch_coalescent (s : ZMod n × ZMod n) :
    (mismatch (coupleStep false s) + mismatch (coupleStep true s)) / 2 ≤ mismatch s := by
  rcases s with ⟨x,y⟩
  by_cases h : x = y
  · simp [coupleStep, mismatch, h]
  · simp only [coupleStep, h, ↓reduceIte, Bool.false_eq_true, mismatch]
    split_ifs <;> norm_num

theorem pair_drift (hn : 2 ≤ n) (s : ZMod n × ZMod n) :
    (pairPotential (coupleStep false s) + pairPotential (coupleStep true s)) / 2 =
      pairPotential s - mismatch s := by
  rcases s with ⟨x,y⟩
  by_cases h : x = y
  · simp [coupleStep, pairPotential, mismatch, h, potential]
  · have hd : x - y ≠ 0 := sub_ne_zero.mpr h
    have hplus : x + 1 - y = (x - y) + 1 := by ring
    have hminus : x - (y + 1) = (x - y) - 1 := by ring
    simpa [coupleStep, h, pairPotential, mismatch, hplus, hminus, add_comm] using
      potential_neighbors hn (x-y) hd

end CycleCouplingAudit
