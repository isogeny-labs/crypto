#!/usr/bin/env sage
"""Complete s=7 solve with a JSON-safe checkpoint and dense LA."""
import hashlib
import json
import time
from pathlib import Path

program_start=time.process_time_ns()
here=Path(__file__).resolve().parent
inp=here/'s7_transfer.json';d=json.loads(inp.read_text())
cover=here.parent/'ghs_odd5_carrier_20261115'/'degree7_quadratic_square_30.json'
hit=json.loads(cover.read_text())['hits'][0]
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
L=GF(3^35,'w',modulus=Z(d['field_modulus']))
a=L(d['a_embedding']);assert a^5+2*a+1==0
k=L.subfield(7);embed_small=k.embeddings(L)[0];back=embed_small.section()
assert [int(c) for c in k.modulus().list()]==d['small_field_modulus']
R=PolynomialRing(L,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=a+a^3
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert f*C^2==B*(A^3+2*A^2*B+b*B^3)
Kx=PolynomialRing(K,'x');xx=Kx.gen()
fk=xx^11+xx^9+2*xx^8+xx^7+2*xx^5+xx^3+xx^2+xx+1
QiK=[q for q,m in fk.factor() if q.degree()==2 for _ in range(m)][hit['poles'][0]]
embedK=K.embeddings(L)[0];assert embedK(K.gen())==a
Qi=R([embedK(c) for c in QiK.list()])
JL=HyperellipticCurve(f).jacobian();OL=JL(0);D0=JL(Qi,R(0))
Rk=PolynomialRing(k,'x');xk=Rk.gen()
Y=HyperellipticCurve(xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1)
J=Y.jacobian();O=J(0)
E=EllipticCurve(L,[0,2,0,0,b])
P=E(L(d['P'][0]),L(d['P'][1]));Q=E(L(d['Q'][0]),L(d['Q'][1]))
N=ZZ(d['source_order']);r=ZZ(d['r']);h=N//r;secret=ZZ(d['secret'])
assert N==h*r and is_prime(r) and r*P==E(0) and Q==secret*P

def pull(T):
 U=(A-T[0]*B).monic();assert U.degree()==7 and U.is_squarefree()
 V=(T[1]*B^2*C.inverse_mod(U))%U
 assert (V^2-f)%U==0
 return JL(U,V)-D0
def frob(D,j):
 return JL(R([c^(3^(7*j)) for c in D[0].list()]),
           R([c^(3^(7*j)) for c in D[1].list()]))
def transfer(T):
 D=pull(T)
 total=sum((frob(D,j) for j in range(5)),OL)
 assert frob(total,1)==total
 return J(Rk([back(c) for c in total[0].list()]),
          Rk([back(c) for c in total[1].list()]))

timing={};start=time.process_time_ns()
G=transfer(P);H=transfer(Q);G37=transfer(37*P)
assert G!=O and r*G==O and H==secret*G and G37==37*G
timing['transfer_cpu_s']=float((time.process_time_ns()-start)/1e9)

def digits(v):
 coeff=[int(c) for c in v.polynomial().list()]
 return tuple(coeff+[int(0)]*(7-len(coeff)))
start=time.process_time_ns()
points=[p for p in Y.rational_points() if p[2]!=0]
base=[];base_index={};base_coords=[]
for pt in points:
 xi,yi=pt[0],pt[1]
 if yi==0:
  base_index[xi]=None
  continue
 if digits(yi)>digits(-yi):continue
 cls=J(Rk([-xi,1]),Rk([yi]))
 base_index[xi]=len(base)
 base.append(cls);base_coords.append([list(digits(xi)),list(digits(yi))])
n=len(base);assert n>0
json.dumps({'factor_base_coords':base_coords},default=int)
timing['setup_factor_base_cpu_s']=float((time.process_time_ns()-start)/1e9)

def smooth(D):
 U,V=D[0],D[1]
 row={}
 if U.degree()==0:return row
 for fac,mult in U.factor():
  if fac.degree()!=1:return None
  xi=-fac[0]/fac[1];yi=V(xi)
  if xi not in base_index:return None
  j=base_index[xi]
  if j is None:continue
  bj=base[j][1][0]
  if yi!=bj and yi!=-bj:return None
  row[j]=row.get(j,0)+(int(mult) if yi==bj else -int(mult))
 return {j:v for j,v in row.items() if v}

start=time.process_time_ns();rows=[];rhs=[];walk=O;rank=0;attempts=0
cap=1000000
for aa in range(1,cap+1):
 walk+=G;attempts=aa
 row=smooth(walk)
 if row is None:continue
 rows.append(row);rhs.append(int(h*aa%r))
 if len(rows)<2*n or (len(rows)-2*n)%250:continue
 M=matrix(GF(r),len(rows),n,sparse=True)
 for i,rr in enumerate(rows):
  for j,v in rr.items():M[i,j]=v
 rank=int(M.rank())
 print(json.dumps({'phase':'relation_rank','attempts':aa,'accepted':len(rows),
                   'rank':rank,'factor_base_size':n}),flush=True)
 if rank==n:break
else:
 print(json.dumps({'phase':'relation_cap','attempts':attempts,'accepted':len(rows),
                   'rank':rank,'factor_base_size':n}),flush=True)
 raise RuntimeError('relation cap reached before full rank')
checkpoint={'schema':'ghs.odd5.s7-relations/v3',
 'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
 'transfer_receipt_sha256':hashlib.sha256(inp.read_bytes()).hexdigest(),
 'r':int(r),'h':int(h),'secret':int(secret),'factor_base_size':n,
 'factor_base_coords':base_coords,'relation_attempts':attempts,
 'accepted_relations':len(rows),'relation_rank':rank,
 'relation_rows':[[[int(j),int(v)] for j,v in sorted(rr.items())] for rr in rows],
 'relation_rhs':rhs}
checkpoint_path=here/'s7_relations_v3.json'
assert not checkpoint_path.exists(), 'refusing to overwrite relation checkpoint'
checkpoint_path.write_text(json.dumps(checkpoint,sort_keys=True,default=int)+'\n')
print(json.dumps({'phase':'relation_checkpoint','path':str(checkpoint_path),
                  'attempts':attempts,'accepted':len(rows),'rank':rank}),flush=True)
M=matrix(GF(r),len(rows),n,sparse=False)
for i,rr in enumerate(rows):
 for j,v in rr.items():M[i,j]=v
logs=M.solve_right(vector(GF(r),rhs))
assert all(int(logs[i])*G==h*base[i] for i in range(n))
timing['relations_linear_algebra_verify_cpu_s']=float((time.process_time_ns()-start)/1e9)

start=time.process_time_ns();T=H;descent_cap=100000
for offset in range(descent_cap):
 row=smooth(T)
 if row is not None:break
 T+=G
else:raise RuntimeError('target descent cap reached')
recovered=(int(sum(row[j]*logs[j] for j in row))*inverse_mod(h,r)-offset)%r
assert recovered==secret and recovered*P==Q and recovered*G==H
timing['descent_verify_cpu_s']=float((time.process_time_ns()-start)/1e9)
timing['ghs_total_cpu_s']=float((time.process_time_ns()-program_start)/1e9)
timing['initial_setup_cpu_s']=timing['ghs_total_cpu_s']-sum(
 timing[v] for v in ('transfer_cpu_s','setup_factor_base_cpu_s',
                    'relations_linear_algebra_verify_cpu_s','descent_verify_cpu_s'))
out={'schema':'ghs.odd5.s7-full-attack/v3',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'transfer_receipt_sha256':hashlib.sha256(inp.read_bytes()).hexdigest(),
     'r':int(r),'secret':int(secret),'recovered':int(recovered),
     'factor_base_size':n,'rational_finite_points':len(points),
     'factor_base_coords':base_coords,'relation_attempts':attempts,
     'accepted_relations':len(rows),'relation_rank':rank,
     'relation_rows':[[[int(j),int(v)] for j,v in sorted(rr.items())] for rr in rows],
     'relation_rhs':rhs,'factor_base_projected_logs':[int(v) for v in logs],
     'descent_row':[[int(j),int(v)] for j,v in sorted(row.items())],
     'descent_offset':offset,'timing':timing}
target=here/'s7_attack_v3.json'
assert not target.exists(), 'refusing to overwrite receipt'
target.write_text(json.dumps(out,sort_keys=True,default=int)+'\n')
print(json.dumps({key:out[key] for key in ('schema','r','factor_base_size','relation_attempts',
 'accepted_relations','relation_rank','descent_offset','recovered','timing')},sort_keys=True))
