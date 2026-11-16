from pathlib import Path
import json,hashlib
from functools import lru_cache
root=Path(__file__).resolve().parent;c=json.loads((root/'certificate.json').read_text());bp=json.loads((root/'P192-blueprint.json').read_text());inp=json.loads((root.parent/'round52-p192/input.json').read_text());p=11;r=7
# Independent finite-field representation: packed base-11 integers with long polynomial reduction.
def digits(v):return [v%11,(v//11)%11,v//121]
def pack(v):return sum((x%11)*11**i for i,x in enumerate(v))
def plus(a,b):return pack([x+y for x,y in zip(digits(a),digits(b))])
def neg(a):return pack([-x for x in digits(a)])
def sub(a,b):return plus(a,neg(b))
def times(a,b):
 v=[0]*5
 for i,x in enumerate(digits(a)):
  for j,y in enumerate(digits(b)):v[i+j]=(v[i+j]+x*y)%11
 for j in [4,3]:
  lead=v[j]
  for k,coeff in enumerate([4,1,0,1]):v[j-3+k]=(v[j-3+k]-lead*coeff)%11
 return pack(v[:3])
def power(a,n):
 out=1
 for bit in bin(n)[2:]:
  out=times(out,out)
  if bit=='1':out=times(out,a)
 return out
@lru_cache(None)
def inverse(a):
 assert a;v=power(a,1329);assert times(a,v)==1;return v
def div(a,b):return times(a,inverse(b))
def ecadd(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and plus(y,v)==0:return None
 slope=div(plus(times(3,times(x,x)),2),times(2,y)) if P==Q else div(sub(v,y),sub(u,x))
 X=sub(sub(times(slope,slope),x),u);return X,sub(times(slope,sub(x,X)),y)
def ecmul(n,P):
 out=None
 for bit in bin(n)[2:]:
  out=ecadd(out,out)
  if bit=='1':out=ecadd(out,P)
 return out
def read(P):return None if P is None else tuple(pack(x) for x in P)
def encode(P):return None if P is None else [digits(x) for x in P]
def line(A,B,R):
 if A is None or B is None or R is None:return 1
 S=ecadd(A,B)
 if S is None:return sub(R[0],A[0])
 slope=div(plus(times(3,times(A[0],A[0])),2),times(2,A[1])) if A==B else div(sub(B[1],A[1]),sub(B[0],A[0]))
 return div(sub(sub(R[1],A[1]),times(slope,sub(R[0],A[0]))),sub(R[0],S[0]))
def miller(n,P,R):
 if n==1:return P,1
 if n%2:
  A,f=miller(n-1,P,R);return ecadd(A,P),times(f,line(A,P,R))
 A,f=miller(n//2,P,R);return ecadd(A,A),times(times(f,f),line(A,A,R))
def weil(P,R):
 if P is None or R is None:return 1
 A,f=miller(r,P,R);B,g=miller(r,R,P);assert A is None and B is None;return neg(div(f,g))
assert all((z**3+z+4)%11 for z in range(11))
G=read(c['curve']['G']);T=read(c['T']);assert G==(6,2) and ecmul(r,G) is None and ecmul(r,T) is None
assert tuple(power(z,11) for z in T)==ecmul(11,T)
Gt=[ecmul(i,G) for i in range(r)];Tt=[ecmul(i,T) for i in range(r)];assert T not in Gt
zeta=weil(G,T);assert zeta==pack(c['zeta']) and zeta!=1 and power(zeta,7)==1
for i in range(r):
 for j in range(r):assert weil(Gt[i],Tt[j])==power(zeta,i*j)
for row in c['targets']:
 Q=read(row['Q']);x=next(i for i,P in enumerate(Gt) if P==Q);assert x!=0;accepted=[];digest=hashlib.sha256()
 for i,U in enumerate(Tt):
  for j,V in enumerate(Tt):
   expected=(i+x*j)%r==0;actual=times(weil(G,U),weil(Q,V))==1;assert expected==actual
   digest.update(json.dumps([i,j,actual]).encode())
   if actual:accepted.append([encode(U),encode(V)])
 assert accepted==row['dual_pairs'] and digest.hexdigest()==row['candidate_pair_digest']
 mixed=[];labels=[]
 for j in range(r):
  for w in row['dual_witnesses']:
   i,l=w['i'],w['j'];assert (i+x*l)%r==0 and w['variables']==258 and w['equations']==1007
   mixed.append([encode(ecadd(Gt[j],Tt[i])),encode(ecadd(ecmul(j,Q),Tt[l]))]);labels.append((j,x*j%r,i,l))
 ordered=sorted(json.dumps(z,separators=(',',':')) for z in mixed);assert len(set(ordered))==49==row['mixed_kernel_cardinality'];assert hashlib.sha256('\n'.join(ordered).encode()).hexdigest()==row['mixed_kernel_sha256']
 S=set(labels)
 for A in labels:
  for B in labels:
   assert tuple((a+b)%r for a,b in zip(A,B)) in S
   assert (A[0]*B[2]+A[1]*B[3]-A[2]*B[0]-A[3]*B[1])%r==0
 assert sum((bb+x*dd)%r==0 for aa in range(r) for bb in range(r) for cc in range(r) for dd in range(r))==r**3
# Independent count derivation, including the short scalar p mod r and the positive NIST power chain.
R=inp['r'];P=inp['p'];mr=R.bit_length()+bin(R).count('1')-2;s=P%R;ms=s.bit_length()+bin(s).count('1')-2
exponent=2**64-1
for width in [32,16,8,4,2,1]:exponent=exponent*2**width+(2**width-1)
exponent=exponent*2*2**64+(2**64-1);assert exponent==P
mf=sum(w//2+1 for w in [2,4,8,16,32,64])+sum(w+1 for w in [32,16,8,4,2,1])+1+64+1;assert mf==204
V=6+48*mr+20*ms+4*mf;E=27+196*mr+88*ms+4*mf;k=R.bit_length()
assert (bp['dual']['variables'],bp['dual']['equations'])==(V,E)
assert (bp['full_mixed']['variables'],bp['full_mixed']['equations'])==(V+21*k+20,E+89*k+88)
assert bp['full_mixed']['maximum_degree']==bp['dual']['maximum_degree']==4
out={'all_checks_pass':True,'independent_extension_pairing_controls':49,'independent_dual_membership_tests':294,'independent_mixed_kernel_points':294,'kernel_closure_and_isotropy_tests':6*49**2,'dual_size':{'variables':258,'equations':1007},'P192_full_size':bp['full_mixed'],'P192_count_audit':{'m_r':mr,'m_p_mod_r':ms,'power_gates':mf},'negative_scope':'omitting eigenspace constraints gives r^3 points; no general quotient evaluated'}
(root/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
