"""Preregister a changed mechanism with cross-epoch duplicate guards."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
from lab.runner import Laboratory
R=Path('/workspace/generalized_roots');C=R/'research/autolab_cycle_20261115_v6';P=R/'lab/research_program.json'
package='dual_obstruction_20261115_v6';review='dual_obstruction_review_20261115_v6';eid='dual-obstruction-20261115-v6'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def put(p,x):
    with p.open('x')as f:json.dump(x,f,indent=2);f.write('\n')
pins=json.loads((R/'research'/review/'SOURCE_PINS.json').read_text())['sha256'];sources={'lab/runner.py','performance_gate.py','performance/GATE.md'}
for n,h in pins.items():
    p=Path(n)if Path(n).is_absolute()else R/n;assert sha(p)==h,n
    if not Path(n).is_absolute():sources.add(n)
for d in [R/'research'/package,R/'research'/review]:
    sources.update(str(p.relative_to(R))for p in d.iterdir()if p.suffix in ['.py','.md','.json']and not p.name.startswith(('raw','RESULT','RECORDED')))
inputs=['research/advance/instances/n83-d18-m5-T001-offset0.json','research/advance/n83_five_divisor_profile.json']
spec=dict(id=eid,hypothesis='Exact odd-XOR zero combinations of actual shared73bit support evaluations may refute the constant-functional certificate CLASS without symbolic ANF expansion; samples never reject parameter cells or prove identities.',command=[sys.executable,'-m','research.'+package+'.worker',eid],source_paths=sorted(sources),input_paths=inputs,artifact_paths=['research/'+package+'/raw.json.gz'],depends_on=[],timeout_seconds=90,memory_limit_mb=4096,cpu=0,expected_sha256={n:sha(R/n)for n in sorted(sources|set(inputs))})
lab=Laboratory(R/'lab/ledger').load(recover=False);identity,fingerprint=lab.provenance(spec)
def semantic(s,i):
    cmd=[('<id>'if a==s['id']else a)for a in s['command']]
    if '-m'in cmd:cmd[cmd.index('-m')+1]='<module>'
    files=i['files_sha256'];code=sorted(h for n,h in files.items()if Path(n).suffix in['.py','.cpp','.hpp','.rs'])
    return dict(command=cmd,code=code,inputs=sorted(files[n]for n in s['input_paths']),timeout=s['timeout_seconds'],memory=s['memory_limit_mb'],cpu=s['cpu'])
sem=semantic(spec,identity);checked=0;epochs=[];program=json.loads(P.read_text());entry=program.get('sealed_parent');seen=set()
while entry:
    name=entry['sealed_path'];assert name not in seen;seen.add(name);d=R/name
    if entry.get('state_sha256'):assert sha(d/'state.json')==entry['state_sha256']
    old=json.loads((d/'state.json').read_text());epochs.append(name)
    for item in old['experiments'].values():
        checked+=1;assert item.get('registered_fingerprint')!=fingerprint,'duplicate historical fingerprint'
        if item.get('registered_identity'):assert semantic(item['spec'],item['registered_identity'])!=sem,'unchanged historical algorithm/input execution'
    entry=old['program'].get('sealed_parent')
for item in lab.state['experiments'].values():
    assert item.get('registered_fingerprint')!=fingerprint
    if item.get('registered_identity'):assert semantic(item['spec'],item['registered_identity'])!=sem
put(C/'DUPLICATE_CHECK.json',dict(historical_epochs=epochs,historical_experiments_checked=checked,duplicate=False,semantic_id_and_module_renames_ignored=True,changed_assumption='Numeric exact dual certificate-class obstruction replaces symbolic support expansion; not an unchanged retry.'))
assert eid not in [x['id']for x in program['experiments']];program['experiments'].append(spec);put(C/'PROGRAM_FROZEN.json',program)
assert not(C/'PROGRAM_BEFORE_REGISTRATION.json').exists()
os.rename(P,C/'PROGRAM_BEFORE_REGISTRATION.json');put(P,program)
put(C/'RESUME_EXPERIMENT.json',json.loads(subprocess.check_output([sys.executable,'-m','lab.runner','resume','--ledger','lab/ledger','--manifest','lab/research_program.json'],cwd=R)))
print(json.dumps(dict(registered=eid,historical_checked=checked,artifact=spec['artifact_paths'][0])))
