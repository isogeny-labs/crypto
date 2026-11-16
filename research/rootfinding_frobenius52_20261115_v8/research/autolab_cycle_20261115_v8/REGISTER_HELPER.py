"""Metadata only: extend a preserved epoch using its verified historical index."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
from lab.runner import Laboratory

R=Path('/workspace/generalized_roots');C=R/'research/autolab_cycle_20261115_v8'
P=R/'lab/research_program.json'
PACKAGE='frobenius52_20261115_v8';REVIEW='frobenius52_review_20261115_v8'
EID='frobenius52-20261115-v8';PREVIOUS='dual-obstruction-20261115-v6'
INDEX='research/typed_ancestry_20261115_v7/INDEX.json'
INDEX_SHA='5f1a9793367033919218856c74e41cd1f48e6df3ae3807baa52ed11882a071c1'
BUILDER='research/typed_ancestry_20261115_v7/builder.py'
BUILDER_SHA='3e1d4b866ab2e2020c72c658730d515250f4173036c3f8ce6cf8f924be345177'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'),allow_nan=False).encode()
def put(path,value):
    with path.open('x')as stream:json.dump(value,stream,indent=2);stream.write('\n')

# Authenticate metadata before importing its reusable normalization function.
assert sha(R/INDEX)==INDEX_SHA
assert sha(R/BUILDER)==BUILDER_SHA
from research.typed_ancestry_20261115_v7.builder import semantic
index=json.loads((R/INDEX).read_text())
receipt=json.loads((R/INDEX).with_name('INDEX_RECEIPT.json').read_text())
assert receipt['index_sha256']==INDEX_SHA and receipt['source_sha256']==BUILDER_SHA
assert receipt['full_history'] and index['root_reached']
assert len(index['experiments'])==339 and len(index['epochs'])==35

# Do not reseal, reconstruct ancestors or rewrite an old state cache.
lab=Laboratory(R/'lab/ledger').load(recover=False)
assert set(lab.state['experiments'])=={PREVIOUS}
assert lab.state['experiments'][PREVIOUS]['status']=='passed'
validation=lab.validate();assert validation['integrity']=='passed',validation
assert sha(P)==lab.state['program']['manifest_sha256'],'unregistered manifest edit'
program=json.loads(P.read_text())
assert program['requested_speed_target']==8 and program['release_policy']=='performance/GATE.md'
assert {x['id']for x in program['experiments']}=={PREVIOUS}
assert program['experiments'][0]==lab.state['experiments'][PREVIOUS]['spec'],'old registered spec changed'
old_spec_sha=hashlib.sha256(canonical(program['experiments'][0])).hexdigest()

pins=json.loads((R/'research'/REVIEW/'SOURCE_PINS.json').read_text())['sha256']
sources={'lab/runner.py','performance_gate.py','performance/GATE.md'}
for name,digest in pins.items():
    path=Path(name)if Path(name).is_absolute()else R/name
    assert sha(path)==digest,name
    if not Path(name).is_absolute():sources.add(name)
for folder in [R/'research'/PACKAGE,R/'research'/REVIEW]:
    sources.update(str(path.relative_to(R))for path in folder.iterdir()
        if path.is_file()and path.suffix in['.py','.md','.json']
        and not path.name.startswith(('raw','RESULT','RECORDED')))
inputs=['research/advance/instances/n83-d18-m5-T001-offset0.json',
        'research/advance/n83_five_divisor_profile.json']
artifact='research/'+PACKAGE+'/raw.json.gz'
assert not(R/artifact).exists(),'fresh artifact already occupied'
spec=dict(id=EID,hypothesis='Evaluate the source-pinned Frobenius52 variant using independent small controls and its declared actual finite-field diagnostic. Conditional correctness or a compact representation never implies global all-roots completion or a qualified speed gain.',
    command=[sys.executable,'-m','research.'+PACKAGE+'.worker',EID],
    source_paths=sorted(sources),input_paths=inputs,artifact_paths=[artifact],depends_on=[],
    timeout_seconds=90,memory_limit_mb=4096,cpu=0,
    expected_sha256={name:sha(R/name)for name in sorted(sources|set(inputs))})
identity,fingerprint=lab.provenance(spec)
sem=semantic(spec,identity);semantic_sha=hashlib.sha256(canonical(sem)).hexdigest()
for old in index['experiments']:
    assert fingerprint not in old['fingerprints'],'historical runner duplicate'
    assert semantic_sha!=old['semantic_sha256'],'unchanged historical execution'
for old in lab.state['experiments'].values():
    fingerprints={a['fingerprint']for a in old['attempts']}
    if old.get('registered_fingerprint'):fingerprints.add(old['registered_fingerprint'])
    assert fingerprint not in fingerprints,'active runner duplicate'
    assert old.get('registered_identity')is not None,'missing active semantic identity'
    assert semantic_sha!=hashlib.sha256(canonical(semantic(old['spec'],old['registered_identity']))).hexdigest(),'unchanged active execution'

put(C/'DUPLICATE_CHECK.json',dict(index_sha256=INDEX_SHA,historical_experiments_checked=339,
    historical_epochs=35,active_experiments_checked=1,duplicate=False,
    previous_passed_source_validation=validation,previous_spec_sha256=old_spec_sha,
    new_semantic_sha256=semantic_sha,old_manifest_sha256=sha(P),
    changed_assumption='Fresh independently reviewed Frobenius52 mechanism; preserved dual-obstruction attempt is not repeated.'))
assert EID not in {x['id']for x in program['experiments']}
program['experiments'].append(spec)
assert hashlib.sha256(canonical(program['experiments'][0])).hexdigest()==old_spec_sha
put(C/'PROGRAM_FROZEN.json',program)
assert not(C/'PROGRAM_BEFORE_REGISTRATION.json').exists()
os.rename(P,C/'PROGRAM_BEFORE_REGISTRATION.json')
put(P,program)
resume=json.loads(subprocess.check_output([sys.executable,'-m','lab.runner','resume',
    '--ledger','lab/ledger','--manifest','lab/research_program.json'],cwd=R))
put(C/'RESUME_EXPERIMENT.json',resume)
print(json.dumps(dict(registered=EID,historical_checked=339,active_checked=1,
    previous_spec_preserved=True,source_files=len(sources),artifact=artifact,
    frozen_manifest='research/autolab_cycle_20261115_v8/PROGRAM_FROZEN.json',
    scientific_executions_by_registration=0)))
