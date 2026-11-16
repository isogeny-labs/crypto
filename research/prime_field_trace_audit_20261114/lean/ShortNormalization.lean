import EquationCount
import ShiftPointGroup

namespace PrimeFieldTraceAudit
noncomputable section
open BKThreeNormalizationAudit SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit
variable {K : Type*} [Field K]

def shortModel (a b : K) : WeierstrassCurve.Affine K := ⟨0,0,0,a,b⟩
def normalS (W : WeierstrassCurve.Affine K) : K := -W.a₁/2
def normalR (W : WeierstrassCurve.Affine K) : K :=
  -(W.a₂-normalS W*W.a₁-(normalS W)^2)/3
def normalT (W : WeierstrassCurve.Affine K) : K := -(W.a₃+normalR W*W.a₁)/2
def normalized (W : WeierstrassCurve.Affine K) : WeierstrassCurve.Affine K :=
  shifted W (normalR W) (normalS W) (normalT W)

theorem normalized_shape (W : WeierstrassCurve.Affine K)
    (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    normalized W=shortModel (normalized W).a₄ (normalized W).a₆ := by
  ext <;> dsimp [normalized,shifted,shortModel,normalR,normalS,normalT]
  all_goals field_simp [h2,h3]
  all_goals ring

theorem normalized_delta (W : WeierstrassCurve.Affine K) : (normalized W).Δ=W.Δ :=
  shifted_discriminant W _ _ _

def normalizationEquiv [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (normalized W).Point ≃+ W.Point := shiftEquiv W hΔ _ _ _

theorem normalized_card [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (normalized W).Point=Nat.card W.Point :=
  Nat.card_congr (normalizationEquiv W hΔ).toEquiv

theorem normalization_semantics [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card W.Point=equationCount (normalized W) ∧
    fieldTrace W=countedTrace (normalized W) ∧
    (sourceSupersingular W ↔ countedSupersingular (normalized W)) := by
  have hc : Nat.card W.Point=equationCount (normalized W) := by
    rw [←normalized_card W hΔ]
    exact actual_point_count _ (by rwa [normalized_delta])
  have ht : fieldTrace W=countedTrace (normalized W) := by
    simp only [fieldTrace,countedTrace,hc]
  exact ⟨hc,ht,by simp only [sourceSupersingular,countedSupersingular,ht]⟩

/-- Transfer a closed short-coefficient table to every actual general model.
No normalization, point-count or Hasse premise is hidden in the conclusion. -/
theorem transfer_short_table [Fintype K] [DecidableEq K]
    (h2 : (2:K)≠0) (h3 : (3:K)≠0)
    (table : ∀ a b : K,(shortModel a b).Δ≠0 →
      (countedTrace (shortModel a b))^2≤4*(Fintype.card K:ℤ) ∧
      (countedSupersingular (shortModel a b) ↔ equationCount (shortModel a b)=Fintype.card K+1))
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (fieldTrace W)^2≤4*(Fintype.card K:ℤ) ∧
    (sourceSupersingular W ↔ Nat.card W.Point=Fintype.card K+1) := by
  have hn : (normalized W).Δ≠0 := by rwa [normalized_delta]
  have hs := normalized_shape W h2 h3
  have tt := table (normalized W).a₄ (normalized W).a₆ (by rwa [←hs])
  rw [←hs] at tt
  obtain ⟨hc,ht,hss⟩ := normalization_semantics W hΔ
  simpa only [ht,hss,hc] using tt

/-- Composite moduli are allowed; only the small modulus r≤2 is excluded. -/
theorem composite_trace_zero_order (q r : ℕ) (hr : 2<r) (hd : r∣q+1) :
    orderOf (q:ZMod r)=2 := by
  have hz : (q:ZMod r)+1=0 := by
    simpa only [Nat.cast_add,Nat.cast_one] using (ZMod.natCast_eq_zero_iff (q+1) r).mpr hd
  have hq : (q:ZMod r) = -1 := (eq_neg_iff_add_eq_zero).mpr hz
  apply orderOf_eq_prime
  · rw [hq]
    ring
  · intro h
    have htwo : (2:ZMod r)=0 := by linear_combination hz-h
    have hdiv : r∣2 := (ZMod.natCast_eq_zero_iff 2 r).mp htwo
    exact (not_le.mpr hr) (Nat.le_of_dvd (by decide) hdiv)

#print axioms normalized_shape
#print axioms normalizationEquiv
#print axioms transfer_short_table
#print axioms composite_trace_zero_order
end
end PrimeFieldTraceAudit
