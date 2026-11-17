import CubicDual

namespace JVHilbertAudit
open MvPolynomial
noncomputable section

/-- The annihilator of the complete multiplication pairing induced by
the cubic functional. Its ideal structure is proved, not assumed. -/
def dualAnnihilator : Ideal Poly where
  carrier := {p | ∀ g, cubicDual (g*p)=0}
  zero_mem' := by intro g; simp
  add_mem' := by
    intro p q hp hq g
    rw [mul_add,map_add,hp g,hq g,add_zero]
  smul_mem' := by
    intro c p hp g
    change cubicDual (g*(c*p))=0
    rw [← mul_assoc]
    exact hp (g*c)

theorem sourceIdeal_le_dualAnnihilator :
    JVMacaulayAudit.sourceIdeal ≤ dualAnnihilator := by
  rw [JVMacaulayAudit.sourceIdeal,Ideal.span_le]
  intro p hp
  simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
  rcases hp with rfl | rfl | rfl
  · intro g
    exact (cubicDual_annihilates_source_multiples g).1
  · intro g
    exact (cubicDual_annihilates_source_multiples g).2.1
  · intro g
    exact (cubicDual_annihilates_source_multiples g).2.2

theorem cubicDual_vanishes_on_sourceIdeal {p : Poly}
    (hp : p∈JVMacaulayAudit.sourceIdeal) : cubicDual p=0 := by
  have h : ∀ g, cubicDual (g*p)=0 := sourceIdeal_le_dualAnnihilator hp
  simpa using h 1

theorem cubicDual_hcube : cubicDual (X 2^3)=1 := by
  have he : (X 2^3 : Poly)=monomial (exponent 0 0 3) 1 := by
    simp [exponent,X_pow_eq_monomial]
  rw [he,cubicDual_apply]
  simp [coeff_monomial,exponent_eq,eq_comm]

theorem source_hcube_not_mem : (X 2^3 : Poly)∉JVMacaulayAudit.sourceIdeal := by
  intro hp
  have hz := cubicDual_vanishes_on_sourceIdeal hp
  rw [cubicDual_hcube] at hz
  exact one_ne_zero hz

theorem actual_quotient_hcube_nonzero : (JVMacaulayAudit.qCoordinate 2)^3≠0 := by
  intro hz
  apply source_hcube_not_mem
  apply Ideal.Quotient.eq_zero_iff_mem.mp
  simpa only [map_pow,JVMacaulayAudit.qCoordinate] using hz

theorem literal_jv_sharp_monomial_cutoff :
    (X 2^3 : Poly)∉JVMacaulayAudit.sourceIdeal ∧
    ∀ d : ℕ, (∀ i j k : ℕ, d ≤ i+j+k →
      (X 0^i*X 1^j*X 2^k : Poly)∈JVMacaulayAudit.sourceIdeal) ↔ 4 ≤ d := by
  refine ⟨source_hcube_not_mem,?_⟩
  intro d
  constructor
  · intro hd
    by_contra hn
    have hm := hd 0 0 3 (by omega)
    exact source_hcube_not_mem (by simpa using hm)
  · intro hd i j k hs
    exact JVMacaulayAudit.actual_all_high_monomials_mem i j k (by omega)

#print axioms literal_jv_sharp_monomial_cutoff

end
end JVHilbertAudit
