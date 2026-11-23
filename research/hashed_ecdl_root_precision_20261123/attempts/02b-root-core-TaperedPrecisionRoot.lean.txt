import CryptoLeanCore.TaperedPrecision

/-!
# Finite root precision from the source array guard

The native root shape has `a=2*u`. These laws concern the shared exact Nat
source expression. Appendix D.11 states an asymptotic envelope; the finite
constant35 here is a separate root resource law. Existing conditional34 and
finite-history81 statements are unchanged. Actual Rust refinement, word-size
admission, later-depth precision and complete counter cost remain consumers.
-/

namespace CryptoLeanCore.TaperedPrecision

private theorem quadratic_step (u : Nat) (hu : 3 ≤ u) :
    (u + 1) * (2 * u + 3) ≤ 2 * u * (2 * u + 1) := by
  have hs : 3 * u ≤ u * u := Nat.mul_le_mul_right u hu
  have hl : (u + 1) * (2 * u + 3) = 2 * (u * u) + 3 * u + 2 * u + 3 := by
    simp only [Nat.add_mul, Nat.mul_add, Nat.one_mul]
    ac_rfl
  have hr : 2 * u * (2 * u + 1) = 4 * (u * u) + 2 * u := by
    rw [Nat.mul_add, Nat.mul_one]
    change (2 * u) * (2 * u) + 2 * u = (2 * 2) * (u * u) + 2 * u
    ac_rfl
  rw [hl, hr]
  omega

private theorem evaluation_tail (k : Nat) :
    16 * (k + 3) * (2 * (k + 3) + 1) ≤ 21 * 2 ^ (k + 4) := by
  induction k with
  | zero => decide
  | succ k ih =>
    have hstep :
        16 * (k + 3 + 1) * (2 * (k + 3 + 1) + 1) ≤
          2 * (16 * (k + 3) * (2 * (k + 3) + 1)) := by
      have he : 2 * (k + 3 + 1) + 1 = 2 * (k + 3) + 3 := by omega
      rw [he]
      calc
        _ = 16 * ((k + 3 + 1) * (2 * (k + 3) + 3)) := by ac_rfl
        _ ≤ 16 * (2 * (k + 3) * (2 * (k + 3) + 1)) :=
          Nat.mul_le_mul_left 16 (quadratic_step (k + 3) (by omega))
        _ = 2 * (16 * (k + 3) * (2 * (k + 3) + 1)) := by ac_rfl
    change 16 * (k + 3 + 1) * (2 * (k + 3 + 1) + 1) ≤ 21 * 2 ^ (k + 5)
    calc
      _ ≤ 2 * (16 * (k + 3) * (2 * (k + 3) + 1)) := hstep
      _ ≤ 2 * (21 * 2 ^ (k + 4)) := Nat.mul_le_mul_left 2 ih
      _ = 21 * 2 ^ (k + 5) := by
        calc
          _ = 21 * (2 ^ (k + 4) * 2) := by ac_rfl
          _ = 21 * 2 ^ ((k + 4) + 1) :=
            congrArg (fun x => 21 * x) (Nat.pow_succ 2 (k + 4)).symm
          _ = 21 * 2 ^ (k + 5) := rfl

/-- Root evaluation loss: an exact finite polynomial/power inequality.
The maximum normalized value occurs at u=3; no asymptotic threshold is used. -/
theorem root_evaluation_envelope (u : Nat) (hu : 0 < u) :
    16 * u * (2 * u + 1) ≤ 21 * 2 ^ (u + 1) := by
  cases u with
  | zero => omega
  | succ u =>
    cases u with
    | zero => decide
    | succ u =>
      cases u with
      | zero => decide
      | succ u => exact evaluation_tail u

/-- The root's smallest active divisor `2^(u+1)` covers four times its scale. -/
theorem root_four_scale (u : Nat) (hu : 0 < u) : 4 * u ≤ 2 ^ (u + 1) := by
  have hs : u - 1 + 1 ≤ 2 ^ (u - 1) := Nat.succ_le_of_lt Nat.lt_two_pow_self
  have hb : u ≤ 2 ^ (u - 1) := by omega
  calc
    _ ≤ 4 * 2 ^ (u - 1) := Nat.mul_le_mul_left 4 hb
    _ = 2 ^ ((u - 1) + 2) := by
      rw [Nat.pow_add]
      change 4 * 2 ^ (u - 1) = 2 ^ (u - 1) * 4
      exact Nat.mul_comm _ _
    _ = 2 ^ (u + 1) := by congr 1; omega

/-- The exact root length/array guard already supplies the finite rounding
margin used by `root_precision_mul_le`. This is Nat arithmetic, not a claim
that any production usize graph reaches a recursive root. -/
theorem root_array_margin (M u : Nat)
    (hguard : 2 ^ (2 * u) * 64 ^ (4 * u) ≤ M) : 32 * u ≤ M := by
  by_cases hz : u = 0
  · subst u
    omega
  have hu : 0 < u := by omega
  have hid : 2 ^ (2 * u) * 64 ^ (4 * u) = 2 ^ (26 * u) := by
    change 2 ^ (2 * u) * (2 ^ 6) ^ (4 * u) = 2 ^ (26 * u)
    rw [← Nat.pow_mul, ← Nat.pow_add]
    congr 1
    omega
  have hsmall : 32 * u ≤ 2 ^ (u + 5) := by
    calc
      _ ≤ 32 * 2 ^ u := Nat.mul_le_mul_left 32 (Nat.le_of_lt Nat.lt_two_pow_self)
      _ = 2 ^ (u + 5) := by
        rw [Nat.pow_add]
        change 32 * 2 ^ u = 2 ^ u * 32
        exact Nat.mul_comm _ _
  calc
    _ ≤ 2 ^ (u + 5) := hsmall
    _ ≤ 2 ^ (26 * u) := Nat.pow_le_pow_right (by decide) (by omega)
    _ = 2 ^ (2 * u) * 64 ^ (4 * u) := hid.symm
    _ ≤ M := hguard

/-- Finite root precision bound. The source shape a=2u, initial p<=M and
rounding margin32u<=M give constant35 with no inherited-history or two
asymptotic absorption premises. Later depths are outside this statement. -/
theorem root_precision_mul_le (p M u i : Nat)
    (hp : p ≤ M) (hmargin : 32 * u ≤ M) (hi : i < u) :
    childPrecision p M u (2 * u) i * u ≤ 35 * M * 2 ^ (2 * u - i) := by
  let D := 2 ^ (2 * u - i)
  have hu : 0 < u := by omega
  have hD : 2 ^ (u + 1) ≤ D := Nat.pow_le_pow_right (by decide) (by omega)
  have hfour : 4 * u ≤ D := Nat.le_trans (root_four_scale u hu) hD
  have hh : ((3 * M + u - 1) / u) * u ≤ 3 * M + u :=
    Nat.le_trans (Nat.div_mul_le_self _ _) (by omega)
  have hpin : 4 * (p * u) ≤ M * D := by
    calc
      _ = p * (4 * u) := by ac_rfl
      _ ≤ M * (4 * u) := Nat.mul_le_mul_right _ hp
      _ ≤ M * D := Nat.mul_le_mul_left M hfour
  have hnode : 4 * (8 * ((3 * M + u - 1) / u) * D * u) ≤ 97 * M * D := by
    calc
      _ = 4 * (8 * (((3 * M + u - 1) / u) * (D * u))) := by
        simp only [Nat.mul_assoc]
      _ = 32 * (((3 * M + u - 1) / u) * (D * u)) :=
        (Nat.mul_assoc 4 8 _).symm
      _ = 32 * D * (((3 * M + u - 1) / u) * u) := by ac_rfl
      _ ≤ 32 * D * (3 * M + u) := Nat.mul_le_mul_left _ hh
      _ = 96 * M * D + (32 * u) * D := by
        rw [Nat.mul_add]
        congr 1 <;> ac_rfl
      _ ≤ 96 * M * D + M * D :=
        Nat.add_le_add_left (Nat.mul_le_mul_right D hmargin) _
      _ = 97 * M * D := by simp only [Nat.mul_assoc]; omega
  have heval : 4 * (8 * M * (2 * u + 1) * u) ≤ 42 * M * D := by
    calc
      _ = 4 * (8 * (M * ((2 * u + 1) * u))) := by simp only [Nat.mul_assoc]
      _ = 32 * (M * ((2 * u + 1) * u)) := (Nat.mul_assoc 4 8 _).symm
      _ = 2 * (16 * (M * ((2 * u + 1) * u))) := Nat.mul_assoc 2 16 _
      _ = (2 * M) * (16 * u * (2 * u + 1)) := by ac_rfl
      _ ≤ (2 * M) * (21 * 2 ^ (u + 1)) :=
        Nat.mul_le_mul_left _ (root_evaluation_envelope u hu)
      _ ≤ (2 * M) * (21 * D) :=
        Nat.mul_le_mul_left _ (Nat.mul_le_mul_left 21 hD)
      _ = 42 * M * D := by ac_rfl
  rw [childPrecision_formula]
  change (p + 8 * ((3 * M + u - 1) / u) * D + 8 * M * (2 * u + 1)) * u ≤ 35 * M * D
  simp only [Nat.add_mul, Nat.mul_assoc] at *
  omega

/-- Root endpoint using the actual source length/array inequality. Source
values and successful checked arithmetic must still be bound by a consumer. -/
theorem root_precision_of_array (p M u i : Nat) (hp : p ≤ M) (hi : i < u)
    (hguard : 2 ^ (2 * u) * 64 ^ (4 * u) ≤ M) :
    childPrecision p M u (2 * u) i * u ≤ 35 * M * 2 ^ (2 * u - i) :=
  root_precision_mul_le p M u i hp (root_array_margin M u hguard) hi

end CryptoLeanCore.TaperedPrecision
