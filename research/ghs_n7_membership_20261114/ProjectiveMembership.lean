import Init

/-!
The scaling step of the projective W1 membership test. This theorem is
parametric in the field multiplication and scalar action. Its hypotheses
state the algebraic facts that must be supplied for a concrete finite field:
every first-factor element has a representative, multiplication commutes
with scaling, and the exact second-order condition is stable under nonzero
base-field scaling. It does not prove the Frobenius-kernel dimension, the
q²+q+1 line count, or the geometric GHS transfer.
-/

namespace GHSN7

variable {K C : Type}
variable (mul : K → K → K) (scale : C → K → K)
variable (First Second Representative : K → Prop)
variable (ScalarUnit : C → Prop)

def Direct (mul : K → K → K) (First Second : K → Prop) (z : K) : Prop :=
  ∃ v w, First v ∧ Second w ∧ z = mul v w

def Projective (mul : K → K → K)
    (Representative Second : K → Prop) (z : K) : Prop :=
  ∃ r w, Representative r ∧ Second w ∧ z = mul r w

theorem projective_iff_direct
    (representatives_first : ∀ r, Representative r → First r)
    (covers : ∀ v, First v → ∃ (c : C) (r : K),
      ScalarUnit c ∧ Representative r ∧ v = scale c r)
    (second_scaled : ∀ c w, ScalarUnit c → Second w →
      ∃ w', Second w' ∧ ∀ r, mul (scale c r) w = mul r w')
    (z : K) :
    Projective mul Representative Second z ↔
      Direct mul First Second z := by
  constructor
  · rintro ⟨r, w, hr, hw, hz⟩
    exact ⟨r, w, representatives_first r hr, hw, hz⟩
  · rintro ⟨v, w, hv, hw, hz⟩
    obtain ⟨c, r, hc, hr, hvr⟩ := covers v hv
    obtain ⟨w', hw', hmul⟩ := second_scaled c w hc hw
    exact ⟨r, w', hr, hw', hz.trans (hvr ▸ hmul r)⟩

end GHSN7
