import DoublingSurjective

namespace EllipticDoublingSurjectivityAudit
noncomputable section
open WeierstrassCurve.Affine
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem actual_dyadic_surjective (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ) :
    Function.Surjective (fun P : W.Point=>(2^n : ℕ) • P) := by
  induction n with
  | zero => intro Q;exact ⟨Q,by simp⟩
  | succ n ih =>
    intro Q
    obtain ⟨R,hR⟩:=ih Q
    obtain ⟨P,hP⟩:=actual_doubling_surjective W hΔ R
    change (2^n : ℕ) • R=Q at hR
    change P+P=R at hP
    refine ⟨P,?_⟩
    change (2^(n+1) : ℕ) • P=Q
    rw [pow_succ,Nat.mul_comm (2^n) 2,mul_nsmul,two_nsmul,hP,hR]

#print axioms actual_dyadic_surjective
end
end EllipticDoublingSurjectivityAudit
