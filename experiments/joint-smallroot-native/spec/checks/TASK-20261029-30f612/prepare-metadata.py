#!/usr/bin/env python3
"""Task-local control metadata only. Hashing uses sha256sum or the existing native seal.

Never performs cryptography, scientific/harness computation, advice, or a case.
This script is single-use: exclusive output creation and a native fresh-directory
guard preserve every partial outcome and prevent a second seal invocation.
"""
import datetime
import json
import os
from pathlib import Path
import shutil
import subprocess

OWNER = Path('/workspace/crypto-native-smallroot')
PROGRAMME = Path('/workspace/crypto-autoresearcher')
BASE = PROGRAMME / 'coordination/goals/GOAL-PFDR-fd09e0/batches/BATCH-ee839d/continuations/DEC-20261029-03e304'
OUT = OWNER / 'experiments/joint-smallroot-native/spec/checks/TASK-20261029-30f612'
LEASE = Path('/workspace/smallroot-session/20261029-prep-current-lease.json')
GIT = ['git', '-c', 'gc.auto=0', '-c', 'maintenance.auto=false']

def now():
    return datetime.datetime.now(datetime.timezone.utc)

def run(argv, cwd=None):
    return subprocess.check_output(argv, cwd=cwd, text=True)

def sha(path):
    return run(['sha256sum', '--', str(path)]).split()[0]

def put(name, value):
    path = OUT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x') as f:
        f.write(json.dumps(value, indent=2) + '\n' if not isinstance(value, str) else value)

def info(path, expected=None):
    path = Path(path)
    st = path.stat()
    digest = sha(path)
    if expected is not None:
        assert digest == expected, (str(path), digest, expected)
    return dict(path=str(path), realpath=str(path.resolve()), bytes=st.st_size,
                sha256=digest, uid=st.st_uid, device=st.st_dev, inode=st.st_ino,
                mtime_ns=st.st_mtime_ns, expected_sha256=expected,
                expected_match=None if expected is None else True)

def lease():
    x = json.loads(LEASE.read_text())
    t = now()
    assert x['task_id'] == 'e103ff3e-29b7-4d66-bd94-c576027e9cc7'
    assert x['lease_id'] == 'c0965d3a-854f-45a6-99d7-d7c64ac93cf9'
    assert x['live'] and x['expected_principal'] and x['fencing_epoch'] == 1
    assert x['released_at'] is None
    assert datetime.datetime.fromisoformat(x['expires_at']) > t
    assert 0 <= (t-datetime.datetime.fromisoformat(x['observed_at'])).total_seconds() < 75
    return x

assert os.getuid() == 1000
proof = lease()
claim_path = BASE / 'claims/TASK-20261029-30f612.1.claim.json'
claim = json.loads(claim_path.read_text())
assert claim['owner'] == 'executor-native-q4prep-30f612' and claim['session'] == 'native-q4prep-20261029'
assert claim['epoch'] == 1 and datetime.datetime.fromisoformat(claim['expires_at'].replace('Z','+00:00')) > now()
assert not (BASE / 'claims/TASK-20261029-30f612.1.release.json').exists()
admission = BASE / 'admission/TASK-20261029-30f612/preparation-admission.json'
admission_info = info(admission, '2313d2447253655a7588a8a641397c9dd46cd8ef5d45d4350653007574041b78')
assert run(GIT+['rev-parse', 'HEAD'], OWNER).strip() == '12c2624c7dcbce932ef26828ab292084a8c340da'
snapshot = 'e6d1de8f1a384f309e52990b1c9682cbd20cb744'
relative_admission = str(admission.relative_to(PROGRAMME))
assert subprocess.check_output(GIT+['show', snapshot+':'+relative_admission], cwd=PROGRAMME) == admission.read_bytes()
dispatch_argv = ['python3','-B','tools/research_dispatch.py',str(BASE/'dispatch_queue.json'),'--ready-only','--claims','refs','--now',now().strftime('%Y-%m-%dT%H:%M:%SZ')]
dispatch = run(dispatch_argv, PROGRAMME)
assert 'ready: 1 task(s)' in dispatch and '\tclaim=executor-native-q4prep-30f612\t' in dispatch
put('dispatch-readiness.txt', dispatch)
put('preparation-authority.json', dict(recorded_at=now().isoformat(), admission=admission_info,
    snapshot_commit=snapshot, snapshot_bytes_verified=True, programme_head=run(GIT+['rev-parse','HEAD'],PROGRAMME).strip(),
    publication_basis='Root verified/pushed exact snapshot; queue binding795f1bb752 and completed9df release ecf190479379d793eb0b7757018d0d7debcc545a published before dispatch.',
    dispatch_argv=dispatch_argv, claim=claim, claim_file=info(claim_path), conductor=proof,
    conductor_scopecheck='Root authenticated check exit0/block_duplicate with explicit ATTACH to own exactT5; not relabeled allow; no bypass.',
    owner_base_commit='12c2624c7dcbce932ef26828ab292084a8c340da',
    owner_current_origin_main='992c2579e2891c1c58ee70183888d3a9ce1d32e4',
    owner_origin_main_scope='Root fresh fetch reports noAGENTS/report-evidence/protectednative delta from a685; no source change required.',
    managed_model='gpt-6.1-sol', reasoning_effort='high', managed_selection='actually selected by parent native session',
    session='native-q4prep-20261029', API_model_probe='unverified operator-supplied; no probe', fallback_used=False))

plan = json.loads((BASE/'cache-preparation-plan.json').read_text())
bindings = json.loads((BASE/'source-bindings.json').read_text())
sources = []
for relative, expected in bindings['protected_src_sha256'].items():
    sources.append(info(OWNER/relative, expected))
    blob = subprocess.check_output(GIT+['show','86ddd19f67e08160c22a0d8ba9a32a92321d18ad:'+relative],cwd=OWNER)
    assert blob == (OWNER/relative).read_bytes()
for entry in bindings['original_C1_driver_worker_sources']:
    relative=entry['path']
    dest=OUT/'source/original-C1'/Path(relative).name
    dest.parent.mkdir(parents=True,exist_ok=True)
    with dest.open('xb') as f:
        f.write(subprocess.check_output(GIT+['show','204735c3f6f3144911df58a6ece7699c0e74d4ad:'+relative],cwd=OWNER))
    sources.append(info(dest,entry['sha256']))
helper=plan['native_helper']
sources.append(info(OWNER/helper['source_owner_path'],helper['source_sha256']))
shutil.copyfile(OWNER/helper['source_owner_path'],OUT/'source/cache-preparation.cpp')
sources.append(info(OWNER/'experiments/joint-smallroot-native/spec/guard-qualification-runtime.sh','77781fea2591c9a9dd9dd63568c29f78dbe8aedcadddb064b8dfe534126e92f6'))
build=json.loads((OWNER/'experiments/joint-smallroot-native/spec/checks/TASK-20261026-1525ac/build-location.json').read_text())
toolchain=[info(x['path'],x['sha256']) for x in build['toolchain']]
binary_entries=bindings['original_runtime_lock']['binaries']+[dict(path=helper['path'],sha256=helper['sha256'])]
binaries=[info(x['path'],x['sha256']) for x in binary_entries]
host_libraries=[info(x['path'],x['sha256']) for x in build['actual_host_dependencies']]
image_libraries=[]
for entry in bindings['original_runtime_lock']['pinned_image_libraries']:
    x=info(Path('/workspace/native-smallroot-guard-build-6374dd/image-libraries')/Path(entry['image_path']).name,entry['sha256'])
    x['immutable_image_path']=entry['image_path']
    x['scope']='Retained original image-library bytes; current image identity independently inspected. No fresh container/extraction.'
    image_libraries.append(x)
elf=[]
for entry in binary_entries:
    name=Path(entry['path']).name
    headers=run(['readelf','-l',entry['path']])
    dynamic=run(['readelf','-d',entry['path']])
    put('runtime-metadata/'+name+'.interpreter.txt','\n'.join(x for x in headers.splitlines() if 'interpreter' in x)+'\n')
    put('runtime-metadata/'+name+'.dynamic.txt',dynamic)
    elf.append(dict(path=entry['path'],inspection='readelf metadata only; no binary invocation'))
put('source-binary-library-manifest.json',dict(schema='q4-source-binary-library-manifest-v1',
    recorded_at=now().isoformat(),owner_base_commit='12c2624c7dcbce932ef26828ab292084a8c340da',
    original_C1='204735c3f6f3144911df58a6ece7699c0e74d4ad',protected_production='86ddd19f67e08160c22a0d8ba9a32a92321d18ad',
    sources=sources,binaries=binaries,host_libraries=host_libraries,retained_image_libraries=image_libraries,
    toolchain=toolchain,historical_build=build,elf_inspections=elf,rebuilds=0,
    current_Q3_helper='Current repaired Q3 source is not selected. Original C1 source retained separately and original external driver/worker used.'))

inventory=plan['candidates']
assert len(inventory)==80 and sum(x['current_metadata_only']['bytes'] for x in inventory)==4557159684
assert len({x['path'] for x in inventory})==80
put('cache-candidate-inventory.json',dict(schema='exact80-frozen-source-only-inventory-v1',
    plan=info(BASE/'cache-preparation-plan.json'),candidate_source=plan['candidate_source'],candidates=inventory,
    historical_hashes_are_expectations=True,no_substitution=True))
lines=[]
fresh=[]
for entry in inventory:
    p=Path(entry['path']); expected=entry['current_metadata_only']
    # Preserve frozen metadata in native input. Changes are excluded by native
    # no-follow fstat rather than accepting current mutable/replaced identities.
    sec,nsec=divmod(expected['mtime_ns'],1000000000)
    lines.append('\t'.join(map(str,[p,expected['uid'],expected['device'],expected['inode'],expected['bytes'],sec,nsec])))
    try:
        s=p.lstat()
        fresh.append(dict(path=str(p),uid=s.st_uid,device=s.st_dev,inode=s.st_ino,bytes=s.st_size,mtime_ns=s.st_mtime_ns,mode=s.st_mode))
    except OSError as e:
        fresh.append(dict(path=str(p),error=str(e)))
put('cache-seal-input/candidates.tsv','\n'.join(lines)+'\n')
put('cache-seal-input/parts.txt',str(OUT/'cache-seal-input/candidates.tsv')+'\n')
put('cache-seal-input/current-lstat.json',fresh)
package=Path('/workspace/smallroot-session/20261029-library-publication-package')
assert sha(package/'manifest.json')=='3036b6fb1bbc5dc2e477d3d04a11271c32b248a1f83502b78f2977797be03a47'
package_manifest=json.loads((package/'manifest.json').read_text())
assert len(package_manifest['members'])==24
for entry in package_manifest['members']:
    assert (package/entry['path']).stat().st_size==entry['bytes']
    assert sha(package/entry['path'])==entry['sha256']
shutil.copytree(package,OUT/'ancillary-library-validation')
put('cache-seal-input/lease-before-seal.json',lease())
argv=[helper['path'],'--seal',str(OUT/'cache-seal-input/parts.txt'),str(OUT/'cache-seal')]
put('cache-seal-input/invocation.json',dict(argv=argv,cwd=str(OWNER),recorded_at=now().isoformat(),
    helper=info(helper['path'],helper['sha256']),native_seal_attempt=1,maximum_native_seal_attempts=1,
    mode='source-only',advice_calls=0,container_calls=0,worker_calls=0,scientific_calls=0,
    environment_policy='Inherited guarded authentication/proxy unchanged; helper dynamic resolution sealed via actual ELF RUNPATH and dependency hashes; no environment dump.',
    input_files=[info(OUT/'cache-seal-input/parts.txt'),info(OUT/'cache-seal-input/candidates.tsv')]))
lease()  # Fresh ownership immediately before the sole native invocation.
with (OUT/'cache-seal-input/stdout').open('x') as stdout,(OUT/'cache-seal-input/stderr').open('x') as stderr:
    result=subprocess.run(argv,cwd=OWNER,stdout=stdout,stderr=stderr,check=False)
put('cache-seal-input/exit',str(result.returncode)+'\n')
put('cache-seal-input/ended-at',now().isoformat()+'\n')
put('cache-seal-input/lease-after-seal.json',lease())
assert result.returncode==0,'Native seal failure preserved; no retry authorized'
expected_by_path={x['path']:x for x in inventory}
accepted=[]; exclusions=[]
for file in sorted((OUT/'cache-seal').glob('eligible-*.tsv')):
    for line in file.read_text().splitlines():
        fields=line.split('\t');p=fields[0];digest=fields[-1]
        assert p in expected_by_path and len(fields)==8
        x=expected_by_path[p]
        if digest==x['historical_sha256_expected']:
            accepted.append(dict(path=p,uid=int(fields[1]),device=int(fields[2]),inode=int(fields[3]),bytes=int(fields[4]),mtime_seconds=int(fields[5]),mtime_nanoseconds=int(fields[6]),sha256=digest,historical_sha256_expected=x['historical_sha256_expected'],match=True,tsv=line))
        else:
            exclusions.append(dict(path=p,reason='current SHA256 differs from historical expected',actual=digest,expected=x['historical_sha256_expected']))
seen={x['path'] for x in accepted}|{x['path'] for x in exclusions}
for file in sorted((OUT/'cache-seal').glob('records-*.tsv')):
    for line in file.read_text().splitlines():
        if line.split('\t')[0] not in seen:
            exclusions.append(dict(path=line.split('\t')[0],reason='native exclusion',raw_record=line))
assert len(accepted)+len(exclusions)==80
assert len({(x['device'],x['inode']) for x in accepted})==len(accepted)
put('cache-eligible/eligible.tsv','\n'.join(x['tsv'] for x in accepted)+ ('\n' if accepted else ''))
put('cache-eligible/parts.txt',str(OUT/'cache-eligible/eligible.tsv')+'\n' if accepted else '')
put('cache-eligible/allowlist.json',dict(schema='historical-hash-compared-exact-eligible-subset-v1',
    candidate_count=80,accepted_count=len(accepted),accepted_bytes=sum(x['bytes'] for x in accepted),excluded_count=len(exclusions),
    accepted=accepted,exclusions=exclusions,native_seal_invocations=1,advice_performed=False,
    future_advice='Separate actual root admission and publication required; no advice authorized by this seal.'))
print(json.dumps(dict(seal_exit=result.returncode,accepted=len(accepted),excluded=len(exclusions),accepted_bytes=sum(x['bytes'] for x in accepted))))
