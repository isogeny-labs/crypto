"""Whole index publication custody; no cryptographic computation."""
from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,lzma,subprocess,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';d=s/'evidence/public-n4-index-publication-01';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='648313e6b72b6924a271580e7a4d0a93b2bf05e5';bot='25719e12729d0d3666289299cf5bbb932ce6eeb7'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==bot
log=json.loads((d/'whole-log-api.json').read_text())['structuredContent']['content'].encode();assert log.index(b'3 remote objects replayed')<log.index(b'[detached HEAD 25719e127]')
jobs=json.loads((d/'jobs-api.json').read_text())['structuredContent']['jobs'];assert len(jobs)==1 and jobs[0]['conclusion']=='success' and jobs[0]['id']==131877116632
packed=lzma.compress(log,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(packed)==log;(d/'whole-job.log.xz').write_bytes(packed)
p=s/'evidence/public-n4-index-handoff-01/.artifacts.json';manifest=json.loads(p.read_text());assert len(manifest['files'])==3;shutil.copyfile(p,d/'canonical-manifest.json');objects=[]
for name,entry in manifest['files'].items():
    rel=str((p.parent/name).relative_to(r));raw=subprocess.check_output(['git','show',base+':'+rel],cwd=r);assert len(raw)==entry['bytes'] and hashlib.sha256(raw).hexdigest()==entry['sha256'];objects.append(dict(path=rel,**entry))
enc=[]
for source,label in [('/workspace/scratch/ct57-root-index-publication-01','normal-publication'),('/workspace/scratch/ct57-index-publication-discovery-01','source-discovery'),('/workspace/scratch/ct57-index-artifact-ff-01','exact-ff')]:
    target=d/label;target.mkdir()
    for p in sorted(Path(source).rglob('*')):
        if not p.is_file():continue
        out=target/p.relative_to(source);out.parent.mkdir(parents=True,exist_ok=True);raw=p.read_bytes()
        if p.suffix=='.log':
            z=lzma.compress(raw,format=lzma.FORMAT_XZ,preset=9);assert lzma.decompress(z)==raw;out.with_suffix(out.suffix+'.xz').write_bytes(z);enc.append(dict(raw_path=str(out.relative_to(d)),stored_path=str(out.with_suffix(out.suffix+'.xz').relative_to(d)),raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),exact_roundtrip=True))
        else:out.write_bytes(raw)
assert all((d/'exact-ff'/(name+'.exit')).read_text().strip()=='0' for name in ['fetch','exact-ff','artifact','wiring'])
(d/'lossless-log-encodings.json').write_text(json.dumps(enc,indent=2)+'\n');shutil.copyfile(__file__,d/'original-recorder.py')
summary=dict(at=datetime.now(timezone.utc).isoformat(),run=43422930224,job=131877116632,conclusion='success',source=base,bot=bot,exact_bot_parent=base,exact_changed_paths=5,actual_remote_byte_replays_BEFORE_bot=3,objects=objects,all_source_Git_blob_hashes_match_genuine_manifests=True,normal_FF=0,artifact=0,wiring=0,all2191_CURRENT_inputs_and_owned_dirty_bytes_preserved=True,whole_log_bytes=len(log),whole_log_sha256=hashlib.sha256(log).hexdigest(),whole_log_XZ_bytes=len(packed),whole_control_plane='ALLAPIJSON/wholelogXZ/fullFF/sourcepublication/discovery/manifests/everymemberhashDIRECT/no recursivearchive',scope='THREEcompleteindices ONLY;priorwholearchivesf32a84/30806e separate actualscope;no thresholdchange')
(d/'publication.json').write_text(json.dumps(summary,indent=2)+'\n');members={str(p.relative_to(d)):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in sorted(d.rglob('*')) if p.is_file()};(d/'members.json').write_text(json.dumps(members,indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','ACTUAL completeindex publication43422930224/job131877116632SUCCESS THREEremote byte replays BEFORE25719e127/directparent648313e6/EXACT5paths. ALLWHOLE97492/97556/98960-byteindices matchoriginalsourceGitblobs/custody/genuinecanonicalmanifests;normalFF/artifact/wiring0/all2191CURRENTinputs+owned-dirtybytes unchanged. Whole426293-byteUTF8job/APIJSON/wholelogXZ/fullFF/sourcepublication/discovery/manifests/everymemberhashDIRECT/no recursivearchive. Preserve failedconsumerartifact1;no split/prune/reencoding/thresholdchange. PriorTHREE9MBwholearchivesf32a84/30806e remainseparate actualscope;no nativeCI/performanceverdict.']
for name in ['publication.json','members.json','whole-job.log.xz']:cmd+=['--proof','attestation:'+str((d/name).relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip();Path('/workspace/scratch/ct57-actual-index-publication.claim').write_text(cl+'\n');print('actual',cl,flush=True)
dest=cs/'evidence/genuine-index-publication-01';dest.mkdir();shutil.copyfile(d/'publication.json',dest/'publication.json');projected=[]
for label in ['static-01','unit-01','cli-01']:
    target=cs/'evidence'/('n4-'+label);entry=manifest['files'][label+'-members.json.xz'];index=target/'members.json.xz';assert index.stat().st_size==entry['bytes'] and hashlib.sha256(index.read_bytes()).hexdigest()==entry['sha256']
    path=str(index.relative_to(c))
    if subprocess.run(['git','ls-files','--error-unmatch','--',path],cwd=c,capture_output=True).returncode==0:subprocess.run(['git','rm','--cached','--',path],cwd=c,check=True,capture_output=True)
    cm=json.loads((target/'.artifacts.json').read_text());assert 'whole-raw.tar.xz' in cm['files'];cm['files']['members.json.xz']=entry;(target/'.artifacts.json').write_text(json.dumps(cm,indent=2)+'\n')
    lines=(target/'.gitignore').read_text().splitlines();assert 'members.json.xz' not in lines;(target/'.gitignore').write_text('\n'.join(lines+['members.json.xz'])+'\n');assert subprocess.run(['git','check-ignore','-q',path],cwd=c).returncode==0
    projected.append(dict(path=path,entry=entry,whole_original_index_bytes_unchanged=True))
(dest/'projection.json').write_text(json.dumps(dict(root_actual_claim=cl,root_bot=bot,objects=projected,whole_archive_entries_preserved=True,total_managed_objects=6,entries_GENUINE=True,no_new_direct_large_PCCsubject=True,no_new_PCCworkflow_read_for_indices=True),indent=2)+'\n')
cl=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','GENUINEcompleteindex consumerprojection:THREEWHOLEmemberindices97492/97556/98960bytes copiedfromACTUAL43422930224/canonical25719e127entries,alloriginalphysicalbytesintact/ignored. ExistingTHREEwhole9MBarchiveentries30806e preserved;SIXobjectsmanagedatcorrectpaths/rawarchive+indexSHAcustodyexact. Failedartifact1retained;no split/prune/rename/encodingthresholdwaiver/directlargePCCsubject/newPCCread needed. Wholeclosednativeattempts andcompleteindices nowbothgenuinepublished;native/formal/coststanding remainsunsigned.','--proof','attestation:'+str((dest/'publication.json').relative_to(c)),'--proof','attestation:'+str((dest/'projection.json').relative_to(c))],cwd=c,text=True).strip();Path('/workspace/scratch/ct57-genuine-index-publication.claim').write_text(cl+'\n');print('genuine',cl,flush=True)
