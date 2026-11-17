import WordMixed
import MixedGeneric
set_option autoImplicit false
set_option maxRecDepth 6000
set_option maxHeartbeats 1200000
namespace PencilWordRefinement
structure GenericExpr where
 k : FieldExpr
 payload : PayloadExpr
structure GenericWords where
 k : UInt64
 payload : WeightedWord

def genericExpr (a : WordD1) (b : WeightedWord) (a3 : UInt64) : GenericExpr :=
 let z2 := (FieldExpr.mul (.leaf b.z) (.leaf b.z))
 let z4 := (FieldExpr.mul z2 z2)
 let z5 := (FieldExpr.mul z4 (.leaf b.z))
 let az2 := (FieldExpr.mul (.leaf a.z) (.leaf a.z))
 let az4 := (FieldExpr.mul az2 az2)
 let az3 := (FieldExpr.mul az2 (.leaf a.z))
 let az5 := (FieldExpr.mul az4 (.leaf a.z))
 let k := (FieldExpr.mul (FieldExpr.add (FieldExpr.add (FieldExpr.mul (.leaf b.q) (FieldExpr.mul z2 (FieldExpr.mul (.leaf a.x) az2))) (FieldExpr.mul (FieldExpr.mul (.leaf a.x) (.leaf a.x)) z4)) (FieldExpr.mul (.leaf b.r) az4)) (.leaf a.w))
 let aa := (FieldExpr.sub (FieldExpr.mul (.leaf a.y) (FieldExpr.mul z5 (.leaf b.w))) (FieldExpr.mul (FieldExpr.add (FieldExpr.mul (.leaf b.s) (FieldExpr.mul az3 (FieldExpr.mul (.leaf a.x) z2))) (FieldExpr.mul (.leaf b.t) az5)) (.leaf a.w)))
 let bb := (FieldExpr.add (FieldExpr.mul (.leaf b.s) (FieldExpr.mul k (.leaf a.z))) (FieldExpr.mul aa (.leaf b.q)))
 let cc := (FieldExpr.add (FieldExpr.mul (.leaf b.t) (FieldExpr.mul k (.leaf a.z))) (FieldExpr.mul aa (.leaf b.r)))
 let ll := (FieldExpr.mul k (FieldExpr.mul (.leaf a.z) (.leaf b.w)))
 let mm := (FieldExpr.mul k (FieldExpr.mul (.leaf b.z) (.leaf b.w)))
 let ll2 := (FieldExpr.mul ll ll)
 let ll4 := (FieldExpr.mul ll2 ll2)
 let mm2 := (FieldExpr.mul mm mm)
 let q3 := (FieldExpr.sub (FieldExpr.sub (FieldExpr.mul (.leaf a.x) mm2) (FieldExpr.mul (.leaf b.q) ll2)) (FieldExpr.mul aa aa))
 let r3 := (FieldExpr.add (FieldExpr.sub (FieldExpr.mul (FieldExpr.add (FieldExpr.sub (FieldExpr.mul (.leaf a3) z4) (.leaf b.r)) (FieldExpr.mul (.leaf b.q) (.leaf b.q))) ll4) (FieldExpr.mul aa (FieldExpr.mul (FieldExpr.add bb (FieldExpr.mul (.leaf b.s) (FieldExpr.mul k (.leaf a.z)))) ll2))) (FieldExpr.mul q3 (FieldExpr.mul (.leaf a.x) mm2)))
 let s3 := (FieldExpr.sub (FieldExpr.mul aa q3) (FieldExpr.mul bb ll2))
 let t3 := (FieldExpr.sub (FieldExpr.mul aa r3) (FieldExpr.mul cc ll4))
 let z3 := (FieldExpr.mul k (FieldExpr.mul (.leaf b.z) (FieldExpr.mul (.leaf a.z) (.leaf b.w))))
 ⟨k,⟨q3,r3,s3,t3,z3,(.leaf 1)⟩⟩

def genericWords (p : UInt64) (a : WordD1) (b : WeightedWord) (a3 : UInt64) : GenericWords :=
 let z2 := ((b.z*b.z)%p)
 let z4 := ((z2*z2)%p)
 let z5 := ((z4*b.z)%p)
 let az2 := ((a.z*a.z)%p)
 let az4 := ((az2*az2)%p)
 let az3 := ((az2*a.z)%p)
 let az5 := ((az4*a.z)%p)
 let k := (((wordAdd p (wordAdd p ((b.q*((z2*((a.x*az2)%p))%p))%p) ((((a.x*a.x)%p)*z4)%p)) ((b.r*az4)%p))*a.w)%p)
 let aa := (wordSub p ((a.y*((z5*b.w)%p))%p) (((wordAdd p ((b.s*((az3*((a.x*z2)%p))%p))%p) ((b.t*az5)%p))*a.w)%p))
 let bb := (wordAdd p ((b.s*((k*a.z)%p))%p) ((aa*b.q)%p))
 let cc := (wordAdd p ((b.t*((k*a.z)%p))%p) ((aa*b.r)%p))
 let ll := ((k*((a.z*b.w)%p))%p)
 let mm := ((k*((b.z*b.w)%p))%p)
 let ll2 := ((ll*ll)%p)
 let ll4 := ((ll2*ll2)%p)
 let mm2 := ((mm*mm)%p)
 let q3 := (wordSub p (wordSub p ((a.x*mm2)%p) ((b.q*ll2)%p)) ((aa*aa)%p))
 let r3 := (wordAdd p (wordSub p (((wordAdd p (wordSub p ((a3*z4)%p) b.r) ((b.q*b.q)%p))*ll4)%p) ((aa*(((wordAdd p bb ((b.s*((k*a.z)%p))%p))*ll2)%p))%p)) ((q3*((a.x*mm2)%p))%p))
 let s3 := (wordSub p ((aa*q3)%p) ((bb*ll2)%p))
 let t3 := (wordSub p ((aa*r3)%p) ((cc*ll4)%p))
 let z3 := ((k*((b.z*((a.z*b.w)%p))%p))%p)
 ⟨k,⟨q3,r3,s3,t3,z3,1⟩⟩

theorem generic_word_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord) (a3 : UInt64)
 (ha : a.canonical p) (hc : b.canonical p) (h3 : a3.toNat<p.toNat) :
 let c := genericWords p a b a3
 c.k.toNat<p.toNat ∧ c.payload.canonical p ∧
 (c.k.toNat:ZMod p.toNat)=MixedGeneric.genericK (a.decode p) (b.decodeTwo p) ∧
 c.payload.decodeTwo p=MixedGeneric.output (a.decode p) (b.decodeTwo p) (a3.toNat:ZMod p.toNat) := by
 have good : (genericExpr a b a3).k.good p ∧ (genericExpr a b a3).payload.q.good p ∧
   (genericExpr a b a3).payload.r.good p ∧ (genericExpr a b a3).payload.s.good p ∧
   (genericExpr a b a3).payload.t.good p ∧ (genericExpr a b a3).payload.z.good p ∧
   (genericExpr a b a3).payload.w.good p := by
  rcases ha with ⟨hax,hay,haz,haw⟩
  rcases hc with ⟨hbq,hbr,hbs,hbt,hbz,hbw⟩
  have h1 : (1:UInt64).toNat<p.toNat := by change 1<p.toNat; omega
  simp [genericExpr,FieldExpr.good,hax,hay,haz,haw,hbq,hbr,hbs,hbt,hbz,hbw,h3,h1] <;> omega
 have hk := FieldExpr.composition p hfive hb _ good.1
 have hq := FieldExpr.composition p hfive hb _ good.2.1
 have hr := FieldExpr.composition p hfive hb _ good.2.2.1
 have hs := FieldExpr.composition p hfive hb _ good.2.2.2.1
 have ht := FieldExpr.composition p hfive hb _ good.2.2.2.2.1
 have hz := FieldExpr.composition p hfive hb _ good.2.2.2.2.2.1
 have hw := FieldExpr.composition p hfive hb _ good.2.2.2.2.2.2
 refine ⟨hk.1,⟨hq.1,hr.1,hs.1,ht.1,hz.1,hw.1⟩,?_,?_⟩
 · change (((genericExpr a b a3).k.word p).toNat:ZMod p.toNat)=_
   rw [hk.2]; rfl
 · change MixedCancellation.D2.mk
     (((genericExpr a b a3).payload.q.word p).toNat:ZMod p.toNat)
     (((genericExpr a b a3).payload.r.word p).toNat:ZMod p.toNat)
     (((genericExpr a b a3).payload.s.word p).toNat:ZMod p.toNat)
     (((genericExpr a b a3).payload.t.word p).toNat:ZMod p.toNat)
     (((genericExpr a b a3).payload.z.word p).toNat:ZMod p.toNat)
     (((genericExpr a b a3).payload.w.word p).toNat:ZMod p.toNat)=_
   rw [hq.2,hr.2,hs.2,ht.2,hz.2,hw.2]
   simp only [genericExpr,FieldExpr.field,UInt64.toNat_one,Nat.cast_one]
   rfl

def genericWordDispatch (p : UInt64) (a : WordD1) (b : WeightedWord) (a3 : UInt64) : Option WeightedWord :=
 let c := genericWords p a b a3
 if c.k=0 then none else some c.payload

theorem generic_guard_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord) (a3 : UInt64)
 (ha : a.canonical p) (hc : b.canonical p) (h3 : a3.toNat<p.toNat) :
 (genericWords p a b a3).k=0 ↔ MixedGeneric.genericK (a.decode p) (b.decodeTwo p)=0 := by
 obtain ⟨hk,hpayload,hdecode,hout⟩ := generic_word_decode p hfive hb a b a3 ha hc h3
 have hp : 0<p.toNat := by omega
 have he : (genericWords p a b a3).k=0 ↔
   ((genericWords p a b a3).k.toNat:ZMod p.toNat)=(0:ZMod p.toNat) := by
  rw [←UInt64.toNat_inj]
  simpa using (canonical_equality p.toNat (genericWords p a b a3).k.toNat 0 hk hp).symm
 simpa only [hdecode] using he

theorem generic_dispatch_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord) (a3 : UInt64)
 (ha : a.canonical p) (hc : b.canonical p) (h3 : a3.toNat<p.toNat) :
 (genericWordDispatch p a b a3).map (fun c => c.decodeTwo p)=
 MixedGeneric.dispatch (a.decode p) (b.decodeTwo p) (a3.toNat:ZMod p.toNat) := by
 have hg := generic_guard_decode p hfive hb a b a3 ha hc h3
 have hd := (generic_word_decode p hfive hb a b a3 ha hc h3).2.2.2
 simp only [genericWordDispatch,MixedGeneric.dispatch,hg]
 split <;> simp_all

theorem generic_outer_guard (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a : WordD1) (b : WeightedWord) (a3 : UInt64)
 (ha : a.canonical p) (hc : b.canonical p) (h3 : a3.toNat<p.toNat) :
 (genericWords p a b a3).k=(mixedWords p a b).k := by
 obtain ⟨hk,hpayload,hdecode,hout⟩ := generic_word_decode p hfive hb a b a3 ha hc h3
 obtain ⟨hmk,hl,hr,hpnt,hmd,hld,hrd,hpd⟩ := mixed_decode p hfive hb a b ha hc
 apply UInt64.toNat_inj.mp
 apply (canonical_equality p.toNat _ _ hk hmk).mp
 rw [hdecode,hmd,MixedGeneric.genericK_same]


-- Polynomial validity composition; source-fidelity review of MixedGeneric is separate.
theorem generic_word_preserves (p : UInt64) [Fact p.toNat.Prime]
 (a0 a1 a2 : ZMod p.toNat) (a3 : UInt64) (a : WordD1) (b : WeightedWord)
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31)
 (ha : a.canonical p) (hb : b.canonical p) (h3 : a3.toNat<p.toNat)
 (haz : a.z ≠ 0) (haw : a.w ≠ 0) (hbz : b.z ≠ 0) (hbw : b.w ≠ 0)
 (hva : a.normalizedValid p (MixedGeneric.curve a0 a1 a2 (a3.toNat:ZMod p.toNat)))
 (hvb : b.normalizedValid p (MixedGeneric.curve a0 a1 a2 (a3.toNat:ZMod p.toNat)))
 (hk : MixedGeneric.genericK (a.decode p) (b.decodeTwo p) ≠ 0) :
 let c := (genericWords p a b a3).payload
 genericWordDispatch p a b a3=some c ∧ c.canonical p ∧ c.z≠0 ∧ c.w≠0 ∧
 c.normalizedValid p (MixedGeneric.curve a0 a1 a2 (a3.toNat:ZMod p.toNat)) := by
 have hp : 0<p.toNat := by omega
 have az : (a.decode p).z ≠ 0 := canonical_word_nonzero p a.z hp ha.2.2.1 haz
 have aw : (a.decode p).w ≠ 0 := canonical_word_nonzero p a.w hp ha.2.2.2 haw
 have bz : (b.decodeTwo p).z ≠ 0 := canonical_word_nonzero p b.z hp hb.2.2.2.2.1 hbz
 have bw : (b.decodeTwo p).w ≠ 0 := canonical_word_nonzero p b.w hp hb.2.2.2.2.2 hbw
 obtain ⟨_,hc,_,hd⟩ := generic_word_decode p hfive hbound a b a3 ha hb h3
 obtain ⟨fz,fw,fv⟩ := MixedGeneric.weighted_generic_preserves a0 a1 a2
   (a3.toNat:ZMod p.toNat) (a.decode p) (b.decodeTwo p) az aw bz bw hk hva hvb
 have hkword : (genericWords p a b a3).k≠0 := by
  intro hz
  exact hk ((generic_guard_decode p hfive hbound a b a3 ha hb h3).mp hz)
 have hcz : ((genericWords p a b a3).payload.decodeTwo p).z≠0 := by rw [hd]; exact fz
 have hcw : ((genericWords p a b a3).payload.decodeTwo p).w≠0 := by rw [hd]; exact fw
 refine ⟨by simp [genericWordDispatch,hkword],hc,?_,?_,?_⟩
 · intro he
   apply hcz
   change (((genericWords p a b a3).payload.z).toNat:ZMod p.toNat)=0
   rw [he]; simp
 · intro he
   apply hcw
   change (((genericWords p a b a3).payload.w).toNat:ZMod p.toNat)=0
   rw [he]; simp
 · change MixedCancellation.validTwo (MixedGeneric.curve a0 a1 a2 (a3.toNat:ZMod p.toNat))
     ((genericWords p a b a3).payload.decodeTwo p)
   rw [hd]
   exact fv

end PencilWordRefinement
