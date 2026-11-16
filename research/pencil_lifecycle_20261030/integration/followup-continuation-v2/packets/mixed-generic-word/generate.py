from pathlib import Path
D=Path(__file__).parent
# Explicit AST mirrors source operations, including every multiplication nesting.
M=lambda a,b:('mul',a,b); A=lambda a,b:('add',a,b); S=lambda a,b:('sub',a,b)
seq=[('z2',M('b.z','b.z')),('z4',M('z2','z2')),('z5',M('z4','b.z')),('az2',M('a.z','a.z')),('az4',M('az2','az2')),('az3',M('az2','a.z')),('az5',M('az4','a.z')),('k',M(A(A(M('b.q',M('z2',M('a.x','az2'))),M(M('a.x','a.x'),'z4')),M('b.r','az4')),'a.w')),('aa',S(M('a.y',M('z5','b.w')),M(A(M('b.s',M('az3',M('a.x','z2'))),M('b.t','az5')),'a.w'))),('bb',A(M('b.s',M('k','a.z')),M('aa','b.q'))),('cc',A(M('b.t',M('k','a.z')),M('aa','b.r'))),('ll',M('k',M('a.z','b.w'))),('mm',M('k',M('b.z','b.w'))),('ll2',M('ll','ll')),('ll4',M('ll2','ll2')),('mm2',M('mm','mm')),('q3',S(S(M('a.x','mm2'),M('b.q','ll2')),M('aa','aa'))),('r3',A(S(M(A(S(M('a3','z4'),'b.r'),M('b.q','b.q')),'ll4'),M('aa',M(A('bb',M('b.s',M('k','a.z'))),'ll2'))),M('q3',M('a.x','mm2')))),('s3',S(M('aa','q3'),M('bb','ll2'))),('t3',S(M('aa','r3'),M('cc','ll4'))),('z3',M('k',M('b.z',M('a.z','b.w'))))]
def form(e,kind):
 if isinstance(e,str):return f'(.leaf {e})' if kind=='expr' and ('.' in e or e in ['a3','1']) else e
 op,a,b=e;x,y=form(a,kind),form(b,kind)
 if kind=='expr':return f'(FieldExpr.{op} {x} {y})'
 return f'(({x}*{y})%p)' if op=='mul' else f'(word{op.title()} p {x} {y})'
s='''import WordMixed
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
'''
for kind,rettype in [('expr','GenericExpr'),('word','GenericWords')]:
 name='genericExpr'if kind=='expr'else'genericWords'
 s+=f'\ndef {name} '+('(p : UInt64) 'if kind=='word'else'')+'(a : WordD1) (b : WeightedWord) (a3 : UInt64) : '+rettype+' :=\n'
 for n,e in seq:s+=f' let {n} := {form(e,kind)}\n'
 s+=' ⟨k,⟨q3,r3,s3,t3,z3,'+('(.leaf 1)'if kind=='expr'else'1')+'⟩⟩\n'
s+='''
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
  simp [genericExpr,FieldExpr.good,hax,hay,haz,haw,hbq,hbr,hbs,hbt,hbz,hbw,h3,h1]
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
   rw [hq.2,hr.2,hs.2,ht.2,hz.2,hw.2]; rfl
end PencilWordRefinement
'''
(D/'WordGeneric.lean').write_text(s)
