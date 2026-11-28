"""Actual whole publication, lossless control plane and genuine projections."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/public-n4-odd-product-publication-01';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='056ac31949604080c5afbebadb0a326fcd399158';bot='22d22eb0e9339bb1d2f3d5319bd11114b0ade83d'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads(lzma.decompress((d/'whole-log-api.json.xz').read_bytes()))['structuredContent']['content'].encode();assert log.index(b'16 remote objects replayed')<log.index(b'[detached HEAD 22d22eb0e]')
jobs=json.loads(lzma.decompress((d/'jobs-api.json.xz').read_bytes()))['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['id']==132162532391 and jobs[0]['conclusion']=='success'
assert lzma.decompress((d/'whole-job.log.xz').read_bytes())==log
p=s/'evidence/public-n4-odd-product-raw-handoff-01/.artifacts.json';manifest=json.loads(p.read_text());assert len(manifest['files'])==16;shutil.copyfile(p,d/'canonical-manifest.json')
objects=[]
for name,entry in manifest['files'].items():
 path=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+path],cwd=r);assert len(raw)==entry['bytes']and hashlib.sha256(raw).hexdigest()==entry['sha256'];objects.append(dict(path=path,**entry))
enc=[]
for source,label in [('/workspace/scratch/ct64-root-handoff-publication-01','normal-publication'),('/workspace/scratch/ct64-raw-publication-discovery-01','source-discovery'),('/workspace/scratch/ct64-odd-product-artifact-ff-01','exact-ff')]:
 target=d/label;target.mkdir()
 for src in sorted(Path(source).rglob('*')):
  if not src.is_file():continue
  out=target/src.relative_to(source);out.parent.mkdir(parents=True,exist_ok=True);raw=src.read_bytes()
  if src.suffix in ['.log','.txt','.stdout','.json']:
   z=lzma.compress(raw);assert lzma.decompress(z)==raw;stored=out.with_suffix(out.suffix+'.xz');stored.write_bytes(z);enc.append(dict(raw_path=str(out.relative_to(d)),stored_path=str(stored.relative_to(d)),raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),exact_roundtrip=True))
  else:out.write_bytes(raw)
assert all((d/'exact-ff'/(n+'.exit')).read_text().strip()=='0'for n in ['fetch','exact-ff','artifact','wiring'])
preservation=json.loads(Path('/workspace/scratch/ct64-odd-product-artifact-ff-01/dirty-preservation.json').read_text());assert preservation['current_union_count']==3241 and preservation['all_consumer_dirty_bytes_preserved']
(d/'lossless-encodings.json').write_text(json.dumps(enc,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43505654954,job=132162532391,event='pull_request_target',conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=18,actual_remote_byte_replays_BEFORE_bot=16,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all3241_selected_inputs_and_both_repositories_owned_dirty_bytes_unchanged=True,source_hold_released_after_FF_terminal=True,whole_UTF8_log_bytes=len(log),whole_UTF8_log_sha256=hashlib.sha256(log).hexdigest(),whole_control_plane='wholeAPIJSONlosslessXZ/logXZ/normalpush/discovery/exactFF/manifests direct,no recursivepublicationarchive',scope='9 WHOLE archives+5 WHOLE large indices+2 WHOLE complete native fixtures S3;4 small WHOLE indices normal Git/all20 physical objects/all9 failure and success captures;not signed execution/math/cost')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n');members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest())for p in sorted(d.rglob('*'))if p.is_file()};(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ACTUAL43505654954/job132162532391/pull_request_target SUCCESS SIXTEEN remote byte replays BEFORE22d22eb0e/directparent056ac319/EXACT18 paths. NINE whole archives+FIVE whole large indices+TWO complete native fixtures match source Git blobs/genuine canonical manifest;FOUR small whole indices ordinary Git. NormalFF/artifact/wiring0/ALL3241 current selected inputs+owned dirty bytes in BOTH repos unchanged;guard06 hold released AFTER terminal. Whole429122-byte UTF8 log/all whole API losslessXZ/push/discovery/FF/manifests/controlindex DIRECT,no recursive publication archive/policy change. All9 failed/success/native/fullCI captures and complete native fixture01+02 retained;not signed math/execution/cost.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
root_claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print('actual',root_claim,flush=True)
dest=cs/'evidence/genuine-odd-product-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json')
projected=[];data_objects=json.loads((p.parent/'objects.json').read_text());groups={}
for obj in data_objects:
 target=c/obj['consumer_path'];raw=target.read_bytes();source_name=Path(obj['path']).name
 assert len(raw)==obj['bytes'] and hashlib.sha256(raw).hexdigest()==obj['sha256']
 if source_name in manifest['files']:
  entry=manifest['files'][source_name];assert len(raw)==entry['bytes']and hashlib.sha256(raw).hexdigest()==entry['sha256']
  groups.setdefault(target.parent,{})[target.name]=entry
  projected.append(dict(path=obj['consumer_path'],entry=entry,genuine_remote=True,role=obj['role']))
 else:
  assert raw==subprocess.check_output(['git','show',bot+':'+obj['path']],cwd=r)and len(raw)<65536
  projected.append(dict(path=obj['consumer_path'],bytes=len(raw),sha256=obj['sha256'],normal_Git_source=bot+':'+obj['path'],genuine_remote=False,role=obj['role']))
for folder,managed in groups.items():
 (folder/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files=managed),indent=2)+'\n')
 (folder/'.gitignore').write_text('\n'.join(sorted(managed))+'\n')
 for name in managed:assert subprocess.run(['git','check-ignore','-q',str((folder/name).relative_to(c))],cwd=c).returncode==0
assert len(projected)==20 and sum(x['genuine_remote']for x in projected)==16
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=root_claim,root_bot=bot,objects=projected,all_original_physical_bytes_intact=True,sixteen_genuine_S3_objects=True,four_small_whole_indices_Git=True,no_direct_large_PCC_subject_or_new_PCCread=True,no_threshold_verifier_or_trust_change=True),indent=2)+'\n')
claim=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINE nine-capture consumer projection16 actual43505654954/22d22eb0e canonical entries:9 WHOLE archives+5 WHOLE large indices+2 COMPLETE native fixture01/02;all bytes/SHA/source Git blobs exact/physical originals intact and ignored. Other4 WHOLE small indices exact ordinaryGit;all20 objects and failed/success/fullCI/native captures retained. No direct large PCC subject/new PCC read/threshold/verifier/trust change;new CI may read complete fixture02 via original artifact tool. All math/formal/cost standing unchanged, unsigned.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();print('genuine',claim,flush=True)
