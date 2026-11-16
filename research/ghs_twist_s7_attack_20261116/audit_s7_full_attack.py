"""Independent reconstruction of the complete twist s=7 GHS solve."""
import hashlib
import json
import time
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, ZZ, matrix, sage_eval, vector

t0=time.process_time_ns()
HERE=Path(__file__).resolve().parent
SRC=HERE.parent/'ghs_twist_extension_20261116'
OLD=HERE.parent/'ghs_odd5_carrier_20261115'
src=json.loads((SRC/'twist_s7_transfer.json').read_text())
d=json.loads((HERE/'s7_full_attack.json').read_text())
rel_path=HERE/'relations_checkpoint.json'
rel=json.loads(rel_path.read_text())
an_path=HERE/'relations_analysis.json'
an=json.loads(an_path.read_text())
assert d['source_sha256']==hashlib.sha256((HERE/'solve_s7_target.sage').read_bytes()).hexdigest()
assert d['relations_sha256']==hashlib.sha256(rel_path.read_bytes()).hexdigest()
assert d['analysis_sha256']==hashlib.sha256(an_path.read_bytes()).hexdigest()
assert an['checkpoint_sha256']==d['relations_sha256']
F3=GF(3)
Z=PolynomialRing(F3,'z')
z=Z.gen()
K=GF(3**5,'a',modulus=z**5+2*z+1)
L=GF(3**35,'w',modulus=Z(sage_eval(src['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def lv(cs):return sum((L(c)*w**i for i,c in enumerate(cs)),L(0))
def lvec(t):return [int(c) for c in t.polynomial().list()]
eK=next(e for e in K.embeddings(L) if lvec(e(K.gen()))==src['a_embedding'])
a=eK(K.gen())
k=L.subfield(7)
ek=k.embeddings(L)[0]
back=ek.section()
tk=k.gen()
def kv(cs):return sum((k(c)*tk**i for i,c in enumerate(cs)),k(0))
R=PolynomialRing(L,'x')
x=R.gen()
Rk=PolynomialRing(k,'x')
xk=Rk.gen()
f0=x**11+x**9+2*x**8+x**7+2*x**5+x**3+x**2+x+1
f=2*f0
fk=2*(xk**11+xk**9+2*xk**8+xk**7+2*xk**5+xk**3+xk**2+xk+1)
E=EllipticCurve(L,[0,1,0,0,2*(a+a**3)])
P=E(lv(d['source_P'][0]),lv(d['source_P'][1]))
Q=E(lv(d['source_Q'][0]),lv(d['source_Q'][1]))
r=ZZ(d['r']);h=ZZ(d['cofactor'])
assert P!=E(0) and r*P==E(0) and Q==d['validation_scalar']*P
J=HyperellipticCurve(fk).jacobian()
O=J(0)
def divk(vs):return J(Rk([kv(c) for c in vs[0]]),Rk([kv(c) for c in vs[1]]))
G=divk(d['trace_G']);H=divk(d['trace_H'])
assert G!=O and r*G==O and H==d['validation_scalar']*G

# Rebuild the cover from the original coefficient vectors, independently of
# the producer's parser of polynomial expressions.
m=json.loads((OLD/'degree7_subgroup_map_30.json').read_text())['maps'][0]
def poly(cs):return R([eK(K(c)) for c in cs])
A,B,C=(poly(m[name]) for name in ('A','B','C'))
assert f*C**2==B*((2*A)**3+(2*A)**2*B+2*(a+a**3)*B**3)
Kx=PolynomialRing(K,'X')
X=Kx.gen()
fK=X**11+X**9+2*X**8+X**7+2*X**5+X**3+X**2+X+1
Qi=R([eK(c) for c in [q for q,n in fK.factor() if q.degree()==2][m['pole_quadratic']].list()])
JL=HyperellipticCurve(f).jacobian()
OL=JL(0)
D0=JL(Qi,R(0))
assert D0!=OL and 2*D0==OL
def pull(V):
    U=(2*A-V[0]*B).monic()
    W=(V[1]*B**2*C.inverse_mod(U))%U
    assert U.degree()==7 and U.is_squarefree() and (W**2-f)%U==0
    return JL(U,W)-D0
def frob(D,i):
    e=3**(7*i)
    return JL(R([c**e for c in D[0].list()]),R([c**e for c in D[1].list()]))
def trace(V):
    T=sum((frob(pull(V),i) for i in range(5)),OL)
    return J(Rk([back(c) for c in T[0].list()]),Rk([back(c) for c in T[1].list()]))
assert trace(P)==G and trace(Q)==H and trace(37*P)==37*G

base=[J(Rk([-kv(pt[0]),1]),Rk([kv(pt[1])])) for pt in rel['factor_base_points']]
assert len(base)==d['factor_base_size']==len(an['logs'])
projected=[h*D for D in base]
logs=an['logs']
assert all(int(logs[i])*G==projected[i] for i in range(len(base)))
F=GF(r)
M=matrix(F,len(rel['rows']),len(base),implementation='flint')
for i,row in enumerate(rel['rows']):
    for j,c in row:M[i,j]+=F(c)
assert M.rank()==d['relation_rank']==len(base)
assert M*vector(F,logs)==vector(F,rel['rhs'])
assert all(sum((int(c)*projected[j] for j,c in row),O)==int(val)*G
           for row,val in zip(rel['rows'],rel['rhs']))
row=d['descent_row'];offset=d['descent_offset']
assert sum((int(c)*base[j] for j,c in row),O)==H+offset*G
recovered=(sum(int(c)*logs[j] for j,c in row)*pow(int(h),-1,int(r))-offset)%r
assert recovered==d['recovered_scalar']==d['validation_scalar']
assert recovered*P==Q and recovered*G==H
out={'schema':'ghs.odd5.twist-s7-full-attack-audit/v1',
     'cover_rebuilt':True,'source_and_target_match':True,
     'factor_base_logs_checked':len(base),'relation_group_equalities_checked':len(rel['rows']),
     'independent_rank':int(M.rank()),'target_scalar':int(recovered),
     'cpu_s':float((time.process_time_ns()-t0)/1e9)}
(HERE/'audit_s7_full_attack.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps(out,sort_keys=True))
