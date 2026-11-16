from pathlib import Path
import json,hashlib,datetime
p=Path(__file__).parent;root=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');ret=Path('/home/ec2-user/pencil-controller-review-20261029/c1-takeover-20261030');h=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
inputs=json.loads((ret/'pencil-preflight-proposal-v1/inputs.json').read_text());rows=[]
for r in inputs:
 actual=h(Path(r['path']));rows.append(dict(r,actual_sha256=actual,hash_matches=actual==r['sha256']))
assert all(r['hash_matches'] for r in rows)
(p/'input-reconciliation.json').write_text(json.dumps({'rows':rows,'enrolled_workloads':sum(r['workloads'] for r in rows[:8]),'enrolled_slots':sum(r['slots'] for r in rows[:8]),'enrolled_warmups':sum(r['warmups'] for r in rows[:8]),'separate_gate_slots':rows[8]['slots']},indent=2)+'\n')
checks=[]
for manifest,base in [(ret/'controller-progress-static-qualification-v1/SOURCE-MANIFEST.json',root),(ret/'fresh-owner-static-qualification-v2/source-manifest.json',root/'taskq/deploy/eks-karpenter/resume/fresh-owner')]:
 d=json.loads(manifest.read_text());rs=[]
 for name,expected in d.items():
  f=base/name;actual=h(f) if f.exists() else None;rs.append({'path':str(f),'expected_sha256':expected,'actual_sha256':actual,'matches':actual==expected})
 checks.append({'manifest':str(manifest),'manifest_sha256':h(manifest),'rows':rs,'all_match':all(x['matches'] for x in rs)})
(p/'source-reconciliation.json').write_text(json.dumps(checks,indent=2)+'\n')
auth=root/'research/pencil_lifecycle_20261030/integration/continuation-planning-20261030/authorization.json';ad=json.loads(auth.read_text());prior=root/'research/pencil_large_prime_20261029/runs/p186379-gate-20261030-c1/terminal-g18-observations';obs=json.loads((prior/'observation.json').read_text());rs=obs['rows'];counts={'recorded_executions':len(rs),'measured':sum(not r['warmup'] for r in rs),'warmups':sum(r['warmup'] for r in rs),'pencil_timeouts':sum(r['arm']=='pencil' and r['outcome']['status']=='timeout' for r in rs),'pencil_verified':sum(r['arm']=='pencil' and r['outcome']['status']=='verified' for r in rs),'missing':18-len(rs),'active_worker_seconds':obs['total_active_worker_seconds']};assert counts['pencil_timeouts']==2
(p/'budget-and-known-data.json').write_text(json.dumps({'observed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'authorization':ad,'authorization_sha256':h(auth),'known_C1':counts,'remaining_active_worker_seconds':None,'spent_active_worker_seconds':None,'outstanding_reservations_seconds':None,'balance_status':'unreconciled; absence of new admission is not a zero ledger','known_data_source_sha256':h(prior/'observation.json'),'deadline_extended':False,'no_scientific_retry':True},indent=2)+'\n')
base=json.loads((ret/'known-c1-amendment-draft/amendment-draft-v2.json').read_text());minimal=Path('/var/tmp/pencil-continuation-minimal-amendment-20261031-v1/amendment-fields.json');retry=Path('/var/tmp/pencil-continuation-retry-clarification-20261031-v1/clarification.json')
ledger=[]
def add(field,status,value,evidence,next_action):ledger.append({'field':field,'status':status,'value':value,'evidence':evidence,'next_action':next_action})
add('resource.authorization','bound_planning_only',ad,str(auth),'Keep25200 and06:00 unchanged; bind canonical reviewed protocol separately.')
add('cohort.enrollment','hash_reconciled',{'targets':1260,'slots':22680,'warmups':3780,'measured_pairs':6300,'plans':8},'input-reconciliation.json','Reuse exact plans and seeds; do not regenerate or count separate gate in enrollment.')
add('known_data.C1','retained',counts,str(prior/'observation.json'),'Preserve seven rows/eleven missing/two timeouts; ratios remain null.')
add('resource.spent_remaining_outstanding','unresolved',None,'budget-and-known-data.json','Read authenticated complete campaign/owner ledger including open reservations and all charged work; do not initialize zero or subtract calendar time.')
add('owner.remote_reconciliation','partially_observed',{'isolab_pencil_jobs':0,'recovery_owner_live_status':None},'external-observation.json;status-projection.json','Compatible read-only durable state retrieval and exact owner/controller terminal closure required; helper rejects memory schema; no role expansion.')
add('resource.worker_unit_and_enforcement','prospective_unbound',None,str(minimal),'Bind reviewed conservative reservation policy, campaign UID/CAS revision, worker unit and observed allocation closure; nested owner intervals are not additive worker charges.')
add('resource.cutoff_and_reserve','proposed_not_admitted',{'latest_payload_stop':'2026-10-31T05:30:00Z','reserve_seconds':1800,'absolute_end':'2026-10-31T06:00:00Z'},str(retry),'Review actual enforcement/grace bound; tighten cutoff if needed; no completion guarantee.')
add('retry.policy','prospective_clarification',{'total_attempts':2,'retries':1,'only_authenticated_never_started':True,'automatic_scientific_retry':False},str(retry),'Bind exact launch/start producer boundary and all-attempt identities; uncertainty/postspawn/preemption/timeouts stop.')
add('gate.decision','unapproved',{'separate_target':'p186379-target0','slots':18,'measured_replays':15,'max_new_scientific_attempts':1},str(minimal),'Prospective scientific decision must address known C1 stop and unchanged algorithm; diagnostics do not reopen it.')
add('instrument.controller_owner_sources','reconciled',{'manifests_match':[x['all_match'] for x in checks]},'source-reconciliation.json','Reuse retained bytes subject to final exact package/source review.')
add('instrument.final_ecbench','await_root_receipt',None,None,'Join corrected-source final binary, toolchain/flags, source tree and donor algorithm/counter equivalence review; historical b985 not substituted.')
add('instrument.image_runtime_qualification','pending',None,None,'Root exact local assembly and Q0-Q4 qualify software only; bind actual runtime/custody qualification required by scientific protocol.')
add('publication.new_package_versions','pending_approval_and_receipts',None,None,'Review exact new package keys/hashes; prior approval of other bytes is not publication of these artifacts; retain versioned download/readback.')
add('request.launch_identity','unallocated',None,None,'Allocate real prospective request only after reviewed protocol and valid balance; real Job/Pod/owner IDs come from actual receipts, never placeholders.')
add('session.preparation_context','unobserved',None,None,'Bind actual native session/preparation/clock/host/enrollment certificate/ordered context subset and owner receipts before accepting rows.')
add('p739.reuse','pending_exact_original_rows_review',None,str(ret/'known-c1-amendment-v2-independent-review.md'),'Original fresh rows need replay or reviewed exact non-wall equivalence; 900 computation reproductions alone insufficient; old timing invalid.')
add('timing.L2_AA_fresh','unqualified',None,str(minimal),'Retain individual-target L2/fresh/A-A obligations; changed observer endpoint is not method timing.')
add('analysis.finite_price_mapping','preserved_mapping_pending_binary',{'square':[0,0.5,1],'inverse':[1,10,100,1000],'native':[0,0.1,1],'cells':36},str(ret/'known-c1-amendment-draft/amendment-draft-v2.json'),'Bind source/export executable and exact operation coverage; incomplete/capped pairs retain null ratios.')
add('protocol.canonical_validation_review_freeze','not_performed',None,str(minimal),'Join claims/variable ledger/controls/seeds/metrics/custody/retry/source review; canonical validate and independent review before freeze and affected data.')
(p/'remaining-binding-ledger.json').write_text(json.dumps({'schema':'pencil.continuation_binding_ledger/v1','planning_only':True,'execution_admitted':False,'fields':ledger},indent=2)+'\n')
refs=[auth,ret/'known-c1-amendment-draft/amendment-draft-v2.json',ret/'known-c1-amendment-v2-independent-review.md',minimal,retry,prior/'observation.json',prior/'terminal-observed-state.json',prior/'review-update/independent-terminal-review.json',root/'taskq/deploy/eks-karpenter/mcp/src/main.rs',root/'taskq/deploy/eks-karpenter/resume/controller/src/cloud.rs',root/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/continuation-audit/AUDIT.md']
(p/'source-bindings.json').write_text(json.dumps([{'path':str(f),'sha256':h(f)} for f in refs],indent=2)+'\n')
print('source manifests',[(len(x['rows']),x['all_match']) for x in checks]);print('known C1',counts)
