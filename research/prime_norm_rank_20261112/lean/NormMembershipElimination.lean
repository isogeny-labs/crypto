module
public import NormPublicJets
public section
namespace NormMembershipElimination
open Polynomial
variable {R : Type*} [CommRing R]

noncomputable def completeTarget (D B : R[X]) (q z : R) : R[X] :=
  D+C (z*B.eval q-D.eval q)

/-- The target constant is constructed from the four retained coefficient values. -/
theorem completionRow (D B : R[X]) (q z : R) :
    (completeTarget D B q z).eval q=z*B.eval q := by
  simp [completeTarget]

/-- The target row determines the eliminated constant uniquely. -/
theorem completionUnique (D B : R[X]) (q z c : R) :
    (D+C c).eval q=z*B.eval q ↔ c=z*B.eval q-D.eval q := by
  simp only [eval_add,eval_C]
  constructor <;> intro h <;> linear_combination h

/-- Actual target incidence and the actual target curve equation force exact norm division. -/
theorem normAtTarget (f A B : R[X]) (q z : R)
    (hp : z^2=f.eval q) (hr : A.eval q=z*B.eval q) :
    (A^2-f*B^2).eval q=0 := by
  simp only [eval_sub,eval_pow,eval_mul,hr,← hp]
  ring

/-- The actual norm quotient is computed, not supplied as a successful witness. -/
theorem canonicalNormQuotient (f A B : R[X]) (q z : R)
    (hp : z^2=f.eval q) (hr : A.eval q=z*B.eval q) :
    (X-C q)*((A^2-f*B^2) /ₘ (X-C q))=A^2-f*B^2 := by
  apply mul_divByMonic_eq_iff_isRoot.mpr
  exact normAtTarget f A B q z hp hr

/-- Any old norm-factor variable equals the computed monic-division quotient. -/
theorem normQuotientUnique (N U : R[X]) (q : R) :
    N=(X-C q)*U → U=N /ₘ (X-C q) := by
  intro h
  rw [h,mul_divByMonic_cancel_left U (monic_X_sub_C q)]

/-- Every bounded quotient/remainder auxiliary pair is the actual monic division result. -/
theorem divisionUnique (P U Q V : R[X]) (hU : U.Monic)
    (he : P=U*Q+V) (hv : V.degree<U.degree) :
    Q=P /ₘ U ∧ V=P %ₘ U := by
  have h := div_modByMonic_unique (f := P) (g := U) Q V hU ⟨by rw [he]; ring,hv⟩
  exact ⟨h.1.symm,h.2.symm⟩

noncomputable def master (s : R) : ℕ → R[X]
  | 0 => X
  | n+1 => master s n^2-C s

noncomputable def reduced (U : R[X]) (s : R) : ℕ → R[X]
  | 0 => X %ₘ U
  | n+1 => (reduced U s n^2-C s) %ₘ U

/-- A modular squaring step preserves the same actual polynomial congruence. -/
theorem reducedStep (P U : R[X]) (s : R) (hU : U.Monic) :
    ((P %ₘ U)^2-C s) %ₘ U=(P^2-C s) %ₘ U := by
  apply modByMonic_eq_of_dvd_sub hU
  have hd := dvd_mul_right (P %ₘ U-P) (P %ₘ U+P)
  have hx : U ∣ (P %ₘ U-P)*(P %ₘ U+P) :=
    dvd_trans (dvd_modByMonic_sub P U) hd
  convert hx using 1
  ring

/-- The complete modular tower equals the direct master-polynomial remainder. -/
theorem reducedMaster (U : R[X]) (s : R) (hU : U.Monic) (n : ℕ) :
    reduced U s n=master s n %ₘ U := by
  induction n with
  | zero => rfl
  | succ n ih =>
    simp only [reduced,master,ih]
    exact reducedStep _ U s hU

/-- Canonical auxiliaries satisfy the actual polynomial recurrence identically. -/
theorem canonicalTowerLift (U : R[X]) (s : R) (n : ℕ) :
    reduced U s n^2-C s=U*((reduced U s n^2-C s) /ₘ U)+reduced U s (n+1) := by
  rw [reduced]
  exact (modByMonic_add_div _ _).symm.trans (add_comm _ _)

/-- Canonical auxiliary remainders meet the actual degree guard. -/
theorem canonicalTowerBound [Nontrivial R] (U : R[X]) (s : R) (hU : U.Monic) (n : ℕ) :
    (reduced U s (n+1)).degree<U.degree :=
  degree_modByMonic_lt _ hU

/-- All old bounded auxiliary remainders and quotients are uniquely recoverable. -/
theorem towerUnique (U : R[X]) (s : R) (hU : U.Monic) (V Q : ℕ → R[X])
    (h0 : V 0=X %ₘ U)
    (he : ∀ n, V n^2-C s=U*Q n+V (n+1))
    (hv : ∀ n, (V (n+1)).degree<U.degree) :
    (∀ n, V n=reduced U s n) ∧
      (∀ n, Q n=(reduced U s n^2-C s) /ₘ U) := by
  have hall : ∀ n, V n=reduced U s n := by
    intro n
    induction n with
    | zero => exact h0
    | succ n ih =>
      rw [(divisionUnique _ U (Q n) (V (n+1)) hU (he n) (hv n)).2,ih]
      rfl
  refine ⟨hall,fun n => ?_⟩
  rw [(divisionUnique _ U (Q n) (V (n+1)) hU (he n) (hv n)).1,hall n]

/-- Terminal membership is exactly divisibility by the actual master polynomial. -/
theorem terminalMembership [Nontrivial R] (U : R[X]) (s c : R) (hU : U.Monic)
    (hdeg : 0<U.degree) (n : ℕ) :
    reduced U s n=C c ↔ U ∣ master s n-C c := by
  have hc : C c %ₘ U=C c :=
    (modByMonic_eq_self_iff hU).mpr (degree_C_le.trans_lt hdeg)
  have he : (master s n-C c) %ₘ U=reduced U s n-C c := by
    rw [sub_modByMonic,← reducedMaster U s hU n,hc]
  rw [← sub_eq_zero,← he]
  exact modByMonic_eq_zero_iff_dvd hU

section Field
variable {K : Type*} [Field K]

/-- Completing only the constant preserves the actual monic cubic window. -/
theorem completionWindow (D B : K[X]) (q z : K) (hD : D.Monic)
    (hd : D.natDegree=3) :
    (completeTarget D B q z).Monic ∧ (completeTarget D B q z).natDegree=3 := by
  have hlow : (C (z*B.eval q-D.eval q)).natDegree<D.natDegree := by
    rw [natDegree_C,hd]
    decide
  refine ⟨hD.add_of_left ?_,?_⟩
  · rw [degree_eq_natDegree hD.ne_zero]
    exact degree_le_natDegree.trans_lt (by exact_mod_cast hlow)
  · rw [completeTarget,natDegree_add_eq_left_of_natDegree_lt hlow,hd]

/-- A degree5 monic modulus and a degree4 remainder give degree<=3 quotient auxiliaries. -/
theorem quotientWindow (U V : K[X]) (s : K) (hU : U.Monic)
    (hu : U.natDegree=5) (hv : V.natDegree≤4) :
    ((V^2-C s) /ₘ U).natDegree≤3 := by
  have hp : (V^2-C s).natDegree≤8 := by
    apply (natDegree_sub_le _ _).trans
    apply max_le
    · rw [natDegree_pow]
      omega
    · simp
  rw [natDegree_divByMonic _ hU,hu]
  omega

/-- The m=5 pole window makes the norm monic of degree6 from the actual coefficient bounds. -/
theorem normWindow (f A B : K[X]) (hf : f.natDegree=3) (hA : A.Monic)
    (ha : A.natDegree=3) (hb : B.natDegree≤1) :
    (A^2-f*B^2).Monic ∧ (A^2-f*B^2).natDegree=6 := by
  have hlow : (f*B^2).natDegree≤5 := by
    calc (f*B^2).natDegree ≤ f.natDegree+(B^2).natDegree := natDegree_mul_le
      _ = 3+2*B.natDegree := by rw [hf,natDegree_pow]
      _ ≤ 5 := by omega
  have hhigh : (A^2).natDegree=6 := by rw [natDegree_pow,ha]
  have hlt : (f*B^2).natDegree<(A^2).natDegree := by omega
  refine ⟨(hA.pow 2).sub_of_left ?_,?_⟩
  · rw [degree_eq_natDegree (pow_ne_zero 2 hA.ne_zero)]
    exact degree_le_natDegree.trans_lt (by exact_mod_cast hlt)
  · rw [natDegree_sub_eq_left_of_natDegree_lt hlt,hhigh]

/-- The eliminated actual support quotient is monic of degree5 in the m=5 chart. -/
theorem supportWindow (f A B : K[X]) (q z : K) (hf : f.natDegree=3)
    (hA : A.Monic) (ha : A.natDegree=3) (hb : B.natDegree≤1)
    (hp : z^2=f.eval q) (hr : A.eval q=z*B.eval q) :
    ((A^2-f*B^2) /ₘ (X-C q)).Monic ∧
      ((A^2-f*B^2) /ₘ (X-C q)).natDegree=5 := by
  have hN := normWindow f A B hf hA ha hb
  have he := canonicalNormQuotient f A B q z hp hr
  refine ⟨(monic_X_sub_C q).of_mul_monic_left (he.symm ▸ hN.1),?_⟩
  rw [natDegree_divByMonic _ (monic_X_sub_C q),hN.2,natDegree_X_sub_C]

end Field
#print axioms completionRow
#print axioms completionUnique
#print axioms normAtTarget
#print axioms canonicalNormQuotient
#print axioms normQuotientUnique
#print axioms divisionUnique
#print axioms reducedStep
#print axioms reducedMaster
#print axioms canonicalTowerLift
#print axioms canonicalTowerBound
#print axioms towerUnique
#print axioms terminalMembership
#print axioms normWindow
#print axioms supportWindow
#print axioms completionWindow
#print axioms quotientWindow
end NormMembershipElimination
