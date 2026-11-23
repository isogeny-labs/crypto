import MixedCancellation
import WordAdd11Validity
set_option autoImplicit false
namespace PencilWordRefinement
open Polynomial

def WeightedWord.decodeTwo (p : UInt64) (b : WeightedWord) : MixedCancellation.D2 (ZMod p.toNat) :=
 ⟨b.q.toNat,b.r.toNat,b.s.toNat,b.t.toNat,b.z.toNat,b.w.toNat⟩
structure MixedExpr where
 k : FieldExpr
 l : FieldExpr
 ry : FieldExpr
 x : FieldExpr
 y : FieldExpr
 z : FieldExpr
 w : FieldExpr

def mixedExpr (a : WordD1) (b : WeightedWord) : MixedExpr :=
 let z12 := FieldExpr.mul (.leaf a.z) (.leaf a.z)
 let z13 := FieldExpr.mul z12 (.leaf a.z)
 let z14 := FieldExpr.mul z12 z12
 let z15 := FieldExpr.mul z14 (.leaf a.z)
 let z22 := FieldExpr.mul (.leaf b.z) (.leaf b.z)
 let z24 := FieldExpr.mul z22 z22
 let z25 := FieldExpr.mul z24 (.leaf b.z)
 let xz2 := FieldExpr.mul (.leaf a.x) z22
 let k := FieldExpr.mul (.add (.add (.mul (.leaf b.q) (.mul xz2 z12))
   (.mul (.mul (.leaf a.x) (.leaf a.x)) z24)) (.mul (.leaf b.r) z14)) (.leaf a.w)
 let y3 := FieldExpr.add (.mul (.leaf b.s) (.mul z13 xz2)) (.mul (.leaf b.t) z15)
 let w3 := FieldExpr.mul z25 (.leaf b.w)
 let x4 := FieldExpr.sub (.neg (.mul (.leaf b.q) z12)) xz2
 let y4 := FieldExpr.add (.mul (.leaf b.s) (.mul z13 x4)) (.mul (.leaf b.t) z15)
 ⟨k,.mul (.leaf a.y) w3,.mul y3 (.leaf a.w),x4,y4,.mul (.leaf a.z) (.leaf b.z),.leaf b.w⟩
structure MixedWords where
 k : UInt64
 l : UInt64
 ry : UInt64
 point : WordD1

def mixedWords (p : UInt64) (a : WordD1) (b : WeightedWord) : MixedWords :=
 let z12 := (a.z*a.z)%p
 let z13 := (z12*a.z)%p
 let z14 := (z12*z12)%p
 let z15 := (z14*a.z)%p
 let z22 := (b.z*b.z)%p
 let z24 := (z22*z22)%p
 let z25 := (z24*b.z)%p
 let xz2 := (a.x*z22)%p
 let k := ((wordAdd p (wordAdd p ((b.q*((xz2*z12)%p))%p) ((((a.x*a.x)%p)*z24)%p)) ((b.r*z14)%p))*a.w)%p
 let y3 := wordAdd p ((b.s*((z13*xz2)%p))%p) ((b.t*z15)%p)
 let w3 := (z25*b.w)%p
 let x4 := wordSub p (wordNeg p ((b.q*z12)%p)) xz2
 let y4 := wordAdd p ((b.s*((z13*x4)%p))%p) ((b.t*z15)%p)
 ⟨k,(a.y*w3)%p,(y3*a.w)%p,⟨x4,y4,(a.z*b.z)%p,b.w⟩⟩

theorem mixed_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord)
 (ha : a.canonical p) (hc : b.canonical p) :
 let m := mixedWords p a b
 let af := a.decode p
 let bf := b.decodeTwo p
 m.k.toNat<p.toNat ∧ m.l.toNat<p.toNat ∧ m.ry.toNat<p.toNat ∧ m.point.canonical p ∧
 (m.k.toNat:ZMod p.toNat)=MixedCancellation.rawK af bf ∧
 (m.l.toNat:ZMod p.toNat)=af.y*MixedCancellation.rawW3 bf ∧
 (m.ry.toNat:ZMod p.toNat)=MixedCancellation.rawY3 af bf*af.w ∧
 m.point.decode p=MixedCancellation.other af bf := by
 have hg : (mixedExpr a b).k.good p ∧ (mixedExpr a b).l.good p ∧ (mixedExpr a b).ry.good p ∧
   (mixedExpr a b).x.good p ∧ (mixedExpr a b).y.good p ∧ (mixedExpr a b).z.good p ∧ (mixedExpr a b).w.good p := by
  rcases ha with ⟨hax,hay,haz,haw⟩
  rcases hc with ⟨hbq,hbr,hbs,hbt,hbz,hbw⟩
  simp [mixedExpr,FieldExpr.good,hax,hay,haz,haw,hbq,hbr,hbs,hbt,hbz,hbw]
 have hk := FieldExpr.composition p hfive hb _ hg.1
 have hl := FieldExpr.composition p hfive hb _ hg.2.1
 have hr := FieldExpr.composition p hfive hb _ hg.2.2.1
 have hx := FieldExpr.composition p hfive hb _ hg.2.2.2.1
 have hy := FieldExpr.composition p hfive hb _ hg.2.2.2.2.1
 have hz := FieldExpr.composition p hfive hb _ hg.2.2.2.2.2.1
 have hw := FieldExpr.composition p hfive hb _ hg.2.2.2.2.2.2
 refine ⟨hk.1,hl.1,hr.1,⟨hx.1,hy.1,hz.1,hw.1⟩,?_,?_,?_,?_⟩
 · change (((mixedExpr a b).k.word p).toNat:ZMod p.toNat)=_
   rw [hk.2]; rfl
 · change (((mixedExpr a b).l.word p).toNat:ZMod p.toNat)=_
   rw [hl.2]; rfl
 · change (((mixedExpr a b).ry.word p).toNat:ZMod p.toNat)=_
   rw [hr.2]; rfl
 · change WeightedAdd11.Input.mk (((mixedExpr a b).x.word p).toNat:ZMod p.toNat)
     (((mixedExpr a b).y.word p).toNat:ZMod p.toNat)
     (((mixedExpr a b).z.word p).toNat:ZMod p.toNat)
     (((mixedExpr a b).w.word p).toNat:ZMod p.toNat)=_
   rw [hx.2,hy.2,hz.2,hw.2]; rfl
def mixedClassifyWord (p : UInt64) (a : WordD1) (b : WeightedWord) : MixedCancellation.Branch :=
 let m := mixedWords p a b
 if m.k ≠ 0 then .common
 else if m.l = wordNeg p m.ry then .cancellation
 else if m.l ≠ m.ry then .mismatch else .positiveCoincidence

theorem mixed_classifier_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord)
 (ha : a.canonical p) (hc : b.canonical p) :
 mixedClassifyWord p a b = MixedCancellation.classify (a.decode p) (b.decodeTwo p) := by
 obtain ⟨hk,hl,hr,hpnt,hkd,hld,hrd,hpd⟩ := mixed_decode p hfive hb a b ha hc
 have hp : 0<p.toNat := by omega
 have hn := wordNeg_composition p (mixedWords p a b).ry hp hr
 have eqw (u v : UInt64) (hu : u.toNat<p.toNat) (hv : v.toNat<p.toNat) :
   u=v ↔ (u.toNat:ZMod p.toNat)=(v.toNat:ZMod p.toNat) := by
  rw [←UInt64.toNat_inj]
  exact (canonical_equality p.toNat u.toNat v.toNat hu hv).symm
 have ek := eqw (mixedWords p a b).k 0 hk (by simpa using hp)
 have en := eqw (mixedWords p a b).l (wordNeg p (mixedWords p a b).ry) hl hn.1
 have er := eqw (mixedWords p a b).l (mixedWords p a b).ry hl hr
 unfold mixedClassifyWord MixedCancellation.classify
 simp only [ne_eq,ek,en,er,hkd,hld,hrd,hn.2,UInt64.toNat_zero,Nat.cast_zero]

theorem mixed_word_branch_invariant (p : UInt64) [Fact p.toNat.Prime]
 (f : Polynomial (ZMod p.toNat)) (a : WordD1) (b : WeightedWord)
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31)
 (ha : a.canonical p) (hb : b.canonical p)
 (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
 (hva : a.normalizedValid p f) (hvb : b.normalizedValid p f) :
 mixedClassifyWord p a b ≠ .mismatch ∧
 (mixedClassifyWord p a b = .cancellation →
   let c := (mixedWords p a b).point
   c.canonical p ∧ c.z ≠ 0 ∧ c.w ≠ 0 ∧ c.normalizedValid p f) := by
 have hp : 0<p.toNat := by omega
 have az : (a.decode p).z ≠ 0 := canonical_word_nonzero p a.z hp ha.2.2.1 haz
 have aw : (a.decode p).w ≠ 0 := canonical_word_nonzero p a.w hp ha.2.2.2 haw
 have bz : (b.decodeTwo p).z ≠ 0 := canonical_word_nonzero p b.z hp hb.2.2.2.2.1 hbz
 have bw : (b.decodeTwo p).w ≠ 0 := canonical_word_nonzero p b.w hp hb.2.2.2.2.2 hbw
 have va : MixedCancellation.validPoint f (a.decode p) := hva
 have vb : MixedCancellation.validTwo f (b.decodeTwo p) := hvb
 have hd := mixed_classifier_decode p hfive hbound a b ha hb
 constructor
 · rw [hd]
   exact MixedCancellation.mismatch_excluded f (a.decode p) (b.decodeTwo p) az aw bz bw va vb
 · intro hcancel
   rw [hd] at hcancel
   have hk : MixedCancellation.rawK (a.decode p) (b.decodeTwo p)=0 := by
    by_contra hn
    simp [MixedCancellation.classify,hn] at hcancel
   obtain ⟨_,hz,hw,hvalid⟩ := MixedCancellation.shared_support_conclusions f (a.decode p) (b.decodeTwo p) az aw bz bw va vb hk
   obtain ⟨_,_,_,hc,_,_,_,hpoint⟩ := mixed_decode p hfive hbound a b ha hb
   have hcz : ((mixedWords p a b).point.decode p).z ≠ 0 := by rw [hpoint]; exact hz
   have hcw : ((mixedWords p a b).point.decode p).w ≠ 0 := by rw [hpoint]; exact hw
   refine ⟨hc,?_,?_,?_⟩
   · intro he
     apply hcz
     change (((mixedWords p a b).point.z).toNat:ZMod p.toNat)=0
     rw [he]; simp
   · intro he
     apply hcw
     change (((mixedWords p a b).point.w).toNat:ZMod p.toNat)=0
     rw [he]; simp
   · change MixedCancellation.validPoint f ((mixedWords p a b).point.decode p)
     rw [hpoint]
     exact hvalid

end PencilWordRefinement
