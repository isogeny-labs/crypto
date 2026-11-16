import ReciprocityFiber

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine Function
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open P1TameReciprocityAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- Restrict an actual full finite fiber to any finite set covering the multiplicative support. -/
theorem product_fiber_restriction {A B M : Type*} [CommMonoid M] [DecidableEq A] [DecidableEq B]
    (π : A→B) (F T : Finset A) (v : B) (G : A→M)
    (hF : ∀ P,P∈F ↔ π P=v) (hT : mulSupport G ⊆ (T : Set A)) :
    (∏ P ∈ F,G P)=∏ P ∈ T with π P=v,G P := by
  apply (Finset.prod_subset ?_ ?_).symm
  · intro P hP
    exact (hF P).mpr (Finset.mem_filter.mp hP).2
  · intro P hP hn
    by_contra he
    exact hn (Finset.mem_filter.mpr ⟨hT he,(hF P).mp hP⟩)

def fiberTameProduct (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) (v : Option K) : Kˣ := by
  classical
  exact ∏ P ∈ T with pointBasePlace W P=v,tameSymbol W P f (baseFieldUnit W c)

def basePlaceTame (a b : (RatFunc K)ˣ) : Option K→Kˣ
  | none=>p1InfinityTame a b
  | some x=>p1FiniteTame x a b

/-- Actual fiber product equals the actual normalized norm symbol, including infinity. -/
theorem actual_norm_fiber_product [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (T : Finset W.Point) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ)
    (hT : mulSupport (fun P=>tameSymbol W P f (baseFieldUnit W c)) ⊆ (T : Set W.Point)) (v : Option K) :
    fiberTameProduct W T f c v=basePlaceTame (normUnit W f) c v := by
  classical
  cases v with
  | none=>
    have hF (P : W.Point) : P∈({0} : Finset W.Point) ↔ pointBasePlace W P=none := by
      rw [Finset.mem_singleton,point_base_place_none]
    rw [fiberTameProduct,←product_fiber_restriction (pointBasePlace W) {0} T none
      (fun P=>tameSymbol W P f (baseFieldUnit W c)) hF hT,Finset.prod_singleton]
    change tameSymbol W 0 f (baseFieldUnit W c)=p1InfinityTame (normUnit W f) c
    rw [p1_infinity_tame_realized W,actual_infinity_norm_projection]
  | some a=>
    obtain ⟨b,h⟩:=actual_nonsingular_point_above W hΔ a
    rw [fiberTameProduct,←product_fiber_restriction (pointBasePlace W)
      (finiteFiber W (Point.some a b h)) T (some a)
      (fun P=>tameSymbol W P f (baseFieldUnit W c)) (finite_fiber_place W h) hT]
    change (∏ P ∈ finiteFiber W (Point.some a b h),tameSymbol W P f (baseFieldUnit W c))=
      p1FiniteTame a (normUnit W f) c
    rw [p1_finite_tame_realized W h,actual_finite_place_norm_projection]

/-- The full global norm product identity is derived by actual finite-support regrouping. -/
theorem actual_global_mixed_norm_identity [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    globalTameProduct W f (baseFieldUnit W c)=p1GlobalProduct (normUnit W f) c := by
  classical
  let G : W.Point→Kˣ:=fun P=>tameSymbol W P f (baseFieldUnit W c)
  have hs : (mulSupport G).Finite:=tame_support_finite W f (baseFieldUnit W c)
  let T : Finset W.Point:=hs.toFinset
  have hT : mulSupport G ⊆ (T : Set W.Point) := by
    intro P hP
    change P∈hs.toFinset
    exact (Set.Finite.mem_toFinset hs).mpr hP
  calc
    globalTameProduct W f (baseFieldUnit W c) = ∏ P ∈ T,G P :=
      finprod_eq_prod_of_mulSupport_subset _ hT
    _ = ∏ᶠ v,fiberTameProduct W T f c v := (finprod_prod_filter (pointBasePlace W) T G).symm
    _ = ∏ᶠ v,basePlaceTame (normUnit W f) c v :=
      finprod_congr (actual_norm_fiber_product W hΔ T f c hT)
    _ = p1InfinityTame (normUnit W f) c*(∏ᶠ a,p1FiniteTame a (normUnit W f) c) :=
      finprod_option (p1_tame_support_finite (normUnit W f) c)
    _ = p1GlobalProduct (normUnit W f) c := mul_comm _ _

theorem actual_mixed_reciprocity [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    globalTameProduct W f (baseFieldUnit W c)=1 := by
  rw [actual_global_mixed_norm_identity W hΔ,p1_reciprocity]

#print axioms product_fiber_restriction
#print axioms actual_norm_fiber_product
#print axioms actual_global_mixed_norm_identity
#print axioms actual_mixed_reciprocity
end
end EllipticGlobalReciprocityAudit
