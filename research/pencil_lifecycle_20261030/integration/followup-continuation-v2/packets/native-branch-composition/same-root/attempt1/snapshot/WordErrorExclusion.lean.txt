import CrossScaling
import WeightedSameSupport
set_option autoImplicit false
namespace PencilWordRefinement
open Polynomial

def WeightedWord.normalizedValid (p : UInt64) [Fact p.toNat.Prime] (f : Polynomial (ZMod p.toNat))
    (a : WeightedWord) : Prop :=
  (X^2 + C ((a.q.toNat : ZMod p.toNat)/(a.z.toNat : ZMod p.toNat)^2)*X +
    C ((a.r.toNat : ZMod p.toNat)/(a.z.toNat : ZMod p.toNat)^4)) ∣
  f - (C ((a.s.toNat : ZMod p.toNat)/((a.z.toNat : ZMod p.toNat)^3*(a.w.toNat : ZMod p.toNat)))*X +
    C ((a.t.toNat : ZMod p.toNat)/((a.z.toNat : ZMod p.toNat)^5*(a.w.toNat : ZMod p.toNat))))^2

theorem canonical_word_nonzero (p z : UInt64) (hp : 0 < p.toNat)
    (hz : z.toNat < p.toNat) (hnz : z ≠ 0) : (z.toNat : ZMod p.toNat) ≠ 0 := by
  intro h
  have he : z.toNat = 0 := (canonical_equality p.toNat z.toNat 0 hz hp).mp (by simpa using h)
  apply hnz
  apply UInt64.toNat.inj
  simpa using he

theorem bounded_word_prelude_excludes_zero_slope (p : UInt64)
    (f : Polynomial (ZMod p.toNat)) (a b : WeightedWord) [Fact p.toNat.Prime]
    (hfive : 5 ≤ p.toNat) (hbound : p.toNat < 2^31)
    (ha : a.canonical p) (hb : b.canonical p)
    (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
    (hva : a.normalizedValid p f) (hvb : b.normalizedValid p f) :
    crossPreludeWord p a b ≠ some ProjectiveLawRuntime.Branch22Prelude.sameUMixedZeroSlope := by
  have hp : 0 < p.toNat := by omega
  have az : (a.z.toNat : ZMod p.toNat) ≠ 0 := canonical_word_nonzero p a.z hp ha.2.2.2.2.1 haz
  have aw : (a.w.toNat : ZMod p.toNat) ≠ 0 := canonical_word_nonzero p a.w hp ha.2.2.2.2.2 haw
  have bz : (b.z.toNat : ZMod p.toNat) ≠ 0 := canonical_word_nonzero p b.z hp hb.2.2.2.2.1 hbz
  have bw : (b.w.toNat : ZMod p.toNat) ≠ 0 := canonical_word_nonzero p b.w hp hb.2.2.2.2.2 hbw
  have hodd : (2 : ZMod p.toNat) ≠ 0 := by
    intro h
    have h2 : 2 = (0 : ℕ) := (canonical_equality p.toNat 2 0 (by omega) hp).mp (by simpa using h)
    omega
  rw [cross_prelude_decode p a b hfive hbound ha hb]
  unfold crossPreludeField
  split_ifs with hsupport
  · have hx := SameSupportCandidate.weighted_valid_inputs_exclude_same_support_error
      f (a.q.toNat : ZMod p.toNat) (a.r.toNat : ZMod p.toNat)
      (a.s.toNat : ZMod p.toNat) (a.t.toNat : ZMod p.toNat)
      (a.z.toNat : ZMod p.toNat) (a.w.toNat : ZMod p.toNat)
      (b.q.toNat : ZMod p.toNat) (b.r.toNat : ZMod p.toNat)
      (b.s.toNat : ZMod p.toNat) (b.t.toNat : ZMod p.toNat)
      (b.z.toNat : ZMod p.toNat) (b.w.toNat : ZMod p.toNat)
      az bz aw bw hodd hva hvb hsupport.1 hsupport.2
    intro he
    exact hx (Option.some.inj he)
  · simp
end PencilWordRefinement
