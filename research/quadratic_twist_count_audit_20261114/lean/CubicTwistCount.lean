import SquareFibers

namespace QuadraticTwistCountAudit
noncomputable section
open SmallPrimeTraceGeometryAudit
variable {K : Type*} [Field K]

def cubicModel (a b c : K) : WeierstrassCurve.Affine K := ⟨0,a,0,b,c⟩
def cubicValue (a b c x : K) : K := x^3+a*x^2+b*x+c
def twistModel (a b c d : K) : WeierstrassCurve.Affine K :=
  cubicModel (d*a) (d^2*b) (d^3*c)

theorem twist_one (a b c : K) : twistModel a b c 1=cubicModel a b c := by
  simp only [twistModel,one_pow,one_mul]

theorem twist_zero_coefficients (d : K) : twistModel (0:K) 0 0 d=cubicModel 0 0 0 := by
  simp only [twistModel,mul_zero]

def equationSigmaEquiv (a b c : K) :
    {xy : K×K // equation (cubicModel a b c) xy} ≃
      (Σ x : K,squareRoots (cubicValue a b c x)) where
  toFun p := ⟨p.val.1,⟨p.val.2,by simpa [equation,cubicModel,cubicValue] using p.property⟩⟩
  invFun p := ⟨(p.1,p.2.val),by simpa [equation,cubicModel,cubicValue] using p.2.property⟩
  left_inv := by rintro ⟨⟨x,y⟩,h⟩; rfl
  right_inv := by rintro ⟨x,⟨y,h⟩⟩; rfl

theorem cubic_actual_count [Fintype K] [DecidableEq K] (a b c : K)
    (hΔ : (cubicModel a b c).Δ≠0) :
    Nat.card (cubicModel a b c).Point=(∑ x : K,rootCard (cubicValue a b c x))+1 := by
  rw [actual_point_count _ hΔ]
  have he : equationCount (cubicModel a b c)=
      Nat.card {xy : K×K // equation (cubicModel a b c) xy}+1 := by
    simp only [equationCount,Nat.card_eq_fintype_card,Fintype.card_subtype]
  rw [he,Nat.card_congr (equationSigmaEquiv a b c),Nat.card_eq_fintype_card,Fintype.card_sigma]
  simp only [rootCard,Nat.card_eq_fintype_card]

theorem twist_discriminant (a b c d : K) :
    (twistModel a b c d).Δ=d^6*(cubicModel a b c).Δ := by
  simp only [twistModel,cubicModel,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring

theorem twist_value (a b c d x : K) :
    cubicValue (d*a) (d^2*b) (d^3*c) (d*x)=d^3*cubicValue a b c x := by
  dsimp [cubicValue]
  ring

theorem twist_root_sum [Fintype K] [DecidableEq K] (a b c d : K) (hd : d≠0) :
    (∑ x : K,rootCard (cubicValue (d*a) (d^2*b) (d^3*c) x))=
      ∑ x : K,rootCard (d^3*cubicValue a b c x) := by
  calc
    _ = ∑ x : K,rootCard (cubicValue (d*a) (d^2*b) (d^3*c) (d*x)) :=
      (Equiv.sum_comp (Equiv.mulLeft₀ d hd) _).symm
    _ = _ := by simp only [twist_value]

/-- Actual elliptic point counts, including the identity, over every finite
odd-characteristic field. Nonsquare scaling is explicit; no nonisomorphism
of the two curves or Frobenius trace formula is assumed. -/
theorem actual_twist_count_sum [Fintype K] [DecidableEq K] (hF : ringChar K≠2)
    (a b c d : K) (hΔ : (cubicModel a b c).Δ≠0) (hd : ¬IsSquare d) :
    Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=
      2*(Fintype.card K+1) := by
  have hd0 : d≠0 := by
    intro h
    apply hd
    rw [h]
    exact ⟨0,by simp⟩
  have ht : (twistModel a b c d).Δ≠0 := by
    rw [twist_discriminant]
    exact mul_ne_zero (pow_ne_zero 6 hd0) hΔ
  have hs : (∑ x : K,rootCard (cubicValue a b c x))+
      (∑ x : K,rootCard (d^3*cubicValue a b c x))=Fintype.card K*2 := by
    rw [←Finset.sum_add_distrib]
    simp only [scaled_root_card_sum hF d hd,Finset.sum_const,Finset.card_univ,smul_eq_mul]
  rw [cubic_actual_count a b c hΔ]
  change (∑ x : K,rootCard (cubicValue a b c x))+1+
    Nat.card (cubicModel (d*a) (d^2*b) (d^3*c)).Point=2*(Fintype.card K+1)
  rw [cubic_actual_count _ _ _ ht,twist_root_sum a b c d hd0]
  omega

theorem actual_twist_trace_neg [Fintype K] [DecidableEq K] (hF : ringChar K≠2)
    (a b c d : K) (hΔ : (cubicModel a b c).Δ≠0) (hd : ¬IsSquare d) :
    SupersingularTraceClassesAudit.fieldTrace (twistModel a b c d)=
      -SupersingularTraceClassesAudit.fieldTrace (cubicModel a b c) := by
  have hc := actual_twist_count_sum hF a b c d hΔ hd
  have hi := congrArg (fun n : ℕ => (n:ℤ)) hc
  push_cast at hi
  unfold SupersingularTraceClassesAudit.fieldTrace
  omega

theorem actual_twist_source_iff [Fintype K] [DecidableEq K] (hF : ringChar K≠2)
    (a b c d : K) (hΔ : (cubicModel a b c).Δ≠0) (hd : ¬IsSquare d) :
    sourceSupersingular (twistModel a b c d) ↔ sourceSupersingular (cubicModel a b c) := by
  unfold sourceSupersingular
  rw [actual_twist_trace_neg hF a b c d hΔ hd]
  simp only [Int.gcd_def,Int.natAbs_neg]

#print axioms cubic_actual_count
#print axioms twist_discriminant
#print axioms actual_twist_count_sum
end
end QuadraticTwistCountAudit
