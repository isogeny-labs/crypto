import PhaseRecovery

/-! Cache validation implies exactly the original ordered sign-lifting rule.
Native request costs and algorithm execution are audited separately. -/
namespace SignedCache

def sign (minus : Bool) : ℤ := if minus then -1 else 1
def signedPoint {A : Type*} [AddCommGroup A] (P : A) (minus : Bool) : A :=
  if minus then -P else P

theorem signed_eq_zsmul {A : Type*} [AddCommGroup A] (P : A) (minus : Bool) :
    signedPoint P minus=sign minus • P := by
  cases minus <;> simp [signedPoint,sign]

theorem validated_cache_value {A ι : Type*} [AddCommGroup A]
    (base : ι → A) (cache : ι → Bool → A)
    (hp : ∀ i, cache i false=base i)
    (hn : ∀ i, cache i true+base i=0) (i : ι) (minus : Bool) :
    cache i minus=signedPoint (base i) minus := by
  cases minus with
  | false => exact hp i
  | true =>
    change cache i true= -base i
    calc
      cache i true=cache i true+base i-base i := by abel
      _ = -base i := by rw [hn i]; simp

theorem cached_sum_eq {A ι : Type*} [AddCommGroup A]
    (base : ι → A) (cache : ι → Bool → A)
    (hp : ∀ i, cache i false=base i)
    (hn : ∀ i, cache i true+base i=0) (i j : ι) (s t : Bool) :
    cache i s+cache j t=sign s • base i+sign t • base j := by
  rw [validated_cache_value base cache hp hn i s,
    validated_cache_value base cache hp hn j t,signed_eq_zsmul,signed_eq_zsmul]

theorem ordered_lifts_eq {A ι : Type*} [AddCommGroup A] [DecidableEq A]
    (base : ι → A) (cache : ι → Bool → A)
    (hp : ∀ i, cache i false=base i)
    (hn : ∀ i, cache i true+base i=0) (Q : A)
    (candidates : List ((ι × Bool) × (ι × Bool))) :
    candidates.filter (fun a => decide (cache a.1.1 a.1.2+cache a.2.1 a.2.2=Q)) =
    candidates.filter (fun a => decide (sign a.1.2 • base a.1.1+sign a.2.2 • base a.2.1=Q)) := by
  have h : (fun a : (ι × Bool) × (ι × Bool) =>
      decide (cache a.1.1 a.1.2+cache a.2.1 a.2.2=Q)) =
      (fun a => decide (sign a.1.2 • base a.1.1+sign a.2.2 • base a.2.1=Q)) := by
    funext a
    rw [cached_sum_eq base cache hp hn]
  rw [h]

theorem cached_relation_recovers_mod {A H : Type*} [AddCommGroup A] [AddCommGroup H]
    (φ : A →+ H) (hφ : Function.Injective φ) (P Q : A)
    (base : Fin 2 → H) (cache : Fin 2 → Bool → H)
    (hp : ∀ i, cache i false=base i)
    (hn : ∀ i, cache i true+base i=0)
    (logs : Fin 2 → ℤ) (shift r : ℤ) (hr : r • P=0)
    (hb : ∀ i, base i=logs i • φ P)
    (i j : Fin 2) (s t : Bool) (hrel : cache i s+cache j t=φ Q+shift • φ P) :
    Q=((sign s*logs i+sign t*logs j-shift)%r) • P := by
  apply PhaseRecovery.shifted_relation_recovers_mod φ hφ P Q (base i) (base j)
    (logs i) (logs j) (sign s) (sign t) shift r hr (hb i) (hb j)
  exact (cached_sum_eq base cache hp hn i j s t).symm.trans hrel

theorem coordinate_inverse {k : Type*} [CommRing k] (y : k) : y+(-y)=0 := by simp

namespace F43
open S3System.F43

def cache (i : Fin 2) (minus : Bool) : T.Point :=
  if minus then ![tp 15 34 (by decide),tp 17 37 (by decide)] i else twistBase i

theorem actual_cache_valid :
    (∀ i, cache i false=twistBase i) ∧ (∀ i, cache i true+twistBase i=0) := by
  constructor
  · intro i; rfl
  · intro i; fin_cases i <;> decide +kernel

def logs : Fin 2 → ℤ := ![5,7]

theorem actual_base_logs : ∀ i, twistBase i=logs i • U := by
  intro i; fin_cases i <;> decide +kernel

theorem actual_cached_recovery (Q : E.Point) (shift : ℤ)
    (i j : Fin 2) (s t : Bool) (hrel : cache i s+cache j t=toTwist Q+shift • U) :
    Q=((sign s*logs i+sign t*logs j-shift)%11) • P := by
  apply cached_relation_recovers_mod toTwist toTwist_injective P Q twistBase cache
    actual_cache_valid.1 actual_cache_valid.2 logs shift 11
    PhaseRecovery.F43.source_annihilator
  · intro n; simpa only [source_image] using actual_base_logs n
  · simpa only [source_image] using hrel

#print axioms actual_base_logs
#print axioms actual_cache_valid
#print axioms actual_cached_recovery
end F43
#print axioms signed_eq_zsmul
#print axioms validated_cache_value
#print axioms cached_sum_eq
#print axioms ordered_lifts_eq
#print axioms cached_relation_recovers_mod
#print axioms coordinate_inverse
end SignedCache
