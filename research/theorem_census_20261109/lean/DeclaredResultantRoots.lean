module
public import Mathlib.RingTheory.Polynomial.Resultant.Basic
public import Mathlib.FieldTheory.IsAlgClosed.Basic
public import Mathlib.Algebra.BigOperators.Ring.Multiset
public import Mathlib.Tactic

public section

/-! Fixed declared Sylvester sizes retain a common-leading-zero case.
The statement is an explicit finite-root/leading-coefficient alternative;
no projective space or geometric zero-sum theorem is assumed. -/
namespace DeclaredResultantRootsAudit
noncomputable section
open Polynomial
variable {K : Type*} [Field K]

theorem full_left_root_iff (f g : K[X]) (n : ℕ) (hf : f≠0)
    (hs : f.Splits) (hg : g.natDegree≤n) :
    Polynomial.resultant f g f.natDegree n=0 ↔ ∃ a : K,f.eval a=0 ∧ g.eval a=0 := by
  classical
  rw [Polynomial.resultant_eq_prod_eval f g n hg hs]
  have hc : f.leadingCoeff^n≠0 := pow_ne_zero n (Polynomial.leadingCoeff_ne_zero.mpr hf)
  simp [hc,Multiset.prod_eq_zero_iff,Polynomial.mem_roots hf,Polynomial.IsRoot]

theorem degree_lt_of_top_zero (f : K[X]) (m : ℕ) (hm : 0<m)
    (hf : f.natDegree≤m) (hc : f.coeff m=0) : f.natDegree<m := by
  by_cases hz : f=0
  · rw [hz,Polynomial.natDegree_zero]
    exact hm
  have hne : f.natDegree≠m := by
    intro heq
    apply Polynomial.leadingCoeff_ne_zero.mpr hz
    simpa only [Polynomial.leadingCoeff,heq] using hc
  omega

theorem declared_root_iff [IsAlgClosed K] (f g : K[X]) (m n : ℕ)
    (hm : 0<m) (hn : 0<n) (hf : f.natDegree≤m) (hg : g.natDegree≤n) :
    Polynomial.resultant f g m n=0 ↔
      (∃ a : K,f.eval a=0 ∧ g.eval a=0) ∨ (f.coeff m=0 ∧ g.coeff n=0) := by
  by_cases hfm : f.coeff m=0
  · by_cases hgn : g.coeff n=0
    · have hfd := degree_lt_of_top_zero f m hm hf hfm
      have hgd := degree_lt_of_top_zero g n hn hg hgn
      constructor
      · intro _
        exact Or.inr ⟨hfm,hgn⟩
      · intro _
        exact Polynomial.resultant_eq_zero_of_lt_lt f g m n hfd hgd
    · have hd : g.natDegree=n := Polynomial.natDegree_eq_of_le_of_coeff_ne_zero hg hgn
      have hg0 : g≠0 := by
        intro hz
        apply hgn
        rw [hz,coeff_zero]
      have he : Polynomial.resultant g f n m=0 ↔ ∃ a : K,g.eval a=0 ∧ f.eval a=0 := by
        rw [←hd]
        exact full_left_root_iff g f m hg0 (IsAlgClosed.splits g) hf
      constructor
      · intro hz
        have hr : Polynomial.resultant g f n m=0 := by
          rw [Polynomial.resultant_comm g f n m,hz,mul_zero]
        obtain ⟨a,hga,hfa⟩ := he.mp hr
        exact Or.inl ⟨a,hfa,hga⟩
      · rintro (⟨a,hfa,hga⟩ | ⟨_,hgz⟩)
        · have hr := he.mpr ⟨a,hga,hfa⟩
          rw [Polynomial.resultant_comm f g m n,hr,mul_zero]
        · exact (hgn hgz).elim
  · have hd : f.natDegree=m := Polynomial.natDegree_eq_of_le_of_coeff_ne_zero hf hfm
    have hf0 : f≠0 := by
      intro hz
      apply hfm
      rw [hz,coeff_zero]
    have he : Polynomial.resultant f g m n=0 ↔ ∃ a : K,f.eval a=0 ∧ g.eval a=0 := by
      rw [←hd]
      exact full_left_root_iff f g n hf0 (IsAlgClosed.splits f) hg
    constructor
    · intro hz
      exact Or.inl (he.mp hz)
    · rintro (hroot | ⟨hfz,_⟩)
      · exact he.mpr hroot
      · exact (hfm hfz).elim

theorem constant_padding_control :
    Polynomial.resultant (C (1:K)) (C (1:K)) 2 2=0 ∧
      ¬ ∃ a : K,(C (1:K)).eval a=0 ∧ (C (1:K)).eval a=0 := by
  constructor
  · simp [Polynomial.coeff_one]
  · simp

theorem affine_only_claim_false :
    ¬ ∀ f g : K[X],f.natDegree≤2 → g.natDegree≤2 →
      Polynomial.resultant f g 2 2=0 → ∃ a : K,f.eval a=0 ∧ g.eval a=0 := by
  intro h
  exact constant_padding_control.2
    (h (C 1) (C 1) (by simp) (by simp) constant_padding_control.1)

def rationalNoRoot : ℚ[X] := X^2+1

theorem rationalNoRoot_degree : rationalNoRoot.natDegree=2 := by
  unfold rationalNoRoot
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
  · exact (Polynomial.natDegree_add_le _ _).trans (by simp)
  · simp [Polynomial.coeff_one]

theorem rational_control : Polynomial.resultant rationalNoRoot rationalNoRoot 2 2=0 ∧
    rationalNoRoot.coeff 2≠0 ∧ ¬ ∃ a : ℚ,rationalNoRoot.eval a=0 ∧ rationalNoRoot.eval a=0 := by
  refine ⟨?_,?_,?_⟩
  · simpa only [rationalNoRoot_degree] using
      (Polynomial.resultant_self_eq_zero rationalNoRoot (by rw [rationalNoRoot_degree]; decide))
  · simp [rationalNoRoot,Polynomial.coeff_one]
  · rintro ⟨a,ha,_⟩
    simp only [rationalNoRoot,Polynomial.eval_add,Polynomial.eval_pow,Polynomial.eval_X,
      Polynomial.eval_one] at ha
    nlinarith [sq_nonneg a]

theorem declared_resultant_root_audit_certificate :
    (∀ (F : Type*) [Field F] [IsAlgClosed F] (f g : F[X]) (m n : ℕ),
      0<m → 0<n → f.natDegree≤m → g.natDegree≤n →
      (Polynomial.resultant f g m n=0 ↔
        (∃ a : F,f.eval a=0 ∧ g.eval a=0) ∨ (f.coeff m=0 ∧ g.coeff n=0))) ∧
    (∀ (F : Type*) [Field F], ¬ ∀ f g : F[X],f.natDegree≤2 → g.natDegree≤2 →
      Polynomial.resultant f g 2 2=0 → ∃ a : F,f.eval a=0 ∧ g.eval a=0) ∧
    (Polynomial.resultant rationalNoRoot rationalNoRoot 2 2=0 ∧
      rationalNoRoot.coeff 2≠0 ∧ ¬ ∃ a : ℚ,rationalNoRoot.eval a=0 ∧ rationalNoRoot.eval a=0) := by
  refine ⟨?_,?_,rational_control⟩
  · intro F _ _ f g m n hm hn hf hg
    exact declared_root_iff f g m n hm hn hf hg
  · intro F _
    exact affine_only_claim_false (K:=F)

#print axioms declared_resultant_root_audit_certificate
#print axioms declared_root_iff
#print axioms affine_only_claim_false
#print axioms rational_control
end
end DeclaredResultantRootsAudit
