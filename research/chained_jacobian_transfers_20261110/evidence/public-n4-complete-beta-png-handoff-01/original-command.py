"""Whole checkpoint/artifact custody and a new hold; no scientific computation."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,io,json,lzma,os,shutil,subprocess,tarfile
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target'
base='556c5ddb454ebdea9a70e2a1a33bdb74152cfbf5';consumer='44df3f0ab0c06ba7bc6d5aaab3e3fe22236bfcb7'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=c,text=True).strip()==consumer
o=Path('/workspace/scratch/ct73-png-handoff-02');o.mkdir();d=s/'evidence/public-n4-complete-beta-png-handoff-01';d.mkdir(exist_ok=True);g=s/'evidence/public-n4-complete-beta-current-guard-12';g.mkdir(exist_ok=True)
at=datetime.now(timezone.utc).isoformat()
state=f'''# Whole report PNG and source checkpoint publication active — {at}

Native3f1ade/threeunfilteredpasses/all845guards0/all448exceptionals; conditionalgroup1e2420/original44declgates andIntd0400f/original1declgates/sevenbindings73partialunique; correctedreal-sitecounts[16,16]Phi/[1,1]origins/[8,8]isogenies withlegacy32actual101/fullthree-accounting-pathaudit/noothermathchanges. Reportf4a048 finalTWO48469BpagesALLviewed/frozen. [pcc:unverified]

Sourcegov582599 passesallrecordedgates exceptartifact1 onwhole151899BPNG. NEWguard12: unchangedgenuinepublisher forwholePNG,completeALLpendingconsumer-source+StageC-runcheckpoint,andwholecurrentguardmap. Preserve PNG/PCCsubjects/nativegoldens/no policyexceptions. HOLD scientificcurrent881+self882/previous5268paths+latestphysicalreplay/stagedproofs andbothrepoownedbytes untilactualremotereplays/exactbotFF/terminalguards. No sourceedits duringhold. [pcc:unverified]

Afterrelease projectgenuinePNGentryinto consumerreportmanifest/preservephysicalbytes; commitcompleteC20source; freshphysical882guardfull/freshoutsidecache andown20automaticCI/currentoriginalcheckpoint. UseCargo-reported executablepaths rather thanoldhardcodedunitname. ThenALLotherclosedcaptures ca9full/CInegative/helpercontrol/398CI/A-Bcomponents/C77fullcache+ownCI andnewcompletefull/CI underNEWguard13. CurrentStageC source/proof/native/reportattempts areincludedwholeinthischeckpoint. [pcc:unverified]

Source consumerparent44df/rootparent556. No localscienceinflight. Root124clock101/mainledger/signing/reviews/conflict andlivebudgetcapacity/matchingimage/calibratedcost gates persist. Vault8edec422locked/KB127. Continueaftereachrung; no mergewaiver orpaidcompute. [pcc:unverified]
'''
for p in[cs/'RESEARCH_QUEUE.md',cs/'CHECKPOINT.md',c/'.resume/RESUME.md']:p.write_text(state+'\n---\n\n'+p.read_text())
with(c/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n'+state+'\n')
scope=['experiments/chained-transfer-public-target','pcc/claims','pcc/ledger.jsonl','docs/session-logs/2026-11-14.md','.resume/RESUME.md','.gitattributes','.github/workflows/chained-transfer-beta-complete.yml','problems/chained-transfer-odd-product-kernel-and-odd-factor.md']
names=sorted((set(subprocess.check_output(['git','ls-files','--modified','--others','--exclude-standard','-z','--',*scope],cwd=c).split(b'\0'))|set(subprocess.check_output(['git','diff','--cached','--name-only','-z'],cwd=c).split(b'\0')))-{b''})
files=[(c/n.decode(),'consumer/'+n.decode())for n in names]
rawdirs=[*(Path('/workspace/scratch').glob('ct73-beta-complete-native-*')),*(Path('/workspace/scratch').glob('ct73-complete-beta-Lean-*')),*(Path('/workspace/scratch').glob('ct73-CRT-projection-native-*')),Path('/workspace/scratch/ct73-CRT-projection-Lean-01'),Path('/workspace/scratch/ct73-preprocessing-accounting-audit-01'),Path('/workspace/scratch/ct73-complete-report-draft02-source'),Path('/workspace/scratch/ct73-complete-report-render-failure-01'),Path('/workspace/scratch/ct73-complete-source-artifact-failure-01'),Path('/workspace/scratch/ct73-beta-custody-recorder-syntax-failure-01')]
rawdirs=[p for p in rawdirs if p.is_dir()]
rawdirs.append(Path('/workspace/scratch/ct73-png-handoff-preparation-failure-01'))
assert len(set(rawdirs))==len(rawdirs)
assert not any(d.iterdir()) and not any(g.iterdir())
for directory in sorted(rawdirs):
 assert directory.is_dir(),directory
 files.extend((p,'runs/'+directory.name+'/'+p.relative_to(directory).as_posix())for p in sorted(directory.rglob('*'))if p.is_file())
for p in sorted(Path('/workspace/scratch').glob('ct73-complete-report-*-page-*.png')):files.append((p,'rendered-pages/'+p.name))
assert len({name for _,name in files})==len(files)
patch=subprocess.check_output(['git','diff',consumer,'--binary','--full-index','--',*scope],cwd=c)
controls={'control/base.txt':(consumer+'\n').encode(),'control/pending-full.patch':patch,'control/RESUME.md':(c/'.resume/RESUME.md').read_bytes(),'control/snapshot.json':(json.dumps(dict(at=at,consumer_base=consumer,root_base=base,all_project_pending_files=len(names),whole_terminal_StageC_run_directories=[str(p)for p in rawdirs],scientific_results='3f1ade/1e2420/d0400f/f4a048 unsigned',not_covered='previous A-B/full/CI closed captures remain locally retained for NEWguard13'),indent=2)+'\n').encode()}
archive=o/'pending-source-and-StageC-checkpoint.tar.xz';members=[]
with tarfile.open(archive,'w:xz',dereference=False)as t:
 for path,name in files:
  assert not path.is_symlink(),path
  b=path.read_bytes();members.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()));t.add(path,arcname=name,recursive=False)
 for name,b in controls.items():
  info=tarfile.TarInfo(name);info.size=len(b);t.addfile(info,io.BytesIO(b));members.append(dict(path=name,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()))
with tarfile.open(archive,'r:xz')as t:
 assert len(t.getmembers())==len(members)
 for row in members:
  b=t.extractfile(row['path']).read();assert len(b)==row['bytes']and hashlib.sha256(b).hexdigest()==row['sha256']
(d/'complete-checkpoint-members.json').write_text(json.dumps(dict(members=members,all_pending_owned_files=len(names),all_StageC_directories_whole=True,StageC_directory_count=len(rawdirs),no_splits_or_pruning=True),indent=2)+'\n')
old=json.loads(Path('/workspace/scratch/ct73-guard12-old11-audit-01/current-old11-paths.json').read_text());changes=json.loads(Path('/workspace/scratch/ct73-guard12-old11-audit-01/changes.json').read_text());assert len(old)==5268
assert {x['path']for x in changes}=={str(c/'.github/workflows/chained-transfer-arithmetic-audit.yml'),str(cs/'arithmetic-audit/check.sh')}
for row in changes:row['reason']='AFTER hold11 release: dependency-only helper fetch/source398d591b, prospective7b2367/control34e2c1/ownCI4ff6d8; no arithmetic/solver/tap/golden changes'
current=old.copy()
for reference,root,n in[(cs/'ci_beta_complete/reference-01.sha256',c,881),(Path('/workspace/scratch/ct73-beta-replay-layout-01/cryptanalysis/experiments/chained-transfer-public-target/ci_beta_lift/reference-02.sha256'),Path('/workspace/scratch/ct73-beta-replay-layout-01/cryptanalysis'),844)]:
 rows=reference.read_text().splitlines();assert len(rows)==n
 for line in rows:
  h,rel=line.split('  ',1);path=(root/rel).resolve();assert hashlib.sha256(path.read_bytes()).hexdigest()==h,rel;current[str(path)]=h
 current[str(reference)]=hashlib.sha256(reference.read_bytes()).hexdigest()
for stage in sorted((r/'target').glob('ct73*')):
 if stage.is_dir():
  for path in stage.glob('*.lean'):current[str(path)]=hashlib.sha256(path.read_bytes()).hexdigest()
for line in Path('/workspace/scratch/ct73-beta-full-01/driver/producer-staging.sha256').read_text().splitlines():
 h,path=line.split('  ',1);assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==h;current[path]=h
report=cs/'reports/beta-complete-rational-01'
for path in sorted(report.iterdir()):
 if path.is_file():current[str(path)]=hashlib.sha256(path.read_bytes()).hexdigest()
links={p:os.readlink(p)for p in current if Path(p).is_symlink()};assert len(links)==6
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
raw=(json.dumps(current,indent=2)+'\n').encode();(o/'current-inputs.json').write_bytes(raw);compressed=lzma.compress(raw);(o/'whole-current-inputs.json.xz').write_bytes(compressed)
(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n');(g/'historical-11-to-current.json').write_text(json.dumps(changes,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=at,NEWguard12=len(current),current_source_inputs=881,manifest_self=1,prior5268_paths_all_preserved=True,prior_changes=changes,latest_physical_beta_replay_inputs=845,original_root_binary_and_fixture='already in retained prior union',actual_staged_proof_sources=True,links=links,hold='NO science edits until genuine remote replay/exact bot FF/currentguard+bothrepoownedbytes terminal; controlplane receipts outside repositories',raw_map=dict(path=str(o/'current-inputs.json'),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()),whole_map_XZ=dict(path=str(o/'whole-current-inputs.json.xz'),bytes=len(compressed),sha256=hashlib.sha256(compressed).hexdigest()),no_pruning_or_policy_change=True),indent=2)+'\n')
png=report/'complete-diagram.png';assert png.stat().st_size==151899
objects=[]
for src in[png,archive,o/'whole-current-inputs.json.xz']:
 dest=d/src.name;shutil.copyfile(src,dest);b=dest.read_bytes();objects.append(dict(path=str(dest.relative_to(r)),bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),local_original=str(src),requires_original_migration=True))
assert len(objects)==3
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');shutil.copyfile(__file__,d/'original-command.py')
(d/'scope.json').write_text(json.dumps(dict(at=at,root_base=base,consumer_base=consumer,objects=objects,checkpoint_members=len(members),all_source_and_selected_StageC_run_directories_whole=True,StageC_directory_count=len(rawdirs),PNG_subject='f4a048 exact151899B/no resizing',guard12=len(current),remote_availability='pending unchanged genuine publisher; each whole object must be replayed before bot',remaining='prior ca9/helper/398/A-B/C77 captures and future completefull/CI need NEWguard13',scope='data custody, unsigned; no source/test/verifier/policy/native changes'),indent=2)+'\n')
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Whole PNG/source handoff: THREEexactwholeobjects151899BPNG f4a048 +ALLpendingconsumerfiles/base44df/patch/PCCledger/resume/report/newCI20 ANDallselectedwholeStageCnative-Lean-CRT-report-failure directories checkpoint +whole current-input mapXZ. Every checkpoint member roundtripped/hashchecked, no pruning/splitting/resize/policy change. NEWguard12 '+str(len(current))+' paths retains ALLprior5268+current881self+latestphysical845+actualproofstages/rootsdiagnosticbinaryfixture/SIXlinktypes; ONLYtwo postreleased11 dependency-helper398 changes mapped. HOLDuntilgenuine3remote-byte replays/exactbotFF/bothrepoownedbytes/allguards. Previous A-B/ca9/helper/398/C77 largecaptures andfuturecompletefullCI remainpending NEWguard13. Allunsigned.']
for path in[d/'objects.json',d/'scope.json',d/'complete-checkpoint-members.json',g/'custody.json',g/'symlink-targets.json',g/'historical-11-to-current.json']:cmd+=['--proof','attestation:'+str(path.relative_to(r))]
cl=subprocess.check_output(cmd,cwd=r,text=True).strip();(o/'handoff.claim').write_text(cl+'\n');(o/'expected-migration-paths.json').write_text(json.dumps([x['path']for x in objects],indent=2)+'\n');print(cl,flush=True)
for path in[s/'QUEUE.md',s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md']:path.write_text(state+'\nRoot handoff '+cl+' / guard12 '+str(len(current))+' / threewholeobjects; archive'+str(len(members))+'members. Publisherpending, no scienceedits. [pcc:unverified]\n\n---\n\n'+path.read_text())
with(r/'docs/session-logs/2026-11-14.md').open('a')as f:f.write('\n## '+at+' — whole complete-beta report PNG and resumable source handoff\n\n### What was done (commands, files, features)\n'+state+'\n'+cl+' preserves3wholeobjects/allpendingconsumer+allselectedwholeStageCdir checkpoint/allmembers andnewguard12. [pcc:unverified]\n\n### Decisions made (and why)\nOriginalPNG policy rejection582599 requiresgenuinepublisher; preservewholeimage andallsource/claims/binaries/proofs/drafts, no threshold/solver/verifier change. [pcc:unverified]\n\n### Open threads (unfinished, pick up next time)\nRootpayloadnormalpush/automaticoriginalpublisher/3replays/exactFF/terminalguardrelease/consumerprojection; sourcecommit/fresh882full/own20CI; remainingwholedata NEWguard13. [pcc:unverified]\n\n### Errors (what went wrong, how it was handled)\nOriginalartifact1 preserved, all mathematical checks unchanged; no currentpublishing resultclaimed. [pcc:unverified]\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
print(json.dumps(dict(objects=[dict(path=x['path'],bytes=x['bytes'])for x in objects],members=len(members),guards=len(current),links=len(links))),flush=True)
