from pathlib import Path
import json,hashlib,tarfile,re,csv
D=Path(__file__).resolve().parent;P=D.parent;R=P/'pivot-control-results-20261111';S=P/'pivot-control-admitted-20261111';F=P/'pivot-v2-results-20261111';h=lambda b:hashlib.sha256(b).hexdigest();J=lambda f:json.loads(f.read_text())
raw=J(R/'raw-manifest.json')['files'];data={}
with tarfile.open(R/'raw-receipts.tar.gz') as t:
 for m in t:
  assert m.isfile() and m.name in raw and m.name not in data;b=t.extractfile(m).read();assert len(b)==raw[m.name]['bytes'] and h(b)==raw[m.name]['sha256'];data[m.name]=b
assert set(data)==set(raw) and len(raw)==64
j=lambda n:json.loads(data[n]);cmd=j('native/commands.json');assert [r['module'] for r in cmd]==['EliminationField','PivotSwap','Controls','Readback','ExpectedFailure'];assert [r['exit'] for r in cmd]==[0,0,0,0,1]
assert all(r['exit']==r['expected_exit'] and r['reaped'] and not r['remaining_group'] for r in cmd)
for r in cmd:
 assert r['argv'][0]=='/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1/bin/lean' and r['argv'][1:4]==['-j1','-M3500','-s16384']
 assert data['native/'+r['module']+'.lean']==(S/(r['module']+'.lean.txt')).read_bytes()
expected=sum(J(S/'names.json').values(),[]);assert len(expected)==len(set(expected))==17;seen={}
for line in data['native/Readback.stdout'].decode().splitlines():
 m=re.fullmatch("'([^']+)' depends on axioms: \\[(.*)\\]",line)
 if m:name=m[1];axioms=[s.strip() for s in m[2].split(',') if s.strip()]
 else:
  m=re.fullmatch("'([^']+)' does not depend on any axioms",line);assert m,line;name=m[1];axioms=[]
 assert name not in seen;seen[name]=axioms
assert set(seen)==set(expected) and set(sum(seen.values(),[]))<={'propext','Classical.choice','Quot.sound'}
assert seen==J(R/'axiom-readback.json')['observed_axioms'];negative=data['native/ExpectedFailure.stdout'].decode();assert 'unsolved goals' in negative and '⊢ False' in negative
pre=j('native/pre-hashes.json');post=j('native/post-hashes.json');assert pre==post and len(pre)==16736 and sum(v['bytes'] for v in pre.values())==3111892318
assert j('native/pre-runtime.json')==j('native/post-runtime.json')
for name in ['pre','compiler','post']:assert j('native/'+name+'-receipt.json')['status']=='passed'
completion=j('outer/completion.json');assert completion['status']=='passed' and all(x['passed'] and x['terminal_owner_confirmed'] and x['returncode']==0 for x in completion['phases'])
caller=j('caller/caller-result.json');assert caller['exit_code']==0
freeze=J(S/'freeze.json');release=j('outer/coordinator-release.json');assert release['freeze_sha256']==h((S/'freeze.json').read_bytes()) and release['released'] is True
for name,digest in freeze['files'].items():assert h((S/name).read_bytes())==digest,name
for name,row in release['receipts'].items():assert h(Path(row['path']).read_bytes())==row['sha256'],name
assert release['prior_debit_seconds']==18180 and release['remaining_before_attempt_seconds']==111420 and release['campaign_total_seconds']==129600
summary=J(R/'summary.json');assert summary['attempt_charge_seconds']==2400 and summary['campaign_charged_seconds']==20580 and summary['campaign_remaining_seconds']==109020
failed=J(F/'summary.json');assert failed['status']=='failed' and 'Decidable' in failed['failure'] and failed['unattempted_modules']==['Readback','ExpectedFailure']
# Reconcile the retained predecessor's failing output, not merely its summary.
with tarfile.open(F/'raw-receipts.tar.gz') as t:
 errors=[]
 for m in t:
  if m.name.endswith(('Controls.stdout','Controls.stderr')):errors.append(t.extractfile(m).read().decode())
 assert any('failed to synthesize' in s and 'Decidable' in s for s in errors)
result={'schema':'research.recorded-machine-evidence-review/v1','scope':'Independent raw-record consistency audit; no fresh Lean or kernel verification','reviewer':'mode_source_review','authorship':{'produced_native_result':False,'executed_native_workflow':False,'prior_source_review_and_operational_preparation':True,'blind':False,'independent_mathematical_rederivation':False},'finding':'No discrepancy found in the inspected record bindings','archive_files_verified':64,'frozen_packet_files_verified':len(freeze['files']),'release_receipt_pins_verified':len(release['receipts']),'source_modules_matched':5,'named_readbacks':17,'observed_axioms':seen,'command_exits':[0,0,0,0,1],'false_control_rejected':True,'recorded_owners_terminal':True,'custody':{'pre_post_equal':True,'dependency_files':16736,'dependency_bytes':3111892318,'runtime_maps_equal':True,'live_dependency_rehash_performed':False,'limitation':'Pinned shared caches; pre/post equality does not exclude transient concurrent change.'},'retained_prior_failure':{'path':str(F),'summary_sha256':h((F/'summary.json').read_bytes()),'failure':failed['failure'],'raw_failure_output_checked':True},'conservative_charge_seconds':2400,'campaign_update_performed':False,'independent_kernel_recheck':False,'compiler_invocations_by_reviewer':0,'universal_claim':False,'official_promotion':False,'subject_hashes':{str(f):h(f.read_bytes()) for f in [R/'summary.json',R/'raw-manifest.json',R/'raw-receipts.tar.gz',S/'freeze.json',S/'names.json',F/'summary.json']}}
(D/'review.json').write_text(json.dumps(result,indent=2)+'\n')
with (D/'readbacks.csv').open('w',newline='') as f:w=csv.writer(f);w.writerow(['name','axioms','present']);w.writerows((n,';'.join(seen[n]),True) for n in expected)
print(json.dumps({'archive':64,'freeze_pins':len(freeze['files']),'release_pins':len(release['receipts']),'names':17,'finding':result['finding']}))
