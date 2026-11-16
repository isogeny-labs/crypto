"""Actual whole-data publication/custody and genuine consumer manifests only."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/public-n4-completed-publication-01'
c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='cbab2b5d619ff489d4dea4148daf94adb46917d8';bot='32c51ec50b64c6dc3b7af41eb489002eac8f925e'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads((d/'whole-log-api.json').read_text())['structuredContent']['content'].encode()
assert log.index(b'13 remote objects replayed')<log.index(b'[detached HEAD 32c51ec50]')
jobs=json.loads((d/'jobs-api.json').read_text())['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['id']==131981714889 and jobs[0]['conclusion']=='success'
z=lzma.compress(log,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(z)==log;(d/'whole-job.log.xz').write_bytes(z)
p=s/'evidence/public-n4-completed-raw-handoff-01/.artifacts.json';manifest=json.loads(p.read_text());assert len(manifest['files'])==13;shutil.copyfile(p,d/'canonical-manifest.json')
objects=[]
for name,entry in manifest['files'].items():
    path=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+path],cwd=r)
    assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256'];objects.append(dict(path=path,**entry))
enc=[]
for source,label in [('/workspace/scratch/ct59-root-handoff-publication-01','normal-publication'),('/workspace/scratch/ct59-raw-publication-discovery-01','source-discovery'),('/workspace/scratch/ct59-completed-artifact-ff-01','exact-ff')]:
    target=d/label;target.mkdir()
    for src in sorted(Path(source).rglob('*')):
        if not src.is_file():continue
        out=target/src.relative_to(source);out.parent.mkdir(parents=True,exist_ok=True);raw=src.read_bytes()
        if src.suffix in ['.log','.txt','.stdout']:
            z=lzma.compress(raw);assert lzma.decompress(z)==raw;stored=out.with_suffix(out.suffix+'.xz');stored.write_bytes(z)
            enc.append(dict(raw_path=str(out.relative_to(d)),stored_path=str(stored.relative_to(d)),raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),exact_roundtrip=True))
        else:out.write_bytes(raw)
assert all((d/'exact-ff'/(name+'.exit')).read_text().strip()=='0' for name in ['fetch','exact-ff','artifact','wiring'])
(d/'lossless-encodings.json').write_text(json.dumps(enc,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43453551818,job=131981714889,conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=15,actual_remote_byte_replays_BEFORE_bot=13,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all2334_current_inputs_and_owned_dirty_bytes_unchanged=True,source_hold_released_after_FF_terminal=True,whole_UTF8_log_bytes=len(log),whole_UTF8_log_sha256=hashlib.sha256(log).hexdigest(),whole_control_plane='allAPI/logXZ/normalpush/source-discovery/exactFF/canonicalmanifest/memberhashes direct;no recursive selfarchive',scope='nineWHOLEarchives+fourlargeWHOLEindices in S3;one smallWHOLEarchive+six smallWHOLEindices normalGit;20physicalobjects/allfailurecaptures intact;no signedexecution orcostclaim')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n');members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted(d.rglob('*')) if p.is_file()};(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ACTUAL43453551818/job131981714889SUCCESS THIRTEENremote byte replays BEFORE32c51ec50/directparentcbab2b5d/EXACT15paths. NINEwholeclosedarchives+FOURwholelargeindices matchsourceGitblobs/genuinecanonicalmanifest;ONEsmallwholearchive+SIXsmallwholeindices normalGit. NormalFF/artifact/wiring0/all2334currentinputs+dirtybytes unchanged;holdreleasedAFTERterminal. WholeUTF8log/allAPI/logXZ/push/discovery/FF/manifests/index DIRECT/no recursivearchive orpolicywaiver. Alltenclosedattempts include failures/partialprovenance,not signedmath/execution orcostverdict.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
root_claim=subprocess.check_output(cmd,cwd=r,text=True).strip();print('actual',root_claim,flush=True)
dest=cs/'evidence/genuine-completed-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json');projected=[]
for label in ['v2-03','V2-full','V2-CI-partial','V2-CI','completed-native','completed-full','completed-CI-partial','completed-CI','two-primary-partial','two-primary']:
    target=cs/'evidence'/('closed-ct59-'+label);managed={};ignore=[]
    for source_name,target_name in [(label+'.tar.xz','whole-raw.tar.xz'),(label+'-members.json.xz','members.json.xz')]:
        raw=(target/target_name).read_bytes()
        if source_name in manifest['files']:
            entry=manifest['files'][source_name];assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256'];managed[target_name]=entry;ignore.append(target_name)
            projected.append(dict(path=str((target/target_name).relative_to(c)),entry=entry,genuine_remote=True))
        else:
            path=str((p.parent/source_name).relative_to(r));assert raw==subprocess.check_output(['git','show',bot+':'+path],cwd=r) and len(raw)<65536
            projected.append(dict(path=str((target/target_name).relative_to(c)),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),normal_Git_source=bot+':'+path,genuine_remote=False))
    if managed:
        (target/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files=managed),indent=2)+'\n');(target/'.gitignore').write_text('\n'.join(ignore)+'\n')
    for name in ignore:assert subprocess.run(['git','check-ignore','-q',str((target/name).relative_to(c))],cwd=c).returncode==0
assert len(projected)==20 and sum(x['genuine_remote'] for x in projected)==13
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=root_claim,root_bot=bot,objects=projected,original_physical_bytes_intact=True,thirteen_genuine_S3_objects=True,seven_small_whole_objects_inGit=True,no_direct_large_PCCsubject_or_new_PCCread=True,no_threshold_or_verifier_change=True),indent=2)+'\n')
claim=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINE completed/two-primary consumerprojection:13actual43453551818/32c51canonical entries for9WHOLEarchives+4WHOLElargeindices,allbytes/SHA/sourceblobs exact/physicaloriginals intact andignored. Other7WHOLEsmallobjects (1archive6indices) exact ordinaryGit. All20physicalobjects forTENclosedcaptures retained inclpartialcollectors/partialprovenance. No directlargePCCsubject/newPCCread/threshold/verifierchange;correctness/formal/cost standing not promoted.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();print('genuine',claim,flush=True)
