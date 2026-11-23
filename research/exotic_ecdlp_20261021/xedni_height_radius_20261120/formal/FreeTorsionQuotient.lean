import Mathlib

/-! The exact finite quotient of a free abelian group plus finite torsion. -/

namespace XedniFreeTorsionQuotient

variable {A : Type*} [AddCommGroup A] [Fintype A]
variable (d N : ℕ) [NeZero N]

abbrev LiftGroup := (Fin d → ℤ) × A

abbrev ResidueGroup := (Fin d → ZMod N) × A

/-- Reduce only the free coordinates and retain the torsion coordinate. -/
def quotientMap : LiftGroup (A := A) d →+ ResidueGroup (A := A) d N where
  toFun z := (fun i => (z.1 i : ZMod N), z.2)
  map_zero' := by
    ext i <;> simp
  map_add' x y := by
    ext i <;> simp

omit [AddCommGroup A] in
/-- The quotient has exactly `|A| * N^d` elements. -/
theorem quotient_card :
    Fintype.card (ResidueGroup (A := A) d N) = Fintype.card A * N ^ d := by
  simp [ResidueGroup, mul_comm]

omit [Fintype A] [NeZero N] in
/-- Every kernel point has zero torsion coordinate. -/
theorem kernel_torsion_zero (z : LiftGroup (A := A) d)
    (hz : quotientMap (A := A) d N z = 0) : z.2 = 0 := by
  exact congrArg Prod.snd hz

omit [Fintype A] in
/-- Every residue class has a lift with the same torsion coordinate. -/
theorem quotient_surjective : Function.Surjective (quotientMap (A := A) d N) := by
  intro y
  refine ⟨((fun i => ((y.1 i).val : ℤ)), y.2), ?_⟩
  apply Prod.ext
  · funext i
    simp [quotientMap]
  · rfl

omit [Fintype A] [NeZero N] in
/-- A kernel point is an `N`-multiple, with a witness in the free summand. -/
theorem kernel_divisible (z : LiftGroup (A := A) d)
    (hz : quotientMap (A := A) d N z = 0) :
    ∃ u : LiftGroup (A := A) d, z = N • u := by
  have hcoord : ∀ i : Fin d, (N : ℤ) ∣ z.1 i := by
    intro i
    have hi : (z.1 i : ZMod N) = 0 := by
      have := congrArg (fun f : Fin d → ZMod N => f i) (congrArg Prod.fst hz)
      simpa [quotientMap] using this
    exact (ZMod.intCast_zmod_eq_zero_iff_dvd (z.1 i) N).mp hi
  choose u hu using hcoord
  refine ⟨(u, 0), ?_⟩
  apply Prod.ext
  · funext i
    have hi := hu i
    simpa [nsmul_eq_mul] using hi
  · simpa using kernel_torsion_zero d N z hz

omit [Fintype A] [NeZero N] in
/-- If `N` annihilates the finite summand, a nonzero `N`-multiple has a
nonzero free coordinate and inherits the minimum free-coordinate norm. -/
theorem min_norm_of_nonzero_multiple (freeNorm : (Fin d → ℤ) → ℝ) (m : ℝ)
    (hkill : ∀ a : A, N • a = 0)
    (hmin : ∀ f, f ≠ 0 → m ≤ freeNorm f)
    (u : LiftGroup (A := A) d) (hu : N • u ≠ 0) :
    m ≤ freeNorm u.1 := by
  apply hmin
  intro hfree
  apply hu
  apply Prod.ext
  · simp [hfree]
  · simpa using hkill u.2

#print axioms XedniFreeTorsionQuotient.quotient_card
#print axioms XedniFreeTorsionQuotient.kernel_torsion_zero
#print axioms XedniFreeTorsionQuotient.quotient_surjective
#print axioms XedniFreeTorsionQuotient.kernel_divisible
#print axioms XedniFreeTorsionQuotient.min_norm_of_nonzero_multiple

end XedniFreeTorsionQuotient
