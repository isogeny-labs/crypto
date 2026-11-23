"""Actual whole publication, lossless control plane and genuine projections."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/public-n4-gluing-publication-01';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='f4565b1f23289a8f7f24141c9fd9b61db018b07a';bot='3bc395c259f09d756e868027f371af25f86aa248'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads(lzma.decompress((d/'whole-log-api.json.xz').read_bytes()))['structuredContent']['content'].encode();assert log.index(b'26 remote objects replayed')<log.index(b'[detached HEAD 3bc395c25]')
jobs=json.loads(lzma.decompress((d/'jobs-api.json.xz').read_bytes()))['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['id']==132102493726 and jobs[0]['conclusion']=='success'
assert lzma.decompress((d/'whole-job.log.xz').read_bytes())==log
p=s/'evidence/public-n4-gluing-raw-handoff-01/.artifacts.json';manifest=json.loads(p.read_text());assert len(manifest['files'])==26;shutil.copyfile(p,d/'canonical-manifest.json')
objects=[]
for name,entry in manifest['files'].items():
 path=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+path],cwd=r);assert len(raw)==entry['bytes']and hashlib.sha256(raw).hexdigest()==entry['sha256'];objects.append(dict(path=path,**entry))
enc=[]
for source,label in [('/workspace/scratch/ct61-root-handoff-publication-01','normal-publication'),('/workspace/scratch/ct61-raw-publication-discovery-01','source-discovery'),('/workspace/scratch/ct61-gluing-artifact-ff-01','exact-ff')]:
 target=d/label;target.mkdir()
 for src in sorted(Path(source).rglob('*')):
  if not src.is_file():continue
  out=target/src.relative_to(source);out.parent.mkdir(parents=True,exist_ok=True);raw=src.read_bytes()
  if src.suffix in ['.log','.txt','.stdout','.json']:
   z=lzma.compress(raw);assert lzma.decompress(z)==raw;stored=out.with_suffix(out.suffix+'.xz');stored.write_bytes(z);enc.append(dict(raw_path=str(out.relative_to(d)),stored_path=str(stored.relative_to(d)),raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),exact_roundtrip=True))
  else:out.write_bytes(raw)
assert all((d/'exact-ff'/(n+'.exit')).read_text().strip()=='0'for n in ['fetch','exact-ff','artifact','wiring'])
preservation=json.loads(Path('/workspace/scratch/ct61-gluing-artifact-ff-01/dirty-preservation.json').read_text());assert preservation['current_union_count']==3096 and preservation['all_consumer_dirty_bytes_preserved']
(d/'lossless-encodings.json').write_text(json.dumps(enc,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43488304162,job=132102493726,event='pull_request_target',conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=28,actual_remote_byte_replays_BEFORE_bot=26,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all3096_selected_inputs_and_both_repositories_owned_dirty_bytes_unchanged=True,source_hold_released_after_FF_terminal=True,whole_UTF8_log_bytes=len(log),whole_UTF8_log_sha256=hashlib.sha256(log).hexdigest(),whole_control_plane='wholeAPIJSONlosslessXZ/logXZ/normalpush/discovery/exactFF/manifests direct,no recursivepublicationarchive',scope='17WHOLEarchives+9largeWHOLEindicesS3;8smallWHOLEindicesordinaryGit/all34physicalobjects/all17failureandsuccesscaptures;not signedexecution/math/cost')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n');members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest())for p in sorted(d.rglob('*'))if p.is_file()};(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ACTUAL43488304162/job132102493726/pull_request_targetSUCCESS TWENTYSIXremote byte replays BEFORE3bc395c25/directparentf4565b1f/EXACT28paths. SEVENTEENwholearchives+NINEwholelargeindices sourceGitblobs/genuinecanonicalmanifest exact;EIGHTsmallwholeindices ordinaryGit. NormalFF/artifact/wiring0/ALL3096selectedinputs+ownedDIRTYbytesBOTHerepos unchanged;holdreleasedAFTERterminal. Whole430522byteUTF8log/allwholeAPIlosslessXZ/push/discovery/FF/manifests/controlindexDIRECT,no recursivearchive/policywaiver. All17failed/success/native/fullCIcaptures retained;not signedmath/execution/cost.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
root_claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print('actual',root_claim,flush=True)
dest=cs/'evidence/genuine-gluing-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json');projected=[];handoff=json.loads((p.parent/'handoff.json').read_text())
for row in handoff['attempts']:
 label=row['label'];target=cs/'evidence'/('closed-ct61-'+label);managed={};ignore=[]
 for source_name,target_name in [(label+'.tar.xz','whole-raw.tar.xz'),(label+'-members.json.xz','members.json.xz')]:
  raw=(target/target_name).read_bytes()
  if source_name in manifest['files']:
   entry=manifest['files'][source_name];assert len(raw)==entry['bytes']and hashlib.sha256(raw).hexdigest()==entry['sha256'];managed[target_name]=entry;ignore.append(target_name);projected.append(dict(path=str((target/target_name).relative_to(c)),entry=entry,genuine_remote=True))
  else:
   path=str((p.parent/source_name).relative_to(r));assert raw==subprocess.check_output(['git','show',bot+':'+path],cwd=r)and len(raw)<65536;projected.append(dict(path=str((target/target_name).relative_to(c)),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),normal_Git_source=bot+':'+path,genuine_remote=False))
 if managed:(target/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files=managed),indent=2)+'\n');(target/'.gitignore').write_text('\n'.join(ignore)+'\n')
 for name in ignore:assert subprocess.run(['git','check-ignore','-q',str((target/name).relative_to(c))],cwd=c).returncode==0
assert len(projected)==34 and sum(x['genuine_remote']for x in projected)==26
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=root_claim,root_bot=bot,objects=projected,original_physical_bytes_intact=True,twentysix_genuine_S3_objects=True,eight_small_whole_indices_Git=True,no_direct_large_PCCsubject_or_new_PCCread=True,no_threshold_or_verifier_change=True),indent=2)+'\n')
claim=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINE17captureconsumerprojection:26actual43488304162/3bc395c25canonicalentries(17WHOLEarchives+9WHOLElargeindices)/allbytesSHA/sourceblobs exact/physicaloriginalsintactandignored;other8WHOLEsmallindicesexact ordinaryGit. ALL34objects/all17nativefailures/success/fullCIcaptures retained. No directlargePCCsubject/newPCCread/threshold/verifier/trustchange;math/formal/coststanding unchanged.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();print('genuine',claim,flush=True)
