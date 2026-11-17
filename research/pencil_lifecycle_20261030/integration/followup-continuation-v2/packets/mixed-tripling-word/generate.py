from pathlib import Path
D=Path(__file__).parent
# Literal source let tree; operation nodes retain Rust parenthesization.
L=lambda x:('leaf',x)
M=lambda a,b:('mul',a,b)
A=lambda a,b:('add',a,b)
S=lambda a,b:('sub',a,b)
v=[]
def put(n,e):v.append((n,e));return L(n)
x,y,z,w=[L('a.'+k)for k in ['x','y','z','w']]
x2=put('x2',M(x,x));x3=put('x3',M(x2,x));x4=put('x4',M(x2,x2));z2=put('z2',M(z,z));z4=put('z4',M(z2,z2));z6=put('z6',M(z4,z2));z8=put('z8',M(z4,z4));y2=put('y2',M(y,y))
fp=put('fp',M(A(A(A(M(L('a1'),z8),M(L('2'),M(L('a2'),M(x,z6)))),M(L('3'),M(L('a3'),M(x2,z4)))),M(L('5'),x4)),w))
fpp=put('fpp',M(L('2'),A(A(M(L('a2'),z6),M(L('3'),M(L('a3'),M(x,z4)))),M(L('10'),x3))))
k=put('k',M(L('8'),M(y2,y)));aa=put('aa',M(S(M(L('2'),M(fpp,y2)),M(fp,fp)),w));bb=put('bb',S(M(L('2'),M(fp,y2)),M(aa,x)));cc=put('cc',S(M(y,k),M(A(M(aa,x),M(L('2'),bb)),M(x,w))));k2=put('k2',M(k,k));k4=put('k4',M(k2,k2));z3=put('z3',M(k,z));q3=put('q3',S(M(L('3'),M(x,k2)),M(aa,aa)));r3=put('r3',S(M(L('a3'),M(M(z3,z3),M(z3,z3))),M(S(M(L('4'),M(aa,bb)),M(L('3'),M(x,S(q3,M(x,k2))))),k2)));s3=put('s3',S(M(aa,M(q3,w)),M(L('2'),M(bb,M(k2,w)))));t3=put('t3',S(M(aa,M(r3,w)),M(cc,k4)))
names={n for n,e in v}
def render(e,word=False):
 if e[0]=='leaf':
  n=e[1]
  if word:return n
  return n if n in names else '.leaf ('+ (n+'%p' if n in ['5','8','10'] else n)+')'
 a,b=render(e[1],word),render(e[2],word)
 if word:return '(('+a+')*('+b+'))%p' if e[0]=='mul' else ('wordAdd' if e[0]=='add' else 'wordSub')+' p ('+a+') ('+b+')'
 return '.'+e[0]+' ('+a+') ('+b+')'
lines=['import WordPositive','import Tripling','set_option autoImplicit false','set_option maxRecDepth 12000','set_option maxHeartbeats 1800000','namespace PencilWordRefinement']
for word,name,typ in [(False,'tripleExpr','PayloadExpr'),(True,'tripleWords','WeightedWord')]:
 lines.append('def '+name+' (p a1 a2 a3 : UInt64) (a : WordD1) : '+typ+' :=')
 lines += [' let '+n+' := '+render(e,word) for n,e in v]
 lines.append(' ⟨q3,r3,s3,t3,z3,'+('a.w' if word else '.leaf a.w')+'⟩')
(D/'WordTriple.lean').write_text('\n'.join(lines)+'\n')
