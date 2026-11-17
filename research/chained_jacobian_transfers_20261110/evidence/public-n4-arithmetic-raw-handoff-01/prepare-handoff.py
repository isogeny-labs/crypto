"""Whole-object handoff and a new current-runtime guard; no scientific computation."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';base='2b8a1e556e7961325783fe5c041f6f1cfc2116b2';assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-arithmetic-raw-handoff-01';handoff=json.loads((d/'handoff.json').read_text());assert len(handoff['attempts'])==6;objects=[]
for row in handoff['attempts']:
 for name,size,h,target in [(row['label']+'.tar.xz',row['archive_bytes'],row['archive_sha256'],'whole-raw.tar.xz'),(row['label']+'-members.json.xz',row['complete_index_xz_bytes'],row['complete_index_xz_sha256'],'members.json.xz')]:
  p=d/name;assert p.stat().st_size==size and hashlib.sha256(p.read_bytes()).hexdigest()==h
  objects.append(dict(path=str(p.relative_to(r)),bytes=size,sha256=h,requires_original_data_migration=size>65536,consumer_path=str((cs/'evidence'/('closed-ct72-'+row['label'])/target).relative_to(c)),role='whole-closed-archive'if target=='whole-raw.tar.xz'else'whole-complete-index'))
canonical=c/'docs/research-artifacts/chained-transfer/arithmetic-audit-01'
for source,carrier,alias,role in [(canonical/'report.pdf','arithmetic-final-report.pdf.bin',cs/'reports/arithmetic-audit-01/report.pdf','whole-final-PDF'),(canonical/'native-output.txt.xz','arithmetic-native-output.txt.xz',cs/'evidence/arithmetic-audit-native-acceptance-04/native04-native.txt.xz','whole-native-PCC-subject')]:
 raw=source.read_bytes();assert alias.is_symlink()and alias.resolve()==source
 (d/carrier).write_bytes(raw);objects.append(dict(path=str((d/carrier).relative_to(r)),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),requires_original_data_migration=len(raw)>65536,consumer_path=str(source.relative_to(c)),consumer_alias=str(alias.relative_to(c)),alias_relative_target=os.readlink(alias),role=role))
assert len(objects)==14 and sum(x['requires_original_data_migration']for x in objects)==8
old=json.loads((s/'evidence/public-n4-norm-sampling-current-guard-09/current-inputs.json').read_text());assert len(old)==5030
changes=[]
for p,h in old.items():
 actual=hashlib.sha256(Path(p).read_bytes()).hexdigest()
 if h!=actual:changes.append(dict(path=p,historical_sha256=h,current_sha256=actual,reason='canonical norm-assisted README availability appended AFTER genuine243bb9/9a7cf7 andhold09release; science/PDF/native bytes unchanged'))
assert {x['path']for x in changes}=={str(c/'docs/research-artifacts/chained-transfer/norm-assisted-IC-01/README.md')},changes
# Current transitive closure; released historical replay trees remain intact and
# their complete5030-map is checked separately above, never silently reused.
historical=['/workspace/scratch/ct67-odd-product-refinement-replay-layout-02/','/workspace/scratch/ct69-new-target-IC-replay-layout-01/','/workspace/scratch/ct70-common-origin-replay-layout-01/']
current={p:hashlib.sha256(Path(p).read_bytes()).hexdigest()for p in old if not any(p.startswith(prefix)for prefix in historical)}
full=json.loads(Path('/workspace/scratch/ct71-norm-sampling-full-01/before.json').read_text());assert len(full)==785
for p,h in full.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h;current[p]=h
for root,manifest,expected in [('/workspace/scratch/ct72-arithmetic-native-layout-03/original-layout','/workspace/scratch/ct72-arithmetic-native-layout-03/original-785.sha256',785),('/workspace/scratch/ct72-arithmetic-native-layout-03/derived-layout','/workspace/scratch/ct72-arithmetic-native-01/derived-before.sha256',None),('/workspace/scratch/ct72-arithmetic-small-root-layout-04','/workspace/scratch/ct72-arithmetic-native-04/root-before.sha256',19)]:
 rows=Path(manifest).read_text().splitlines();assert expected is None or len(rows)==expected
 for line in rows:
  h,rel=line.split('  ',1);p=(Path(root)/rel).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==h;current[str(p)]=h
ref=cs/'arithmetic-audit/reference-01.sha256';rows=ref.read_text().splitlines();assert len(rows)==19
for line in rows:
 h,rel=line.split('  ',1);p=c/rel;assert hashlib.sha256(p.read_bytes()).hexdigest()==h;current[str(p)]=h
selected=[ref,*canonical.iterdir(),*list((cs/'reports/arithmetic-audit-01').iterdir()),cs/'evidence/arithmetic-audit-native-acceptance-04/native04-native.txt.xz']
for p in selected:
 if p.is_file():current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert len(links)==5,links
g=s/'evidence/public-n4-arithmetic-current-guard-10';g.mkdir();(g/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n');assert(g/'current-inputs.json').stat().st_size<1048576
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(g/'historical-09-to-current.json').write_text(json.dumps(changes,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),current_guard_count=len(current),version='NEW10/current transitiveclosure',previous09='released historical5030 separatelychecked/allfileskept/ONLYpostpublicationcanonicalnormREADMEdeltamapped',historical_replay_trees_not_in_current_runtime_closure=historical,historical_replay_files_preserved=True,current_original785_full_and_native785_derivative_and_smallroot19_and_newsource19plusself20=True,original_root_inputs_binary_and_authenticfixture_retained=True,all_report_PDFs_native_subjects_link_types_targets=True,hold='ALLselectedbytes and FIVElinktypes/targets held untilgenuinebotFF/finalguards; onlydisjointmetadata/additivefiles maychange'),indent=2)+'\n')
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');shutil.copyfile(__file__,d/'prepare-handoff.py')
receipt=Path('/workspace/scratch/ct72-staging-dedup-receipt-01.json');(d/'whole-staging-dedup-receipt.json.xz').write_bytes(lzma.compress(receipt.read_bytes()));assert(d/'whole-staging-dedup-receipt.json.xz').stat().st_size<=65536
(d/'scope.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,whole_closed_captures=6,complete_indices=6,canonical_wholePDF_and_nativeXZ=2,objects=14,original_migrations=8,small_whole_Git_indices=6,current_guards=len(current),current_guard_bytes=(g/'current-inputs.json').stat().st_size,links=5,source_checkpoint='ALL221ownedpendingfiles+base584+patch/PCCledger/resume/new19runtime+self20/reportwholePDF+nativeXZ+relativealiases;225capturemembers',native='8d0c27/all86exact/actualomittedhook0+101/vectors8c2406/7113rows/report03cfb7/THREE92135BpagesALLviewed',remote_availability='pending genuineoriginalpublisher',no_split_prune_policy_threshold_verifier_ortrust_change=True,staging_dedup='3948closedimmutable stagingcopies sharelocalinodes; allpaths/allbytes/alloriginals kept; whole1.96MBreceipt lossless64744BXZ retained directly',control_plane='actualAPI/logs/normalpush/discovery/FF/manifests remainDIRECT afterpublication, no recursivearchive'),indent=2)+'\n')
c=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','SIXwholeclosedcaptures+SIXcompleteindices+wholefinal92135BPDF+whole71856BnativeXZ14objects/EIGHToriginalmigrationpending/SIXsmallwholeGitindices/everymemberroundtrip. Includes852file785full333af1/862file17CI42073d/84filenonsecret21afcheckpoint66da4bALL28membersONLYprivate temporarydownloadreferenceexcludedwithhash/fullprivateoriginalkept;4016fileWHOLEarithmeticallattempts-source-layouts-exercisedbinaries/callerfailures/native8d0c27/all86exact/actualomittedhook0+101/vectors8c2406/7113rows/report03cfb7 ALLTHREEfinalpagesviewed;reportdrafts13files/sourcecheckpointALL221ownedpending+base584+patch/PCCledger/resume/new19runtime+self20/report+nativewholebytes+relativealiases225members/no recursion. NEWguard10actual'+str(len(current))+' CURRENTtransitiveinputs/19runtime+self20/native785+derivative+smallroot19/latestphysical785/rootoriginalbinaryfixture/FIVElinktypes HOLDuntilgenuineFF/finalguards. Old09historical5030separatelychecked/allhistoricalfilesretained/ONLYpostpublicationnormREADMEavailabilitydelta mapped. Currentmapexcludesonlythree nowhistoricalunexecutedphysicalreplaytrees, not deleted. Closedstagingdedup3948copies sharesinodes/allpathsbytesoriginalskept/WHOLE64744BXZreceipt direct. Remoteavailabilitypending/no executionverification/calibratedprice/stagewin/computationaladvantage/mergewaiver.']
for p in [d/'handoff.json',d/'objects.json',d/'scope.json',d/'whole-staging-dedup-receipt.json.xz',g/'custody.json',g/'symlink-targets.json',g/'historical-09-to-current.json']:c+=['--proof','attestation:'+str(p.relative_to(r))]
cl=subprocess.check_output(c,cwd=r,text=True).strip();Path('/workspace/scratch/ct72-root-arithmetic-handoff.claim').write_text(cl+'\n');Path('/workspace/scratch/ct72-root-publication-expected.json').write_text(json.dumps([x['path']for x in objects if x['requires_original_data_migration']],indent=2)+'\n');print(cl,flush=True)
state=f'''# Arithmetic whole-data handoff and guard10 — {datetime.now(timezone.utc).isoformat()}

Native8d0c27 oneunfilteredpass/all86exact/all225scalarpairs1125triples60ICpairs120reports/all15FB30failures/actualomittedhookpositive0negative101. Vectors8c2406 conserve7113rows:moreproducts/fewerinverses/unpriced, no computationaladvantage. Report03cfb7 THREEfinal92135BpagesALLviewed. Newread-only18thworkflow/new19source+self20/14positivephases/freshfull+ownCIawaitsourcepublication. Allunsigned. [pcc:unverified]

{cl}:SIXwholecaptures+SIXcompleteindices+wholePDF+nativeXZ14objects/8genuineS3pending+6smallGit. 852file785full/862file17CI/84file21aftransport(all28members)/4016fileALLarithmeticattempts-layouts-binaries-failures/13filedrafts/225memberALL221pending sourcefiles+base584+patch/PCCledger/resume/report/nativewholebytes+aliases. ONLYprivate temporarydownloadreference excludedwithhash/fulloriginalprivatekept. No splitprune/policychange. Whole64744Bstagingreceipt/3948closedduplicatecopieshardlinked/allpathsbytesoriginals retained. [pcc:unverified]

NEWguard10 HOLDS{len(current)} CURRENTtransitiveinputs/FIVEPDF-native-linktypes andtargets untilactual8remote replays/genuinebotFF/finalguards+BOTHereposownedbytes. Historical09released5030allchecked/ONLYpostpublicationnormREADMEdelta mapped; old703/730/755physicaltreesretained butnotcurrentruntime. Normalrootpayloadpush/actualoriginalpublisher/exactFF/projections; thennormalconsumercommit/freshfull/own18CI andnextunblockedresearch. [pcc:unverified]

Root124clock101/mainledger/signing/reviews/conflict/globalgeometry/beta/logextension/completecoldonlinecost/strongestrefs/livebudgetcapacity/vault8edec422locked/KB127 remain. No paidbenchmark ormergewaiver. [pcc:unverified]
'''
for p in [s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(r/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n## '+datetime.now(timezone.utc).isoformat()+' — arithmetic whole-data handoff\n\n### What was done (commands, files, features)\n'+state+'\n### Decisions made (and why)\nUseunchangedgenuinepublisher/currentguard10/directcontrolplane; preservealloriginalsources/PCCsubjects/thresholds. [pcc:unverified]\n\n### Open threads (unfinished, pick up next time)\nNormalpayloadpush/actualremotereplay/exactFF/allguards+ownedbytes+FIVElinktypes/projections,thenconsumercommit/freshfull/own18CI. Continueaftereachrung. [pcc:unverified]\n\n### Errors (what went wrong, how it was handled)\nAllnegativearithmetic/caller/controlrecords archivedwhole. No currentpublicationfailure. Originalroot/formal/merge/admission/vaultKB gapsremain. [pcc:unverified]\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(json.dumps(dict(objects=14,large=8,small=6,guards=len(current),guard_bytes=(g/'current-inputs.json').stat().st_size,links=links)),flush=True)
