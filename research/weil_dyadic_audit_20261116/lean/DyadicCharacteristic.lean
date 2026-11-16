import DyadicNondegenerate

namespace WeilDyadicAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit
variable {K : Type*} [Field K]
private instance : Fact (Nat.Prime 2):=⟨by decide⟩

theorem unit_dyadic_root_one [CharP K 2] (n : ℕ) (u : Kˣ)
    (hu : u^(2^n : ℕ)=1) : u=1 := by
  induction n with
  | zero => simpa only [pow_zero,pow_one] using hu
  | succ n ih =>
    apply ih
    apply characteristic_root_one 2
    simpa only [pow_succ,pow_mul] using hu

variable [DecidableEq K] [IsAlgClosed K]

theorem geometric_dyadic_characteristic_trivial [CharP K 2]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ)
    (P Q : torsionPoints W ((2 : ℤ)^n)) : geometricPairing W hΔ ((2 : ℤ)^n) P Q=1 := by
  apply unit_dyadic_root_one n
  have h:=geometric_pairing_root W hΔ ((2 : ℤ)^n) P Q
  have hc : ((2^n : ℕ) : ℤ)=(2 : ℤ)^n := by simp only [Nat.cast_pow,Nat.cast_ofNat]
  have hcast := congrArg
    (fun e : ℤ => geometricPairing W hΔ ((2 : ℤ)^n) P Q ^ e) hc
  simpa only [zpow_natCast] using hcast.trans h

def ordinaryDyadicPoint (n : ℕ) := twoToDyadic _ n WeilTwoNondegeneracyAudit.ordinaryBinaryPoint

theorem actual_binary_dyadic_radical (n : ℕ) :
    WeilTwoNondegeneracyAudit.ordinaryBinaryCurve.Δ≠0 ∧ ordinaryDyadicPoint n≠0 ∧
      ∀ Q,geometricPairing WeilTwoNondegeneracyAudit.ordinaryBinaryCurve
        WeilTwoNondegeneracyAudit.ordinary_binary_delta ((2 : ℤ)^(n+1)) (ordinaryDyadicPoint n) Q=1 := by
  refine ⟨WeilTwoNondegeneracyAudit.ordinary_binary_delta,
    two_to_dyadic_ne_zero _ n _ WeilTwoNondegeneracyAudit.ordinary_binary_point_ne_zero,?_⟩
  intro Q
  exact geometric_dyadic_characteristic_trivial _ _ _ _ Q

def UnguardedDyadicNondegeneracyReading : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (_ : IsAlgClosed K),
    ∀ (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℕ)
      (P : torsionPoints W ((2 : ℤ)^(n+1))),
      (∀ Q,geometricPairing W hΔ ((2 : ℤ)^(n+1)) P Q=1)→P=0

theorem unguarded_dyadic_nondegeneracy_false : ¬UnguardedDyadicNondegeneracyReading := by
  intro h
  exact (actual_binary_dyadic_radical 0).2.1
    (h WeilTwoNondegeneracyAudit.BinaryClosure inferInstance inferInstance inferInstance
      _ WeilTwoNondegeneracyAudit.ordinary_binary_delta 0 _ (actual_binary_dyadic_radical 0).2.2)

#print axioms unit_dyadic_root_one
#print axioms geometric_dyadic_characteristic_trivial
#print axioms actual_binary_dyadic_radical
#print axioms unguarded_dyadic_nondegeneracy_false
end
end WeilDyadicAudit
