import WordPositive
import Tripling
set_option autoImplicit false
set_option maxRecDepth 12000
set_option maxHeartbeats 1800000
namespace PencilWordRefinement
def tripleExpr (p a1 a2 a3 : UInt64) (a : WordD1) : PayloadExpr :=
 let x2 := .mul (.leaf (a.x)) (.leaf (a.x))
 let x3 := .mul (x2) (.leaf (a.x))
 let x4 := .mul (x2) (x2)
 let z2 := .mul (.leaf (a.z)) (.leaf (a.z))
 let z4 := .mul (z2) (z2)
 let z6 := .mul (z4) (z2)
 let z8 := .mul (z4) (z4)
 let y2 := .mul (.leaf (a.y)) (.leaf (a.y))
 let fp := .mul (.add (.add (.add (.mul (.leaf (a1)) (z8)) (.mul (.leaf (2)) (.mul (.leaf (a2)) (.mul (.leaf (a.x)) (z6))))) (.mul (.leaf (3)) (.mul (.leaf (a3)) (.mul (x2) (z4))))) (.mul (.leaf (5%p)) (x4))) (.leaf (a.w))
 let fpp := .mul (.leaf (2)) (.add (.add (.mul (.leaf (a2)) (z6)) (.mul (.leaf (3)) (.mul (.leaf (a3)) (.mul (.leaf (a.x)) (z4))))) (.mul (.leaf (10%p)) (x3)))
 let k := .mul (.leaf (8%p)) (.mul (y2) (.leaf (a.y)))
 let aa := .mul (.sub (.mul (.leaf (2)) (.mul (fpp) (y2))) (.mul (fp) (fp))) (.leaf (a.w))
 let bb := .sub (.mul (.leaf (2)) (.mul (fp) (y2))) (.mul (aa) (.leaf (a.x)))
 let cc := .sub (.mul (.leaf (a.y)) (k)) (.mul (.add (.mul (aa) (.leaf (a.x))) (.mul (.leaf (2)) (bb))) (.mul (.leaf (a.x)) (.leaf (a.w))))
 let k2 := .mul (k) (k)
 let k4 := .mul (k2) (k2)
 let z3 := .mul (k) (.leaf (a.z))
 let q3 := .sub (.mul (.leaf (3)) (.mul (.leaf (a.x)) (k2))) (.mul (aa) (aa))
 let r3 := .sub (.mul (.leaf (a3)) (.mul (.mul (z3) (z3)) (.mul (z3) (z3)))) (.mul (.sub (.mul (.leaf (4)) (.mul (aa) (bb))) (.mul (.leaf (3)) (.mul (.leaf (a.x)) (.sub (q3) (.mul (.leaf (a.x)) (k2)))))) (k2))
 let s3 := .sub (.mul (aa) (.mul (q3) (.leaf (a.w)))) (.mul (.leaf (2)) (.mul (bb) (.mul (k2) (.leaf (a.w)))))
 let t3 := .sub (.mul (aa) (.mul (r3) (.leaf (a.w)))) (.mul (cc) (k4))
 ⟨q3,r3,s3,t3,z3,.leaf a.w⟩
def tripleWords (p a1 a2 a3 : UInt64) (a : WordD1) : WeightedWord :=
 let x2 := ((a.x)*(a.x))%p
 let x3 := ((x2)*(a.x))%p
 let x4 := ((x2)*(x2))%p
 let z2 := ((a.z)*(a.z))%p
 let z4 := ((z2)*(z2))%p
 let z6 := ((z4)*(z2))%p
 let z8 := ((z4)*(z4))%p
 let y2 := ((a.y)*(a.y))%p
 let fp := ((wordAdd p (wordAdd p (wordAdd p (((a1)*(z8))%p) (((2)*(((a2)*(((a.x)*(z6))%p))%p))%p)) (((3)*(((a3)*(((x2)*(z4))%p))%p))%p)) (((5)*(x4))%p))*(a.w))%p
 let fpp := ((2)*(wordAdd p (wordAdd p (((a2)*(z6))%p) (((3)*(((a3)*(((a.x)*(z4))%p))%p))%p)) (((10)*(x3))%p)))%p
 let k := ((8)*(((y2)*(a.y))%p))%p
 let aa := ((wordSub p (((2)*(((fpp)*(y2))%p))%p) (((fp)*(fp))%p))*(a.w))%p
 let bb := wordSub p (((2)*(((fp)*(y2))%p))%p) (((aa)*(a.x))%p)
 let cc := wordSub p (((a.y)*(k))%p) (((wordAdd p (((aa)*(a.x))%p) (((2)*(bb))%p))*(((a.x)*(a.w))%p))%p)
 let k2 := ((k)*(k))%p
 let k4 := ((k2)*(k2))%p
 let z3 := ((k)*(a.z))%p
 let q3 := wordSub p (((3)*(((a.x)*(k2))%p))%p) (((aa)*(aa))%p)
 let r3 := wordSub p (((a3)*(((((z3)*(z3))%p)*(((z3)*(z3))%p))%p))%p) (((wordSub p (((4)*(((aa)*(bb))%p))%p) (((3)*(((a.x)*(wordSub p (q3) (((a.x)*(k2))%p)))%p))%p))*(k2))%p)
 let s3 := wordSub p (((aa)*(((q3)*(a.w))%p))%p) (((2)*(((bb)*(((k2)*(a.w))%p))%p))%p)
 let t3 := wordSub p (((aa)*(((r3)*(a.w))%p))%p) (((cc)*(k4))%p)
 ⟨q3,r3,s3,t3,z3,a.w⟩

theorem small_literal_mul_reduce (p c a : UInt64) (hp : 0<p.toNat) (hb : p.toNat<2^31)
 (hc : c.toNat≤10) (ha : a.toNat<p.toNat) : (c*a)%p=((c%p)*a)%p := by
 have hh : c.toNat*a.toNat<2^64 := by
  have hm := Nat.mul_le_mul_right a.toNat hc
  omega
 have hr : (c%p).toNat<p.toNat := by rw [UInt64.toNat_mod];exact Nat.mod_lt _ hp
 apply UInt64.toNat_inj.mp
 rw [UInt64.toNat_mod,UInt64.toNat_mul,Nat.mod_eq_of_lt hh]
 rw [uint64_mul_mod p (c%p) a hb hr ha]
 simp only [UInt64.toNat_mod]
 simp [Nat.mul_mod]

theorem triple_literal_tree (p a1 a2 a3 : UInt64) (a : WordD1)
 (hp : 0<p.toNat) (hb : p.toNat<2^31) :
 tripleWords p a1 a2 a3 a =
 ⟨(tripleExpr p a1 a2 a3 a).q.word p,(tripleExpr p a1 a2 a3 a).r.word p,
 (tripleExpr p a1 a2 a3 a).s.word p,(tripleExpr p a1 a2 a3 a).t.word p,
 (tripleExpr p a1 a2 a3 a).z.word p,(tripleExpr p a1 a2 a3 a).w.word p⟩ := by
 have hc (x : UInt64) : (x%p).toNat<p.toNat := by rw [UInt64.toNat_mod];exact Nat.mod_lt _ hp
 have h5 (x : UInt64) : (5*(x%p))%p=((5%p)*(x%p))%p := small_literal_mul_reduce p 5 (x%p) hp hb (by decide) (hc x)
 have h8 (x : UInt64) : (8*(x%p))%p=((8%p)*(x%p))%p := small_literal_mul_reduce p 8 (x%p) hp hb (by decide) (hc x)
 have h10 (x : UInt64) : (10*(x%p))%p=((10%p)*(x%p))%p := small_literal_mul_reduce p 10 (x%p) hp hb (by decide) (hc x)
 simp only [tripleWords,tripleExpr,FieldExpr.word,h5,h8,h10]

theorem triple_word_decode (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hb : p.toNat<2^31) (a1 a2 a3 : UInt64) (a : WordD1)
 (h1 : a1.toNat<p.toNat) (h2 : a2.toNat<p.toNat) (h3 : a3.toNat<p.toNat) (ha : a.canonical p) :
 (tripleWords p a1 a2 a3 a).canonical p ∧
 (tripleWords p a1 a2 a3 a).decodeTwo p=MixedTripling.rawOutput
 (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat) (a.decode p) := by
 have hp : 0<p.toNat := by omega
 have hc (c : UInt64) : (c%p).toNat<p.toNat := by rw [UInt64.toNat_mod];exact Nat.mod_lt _ hp
 have htwo : (2:UInt64).toNat<p.toNat := by change 2<p.toNat;omega
 have hthree : (3:UInt64).toNat<p.toNat := by change 3<p.toNat;omega
 have hfour : (4:UInt64).toNat<p.toNat := by change 4<p.toNat;omega
 have good : (tripleExpr p a1 a2 a3 a).q.good p ∧ (tripleExpr p a1 a2 a3 a).r.good p ∧
  (tripleExpr p a1 a2 a3 a).s.good p ∧ (tripleExpr p a1 a2 a3 a).t.good p ∧
  (tripleExpr p a1 a2 a3 a).z.good p ∧ (tripleExpr p a1 a2 a3 a).w.good p := by
  rcases ha with ⟨hx,hy,hz,hw⟩
  simp only [tripleExpr,FieldExpr.good,h1,h2,h3,hx,hy,hz,hw,htwo,hthree,hfour,hc,and_self]
 have q := FieldExpr.composition p hfive hb _ good.1
 have r := FieldExpr.composition p hfive hb _ good.2.1
 have s := FieldExpr.composition p hfive hb _ good.2.2.1
 have t := FieldExpr.composition p hfive hb _ good.2.2.2.1
 have z := FieldExpr.composition p hfive hb _ good.2.2.2.2.1
 have w := FieldExpr.composition p hfive hb _ good.2.2.2.2.2
 rw [triple_literal_tree p a1 a2 a3 a hp hb]
 refine ⟨⟨q.1,r.1,s.1,t.1,z.1,w.1⟩,?_⟩
 unfold WeightedWord.decodeTwo
 rw [q.2,r.2,s.2,t.2,z.2,w.2]
 simp only [tripleExpr,FieldExpr.field,UInt64.toNat_mod,ZMod.natCast_mod]
 rfl
def tripleWordDispatch (p a1 a2 a3 : UInt64) (a : WordD1) (b : WeightedWord) : Option WeightedWord :=
 if mixedClassifyWord p a b=.positiveCoincidence then
  if positiveXz p a b=(mixedWords p a b).point.x then some (tripleWords p a1 a2 a3 a) else none
 else none

attribute [local irreducible] tripleWords mixedWords positiveXz

theorem repeated_word_preserves (p : UInt64) [Fact p.toNat.Prime]
 (hfive : 5≤p.toNat) (hbound : p.toNat<2^31) (a0 a1 a2 a3 : UInt64) (a : WordD1) (b : WeightedWord)
 (h1 : a1.toNat<p.toNat) (h2 : a2.toNat<p.toNat) (h3 : a3.toNat<p.toNat)
 (ha : a.canonical p) (hb : b.canonical p)
 (haz : a.z≠0) (haw : a.w≠0) (hbz : b.z≠0) (hbw : b.w≠0)
 (hva : a.normalizedValid p (MixedGeneric.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)))
 (hvb : b.normalizedValid p (MixedGeneric.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)))
 (hk : (mixedWords p a b).k=0)
 (hc : (mixedWords p a b).l≠wordNeg p (mixedWords p a b).ry)
 (he : positiveXz p a b=(mixedWords p a b).point.x) :
 let c := tripleWords p a1 a2 a3 a
 a.y≠0 ∧ tripleWordDispatch p a1 a2 a3 a b=some c ∧ c.canonical p ∧ c.z≠0 ∧ c.w≠0 ∧
 c.normalizedValid p (MixedGeneric.curve (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)) := by
 have hp : 0<p.toNat := by omega
 obtain ⟨hmk,hml,hmr,hmc,hmkd,hmld,hmrd,hmpd⟩ := mixed_decode p hfive hbound a b ha hb
 have fk : MixedCancellation.rawK (a.decode p) (b.decodeTwo p)=0 := by rw [←hmkd,hk];simp
 have hn := wordNeg_composition p (mixedWords p a b).ry hp hmr
 have fc : (a.decode p).y*MixedCancellation.rawW3 (b.decodeTwo p) ≠ -(MixedCancellation.rawY3 (a.decode p) (b.decodeTwo p)*(a.decode p).w) := by
  intro heq;apply hc;apply UInt64.toNat_inj.mp
  apply (canonical_equality p.toNat _ _ hml hn.1).mp
  rw [hmld,hn.2,hmrd];exact heq
 have hxd := (positive_xz_decode p hfive hbound a b ha hb).2
 have fe : (a.decode p).x*((b.decodeTwo p).z*(b.decodeTwo p).z)=(MixedCancellation.other (a.decode p) (b.decodeTwo p)).x := by
  rw [←hxd,he]
  exact congrArg WeightedAdd11.Input.x hmpd
 have az := canonical_word_nonzero p a.z hp ha.2.2.1 haz
 have aw := canonical_word_nonzero p a.w hp ha.2.2.2 haw
 have bz := canonical_word_nonzero p b.z hp hb.2.2.2.2.1 hbz
 have bw := canonical_word_nonzero p b.w hp hb.2.2.2.2.2 hbw
 obtain ⟨fclass,_,fy,fz,fw,fv⟩ := MixedTripling.source_repeated_preserves
  (a0.toNat:ZMod p.toNat) (a1.toNat:ZMod p.toNat) (a2.toNat:ZMod p.toNat) (a3.toNat:ZMod p.toNat)
  (a.decode p) (b.decodeTwo p) az aw bz bw hva hvb fk fc fe
 have cls : mixedClassifyWord p a b=.positiveCoincidence := by rw [mixed_classifier_decode p hfive hbound a b ha hb];exact fclass
 obtain ⟨cc,cdec⟩ := triple_word_decode p hfive hbound a1 a2 a3 a h1 h2 h3 ha
 have ay : a.y≠0 := by intro hz;apply fy;change (a.y.toNat:ZMod p.toNat)=0;rw [hz];simp
 refine ⟨ay,by simp only [tripleWordDispatch,cls,he,ite_true],cc,?_,?_,positive_valid_transport p _ _ _ cdec fv⟩
 · have hz : ((tripleWords p a1 a2 a3 a).decodeTwo p).z≠0 := by rw [cdec];exact fz
   intro heq;apply hz;change ((tripleWords p a1 a2 a3 a).z.toNat:ZMod p.toNat)=0;rw [heq];simp
 · have hw : ((tripleWords p a1 a2 a3 a).decodeTwo p).w≠0 := by rw [cdec];exact fw
   intro heq;apply hw;change ((tripleWords p a1 a2 a3 a).w.toNat:ZMod p.toNat)=0;rw [heq];simp
end PencilWordRefinement
