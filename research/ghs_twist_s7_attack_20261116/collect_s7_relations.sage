#!/usr/bin/env sage
"""Bounded relation-yield probe on the saved twist s=7 transfer image."""
import hashlib
import json
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE.parent/'ghs_twist_extension_20261116'
RECEIPT=SRC/'twist_s7_transfer.json'
d = json.loads(RECEIPT.read_text())
assert len(sys.argv)==2
limit=int(sys.argv[1]);assert 1<=limit<=300000
F3 = GF(3)
Z = PolynomialRing(F3,'z')
z = Z.gen()
L = GF(3^35,'w',modulus=Z(sage_eval(d['field_modulus'],locals={'x':z},preparse=True)))
w = L.gen()
k = L.subfield(7)
e = k.embeddings(L)[0]
back = e.section()
R = PolynomialRing(L,'x')
Rk = PolynomialRing(k,'x')
x = R.gen()
xk = Rk.gen()
f = 2*(x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1)
fk = 2*(xk^11+xk^9+2*xk^8+xk^7+2*xk^5+xk^3+xk^2+xk+1)
JL = HyperellipticCurve(f).jacobian()
J = HyperellipticCurve(fk).jacobian()
O = J(0)
def v(cs):return sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
def div(vs):return JL(R([v(c) for c in vs[0]]),R([v(c) for c in vs[1]]))
T = div(d['trace_P'])
G = J(Rk([back(c) for c in T[0].list()]),Rk([back(c) for c in T[1].list()]))
r = ZZ(d['r'])
h = ZZ(d['cofactor'])
assert G!=O and r*G==O
def digits(a):
    cs=[int(c) for c in a.polynomial().list()]
    return tuple(cs+[0]*(7-len(cs)))
setup_start=time.process_time_ns()
pts=[p for p in HyperellipticCurve(fk).rational_points() if p[2]!=0]
base=[]
idx={}
for pt in pts:
    xi,yi=pt[0],pt[1]
    if yi==0 or digits(yi)>digits(-yi):continue
    D=J(Rk([-xi,1]),Rk([yi]))
    if h*D==O:idx[xi]=None
    else:idx[xi]=len(base);base.append(D)
setup_cpu_s=float((time.process_time_ns()-setup_start)/1e9)
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
checkpoint=HERE/'relations_checkpoint.json'
if checkpoint.exists():
    prior=json.loads(checkpoint.read_text())
    assert prior['schema']=='ghs.odd5.twist-s7-relations/v1'
    assert prior['source_receipt_sha256']==hashlib.sha256(RECEIPT.read_bytes()).hexdigest()
    assert prior['factor_base_size']==len(base) and prior['r']==int(r)
    attempts=int(prior['attempts'])
    rows=prior['rows']
    rhs=prior['rhs']
    relation_cpu_s=float(prior['relation_cpu_s'])
else:
    attempts=0
    rows=[]
    rhs=[]
    relation_cpu_s=0.0
walk=attempts*G
start=time.process_time_ns()
for aa in range(attempts+1,limit+1):
    walk+=G
    row=smooth(walk)
    if row is not None:
        rows.append(row)
        rhs.append(int(h*aa%r))
    if aa%20000==0 or aa==limit:
        elapsed=float((time.process_time_ns()-start)/1e9)
        covered={j for row in rows for j,c in row}
        out={'schema':'ghs.odd5.twist-s7-relations/v1',
             'source_receipt_sha256':hashlib.sha256(RECEIPT.read_bytes()).hexdigest(),
             'q':int(k.cardinality()),'r':int(r),'h':int(h),
             'factor_base_size':len(base),'rational_points':len(pts),
             'attempts':aa,'accepted':len(rows),'covered_columns':len(covered),
             'setup_cpu_s_current_run':float(setup_cpu_s),
             'relation_cpu_s':float(relation_cpu_s+elapsed),
             'factor_base_points':[[list(digits(z)) for z in (D[0][0]*(-1),D[1][0])] for D in base],
             'rows':rows,'rhs':rhs}
        checkpoint.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
        print(json.dumps({key:out[key] for key in ('attempts','accepted','covered_columns','relation_cpu_s')},sort_keys=True,default=int),flush=True)
