import CantorTransport
import Mathlib.FieldTheory.Finite.GaloisField
import Mathlib.NumberTheory.LegendreSymbol.QuadraticChar.Basic

namespace MumfordFixedGenusAudit
open Polynomial
noncomputable section
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide +kernel⟩

abbrev FamilyField (t : ℕ) := GaloisField 3 (2*t+1)
noncomputable instance familyFintype (t : ℕ) : Fintype (FamilyField t) := Fintype.ofFinite _

def coefficientMap (t : ℕ) : Base →+* FamilyField t := algebraMap _ _

theorem family_card (t : ℕ) : Nat.card (FamilyField t)=3^(2*t+1) :=
  GaloisField.card 3 _ (by omega)

theorem odd_power_mod_four (t : ℕ) : (3^(2*t+1))%4=3 := by
  rw [pow_succ,pow_mul,Nat.mul_mod,Nat.pow_mod]
  norm_num

theorem family_nonsquare_neg_one (t : ℕ) : ¬IsSquare (-1 : FamilyField t) := by
  intro hs
  have h := FiniteField.isSquare_neg_one_iff.mp hs
  have hc : Fintype.card (FamilyField t)%4=3 := by
    rw [← Nat.card_eq_fintype_card,family_card,odd_power_mod_four]
  exact h hc

theorem quadratic_no_root (t : ℕ) (x : FamilyField t) :
    Polynomial.eval x (X^2+1 : (FamilyField t)[X])≠0 := by
  intro he
  have hx : x^2= -1 := by simpa only [eval_add,eval_pow,eval_X,eval_one,
    add_eq_zero_iff_eq_neg] using he
  exact family_nonsquare_neg_one t ⟨x,by simpa only [pow_two] using hx.symm⟩

theorem quadratic_nonsplit (t : ℕ) : ¬(X^2+1 : (FamilyField t)[X]).Splits := by
  intro hs
  have hd : (X^2+1 : (FamilyField t)[X]).natDegree=2 := by compute_degree!
  obtain ⟨x,hx⟩ := hs.exists_eval_eq_zero (degree_ne_of_natDegree_ne (by rw [hd]; decide))
  exact quadratic_no_root t x hx

theorem nonsmooth_point_coordinates (t : ℕ) :
    (extend (coefficientMap t) (MumfordCantorAudit.point 7)).u=X^2+1 := by
  simp [extend,MumfordCantorAudit.point,MumfordSmoothnessAudit.candidatePair,
    MumfordSmoothnessAudit.representative,MumfordSmoothnessAudit.uPolynomial]

theorem actual_nonsmooth_point (t : ℕ) :
    ¬Smooth (extend (coefficientMap t) (MumfordCantorAudit.point 7)) := by
  unfold Smooth
  rw [nonsmooth_point_coordinates]
  exact quadratic_nonsplit t

theorem fields_unbounded (B : ℕ) : ∃ t : ℕ,B < Nat.card (FamilyField t) := by
  refine ⟨B,?_⟩
  rw [family_card]
  have h : B < 3^B := Nat.lt_pow_self (by decide)
  exact h.trans_le (Nat.pow_le_pow_right (by decide : 0<3) (by omega))

#print axioms actual_nonsmooth_point
#print axioms fields_unbounded
end
end MumfordFixedGenusAudit
