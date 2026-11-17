module
public import NormLift
public import Mathlib.Algebra.Polynomial.Derivative
public import Mathlib.Algebra.Polynomial.Div
public import Mathlib.FieldTheory.Separable
public section
namespace NormWronskian
open Polynomial
variable {K : Type*} [Field K]

noncomputable def filter (G N : Polynomial K) : Polynomial K :=
  G*N.derivative - G.derivative*N

/-- Separability makes a zero differential filter force divisibility. -/
theorem filterDivisibility (G N : Polynomial K)
    (hcop : IsCoprime G G.derivative) (hw : filter G N = 0) : G ∣ N := by
  have heq : G*N.derivative = G.derivative*N := sub_eq_zero.mp hw
  apply hcop.dvd_of_dvd_mul_left
  rw [← heq]
  exact dvd_mul_right G N.derivative

/-- Within the prescribed degree window, the differential kernel is exactly span(G). -/
theorem filterZeroIff (G N : Polynomial K) (hm : G.Monic)
    (hcop : IsCoprime G G.derivative) (hd : N.natDegree ≤ G.natDegree) :
    filter G N = 0 ↔ ∃ c : K, N = C c*G := by
  constructor
  · intro hw
    exact ⟨N.leadingCoeff,
      eq_leadingCoeff_mul_of_monic_of_dvd_of_natDegree_le hm
        (filterDivisibility G N hcop hw) hd⟩
  · rintro ⟨c,rfl⟩
    simp only [filter, derivative_C_mul]
    ring

/-- Nonzero norm output recovers the required nonzero free scalar. -/
theorem nonzeroFilterIff (G N : Polynomial K) (hm : G.Monic)
    (hcop : IsCoprime G G.derivative) (hd : N.natDegree ≤ G.natDegree)
    (hn : N ≠ 0) : filter G N = 0 ↔ ∃ c : K, c ≠ 0 ∧ N = c • G := by
  rw [filterZeroIff G N hm hcop hd]
  constructor
  · rintro ⟨c,hc⟩
    refine ⟨c, ?_, ?_⟩
    · intro hz
      subst c
      simp at hc
      exact hn hc
    · simpa only [Polynomial.smul_eq_C_mul] using hc
  · rintro ⟨c,hc,heq⟩
    exact ⟨c, by simpa only [Polynomial.smul_eq_C_mul] using heq⟩

/-- Instantiated at the actual norm lift: the filter has precisely the same feasible matrices. -/
theorem liftedFilterIff {I : Type*} [Fintype I]
    (f G : Polynomial K) (A B : I → Polynomial K) (M : I → I → K)
    (hm : G.Monic) (hcop : IsCoprime G G.derivative)
    (hd : (NormLift.lift f A B M).natDegree ≤ G.natDegree)
    (hn : NormLift.lift f A B M ≠ 0) :
    filter G (NormLift.lift f A B M) = 0 ↔
      ∃ c : K, c ≠ 0 ∧ NormLift.lift f A B M = c • G :=
  nonzeroFilterIff G (NormLift.lift f A B M) hm hcop hd hn

/-- Differentiating the norm keeps its quadratic dependence on A and B. -/
theorem normDerivative (f A B : Polynomial K) :
    (A^2-f*B^2).derivative =
      C 2*A*A.derivative-f.derivative*B^2-C 2*f*B*B.derivative := by
  simp only [derivative_sub, derivative_mul, derivative_pow]
  norm_num
  ring

/-- The actual pole-degree constraints give the polynomial degree cap. -/
theorem normDegreeBound (f A B : Polynomial K) (D : ℕ)
    (ha : 2*A.natDegree ≤ D) (hb : f.natDegree+2*B.natDegree ≤ D) :
    (A^2-f*B^2).natDegree ≤ D := by
  apply (natDegree_sub_le (A^2) (f*B^2)).trans
  apply max_le
  · exact natDegree_pow_le.trans ha
  · exact (natDegree_mul_le.trans
      (Nat.add_le_add_left (natDegree_pow_le : (B^2).natDegree ≤ 2*B.natDegree)
        f.natDegree)).trans hb

/-- Distinct rational x nodes supply monicity and coprimality without a conjecture. -/
theorem splitProductConditions (S : Finset K) :
    (∏ x ∈ S, (X-C x : Polynomial K)).Monic ∧
    IsCoprime (∏ x ∈ S, (X-C x : Polynomial K))
      (∏ x ∈ S, (X-C x : Polynomial K)).derivative := by
  constructor
  · exact monic_prod_of_monic S (fun x => X-C x) (fun x hx => monic_X_sub_C x)
  · have hs : (∏ x ∈ S, (X-C x : Polynomial K)).Separable :=
      separable_prod_X_sub_C_iff'.mpr (fun x hx y hy hxy => hxy)
    exact hs

local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

/-- If the degree window is omitted, characteristic-p factors are invisible. -/
theorem frobeniusGhost (G H : Polynomial (ZMod 17)) : filter G (G*H^17)=0 := by
  simp only [filter, derivative_mul, derivative_pow, ZMod.natCast_self,
    C_0, zero_mul, mul_zero, add_zero]
  ring

noncomputable def ghost : Polynomial (ZMod 17) := X*(X-C 1)^17

theorem ghostPasses : filter X ghost=0 := frobeniusGhost X (X-C 1)

/-- This false positive is not any scalar multiple of G=X. -/
theorem ghostNotScalar : ¬∃ c : ZMod 17, ghost=C c*X := by
  rintro ⟨c,h⟩
  have h1 := congrArg (fun P : Polynomial (ZMod 17) => P.eval 1) h
  have hc : c=0 := by simpa [ghost] using h1.symm
  subst c
  have h2 := congrArg (fun P : Polynomial (ZMod 17) => P.eval 2) h
  norm_num [ghost] at h2
  exact (by decide : (2 : ZMod 17) ≠ 0) h2
end NormWronskian
#print axioms NormWronskian.filterDivisibility
#print axioms NormWronskian.filterZeroIff
#print axioms NormWronskian.nonzeroFilterIff
#print axioms NormWronskian.liftedFilterIff
#print axioms NormWronskian.normDerivative
#print axioms NormWronskian.frobeniusGhost
#print axioms NormWronskian.ghostPasses
#print axioms NormWronskian.ghostNotScalar
#print axioms NormWronskian.normDegreeBound
#print axioms NormWronskian.splitProductConditions
