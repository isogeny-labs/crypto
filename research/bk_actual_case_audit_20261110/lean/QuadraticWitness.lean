import ThreeTorsion

namespace BKActualWitnessAudit
noncomputable section
open BKActualThreeAudit SemaevS3 FrobeniusPointDescentAudit
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {L : Type*} [Field L] [Algebra F L] [IsAlgClosed L]
local instance extensionChar : CharP L 5 := charP_of_injective_algebraMap' F 5

def beta : L := Classical.choose
  (IsAlgClosed.exists_pow_nat_eq (2:L) (n:=2) (by decide))

omit [Algebra F L] in
theorem beta_square : (beta (L:=L))^2=2 := Classical.choose_spec
  (IsAlgClosed.exists_pow_nat_eq (2:L) (n:=2) (by decide))

theorem beta_ne_zero : beta (L:=L)≠0 := by
  intro h
  apply two_ne_zero (L:=L)
  have hh := beta_square (L:=L)
  simpa only [h,zero_pow (by decide : (2:ℕ)≠0)] using hh.symm

theorem beta_fifth : (beta (L:=L))^5= -(beta (L:=L)) := by
  calc
    (beta (L:=L))^5=((beta (L:=L))^2)^2*(beta (L:=L)) := by ring
    _= -(beta (L:=L)) := by
      rw [beta_square]
      norm_num
      linear_combination (beta (L:=L))*(five_zero (L:=L))

theorem beta_not_fixed : (beta (L:=L))^5≠(beta (L:=L)) := by
  intro h
  rw [beta_fifth] at h
  have hz : (2:L)*(beta (L:=L))=0 := by linear_combination -h
  exact (mul_ne_zero (BKActualThreeAudit.two_ne_zero (L:=L)) (beta_ne_zero (L:=L))) hz

variable [DecidableEq L]

omit [DecidableEq L] in
theorem witness_nonsingular : (geomCurve (L:=L)).Nonsingular 1 (beta (L:=L)) := by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero
    geom_nonsingular).mp
  rw [WeierstrassCurve.Affine.equation_iff]
  obtain ⟨h1,h2,h3,h4,h6⟩ := coefficients (L:=L)
  simpa only [h1,h2,h3,h4,h6,zero_mul,zero_add,mul_zero,add_zero,one_pow,
    one_add_one_eq_two] using beta_square (L:=L)

def P : (geomCurve (L:=L)).Point := .some 1 (beta (L:=L)) witness_nonsingular

omit [DecidableEq L] in
theorem P_ne_zero : P (L:=L)≠0 := by simp [P]

theorem P_three : (3:ℕ) • P (L:=L)=0 := by
  obtain ⟨h1,h2,h3,h4,h6⟩ := coefficients (L:=L)
  apply TorsionBoundaryAudit.repeated_s3_three_torsion
    (geomCurve (L:=L)) h1 h2 h3 two_ne_zero witness_nonsingular
  rw [h4,h6,short_diagonal]
  norm_num
  linear_combination 3*(five_zero (L:=L))

theorem P_order : addOrderOf (P (L:=L))=3 := addOrderOf_eq_prime P_three P_ne_zero

theorem P_frobenius_neg : pointFrobenius curve (P (L:=L))= -P := by
  change WeierstrassCurve.Affine.Point.some
    (FiniteField.frobeniusAlgHom F L 1) (FiniteField.frobeniusAlgHom F L (beta (L:=L))) _=
      -WeierstrassCurve.Affine.Point.some 1 (beta (L:=L)) witness_nonsingular
  rw [WeierstrassCurve.Affine.Point.neg_some,
    WeierstrassCurve.Affine.Point.some.injEq]
  constructor
  · exact map_one _
  · change (beta (L:=L))^Fintype.card F=(geomCurve (L:=L)).negY 1 (beta (L:=L))
    obtain ⟨h1,h2,h3,h4,h6⟩ := coefficients (L:=L)
    simpa only [ZMod.card,WeierstrassCurve.Affine.negY,h1,h3,zero_mul,sub_zero]
      using beta_fifth (L:=L)

theorem P_not_fixed : pointFrobenius curve (P (L:=L))≠P := by
  intro h
  change WeierstrassCurve.Affine.Point.some
    (FiniteField.frobeniusAlgHom F L 1) (FiniteField.frobeniusAlgHom F L (beta (L:=L))) _=
      WeierstrassCurve.Affine.Point.some 1 (beta (L:=L)) witness_nonsingular at h
  have hh := (WeierstrassCurve.Affine.Point.some.inj h).2
  change (beta (L:=L))^Fintype.card F=(beta (L:=L)) at hh
  exact beta_not_fixed (by simpa only [ZMod.card] using hh)

theorem P_not_rational :
    ¬∃ Q : curve.Point,
      WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId F L) Q=P := by
  intro h
  exact P_not_fixed ((point_frobenius_fixed_iff curve P).mpr h)

#print axioms P_order
#print axioms P_frobenius_neg
#print axioms P_not_rational
end
end BKActualWitnessAudit
