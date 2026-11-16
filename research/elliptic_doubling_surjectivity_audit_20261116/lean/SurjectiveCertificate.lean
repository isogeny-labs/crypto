import SurjectiveControls

namespace EllipticDoublingSurjectivityAudit
noncomputable section

def ActualPointDoublingSurjectivity : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K),
    ∀ (W : WeierstrassCurve.Affine K) (_hΔ : W.Δ≠0),
      Function.Surjective (fun P : W.Point=>P+P)

def ActualDyadicSurjectivity : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K),
    ∀ (W : WeierstrassCurve.Affine K) (_hΔ : W.Δ≠0) (n : ℕ),
      Function.Surjective (fun P : W.Point=>(2^n : ℕ) • P)

def ActualRationalCounter : Prop:=
  fiveCurve.Δ≠0 ∧ fivePoint≠0 ∧ (∀ P : fiveCurve.Point,P+P=0) ∧
    ¬∃ P : fiveCurve.Point,P+P=fivePoint

theorem actual_doubling_surjectivity_certificate :
    ActualPointDoublingSurjectivity ∧ ActualDyadicSurjectivity ∧
      ActualRationalCounter ∧ ActualSurjectivityControls ∧ ¬UnguardedPointSurjectivityReading := by
  refine ⟨?_,?_,⟨five_delta,five_point_ne_zero,five_all_doubles_zero,actual_five_no_half⟩,
    actual_surjectivity_controls,unguarded_point_surjectivity_false⟩
  · intro K _ _ _ W hΔ;exact actual_doubling_surjective W hΔ
  · intro K _ _ _ W hΔ n;exact actual_dyadic_surjective W hΔ n

#print axioms actual_doubling_surjectivity_certificate
end
end EllipticDoublingSurjectivityAudit
