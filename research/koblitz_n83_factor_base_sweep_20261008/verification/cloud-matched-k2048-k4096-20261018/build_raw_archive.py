from pathlib import Path
import json,hashlib,zipfile,re
root=Path(__file__).resolve().parent
repo=Path('/home/ec2-user/crypto-n83-resume');study=repo/'research/koblitz_n83_factor_base_sweep_20261008'
log=(root/'validation/lib-and-example.log').read_text()
assert '4091 passed; 0 failed; 117 ignored' in log and '19 passed; 0 failed' in log
assert 'Ran 63 tests' in (root/'validation/n83-python.log').read_text() and 'OK' in (root/'validation/n83-python.log').read_text()
assert 'Ran 46 tests' in (root/'validation/boundary-python.log').read_text() and 'OK' in (root/'validation/boundary-python.log').read_text()
assert len(list((root/'negative-controls').glob('*/result.json')))==6
entries={}
def add(name,p):
 p=Path(p)
 assert p.is_file() and not p.is_symlink(),str(p)
 assert name not in entries and not name.startswith('/') and '..' not in Path(name).parts
 entries[name]=p
for stage,k in [('capacity',2048),('capacity',4096),('search',2048),('search',4096),('control',2048),('control',4096)]:
 label=stage+'-k'+str(k)
 a=json.loads((root/label/'independent-audit.json').read_text());assert a['status'].startswith('PASS')
 for p in sorted((root/label).rglob('*')):
  if p.is_file():add(str(p.relative_to(root)),p)
 for suffix in ['.launch.json','.supervisor.stdout','.supervisor.stderr']:add(label+suffix,root/(label+suffix))
for directory in ['inputs','negative-controls','negative-controls-initial','negative-controls-before-final-replay','validation']:
 for p in sorted((root/directory).rglob('*')):
  if p.is_file() and not p.name.endswith('.gz'):add(str(p.relative_to(root)),p)
for name in ['audit_stage.py','audit_stage-initial.py','observe_stage.py','observe_control.py','negative_controls.py','negative_controls-initial.py','negative_controls-before-label-fix.py','run_capture.py','prepare_inputs.py','build-attestation.json','short-control-context.json','runtime-preflight.json','image-id.txt','image-repodigests.json','daemon.json','host.json','comparison-k4096.json','construct-k4096.json','construct-k4096.stdout','construct-k4096.stderr','replay-k4096.json','replay-k4096.stdout','replay-k4096.stderr','negative-control-initial-k4096.json','negative-control-name-collision.json','query-identity.json','accounting.json','summary.json','docker-service-state.txt','containers-after.txt','toolchain-observed.txt','worker-compiler-markers.txt','bin/docker']:
 add(name,root/name)
for name in ['manifest.json','replay.json']:add('input-recovery-k4096/'+name,root/'reconstructed-k4096'/name)
for p in sorted((root/'setec-k4096-publication').iterdir()):
 if p.is_file() and p.suffix in ('.json','.stdout','.stderr','.py'):add('setec-k4096-publication/'+p.name,p)
p=study/'verification/cloud-matched-k2048-k4096-20261018'
for name in ['PROTOCOL.md','SHORT_CONTROL_PROTOCOL.json']:add(name,p/name)
worker=Path('/home/ec2-user/scratch/n83-resume-20261018/reconstruction-target/release/examples/koblitz_n83_factor_base_export');assert hashlib.sha256(worker.read_bytes()).hexdigest()=='9aa6f722d79f460d9388f63954c6ddbed1c2672e08c35c0bd34e697f5d7ddfc0'
add('worker',worker);add('b3file',root/'b3file');add('b3file.rs','/home/ec2-user/scratch/n83-resume-20261018/b3file.rs')
rows=[{'path':name,'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for name,p in sorted(entries.items())]
manifest={'schema':'n83.cloud-matched-raw-archive/v1','source_commits':['f08f9f887ba9f87d0b8976ec1c7a6cd65eb63127','19c2f73268be9c4fa16b18f55f234e077ffb5e53'],'factor_base_storage':'setec-astronomy-artifacts; compressed bases stored separately at exact input.json URIs','files':rows,'scope':'allowlisted public research artifacts only: exact worker, source snapshots, public fixture metadata, receipts, logs and control scripts; no git directory, home configuration or credentials','selected_best_total_runtime':None}
m=root/'raw-artifact-manifest.json';m.write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n');add('raw-artifact-manifest.json',m)
artifacts=root/'artifacts';artifacts.mkdir();tmp=artifacts/'raw-evidence.zip'
with zipfile.ZipFile(tmp,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=6) as archive:
 for name,p in sorted(entries.items()):
  info=zipfile.ZipInfo(name,(2026,10,18,12,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=(0o100755 if name in ('worker','b3file','bin/docker') else 0o100644)<<16
  archive.writestr(info,p.read_bytes())
h=hashlib.sha256(tmp.read_bytes()).hexdigest();dest=artifacts/(h+'.zip');tmp.rename(dest)
with zipfile.ZipFile(dest) as archive:
 for row in rows:
  b=archive.read(row['path']);assert len(b)==row['bytes'] and hashlib.sha256(b).hexdigest()==row['sha256']
plan={'schema':'n83.raw-artifact-upload-plan/v1','archive_path':str(dest),'archive_sha256':h,'archive_bytes':dest.stat().st_size,'manifest_sha256':hashlib.sha256(m.read_bytes()).hexdigest(),'file_count':len(entries),'bucket':'setec-astronomy-artifacts','region':'us-east-2','key':'factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/verification/cloud-matched-k2048-k4096-20261018/raw/'+h+'.zip'}
(root/'raw-artifact-plan.json').write_text(json.dumps(plan,indent=2)+'\n');print(json.dumps(plan))
