"""Metadata only. Invoke with python -m research.autolab_cycle_20261115_v16.REGISTER_HELPER."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
from lab.runner import Laboratory

R=Path('/workspace/generalized_roots');C=R/'research/autolab_cycle_20261115_v18'
P=R/'lab/research_program.json'
PREVIOUS={'dual-obstruction-20261115-v6','frobenius52-20261115-v8',
          'full-quintic-block-20261115-v9','full-quintic-87-20261115-v10','odd-positive-20261115-v11','gold-projection-preparation-20261115-v14','joint-cubic-preparation-20261115-v15','moment-search-preparation-20261115-v16','moment-factor-search-preparation-20261115-v16','moment-factor-search-repair-preparation-20261115-v16','e2-fiber-preparation-20261115-v17','e2-fiber-repair-preparation-20261115-v17','e2-certificate-preparation-20261115-v17'}
INDEX='research/typed_ancestry_20261115_v7/INDEX.json'
INDEX_SHA='5f1a9793367033919218856c74e41cd1f48e6df3ae3807baa52ed11882a071c1'
BUILDER='research/typed_ancestry_20261115_v7/builder.py'
BUILDER_SHA='3e1d4b866ab2e2020c72c658730d515250f4173036c3f8ce6cf8f924be345177'
CASES=[{'package': 'e2_compression_20261115_v18', 'review': 'e2_compression_self_review_20261115_v18', 'eid': 'e2-compression-preparation-20261115-v18', 'inputs': ['research/joint_cubic_20261115_v15/raw.json.gz', 'research/advance/instances/n83-d18-m5-T001-offset0.json'], 'hypothesis': 'Matched32conditionalE2cells,3freshprocessreps of reference/cache/lean/rowbasis; avoid unusedprelinearG9 and exact compress scalar equations, fullkernel/syndromehashes identical, totalpaidtime/RSS; no genuineGroebner comparison or globalroots.'}]


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'),allow_nan=False).encode()
def put(path,value):
    with path.open('x')as stream:json.dump(value,stream,indent=2);stream.write('\n')

assert sha(R/INDEX)==INDEX_SHA and sha(R/BUILDER)==BUILDER_SHA
from research.typed_ancestry_20261115_v7.builder import semantic
index=json.loads((R/INDEX).read_text())
receipt=json.loads((R/INDEX).with_name('INDEX_RECEIPT.json').read_text())
assert receipt['index_sha256']==INDEX_SHA and receipt['source_sha256']==BUILDER_SHA
assert receipt['full_history'] and index['root_reached']
assert len(index['experiments'])==339 and len(index['epochs'])==35

lab=Laboratory(R/'lab/ledger').load(recover=False)
assert set(lab.state['experiments'])==PREVIOUS
assert all(item['status']in['passed','inconclusive']for item in lab.state['experiments'].values())
validation=lab.validate();assert validation['integrity']=='passed',validation
assert sha(P)==lab.state['program']['manifest_sha256'],'unregistered manifest edit'
program=json.loads(P.read_text())
assert program['requested_speed_target']==8 and program['release_policy']=='performance/GATE.md'
assert {spec['id']for spec in program['experiments']}==PREVIOUS
assert all(spec==lab.state['experiments'][spec['id']]['spec']for spec in program['experiments']),'old spec changed'
prefix_count=len(program['experiments']);assert prefix_count==13
prefix_sha=hashlib.sha256(canonical(program['experiments'])).hexdigest()
old_manifest_sha=sha(P)
free_bytes=os.statvfs(R).f_bavail*os.statvfs(R).f_frsize
assert free_bytes>=1400*1024,'insufficient declared metadata/evidence headroom'

new_specs=[];new_fingerprints=set();new_semantics=set();checks=[]
for case in CASES:
    package,review,eid=case['package'],case['review'],case['eid']
    assert eid not in PREVIOUS and eid not in {spec['id']for spec in new_specs}
    assert eid not in {old['id']for old in index['experiments']},'historical experiment id reused'
    pins=json.loads((R/'research'/review/'SOURCE_PINS.json').read_text())['sha256']
    sources={'lab/runner.py','performance_gate.py','performance/GATE.md'}
    for name,digest in pins.items():
        path=Path(name)if Path(name).is_absolute()else R/name
        assert sha(path)==digest,name
        if not Path(name).is_absolute():sources.add(name)
    for folder in [R/'research'/package,R/'research'/review]:
        sources.update(str(path.relative_to(R))for path in folder.iterdir()
            if path.is_file()and path.suffix in['.py','.md','.json']
            and not path.name.startswith(('raw','RESULT','RECORDED')))
    artifact='research/'+package+'/raw.json.gz'
    assert not(R/artifact).exists(),'fresh artifact already occupied'
    inputs=case['inputs']
    spec=dict(id=eid,hypothesis=case['hypothesis'],
        command=[sys.executable,'-m','research.'+package+'.worker',eid],
        source_paths=sorted(sources),input_paths=inputs,artifact_paths=[artifact],depends_on=[],
        timeout_seconds=90,memory_limit_mb=4096,cpu=0,
        expected_sha256={name:sha(R/name)for name in sorted(sources|set(inputs))})
    identity,fingerprint=lab.provenance(spec)
    semantic_sha=hashlib.sha256(canonical(semantic(spec,identity))).hexdigest()
    for old in index['experiments']:
        assert fingerprint not in old['fingerprints'],'historical runner duplicate'
        assert semantic_sha!=old['semantic_sha256'],'unchanged historical execution'
    for old in lab.state['experiments'].values():
        fingerprints={attempt['fingerprint']for attempt in old['attempts']}
        if old.get('registered_fingerprint'):fingerprints.add(old['registered_fingerprint'])
        assert fingerprint not in fingerprints,'active runner duplicate'
        assert old.get('registered_identity')is not None,'missing active semantic identity'
        assert semantic_sha!=hashlib.sha256(canonical(semantic(old['spec'],old['registered_identity']))).hexdigest(),'unchanged active execution'
    assert fingerprint not in new_fingerprints,'new-vs-new runner duplicate'
    assert semantic_sha not in new_semantics,'new-vs-new semantic duplicate'
    new_fingerprints.add(fingerprint);new_semantics.add(semantic_sha);new_specs.append(spec)
    checks.append(dict(id=eid,source_files=len(sources),semantic_sha256=semantic_sha,
                       runner_fingerprint=fingerprint,inputs=inputs,artifact=artifact))

assert len(new_specs)==1 and all(not spec['depends_on']for spec in new_specs)
put(C/'DUPLICATE_CHECK.json',dict(index_sha256=INDEX_SHA,historical_experiments_checked=339,
    historical_epochs=35,active_experiments_checked=13,new_specs_checked=1,
    new_vs_new_checked=True,duplicate=False,checks=checks,
    previous_passed_source_validation=validation,previous_prefix_sha256=prefix_sha,
    old_manifest_sha256=old_manifest_sha,free_bytes_before=free_bytes,
    changed_assumption='Changed from enumerating or certifying individual cells to matched cold-process scalar compiler comparison;13prioractive attempts preserved.'))
program['experiments'].extend(new_specs)
assert hashlib.sha256(canonical(program['experiments'][:prefix_count])).hexdigest()==prefix_sha
put(C/'PROGRAM_FROZEN.json',program)
assert not(C/'PROGRAM_BEFORE_REGISTRATION.json').exists()
os.rename(P,C/'PROGRAM_BEFORE_REGISTRATION.json');put(P,program)
resume=json.loads(subprocess.check_output([sys.executable,'-m','lab.runner','resume',
    '--ledger','lab/ledger','--manifest','lab/research_program.json'],cwd=R))
put(C/'RESUME_EXPERIMENT.json',resume)
print(json.dumps(dict(registered=[spec['id']for spec in new_specs],historical_checked=339,
    active_checked=13,new_vs_new_checked=True,old_prefix_preserved=True,
    frozen_manifest='research/autolab_cycle_20261115_v18/PROGRAM_FROZEN.json',
    scientific_executions_by_registration=0)))
