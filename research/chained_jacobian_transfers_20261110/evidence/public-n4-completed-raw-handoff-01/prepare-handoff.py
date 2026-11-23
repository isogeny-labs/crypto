"""Whole-data handoff with a new actual-current source hold; no crypto."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,subprocess,shutil
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='237230b3d807f53f93a5562fa9ba84332efc0878';assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-completed-raw-handoff-01';handoff=json.loads((d/'handoff.json').read_text());assert len(handoff['attempts'])==10
objects=[]
for row in handoff['attempts']:
 for name,size,digest in [(row['label']+'.tar.xz',row['archive_bytes'],row['archive_sha256']),(row['label']+'-members.json.xz',row['complete_index_xz_bytes'],row['complete_index_xz_sha256'])]:
  p=d/name;assert p.stat().st_size==size and hashlib.sha256(p.read_bytes()).hexdigest()==digest
  objects.append(dict(path=str(p.relative_to(r)),bytes=size,sha256=digest,requires_original_data_migration=size>65536))
assert len(objects)==20 and sum(x['requires_original_data_migration']for x in objects)==13
g=s/'evidence/public-n4-completed-current-guard-04';g.mkdir()
current=json.loads(Path('/workspace/scratch/ct59-two-primary-native-02/before.json').read_text());assert len(current)==2318
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
for line in (cs/'ci_two_primary/reference-01.sha256').read_text().splitlines():
 h,p=line.split('  ',1);p=(c/p).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==h;current[str(p)]=h
p=cs/'ci_two_primary/reference-01.sha256';current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
(g/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),preserves_native02_all2318=True,current_guard_count=len(current),scope='native02 full selected union plus actual new502 portable two-primary CI and its manifest; all selected current hashes checked',hold='No edit to selected physical bytes until exact artifactFF terminal/finalguards; independent runs use physical Git clones',historical_guard03='released/stale/not reused',native='510d03 C4;65f7a4 writtenfield6/fourpartialbindings;completedfullb40fec/CIa42f6a'),indent=2)+'\n')
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');shutil.copyfile(__file__,d/'prepare-handoff.py')
(d/'scope.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,physical_data_files=20,whole_closed_archives=10,complete_member_indices=10,original_policy_requires_migration=13,small_whole_objects_in_Git=7,actual_remote_availability='pending genuine trusted-base publisher',all_bytes_exact=True,no_split_prune_rename_threshold_change=True,current_guard_count=len(current),guard_path=str(g.relative_to(r)),publication_control_plane='retain direct whole API/logXZ/push/FF/manifests; no recursive publication archive'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','TENwhole closedcapture handoff BEFOREactualpublication:10complete archives+10wholememberindices/all20byteexact. Originalpolicy selects13largeobjects (9archives/4indices);7whole smallobjects normalGit. Includes newV2native03/full/partial+completeCI,candidateunit/full/currentpartial+completeCI,andpartial/fullprovenance two-primary scouts. Allfailure/native/data bytes preserved. Freshcurrentguard04 preserves2318nativeinputs plus502portableCI/self, '+str(len(current))+'selectedhashes allmatch. HOLDuntilactualbotFF/finalguards;old2283guard03 not reused. Genuine remoteavailabilitypending/no policyortrust waiver.']
for p in [d/'handoff.json',d/'objects.json',d/'scope.json',g/'custody.json']:cmd+=['--proof','attestation:'+str(p.relative_to(r))]
print(subprocess.check_output(cmd,cwd=r,text=True).strip(),len(current),flush=True)
entry=f'''\n## {datetime.now(timezone.utc).isoformat()} — actual q223 completed descent and two-primary source\n\nConsumer6ccd4334 normalpublished0/140paths/a9893d. Completed candidate independentfullb40fec484guards/fouradmissioncontrols and allfour exact68febCI workflows succeed/a42f6a;all42nativeJSON and17canonicalPCCdigests match. Newnative510d03 rationalJacC4/sourceC2×C2/2318inputguards;written65f7a4 fullgeometrictorsionfielddegree6 withfourfaithfulpartialLeanbindings,bothunchangedoriginalgates. TWO51639bytePDFpagesALLinspected/ce9a6d. Actualgeometryformalbridges andnondualkernel/map remainopen. [pcc:unverified]\n\nTENwhole closedarchives+TENwholeindices are ready for actualtrustedpublication:13largeobjects underunchangedpolicy,7whole smallobjects ordinaryGit. Guard04 freshlyfreezes{len(current)}actualcurrentinputs includingnew502portabletwo-primaryCI;HOLDselectedphysicalbytes untilactualbotFF/terminalguards. Historicalguard03 is stale/released. Keep allpartial/failurecaptures,allAPI/logXZ/normalpush/FF directly, no recursivepublicationarchive. Rootcurrent237 metadata/no native changes. [pcc:unverified]\n\nPersistentnextqueue: independenttwo-primary strict full/ownCI (newdriverimplemented,notyet independentlyaccepted);finishgenuine rawpublication andconsumerprojection;constructactualgeometric2torsion/bitangents/FrobeniusoverF223^6 orequivalentkerneldescent beforeclaiming secondnondualchain. OriginalS3checkpoint/livebudget/capacity/reference/countercoverage andglobalfixture/mainledger/signing/review/conflict/vault/KB gaps persist. No computationaladvantage. Nextglobalroutine17:45UTC unlessmaterialsignal. Continueaftereachrung. [pcc:unverified]\n'''
for p in [s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md',r/'docs/session-logs/2026-11-13.md']:
 with p.open('a')as f:f.write(entry)
q=s/'QUEUE.md';q.write_text('Current checkpoint: PUBLIC_TARGET_CHECKPOINT.md, actual q223 completed descent and two-primary continuation.\n\n'+entry+'\nEarlier entries follow as historical context.\n\n'+q.read_text())
Path('/workspace/scratch/ct59-root-publication-expected.json').write_text(json.dumps([x['path']for x in objects if x['requires_original_data_migration']],indent=2)+'\n')
