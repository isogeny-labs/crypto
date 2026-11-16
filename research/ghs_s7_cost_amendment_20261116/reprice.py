#!/usr/bin/env python3
"""Amend s7 cost accounting with the saved targeted cover-search run."""
import hashlib
import json
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
OLD=HERE.parent/'ghs_odd5_carrier_20261115'
S7=HERE.parent/'ghs_twist_s7_attack_20261116'
search_source=OLD/'degree7_quadratic_square.sage'
search_result=OLD/'degree7_quadratic_square_30.json'
search_stderr=OLD/'degree7_quadratic_square_30.stderr'
attack_path=S7/'s7_full_attack.json'
rho_path=S7/'rho_s7_result.json'
search=json.loads(search_result.read_text())
attack=json.loads(attack_path.read_text())
rho=json.loads(rho_path.read_text())
assert search['bmask']==30 and len(search['hits'])==1
assert search['source_sha256']==hashlib.sha256(search_source.read_bytes()).hexdigest()
m=re.search(r'^\s*([0-9.]+) real\s+([0-9.]+) user\s+([0-9.]+) sys',search_stderr.read_text(),re.M)
assert m
search_wall,search_user,search_system=map(float,m.groups())
search_cpu=search_user+search_system
assert (search_wall,search_user,search_system)==(216.77,186.99,2.24)
solve_cpu=attack['timing']['ghs_one_solve_cpu_s_stitched']
rho_cpu=rho['one_solve_cpu_s']
assert attack['recovered_scalar']==rho['solution']['scalar']==12345
assert abs(solve_cpu-231.125131)<1e-6 and abs(rho_cpu-3268.034356)<1e-6
total=search_cpu+solve_cpu
out={'schema':'ghs.odd5.s7-targeted-cover-cost-amendment/v1',
     'source_mask':30,'twist_source':'negative trace quadratic twist',
     'cover_search_shape':'degree-seven quadratic-square fixed-carrier search',
     'cover_search_wall_s':search_wall,
     'cover_search_user_cpu_s':search_user,
     'cover_search_system_cpu_s':search_system,
     'cover_search_total_cpu_s':search_cpu,
     'given_cover_ghs_solve_cpu_s':solve_cpu,
     'targeted_search_plus_solve_cpu_s':total,
     'rho_same_target_cpu_s':rho_cpu,
     'rho_over_given_cover_ghs':rho_cpu/solve_cpu,
     'rho_over_search_plus_solve':rho_cpu/total,
     'search_source_sha256':hashlib.sha256(search_source.read_bytes()).hexdigest(),
     'search_receipt_sha256':hashlib.sha256(search_result.read_bytes()).hexdigest(),
     'search_stderr_sha256':hashlib.sha256(search_stderr.read_bytes()).hexdigest(),
     'attack_receipt_sha256':hashlib.sha256(attack_path.read_bytes()).hexdigest(),
     'rho_receipt_sha256':hashlib.sha256(rho_path.read_bytes()).hexdigest(),
     'scope':'one fixed carrier, one targeted degree-seven search, then the saved s7 solve; carrier discovery and other failed searches not charged',
     'timing_note':'cover search CPU comes from external time user+sys rounded to 0.01s; GHS and rho solve CPUs are in-process and exclude interpreter startup'}
(HERE/'cost_amendment.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps({k:v for k,v in out.items() if not k.endswith('sha256')},sort_keys=True))
