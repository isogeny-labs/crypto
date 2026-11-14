module
public import NormPublicJets
public import Mathlib.LinearAlgebra.Lagrange
public import Mathlib.Algebra.Polynomial.OfFn
public import Mathlib.LinearAlgebra.Matrix.ToLinearEquiv
public section
namespace NormSignedHankel
open Polynomial
variable {K : Type*} [Field K] [DecidableEq K]

/-- A finite top-moment test characterizes an actual degree cutoff. -/
theorem momentDegreeIff (G R : K[X]) (a : ℕ) (hG : G.Monic)
    (hR : R.natDegree<G.natDegree) :
    R.natDegree≤a ↔ ∀ j<G.natDegree-a-1,
      ((R*X^j) %ₘ G).coeff (G.natDegree-1)=0 := by
  constructor
  · intro h j hj
    have hd : (R*X^j).natDegree<G.natDegree-1 :=
      natDegree_mul_le.trans_lt (by simp only [natDegree_X_pow]; omega)
    have he : (R*X^j) %ₘ G=R*X^j := by
      apply (modByMonic_eq_self_iff hG).mpr
      by_cases hz : R*X^j=0
      · simp [hz,degree_eq_natDegree hG.ne_zero]
      · rw [degree_eq_natDegree hz,degree_eq_natDegree hG.ne_zero]
        exact_mod_cast (show (R*X^j).natDegree<G.natDegree by omega)
    rw [he]
    exact coeff_eq_zero_of_natDegree_lt hd
  · intro h
    by_contra hlarge
    have hr0 : R≠0 := by intro hz; simp [hz] at hlarge
    let j := G.natDegree-1-R.natDegree
    have hj : j<G.natDegree-a-1 := by dsimp [j]; omega
    have hdegree : (R*X^j).natDegree=G.natDegree-1 := by
      rw [natDegree_mul_X_pow j hr0]
      dsimp [j]
      omega
    have hp0 : R*X^j≠0 := mul_ne_zero hr0 (pow_ne_zero _ X_ne_zero)
    have he : (R*X^j) %ₘ G=R*X^j := by
      apply (modByMonic_eq_self_iff hG).mpr
      rw [degree_eq_natDegree hp0,degree_eq_natDegree hG.ne_zero,hdegree]
      exact_mod_cast (show G.natDegree-1<G.natDegree by omega)
    have hz := h j hj
    rw [he,← hdegree,coeff_natDegree] at hz
    exact (leadingCoeff_ne_zero.mpr hp0) hz

omit [DecidableEq K] in
/-- Remainder evaluation at actual support nodes, without an interpolation oracle. -/
theorem remainderAtNode (S : Finset K) (P : K[X]) (x : K) (hx : x∈S) :
    (P %ₘ Lagrange.nodal S id).eval x=P.eval x := by
  simpa only [eval₂_id] using
    (eval₂_modByMonic_eq_self_of_root (p:=P) (f:=RingHom.id K)
      (show (Lagrange.nodal S id).eval₂ (RingHom.id K) x=0 from
        by simpa only [eval₂_id,id_eq] using (Lagrange.eval_nodal_at_node (v:=id) hx)))

/-- Exact barycentric formula for the top coefficient of the actual remainder. -/
theorem barycentricMoment (S : Finset K) (P : K[X]) :
    (P %ₘ Lagrange.nodal S id).coeff (S.card-1)=
      ∑ x∈S, P.eval x / ∏ t∈S.erase x, (x-t) := by
  have hd : (P %ₘ Lagrange.nodal S id).degree<(S.card : WithBot ℕ) := by
    simpa only [Lagrange.degree_nodal] using
      degree_modByMonic_lt P (Lagrange.nodal_monic (s:=S) (v:=id))
  rw [Lagrange.coeff_eq_sum Function.injective_id.injOn hd]
  apply Finset.sum_congr rfl
  intro x hx
  simp only [id_eq]
  rw [remainderAtNode S P x hx]

omit [DecidableEq K] in
/-- The actual product divides a polynomial vanishing at all distinct support nodes. -/
theorem nodalDivides (S : Finset K) (P : K[X]) (hp : ∀ x∈S, P.eval x=0) :
    Lagrange.nodal S id ∣ P := by
  have hz : P %ₘ Lagrange.nodal S id=0 := by
    apply Polynomial.eq_zero_of_degree_lt_of_eval_finset_eq_zero S
    · simpa only [Lagrange.degree_nodal] using
        degree_modByMonic_lt P (Lagrange.nodal_monic (s:=S) (v:=id))
    · intro x hx
      rw [remainderAtNode S P x hx]
      exact hp x hx
  exact (modByMonic_eq_zero_iff_dvd (Lagrange.nodal_monic (s:=S) (v:=id))).mp hz

noncomputable def moment (S : Finset K) (z : K → K) (j : ℕ) : K :=
  ∑ x∈S, z x*x^j / ∏ t∈S.erase x, (x-t)

/-- Literal Hankel multiplication equals the public branch-weighted evaluation. -/
theorem rowExpansion (S : Finset K) (z : K → K) (B : K[X]) (m j : ℕ)
    (hb : B.natDegree<m) :
    (∑ k∈Finset.range m, moment S z (j+k)*B.coeff k)=
      ∑ x∈S, z x*B.eval x*x^j / ∏ t∈S.erase x, (x-t) := by
  simp only [moment,Finset.sum_mul]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro x hx
  rw [Polynomial.eval_eq_sum_range' hb]
  simp only [Finset.mul_sum,Finset.sum_mul,Finset.sum_div]
  apply Finset.sum_congr rfl
  intro k hk
  rw [pow_add]
  ring

/-- Actual public signed Hankel equations are exactly the bounded reconstruction test. -/
theorem hankelDegreeIff (S : Finset K) (z : K → K) (s B : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a)
    (hs : ∀ x∈S, s.eval x=z x) (hb : B.natDegree<a-1) :
    ((s*B) %ₘ Lagrange.nodal S id).natDegree≤a ↔
      ∀ j<a-1, (∑ k∈Finset.range (a-1),moment S z (j+k)*B.coeff k)=0 := by
  let G := Lagrange.nodal S id
  let R := (s*B) %ₘ G
  have hG : G.Monic := Lagrange.nodal_monic
  have hdeg : G.natDegree=2*a := by simp [G,hcard]
  have hne : G≠1 := by intro h; rw [h,natDegree_one] at hdeg; omega
  have hr : R.natDegree<G.natDegree := natDegree_modByMonic_lt _ hG hne
  have heq (j : ℕ) : ((R*X^j) %ₘ G).coeff (G.natDegree-1)=
      ∑ k∈Finset.range (a-1), moment S z (j+k)*B.coeff k := by
    rw [show G.natDegree=S.card by simp [G], barycentricMoment]
    rw [rowExpansion S z B (a-1) j hb]
    apply Finset.sum_congr rfl
    intro x hx
    have heval : R.eval x=z x*B.eval x := by
      dsimp [R,G]
      rw [remainderAtNode S (s*B) x hx,eval_mul,hs x hx]
    rw [eval_mul,eval_pow,eval_X,heval]
  have hd := momentDegreeIff G R a hG hr
  have hcount : G.natDegree-a-1=a-1 := by rw [hdeg]; omega
  simpa only [hcount,heq] using hd

omit [DecidableEq K] in
/-- An odd cubic cannot be a rational polynomial square: the norm is nonzero. -/
theorem cubicNormNonzero (f A B : K[X]) (hf : f.natDegree=3) (hb : B≠0) :
    A^2-f*B^2≠0 := by
  intro hz
  have hf0 : f≠0 := by intro h; simp [h] at hf
  have hbp : B^2≠0 := pow_ne_zero _ hb
  have he := sub_eq_zero.mp hz
  have ha : A≠0 := by
    intro h
    rw [h,zero_pow (by decide : 2≠0)] at he
    exact (mul_ne_zero hf0 hbp) he.symm
  have hd := congrArg Polynomial.natDegree he
  rw [natDegree_pow,natDegree_mul hf0 hbp,natDegree_pow,hf] at hd
  omega

/-- A nonzero kernel polynomial in the actual public Hankel family gives a full norm.
Branch selection and finding that kernel remain algorithmic tasks. -/
theorem publicHankelCertificate (S : Finset K) (z : K → K) (s f B : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hs : ∀ x∈S, s.eval x=z x) (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (hb : B.natDegree<a-1) (hb0 : B≠0)
    (hker : ∀ j<a-1, (∑ k∈Finset.range (a-1),moment S z (j+k)*B.coeff k)=0) :
    ∃ c : K, c≠0 ∧
      (((s*B) %ₘ Lagrange.nodal S id)^2-f*B^2)=C c*Lagrange.nodal S id := by
  let G := Lagrange.nodal S id
  let A := (s*B) %ₘ G
  have hA : A.natDegree≤a := (hankelDegreeIff S z s B a ha hcard hs hb).mpr hker
  apply NormPublicJets.publicResidueCertificate f G s A B Lagrange.nodal_monic
  · apply nodalDivides S
    intro x hx
    simp only [eval_sub,eval_pow,hs x hx,hpoints x hx,sub_self]
  · apply nodalDivides S
    intro x hx
    dsimp [A,G]
    rw [eval_sub,remainderAtNode S (s*B) x hx,sub_self]
  · have hd : G.natDegree=2*a := by simp [G,hcard]
    omega
  · have hd : G.natDegree=2*a := by simp [G,hcard]
    omega
  · exact cubicNormNonzero f A B hf hb0

/-- Changing one public branch adds one explicit outer product to every Hankel entry. -/
theorem branchUpdate (S : Finset K) (z : K → K) (t delta : K) (ht : t∈S)
    (j k : ℕ) :
    moment S (Function.update z t (z t+delta)) (j+k)-moment S z (j+k)=
      (delta / ∏ x∈S.erase t, (t-x))*t^j*t^k := by
  simp only [moment]
  rw [← Finset.sum_sub_distrib,Finset.sum_eq_single t]
  · simp only [Function.update_self]
    rw [pow_add]
    ring
  · intro x hx hxt
    rw [Function.update_of_ne hxt]
    simp
  · exact fun h => (h ht).elim

noncomputable def hankel (S : Finset K) (z : K → K) (m : ℕ) :
    Matrix (Fin m) (Fin m) K := fun j k => moment S z (j.val+k.val)

/-- The actual signed public matrix is symmetric. -/
theorem hankelSymmetric (S : Finset K) (z : K → K) (m : ℕ) :
    (hankel S z m).IsSymm := by
  ext j k
  change moment S z (k.val+j.val)=moment S z (j.val+k.val)
  rw [Nat.add_comm]

/-- The branch update is an explicit rank-one-form matrix correction. -/
theorem matrixBranchUpdate (S : Finset K) (z : K → K) (m : ℕ) (t delta : K)
    (ht : t∈S) :
    hankel S (Function.update z t (z t+delta)) m-hankel S z m=
      (fun j k => (delta / ∏ x∈S.erase t, (t-x))*t^j.val*t^k.val) := by
  ext j k
  exact branchUpdate S z t delta ht j.val k.val

/-- Literal finite matrix rows coincide with coefficient equations for ofFn. -/
theorem matrixRow (S : Finset K) (z : K → K) (m : ℕ) (b : Fin m → K) (j : Fin m) :
    (hankel S z m).mulVec b j=
      ∑ k∈Finset.range m, moment S z (j.val+k)*(ofFn m b).coeff k := by
  change (∑ k : Fin m, moment S z (j.val+k.val)*b k)=_
  rw [Finset.sum_range]
  apply Finset.sum_congr rfl
  intro k hk
  rw [ofFn_coeff_eq_val_of_lt b k.isLt]

/-- Singularity of the actual public Hankel matrix supplies the bounded norm pair. -/
theorem singularHankelCertificate (S : Finset K) (z : K → K) (s f : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hs : ∀ x∈S, s.eval x=z x) (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (hd : (hankel S z (a-1)).det=0) :
    ∃ B : K[X], B≠0 ∧ B.natDegree<a-1 ∧ ∃ c : K, c≠0 ∧
      (((s*B) %ₘ Lagrange.nodal S id)^2-f*B^2)=C c*Lagrange.nodal S id := by
  obtain ⟨b,hb,hkernel⟩ := Matrix.exists_mulVec_eq_zero_iff.mpr hd
  have hb0 : ofFn (a-1) b≠0 := by
    intro hz
    apply hb
    apply Polynomial.injective_ofFn (a-1)
    simpa only [map_zero] using hz
  have hdeg : (ofFn (a-1) b).natDegree<a-1 := ofFn_natDegree_lt (by omega) b
  refine ⟨ofFn (a-1) b,hb0,hdeg,?_⟩
  apply publicHankelCertificate S z s f _ a ha hcard hf hs hpoints hdeg hb0
  intro j hj
  rw [← matrixRow S z (a-1) b ⟨j,hj⟩]
  exact congrFun hkernel ⟨j,hj⟩

/-- Fully public chart construction: the interpolant itself is derived from the nodes. -/
theorem publicInputCertificate (S : Finset K) (z : K → K) (f : K[X]) (a : ℕ)
    (ha : 2≤a) (hcard : S.card=2*a) (hf : f.natDegree=3)
    (hpoints : ∀ x∈S, (z x)^2=f.eval x)
    (hd : (hankel S z (a-1)).det=0) :
    ∃ B : K[X], B≠0 ∧ B.natDegree<a-1 ∧ ∃ c : K, c≠0 ∧
      ((((Lagrange.interpolate S id z)*B) %ₘ Lagrange.nodal S id)^2-f*B^2)=
        C c*Lagrange.nodal S id := by
  apply singularHankelCertificate S z (Lagrange.interpolate S id z) f a ha hcard hf
  · intro x hx
    exact Lagrange.eval_interpolate_at_node z Function.injective_id.injOn hx
  · exact hpoints
  · exact hd

#print axioms momentDegreeIff
#print axioms remainderAtNode
#print axioms barycentricMoment
#print axioms nodalDivides
#print axioms rowExpansion
#print axioms hankelDegreeIff
#print axioms cubicNormNonzero
#print axioms publicHankelCertificate
#print axioms branchUpdate
#print axioms matrixRow
#print axioms singularHankelCertificate
#print axioms hankelSymmetric
#print axioms matrixBranchUpdate
#print axioms publicInputCertificate
end NormSignedHankel
