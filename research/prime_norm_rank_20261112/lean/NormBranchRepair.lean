module
public import NormSignedHankel
public section
namespace NormBranchRepair
open Polynomial Matrix
variable {K : Type*} [Field K]

/-- A constant ratio of polynomial squares gives a constant ratio, without a root oracle. -/
theorem squaresProportional (P Q : K[X]) (c d : K) (hc : c≠0) (hq : Q≠0)
    (h : C c*P^2=C d*Q^2) : ∃ t : K, P=C t*Q := by
  have hq0 : Q.leadingCoeff≠0 := leadingCoeff_ne_zero.mpr hq
  have hl := congrArg Polynomial.leadingCoeff h
  simp only [leadingCoeff_mul,leadingCoeff_C,leadingCoeff_pow] at hl
  let t := P.leadingCoeff/Q.leadingCoeff
  have ht : c*t^2=d := by
    dsimp [t]
    field_simp
    linear_combination hl
  have he : P^2=(C t*Q)^2 := by
    apply mul_left_cancel₀ (show (C c : K[X])≠0 from C_ne_zero.mpr hc)
    calc
      C c*P^2=C d*Q^2 := h
      _=C c*(C t*Q)^2 := by rw [← ht]; simp only [map_mul,map_pow]; ring
  have hz : (P-C t*Q)*(P+C t*Q)=0 := by linear_combination he
  rcases mul_eq_zero.mp hz with hminus|hplus
  · exact ⟨t,sub_eq_zero.mp hminus⟩
  · refine ⟨-t,?_⟩
    simpa only [map_neg,neg_mul] using (eq_neg_iff_add_eq_zero.mpr hplus)

/-- Actual shared-node branch equations force the low-degree cross product to vanish. -/
theorem crossZero (S : Finset K) (z : K → K) (A0 B0 A1 B1 : K[X]) (a : ℕ)
    (hcard : S.card=2*a) (hA0 : A0.natDegree≤a) (hA1 : A1.natDegree≤a)
    (hB0 : B0.natDegree<a-1) (hB1 : B1.natDegree<a-1)
    (hp0 : ∀ x∈S, A0.eval x=z x*B0.eval x)
    (hp1 : ∀ x∈S, A1.eval x=z x*B1.eval x) : A0*B1-A1*B0=0 := by
  have hdiv : Lagrange.nodal S id ∣ A0*B1-A1*B0 := by
    apply NormSignedHankel.nodalDivides S
    intro x hx
    rw [eval_sub,eval_mul,eval_mul,hp0 x hx,hp1 x hx]
    ring
  by_contra hn
  have hhigh := natDegree_le_of_dvd hdiv hn
  rw [Lagrange.natDegree_nodal,hcard] at hhigh
  have hlow := natDegree_sub_le (A0*B1) (A1*B0)
  have h0 := natDegree_mul_le (p:=A0) (q:=B1)
  have h1 := natDegree_mul_le (p:=A1) (q:=B0)
  omega

/-- Exact norm identities and a vanishing cross product yield proportional squares. -/
theorem normSquareRelation (f G A0 B0 A1 B1 : K[X]) (c0 c1 : K)
    (hg : G≠0) (h0 : A0^2-f*B0^2=C c0*G)
    (h1 : A1^2-f*B1^2=C c1*G) (hcross : A0*B1-A1*B0=0) :
    C c0*B1^2=C c1*B0^2 := by
  have hraw : (A0^2-f*B0^2)*B1^2-(A1^2-f*B1^2)*B0^2=
      (A0*B1-A1*B0)*(A0*B1+A1*B0) := by ring
  rw [h0,h1,hcross,zero_mul] at hraw
  have hz : G*(C c0*B1^2-C c1*B0^2)=0 := by linear_combination hraw
  exact sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_left hg)

/-- Continuous coordinate repair stays on the same square exactly at the two branches. -/
theorem branchDeltaIff (z delta : K) :
    (z+delta)^2=z^2 ↔ delta=0 ∨ delta=-(2*z) := by
  constructor
  · intro h
    have hf : delta*(2*z+delta)=0 := by linear_combination h
    rcases mul_eq_zero.mp hf with h0|h1
    · exact Or.inl h0
    · right
      linear_combination h1
  · rintro (rfl|rfl) <;> ring

variable [DecidableEq K]
noncomputable def numerator (S : Finset K) (z : K → K) (B : K[X]) : K[X] :=
  ((Lagrange.interpolate S id z)*B) %ₘ Lagrange.nodal S id

/-- The actual constructed numerator has the specified public branches. -/
theorem numeratorAtNode (S : Finset K) (z : K → K) (B : K[X]) (x : K) (hx : x∈S) :
    (numerator S z B).eval x=z x*B.eval x := by
  rw [numerator,NormSignedHankel.remainderAtNode S _ x hx,eval_mul]
  have hv : ((Lagrange.interpolate S id) z).eval x=z x :=
    Lagrange.eval_interpolate_at_node z Function.injective_id.injOn hx
  rw [hv]

/-- Literal kernel equations for the actual ofFn polynomial. -/
theorem kernelEquations (S : Finset K) (z : K → K) (m : ℕ) (b : Fin m → K)
    (h : (NormSignedHankel.hankel S z m).mulVec b=0) :
    ∀ j<m, (∑ k∈Finset.range m,NormSignedHankel.moment S z (j+k)*(ofFn m b).coeff k)=0 := by
  intro j hj
  rw [← NormSignedHankel.matrixRow S z m b ⟨j,hj⟩]
  exact congrFun h ⟨j,hj⟩

/-- No nonzero vector is lost in the actual coefficient-polynomial conversion. -/
theorem ofFnNonzero (m : ℕ) (b : Fin m → K) (hb : b≠0) : ofFn m b≠0 := by
  intro hz
  apply hb
  apply Polynomial.injective_ofFn m
  simpa only [map_zero] using hz

/-- Actual kernel vectors reconstruct numerators in the pole window. -/
theorem kernelNumeratorBound (S : Finset K) (z : K → K) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (b : Fin (a-1) → K)
    (h : (NormSignedHankel.hankel S z (a-1)).mulVec b=0) :
    (numerator S z (ofFn (a-1) b)).natDegree≤a := by
  apply (NormSignedHankel.hankelDegreeIff S z (Lagrange.interpolate S id z) _ a
    ha hcard (fun x hx => Lagrange.eval_interpolate_at_node z Function.injective_id.injOn hx)
    (ofFn_natDegree_lt (by omega) b)).mpr
  exact kernelEquations S z (a-1) b h

/-- Every nonzero actual kernel vector has its own derived nonzero norm. -/
theorem kernelNorm (S : Finset K) (z : K → K) (f : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (b : Fin (a-1) → K) (hb : b≠0)
    (h : (NormSignedHankel.hankel S z (a-1)).mulVec b=0) :
    ∃ c : K, c≠0 ∧
      (numerator S z (ofFn (a-1) b))^2-f*(ofFn (a-1) b)^2=C c*Lagrange.nodal S id := by
  apply NormSignedHankel.publicHankelCertificate S z (Lagrange.interpolate S id z)
    f _ a ha hcard hf
    (fun x hx => Lagrange.eval_interpolate_at_node z Function.injective_id.injOn hx)
    hpoints (ofFn_natDegree_lt (by omega) b) (ofFnNonzero (a-1) b hb)
  exact kernelEquations S z (a-1) b h

/-- The actual public kernel is exactly the line through any nonzero kernel vector. -/
theorem kernelLine (S : Finset K) (z : K → K) (f : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (u : Fin (a-1) → K) (hu : u≠0)
    (hU : (NormSignedHankel.hankel S z (a-1)).mulVec u=0) (b : Fin (a-1) → K) :
    (NormSignedHankel.hankel S z (a-1)).mulVec b=0 ↔ ∃ t : K, b=t • u := by
  constructor
  · intro hb
    by_cases hb0 : b=0
    · exact ⟨0,by simp [hb0]⟩
    let B0 := ofFn (a-1) u
    let B1 := ofFn (a-1) b
    let A0 := numerator S z B0
    let A1 := numerator S z B1
    have hB0 := ofFnNonzero (a-1) u hu
    obtain ⟨c0,hc0,hn0⟩ := kernelNorm S z f a ha hcard hf hpoints u hu hU
    obtain ⟨c1,hc1,hn1⟩ := kernelNorm S z f a ha hcard hf hpoints b hb0 hb
    have hcross : A0*B1-A1*B0=0 := crossZero S z A0 B0 A1 B1 a hcard
      (kernelNumeratorBound S z a ha hcard u hU)
      (kernelNumeratorBound S z a ha hcard b hb)
      (ofFn_natDegree_lt (by omega) u) (ofFn_natDegree_lt (by omega) b)
      (fun x hx => numeratorAtNode S z B0 x hx)
      (fun x hx => numeratorAtNode S z B1 x hx)
    have hsq := normSquareRelation f (Lagrange.nodal S id) A0 B0 A1 B1 c0 c1
      Lagrange.nodal_ne_zero hn0 hn1 hcross
    obtain ⟨t,ht⟩ := squaresProportional B1 B0 c0 c1 hc0 hB0 hsq
    refine ⟨t,?_⟩
    apply Polynomial.injective_ofFn (a-1)
    rw [map_smul]
    simpa only [Polynomial.smul_eq_C_mul] using ht
  · rintro ⟨t,rfl⟩
    rw [Matrix.mulVec_smul,hU,smul_zero]

/-- Numerical corank one is derived for every singular actual public matrix. -/
theorem singularCorankOne (S : Finset K) (z : K → K) (f : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (hd : (NormSignedHankel.hankel S z (a-1)).det=0) :
    (NormSignedHankel.hankel S z (a-1)).rank+1=a-1 := by
  let M := NormSignedHankel.hankel S z (a-1)
  obtain ⟨u,hu,hU⟩ := Matrix.exists_mulVec_eq_zero_iff.mpr hd
  have hker : LinearMap.ker M.mulVecLin=K ∙ u := by
    ext b
    simp only [LinearMap.mem_ker,Matrix.mulVecLin_apply]
    rw [kernelLine S z f a ha hcard hf hpoints u hu hU b,Submodule.mem_span_singleton]
    exact exists_congr (fun t => eq_comm)
  have hk : Module.finrank K (LinearMap.ker M.mulVecLin)=1 := by
    rw [hker,finrank_span_singleton hu]
  have hr := M.mulVecLin.finrank_range_add_finrank_ker
  rw [hk,Module.finrank_pi,Fintype.card_fin] at hr
  exact hr

#print axioms squaresProportional
#print axioms crossZero
#print axioms normSquareRelation
#print axioms branchDeltaIff
#print axioms numeratorAtNode
#print axioms kernelEquations
#print axioms ofFnNonzero
#print axioms kernelNumeratorBound
#print axioms kernelNorm
#print axioms kernelLine
#print axioms singularCorankOne
end NormBranchRepair
