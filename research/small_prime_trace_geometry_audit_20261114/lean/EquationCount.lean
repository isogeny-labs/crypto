import SmallCharacteristicCertificate
import SupersingularTraceClasses

namespace SmallPrimeTraceGeometryAudit
noncomputable section
open WeierstrassCurve.Affine
variable {K : Type*} [Field K]

def coefficientEquiv : (Fin 5 → K) ≃ WeierstrassCurve.Affine K where
  toFun a := ⟨a 0,a 1,a 2,a 3,a 4⟩
  invFun W := ![W.a₁,W.a₂,W.a₃,W.a₄,W.a₆]
  left_inv a := by funext i; fin_cases i <;> rfl
  right_inv W := by cases W; rfl

instance modelFintype [Fintype K] : Fintype (WeierstrassCurve.Affine K) :=
  Fintype.ofEquiv (Fin 5 → K) coefficientEquiv

def equation (W : WeierstrassCurve.Affine K) (xy : K×K) : Prop :=
  xy.2^2+W.a₁*xy.1*xy.2+W.a₃*xy.2=
    xy.1^3+W.a₂*xy.1^2+W.a₄*xy.1+W.a₆

instance equationDecidable [DecidableEq K] (W : WeierstrassCurve.Affine K) (xy : K×K) :
    Decidable (equation W xy) := inferInstanceAs (Decidable (_=_))

def equationCount [Fintype K] [DecidableEq K] (W : WeierstrassCurve.Affine K) : ℕ :=
  (Finset.univ.filter (equation W)).card+1

theorem nonsingular_iff_equation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (xy : K×K) :
    W.Nonsingular xy.1 xy.2 ↔ equation W xy := by
  rw [←W.equation_iff_nonsingular_of_Δ_ne_zero hΔ,WeierstrassCurve.Affine.equation_iff]
  rfl

theorem actual_point_count [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) : Nat.card W.Point=equationCount W := by
  let e : {xy : K×K // W.Nonsingular xy.1 xy.2} ≃ {xy : K×K // equation W xy} :=
    Equiv.subtypeEquivProp (by funext xy; exact propext (nonsingular_iff_equation W hΔ xy))
  rw [Nat.card_congr W.nonsingularPointEquiv]
  change Nat.card (Option {xy : K×K // W.Nonsingular xy.1 xy.2})=equationCount W
  rw [Nat.card_congr e.optionCongr]
  rw [Nat.card_eq_fintype_card,Fintype.card_option]
  simp only [Fintype.card_subtype,equationCount]

def countedTrace [Fintype K] [DecidableEq K] (W : WeierstrassCurve.Affine K) : ℤ :=
  (Fintype.card K:ℤ)+1-equationCount W

def countedSupersingular [Fintype K] [DecidableEq K] (W : WeierstrassCurve.Affine K) : Prop :=
  Int.gcd (countedTrace W) (Fintype.card K)≠1

instance countedSupersingularDecidable [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) : Decidable (countedSupersingular W) :=
  inferInstanceAs (Decidable (_≠1))

theorem actual_trace [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    SupersingularTraceClassesAudit.fieldTrace W=countedTrace W := by
  simp only [SupersingularTraceClassesAudit.fieldTrace,countedTrace,actual_point_count W hΔ]

def sourceSupersingular [Fintype K] (W : WeierstrassCurve.Affine K) : Prop :=
  Int.gcd (SupersingularTraceClassesAudit.fieldTrace W) (Fintype.card K)≠1

theorem source_iff_counted [Fintype K] [DecidableEq K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ countedSupersingular W := by
  rw [sourceSupersingular,actual_trace W hΔ]
  rfl

theorem modular_order_dvd (q r d : ℕ) (hq : 1≤q^d) (hr : r∣q^d-1) :
    orderOf (q:ZMod r)∣d := by
  have hz : ((q^d-1:ℕ):ZMod r)=0 := (ZMod.natCast_eq_zero_iff _ _).mpr hr
  rw [Nat.cast_sub hq,Nat.cast_pow,Nat.cast_one] at hz
  exact orderOf_dvd_of_pow_eq_one (sub_eq_zero.mp hz)

def sourceModelEquiv [Fintype K] [DecidableEq K] :
    {W : WeierstrassCurve.Affine K // W.Δ≠0 ∧ sourceSupersingular W} ≃
      {W : WeierstrassCurve.Affine K // W.Δ≠0 ∧ countedSupersingular W} :=
  Equiv.subtypeEquivProp (by
    funext W
    apply propext
    exact and_congr_right (fun hΔ => source_iff_counted W hΔ))

#print axioms actual_point_count
#print axioms actual_trace
end
end SmallPrimeTraceGeometryAudit
