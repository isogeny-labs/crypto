"""Actual publication control-plane custody; retains every whole API/log directly."""
from pathlib import Path
from datetime import datetime,timezone
import json,subprocess,hashlib,lzma,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/public-n4-raw-publication-01'
c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='29b14aa697d8583c36ce11164eb72887e987fa25';bot='d31220a234db2400552586716ec26ba71ac98f98'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads((d/'whole-log-api.json').read_text())['structuredContent']['content'].encode()
assert log.index(b'3 remote objects replayed')<log.index(b'[detached HEAD d31220a23]')
jobs=json.loads((d/'jobs-api.json').read_text())['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['conclusion']=='success' and jobs[0]['id']==131858081270
z=lzma.compress(log,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(z)==log;(d/'whole-job.log.xz').write_bytes(z)
objects=[];p=s/'evidence/n4-public-target-raw-handoff-01/.artifacts.json';manifest=json.loads(p.read_text());assert set(manifest['files'])=={'static-01.tar.xz','unit-01.tar.xz','cli-01.tar.xz'};shutil.copyfile(p,d/'canonical-manifest.json')
for name,entry in manifest['files'].items():
    rel=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+rel],cwd=r)
    assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256'];objects.append(dict(path=rel,**entry))
encodings=[]
for source,label in [('/workspace/scratch/ct57-root-raw-publication-01','normal-publication'),('/workspace/scratch/ct57-raw-publication-discovery-01','source-discovery'),('/workspace/scratch/ct57-n4-artifact-ff-01','exact-ff')]:
    target=d/label;target.mkdir()
    for src in sorted(Path(source).rglob('*')):
        if not src.is_file():continue
        rel=src.relative_to(source);out=target/rel;out.parent.mkdir(parents=True,exist_ok=True);v=src.read_bytes()
        if src.suffix=='.log':
            stored=out.with_suffix(out.suffix+'.xz');packed=lzma.compress(v,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(packed)==v;stored.write_bytes(packed);encodings.append(dict(original=str((out).relative_to(d)),stored=str(stored.relative_to(d)),raw_bytes=len(v),raw_sha256=hashlib.sha256(v).hexdigest(),exact_decompress_roundtrip=True))
        else:out.write_bytes(v)
assert all((d/'exact-ff'/(name+'.exit')).read_text().strip()=='0' for name in ['fetch','exact-ff','artifact','wiring'])
(d/'lossless-log-encodings.json').write_text(json.dumps(encodings,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43417227915,job=131858081270,conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=5,actual_remote_byte_replays_BEFORE_bot=3,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all2188_CURRENT_inputs_and_owned_dirty_bytes_preserved=True,whole_log_bytes=len(log),whole_log_sha256=hashlib.sha256(log).hexdigest(),whole_log_XZ_bytes=len(z),whole_control_plane='ALLoriginalAPIJSON/wholeloglosslessXZ/FF/discovery/normalpublication/canonicalmanifest/memberhashes DIRECT;no recursive selfarchive',scope='actual bytepublication/unsigned;no signedexecution/nativeCI/performanceverdict')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n');members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted(d.rglob('*')) if p.is_file()};(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ACTUAL publicN4 rawpublication43417227915/job131858081270SUCCESS THREEremote byte replays BEFOREd31220a23/directparent29b14aa6/EXACT5paths. WHOLEstatic9005108/unit9008132/CLI9010244-byteattemptarchives matchoriginalsourceGitblobs/custody/canonicalmanifest. NormalFF/artifact/wiring0/all2188CURRENTnativeinputs+owned-dirtybytesunchanged. Whole426428-byteUTF8job/allAPIJSON/stdwholelogXZ/FF/discovery/sourcepublication/canonicalmanifest/everymemberhash retainedDIRECT;rawpushloglosslesslyencodedbeforestaging/no normalization. No recursive extraarchive oravailabilityinference fromlocalcopy;no nativeCI/performanceverdict.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip();Path('/workspace/scratch/ct57-actual-raw-publication.claim').write_text(cl+'\n');print('actual',cl,flush=True)
dest=cs/'evidence/genuine-raw-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json');shutil.copyfile(d/'members.json',dest/'root-control-members.json');projected=[]
for label in ['static-01','unit-01','cli-01']:
    target=cs/'evidence'/('n4-'+label);entry=manifest['files'][label+'.tar.xz'];archive=target/'whole-raw.tar.xz'
    assert archive.stat().st_size==entry['bytes'] and hashlib.sha256(archive.read_bytes()).hexdigest()==entry['sha256']
    (target/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files={'whole-raw.tar.xz':entry}),indent=2)+'\n');(target/'.gitignore').write_text('whole-raw.tar.xz\n');assert subprocess.run(['git','check-ignore','-q',str(archive.relative_to(c))],cwd=c).returncode==0
    projected.append(dict(path=str(archive.relative_to(c)),entry=entry,original_physical_bytes_unchanged=True))
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=cl,root_bot=bot,objects=projected,entries_copied_from_GENUINE_canonical_manifest=True,no_direct_large_PCCsubject=True,no_PCCworkflow_delta=True,standing='unsigned genuinepublication,cannotpromotemathorcoststanding'),indent=2)+'\n')
cl=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINEconsumer publicN4rawprojection:ALLTHREEwhole9005108/9008132/9010244archives exactactual43417227915/canonicald31220a23 remote replayentries;originalphysicalarchivesunchanged/ignored/SHAbytesmatchsourceGitblob/custody. Staticae3efe/unite08eac/CLI9e522f stayunsigned;allfailure rows retained. No directlargePCCsubject ornewPCCworkflowread/verifier/trust/policy edit.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();Path('/workspace/scratch/ct57-genuine-raw-publication.claim').write_text(cl+'\n');print('genuine',cl,flush=True)
