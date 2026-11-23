"""Retain the whole final publication control plane directly; no self-archive loop."""
from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,lzma,subprocess,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/g3-norm-ci-final-publication-01'
c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-reference'
base='9b795075574a11efc918c1e393c0d8e388973ffa';bot='aef7ecb51943090d0afe10fd5a947529d378973d'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads((d/'whole-log-api.json').read_text())['structuredContent']['content'].encode()
assert log.index(b'3 remote objects replayed')<log.index(b'[detached HEAD aef7ecb51]')
jobs=json.loads((d/'jobs-api.json').read_text())['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['conclusion']=='success' and jobs[0]['id']==131834185847
z=lzma.compress(log,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(z)==log;(d/'whole-job.log.xz').write_bytes(z)
objects=[]
panels={'genus3-primary-publication-raw-01':['whole-raw.tar.xz'],'g3-norm-ci-raw-handoff-01':['own-ci-raw-01.tar.xz','own-ci-raw-02.tar.xz']}
for folder,files in panels.items():
    p=s/'evidence'/folder/'.artifacts.json';manifest=json.loads(p.read_text());shutil.copyfile(p,d/(folder+'-canonical-manifest.json'))
    assert set(manifest['files'])==set(files)
    for name,entry in manifest['files'].items():
        rel=str((p.parent/name).relative_to(r));original=subprocess.check_output(['git','show',base+':'+rel],cwd=r)
        assert len(original)==entry['bytes'] and hashlib.sha256(original).hexdigest()==entry['sha256'];objects.append(dict(path=rel,**entry))
for source,name in [('/workspace/scratch/ct56-root-ci-publication-01','normal-publication'),('/workspace/scratch/ct56-ci-publication-discovery-01','source-discovery'),('/workspace/scratch/ct56-ci-artifact-ff-01','exact-ff')]:
    shutil.copytree(source,d/name)
ff=d/'exact-ff';assert all((ff/(p+'.exit')).read_text().strip()=='0' for p in ['fetch','exact-ff','artifact','wiring'])
shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43410133205,job=131834185847,conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=7,actual_remote_byte_replays_BEFORE_bot=3,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all2175_CURRENT_inputs_and_owned_dirty_bytes_preserved=True,whole_log_bytes=len(log),whole_log_sha256=hashlib.sha256(log).hexdigest(),whole_log_XZ_bytes=len(z),control_plane_retention='ALL APIs directly plus whole log losslessXZ/FF/source-discovery/normal-publication/canonical manifests/memberhashes; no redundant recursive selfarchive',standing='unsigned actual publication/custody, not a mathematical/performance/signed execution verdict')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n')
members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted(d.rglob('*')) if p.is_file()}
(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Unsigned ACTUAL final normCI raw publication43410133205/job131834185847SUCCESS:THREEremote byte replays BEFOREcanonicalaef7ecb51/directparent9b795075/EXACT7paths. Whole67172primarypublicationcollector andWHOLE134968partial/790964completeCIcaptures matchoriginalsourceGitblobs/genuinecanonicalmanifests. NormalFF/artifact/wiring0/all2175CURRENTinputs+owned-dirtybytes unchanged. ALLoriginalAPIJSON/whole426564-byteUTF8 log losslessXZ/fullFF+publication+discovery+manifests/everymemberhash retained directly; no recursive extra archive. No policy/verifier/source changes oravailabilityinference fromlocalcopy.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip();Path('/workspace/scratch/ct56-final-ci-actual-publication.claim').write_text(cl+'\n');print('actual publication',cl,flush=True)
dest=cs/'evidence/g3-norm-ci-genuine-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json');shutil.copyfile(d/'members.json',dest/'root-publication-members.json')
manifest=json.loads((s/'evidence/g3-norm-ci-raw-handoff-01/.artifacts.json').read_text())
projected=[]
for label in ['own-ci-raw-01','own-ci-raw-02']:
    target=cs/'evidence'/('g3-norm-'+label);entry=manifest['files'][label+'.tar.xz'];p=target/'whole-raw.tar.xz'
    assert p.stat().st_size==entry['bytes'] and hashlib.sha256(p.read_bytes()).hexdigest()==entry['sha256']
    (target/'.artifacts.json').write_text(json.dumps(dict(schema=manifest['schema'],files={'whole-raw.tar.xz':entry}),indent=2)+'\n');(target/'.gitignore').write_text('whole-raw.tar.xz\n')
    assert subprocess.run(['git','check-ignore','-q',str(p.relative_to(c))],cwd=c).returncode==0
    projected.append(dict(path=str(p.relative_to(c)),entry=entry,original_physical_archive_unchanged=True))
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=cl,root_bot=bot,objects=projected,all_entries_copied_from_genuine_canonical_manifest=True,no_direct_large_PCC_subject=True,no_new_PCC_workflow_read_needed=True,scope='genuine byte publication;native and signedstanding separate'),indent=2)+'\n')
cl=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINE consumer normCI raw manifests project BOTH whole original134968/790964 archives from actual43410133205 canonicalaef7ecb51 publication;all originalphysicalbytes unchanged,ignored,exact SHA/size matches canonicalentry and sourceGitblob. CompleteCI168c99 settles all7source069jobs;rawpartialfailure retained. No newdirectlargePCCarchive subject/PCCworkflowread/verifier/policy edit.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();Path('/workspace/scratch/ct56-ci-genuine-publication.claim').write_text(cl+'\n');print('genuine projection',cl,flush=True)
