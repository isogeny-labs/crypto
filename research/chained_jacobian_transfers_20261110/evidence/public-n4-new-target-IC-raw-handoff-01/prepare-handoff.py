"""Whole data objects and a fresh actual-current scientific-input guard."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,lzma,os,shutil,subprocess
r=Path('/workspace/crypto');s=r/'research/chained_jacobian_transfers_20261110';c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';base='51e0a088e16a9b04750d14281f2451faa37db990'
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()==base
d=s/'evidence/public-n4-new-target-IC-raw-handoff-01';handoff=json.loads((d/'handoff.json').read_text());assert len(handoff['attempts'])==12
objects=[]
for row in handoff['attempts']:
 label=row['label']
 for name,size,h,target in [(label+'.tar.xz',row['archive_bytes'],row['archive_sha256'],'whole-raw.tar.xz'),(label+'-members.json.xz',row['complete_index_xz_bytes'],row['complete_index_xz_sha256'],'members.json.xz')]:
  p=d/name;assert p.stat().st_size==size and hashlib.sha256(p.read_bytes()).hexdigest()==h
  objects.append(dict(path=str(p.relative_to(r)),bytes=size,sha256=h,requires_original_data_migration=size>65536,consumer_path=str((cs/'evidence'/('closed-ct69-'+label)/target).relative_to(c)),role='whole-closed-archive'if target=='whole-raw.tar.xz'else'whole-complete-index'))
report=cs/'reports/new-target-IC-01/report.pdf';canonical=c/'docs/research-artifacts/chained-transfer/new-target-IC-01/report.pdf';assert report.is_symlink()and report.resolve()==canonical and report.read_bytes()==canonical.read_bytes()
raw=canonical.read_bytes();assert len(raw)==69377;carrier=d/'new-target-IC-final-report.pdf.bin';carrier.write_bytes(raw)
objects.append(dict(path=str(carrier.relative_to(r)),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),requires_original_data_migration=True,consumer_path=str(canonical.relative_to(c)),consumer_alias=str(report.relative_to(c)),alias_relative_target=os.readlink(report),role='whole-final-PDF-raw-byte-carrier',content_type='application/pdf',policy_scope='root PDF Git exception does not apply to explicit binary carrier;consumer whole PDF is genuine S3 data;no conversion/split/prune/threshold/policy change'))
assert len(objects)==25
g=s/'evidence/public-n4-new-target-current-guard-07';g.mkdir()
current=json.loads(Path('/workspace/scratch/ct68-new-target-IC-native-01/before.json').read_text());assert len(current)==2543 and all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())
for line in(cs/'ci_new_target_ic/reference-01.sha256').read_text().splitlines():
 h,p=line.split('  ',1);p=(c/p).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==h;current[str(p)]=h
selected=[cs/'ci_new_target_ic/reference-01.sha256',canonical,canonical.parent/'README.md',cs/'NEW_TARGET_REPORT_STORAGE_PROTOCOL_01.md',cs/'ROOT_FIXTURE_REGRESSION_GUARD_PROTOCOL_02.md']
for folder in ['new-target-ic','ci_new_target_ic','reports','evidence/new-target-IC-bindings-01']:
 selected +=[p for p in(cs/folder).rglob('*')if p.is_file()]
for p in selected:current[str(p)]=hashlib.sha256(p.read_bytes()).hexdigest()
physical=json.loads(Path('/workspace/scratch/ct67-odd-product-refinement-full-02/before.json').read_text());assert len(physical)==703
for p,h in physical.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h;current[p]=h
root_inputs=json.loads(Path('/workspace/scratch/ct68-root-library-regression-03/before.json').read_text());assert len(root_inputs)==2137
for p,h in root_inputs.items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h;current[p]=h
links={p:os.readlink(p)for p in current if Path(p).is_symlink()}
(g/'current-inputs.json').write_text(json.dumps(current,indent=2)+'\n');(g/'symlink-targets.json').write_text(json.dumps(links,indent=2)+'\n')
(g/'custody.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),current_guard_count=len(current),native_runtime_inputs=2543,latest_published_CI=702,new_CI=729,new_CI_with_self=730,latest_completed_physical_full=703,original_root_library_inputs=2137,original_binary_and_authentic_fixture_included=True,additional='ALL current report/source/Lean/README/proofmap/bindings plus canonicalwholePDF and transparent report symlink/linktype',old_guards='06historical3241/released/NOT reused;olderfullsnapshots are closed artifacts,latest completed703 selected',hold='Do not edit selected physical bytes/linktargets until exact genuine botFF/finalguards;metadata checkpoints and later additive files allowed',scope='fresh actual-current union;not a stale historical guard'),indent=2)+'\n')
(d/'objects.json').write_text(json.dumps(objects,indent=2)+'\n');shutil.copyfile(__file__,d/'prepare-handoff.py')
large=[x for x in objects if x['requires_original_data_migration']]
(d/'scope.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),base=base,whole_closed_archives=12,complete_indices=12,whole_final_PDF=1,objects=25,requires_original_migration=len(large),small_whole_Git_objects=25-len(large),current_guard_count=len(current),guard_path=str(g.relative_to(r)),all_bytes_exact=True,remote_availability='pending actual unchanged trustedpublisher',control_plane='whole API/log losslessXZ/normalpush/FF/manifests direct;no recursive publication archive',no_split_prune_threshold_verifier_or_trust_change=True),indent=2)+'\n')
def claim(repo,statement,paths):
 cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement',statement]
 for p in paths:cmd+=['--proof','attestation:'+str(p.relative_to(repo))]
 print(repo.name,subprocess.check_output(cmd,cwd=repo,text=True).strip(),flush=True)
claim(r,'TWELVEwholeclosedcaptures+TWELVEcompleteindices+ONEwholefinalPDF25objects/'+str(len(large))+' requireoriginalunchangeddata migration/'+str(25-len(large))+'smallwholeGitobjects. Allfailed/success/model/maps/fullCI/authenticfixture/rootregression/renderdraft originals intact/everymemberhashroundtrip. FinalPDF69377bytes/d102 preservedasexplicitrawbinarycarrierforstricterconsumerS3policy;canonicaldocsPDF/relative researchlink keepunchangedPCC docs hydration andsubjectbytes;originalPCC2/negativecopies1missing1substitution/2restored. No formatconversion/split/prune/policythreshold/verifier/trust change. Freshguard07 actual'+str(len(current))+' inputs/native2543/current729CI+self/latest703physical/fullroot2137/originalbinaryfixture/ALLreports/57bindings/linktypes HOLDuntilgenuinebotFF/finalguards. Guard06historicalreleased/NOTreused. ActualICe869bd15cases60attempts/57bindings/reportd1027f;rootfull124guards0/two visiblefailures retained. Remoteavailabilitypending;unsigned/no cost/globalrootgreen.',[d/'handoff.json',d/'objects.json',d/'scope.json',g/'custody.json',g/'symlink-targets.json'])
rootraw=Path('/workspace/scratch/ct68-root-library-regression-03');log=(rootraw/'native.log').read_bytes();failures=[x.split('test ',1)[1].split(' ... ',1)[0]for x in log.decode().splitlines()if x.startswith('test ')and' ... FAILED'in x]
assert(rootraw/'native.exit').read_text().strip()=='124'and(rootraw/'guard.exit').read_text().strip()=='0'
summary=dict(original_exit=124,guard_exit=0,guards=2137,timeout_s=1800,command=['cargo','test','--release','--lib','--offline','--locked'],unfiltered_original_scope=True,visible_failure_labels_before_timeout=failures,full_suite_totals='unknown;not completed',authentic_fixture_targeted='1ea795 pass1/6329filtered does not make fullgreen',raw='closed-ct69-root-library-timeout/whole-raw.tar.xz',next='exact original diagnostics for two visible failure labels;no unchanged whole-suite rerun')
for repo,dest in [(r,s/'evidence/original-root-library-retry03-terminal'),(c,cs/'evidence/original-root-library-retry03-terminal')]:
 dest.mkdir();(dest/'observation.json').write_text(json.dumps(summary,indent=2)+'\n');z=lzma.compress(log);assert len(z)<65536;(dest/'whole-native.log.xz').write_bytes(z);shutil.copyfile(rootraw/'guards.json',dest/'guards.json')
 claim(repo,'Originalunfilteredrootlibraryretry03 SAME1800sbound exits124/ALL2137guardsunchanged afterauthenticfixture restored/samebinarytargetedpass. TWOvisible diagnostic-reap failurelabels before timeout;fullsuitedidnotcomplete/totalfailuresunknown. No testfilters/ignores/source/workflow/verifier change. Wholelog andclosed12thrawcapture retained;nextexactoriginalfailurediagnostics,not unchangedfullretry. Ordinaryunitexception/PCCbypass/checkpointpush1 remainexplicit,no benchmarkadmission/globalgreen/mergewaiver.',list(sorted(dest.iterdir())))
entry='''
## {at} — whole new-target IC data handoff and fresh guard07

ActualnewICe869bd/all2543guards/ALL60attempts/all15publiccases/82wholeJSON;written1e65f0/twofaithfulbindings01efbf671d42/bothoriginalgates2/57total;TWO69377-byte finalPDFpages ALLinspected d1027f. Current729CI/730outer/9phasepairs/14workflow190native57canonical prepared,not yet published or replayed. [pcc:unverified]

TWELVEwholeclosedcaptures+TWELVEcompleteindices+ONEwholefinalPDF25objects require {large}genuineS3migrations;other{small} wholeobjectsordinaryGit. Freshcurrentguard07 holds {count} scientific/code/proof/report/canonicalPDF+link/current729CI/latest703full/root2137 input bytes until exactgenuinebotFF/finalguards. Guard06historical3241/released/NOT reused. FinalPDFexplicitwholebyte .pdf.bin carrier appliesoriginalstricterdata policy;canonicaldocsPDF+researchsymlink preserveunchangedPCCdocs hydration/pins,negativecontrolsmissing/substitutionfail. No split/prune/threshold/policy/gate/verifier/trustchange;remoteavailabilitypending. [pcc:unverified]

Rootfull03 actuallyterminal124/1800s/all2137guards0/two visible diagnostic-reap failures;fulltotalsunknown. Wholecapture/log retained;next exact original failurediagnostics afterthisdataFF,not unchangedfullretry. OriginalS3checkpoint/livebudget/capacity/countercoverage/reference admission andfullgeometry/mainledger/signing/reviews/conflict/vault/KB gapsremain. Nextmateriallydifferent checkpoint route:originaluploader inCIcredentials withoutkeyexport/gateedit. Continue aftereachrung. [pcc:unverified]
'''.format(at=datetime.now(timezone.utc).isoformat(),large=len(large),small=25-len(large),count=len(current))
for p in [s/'PUBLIC_TARGET_CHECKPOINT.md',r/'.resume/RESUME.md',r/'docs/session-logs/2026-11-13.md',r/'problems/chained-transfer-admission-and-ledger-sync.md']:
 with p.open('a')as f:f.write(entry)
p=s/'QUEUE.md';p.write_text(entry+'\n---\n\n'+p.read_text())
Path('/workspace/scratch/ct69-root-publication-expected.json').write_text(json.dumps([x['path']for x in large],indent=2)+'\n')
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in current.items())and all(os.readlink(p)==target for p,target in links.items())
print(json.dumps(dict(objects=25,large=len(large),small=25-len(large),guard_count=len(current),symlinks=links)),flush=True)
