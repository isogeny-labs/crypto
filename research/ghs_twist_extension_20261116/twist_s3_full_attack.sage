#!/usr/bin/env sage
"""End-to-end degree-seven transfer, genus-five IC and Frobenius rho at s=3."""
import hashlib
import json
import random
import time
from pathlib import Path

t_program_start=time.process_time_ns()
PTH=Path(__file__).resolve().parent
OLD=PTH.parent/'ghs_odd5_carrier_20261115'
hit=json.loads((OLD/'degree7_quadratic_square_30.json').read_text())['hits'][0]
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
L=GF(3^15,'w');ek=K.embeddings(L)[0];a=ek(K.gen())
k=L.subfield(3);e3=k.embeddings(L)[0];back=e3.section()
R=PolynomialRing(L,'x');x=R.gen()
f=2*(x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1)
b=a+a^3
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert f*C^2==B*((2*A)^3+(2*A)^2*B+2*b*B^3)
Kx=PolynomialRing(K,'x');xx=Kx.gen();fk=xx^11+xx^9+2*xx^8+xx^7+2*xx^5+xx^3+xx^2+xx+1
Qi0=[q for q,n in fk.factor() if q.degree()==2 for _ in range(n)][hit['poles'][0]]
Qi=R([ek(c) for c in Qi0.list()])
JL=HyperellipticCurve(f).jacobian();OL=JL(0);D0=JL(Qi,R(0))
Rk=PolynomialRing(k,'x');xk=Rk.gen()
fk2=2*(xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1)
Yk=HyperellipticCurve(fk2);J=Yk.jacobian();O=J(0)
N=ZZ(14347530);r=ZZ(17713);h=N//r
assert N==810*r and is_prime(r) and J.cardinality()==N
E=EllipticCurve(L,[0,1,0,0,2*b]);set_random_seed(20261116+3+30)
P=E(0)
for _ in range(30):
 candidate=h*E.random_point()
 if not candidate.is_zero():P=candidate;break
assert P!=E(0) and r*P==E(0)
secret=12345;Q=secret*P
def pull(Pt):
 U=(2*A-Pt[0]*B).monic();assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
 V=(Pt[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 return JL(U,V)-D0
def frobL(D,i):return JL(R([c^(3^(3*i)) for c in D[0].list()]),R([c^(3^(3*i)) for c in D[1].list()]))
def trace(Pt):
 D=pull(Pt);T=sum((frobL(D,i) for i in range(5)),OL)
 assert frobL(T,1)==T
 return J(Rk([back(c) for c in T[0].list()]),Rk([back(c) for c in T[1].list()]))
timing={};start=time.process_time_ns()
G=trace(P);H=trace(Q);G37=trace(37*P)
assert G!=O and r*G==O and H==secret*G and G37==37*G
timing['transfer_cpu_s']=float((time.process_time_ns()-start)/1e9)

def digits(v,n):
 c=[int(x) for x in v.polynomial().list()]
 return tuple(c+[0]*(n-len(c)))
pts=[p for p in Yk.rational_points() if p[2]!=0]
base=[];base_index={}
for pt in pts:
 xi,yi=pt[0],pt[1]
 if yi==0 or digits(yi,3)>digits(-yi,3):continue
 cls=J(Rk([-xi,1]),Rk([yi]))
 if h*cls==O:
  base_index[xi]=None
 else:
  base_index[xi]=len(base);base.append(cls)
n=len(base);assert n>0
def smooth(D):
 U,V=D[0],D[1]
 row=[0]*n
 if U.degree()==0:return row
 for fac,mult in U.factor():
  if fac.degree()!=1:return None
  xi=-fac[0]/fac[1];yi=V(xi)
  if yi==0:continue
  if xi not in base_index:return None
  j=base_index[xi]
  if j is None:continue
  bj=base[j][1][0]
  if yi!=bj and yi!=-bj:return None
  row[j]+=(1 if yi==bj else -1)*int(mult)
 return row

start=time.process_time_ns();rows=[];rhs=[];attempts=0;full_rank=False
for shift_index in [-1]+list(range(n)):
 walk=O if shift_index<0 else base[shift_index]
 for aa in range(1,3001):
  walk+=G;attempts+=1
  row=smooth(walk)
  if row is None:continue
  if shift_index>=0:row[shift_index]-=1
  rows.append(row);rhs.append(int(h*aa%r))
  if len(rows)>=n and matrix(GF(r),rows).rank()==n:
   full_rank=True;break
 if full_rank:break
if not full_rank and (not rows or matrix(GF(r),rows).rank()<n-1):
 print(json.dumps({'phase':'relation_rank_cap','factor_base_size':n,'rational_points':len(pts),
                   'attempts':attempts,'accepted':len(rows),
                   'rank':int(matrix(GF(r),rows).rank()) if rows else 0,
                   'column_support':[sum(v[i]!=0 for v in rows) for i in range(n)],
                   'right_kernel':[list(map(int,v)) for v in matrix(GF(r),rows).right_kernel().basis()] if rows else []},sort_keys=True,default=int),flush=True)
 raise RuntimeError('relation rank not reached within shifted relation cap')
M=matrix(GF(r),rows);logs=M.solve_right(vector(GF(r),rhs))
assert M*logs==vector(GF(r),rhs)
kernel=M.right_kernel().basis()
timing['relations_and_la_cpu_s']=float((time.process_time_ns()-start)/1e9)
start=time.process_time_ns();T=H;descent_attempts=0
for tt in range(20001):
 row=smooth(T);descent_attempts=tt+1
 if row is not None and all(vector(GF(r),row).dot_product(v)==0 for v in kernel):break
 T+=G
else:raise RuntimeError('target descent not reached within 20000 offsets')
recover=(int(sum(row[i]*logs[i] for i in range(n)))*inverse_mod(h,r)-tt)%r
assert recover*P==Q and recover*G==H and recover==secret
timing['descent_verify_cpu_s']=float((time.process_time_ns()-start)/1e9)
t_after_attack=time.process_time_ns()
phase_cpu_s=sum(timing.values())
timing['ghs_total_cpu_s']=float((t_after_attack-t_program_start)/1e9)
timing['setup_and_factor_base_cpu_s']=timing['ghs_total_cpu_s']-phase_cpu_s

# The K-Frobenius eigenvalue has order three on this new r-primary subgroup.
Fr=lambda Pt:E(Pt[0]^243,Pt[1]^243)
S=PolynomialRing(GF(r),'t');t=S.gen()
roots=[int(v) for v,mult in (t^2+26*t+243).roots()]
lam=next(v for v in roots if Fr(P)==v*P)
assert pow(lam,3,r)==1 and lam!=1 and Fr(Q)==lam*Q
def key(Pt):return digits(Pt[0],15)+digits(Pt[1],15)
def canon(Pt,aa,bb):
 if Pt.is_zero():return Pt,aa,bb
 choices=[(Pt,aa,bb)];D=Pt
 for j in (1,2):
  D=Fr(D);choices.append((D,aa*pow(lam,j,r)%r,bb*pow(lam,j,r)%r))
 return min(choices,key=lambda z:key(z[0]))
def rho_trial(seed):
 rng=random.Random(int(seed));start=time.process_time_ns()
 count=0
 for restart in range(100):
  steps=[(rng.randrange(r),rng.randrange(r)) for _ in range(16)]
  jumps=[u*P+v*Q for u,v in steps]
  aa=rng.randrange(r);bb=rng.randrange(r);D=aa*P+bb*Q
  seen={}
  for _ in range(5000):
   D,aa,bb=canon(D,aa,bb);state=key(D) if not D.is_zero() else ('O',)
   if state in seen:
    aa0,bb0=seen[state];den=(bb-bb0)%r
    if den:
     candidate=((aa0-aa)*inverse_mod(den,r))%r
     if candidate*P==Q:
      return {'cpu_s':float((time.process_time_ns()-start)/1e9),'steps':count,
              'restarts':restart,'scalar':int(candidate)}
    break
   seen[state]=(aa,bb)
   j=hashlib.blake2s(bytes(state),digest_size=2).digest()[0]%len(jumps) if state!=('O',) else 0
   D+=jumps[j];aa=(aa+steps[j][0])%r;bb=(bb+steps[j][1])%r;count+=1
   if count>=20000:break
  if count>=20000:break
 raise RuntimeError('rho cap reached')
rho=[rho_trial(20261116+i) for i in range(30)]
assert all(z['scalar']==secret for z in rho)
out={'schema':'ghs.odd5.twist-s3-full-attack/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'bmask':30,'q':27,'source_order':int(N),'r':int(r),'validation_scalar':secret,
 'field_modulus':[int(c) for c in L.modulus().list()],
 'small_field_modulus':[int(c) for c in k.modulus().list()],
 'a_embedding':[int(c) for c in a.polynomial().list()],
 'small_field_embedding':[int(c) for c in e3(k.gen()).polynomial().list()],
 'factor_base_size':n,'factor_base_rational_points':len(pts),'relation_attempts':attempts,'accepted_relations':len(rows),
 'relation_rank':int(M.rank()),'relation_right_kernel':[list(map(int,v)) for v in kernel],
 'descent_row_kernel_orthogonal':all(vector(GF(r),row).dot_product(v)==0 for v in kernel),
 'descent_attempts':descent_attempts,'recovered_scalar':int(recover),
 'timing':timing,'rho_eigenvalue':lam,'rho_trials':rho,'rho_median_cpu_s':sorted(z['cpu_s'] for z in rho)[len(rho)//2],
 'rho_median_steps':sorted(z['steps'] for z in rho)[len(rho)//2],
 'source_P':[[int(c) for c in z.polynomial().list()] for z in (P[0],P[1])],
 'source_Q':[[int(c) for c in z.polynomial().list()] for z in (Q[0],Q[1])],
 'trace_G':[[[int(c) for c in v.polynomial().list()] for v in z.list()] for z in (G[0],G[1])],
 'trace_H':[[[int(c) for c in v.polynomial().list()] for v in z.list()] for z in (H[0],H[1])],
 'factor_base_points':[[list(digits(z,3)) for z in (pt[0],pt[1])] for pt in pts if pt[1]!=0 and digits(pt[1],3)<=digits(-pt[1],3) and h*J(Rk([-pt[0],1]),Rk([pt[1]]))!=O],
 'relation_rows':rows,'relation_rhs':rhs,'factor_base_logs':[int(z) for z in logs],
 'descent_row':row,'descent_offset':tt}
target=PTH/'twist_s3_full_attack.json'
assert not target.exists(), 'refusing to overwrite immutable receipt'
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({k:out[k] for k in ['schema','r','factor_base_size','relation_attempts','accepted_relations','relation_rank','descent_attempts','recovered_scalar','timing','rho_eigenvalue','rho_median_cpu_s','rho_median_steps']},sort_keys=True,default=int))
