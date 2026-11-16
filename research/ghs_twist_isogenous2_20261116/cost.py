#!/usr/bin/env python3
"""Bind the new public target preprocessing to the fully audited common suffix."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
def read(path):return json.loads(path.read_text())
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
construct_path=HERE/'construct.json'
audit_path=HERE/'audit_manual.json'
pre_path=HERE/'preprocess.json'
attack_path=ROOT/'ghs_twist_s7_attack_20261116'/'s7_full_attack.json'
attack_audit_path=ROOT/'ghs_twist_s7_attack_20261116'/'audit_s7_full_attack.json'
rho_path=ROOT/'ghs_twist_s7_attack_20261116'/'rho_s7_result.json'
rho_audit_path=ROOT/'ghs_twist_s7_attack_20261116'/'audit_rho_s7.json'
search_path=ROOT/'ghs_s7_cost_amendment_20261116'/'cost_amendment.json'
rec=read(construct_path)
audit=read(audit_path)
pre=read(pre_path)
attack=read(attack_path)
attack_audit=read(attack_audit_path)
rho=read(rho_path)
rho_audit=read(rho_audit_path)
search=read(search_path)
assert pre['source_sha256']==digest(HERE/'preprocess.sage')
assert pre['construct_sha256']==digest(construct_path)
assert audit['construct_sha256']==digest(construct_path)
assert audit['rho_sha256']==digest(rho_path)
assert audit['status']=='PASS' and audit['rho_collision_transferred']
assert attack['recovered_scalar']==rho['solution']['scalar']==12345
assert attack['r']==rho['r']==rec['r']==audit['r']
assert attack_audit['target_scalar']==rho_audit['scalar']==12345
assert attack_audit['source_and_target_match'] and rho_audit['collision_group_equality']
assert search['attack_receipt_sha256']==digest(attack_path)
assert search['rho_receipt_sha256']==digest(rho_path)
def norm(P):return [list(v[:next((i+1 for i in range(len(v)-1,-1,-1) if v[i]),0)]) for v in P]
assert norm(pre['P'])==norm(attack['source_P'])
assert norm(pre['Q'])==norm(attack['source_Q'])
assert norm(pre['P'])==norm(rec['Pback'])
assert norm(pre['Q'])==norm(rec['Qback'])

new_pre=float(pre['cpu_seconds'])
ghs_old=float(attack['timing']['ghs_one_solve_cpu_s_stitched'])
rho_old=float(rho['one_solve_cpu_s'])
cover_search=float(search['cover_search_total_cpu_s'])
out={
  'schema':'ghs.odd5.isogenous2-cost/v1',
  'claim_type':'stitched same-target transport of audited GHS and rho suffixes',
  'construct_sha256':digest(construct_path),
  'manual_audit_sha256':digest(audit_path),
  'preprocess_sha256':digest(pre_path),
  'attack_sha256':digest(attack_path),
  'attack_audit_sha256':digest(attack_audit_path),
  'rho_sha256':digest(rho_path),
  'rho_audit_sha256':digest(rho_audit_path),
  'cover_search_sha256':digest(search_path),
  'preprocess_cpu_s':new_pre,
  'given_cover_ghs_suffix_cpu_s':ghs_old,
  'rho_suffix_cpu_s':rho_old,
  'new_source_given_cover_ghs_cpu_s':new_pre+ghs_old,
  'new_source_rho_cpu_s':new_pre+rho_old,
  'rho_over_ghs_given_cover':(new_pre+rho_old)/(new_pre+ghs_old),
  'fixed_carrier_targeted_cover_search_cpu_s':cover_search,
  'new_source_search_plus_ghs_cpu_s':new_pre+cover_search+ghs_old,
  'rho_over_search_plus_ghs':(new_pre+rho_old)/(new_pre+cover_search+ghs_old),
  'note':'Only the normalized mask-167-to-mask-30 preprocessing was newly timed. The GHS and rho suffixes are the previously audited exact P,Q workloads reached by that map; they were not rerun for mask 167. Cover-search timing uses external user+sys CPU rounded to 0.01s, while the other timings use in-process CPU. Carrier discovery and failed search arms are excluded.'
}
(HERE/'cost.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps(out,indent=2,sort_keys=True))
