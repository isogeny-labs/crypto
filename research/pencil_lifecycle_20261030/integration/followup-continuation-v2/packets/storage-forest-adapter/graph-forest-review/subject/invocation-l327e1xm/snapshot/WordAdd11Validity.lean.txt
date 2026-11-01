import WordAdd11
import WordErrorExclusion
set_option autoImplicit false
namespace PencilWordRefinement
open Polynomial

def WordD1.normalizedValid (p : UInt64) [Fact p.toNat.Prime]
    (f : Polynomial (ZMod p.toNat)) (a : WordD1) : Prop :=
 (X-C (WeightedAdd11.nx (a.decode p))) ∣ f-(C (WeightedAdd11.ny (a.decode p)))^2

theorem distinct_word_branch_preserves (p : UInt64) [Fact p.toNat.Prime]
    (f : Polynomial (ZMod p.toNat)) (a b : WordD1)
    (hfive : 5 ≤ p.toNat) (hbound : p.toNat < 2^31)
    (ha : a.canonical p) (hb : b.canonical p)
    (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
    (hva : a.normalizedValid p f) (hvb : b.normalizedValid p f)
    (hx : WeightedAdd11.nx (a.decode p) ≠ WeightedAdd11.nx (b.decode p)) :
    let c := add11WordPayload p a b
    wordGuard p a b = .distinct ∧ c.canonical p ∧ c.z ≠ 0 ∧ c.w ≠ 0 ∧ c.normalizedValid p f := by
  have hp : 0 < p.toNat := by omega
  have az : (a.decode p).z ≠ 0 := canonical_word_nonzero p a.z hp ha.2.2.1 haz
  have aw : (a.decode p).w ≠ 0 := canonical_word_nonzero p a.w hp ha.2.2.2 haw
  have bz : (b.decode p).z ≠ 0 := canonical_word_nonzero p b.z hp hb.2.2.1 hbz
  have bw : (b.decode p).w ≠ 0 := canonical_word_nonzero p b.w hp hb.2.2.2 hbw
  obtain ⟨hc,hq,hr,hs,ht,hz,hw⟩ := payload_decode p hfive hbound a b ha hb
  obtain ⟨fz,fw,fv⟩ := WeightedAdd11.preserves_mumford (a.decode p) (b.decode p) f az aw bz bw hx hva hvb
  refine ⟨distinct_word_guard p hfive hbound a b ha hb az bz hx,hc,?_,?_,?_⟩
  · intro he
    apply fz
    rw [←hz,he]
    simp
  · intro he
    apply fw
    rw [←hw,he]
    simp
  · unfold WeightedWord.normalizedValid
    rw [hq,hr,hs,ht,hz,hw]
    exact fv
end PencilWordRefinement
