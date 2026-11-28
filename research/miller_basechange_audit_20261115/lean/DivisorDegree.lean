module
public import NegationCertificate

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open NegationPullbackAudit
open FunctionDivisorFinitenessAudit MillerFinitePointAudit
variable {K : Type*} [Field K]

theorem function_divisor_congr (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) (he : f = g) : functionDivisor W f hf = functionDivisor W g hg := by
  ext P
  change pointOrder W f P = pointOrder W g P
  rw [he]

/-- Sum of actual integer point-divisor coefficients. Over an algebraically closed field,
the actual nonsingular elliptic points are all geometric points. -/
def divisorDegree (W : WeierstrassCurve.Affine K) : (W.Point →₀ ℤ) →+ ℤ where
  toFun D := D.sum fun _ n => n
  map_zero' := by simp
  map_add' D E := Finsupp.sum_add_index' (fun _ => rfl) (fun _ _ _ => rfl)

theorem degree_single (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℤ) :
    divisorDegree W (Finsupp.single P n) = n := by
  simp [divisorDegree]

theorem degree_negation (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) :
    divisorDegree W (divisorNeg W D) = divisorDegree W D := by
  change (Finsupp.equivMapDomain (pointNegEquiv W) D).sum (fun _ n => n) = D.sum (fun _ n => n)
  rw [Finsupp.equivMapDomain_eq_mapDomain]
  exact Finsupp.sum_mapDomain_index (fun _ => rfl) (fun _ _ _ => rfl)

theorem degree_nonnegative (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (hD : ∀ P, 0 ≤ D P) : 0 ≤ divisorDegree W D :=
  Finset.sum_nonneg (fun P _ => hD P)

theorem nonnegative_degree_zero (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (hD : ∀ P, 0 ≤ D P) (hd : divisorDegree W D = 0) : D = 0 := by
  classical
  ext P
  by_cases hz : D P = 0
  · exact hz
  have hm : P ∈ D.support := Finsupp.mem_support_iff.mpr hz
  have hle : D P ≤ divisorDegree W D := Finset.single_le_sum (fun Q _ => hD Q) hm
  rw [hd] at hle
  exact le_antisymm hle (hD P)

#print axioms degree_negation
#print axioms nonnegative_degree_zero
end
end GeometricPrincipalDivisorAudit
