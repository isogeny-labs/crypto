import WordPositive
import SameUComposition
set_option autoImplicit false
set_option maxRecDepth 6000
set_option maxHeartbeats 1600000
namespace PencilWordRefinement
structure CrossTrees where
 q : FieldExpr
 r : FieldExpr
 s : FieldExpr
 t : FieldExpr
structure CrossWords where
 q : UInt64
 r : UInt64
 s : UInt64
 t : UInt64

def crossTrees (a b : WeightedWord) : CrossTrees :=
 let z2 := FieldExpr.mul (.leaf b.z) (.leaf b.z)
 let z3 := FieldExpr.mul z2 (.leaf b.z)
 let z4 := FieldExpr.mul z2 z2
 let z5 := FieldExpr.mul z4 (.leaf b.z)
 ⟨.mul (.leaf a.q) z2,.mul (.leaf a.r) z4,
 .mul (.mul (.leaf a.s) z3) (.leaf b.w),.mul (.mul (.leaf a.t) z5) (.leaf b.w)⟩
def crossWords (p : UInt64) (a b : WeightedWord) : CrossWords :=
 let z2 := (b.z*b.z)%p
 let z3 := (z2*b.z)%p
 let z4 := (z2*z2)%p
 let z5 := (z4*b.z)%p
 ⟨(a.q*z2)%p,(a.r*z4)%p,(((a.s*z3)%p)*b.w)%p,(((a.t*z5)%p)*b.w)%p⟩

theorem cross_word_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31) (a b : WeightedWord)
 (ha : a.canonical p) (hb : b.canonical p) :
 let c := crossWords p a b
 c.q.toNat<p.toNat ∧ c.r.toNat<p.toNat ∧ c.s.toNat<p.toNat ∧ c.t.toNat<p.toNat ∧
 (c.q.toNat:ZMod p.toNat)=SameUZeroSlope.crossQ (a.decodeTwo p) (b.decodeTwo p) ∧
 (c.r.toNat:ZMod p.toNat)=SameUZeroSlope.crossR (a.decodeTwo p) (b.decodeTwo p) ∧
 (c.s.toNat:ZMod p.toNat)=SameUZeroSlope.crossS (a.decodeTwo p) (b.decodeTwo p) ∧
 (c.t.toNat:ZMod p.toNat)=SameUZeroSlope.crossT (a.decodeTwo p) (b.decodeTwo p) := by
 have hg : (crossTrees a b).q.good p ∧ (crossTrees a b).r.good p ∧
  (crossTrees a b).s.good p ∧ (crossTrees a b).t.good p := by
  rcases ha with ⟨aq,ar,ass,att,az,aw⟩
  rcases hb with ⟨bq,br,bs,bt,bz,bw⟩
  simp [crossTrees,FieldExpr.good,aq,ar,ass,att,bz,bw]
 have q := FieldExpr.composition p hfive hbound _ hg.1
 have r := FieldExpr.composition p hfive hbound _ hg.2.1
 have s := FieldExpr.composition p hfive hbound _ hg.2.2.1
 have t := FieldExpr.composition p hfive hbound _ hg.2.2.2
 refine ⟨q.1,r.1,s.1,t.1,?_,?_,?_,?_⟩
 · change (((crossTrees a b).q.word p).toNat:ZMod p.toNat)=_
   rw [q.2];rfl
 · change (((crossTrees a b).r.word p).toNat:ZMod p.toNat)=_
   rw [r.2];rfl
 · change (((crossTrees a b).s.word p).toNat:ZMod p.toNat)=_
   rw [s.2];rfl
 · change (((crossTrees a b).t.word p).toNat:ZMod p.toNat)=_
   rw [t.2];rfl

structure SamePointTrees where
 k : FieldExpr
 x : FieldExpr
 y : FieldExpr
 z : FieldExpr
 w : FieldExpr
structure SamePointWords where
 k : UInt64
 point : WordD1

def samePointTrees (a b : WeightedWord) : SamePointTrees :=
 let ca := crossTrees a b
 let cb := crossTrees b a
 let k := FieldExpr.sub ca.s cb.s
 let x := FieldExpr.mul (.sub cb.t ca.t) k
 let k2 := FieldExpr.mul k k
 let k3 := FieldExpr.mul k2 k
 let z22 := FieldExpr.mul (.leaf b.z) (.leaf b.z)
 let z23 := FieldExpr.mul z22 (.leaf b.z)
 let y := FieldExpr.mul (.mul (.add (.mul (.leaf a.s) x)
   (.mul (.leaf a.t) (.mul z22 k2))) k3) z23
 ⟨k,x,y,.mul k (.mul (.leaf a.z) (.leaf b.z)),.leaf a.w⟩
def samePointWords (p : UInt64) (a b : WeightedWord) : SamePointWords :=
 let ca := crossWords p a b
 let cb := crossWords p b a
 let k := wordSub p ca.s cb.s
 let x := ((wordSub p cb.t ca.t)*k)%p
 let k2 := (k*k)%p
 let k3 := (k2*k)%p
 let z22 := (b.z*b.z)%p
 let z23 := (z22*b.z)%p
 let y := ((((wordAdd p ((a.s*x)%p) ((a.t*((z22*k2)%p))%p))*k3)%p)*z23)%p
 ⟨k,⟨x,y,(k*((a.z*b.z)%p))%p,a.w⟩⟩

theorem same_point_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31) (a b : WeightedWord)
 (ha : a.canonical p) (hb : b.canonical p) :
 let c := samePointWords p a b
 c.k.toNat<p.toNat ∧ c.point.canonical p ∧
 (c.k.toNat:ZMod p.toNat)=SameUMixed.sourceK (a.decodeTwo p) (b.decodeTwo p) ∧
 c.point.decode p=SameUMixed.sourcePoint (a.decodeTwo p) (b.decodeTwo p) := by
 have hg : (samePointTrees a b).k.good p ∧ (samePointTrees a b).x.good p ∧
 (samePointTrees a b).y.good p ∧ (samePointTrees a b).z.good p ∧ (samePointTrees a b).w.good p := by
  rcases ha with ⟨aq,ar,ass,att,az,aw⟩
  rcases hb with ⟨bq,br,bs,bt,bz,bw⟩
  simp [samePointTrees,crossTrees,FieldExpr.good,ass,att,az,aw,bs,bt,bz,bw]
 have k := FieldExpr.composition p hfive hbound _ hg.1
 have x := FieldExpr.composition p hfive hbound _ hg.2.1
 have y := FieldExpr.composition p hfive hbound _ hg.2.2.1
 have z := FieldExpr.composition p hfive hbound _ hg.2.2.2.1
 have w := FieldExpr.composition p hfive hbound _ hg.2.2.2.2
 refine ⟨k.1,⟨x.1,y.1,z.1,w.1⟩,?_,?_⟩
 · change (((samePointTrees a b).k.word p).toNat:ZMod p.toNat)=_
   rw [k.2];rfl
 · change WeightedAdd11.Input.mk
    (((samePointTrees a b).x.word p).toNat:ZMod p.toNat)
    (((samePointTrees a b).y.word p).toNat:ZMod p.toNat)
    (((samePointTrees a b).z.word p).toNat:ZMod p.toNat)
    (((samePointTrees a b).w.word p).toNat:ZMod p.toNat)=_
   rw [x.2,y.2,z.2,w.2];rfl


def sameUWordDispatch (p a0 a1 a2 a3 : UInt64) (a b : WeightedWord) : Option WeightedWord :=
 let ca := crossWords p a b
 let cb := crossWords p b a
 if ca.q=cb.q ∧ ca.r=cb.r then
  if ca.s=wordNeg p cb.s ∧ ca.t=wordNeg p cb.t then none
  else if ca.s=cb.s ∧ ca.t=cb.t then none
  else if (samePointWords p a b).k=0 then none
  else some (dblWordPayload p a0 a1 a2 a3 (samePointWords p a b).point)
 else none

attribute [local irreducible] crossWords samePointWords dblWordPayload

theorem same_u_word_preserves (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31) (a0 a1 a2 a3 : UInt64) (a b : WeightedWord)
 (h0 : a0.toNat<p.toNat) (h1 : a1.toNat<p.toNat) (h2 : a2.toNat<p.toNat) (h3 : a3.toNat<p.toNat)
 (ha : a.canonical p) (hb : b.canonical p)
 (haz : a.z≠0) (haw : a.w≠0) (hbz : b.z≠0) (hbw : b.w≠0)
 (hva : a.normalizedValid p (WeightedDoubling.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)))
 (hvb : b.normalizedValid p (WeightedDoubling.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)))
 (hq : (crossWords p a b).q=(crossWords p b a).q)
 (hr : (crossWords p a b).r=(crossWords p b a).r)
 (hni : ¬((crossWords p a b).s=wordNeg p (crossWords p b a).s ∧
          (crossWords p a b).t=wordNeg p (crossWords p b a).t))
 (hne : ¬((crossWords p a b).s=(crossWords p b a).s ∧ (crossWords p a b).t=(crossWords p b a).t))
 (hg : IsCoprime (WeightedDoubling.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat))
  (WeightedDoubling.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)).derivative) :
 let d := (samePointWords p a b).point
 let c := dblWordPayload p a0 a1 a2 a3 d
 (samePointWords p a b).k≠0 ∧ d.canonical p ∧ d.y≠0 ∧
 sameUWordDispatch p a0 a1 a2 a3 a b=some c ∧ c.canonical p ∧ c.z≠0 ∧ c.w≠0 ∧
 c.normalizedValid p (WeightedDoubling.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)) := by
 have hp : 0<p.toNat := by omega
 obtain ⟨aqc,arc,asc,atc,aqd,ard,asd,atd⟩ := cross_word_decode p hfive hbound a b ha hb
 obtain ⟨bqc,brc,bsc,btc,bqd,brd,bsd,btd⟩ := cross_word_decode p hfive hbound b a hb ha
 obtain ⟨kc,pc,kd,pd⟩ := same_point_decode p hfive hbound a b ha hb
 have eqw (u v : UInt64) (hu : u.toNat<p.toNat) (hv : v.toNat<p.toNat) :
  u=v ↔ (u.toNat:ZMod p.toNat)=(v.toNat:ZMod p.toNat) := by
  rw [←UInt64.toNat_inj]
  exact (canonical_equality p.toNat u.toNat v.toNat hu hv).symm
 have fq : SameUZeroSlope.crossQ (a.decodeTwo p) (b.decodeTwo p)=SameUZeroSlope.crossQ (b.decodeTwo p) (a.decodeTwo p) := by
  rw [←aqd,←bqd,hq]
 have fr : SameUZeroSlope.crossR (a.decodeTwo p) (b.decodeTwo p)=SameUZeroSlope.crossR (b.decodeTwo p) (a.decodeTwo p) := by
  rw [←ard,←brd,hr]
 have ns := wordNeg_composition p (crossWords p b a).s hp bsc
 have nt := wordNeg_composition p (crossWords p b a).t hp btc
 have fni : ¬(SameUZeroSlope.crossS (a.decodeTwo p) (b.decodeTwo p)= -SameUZeroSlope.crossS (b.decodeTwo p) (a.decodeTwo p) ∧
              SameUZeroSlope.crossT (a.decodeTwo p) (b.decodeTwo p)= -SameUZeroSlope.crossT (b.decodeTwo p) (a.decodeTwo p)) := by
  rintro ⟨hs,ht⟩;apply hni
  constructor
  · apply (eqw _ _ asc ns.1).mpr
    rw [asd,ns.2,bsd];exact hs
  · apply (eqw _ _ atc nt.1).mpr
    rw [atd,nt.2,btd];exact ht
 have fne : ¬(SameUZeroSlope.crossS (a.decodeTwo p) (b.decodeTwo p)=SameUZeroSlope.crossS (b.decodeTwo p) (a.decodeTwo p) ∧
              SameUZeroSlope.crossT (a.decodeTwo p) (b.decodeTwo p)=SameUZeroSlope.crossT (b.decodeTwo p) (a.decodeTwo p)) := by
  rintro ⟨hs,ht⟩;apply hne
  constructor
  · apply (eqw _ _ asc bsc).mpr;rw [asd,bsd];exact hs
  · apply (eqw _ _ atc btc).mpr;rw [atd,btd];exact ht
 have az := canonical_word_nonzero p a.z hp ha.2.2.2.2.1 haz
 have aw := canonical_word_nonzero p a.w hp ha.2.2.2.2.2 haw
 have bz := canonical_word_nonzero p b.z hp hb.2.2.2.2.1 hbz
 have bw := canonical_word_nonzero p b.w hp hb.2.2.2.2.2 hbw
 have two : (2:ZMod p.toNat)≠0 := by
  have tt := canonical_word_nonzero p (2:UInt64) hp (by change 2<p.toNat;omega) (by decide)
  simpa using tt
 obtain ⟨fk,_,pz,pw,py,pv,oz,ow,ov⟩ := SameUComposition.source_same_u_mixed
  (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)
  (a.decodeTwo p) (b.decodeTwo p) az aw bz bw hva hvb fq fr fni fne hg two
 have kw : (samePointWords p a b).k≠0 := by
  intro hz;apply fk;rw [←kd,hz];simp
 have yw : (samePointWords p a b).point.y≠0 := by
  intro hy;apply py;rw [←pd]
  change (((samePointWords p a b).point.y.toNat:ZMod p.toNat))=0
  rw [hy];simp
 obtain ⟨cc,cd⟩ := doubled_decode p hfive hbound a0 a1 a2 a3 (samePointWords p a b).point h0 h1 h2 h3 pc
 rw [pd] at cd
 refine ⟨kw,pc,yw,?_,cc,?_,?_,?_⟩
 · simp [sameUWordDispatch,hq,hr,hni,hne,kw]
 · intro hz;apply oz;rw [←cd]
   change (((dblWordPayload p a0 a1 a2 a3 (samePointWords p a b).point).z.toNat:ZMod p.toNat))=0
   rw [hz];simp
 · intro hw;apply ow;rw [←cd]
   change (((dblWordPayload p a0 a1 a2 a3 (samePointWords p a b).point).w.toNat:ZMod p.toNat))=0
   rw [hw];simp
 · exact positive_valid_transport p _ _ _ cd ov

end PencilWordRefinement
