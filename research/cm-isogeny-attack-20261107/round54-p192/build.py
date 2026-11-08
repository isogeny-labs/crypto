from pathlib import Path
import json,hashlib,importlib.util
from tower import F,K
root=Path(__file__).resolve().parent;c=json.loads((root.parent/'round53-p192/certificate.json').read_text());s=importlib.util.spec_from_file_location('curve52',root.parent/'round52-p192/model.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
def add(P,Q):return m.add(P,Q,11,2)
def mul(n,P):return m.mul(n,P,11,2)
def read(P):return None if P is None else tuple(K(F(z)) for z in P)
def code(P):return None if P is None else [z.code() for z in P]
def pk(P):return json.dumps(code(P),separators=(',',':'))
G=read(c['curve']['G']);T=read(c['T']);torsion=[add(mul(i,G),mul(j,T)) for i in range(7) for j in range(7)];idx={pk(P):i for i,P in enumerate(torsion)}
assert len(idx)==49 and K(2)**((11**6-1)//2)==1
# 2 is nonsquare in F11 and remains nonsquare in its odd-degree cubic extension.
assert pow(2,5,11)==10 and F(2)**665==-1
W=K(0,1);S=(K(0),3*W);assert S[1]*S[1]==S[0]**3+2*S[0]+7 and mul(17,S) is None and mul(7,S) is not None
assert tuple(z**11 for z in S)==(S[0],-S[1])
addidx=lambda a,b:((a//7+b//7)%7)*7+(a%7+b%7)%7
shifts=[(S,mul(n,S)) for n in range(1,9)]
Xs=[add(S,U)[0] for U in torsion];assert sum(Xs,K())==49*mul(7,S)[0]
Ys=[[add(R,U)[0] for U in torsion] for _,R in shifts]
assert all(sum(ys,K())==49*mul(7,R)[0] for ys,(_,R) in zip(Ys,shifts))
rows=[]
for target in c['targets']:
 Q=read(target['Q']);H=[]
 for j in range(7):
  A,B=mul(j,G),mul(j,Q)
  for UV in target['dual_pairs']:H.append((add(A,read(UV[0])),add(B,read(UV[1]))))
 psi=[None]*49
 for A,B in H:psi[idx[pk(A)]]=idx[pk(B)]
 assert sorted(psi)==list(range(49))
 arrays=[];attempts=[]
 for n,ys in enumerate(Ys,1):
  values=[sum((Xs[u]*ys[addidx(psi[u],d)] for u in range(49)),K()) for d in range(49)]
  rational=[values[7*i] for i in range(7)]
  for i,v in enumerate(rational):assert v**121==v and v**11==rational[-i%7]
  for d,v in enumerate(values):
   frobminus=((-(d//7))%7)*7+((-4*(d%7))%7)
   assert v**11==values[frobminus]
  arrays.append(values);attempts.append({'shift_n':n,'distinct_conjugates':len(set(values)),'distinct_rational_cosets':len(set(rational)),'values':[v.code() for v in values]})
 signatures=[tuple(tuple(v[d].code()) for v in arrays) for d in range(49)];assert len(set(signatures))==49
 # Full translation fiber of the joint evaluation signature, via the checked group reindexing.
 fixed=[]
 for a in range(49):
  for b in range(49):
   pa=psi[a];delta=addidx(b,((-(pa//7))%7)*7+((-(pa%7))%7))
   if signatures[delta]==signatures[0]:fixed.append([a,b])
 assert fixed==[[a,psi[a]] for a in range(49)]
 # Direct H-invariance check at the first shift, including genuine point translations.
 for A,B in H:
  P,R=add(S,A),add(S,B);v=sum((add(P,U)[0]*add(R,V)[0] for U,V in H),K());assert v==arrays[0][0]
 # Norm polynomial at the first shift; all repeated roots, if any, are retained.
 poly=[K(1)]
 for v in arrays[0]:
  out=[K()]*(len(poly)+1)
  for i,a in enumerate(poly):out[i]=out[i]-v*a;out[i+1]=out[i+1]+a
  poly=out
 assert all(a**11==a for a in poly)
 separating=arrays[5];assert len(set(separating))==49
 poly6=[K(1)]
 for v in separating:
  out6=[K()]*(len(poly6)+1)
  for i,a in enumerate(poly6):out6[i]=out6[i]-v*a;out6[i+1]=out6[i+1]+a
  poly6=out6
 assert all(v**11==v for v in poly6)
 rows.append({'Q':code(Q),'psi_permutation':psi,'attempts':attempts,'joint_conjugate_signatures':49,'joint_translation_stabilizer':fixed,'norm_polynomial_coefficients':[v.code() for v in poly],'norm_degree':49,'separating_shift_n':6,'separating_norm_polynomial_coefficients':[v.code() for v in poly6]})
 print('TARGET',code(Q),'distinct',[(z['distinct_conjugates'],z['distinct_rational_cosets']) for z in attempts],flush=True)
out={'all_checks_pass':True,'tower':{'base_modulus':[4,1,0,1],'quadratic_relation':'W^2=2'},'curve':{'p':11,'a':2,'b':7,'r':7,'G':code(G),'T':code(T)},'shift_generator':code(S),'shift_order':17,'targets':rows,'norm_selection':'n=6 chosen after evaluating the complete fixed panel; every attempt retained, no statistical inference','marginal_identity':'sum x(P+u)=r^2*x([r]P), tested on all eight shifts','scope':'exact mixed trace evaluations and quotient-field construction; no P192 evaluation'}
(root/'certificate.json').write_text(json.dumps(out,separators=(',',':'))+'\n')
