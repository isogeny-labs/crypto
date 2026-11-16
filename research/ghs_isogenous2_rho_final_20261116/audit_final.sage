#!/usr/bin/env sage
"""Audit the terminal native rho collision and paired native GHS cost."""
import hashlib
import json
from decimal import Decimal
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
RHO=HERE/'rho167_s7_result.json'
RHO_ORIG=ROOT/'ghs_twist_isogenous2_20261116'/'rho167_s7_result.json'
RHO_CODE=ROOT/'ghs_twist_isogenous2_20261116'/'rho167_s7.sage'
CONSTRUCT=ROOT/'ghs_twist_isogenous2_20261116'/'construct.json'
GHS=ROOT/'ghs_isogenous2_native_20261116'/'native_ghs167_result.json'
GHS_AUDIT=ROOT/'ghs_isogenous2_native_20261116'/'audit_native.json'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(RHO)==sha(RHO_ORIG)
rho=json.loads(RHO.read_text())
rec=json.loads(CONSTRUCT.read_text())
ghs=json.loads(GHS.read_text())
ghs_audit=json.loads(GHS_AUDIT.read_text())
assert rho['status']=='verified_solve'
assert rho['script_sha256']==sha(RHO_CODE)
assert rho['attack_sha256']==sha(CONSTRUCT)
assert rho['r']==rec['r']==ghs['r']
assert rho['source_P']==rec['P167']==ghs['source_P167']
assert rho['source_Q']==rec['Q167']==ghs['source_Q167']
assert ghs_audit['native_run_sha256']==sha(GHS)
assert ghs['recovered_scalar']==rho['solution']['scalar']==12345
assert rho['steps']==rho['solution']['collision_steps'][1]
assert 0<=rho['solution']['collision_steps'][0]<rho['steps']
assert abs(rho['setup_cpu_s']+rho['trial_cpu_s']-rho['one_solve_cpu_s'])<1e-6

F3=GF(3)
Z=PolynomialRing(F3,'z');z=Z.gen()
L=GF(3^35,'w',modulus=Z(sage_eval(rec['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
v=lambda cs:sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
a=v(rec['a_embedding'])
assert a^5+2*a+1==0
E=EllipticCurve(L,[0,1,0,0,a^4+2*a+1])
P=E(v(rec['P167'][0]),v(rec['P167'][1]))
Q=E(v(rec['Q167'][0]),v(rec['Q167'][1]))
r=ZZ(rho['r'])
assert is_prime(r) and r*P==E(0) and P!=E(0)
assert Q==12345*P and r*Q==E(0)
lam=ZZ(rho['eigenvalue'])
assert lam!=1 and pow(lam,7,r)==1
assert E(P[0]^243,P[1]^243)==lam*P
assert E(Q[0]^243,Q[1]^243)==lam*Q
(a0,b0),(a1,b1)=rho['solution']['collision_coefficients']
assert ZZ(a0)*P+ZZ(b0)*Q==ZZ(a1)*P+ZZ(b1)*Q
den=(b1-b0)%r
assert den!=0
recovered=int(((a0-a1)*inverse_mod(den,r))%r)
assert recovered==rho['solution']['scalar'] and recovered*P==Q

G=Decimal(str(ghs['timing']['ghs_one_solve_cpu_s_stitched']))
R=Decimal(str(rho['one_solve_cpu_s']))
assert G>Decimal('0') and R>G
out={'schema':'ghs.odd5.isogenous2-final-audit/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'rho_receipt_sha256':sha(RHO),'ghs_receipt_sha256':sha(GHS),
     'status':'PASS','r':int(r),'scalar':recovered,
     'rho_steps':rho['steps'],'rho_cpu_seconds':str(R),
     'ghs_cpu_seconds':str(G),'rho_over_ghs':str(R/G),
     'copied_receipt_matches_original':True,'collision_verified_on_source':True}
(HERE/'audit_final.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
