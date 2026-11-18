"""Audit fixture provenance, exact counts, label construction and separation.

Curve counts use an independently written direct-cubic evaluator. Mixed
labels are replayed through the public group law. Validation scalar lookup
is a fixture truth oracle only; it is not an attack implementation.
"""
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

from generate import Curve, HERE, OUTPUT, dump, verify_fixture

os.sched_setaffinity(0,{3})
manifest=json.loads((OUTPUT/'manifest.json').read_text())
subprocess.run(['g++','-O3','-std=c++17',str(HERE/'audit_counts.cpp'),'-o',str(HERE/'audit_counts')],check=True)
rows=[]
for search in manifest['curve_searches']:
    rows.extend(search['curves'])
inputs=''.join(f"{row['p']} {row['a']} {row['b']} {row['r']}\n" for row in rows)
wall=time.perf_counter()
audited=subprocess.run([str(HERE/'audit_counts')],input=inputs,text=True,capture_output=True,check=True)
curve_audit=[json.loads(line) for line in audited.stdout.splitlines()]
curve_seconds=time.perf_counter()-wall
assert all(z['matches'] for z in curve_audit)
byB={}
checked=0;informative=0
for entry in manifest['fixtures']:
    data=json.loads(Path(entry['path']).read_text())
    val=json.loads(Path(entry['validation_path']).read_text())
    q=data['curve'];c=Curve(q['p'],q['a'],q['b'],q['r']);P=tuple(q['P']);Q=tuple(q['Q'])
    witness=verify_fixture(c,P,Q,data['pools'],val['secret_k'])
    assert (witness is not None)==val['signed_zero_sum_exists']
    if witness is not None:
        AA=BB=0
        for pool,index,sign in zip(data['pools'],witness['indices'],witness['signs']):
            AA+=sign*pool[index]['alpha'];BB+=sign*pool[index]['beta']
        AA%=c.r;BB%=c.r
        assert BB and (-AA*pow(BB,-1,c.r))%c.r==val['secret_k']
        informative+=1
    if data['kind']=='natural':
        assert data['generation']['secret_used_for_pool_generation'] is False
        s=byB.setdefault(data['B'],{'count':0,'positives':0,'misses':0,'generation_cpu':[],'draws':[],'ratios':[]})
        s['count']+=1;s['positives']+=int(witness is not None);s['misses']+=int(witness is None)
        s['generation_cpu'].append(data['generation']['cpu_seconds'])
        s['draws'].append(data['generation']['draws']);s['ratios'].append(data['r_over_B_cubed'])
    entry['sha256']=hashlib.sha256(Path(entry['path']).read_bytes()).hexdigest()
    entry['validation_sha256']=hashlib.sha256(Path(entry['validation_path']).read_bytes()).hexdigest()
    checked+=1
summary=[]
for B,s in sorted(byB.items()):
    summary.append({'B':B,'natural_instances':s['count'],'signed_positives':s['positives'],'signed_misses':s['misses'],
                    'median_public_generation_cpu_seconds':statistics.median(s['generation_cpu']),
                    'median_public_draws':statistics.median(s['draws']),
                    'r_over_B_cubed_min':min(s['ratios']),'r_over_B_cubed_max':max(s['ratios'])})
separation=[]
for path in manifest['structured_controls']:
    data=json.loads(Path(path).read_text())
    assert not data['mixed_labels_available']
    assert all('alpha' not in z and 'beta' not in z for z in data['points'])
    item={'id':data['id'],'nominal_B':data['nominal_B'],'actual_unsigned_points':len(data['points']),
          'label_acquisition_measured': 'charged_label_acquisition' in data,
          'expected_uniform_draws_per_accepted_point':data['expected_uniform_public_draws_per_accepted_point']}
    if 'charged_label_acquisition' in data:
        measured=data['charged_label_acquisition'];assert measured['complete']
        item['actual_label_acquisition_draws']=measured['draws']
        item['actual_label_acquisition_cpu_seconds']=measured['cpu_seconds']
    separation.append(item)
audit={'schema':'prime-pools-audit/v1','scope':'Exact small-instance input audit. Separate C++ curve-count path; public group-law label replay. Not an independent implementation of the group law or a certified cryptanalytic benchmark.',
       'status':'passed','cpu_affinity':[3],'curve_count_audit':curve_audit,'curve_count_audit_wall_seconds':curve_seconds,
       'fixture_files_checked':checked,'positive_fixture_first_reference_witnesses_informative':informative,
       'natural_summary':summary,'structured_label_separation':separation,
       'counter_scope':'Point-generation counts are exact at the explicit Python group/field API; pow inverse internal arithmetic, RNG and hash operations are included in CPU time but not expanded into field-multiply counts.',
       'control_scope':'15 planted controls use the validation secret to relabel a forced relation informatively. 15 negative controls alias existing exact-negative natural cases. Neither is an additional independent natural observation.',
       'timing_scope':'CPU-pinned single-thread exploratory shared-host measurements; no isolation guarantee or runtime/exponent claim.',
       'source_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [HERE/'generate.py',HERE/'count_curves.cpp',HERE/'audit_counts.cpp',HERE/'add_controls.py',Path(__file__)]}}
dump(HERE/'audit_results.json',audit)
manifest['audit_results']=str(HERE/'audit_results.json')
manifest['control_scope']=audit['control_scope']
manifest['fixture_count']=len(manifest['fixtures'])
dump(OUTPUT/'manifest.json',manifest)
print(json.dumps({'status':'passed','fixture_files_checked':checked,'independently_recounted_curves':len(curve_audit),'natural_summary':summary,'structured_measured':[z for z in separation if z['label_acquisition_measured']]},indent=2))
