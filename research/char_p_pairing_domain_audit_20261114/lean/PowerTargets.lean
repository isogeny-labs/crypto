module

public import Mathlib.Algebra.CharP.Reduced
public import Mathlib.FieldTheory.Finite.Basic
public import Mathlib.GroupTheory.Index

public section

namespace CharPPairingDomainAudit
noncomputable section

def LeftBilinear {S R T : Type*} [Add S] [Mul T] (e : S → R → T) : Prop :=
  ∀ P Q Z, e (P+Q) Z=e P Z*e Q Z

def LeftNondegenerate {S R T : Type*} [Zero S] [One T] (e : S → R → T) : Prop :=
  ∀ P, (∀ Q, e P Q=1) → P=0

abbrev PowerTarget (K : Type*) [Field K] (n : ℕ) :=
  Kˣ ⧸ (powMonoidHom n : Kˣ →* Kˣ).range

variable {K : Type*} [Field K] (p : ℕ) [Fact p.Prime] [CharP K p]

theorem unit_prime_power_root (k : ℕ) (u : Kˣ) (hu : u^(p^k)=1) : u=1 := by
  apply Units.ext
  apply iterateFrobenius_inj K p k
  change (u:K)^(p^k)=(1:K)^(p^k)
  simpa only [Units.val_pow_eq_pow_val,Units.val_one,one_pow] using congrArg Units.val hu

theorem finite_unit_power_surjective [Finite K] (k : ℕ) :
    Function.Surjective (powMonoidHom (p^k) : Kˣ →* Kˣ) := by
  intro u
  obtain ⟨v,hv⟩ := Finite.surjective_of_injective (iterateFrobenius_inj K p k) (u:K)
  have hv0 : v≠0 := by
    intro hz
    have he := hv
    rw [iterateFrobenius_def,hz,zero_pow (pow_ne_zero k (Fact.out : p.Prime).ne_zero)] at he
    exact u.ne_zero he.symm
  refine ⟨Units.mk0 v hv0,?_⟩
  apply Units.ext
  exact hv

theorem finite_power_range_top [Finite K] (k : ℕ) :
    (powMonoidHom (p^k) : Kˣ →* Kˣ).range=⊤ :=
  MonoidHom.range_eq_top.mpr (finite_unit_power_surjective p k)

theorem finite_power_target_card [Finite K] (k : ℕ) :
    Nat.card (PowerTarget K (p^k))=1 := by
  change Nat.card (Kˣ ⧸ (powMonoidHom (p^k) : Kˣ →* Kˣ).range)=1
  rw [finite_power_range_top p k]
  exact Nat.card_eq_one_iff_unique.mpr ⟨QuotientGroup.subsingleton_quotient_top,⟨1⟩⟩

theorem finite_power_target_eq_one [Finite K] (k : ℕ) (u : PowerTarget K (p^k)) : u=1 := by
  have h := finite_power_range_top (K:=K) p k
  have : Subsingleton (PowerTarget K (p^k)) := by
    unfold PowerTarget
    rw [h]
    exact QuotientGroup.subsingleton_quotient_top
  exact Subsingleton.elim _ _

omit [Fact p.Prime] [CharP K p] in
theorem left_bilinear_nsmul {S R : Type*} [AddMonoid S]
    (e : S → R → Kˣ) (hb : LeftBilinear e) (P : S) (Q : R) (n : ℕ) :
    e (n • P) Q=(e P Q)^n := by
  have hz : e 0 Q=1 := mul_left_cancel (show e 0 Q*e 0 Q=e 0 Q*1 by
    simpa only [zero_add,mul_one] using (hb 0 0 Q).symm)
  induction n with
  | zero => simpa only [zero_nsmul,pow_zero] using hz
  | succ n ih => rw [succ_nsmul,hb,ih,pow_succ]

theorem characteristic_left_pairing_trivial {S R : Type*} [AddMonoid S]
    (k : ℕ) (e : S → R → Kˣ) (hb : LeftBilinear e)
    (hk : ∀ P : S,(p^k) • P=0) (P : S) (Q : R) : e P Q=1 := by
  apply unit_prime_power_root p k
  have h := left_bilinear_nsmul e hb P Q (p^k)
  rw [hk P] at h
  have hz := left_bilinear_nsmul e hb P Q 0
  simp only [zero_nsmul,pow_zero] at hz
  exact h.symm.trans hz

theorem characteristic_left_nondegenerate_impossible {S R : Type*} [AddMonoid S]
    (k : ℕ) (hk : ∀ P : S,(p^k) • P=0) (P : S) (hP : P≠0) :
    ¬∃ e : S → R → Kˣ,LeftBilinear e ∧ LeftNondegenerate e := by
  rintro ⟨e,hb,hn⟩
  exact hP (hn P (characteristic_left_pairing_trivial p k e hb hk P))

omit [CharP K p] in
/-- For prime order, a nonidentity root condition already forces coprimality.
This source guard is retained; the characteristic-order controls do not meet it. -/
theorem prime_nontrivial_root_coprime [Fintype K] (u : Kˣ)
    (hu : u^p=1) (hne : u≠1) : Nat.Coprime p (Fintype.card K) := by
  classical
  have ho : orderOf u=p := orderOf_eq_prime hu hne
  have hd : p∣Fintype.card K-1 := by
    have h := orderOf_dvd_natCard u
    rw [ho,Nat.card_eq_fintype_card,Fintype.card_units] at h
    exact h
  have hc : Nat.Coprime (Fintype.card K-1) (Fintype.card K) :=
    (Nat.coprime_self_sub_left (m:=1) (Nat.succ_le_of_lt (Fintype.card_pos (α:=K)))).mpr (by simp)
  exact hc.of_dvd_left hd

#print axioms prime_nontrivial_root_coprime
#print axioms unit_prime_power_root
#print axioms finite_power_target_card
#print axioms characteristic_left_pairing_trivial
#print axioms characteristic_left_nondegenerate_impossible
end
end CharPPairingDomainAudit
