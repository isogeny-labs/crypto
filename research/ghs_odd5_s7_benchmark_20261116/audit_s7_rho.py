#!/usr/bin/env python3
"""Audit all saved same-target Frobenius-rho trials and summary statistics."""
from pathlib import Path
import hashlib
import json
import statistics

here=Path(__file__).resolve().parent
source=here/'s7_rho_v2.sage'
transfer=here/'s7_transfer.json'
summary_path=here/'rho_v2_summary.json'
summary=json.loads(summary_path.read_text())
source_hash=hashlib.sha256(source.read_bytes()).hexdigest()
transfer_hash=hashlib.sha256(transfer.read_bytes()).hexdigest()
trials=[]
for i in range(1,31):
    path=here/f'rho_v2_trial_{i:02d}.json'
    d=json.loads(path.read_text())
    assert d['schema']=='ghs.odd5.s7-rho-trial/v2'
    assert d['source_sha256']==source_hash
    assert d['transfer_receipt_sha256']==transfer_hash
    assert d['r']==1070455751 and d['frobenius_orbit_size']==7
    assert d['seed']==20261116700+i-1
    assert d['scalar']==123456789
    assert d['steps']>0 and d['cpu_s']>0
    trials.append(d)
assert summary['schema']=='ghs.odd5.s7-rho-summary/v2'
assert summary['source_sha256']==source_hash
assert summary['transfer_receipt_sha256']==transfer_hash
assert summary['trials']==len(trials)==30 and summary['all_scalars_verified']
times=[d['cpu_s'] for d in trials]
assert abs(statistics.median(times)-summary['median_trial_cpu_s'])<1e-9
assert abs(statistics.mean(times)-summary['mean_trial_cpu_s'])<1e-9
assert abs(summary['setup_cpu_s']+statistics.median(times)
           -summary['median_one_solve_cpu_s'])<1e-9
out={'schema':'ghs.odd5.s7-rho-audit/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'summary_sha256':hashlib.sha256(summary_path.read_bytes()).hexdigest(),
     'trials_checked':len(trials),
     'median_one_solve_cpu_s':summary['median_one_solve_cpu_s'],
     'median_steps':statistics.median(d['steps'] for d in trials),
     'all_saved_scalars_match_transfer_target':True}
target=here/'rho_v2_audit.json'
assert not target.exists(), 'refusing to overwrite audit receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps(out,sort_keys=True))
