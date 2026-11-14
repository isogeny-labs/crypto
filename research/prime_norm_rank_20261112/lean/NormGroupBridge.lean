module
public import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
public import Mathlib.LinearAlgebra.FiniteDimensional.Lemmas
public import Mathlib.LinearAlgebra.DirectSum.Finite
public import Mathlib.RingTheory.Ideal.Quotient.Operations
public import NormSignedHankel

public section
open Polynomial WeierstrassCurve.Affine
open scoped nonZeroDivisors Polynomial.Bivariate
namespace NormGroupBridge
variable {K : Type*} [Field K] (W : WeierstrassCurve.Affine K)

/-- Nonzero principal quotients of the actual coordinate ring are finite dimensional. -/
theorem quotientFinite {h : W.CoordinateRing} (hh : h ≠ 0) :
    FiniteDimensional K (W.CoordinateRing ⧸ Ideal.span {h}) := by
  classical
  let I : Ideal W.CoordinateRing := Ideal.span {h}
  have hI : I ≠ ⊥ := Ideal.span_singleton_eq_bot.not.mpr hh
  let e := Ideal.quotientEquivDirectSum K (CoordinateRing.basis W) hI
  exact e.symm.finiteDimensional

/-- A quotient inclusion between equal finite dimensions is equality of actual ideals. -/
theorem idealsEqualOfDimension {I J : Ideal W.CoordinateRing} (hle : I ≤ J)
    [FiniteDimensional K (W.CoordinateRing ⧸ I)]
    [FiniteDimensional K (W.CoordinateRing ⧸ J)]
    (hd : Module.finrank K (W.CoordinateRing ⧸ I) =
      Module.finrank K (W.CoordinateRing ⧸ J)) : I = J := by
  have hi : Function.Injective (Ideal.Quotient.factorₐ K hle) :=
    (LinearMap.injective_iff_surjective_of_finrank_eq_finrank
      (f := (Ideal.Quotient.factorₐ K hle).toLinearMap) hd).mpr
      (Ideal.Quotient.factor_surjective hle)
  apply le_antisymm hle
  intro x hx
  apply Ideal.Quotient.eq_zero_iff_mem.mp
  apply hi
  simpa using (Ideal.Quotient.eq_zero_iff_mem.mpr hx)

/-- Distinct x coordinates give coprime actual point ideals. -/
theorem pointIdealsCoprime (x₁ x₂ y₁ y₂ : K) (hx : x₁ ≠ x₂) :
    IsCoprime (CoordinateRing.XYIdeal W x₁ (C y₁))
      (CoordinateRing.XYIdeal W x₂ (C y₂)) := by
  apply Ideal.isCoprime_iff_exists.mpr
  let c := algebraMap K W.CoordinateRing (x₂-x₁)⁻¹
  refine ⟨c*CoordinateRing.XClass W x₁, ?_,
    -c*CoordinateRing.XClass W x₂, ?_, ?_⟩
  · apply Ideal.mul_mem_left
    exact Ideal.subset_span (by simp)
  · apply Ideal.mul_mem_left
    exact Ideal.subset_span (by simp)
  · have hd : CoordinateRing.XClass W x₁-CoordinateRing.XClass W x₂ =
        algebraMap K W.CoordinateRing (x₂-x₁) := by
      change CoordinateRing.mk W (C (X-C x₁))-
          CoordinateRing.mk W (C (X-C x₂)) =
        CoordinateRing.mk W (C (C (x₂-x₁)))
      rw [← map_sub, ← C_sub]
      congr 2
      rw [map_sub]
      ring
    calc c*CoordinateRing.XClass W x₁+-c*CoordinateRing.XClass W x₂ =
        c*(CoordinateRing.XClass W x₁-CoordinateRing.XClass W x₂) := by ring
      _ = c*algebraMap K W.CoordinateRing (x₂-x₁) := by rw [hd]
      _ = 1 := by
        dsimp [c]
        rw [← map_mul, inv_mul_cancel₀ (sub_ne_zero.mpr hx.symm), map_one]

variable {ι : Type*} [Fintype ι]

/-- CRT for the actual point ideals, followed by their actual residue-field equivalences. -/
noncomputable def pointQuotientEquiv (x y : ι → K)
    (hp : ∀ i, W.Nonsingular (x i) (y i)) (hx : Function.Injective x) :
    (W.CoordinateRing ⧸ ⨅ i, CoordinateRing.XYIdeal W (x i) (C (y i))) ≃ₗ[K]
      (ι → K) := by
  let I := fun i => CoordinateRing.XYIdeal W (x i) (C (y i))
  have hc : Pairwise (Function.onFun IsCoprime I) := by
    intro i j hij
    exact pointIdealsCoprime W _ _ _ _ (fun he => hij (hx he))
  let e : (W.CoordinateRing ⧸ ⨅ i, I i) ≃ₐ[K]
      ((i : ι) → W.CoordinateRing ⧸ I i) :=
    { Ideal.quotientInfRingEquivPiQuotient I hc with
      commutes' := by intro r; rfl }
  exact e.toLinearEquiv.trans (LinearEquiv.piCongrRight fun i =>
    (CoordinateRing.quotientXYIdealEquiv (W' := W) (hp i).left).toLinearEquiv)

/-- The actual simultaneous point quotient has dimension equal to the number of nodes. -/
theorem pointQuotientDimension (x y : ι → K)
    (hp : ∀ i, W.Nonsingular (x i) (y i)) (hx : Function.Injective x) :
    Module.finrank K (W.CoordinateRing ⧸
      ⨅ i, CoordinateRing.XYIdeal W (x i) (C (y i))) = Fintype.card ι := by
  rw [(pointQuotientEquiv W x y hp hx).finrank_eq, Module.finrank_pi]

/-- Degree exhaustion identifies the product of actual point ideals with the principal ideal. -/
theorem principalPointProduct (x y : ι → K)
    (hp : ∀ i, W.Nonsingular (x i) (y i)) (hx : Function.Injective x)
    (h : W.CoordinateRing) (hh : h ≠ 0)
    (hm : ∀ i, h ∈ CoordinateRing.XYIdeal W (x i) (C (y i)))
    (hd : (Algebra.norm K[X] h).natDegree=Fintype.card ι) :
    (∏ i, CoordinateRing.XYIdeal W (x i) (C (y i)))=Ideal.span {h} := by
  classical
  let I := fun i => CoordinateRing.XYIdeal W (x i) (C (y i))
  have hc : Pairwise (Function.onFun IsCoprime I) := by
    intro i j hij
    exact pointIdealsCoprime W _ _ _ _ (fun he => hij (hx he))
  have heq : (∏ i, I i)=⨅ i, I i := by
    simpa using (Ideal.prod_eq_iInf_of_pairwise_isCoprime
      (s := Finset.univ) (J := I) (by intro i _ j _ hij; exact hc hij))
  have hle : Ideal.span {h} ≤ ⨅ i, I i := by
    apply le_iInf
    intro i
    exact (Ideal.span_singleton_le_iff_mem _).mpr (hm i)
  let : FiniteDimensional K (W.CoordinateRing ⧸ ⨅ i, I i) :=
    (pointQuotientEquiv W x y hp hx).symm.finiteDimensional
  let : FiniteDimensional K (W.CoordinateRing ⧸ Ideal.span {h}) := quotientFinite W hh
  have hdim : Module.finrank K (W.CoordinateRing ⧸ Ideal.span {h}) =
      Module.finrank K (W.CoordinateRing ⧸ ⨅ i, I i) := by
    rw [finrank_quotient_span_eq_natDegree_norm (CoordinateRing.basis W) hh,
      hd, pointQuotientDimension W x y hp hx]
  exact heq.trans (idealsEqualOfDimension W hle hdim).symm

/-- The actual cubic in the Weierstrass equation. -/
noncomputable def cubic : K[X] := X^3+C W.a₂*X^2+C W.a₄*X+C W.a₆

/-- The actual coordinate-ring element A-BY. -/
noncomputable def normFunction (A B : K[X]) : W.CoordinateRing :=
  A • (1 : W.CoordinateRing)+(-B) • CoordinateRing.mk W Y

/-- On the stated short-form coefficients, the polynomial certificate is the actual algebra norm. -/
theorem normFunctionNorm (h₁ : W.a₁=0) (h₃ : W.a₃=0) (A B : K[X]) :
    Algebra.norm K[X] (normFunction W A B)=A^2-cubic W*B^2 := by
  rw [normFunction,CoordinateRing.norm_smul_basis]
  simp only [h₁,h₃,C_0,zero_mul,add_zero,mul_zero,sub_zero,neg_sq,cubic]
  ring

/-- Actual vanishing puts A-BY in the actual point ideal, by polynomial division. -/
theorem normFunctionAtPoint (A B : K[X]) (x y : K) (hr : A.eval x=y*B.eval x) :
    normFunction W A B ∈ CoordinateRing.XYIdeal W x (C y) := by
  let D := A-C y*B
  have hd : X-C x ∣ D := by
    apply (dvd_iff_isRoot).mpr
    simp [D,IsRoot.def,hr]
  obtain ⟨T,hT⟩ := hd
  have hx : CoordinateRing.mk W (C D) ∈ CoordinateRing.XYIdeal W x (C y) := by
    rw [hT,map_mul,map_mul]
    change CoordinateRing.XClass W x * _ ∈ _
    exact Ideal.mul_mem_right _ _ (Ideal.subset_span (by simp))
  have hy : CoordinateRing.YClass W (C y) ∈ CoordinateRing.XYIdeal W x (C y) :=
    Ideal.subset_span (by simp)
  have he : normFunction W A B=CoordinateRing.mk W (C D)-
      CoordinateRing.mk W (C B)*CoordinateRing.YClass W (C y) := by
    simp only [normFunction,CoordinateRing.smul,CoordinateRing.YClass,D,
      map_sub,map_neg,map_mul,mul_one]
    ring
  rw [he]
  exact Ideal.sub_mem _ hx (Ideal.mul_mem_left _ _ hy)

variable [DecidableEq K]

/-- A principal product of actual point ideals gives an actual elliptic group relation. -/
theorem pointSumOfPrincipal (x y : ι → K)
    (hp : ∀ i, W.Nonsingular (x i) (y i))
    (h : W.CoordinateRing) (hh : h ≠ 0)
    (he : (∏ i, CoordinateRing.XYIdeal W (x i) (C (y i)))=Ideal.span {h}) :
    (∑ i, Point.some (x i) (y i) (hp i))=0 := by
  classical
  let u := fun i => CoordinateRing.XYIdeal' (W := W) (hp i)
  have hu : (((∏ i, u i) : (FractionalIdeal W.CoordinateRing⁰ W.FunctionField)ˣ) :
      FractionalIdeal W.CoordinateRing⁰ W.FunctionField)=
      ((∏ i, CoordinateRing.XYIdeal W (x i) (C (y i))) : Ideal W.CoordinateRing) := by
    rw [show (((∏ i, CoordinateRing.XYIdeal W (x i) (C (y i))) : Ideal W.CoordinateRing) :
      FractionalIdeal W.CoordinateRing⁰ W.FunctionField)=
      ∏ i, (CoordinateRing.XYIdeal W (x i) (C (y i)) :
        FractionalIdeal W.CoordinateRing⁰ W.FunctionField) from
      map_prod (FractionalIdeal.coeIdealHom W.CoordinateRing⁰ W.FunctionField) _ _]
    simp only [Units.coe_prod, CoordinateRing.XYIdeal'_eq, u]
  have hc : (∏ i, ClassGroup.mk W.FunctionField (u i))=1 := by
    rw [← map_prod]
    exact (ClassGroup.mk_eq_one_of_coe_ideal hu).mpr ⟨h,hh,he⟩
  apply Point.toClass_injective
  rw [map_sum, map_zero]
  change Additive.ofMul (∏ i, ClassGroup.mk W.FunctionField (u i))=0
  rw [hc]
  rfl

/-- A full-degree norm through distinct actual points forces their sum to be zero. -/
theorem normDegreePointSum (x y : ι → K)
    (hp : ∀ i, W.Nonsingular (x i) (y i)) (hx : Function.Injective x)
    (h : W.CoordinateRing) (hh : h ≠ 0)
    (hm : ∀ i, h ∈ CoordinateRing.XYIdeal W (x i) (C (y i)))
    (hd : (Algebra.norm K[X] h).natDegree=Fintype.card ι) :
    (∑ i, Point.some (x i) (y i) (hp i))=0 :=
  pointSumOfPrincipal W x y hp h hh (principalPointProduct W x y hp hx h hh hm hd)

/-- The full polynomial norm certificate implies the actual elliptic point relation. -/
theorem fullNormPointSum (h₁ : W.a₁=0) (h₃ : W.a₃=0)
    (S : Finset K) (z : K → K) (A B : K[X]) (c : K) (hc : c≠0)
    (hp : ∀ x∈S, W.Nonsingular x (z x))
    (hr : ∀ x∈S, A.eval x=z x*B.eval x)
    (hn : A^2-cubic W*B^2=C c*Lagrange.nodal S id) :
    (∑ x : S, Point.some (x : K) (z x) (hp x x.property))=0 := by
  have he : Algebra.norm K[X] (normFunction W A B)=C c*Lagrange.nodal S id :=
    (normFunctionNorm W h₁ h₃ A B).trans hn
  have hne : normFunction W A B≠0 := by
    intro hz
    have hzero : Algebra.norm K[X] (0 : W.CoordinateRing)=0 := by
      simpa using (CoordinateRing.norm_smul_basis (W' := W) (0 : K[X]) 0)
    rw [hz,hzero] at he
    exact (mul_ne_zero (C_ne_zero.mpr hc) Lagrange.nodal_monic.ne_zero) he.symm
  apply normDegreePointSum W (fun x : S => (x : K)) (fun x : S => z x)
    (fun x => hp x x.property) Subtype.coe_injective (normFunction W A B) hne
  · intro x
    exact normFunctionAtPoint W A B x (z x) (hr x x.property)
  · rw [he,natDegree_mul (C_ne_zero.mpr hc) Lagrange.nodal_monic.ne_zero,
      natDegree_C,zero_add,Lagrange.natDegree_nodal]
    simp

omit [DecidableEq K] in
/-- The actual cubic has degree3, independent of its lower coefficients. -/
theorem cubicDegree : (cubic W).natDegree=3 := by
  unfold cubic
  compute_degree!

/-- Singularity of the actual public signed matrix yields an actual elliptic relation. -/
theorem publicHankelPointSum (h₁ : W.a₁=0) (h₃ : W.a₃=0)
    (S : Finset K) (z : K → K) (a : ℕ) (ha : 2≤a) (hcard : S.card=2*a)
    (hp : ∀ x∈S, W.Nonsingular x (z x))
    (hd : (NormSignedHankel.hankel S z (a-1)).det=0) :
    (∑ x : S, Point.some (x : K) (z x) (hp x x.property))=0 := by
  have hpoints : ∀ x∈S, (z x)^2=(cubic W).eval x := by
    intro x hx
    have he := (equation_iff ..).mp (hp x hx).left
    simpa [cubic,h₁,h₃] using he
  obtain ⟨B,_hb,_hdeg,c,hc,hn⟩ := NormSignedHankel.publicInputCertificate S z
    (cubic W) a ha hcard (cubicDegree W) hpoints hd
  apply fullNormPointSum W h₁ h₃ S z _ B c hc hp _ hn
  intro x hx
  rw [NormSignedHankel.remainderAtNode S _ x hx,eval_mul]
  have heval : (Lagrange.interpolate S id z).eval x=z x := by
    simpa only [id_eq] using
      (Lagrange.eval_interpolate_at_node z Function.injective_id.injOn hx)
  rw [heval]

/-- With all other scalar labels given, a zero point sum explicitly recovers the target label. -/
theorem targetScalarFromSum [DecidableEq ι] (P : W.Point) (points : ι → W.Point) (t : ι)
    (m : ι → ℤ) (hlabels : ∀ i, i≠t → points i=m i • P)
    (hsum : ∑ i, points i=0) :
    points t=(-∑ i∈Finset.univ.erase t, m i) • P := by
  classical
  have he : (∑ i∈Finset.univ.erase t, m i) • P+points t=0 := by
    rw [Finset.sum_smul]
    convert hsum using 1
    rw [← Finset.sum_erase_add _ _ (Finset.mem_univ t)]
    congr 1
    apply Finset.sum_congr rfl
    intro i hi
    exact (hlabels i (Finset.ne_of_mem_erase hi)).symm
  rw [neg_smul]
  exact eq_neg_of_add_eq_zero_right he

/-- Public matrix singularity plus the other supplied scalar labels gives the target scalar. -/
theorem publicHankelTargetScalar (h₁ : W.a₁=0) (h₃ : W.a₃=0)
    (S : Finset K) (z : K → K) (a : ℕ) (ha : 2≤a) (hcard : S.card=2*a)
    (hp : ∀ x∈S, W.Nonsingular x (z x))
    (hd : (NormSignedHankel.hankel S z (a-1)).det=0)
    (P : W.Point) (t : S) (m : S → ℤ)
    (hlabels : ∀ i : S, i≠t → Point.some (i : K) (z i) (hp i i.property)=m i • P) :
    Point.some (t : K) (z t) (hp t t.property)=
      (-∑ i∈Finset.univ.erase t,m i) • P :=
  targetScalarFromSum W P _ t m hlabels (publicHankelPointSum W h₁ h₃ S z a ha hcard hp hd)

#print axioms quotientFinite
#print axioms idealsEqualOfDimension
#print axioms pointIdealsCoprime
#print axioms pointQuotientDimension
#print axioms principalPointProduct
#print axioms pointSumOfPrincipal
#print axioms normDegreePointSum
#print axioms normFunctionNorm
#print axioms normFunctionAtPoint
#print axioms fullNormPointSum
#print axioms cubicDegree
#print axioms publicHankelPointSum
#print axioms targetScalarFromSum
#print axioms publicHankelTargetScalar
end NormGroupBridge
