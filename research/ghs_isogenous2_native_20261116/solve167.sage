#!/usr/bin/env sage
"""Replay the complete saved-relation GHS solve from public E167 points."""
import hashlib
import json
import time
from pathlib import Path

t_start=time.process_time_ns()
HERE=Path(__file__).resolve().parent
SRC=HERE.parent/'ghs_twist_extension_20261116'
OLD=HERE.parent/'ghs_odd5_carrier_20261115'
ISOG=HERE.parent/'ghs_twist_isogenous2_20261116'
ATTACK=HERE.parent/'ghs_twist_s7_attack_20261116'
src=json.loads((SRC/'twist_s7_transfer.json').read_text())
isog_path=ISOG/'construct.json'
isog=json.loads(isog_path.read_text())
hit=json.loads((OLD/'degree7_quadratic_square_30.json').read_text())['hits'][0]
checkpoint_path=ATTACK/'relations_checkpoint.json'
rel=json.loads(checkpoint_path.read_text())
analysis=json.loads((ATTACK/'relations_analysis.json').read_text())
assert analysis['checkpoint_sha256']==hashlib.sha256(checkpoint_path.read_bytes()).hexdigest()
assert analysis['status']=='solved' and analysis['rank']==rel['factor_base_size']
F3=GF(3)
Z=PolynomialRing(F3,'z')
z=Z.gen()
K=GF(3^5,'a',modulus=z^5+2*z+1)
L=GF(3^35,'w',modulus=Z(sage_eval(src['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
eK=next(e for e in K.embeddings(L) if [int(c) for c in e(K.gen()).polynomial().list()]==src['a_embedding'])
a=eK(K.gen())
k=L.subfield(7)
ek=k.embeddings(L)[0]
back=ek.section()
R=PolynomialRing(L,'x')
x=R.gen()
Rk=PolynomialRing(k,'x')
xk=Rk.gen()
f0=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
f=2*f0
fk=2*(xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1)
b=a+a^3
A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
assert f*C^2==B*((2*A)^3+(2*A)^2*B+2*b*B^3)
Kx=PolynomialRing(K,'X')
X=Kx.gen()
fK=X^11+X^9+2*X^8+X^7+2*X^5+X^3+X^2+X+1
Qi0=[q for q,n in fK.factor() if q.degree()==2 for _ in range(n)][hit['poles'][0]]
Qi=R([eK(c) for c in Qi0.list()])
JL=HyperellipticCurve(f).jacobian()
OL=JL(0)
D0=JL(Qi,R(0))
assert D0!=OL and 2*D0==OL
Yk=HyperellipticCurve(fk)
J=Yk.jacobian()
O=J(0)
E=EllipticCurve(L,[0,1,0,0,2*b])
def v(cs):return sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
E2=EllipticCurve(L,[0,1,0,2*a^3+2*a^2+1,a^4+a^3+1])
E167=EllipticCurve(L,[0,1,0,0,a^4+2*a+1])
P167=E167(v(isog['P167'][0]),v(isog['P167'][1]))
Q167=E167(v(isog['Q167'][0]),v(isog['Q167'][1]))
shift=2*a^3+2*a^2+1
def psi(R):
    xx,yy=R[0],R[1]
    d=-a^3-a^2-1
    xg=(xx^2+d*xx+a^3+a^2)/(xx+d)
    gm=xx^2+(a^3+a^2+1)*xx+(a^4+a^3-a^2+a-1)
    gd=xx^2+(a^3+a^2+1)*xx+(a^4-a^3+a-1)
    return E(xg-1,-yy*gm/gd)
preprocess_start=time.process_time_ns()
P=psi(E2(P167[0]+shift,P167[1]))
Q=psi(E2(Q167[0]+shift,Q167[1]))
preprocess_cpu_s=float((time.process_time_ns()-preprocess_start)/1e9)
assert P==E(v(src['point_P'][0]),v(src['point_P'][1]))
assert Q==E(v(isog['Qback'][0]),v(isog['Qback'][1]))
r=ZZ(src['r'])
h=ZZ(src['cofactor'])
secret=ZZ(12345)
assert P!=E(0) and r*P==E(0) and r*Q==E(0)
assert r*P167==E167(0) and Q167==secret*P167
source_setup_cpu_s=float((time.process_time_ns()-t_start)/1e9)

def pull(Pt):
    U=(2*A-Pt[0]*B).monic()
    assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
    V=(Pt[1]*B^2*C.inverse_mod(U))%U
    assert (V^2-f)%U==0
    return JL(U,V)-D0
def frob(D,i):
    e=3^(7*i)
    return JL(R([c^e for c in D[0].list()]),R([c^e for c in D[1].list()]))
def trace(Pt):
    D=pull(Pt)
    T=sum((frob(D,i) for i in range(5)),OL)
    assert frob(T,1)==T
    return J(Rk([back(c) for c in T[0].list()]),Rk([back(c) for c in T[1].list()]))
start=time.process_time_ns()
G=trace(P)
H=trace(Q)
assert G!=O and r*G==O and H==secret*G
transfer_cpu_s=float((time.process_time_ns()-start)/1e9)

def digits(x):
    c=[int(a) for a in x.polynomial().list()]
    return tuple(c+[0]*(7-len(c)))
start=time.process_time_ns()
pts=[p for p in Yk.rational_points() if p[2]!=0]
base=[]
idx={}
base_pts=[]
for pt in pts:
    xi,yi=pt[0],pt[1]
    if yi==0 or digits(yi)>digits(-yi):continue
    D=J(Rk([-xi,1]),Rk([yi]))
    if h*D==O:idx[xi]=None
    else:
        idx[xi]=len(base)
        base.append(D)
        base_pts.append([list(digits(xi)),list(digits(yi))])
assert base_pts==rel['factor_base_points'] and len(base)==rel['factor_base_size']
factor_base_cpu_s=float((time.process_time_ns()-start)/1e9)
logs=analysis['logs']
assert len(logs)==len(base)
def smooth(D):
    U,V=D[0],D[1]
    if U.degree()==0:return []
    row=[]
    for fac,mult in U.factor():
        if fac.degree()!=1:return None
        xi=-fac[0]/fac[1];yi=V(xi)
        if yi==0:continue
        if xi not in idx:return None
        j=idx[xi]
        if j is None:continue
        bj=base[j][1][0]
        if yi!=bj and yi!=-bj:return None
        row.append((j,(1 if yi==bj else -1)*int(mult)))
    return row
start=time.process_time_ns()
walk=H
for offset in range(100000):
    row=smooth(walk)
    if row is not None:break
    walk+=G
else:raise RuntimeError('target descent cap reached')
assert sum((int(c)*base[j] for j,c in row),O)==walk
recover=(sum(int(c)*logs[j] for j,c in row)*inverse_mod(h,r)-offset)%r
assert recover*P==Q and recover*P167==Q167 and recover*G==H and recover==secret
descent_verify_cpu_s=float((time.process_time_ns()-start)/1e9)

start=time.process_time_ns()
projected=[h*D for D in base]
assert all(logs[i]*G==projected[i] for i in range(len(base)))
factor_base_audit_cpu_s=float((time.process_time_ns()-start)/1e9)
la_cpu_s=sum(float(analysis[x]) for x in ('build_cpu_s','rank_cpu_s','solve_cpu_s'))
timing={'source_setup_cpu_s':source_setup_cpu_s,'isogeny_preprocess_cpu_s_in_setup':preprocess_cpu_s,
        'transfer_cpu_s':transfer_cpu_s,
        'factor_base_cpu_s':factor_base_cpu_s,'relation_collection_cpu_s':float(rel['relation_cpu_s']),
        'linear_algebra_cpu_s':la_cpu_s,'descent_verify_cpu_s':descent_verify_cpu_s,
        'factor_base_audit_cpu_s_excluded':factor_base_audit_cpu_s}
timing['ghs_one_solve_cpu_s_stitched']=sum(timing[x] for x in
    ('source_setup_cpu_s','transfer_cpu_s','factor_base_cpu_s',
     'relation_collection_cpu_s','linear_algebra_cpu_s','descent_verify_cpu_s'))
out={'schema':'ghs.odd5.isogenous2-native-replay/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'source_transfer_sha256':hashlib.sha256((SRC/'twist_s7_transfer.json').read_bytes()).hexdigest(),
     'isogeny_construct_sha256':hashlib.sha256(isog_path.read_bytes()).hexdigest(),
     'relations_sha256':hashlib.sha256(checkpoint_path.read_bytes()).hexdigest(),
     'analysis_sha256':hashlib.sha256((ATTACK/'relations_analysis.json').read_bytes()).hexdigest(),
     'q':int(k.cardinality()),'r':int(r),'cofactor':int(h),
     'source_P167':isog['P167'],'source_Q167':isog['Q167'],
     'mapped_P':[[int(c) for c in z.polynomial().list()] for z in (P[0],P[1])],
     'mapped_Q':[[int(c) for c in z.polynomial().list()] for z in (Q[0],Q[1])],
     'trace_G':[[[int(c) for c in v.polynomial().list()] for v in z.list()] for z in (G[0],G[1])],
     'trace_H':[[[int(c) for c in v.polynomial().list()] for v in z.list()] for z in (H[0],H[1])],
     'relation_attempts':rel['attempts'],'accepted_relations':rel['accepted'],'relation_rank':analysis['rank'],
     'factor_base_size':len(base),'factor_base_logs_verified':len(base),
     'descent_offset':offset,'descent_row':row,'recovered_scalar':int(recover),
     'validation_scalar':int(secret),'timing':timing}
target=HERE/'native_ghs167_result.json'
assert not target.exists(), 'refusing to overwrite immutable receipt'
target.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({key:out[key] for key in ('schema','q','r','relation_attempts','accepted_relations',
      'relation_rank','factor_base_size','descent_offset','recovered_scalar','timing')},sort_keys=True,default=int),flush=True)
