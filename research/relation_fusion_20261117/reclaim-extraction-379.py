from pathlib import Path
import json,hashlib,tarfile,shutil,os,datetime
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';archive=s/'flat-matched-qualification-373.tar.xz';receipt=json.loads((s/'FLAT_MATCHED_ARCHIVE_373.json').read_text());assert hashlib.sha256(archive.read_bytes()).hexdigest()==receipt['sha256']
root=Path('/workspace/flat-matched-extracted-375');assert root.is_dir()
with tarfile.open(archive,'r:xz') as t:index_bytes=t.extractfile('index.json').read()
assert (root/'index.json').read_bytes()==index_bytes;rows=json.loads(index_bytes)['files'];expected={r['path']:r for r in rows}
assert sorted(expected)==sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file() and p!=root/'index.json')
for rel,r in expected.items():
 p=root/rel;assert not p.is_symlink() and p.stat().st_size==r['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==r['sha256'],rel
record=[{'path':str(root),'files':len(rows)+1,'binding':'Every published archive373index payload plus exact index bytes checked; no extras.'}]
base={rel.removeprefix('neutral-367/'):r['sha256'] for rel,r in expected.items() if rel.startswith('neutral-367/')}
for mode in ['clock','duplicate']:
 view=Path(f'/workspace/flat-matched-extracted-{mode}-375');assert view.is_dir();d=root/'canary-deltas'/mode;changed=json.loads((d/'delta-roster.json').read_text())['changed_files'];wanted=dict(base)
 for rel in changed:wanted[rel]=expected[str(Path('canary-deltas')/mode/rel)]['sha256']
 assert sorted(wanted)==sorted(str(p.relative_to(view)) for p in view.rglob('*') if p.is_file())
 for rel,h in wanted.items():
  p=view/rel;assert not p.is_symlink() and hashlib.sha256(p.read_bytes()).hexdigest()==h,(mode,rel)
 record.append({'path':str(view),'files':len(wanted),'binding':f'Exact neutral373payload byte map plus published {mode} delta; no extras.'})
protected=[s/'flat-matched-qualification-373.tar.xz',Path('/workspace/relation-native-qualifiers/flat-supervised-366/flat-supervised-analyze'),Path('/workspace/flat-supervised-neutral-367/summary.json'),Path('/workspace/flat-supervised-hypothetical20-368/summary.json'),Path('/workspace/crypto-root-unoptimized-181/release/build/crypto/e7435ac0b284a6c7/out/crypto_lib-e7435ac0b284a6c7')]
protected_hashes={str(p):hashlib.file_digest(p.open('rb'),'sha256').hexdigest() for p in protected}
r={'schema':'verified-reproducible-extraction-scratch-reclamation/v1','recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'archive373_sha256':receipt['sha256'],'archive_index_sha256':hashlib.sha256(index_bytes).hexdigest(),'removed_only_verified_scratch':record,'protected_hashes':protected_hashes,'reconstruction':'Exact published373archive plus its indexedcanary deltas; safe extraction375/complete replay receipts preserved.','all_original_cohorts_package_frozen_tools_and_replay_reports_preserved':True,'performance_observations_collected':0};out=s/'ROOT_SCRATCH_RECLAIM_379.json';out.write_text(json.dumps(r,indent=2)+'\n')
before=os.statvfs('/workspace')
for item in record:shutil.rmtree(item['path'])
after=os.statvfs('/workspace')
assert all(hashlib.file_digest(Path(p).open('rb'),'sha256').hexdigest()==h for p,h in protected_hashes.items())
r['free_bytes_before']=before.f_bavail*before.f_frsize;r['free_bytes_after']=after.f_bavail*after.f_frsize;r['protected_hashes_unchanged']=True;out.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({'verified_scratch_trees_reclaimed':len(record),'free_bytes_before':r['free_bytes_before'],'free_bytes_after':r['free_bytes_after'],'all_protected_hashes_unchanged':True}))
