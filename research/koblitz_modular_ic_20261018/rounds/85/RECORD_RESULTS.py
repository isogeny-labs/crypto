"""Verify frozen R85 native and Lean attempts, source pins and donor bytes."""
from datetime import datetime
from pathlib import Path
import hashlib,json,re,subprocess

h=Path(__file__).resolve().parent
repo=h.parents[3]
native=h/'runs/diamond-base85-01'
lean=h/'runs/diamond-lean85-01'
assert sorted(p.name for p in (h/'runs').iterdir() if p.is_dir())==['diamond-base85-01','diamond-lean85-01']
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
pins=[]
def pin(raw,rev,local,source):
    path=str((h/source).relative_to(repo))
    assert subprocess.check_output(['git','show',rev+':'+path],cwd=repo)==(raw/local).read_bytes()
    pins.append({'attempt':raw.name,'artifact':local,'git_source':path,'revision':rev,'sha256':digest(raw/local)})
nrev=(native/'source-revision.txt').read_text().strip()
log=(native/'native.log').read_text().splitlines()
expected='DIA85_RESULT first_open_product=1 second_open_numerator=1 transformed_equation_lift=1 corrupted_numerator_rejected=1 base_roundtrip_bridge=1'
assert int((native/'exit.txt').read_text())==0
assert log.count(expected)==1 and log.count('DIAMOND85_ALL_CHECKS_PASS')==1
assert not [s for s in log if re.match(r'^\s*\?',s)]
for local,source in [('input.sing','DIAMOND85.sing'),('driver.sh','NATIVE_DRIVER.sh'),
                     ('launcher.sh','REPLAY_CAS.sh'),('PROTOCOL.md','PROTOCOL.md'),
                     ('protocol.json','protocol.json'),('donors.json','donors.json')]:
    pin(native,nrev,local,source)
for donor in json.loads((native/'donors.json').read_text()):
    p=h/donor['path']
    assert p.stat().st_size==donor['bytes'] and digest(p)==donor['sha256']
lrev=(lean/'source-revision.txt').read_text().strip()
assert (lean/'compile.exit').read_text().strip()=='0'
assert (lean/'audit.exit').read_text().strip()=='0'
assert (lean/'mathlib-revision.txt').read_text().strip()=='d13f23b723b8a846827a245b89c10fc7d3f11612'
audit=(lean/'audit.log').read_text()
for name in ['first_open_product','second_open_numerator','overlap_relation']:
    assert 'ok    Diamond85.'+name+'  ' in audit
assert 'audited 3 theorems in [Diamond85]; 0 failing, 0 trusted (native_decide)' in audit
assert not any(s.startswith(('FAIL  ','TRUST ')) for s in audit.splitlines())
for local,source in [('driver.sh','LEAN_DRIVER.sh'),('launcher.sh','REPLAY_LEAN.sh'),
                     ('Diamond85.lean','lean/Diamond85.lean'),('LEAN-PROTOCOL.md','LEAN-PROTOCOL.md')]:
    pin(lean,lrev,local,source)
assert subprocess.check_output(['git','show',lrev+':tools/lean/AuditAxioms.lean'],cwd=repo)==(lean/'AuditAxioms.lean').read_bytes()
pins.append({'attempt':lean.name,'artifact':'AuditAxioms.lean','git_source':'tools/lean/AuditAxioms.lean','revision':lrev,'sha256':digest(lean/'AuditAxioms.lean')})
start=(native/'started-utc.txt').read_text().strip();finish=(native/'finished-utc.txt').read_text().strip()
artifacts=lambda raw:[{'path':str(p.relative_to(h)),'sha256':digest(p),'bytes':p.stat().st_size} for p in sorted(raw.iterdir()) if p.is_file()]
results={'schema':'koblitz.results/v1','round':85,'runs':[
    {'attempt':native.name,'status':'accepted','kind':'Singular','source_revision':nrev,'native_exit':0,'marker':'DIAMOND85_ALL_CHECKS_PASS','error_count':0,'result_line':expected,'started_utc':start,'finished_utc':finish,'native_wall_seconds':(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds(),'artifacts':artifacts(native)},
    {'attempt':lean.name,'status':'accepted','kind':'Lean','source_revision':lrev,'compile_exit':0,'audit_exit':0,'theorems':3,'failing':0,'trusted':0,'artifacts':artifacts(lean)}],
    'input_pins':pins,'donor_hashes':len(json.loads((h/'donors.json').read_text()))}
assert len(pins)==11 and results['donor_hashes']==5
(h/'results.json').write_text(json.dumps(results,indent=2)+'\n')
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R85-FULL-U0-FIBER','claim_class':'exact_computation',
        'protocols':[{'attempt':native.name,'path':'runs/'+native.name+'/protocol.json','sha256':digest(native/'protocol.json'),'frozen_revision':nrev}],
        'raw_archive':'runs.tar.xz; content address in .artifacts.json after custody publication',
        'results':{'path':'results.json','sha256':digest(h/'results.json')},
        'flow':{'accepted_native':1,'accepted_lean':1,'rejected':0,'interrupted':0},
        'canonical_schema_validation':'unavailable in selected Git index and disk; manual contract/source-byte checks only',
        'notes':'Quadratic-base diamond principal-open products and transformed equation pass; upper rank-32 generator transport remains.'}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n')
print(json.dumps({'native_accepted':1,'lean_accepted':1,'source_byte_pins':11,'donor_hashes':5}))
