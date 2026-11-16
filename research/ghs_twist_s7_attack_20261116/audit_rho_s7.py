"""Independently check the saved s=7 rho collision and recovered scalar."""
import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, PolynomialRing, ZZ, sage_eval

HERE=Path(__file__).resolve().parent
SRC=HERE.parent/'ghs_twist_extension_20261116'
out=json.loads((HERE/'rho_s7_result.json').read_text())
attack_path=HERE/'s7_full_attack.json'
attack=json.loads(attack_path.read_text())
src=json.loads((SRC/'twist_s7_transfer.json').read_text())
assert out['status']=='verified_solve'
assert out['script_sha256']==hashlib.sha256((HERE/'rho_s7.sage').read_bytes()).hexdigest()
assert out['attack_sha256']==hashlib.sha256(attack_path.read_bytes()).hexdigest()
F3=GF(3)
Z=PolynomialRing(F3,'z')
z=Z.gen()
L=GF(3**35,'w',modulus=Z(sage_eval(src['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
def v(cs):return sum((L(c)*w**i for i,c in enumerate(cs)),L(0))
a=v(src['a_embedding'])
E=EllipticCurve(L,[0,1,0,0,2*(a+a**3)])
P=E(v(attack['source_P'][0]),v(attack['source_P'][1]))
Q=E(v(attack['source_Q'][0]),v(attack['source_Q'][1]))
r=ZZ(out['r'])
assert r==attack['r'] and r*P==E(0) and r*Q==E(0)
lam=ZZ(out['eigenvalue'])
Fr=lambda T:E(T[0]**243,T[1]**243)
assert Fr(P)==lam*P and Fr(Q)==lam*Q and pow(int(lam),7,int(r))==1
assert out['source_P']==attack['source_P'] and out['source_Q']==attack['source_Q']
(aa0,bb0),(aa,bb)=out['solution']['collision_coefficients']
assert aa0*P+bb0*Q==aa*P+bb*Q
den=(bb-bb0)%r
assert den!=0
scalar=int((aa0-aa)*pow(int(den),-1,int(r))%r)
assert scalar==out['solution']['scalar']==attack['validation_scalar']
assert scalar*P==Q
assert out['one_solve_cpu_s']==out['setup_cpu_s']+out['trial_cpu_s']
assert out['steps']>=out['solution']['collision_steps'][1]
result={'schema':'ghs.odd5.twist-s7-rho-audit/v1',
        'collision_group_equality':True,'frobenius_eigenvalue_checked':True,
        'scalar':scalar,'steps':out['steps'],'one_solve_cpu_s':out['one_solve_cpu_s']}
(HERE/'audit_rho_s7.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps(result,sort_keys=True))
