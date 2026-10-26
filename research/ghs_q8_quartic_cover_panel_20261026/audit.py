#!/usr/bin/env python3
"""Fail closed on the panel's retained computations and input hashes."""

import hashlib
import json
from pathlib import Path

root=Path(__file__).resolve().parent

def receipt(name):
    row=json.loads((root/f'{name}_receipt.json').read_text())
    source=root/row['input_file']
    assert row['input_sha256']==hashlib.sha256(source.read_bytes()).hexdigest(), name
    assert row['http_status']==200 and row['response']['success'] is True, name
    assert row['response']['exit_code']==0 and row['response']['truncated'] is False, name
    assert 'Runtime error' not in row['response']['stdout'], name
    return row['response']['stdout']

groups={}
for n in (482,524,554,556):
    out=receipt(str(n))
    assert 'field $.1^3 + $.1 + 1' in out
    assert 'rational_points 9' in out and 'smooth true' in out
    assert 'base_automorphism_order 1' in out
    group=4 if n==554 else 1
    assert f'geometric_automorphism_order {group}' in out
    groups[n]=group

base=receipt('554_basechange')
assert 'extension_field_polynomial $.1^9 + $.1^4 + 1' in base
assert 'extension_points 636' in base and 'extension_automorphism_order 4' in base
failed=json.loads((root/'554_quotients_receipt.json').read_text())
assert failed['input_sha256']==hashlib.sha256((root/failed['input_file']).read_bytes()).hexdigest()
assert failed['response']['success'] is False
assert 'assert Dimension(VG) eq gG+3' in failed['response']['stdout']
ell=receipt('554_elliptic_quotient')
for phrase in ('quotient_smooth true','quotient_genus 1','quotient_points 554',
               'projection_regular true','source_b_order_isomorphic 105 554 true',
               'paired_source_generator (L.1^478 : L.1^433 : 1)',
               'paired_source_target (L.1^77 : L.1^119 : 1)',
               'paired_generator_fiber [ (L.1^164 : L.1^2 : 1), (L.1^262 : L.1^140 : 1) ]',
               'paired_target_fiber [ (L.1^328 : L.1^211 : 1), (L.1^109 : 1 : 0) ]',
               'paired_origin_fiber [ (L.1^54 : L.1^2 : 1), (L.1^75 : L.1^281 : 1) ]'):
    assert phrase in ell, phrase
two=receipt('554_two_isogenies')
edges={14:493,84:14,105:454,222:235,235:105,408:84,437:408,454:437,493:222}
for u,v in edges.items():
    assert f'edge {u} {v}' in two
paths=receipt('554_isogeny_paths')
assert paths.count('path [')==9
assert paths.count('edge_checked')==36
for n in edges:
    assert f'source {n} generator' in paths
invariants=json.loads((root/'explicit_invariants.json').read_text())
assert len(invariants['results'])==3
assert all(row['quotient_points']==554 for row in invariants['results'])
trace=json.loads((root/'trace_subgroup.json').read_text())
for key in ('pullback_nonzero','pullback_277_zero','trace_nonzero',
            'trace_277_zero','trace_target_equals_7G','trace_target_nonzero'):
    assert trace[key] is True, key
rho=json.loads((root/'rho_baseline.json').read_text())
assert rho['successes']==32 and all(row['answer']==7 for row in rho['trials'])
assert rho['median_operations']>0 and rho['median_seconds']>0
attack=json.loads((root/'full_attack.json').read_text())
assert attack['status']=='solved' and attack['answer']==7
assert attack['source_b']==105 and attack['prime']==277
assert attack['walk_steps']<=2000 and attack['factor_base_size']==8
assert attack['factor_base_point_count']==9 and attack['kernel_dimension']==1
assert attack['relation_count']==len(attack['relations'])
kernel=attack['kernel_vector']
assert len(kernel)==8 and any(v%277 for v in kernel)
assert all(len(row)==8 and sum(x*y for x,y in zip(row,kernel))%277==0
           for row in attack['relations'])
decomp=attack['decompositions']
assert set(decomp)=={'G','Q'} and all(len(row)==8 for row in decomp.values())
score_g=sum(x*y for x,y in zip(decomp['G'],kernel))%277
score_q=sum(x*y for x,y in zip(decomp['Q'],kernel))%277
assert score_g!=0 and score_q*pow(score_g,-1,277)%277==7
prior=json.loads((root/'prior_work.json').read_text())
assert [row['order'] for row in prior['candidates']]==[482,524,548,554,556]
assert all(row['exact_quartic_listed'] is False for row in prior['candidates'])
result={'schema':'ghs-q8-554-cover-panel-audit/v1',
        'geometric_group_orders':groups,'source_models':9,
        'all_isogeny_paths_checked':True,'quotient_point_counts':[554]*3,
        'trace_target_equals_7G':True,
        'trace_total_seconds':trace['total_seconds'],
        'attack_walk_steps':attack['walk_steps'],
        'attack_relation_count':attack['relation_count'],
        'attack_total_seconds':attack['total_seconds'],
        'rho_median_operations':rho['median_operations'],
        'rho_median_seconds':rho['median_seconds']}
(root/'audit_certificate.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
