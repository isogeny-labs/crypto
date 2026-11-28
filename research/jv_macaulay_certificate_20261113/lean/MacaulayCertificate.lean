import CubicNormalForms

namespace JVMacaulayAudit
open MvPolynomial
noncomputable section

theorem map_f0 {R S : Type*} [CommRing R] [CommRing S]
    (f : R →+* S) (x y h : R) : f (f0 x y h)=f0 (f x) (f y) (f h) := by
  simp [f0,map_ofNat]
theorem map_f1 {R S : Type*} [CommRing R] [CommRing S]
    (f : R →+* S) (x y h : R) : f (f1 x y h)=f1 (f x) (f y) (f h) := by
  simp [f1,map_ofNat]
theorem map_f2 {R S : Type*} [CommRing R] [CommRing S]
    (f : R →+* S) (x y h : R) : f (f2 x y h)=f2 (f x) (f y) (f h) := by
  simp [f2,map_ofNat]

theorem sourceIdeal_le_eval_kernel :
    sourceIdeal ≤ RingHom.ker (MvPolynomial.eval (0 : Fin 3→F)) := by
  rw [sourceIdeal,Ideal.span_le]
  intro p hp
  change MvPolynomial.eval (0 : Fin 3→F) p=0
  simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
  rcases hp with rfl | rfl | rfl
  all_goals simp [f0,f1,f2]

theorem sourceIdeal_proper : sourceIdeal≠⊤ := by
  intro h
  have hm : (1:Poly)∈sourceIdeal := by rw [h]; trivial
  have hk := sourceIdeal_le_eval_kernel hm
  change MvPolynomial.eval (0 : Fin 3→F) 1=0 at hk
  simp at hk

abbrev CoordinateAlgebra := Poly ⧸ sourceIdeal
instance : Nontrivial CoordinateAlgebra := Ideal.Quotient.nontrivial_iff.mpr sourceIdeal_proper
instance : CharP CoordinateAlgebra 101 := charP_of_injective_algebraMap' F 101

def qCoordinate (i : Fin 3) : CoordinateAlgebra := Ideal.Quotient.mk sourceIdeal (X i)

theorem actual_quotient_equations :
    f0 (qCoordinate 0) (qCoordinate 1) (qCoordinate 2)=0 ∧
    f1 (qCoordinate 0) (qCoordinate 1) (qCoordinate 2)=0 ∧
    f2 (qCoordinate 0) (qCoordinate 1) (qCoordinate 2)=0 := by
  have h0 : f0 (X 0) (X 1) (X 2)∈sourceIdeal := Ideal.subset_span (by simp)
  have h1 : f1 (X 0) (X 1) (X 2)∈sourceIdeal := Ideal.subset_span (by simp)
  have h2 : f2 (X 0) (X 1) (X 2)∈sourceIdeal := Ideal.subset_span (by simp)
  exact ⟨by simpa only [map_f0,qCoordinate] using Ideal.Quotient.eq_zero_iff_mem.mpr h0,
    by simpa only [map_f1,qCoordinate] using Ideal.Quotient.eq_zero_iff_mem.mpr h1,
    by simpa only [map_f2,qCoordinate] using Ideal.Quotient.eq_zero_iff_mem.mpr h2⟩

/-- A statement about the actual ideal, not merely an assumed Gröbner basis
or a finite set of rational zeros. Every degree-four monomial vanishes in
its actual quotient, using the checked source-equation relations. -/
theorem actual_degree_four_monomials_mem (i j k : ℕ) (hs : i+j+k=4) :
    (X 0^i*X 1^j*X 2^k : Poly)∈sourceIdeal := by
  apply Ideal.Quotient.eq_zero_iff_mem.mp
  have h := all_degree_four_vanish (qCoordinate 0) (qCoordinate 1) (qCoordinate 2)
    actual_quotient_equations.1 actual_quotient_equations.2.1
    actual_quotient_equations.2.2 i j k hs
  simpa only [map_mul,map_pow,qCoordinate] using h

theorem actual_all_high_monomials_mem (i j k : ℕ) (hs : 4 ≤ i+j+k) :
    (X 0^i*X 1^j*X 2^k : Poly)∈sourceIdeal := by
  let a := min i 4
  let b := min j (4-a)
  let c := 4-a-b
  have ha : a ≤ i := Nat.min_le_left _ _
  have hb : b ≤ j := Nat.min_le_left _ _
  have hc : c ≤ k := by dsimp [a,b,c]; omega
  have hsum : a+b+c=4 := by dsimp [a,b,c]; omega
  have hbase := actual_degree_four_monomials_mem a b c hsum
  have hmul : (X 0^i*X 1^j*X 2^k : Poly)=
      (X 0^(i-a)*X 1^(j-b)*X 2^(k-c))*(X 0^a*X 1^b*X 2^c) := by
    have hi : i=(i-a)+a := by omega
    have hj : j=(j-b)+b := by omega
    have hk : k=(k-c)+c := by omega
    conv_lhs => rw [hi,hj,hk,pow_add,pow_add,pow_add]
    ring
  rw [hmul]
  exact sourceIdeal.mul_mem_left _ hbase

/-- Closed literal-source certificate. No generic regularity or native
F4 execution premise occurs. This is an upper cutoff, not a proof that
four is minimal or that the native implementation never processes a
higher-degree zero reduction. -/
theorem literal_jv_macaulay_upper_certificate :
    affineIdeal=⊤ ∧ sourceIdeal≠⊤ ∧
      ∀ i j k : ℕ, 4 ≤ i+j+k → (X 0^i*X 1^j*X 2^k : Poly)∈sourceIdeal :=
  ⟨actual_affine_ideal_is_unit,sourceIdeal_proper,actual_all_high_monomials_mem⟩

#print axioms literal_jv_macaulay_upper_certificate

end
end JVMacaulayAudit
