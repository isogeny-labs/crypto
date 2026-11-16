#!/usr/bin/env python3
"""Retained-output metadata only; no research arithmetic or mathematical verifier."""
from pathlib import Path
import json, re, hashlib, subprocess, platform
h=Path(__file__).resolve().parent
def save(name,value): (h/name).write_text(json.dumps(value,indent=2)+'\n')
runs=[]
for rid,marker in [('color83-01','COLOR31'),('color284-01','COLOR31'),('cm31-01','CM31'),
                   ('reverse83-01','REVERSE31'),('reverse284-01','REVERSE31'),
                   ('frob83-01','FROB31'),('frob284-01','FROB31')]:
    p=h/'runs'/rid;log=(p/'native.log').read_text()
    assert (p/'exit.txt').read_text().strip()=='0'
    assert log.rstrip().endswith(marker+'_ALL_CHECKS_PASS')
    assert not re.search(r'^\s*\?',log,re.M)
    runs.append({'id':rid,'stage':'native','exit':0,'marker':marker+'_ALL_CHECKS_PASS',
                 'source_revision':(p/'source-revision.txt').read_text().strip()})
p=h/'runs/lean31-01'
assert (p/'compile.exit').read_text().strip()=='0'
assert (p/'audit.exit').read_text().strip()=='0'
audit=(p/'audit.log').read_text().rstrip().splitlines()[-1]
ma=re.fullmatch(r'audited (\d+) theorems in \[.*\]; 0 failing, 0 trusted \(native_decide\)',audit);assert ma
runs.append({'id':'lean31-01','stage':'lean','compile_exit':0,'audit_exit':0,'audited':int(ma[1]),
             'source_revision':(p/'source-revision.txt').read_text().strip()})
cm=json.loads((h/'runs/cm31-01/certificates/results.json').read_text())
carriers=[{k:z[k] for k in ('n','p','level','character_cover_genus','untwisted_E1_E0','diamond_twisted_E1_E0')}
          for z in cm['winners']]
closed=[]
pattern=r'FROB31_RESULT root=(\d+) twist=(\d+) factor=(\d+) factor_degree=(\d+) source_degree=(\d+) deck_closed_classes=(\d+) formal_support=(\d+)'
for d in (83,284):
    lines=(h/f'runs/frob{d}-01/native.log').read_text().splitlines()
    for i,line in enumerate(lines):
        m=re.fullmatch(pattern,line)
        if m:
            z=dict(zip(('root','twist','factor','factor_degree','source_degree','deck_closed_classes','formal_support'),map(int,m.groups())))
            assert lines[i+1]=='FROB31_CLASSES' and lines[i+3]=='FROB31_WEIGHTS'
            z.update({'kernel_field_degree':d,'classes_literal':lines[i+2],'weights_literal':lines[i+4]});closed.append(z)
save('results.json',{'schema':'koblitz.round31-results/v1','runs':runs,'carriers':carriers,
 'closed_point_records':closed,'native_runs_accepted':7,'native_failures':0,'lean_audited':int(ma[1]),
 'nonzero_formal_divisor_not_nonprincipal':True,'nonzero_target_class_proved':False,
 'performance_measured':False,'independent_scientific_replay':False})
save('protocol.json',{'schema':'koblitz.exact-computation-protocol/v1','round':31,
 'normative_record':['PROTOCOL.md','REVERSE31-PROTOCOL.md','FROB31-PROTOCOL.md'],
 'machine_index_created_after_runs':True,'generic_checker_available':False,
 'comparative_or_statistical_claim':False,'native_limits':{'virtual_memory_KiB':4194304,'timeout_seconds_each':1200},
 'lean_limits':{'version':'4.34.1','mathlib':'d13f23b723b8a846827a245b89c10fc7d3f11612','threads':1,'managed_memory_MiB':4096,'timeout_seconds_each':1200},
 'deviations':['CM31 base permutations checked internally, not serialized; retain source, cycles and Bezout chain'],
 'new_EKS_or_Diem_or_fifth_claim':False,'merge_authorized':False})
bins={'cm31':Path('/var/tmp/koblitz-r31-target/release/cm31'),
      'Singular':Path('/home/ec2-user/.cache/koblitz-native-cas/singular/bin/Singular')}
save('ENVIRONMENT.json',{'schema':'koblitz.environment/v1','boot_id':Path('/proc/sys/kernel/random/boot_id').read_text().strip(),
 'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
 'binary_sha256_recorded_after_execution':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},
 'CPU_timing_claim':False,'execution_times':'native per-run timestamps, not comparative metrics'})
print('Retained output metadata accepted: seven native attempts and structural Lean compile/audit.')
