#!/usr/bin/env sage
"""Independently audit the native mask-167 replay using Sage's isogeny constructor."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
RUN=HERE/'native_ghs167_result.json'
CODE=HERE/'solve167.sage'
REC=ROOT/'ghs_twist_isogenous2_20261116'/'construct.json'
OLD=ROOT/'ghs_twist_s7_attack_20261116'/'s7_full_attack.json'
RELS=ROOT/'ghs_twist_s7_attack_20261116'/'relations_checkpoint.json'
ANA=ROOT/'ghs_twist_s7_attack_20261116'/'relations_analysis.json'
SRC=ROOT/'ghs_twist_extension_20261116'/'twist_s7_transfer.json'
data=json.loads(RUN.read_text())
rec=json.loads(REC.read_text())
old=json.loads(OLD.read_text())
rel=json.loads(RELS.read_text())
analysis=json.loads(ANA.read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert data['source_sha256']==sha(CODE)
assert data['isogeny_construct_sha256']==sha(REC)
assert data['relations_sha256']==sha(RELS)
assert data['analysis_sha256']==sha(ANA)
assert data['source_transfer_sha256']==sha(SRC)
assert data['r']==rec['r']==old['r']
assert data['source_P167']==rec['P167'] and data['source_Q167']==rec['Q167']
assert data['mapped_P']==old['source_P'] and data['mapped_Q']==old['source_Q']
assert data['trace_G']==old['trace_G'] and data['trace_H']==old['trace_H']
assert data['relation_attempts']==rel['attempts']
assert data['accepted_relations']==rel['accepted']
assert data['relation_rank']==analysis['rank']==data['factor_base_size']
assert data['descent_offset']==old['descent_offset']
assert data['descent_row']==old['descent_row']
assert data['recovered_scalar']==12345
cost_keys=('source_setup_cpu_s','transfer_cpu_s','factor_base_cpu_s',
           'relation_collection_cpu_s','linear_algebra_cpu_s','descent_verify_cpu_s')
assert abs(sum(data['timing'][k] for k in cost_keys)-data['timing']['ghs_one_solve_cpu_s_stitched'])<1e-5

F3=GF(3)
Z=PolynomialRing(F3,'z');z=Z.gen()
L=GF(3^35,'w',modulus=Z(sage_eval(rec['field_modulus'],locals={'x':z},preparse=True)))
w=L.gen()
unpack=lambda cs:sum((L(c)*w^i for i,c in enumerate(cs)),L(0))
pack=lambda P:[[int(c) for c in z.polynomial().list()] for z in (P[0],P[1])]
a=unpack(rec['a_embedding'])
assert a^5+2*a+1==0
E167=EllipticCurve(L,[0,1,0,0,a^4+2*a+1])
E30=EllipticCurve(L,[0,1,0,0,2*(a+a^3)])
P167=E167(unpack(rec['P167'][0]),unpack(rec['P167'][1]))
Q167=E167(unpack(rec['Q167'][0]),unpack(rec['Q167'][1]))
P30=E30(unpack(data['mapped_P'][0]),unpack(data['mapped_P'][1]))
Q30=E30(unpack(data['mapped_Q'][0]),unpack(data['mapped_Q'][1]))
r=ZZ(data['r'])
assert is_prime(r) and r*P167==E167(0) and P167!=E167(0)
assert Q167==12345*P167 and r*P30==E30(0) and Q30==12345*P30

isos=E167.isogenies_prime_degree(2)
assert len(isos)==1
phi=isos[0]
assert phi.codomain().is_isomorphic(E30)
iso=phi.codomain().isomorphism_to(E30)
im=iso(phi(P167))
assert im==P30 or im==-P30
sgn=1 if im==P30 else -1
assert sgn*iso(phi(Q167))==Q30

out={'schema':'ghs.odd5.isogenous2-native-audit/v1',
     'native_run_sha256':sha(RUN),'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'status':'PASS','independent_isogeny_degree':2,'r':int(r),
     'recovered_scalar':data['recovered_scalar'],
     'ghs_cpu_seconds_with_saved_relations':data['timing']['ghs_one_solve_cpu_s_stitched']}
(HERE/'audit_native.json').write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps(out,sort_keys=True,default=int))
