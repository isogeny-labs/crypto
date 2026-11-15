from pathlib import Path
from itertools import product
import json,hashlib
from kernel import *
from compile import Compiler,evaluate
root=Path(__file__).resolve().parent
assert all((z**3+z+4)%11 for z in range(11))
G=(F(6),F(2),0);rational=[(F(x),F(y),0) for x in range(11) for y in range(11) if (y*y-x*x*x-2*x-7)%11==0];assert len(rational)==6
T=None
for coeff in product(range(11),repeat=3):
 x=F(coeff);rhs=x*x*x+2*x+7;y=rhs**333
 if y*y!=rhs:continue
 P=(x,y,0);candidate=mul(28,P)
 if not candidate[2] and mul(r,candidate)==O and (candidate[0]**11,candidate[1]**11,0)==mul(11,candidate):T=candidate;break
assert T is not None and weil(G,T)**7==1 and weil(G,T)!=1
Tp=[mul(i,T) for i in range(r)];zeta=weil(G,T)
for i in range(r):
 for j in range(r):assert weil(mul(i,G),Tp[j])==zeta**(i*j)
rows=[]
for Q in rational:
 compiled=Compiler(p,a,b,r,[6,2],[Q[0].c[0],Q[1].c[0]],keep=True);compiled.dual();full=Compiler(p,a,b,r,[6,2],[Q[0].c[0],Q[1].c[0]],keep=True);full.full()
 pairedG=[weil(G,U) for U in Tp];pairedQ=[weil(Q,V) for V in Tp];accepted=[];digest=hashlib.sha256();witnesses=[]
 for i,U in enumerate(Tp):
  for j,V in enumerate(Tp):
   yes=pairedG[i]*pairedQ[j]==1;digest.update(json.dumps([i,j,yes]).encode())
   if yes:
    ok,trace,values=dual(U,V,G,Q,with_values=True);assert ok
    assert len(values)==compiled.nv and not any(evaluate(e,values,F) for e in compiled.equations)
    assert trace['equations']==compiled.ne
    for n in [0,1,r-1]:
     outputs,ft,fv=full_witness(n,U,V,G,Q)
     assert len(fv)==full.nv and ft['equations']==full.ne and not any(evaluate(e,fv,F) for e in full.equations)
     assert outputs==[add(mul(n,G),U),add(mul(n,Q),V)]
    bad=values[:];bad[17]=bad[17]+1;assert any(evaluate(e,bad,F) for e in compiled.equations)
    accepted.append([enc(U),enc(V)]);witnesses.append({'i':i,'j':j,**trace})
 assert len(accepted)==r
 for i,U in enumerate(Tp):
  hit=False
  for j,V in enumerate(Tp):
   if pairedG[i]*pairedQ[j]!=1:
    ok,trace,values=dual(U,V,G,Q,with_values=True);assert not ok
    residues=[evaluate(e,values,F) for e in compiled.equations];assert all(not x for x in residues[:-1]) and residues[-1];hit=True;break
  if hit:break
 graph=[(mul(i,G),mul(i,Q)) for i in range(r)];mix=[]
 for A,B in graph:
  for z in witnesses:
   mix.append([enc(add(A,Tp[z['i']])),enc(add(B,Tp[z['j']]))])
 ordered=sorted(json.dumps(z,separators=(',',':')) for z in mix);assert len(set(ordered))==r*r
 rows.append({'Q':enc(Q),'dual_pairs':accepted,'dual_witnesses':witnesses,'candidate_pair_digest':digest.hexdigest(),'mixed_kernel_cardinality':len(set(ordered)),'mixed_kernel_sha256':hashlib.sha256('\n'.join(ordered).encode()).hexdigest()})
mr=r.bit_length()+bin(r).count('1')-2;mp=p.bit_length()+bin(p).count('1')-2;ms=(p%r).bit_length()+bin(p%r).count('1')-2
assert all(z['variables']==6+48*mr+20*ms+4*mp and z['equations']==27+196*mr+88*ms+4*mp for row in rows for z in row['dual_witnesses'])
out={'all_checks_pass':True,'field':{'p':p,'degree':3,'modulus':[4,1,0,1]},'curve':{'a':a,'b':b,'r':r,'G':enc(G)},'T':enc(T),'zeta':list(zeta.c),'targets':rows,'pairing_bilinearity_controls':49,'candidate_pair_checks':len(rows)*r*r,'dual_witnesses':len(rows)*r,'full_witnesses':len(rows)*r*3,'rejected_miller_accumulator_mutations':len(rows)*r,'rejected_nonorthogonal_witnesses':len(rows),'toy_dual_size':{'variables':6+48*mr+20*ms+4*mp,'equations':27+196*mr+88*ms+4*mp},'scope':'actual small-field kernel construction, no target logarithms supplied to producer'}
(root/'certificate.json').write_text(json.dumps(out,separators=(',',':'))+'\n');print('TARGETS',len(rows),'PAIR_TESTS',out['candidate_pair_checks'],'DUAL_WITNESSES',out['dual_witnesses'],'MIXED_POINTS_EACH',49,'DUAL_SIZE',out['toy_dual_size'])
