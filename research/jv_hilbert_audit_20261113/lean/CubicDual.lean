import MacaulayCertificate

namespace JVHilbertAudit
open MvPolynomial
noncomputable section
abbrev F := JVMacaulayAudit.F
abbrev Poly := JVMacaulayAudit.Poly

def exponent (i j k : ℕ) : Fin 3 →₀ ℕ :=
  Finsupp.single 0 i+Finsupp.single 1 j+Finsupp.single 2 k

theorem exponent_add (i j k a b c : ℕ) :
    exponent i j k+exponent a b c=exponent (i+a) (j+b) (k+c) := by
  ext t
  fin_cases t <;> simp [exponent]

theorem exponent_sub (i j k a b c : ℕ) :
    exponent i j k-exponent a b c=exponent (i-a) (j-b) (k-c) := by
  ext t
  fin_cases t <;> simp [exponent]

theorem exponent_le (i j k a b c : ℕ) :
    exponent i j k ≤ exponent a b c ↔ i ≤ a ∧ j ≤ b ∧ k ≤ c := by
  constructor
  · intro h
    have h0 := h 0
    have h1 := h 1
    have h2 := h 2
    simpa [exponent] using And.intro h0 (And.intro h1 h2)
  · rintro ⟨hi,hj,hk⟩ t
    fin_cases t <;> simp [exponent] <;> assumption

theorem exponent_eq (i j k a b c : ℕ) :
    exponent i j k=exponent a b c ↔ i=a ∧ j=b ∧ k=c := by
  constructor
  · intro h
    have h0 := congrArg (fun e => e 0) h
    have h1 := congrArg (fun e => e 1) h
    have h2 := congrArg (fun e => e 2) h
    simpa [exponent] using And.intro h0 (And.intro h1 h2)
  · rintro ⟨rfl,rfl,rfl⟩
    rfl

theorem exponent_monomial (i j k : ℕ) (c : F) :
    (monomial (exponent i j k) c : Poly)=C c*X 0^i*X 1^j*X 2^k := by
  simp only [exponent,monomial_add_single,← C_mul_X_pow_eq_monomial]

/-- A genuine F101-linear functional on every polynomial, using ten
degree-three coefficients. No ideal-membership oracle is assumed. -/
def cubicDual : Poly →ₗ[F] F :=
  (79:F) • lcoeff F (exponent 3 0 0)+(36:F) • lcoeff F (exponent 2 1 0)+
  (71:F) • lcoeff F (exponent 1 2 0)+(79:F) • lcoeff F (exponent 0 3 0)+
  (93:F) • lcoeff F (exponent 2 0 1)+(77:F) • lcoeff F (exponent 1 1 1)+
  (47:F) • lcoeff F (exponent 0 2 1)+(35:F) • lcoeff F (exponent 1 0 2)+
  (19:F) • lcoeff F (exponent 0 1 2)+lcoeff F (exponent 0 0 3)

theorem cubicDual_apply (p : Poly) :
    cubicDual p=79*p.coeff (exponent 3 0 0)+36*p.coeff (exponent 2 1 0)+
      71*p.coeff (exponent 1 2 0)+79*p.coeff (exponent 0 3 0)+
      93*p.coeff (exponent 2 0 1)+77*p.coeff (exponent 1 1 1)+
      47*p.coeff (exponent 0 2 1)+35*p.coeff (exponent 1 0 2)+
      19*p.coeff (exponent 0 1 2)+p.coeff (exponent 0 0 3) := by
  simp [cubicDual,smul_eq_mul]

theorem actual_source_as_monomials :
    JVMacaulayAudit.f0 (X 0) (X 1) (X 2)=
      (monomial (exponent 2 0 0) 100+monomial (exponent 1 1 0) 32+
        monomial (exponent 1 0 1) 55+monomial (exponent 0 2 0) 1+
        monomial (exponent 0 1 1) 17+monomial (exponent 0 0 2) 81 : Poly) ∧
    JVMacaulayAudit.f1 (X 0) (X 1) (X 2)=
      (monomial (exponent 2 0 0) 29+monomial (exponent 1 1 0) 34+
        monomial (exponent 1 0 1) 71+monomial (exponent 0 1 1) 83+
        monomial (exponent 0 0 2) 16 : Poly) ∧
    JVMacaulayAudit.f2 (X 0) (X 1) (X 2)=
      (monomial (exponent 2 0 0) 59+monomial (exponent 1 1 0) 27+
        monomial (exponent 1 0 1) 31+monomial (exponent 0 1 1) 48+
        monomial (exponent 0 0 2) 32 : Poly) := by
  refine ⟨?_,?_,?_⟩
  all_goals simp [exponent_monomial,JVMacaulayAudit.f0,JVMacaulayAudit.f1,
    JVMacaulayAudit.f2]
  all_goals norm_num only [map_ofNat]

theorem cubicDual_annihilates_source_multiples (p : Poly) :
    cubicDual (p*JVMacaulayAudit.f0 (X 0) (X 1) (X 2))=0 ∧
    cubicDual (p*JVMacaulayAudit.f1 (X 0) (X 1) (X 2))=0 ∧
    cubicDual (p*JVMacaulayAudit.f2 (X 0) (X 1) (X 2))=0 := by
  have : CharP F 101 := inferInstance
  rw [actual_source_as_monomials.1,actual_source_as_monomials.2.1,
    actual_source_as_monomials.2.2]
  refine ⟨?_,?_,?_⟩
  all_goals simp only [cubicDual_apply,mul_add,AddMonoidAlgebra.coeff_add,
    Finsupp.add_apply,coeff_mul_monomial',exponent_le,exponent_sub]
  all_goals norm_num
  all_goals ring_nf
  all_goals reduce_mod_char!

end
end JVHilbertAudit
