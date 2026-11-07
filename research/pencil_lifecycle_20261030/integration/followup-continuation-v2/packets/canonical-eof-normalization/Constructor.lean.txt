import WordErrorExclusion
set_option autoImplicit false
namespace PencilWordRefinement
open Polynomial

def constructorExpr (q r s t z w : UInt64) : MulExpr × MulExpr × MulExpr × MulExpr :=
  let z2 := MulExpr.mul (.leaf z) (.leaf z)
  let z3 := MulExpr.mul z2 (.leaf z)
  let z4 := MulExpr.mul z2 z2
  let z5 := MulExpr.mul z4 (.leaf z)
  (.mul (.leaf q) z2, .mul (.leaf r) z4,
   .mul (.leaf s) (.mul z3 (.leaf w)), .mul (.leaf t) (.mul z5 (.leaf w)))

def weightConstructor (p q r s t z w : UInt64) : WeightedWord :=
  let e := constructorExpr q r s t z w
  ⟨e.1.word p,e.2.1.word p,e.2.2.1.word p,e.2.2.2.word p,z,w⟩

theorem constructor_decode (p q r s t z w : UInt64)
    (hp : 0 < p.toNat) (hb : p.toNat < 2^31)
    (hq : q.toNat < p.toNat) (hr : r.toNat < p.toNat)
    (hs : s.toNat < p.toNat) (ht : t.toNat < p.toNat)
    (hz : z.toNat < p.toNat) (hw : w.toNat < p.toNat) :
    let a := weightConstructor p q r s t z w
    a.canonical p ∧
    (a.q.toNat : ZMod p.toNat) = (q.toNat : ZMod p.toNat)*(z.toNat : ZMod p.toNat)^2 ∧
    (a.r.toNat : ZMod p.toNat) = (r.toNat : ZMod p.toNat)*(z.toNat : ZMod p.toNat)^4 ∧
    (a.s.toNat : ZMod p.toNat) = (s.toNat : ZMod p.toNat)*((z.toNat : ZMod p.toNat)^3*(w.toNat : ZMod p.toNat)) ∧
    (a.t.toNat : ZMod p.toNat) = (t.toNat : ZMod p.toNat)*((z.toNat : ZMod p.toNat)^5*(w.toNat : ZMod p.toNat)) := by
  have g1 : (constructorExpr q r s t z w).1.good p := by simp [constructorExpr,MulExpr.good,hq,hz]
  have g2 : (constructorExpr q r s t z w).2.1.good p := by simp [constructorExpr,MulExpr.good,hr,hz]
  have g3 : (constructorExpr q r s t z w).2.2.1.good p := by simp [constructorExpr,MulExpr.good,hs,hz,hw]
  have g4 : (constructorExpr q r s t z w).2.2.2.good p := by simp [constructorExpr,MulExpr.good,ht,hz,hw]
  have h1 := MulExpr.composition p hp hb _ g1
  have h2 := MulExpr.composition p hp hb _ g2
  have h3 := MulExpr.composition p hp hb _ g3
  have h4 := MulExpr.composition p hp hb _ g4
  refine ⟨⟨h1.1,h2.1,h3.1,h4.1,hz,hw⟩,?_,?_,?_,?_⟩
  all_goals
    first | change (((constructorExpr q r s t z w).1.word p).toNat : ZMod p.toNat) = _; rw [h1.2]
          | change (((constructorExpr q r s t z w).2.1.word p).toNat : ZMod p.toNat) = _; rw [h2.2]
          | change (((constructorExpr q r s t z w).2.2.1.word p).toNat : ZMod p.toNat) = _; rw [h3.2]
          | change (((constructorExpr q r s t z w).2.2.2.word p).toNat : ZMod p.toNat) = _; rw [h4.2]
    simp only [constructorExpr,MulExpr.field]
    ring

theorem constructor_preserves_invariant (p q r s t z w : UInt64) [Fact p.toNat.Prime]
    (hp : 0 < p.toNat) (hb : p.toNat < 2^31)
    (hq : q.toNat < p.toNat) (hr : r.toNat < p.toNat)
    (hs : s.toNat < p.toNat) (ht : t.toNat < p.toNat)
    (hz : z.toNat < p.toNat) (hw : w.toNat < p.toNat)
    (hnz : z ≠ 0) (hnw : w ≠ 0) (f : Polynomial (ZMod p.toNat))
    (valid : (X^2+C (q.toNat : ZMod p.toNat)*X+C (r.toNat : ZMod p.toNat)) ∣
      f-(C (s.toNat : ZMod p.toNat)*X+C (t.toNat : ZMod p.toNat))^2) :
    let a := weightConstructor p q r s t z w
    a.canonical p ∧ a.z ≠ 0 ∧ a.w ≠ 0 ∧ a.normalizedValid p f := by
  obtain ⟨hc,hq',hr',hs',ht'⟩ := constructor_decode p q r s t z w hp hb hq hr hs ht hz hw
  have hzf := canonical_word_nonzero p z hp hz hnz
  have hwf := canonical_word_nonzero p w hp hw hnw
  refine ⟨hc,hnz,hnw,?_⟩
  unfold WeightedWord.normalizedValid
  rw [hq',hr',hs',ht']
  simpa only [weightConstructor, mul_div_cancel_right₀ _ (pow_ne_zero 2 hzf),
    mul_div_cancel_right₀ _ (pow_ne_zero 4 hzf),
    mul_div_cancel_right₀ _ (mul_ne_zero (pow_ne_zero 3 hzf) hwf),
    mul_div_cancel_right₀ _ (mul_ne_zero (pow_ne_zero 5 hzf) hwf)] using valid

-- Literal degree-two arm of from_affine: unit weights, no inversion.
theorem from_affine_two_invariant (p q r s t : UInt64) [Fact p.toNat.Prime]
    (hfive : 5 ≤ p.toNat) (hb : p.toNat < 2^31)
    (hq : q.toNat < p.toNat) (hr : r.toNat < p.toNat)
    (hs : s.toNat < p.toNat) (ht : t.toNat < p.toNat)
    (f : Polynomial (ZMod p.toNat))
    (valid : (X^2+C (q.toNat : ZMod p.toNat)*X+C (r.toNat : ZMod p.toNat)) ∣
      f-(C (s.toNat : ZMod p.toNat)*X+C (t.toNat : ZMod p.toNat))^2) :
    let a := weightConstructor p q r s t 1 1
    a.canonical p ∧ a.z ≠ 0 ∧ a.w ≠ 0 ∧ a.normalizedValid p f := by
  exact constructor_preserves_invariant p q r s t 1 1 (by omega) hb hq hr hs ht
    (by simpa using (show 1 < p.toNat by omega))
    (by simpa using (show 1 < p.toNat by omega)) (by decide) (by decide) f valid

-- Constructor-to-prelude composition; partner validity remains an explicit input.
theorem initialized_against_valid_partner (p q r s t : UInt64) [Fact p.toNat.Prime]
    (hfive : 5 ≤ p.toNat) (hb : p.toNat < 2^31)
    (hq : q.toNat < p.toNat) (hr : r.toNat < p.toNat)
    (hs : s.toNat < p.toNat) (ht : t.toNat < p.toNat)
    (f : Polynomial (ZMod p.toNat))
    (valid : (X^2+C (q.toNat : ZMod p.toNat)*X+C (r.toNat : ZMod p.toNat)) ∣
      f-(C (s.toNat : ZMod p.toNat)*X+C (t.toNat : ZMod p.toNat))^2)
    (b : WeightedWord) (hbc : b.canonical p) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
    (hbv : b.normalizedValid p f) :
    crossPreludeWord p (weightConstructor p q r s t 1 1) b ≠
      some ProjectiveLawRuntime.Branch22Prelude.sameUMixedZeroSlope := by
  obtain ⟨hc,hz,hw,hv⟩ := from_affine_two_invariant p q r s t hfive hb hq hr hs ht f valid
  exact bounded_word_prelude_excludes_zero_slope p f _ b hfive hb hc hbc hz hw hbz hbw hv hbv
end PencilWordRefinement
