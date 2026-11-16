import ReciprocityCertificate
import ChoiceImports

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit WeilAuxiliaryRatioAudit EllipticTameSymbolAudit
variable {K : Type*} [Field K]

/-- Auxiliary extension by1; it is used as an evaluation only after a proper-domain proof. -/
def valueHelper (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) (P : W.Point) : Kˣ := by
  classical
  exact if h : pointOrder W f P=0 then localUnitValue W P f h else 1

def exponentEvaluation (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    (W.Point →₀ ℤ) →+ Additive Kˣ where
  toFun D := Additive.ofMul (D.prod fun P n=>valueHelper W f P^n)
  map_zero' := by simp
  map_add' D E := by
    change (D+E).prod (fun P n=>valueHelper W f P^n)=
      D.prod (fun P n=>valueHelper W f P^n)*E.prod (fun P n=>valueHelper W f P^n)
    exact Finsupp.prod_add_index' (fun _=>zpow_zero _) (fun _ _ _=>zpow_add _ _ _)

theorem proper_evaluation_helper (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W D f hd=(exponentEvaluation W f D).toMul := by
  classical
  rw [evaluateDivisor,divisorEvaluation,divisor_evaluation_on_apply]
  apply Finsupp.prod_congr
  intro P hP
  have h0 := (mem_support_units_iff W D.support f).mp
    ((disjoint_principal_support_iff W f D).mp hd) P hP
  simp only [
    valueHelper,dite_eq_left h0,supportPointEvaluation,dite_eq_left hP,MonoidHom.comp_apply]
  rfl

theorem proper_evaluation_add (W : WeierstrassCurve.Affine K) (D E : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ)
    (hD : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hE : Disjoint E.support (functionDivisor W f f.ne_zero).support)
    (hDE : Disjoint (D+E).support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W (D+E) f hDE=evaluateDivisor W D f hD*evaluateDivisor W E f hE := by
  simp only [proper_evaluation_helper,map_add,toMul_add]

theorem proper_evaluation_zsmul (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (n : ℤ)
    (hD : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hn : Disjoint (n • D).support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W (n • D) f hn=evaluateDivisor W D f hD^n := by
  simp only [proper_evaluation_helper,map_zsmul,toMul_zsmul]

theorem unit_function_divisor_zpow (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (n : ℤ) :
    functionDivisor W (↑(f^n) : W.FunctionField) (f^n).ne_zero=n • functionDivisor W f f.ne_zero :=
  (principalDivisorHom W).map_zsmul n (Additive.ofMul f)

theorem proper_evaluation_function_zpow (W : WeierstrassCurve.Affine K)
    (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ) (n : ℤ)
    (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hn : Disjoint D.support (functionDivisor W (↑(f^n) : W.FunctionField) (f^n).ne_zero).support) :
    evaluateDivisor W D (f^n) hn=evaluateDivisor W D f hd^n := by
  let u : supportUnits W D.support := ⟨f,(disjoint_principal_support_iff W f D).mp hd⟩
  exact (divisorEvaluation W D).map_zpow u n

#print axioms proper_evaluation_add
#print axioms proper_evaluation_zsmul
#print axioms unit_function_divisor_zpow
#print axioms proper_evaluation_function_zpow
end
end WeilDivisorChoiceAudit
