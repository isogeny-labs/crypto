import hashlib,json,pathlib,datetime
p=pathlib.Path(__file__).parent
old=pathlib.Path('/var/tmp/pencil-continuation-minimal-amendment-20261031-v1')
sha=lambda q:hashlib.sha256(q.read_bytes()).hexdigest()
bind={n:sha(old/n) for n in ['manifest.json','amendment-fields.json','variable-ledger.csv','CHECKLIST.md']}
assert bind['amendment-fields.json']=='31a1279b3a95c319848221ea48117d68b7cc20be13d553b16db0ac0a0420c78b'
d={
 'schema':'pencil.continuation_retry_clarification/v1',
 'status':'proposed_addendum_not_frozen_not_admitted',
 'parent_packet':str(old),'parent_files_sha256':bind,
 'scope':'Clarify retry classification and bounded scheduling; all other parent fields unchanged.',
 'execution_authorized':False,'canonical_protocol_validation_performed':False,
 'retry_proposal':{
  'maximum_infrastructure_attempts_total':2,'initial_attempt_included':True,'maximum_retries':1,
  'counter_scope':'Same exact prepared execution launch across the entire continuation, including owner/controller restarts; never reset by a new process, request ID, owner or calendar interval.',
  'eligible_failure':'Authenticated infrastructure failure before the scientific payload was ever started.',
  'required_evidence':'Authenticated owner/controller launch receipts must positively establish that the payload never crossed the launch/start boundary, bound to the exact request, execution, instrument, owner and job/process identities and complete attempt history. The precise producer boundary and receipt verification must be source-reviewed and bound in the prospective protocol.',
  'insufficient_evidence':['missing stdout','missing output rows','missing receipts','an exec error alone','a Pod or Job status alone','an unauthenticated local JSON claim'],
  'must_match':['exact inputs','exact instrument','exact prepared execution identity and seed','existing campaign ledger and remaining allowance','original absolute deadline'],
  'ineligible_fail_closed':['payload started','payload spawn cannot be excluded','post-spawn failure','preemption after spawn','restore/recovery of a started payload','scientific timeout','unknown timeout classification','coefficient or memory cap','wrong answer','missing or contradictory launch evidence'],
  'automatic_scientific_retry':False,
  'unknown_classification':'Stop and preserve evidence; no automatic launch or retry. Any replacement scientific execution requires a separate prospective decision and valid remaining authorization.',
  'second_failed_attempt':'Stop; no third attempt.',
  'admission_before_retry':'Retry is only eligible, not automatically admitted: authenticate no-start evidence, reconcile absence of live owner/payload, charge prior work and verify remaining ledger/deadline, retain all original records, and satisfy the frozen protocol and execution bindings.'
 },
 'accounting':{
  'cumulative_active_worker_seconds':25200,'absolute_deadline_utc':'2026-10-31T06:00:00Z','maximum_active_scientific_payloads':1,
  'prior_allowances_reset':False,'charged':['initial attempts','eligible infrastructure retries','all failed and partial work','candidate/reference/A-A','warmups','replay and validation','diagnostic and owner work','collection','recovery','active report work'],
  'partial_rows_and_original_failures_retained':True,'replace_partial_with_success':False,
  'remaining_allowance_source':'Authenticated cumulative owner/campaign ledger; no estimate from elapsed wall clock and no restart reset.',
  'stop':'Stop at the earliest applicable cumulative or absolute limit. Collection and recovery also remain within the same limits. Unknown accounting blocks admission.'
 },
 'scheduling':{
  'bounded_censored_campaign':True,'completion_assured':False,
  'runtime_estimate_seconds':None,
  'per_execution_timeout_seconds':3600,'proposed_collection_reserve_seconds':1800,'proposed_latest_payload_stop_utc':'2026-10-31T05:30:00Z',
  'reserve_status':'Unapproved scheduling proposal; may only tighten limits, never extend them; not a measured duration or completion guarantee.',
  'incomplete_outcome':'Retain all attempted and missing slots with capped/incomplete/unknown status and null unqualified ratios. The full 1260-target scope and separate 18-slot gate remain unchanged even if the campaign cannot finish them.'
 },
 'scientific_gate':{
  'maximum_changed_instrument_gate_attempts_proposed':1,
  'status':'Unapproved pending source and counter-equivalence review plus an explicit prospective scientific decision.',
  'known_C1_stop_preserved':True,
  'diagnostic_change_reopens_stop':False,
  'infrastructure_retry_rule_reopens_stop':False,
  'scope':'Separate p186379 target0 18-slot gate, not cohort enrollment; complete gate and independent review remain prerequisites to remainder.',
  'known_observations_retained':{'executions':7,'measured':4,'missing':11,'pencil_watchdog_timeouts':2,'pencil_solves':0}
 },
 'text_replacements_proposed':[
  {'path':'execution_proposal.infrastructure_retry_proposal','instruction':'Replace ambiguous max_attempts wording with retry_proposal above when constructing the reviewed canonical protocol.'},
  {'path':'variable_ledger.factors[name=preemption].how','text':'Retain interruption, checkpoint and restore records. Any post-spawn/preemption/unknown-start failure stops automatic scientific retry; resumed clocks remain invalid. Only authenticated never-started-payload infrastructure failures may be eligible for at most one retry, two total attempts including initial, under unchanged inputs/instrument and remaining campaign limits.'},
  {'path':'variable-ledger.csv preemption row and CHECKLIST.md retry references','instruction':'Carry the same no-start boundary and two-total-attempt wording into final reviewed documents; prior documents remain immutable evidence.'}
 ],
 'independent_followup_review':None
}
(p/'clarification.json').write_text(json.dumps(d,indent=2)+'\n')
(p/'CLARIFICATION.md').write_text('''# Retry and scheduling clarification

This additive proposal resolves the retry ambiguity in the source-bound parent packet. It does not freeze a protocol or authorize execution. The prior packet remains unchanged.

**Two attempts means the initial attempt plus at most one retry.** The count follows the same exact prepared execution across owner/controller restarts; a new process or request does not reset it. A retry is eligible only for an authenticated infrastructure failure for which owner/controller launch receipts positively establish that the scientific payload never started. The exact launch/start boundary and receipt verification must be source-reviewed and prospectively bound. Missing stdout, rows, receipts, an exec error, a Pod/Job status, or an unauthenticated JSON assertion cannot establish nonexecution.

A started payload, uncertain spawn, post-spawn failure, preemption, restore, timeout of unknown classification, scientific timeout, cap or wrong answer fails closed: no automatic scientific retry. Preserve every partial row, failure and charge. An eligible no-start retry still requires reconciliation of live handles, authenticated evidence, exact unchanged inputs/instrument/seed, remaining budget and deadline, and the frozen protocol's admission conditions. A second failed infrastructure attempt ends that path.

The seven-hour (25,200 active worker-second) ceiling and 2026-10-31 06:00 UTC absolute deadline remain unchanged. Initial attempts, retries, failures, controls, warmups, validation/replay, diagnostic/owner work, collection, recovery and active reporting all count. No restart resets the ledger. Unknown accounting prevents admission. The proposed 05:30 payload cutoff and 1,800-second collection reserve only tighten scheduling; neither is a measured runtime or approved guarantee.

This is bounded, potentially censored scheduling: neither the per-execution cap nor campaign ceiling assures completion. Keep the full 1,260-target scope and separate 18-slot upper gate, recording all uncompleted slots and null unqualified ratios rather than reducing the denominator.

The distinct proposal for **one changed-instrument scientific gate remains unapproved**, pending source/counter-equivalence review and a prospective scientific decision. A diagnostic change and this infrastructure retry clarification do not reopen the known-C1 stop. The seven observed executions, four measured rows, eleven missing rows, two pencil watchdog timeouts and zero pencil solves remain recorded. Complete gate results and independent review remain prerequisites to remainder.

`clarification.json` supplies exact parent hashes, machine-readable rules and proposed final-document replacements. No source, runtime, owner or execution identity has been invented. No job, build or scientific run was launched by this addendum.
''')
(p/'check-result.json').write_text(json.dumps({'scope':'static addendum consistency and immutable parent digest check','passed':True,'checks':['parent amendment digest matches sealed proposal','two total attempts = initial plus one retry','25200-second ceiling and original deadline unchanged','scientific gate remains unapproved','full1260 and separate18 preserved','no execution authorization']},indent=2)+'\n')
files={q.name:{'sha256':sha(q),'bytes':q.stat().st_size} for q in sorted(p.iterdir()) if q.is_file() and q.name!='manifest.json'}
(p/'manifest.json').write_text(json.dumps({'schema':'research.private-packet-manifest/v1','files':files},indent=2)+'\n')
print(json.dumps({'path':str(p),'clarification_sha256':sha(p/'clarification.json'),'manifest_sha256':sha(p/'manifest.json')}))
