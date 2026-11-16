"""Independently rebuild the saved s=3 map, modular solve and source scalar."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing, ZZ

PTH=Path(__file__).resolve().parent
OLD=PTH.parent/'ghs_odd5_carrier_20261115'
d=json.loads((PTH/'twist_s3_full_attack.json').read_text())
assert d['source_sha256']==hashlib.sha256((PTH/'twist_s3_full_attack.sage').read_bytes()).hexdigest()
F3=GF(3);Z=PolynomialRing(F3,'z')
K=GF(3**5,'a',modulus=Z([1,2,0,0,0,1]))
L=GF(3**15,'w',modulus=Z(d['field_modulus']))
k=GF(3**3,'t',modulus=Z(d['small_field_modulus']))
def vec(v):return [int(c) for c in v.polynomial().list()]
eK=next(e for e in K.embeddings(L) if vec(e(K.gen()))==d['a_embedding'])
ek=next(e for e in k.embeddings(L) if vec(e(k.gen()))==d['small_field_embedding'])
w=L.gen();tt=k.gen()
def l(v):return sum((L(c)*w**i for i,c in enumerate(v)),L(0))
def s(v):return sum((k(c)*tt**i for i,c in enumerate(v)),k(0))
def pL(v):return L(0) if not v else l(v)
R=PolynomialRing(L,'x');x=R.gen()
Rk=PolynomialRing(k,'x');xx=Rk.gen()
f=2*(x**11+x**9+2*x**8+x**7+2*x**5+x**3+x**2+x+1)
fk=2*(xx**11+xx**9+2*xx**8+xx**7+2*xx**5+xx**3+xx**2+xx+1)
a=eK(K.gen());E=EllipticCurve(L,[0,1,0,0,2*(a+a**3)])
P=E(l(d['source_P'][0]),l(d['source_P'][1]))
Q=E(l(d['source_Q'][0]),l(d['source_Q'][1]))
r=ZZ(d['r']);h=ZZ(d['source_order'])//r
assert r*P==E(0) and Q==d['validation_scalar']*P
J=HyperellipticCurve(fk).jacobian();O=J(0)
def divk(v):return J(Rk([s(c) for c in v[0]]),Rk([s(c) for c in v[1]]))
G=divk(d['trace_G']);H=divk(d['trace_H'])
assert G!=O and r*G==O and H==d['validation_scalar']*G

# Reconstruct the cover from saved F243 coefficient vectors, independently of
# the original search's string parser and the attack producer's code path.
m=json.loads((OLD/'degree7_subgroup_map_30.json').read_text())['maps'][0]
def poly(v):return R([eK(K(c)) for c in v])
A,B,C=(poly(m[name]) for name in ('A','B','C'))
assert f*C**2==B*((2*A)**3+(2*A)**2*B+2*(a+a**3)*B**3)
Kx=PolynomialRing(K,'X');X=Kx.gen()
fK=X**11+X**9+2*X**8+X**7+2*X**5+X**3+X**2+X+1
Qi=R([eK(c) for c in [q for q,n in fK.factor() if q.degree()==2][m['pole_quadratic']].list()])
JL=HyperellipticCurve(f).jacobian();OL=JL(0);D0=JL(Qi,R(0))
def pull(V):
 U=(2*A-V[0]*B).monic();W=(V[1]*B**2*C.inverse_mod(U))%U
 assert U.degree()==7 and (W**2-f)%U==0
 return JL(U,W)-D0
def frob(D,i):return JL(R([c**(3**(3*i)) for c in D[0].list()]),R([c**(3**(3*i)) for c in D[1].list()]))
def trace(V):
 T=sum((frob(pull(V),i) for i in range(5)),OL)
 return J(Rk([ek.section()(c) for c in T[0].list()]),Rk([ek.section()(c) for c in T[1].list()]))
assert trace(P)==G and trace(Q)==H and trace(37*P)==37*G

pts=d['factor_base_points'];assert len(pts)==d['factor_base_size']
base=[J(Rk([-s(v[0]),1]),Rk([s(v[1])])) for v in pts]
logs=d['factor_base_logs'];assert len(logs)==len(base)
projected=[h*B for B in base]
rows=d['relation_rows'];rhs=d['relation_rhs']
assert len(rows)==len(rhs)==d['accepted_relations']
assert all(sum(row[i]*logs[i] for i in range(len(logs)))%r==val%r for row,val in zip(rows,rhs))
assert all(sum((int(c)*projected[i] for i,c in enumerate(row) if c),O)==int(val)*G for row,val in zip(rows,rhs))

# Independent modular Gaussian elimination on the recorded relation matrix.
mat=[[int(z)%int(r) for z in row] for row in rows]
rank=0
for col in range(len(logs)):
 pivot=next((i for i in range(rank,len(mat)) if mat[i][col]),None)
 if pivot is None:continue
 mat[rank],mat[pivot]=mat[pivot],mat[rank]
 inv=pow(mat[rank][col],-1,int(r))
 mat[rank]=[(v*inv)%r for v in mat[rank]]
 for i in range(rank+1,len(mat)):
  mul=mat[i][col]
  if mul:mat[i]=[(mat[i][j]-mul*mat[rank][j])%r for j in range(len(logs))]
 rank+=1
assert rank==d['relation_rank']==len(logs)-1
ker=d['relation_right_kernel'];assert len(ker)==1 and any(ker[0])
assert all(sum(row[i]*ker[0][i] for i in range(len(logs)))%r==0 for row in rows)
target=d['descent_row'];offset=d['descent_offset']
assert sum(target[i]*ker[0][i] for i in range(len(logs)))%r==0
assert sum((int(c)*base[i] for i,c in enumerate(target) if c),O)==H+offset*G
recovered=(sum(target[i]*logs[i] for i in range(len(logs)))*pow(int(h),-1,int(r))-offset)%r
assert recovered==d['recovered_scalar']==d['validation_scalar']
assert recovered*P==Q and recovered*G==H
assert len(d['rho_trials'])==30 and all(z['scalar']==recovered for z in d['rho_trials'])
assert all(z['cpu_s']>0 and z['steps']>=0 for z in d['rho_trials'])
print(json.dumps({'schema':'ghs.odd5.twist-s3-full-attack-audit/v1','cover_rebuilt':True,
                  'source_and_target_match':True,'relation_group_equalities_verified':len(rows),
                  'independent_modular_rank':rank,'relations_verified':len(rows),
                  'target_scalar':int(recovered),'rho_trials_verified':len(d['rho_trials'])},sort_keys=True))
