#!/usr/bin/env python3
"""Independent complete pair-support check of stored incidence GCD outputs.

The support oracle uses ordinary affine group addition (one inverse per pair)
and finite public pool points, not the quotient/norm implementation.  Every
pair is visited, including after a hit, to verify the complete root support.
Fresh child interpreters measure memory without accumulated benchmark records.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time
from controls import add, neg, verify_witness

ROOT=Path(__file__).resolve().parent
F=Path('/workspace/ecdlp_research/prime_fixtures')
I=Path('/workspace/ecdlp_research/incidence/results')


def affine_support(fixture):
    c=fixture['curve'];p,a=c['p'],c['a']
    byx={}
    for k,e in enumerate(fixture['pools'][2]):
        byx.setdefault(e['x'],[]).append(k)
    support=set()
    for e0 in fixture['pools'][0]:
        u=e0['x'],e0['y']
        for e1 in fixture['pools'][1]:
            v=e1['x'],e1['y']
            for sign in (1,-1):
                q=add(p,a,u,v if sign==1 else neg(p,v))
                if q is None:
                    continue
                for k in byx.get(q[0],[]):
                    y=fixture['pools'][2][k]['y']
                    if q[1] in (y,-y%p):
                        support.add(k)
    return support


def main():
    os.sched_setaffinity(0,{2})
    begin=time.process_time();checks=[];mem=[]
    for path in sorted(F.glob('b*.json')):
        if path.name.endswith('.validation.json'):
            continue
        fixture=json.loads(path.read_text())
        if 'pools' not in fixture:
            continue
        support=affine_support(fixture)
        for arm in ('horner','blocked'):
            result_path=I/(fixture['id']+'_'+arm+'.json')
            if not result_path.exists():
                continue
            result=json.loads(result_path.read_text())
            coefficients=result['gcd_coefficients'];p=fixture['curve']['p']
            got=set()
            for k,e in enumerate(fixture['pools'][2]):
                value=0
                for c in reversed(coefficients):
                    value=(value*e['x']+c)%p
                if value==0:
                    got.add(k)
            assert got==support,(fixture['id'],arm,got,support)
            # All pools are squarefree, so the GCD degree equals root count.
            assert result['gcd_degree']==len(support)
            assert (result['status']=='relation')==bool(support)
            if result['status']=='relation':
                recovery=verify_witness(fixture,result['witness'])
                assert recovery['recovered_scalar_verified']
                assert all(recovery[k]==result['recovery'][k] for k in ('A','B','k'))
            assert result['fixture_sha256']==hashlib.sha256(path.read_bytes()).hexdigest()
            checks.append({'id':fixture['id'],'arm':arm,'status':result['status'],
                'gcd_degree':result['gcd_degree'],'all_roots_match_independent_pair_support':True})
        if fixture['kind']=='natural':
            # VmHWM belongs to the current executable address space; getrusage
            # maxrss can retain the parent's pre-exec RSS and is not used here.
            code="""import sys,json,pathlib
sys.path.insert(0,'/workspace/ecdlp_research/repo_controls')
from controls import solve_quotient_scan
f=json.loads(pathlib.Path(sys.argv[1]).read_text())
r=solve_quotient_scan(f)
hwm=next(line for line in pathlib.Path('/proc/self/status').read_text().splitlines() if line.startswith('VmHWM:'))
print(json.dumps({'id':f['id'],'B':f['B'],'status':r['status'],'vm_hwm_kib':int(hwm.split()[1]),'oracle_cpu_seconds':r['total_cpu_seconds']}))
"""
            trial=subprocess.run(['taskset','-c','2',sys.executable,'-c',code,str(path)],
                capture_output=True,text=True,check=True)
            mem.append(json.loads(trial.stdout))
    record={'schema':'repo-controls-incidence-parity-and-memory/v1',
        'checks':checks,'checked_incidence_outputs':len(checks),
        'independent_support_method':'Full signed pair enumeration using ordinary affine group law; no quotient/GCD/norm arithmetic',
        'fresh_memory_quotient':mem,'cpu_affinity':[2],
        'fresh_memory_scope':'one fresh Python/FLINT child per natural fixture; public input already constructed; VmHWM includes runtime and input but not pool generation',
        'timing_driver_memory_limitation':'pair_benchmark.json quotient peak_rss includes the timing driver and accumulated records; use fresh_memory_quotient for memory comparison',
        'source_sha256':{name:hashlib.sha256((ROOT/name).read_bytes()).hexdigest()
            for name in ('controls.py','pair_control.cpp','audit_incidence_and_memory.py')},
        'parent_cpu_seconds':time.process_time()-begin}
    (ROOT/'incidence_parity_and_memory.json').write_text(json.dumps(record,indent=2)+'\n')
    print(json.dumps({'checked_incidence_outputs':len(checks),'fresh_memory_cases':len(mem),
        'max_fresh_quotient_vm_hwm_kib':max(x['vm_hwm_kib'] for x in mem)}))


if __name__=='__main__':
    main()
