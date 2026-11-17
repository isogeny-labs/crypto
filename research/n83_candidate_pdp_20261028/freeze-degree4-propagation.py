"""Metadata-only adapter. All polynomial, closure and certificate work is Rust."""
from pathlib import Path
import copy,hashlib,json,subprocess
s=Path(__file__).resolve().parent
scratch=Path('/home/ec2-user/n83-candidate-recovery')
witness=scratch/'degree4-witness-r1'
read=lambda p:json.loads(p.read_text())
def sha(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def ref(p):return {'sha256':sha(p),'bytes':p.stat().st_size}
p=read(s/'protocol-degree4-witness.json')
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
p.update(protocol_id='N83-DEGREE4-PROPAGATION-20261029-R1',created='2026-10-29',question='Can restricted propagation of the verified degree4 witnesses produce a unit or exhaustive affine completion and hence global LFD upper4 on the unchanged twelve retained m3 cases?',links={'design':'DEGREE4_PROPAGATION_DESIGN.md'})
p['claims'][0]['text']='Every admitted row has a degree<=4 derivation. Unit or exhaustive affine completion proves global LFD<=4; with the replayed inherited strict lower4 this gives exact LFD4. A cap or restricted queue exhaustion without either certificate gives no global upper bound and no lower5.'
p['claims'][1]['text']='Universal FFD/LFD, full unrestricted V4 computation, elite-base/regularity improvement, performance and ECDLP recovery remain outside this batch and open in the broader objective.'
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=s,text=True).strip()
binary=scratch/'target/release/degree4_propagation'
sources=['src/bin/degree4_propagation.rs','src/degree4_propagation_parent.rs','src/degree4_propagation_impl.rs','DEGREE4_PROPAGATION_DESIGN.md','DEGREE4_PROPAGATION_FORMAT.json','run-degree4-propagation.sh']
source_refs={name:ref(s/name) for name in sources}
for arm in p['arms']:
    arm['instrument'].update(commit=head,binary_sha256=sha(binary));arm['instrument']['build'][0][-1]='degree4_propagation'
    arm['parameters']={'n':83,'m':[3],'ell':27,'seconds':1800,'max_rows':65536,'max_new_operations':262144,'affine_free_variables':12,'target_kinds':['planted','natural']}
    arm['description']=arm['id']+': complete V3 plus verified degree4 seeds; restricted propagation with certificate-only global upper gate'
cases=[]
names=['source.fieldpoly','X0.ordinary.fieldpoly','X0.normal.fieldpoly','X1.ordinary.fieldpoly','X1.normal.fieldpoly','U.ordinary.fieldpoly','U.normal.fieldpoly','candidates.rows.u64le','remainders.rows.u64le','candidates.jsonl','family.json','result.json','partial-083.json','partial-166.json','partial-249.json']
for old in p['extensions']['cases']:
    case=copy.deepcopy(old);key,arm,ident=case['key'],case['arm'],case['id'];directory=witness/'cases'/key
    result=read(directory/'result.json');assert result['input_binding']==old
    assert result['last_fall_lower_bound']==4 and result['strict_candidates']==result['candidates']==249
    status=read(witness/'status'/(key+'.json'));assert status['run_exit_code']==status['replay_exit_code']==0
    case['witness_case']=copy.deepcopy(old);case['witnessdirectory']=str(directory)
    case['witness_files']={name:ref(directory/name) for name in names}
    case['directory']=str(witness/'inputs'/arm/ident);case['v2directory']=str(witness/'v2'/arm/ident)
    assert sha(Path(case['directory'])/'input.json')==case['input_json_sha256']
    assert sha(Path(case['directory'])/'system.sing')==case['system_sha256']
    cases.append(case)
p['controls']['positive']['not_applicable_reason']='Native runtime xyz+1 must reach an exhaustive affine completion with exact LFD4; independently computed tiny V3/V4 spaces verify its strict lower4. Unit and capped-affine certificate paths have native tests.'
p['controls']['negative']['not_applicable_reason']='Native xyz fixture must finish this restricted procedure with no global upper certificate. Nineteen native tests cover caps, dependent seeds, ordinary-degree guards, parent/witness replay, altered rows/reducers/pending queues/upper claims and incomplete files.'
for factor in p['variable_ledger']['factors']:
    if factor['name']=='source':factor.update(value=source_refs['src/degree4_propagation_impl.rs']['sha256'],how='New native propagation implementation plus minimally adapted frozen predecessor copy returning verified parent rows')
    elif factor['name']=='host':factor['how']='Existing host after retained failed EKS storage probe; fresh inventory confirms no duplicate n83 execution and active jobs owned elsewhere; no timing claim'
    elif factor['name']=='limits':factor['how']='8GiB one native process;65536 total rows;262144 new operations;12 free variables for exhaustive affine check;1800s producer/replay plus5s grace;48000s outer;no automatic retry'
p['variable_ledger']['comparisons'][0]['threshold']='Only admitted1 or exhaustive affine completion gives upper4; inherited strict lower4 then gives exact4. Rank growth, caps and restricted queue exhaustion alone never do.'
p['sample_size']['power']['note']='Entire twelve-case retained provenance panel; exact deterministic computations, correlated/repeated inputs; no independence or statistical-power claim'
p['sample_size']['preflight_runtime'].update(fixed_seconds=0,per_sample_seconds=1800,estimated_total_seconds=48000,measured_on='Nineteen native control tests passed; these numbers are ceilings, not runtime predictions')
def metric(name,definition,unit='integer'):
    return {'id':name,'name':name.replace('_',' '),'unit':unit,'kind':'counted','definition':definition,'source':'summary.json rows '+('state.'+name if name in {'added_rows','independent_seed_rows','processed_operations'} else name),'aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}
p['metrics']={'primary':[metric('added_rows','state.added_rows: new independent admitted rows after complete V3; not a base-quality score')],
    'secondary':[metric('independent_seed_rows','state.independent_seed_rows: independent insertions among the saved coordinate seeds'),metric('processed_operations','state.processed_operations: all new attempted insertions/multiples including zero remainders'),metric('linear_rank','Rank of admitted degree-one constraints'),metric('last_fall_lower_bound','Inherited strict degree4 bound, reverified','polynomial degree'),metric('last_fall_upper_bound','4 only with global unit/affine certificate; absent when unknown','polynomial degree'),metric('last_fall_degree','4 only when inherited lower4 meets certified upper4; absent when unknown','polynomial degree')]}
p['measurement']['counters']=[m['id'] for m in p['metrics']['primary']+p['metrics']['secondary']]
p['execution']['timeout_seconds']=48000
p['stopping_rules'][0]['rule']='Attempt all12 retained cases; stop each at first unit, restricted queue exhaustion, total row cap or new-operation cap. Replay every successful producer in a fresh process. No automatic retry.'
p['stopping_rules'][1]['rule']='65536 total rows,262144 new operations,1800s producer/replay plus5s grace,8GiB one process,48000s outer; preserve every partial file and pending queue'
p['invalidation_rules']=['Any input/base/parent/witness hash mismatch invalidates case','All three immutable parents must replay exactly before seed admission','Every variable-multiplied row has degree<=2, so ordinary multiplication degree<=3; no Boolean shortcut for cubic products','Altered operations, dense rows, pending queues, affine assignments or verdict invalidate replay','Global upper requires unit or exhaustive affine completion; queue exhaustion/caps cannot imply upper4 or lower5','An exhaustive affine check is attempted only with at most12 free variables; larger spaces remain unknown','No regularity, speed, independence or elite-base conclusion from closure ranks']
p['analysis_plan'].update(script={'path':'src/degree4_propagation_impl.rs','sha256':source_refs['src/degree4_propagation_impl.rs']['sha256']},inputs=['protocol-degree4-propagation.json extensions.cases','cases/*/result.json','status/*.json'])
p['literature']=[{'ref':'https://arxiv.org/html/2112.05579','provenance':'retrieved','claim':'Definition1.5 and Theorems2.8--2.9 require global upper certification; unit/degree-compatible ideal presentation supplies a sufficient condition','verified_by':'Read ordinary-degree closure and strict-fall/GB criteria before method design'}, {'ref':'https://arxiv.org/html/1504.01175','provenance':'retrieved','claim':'Section4.5 known field-multiplier identity supplies inherited V4 seeds','verified_by':'Read field formula and ordinary/Boolean degree distinction'}, {'ref':'DEGREE4_WITNESS_DESIGN.md','provenance':'internal','claim':'Twelve unchanged strict-fall datasets with complete V3 and verified degree4 seeds','verified_by':sha(s/'DEGREE4_WITNESS_DESIGN.md')}]
p['reporting']['graphs']=['New independent rows and verified upper-certificate presence per retained case; caps/nulls visible; no base-quality ranking']
archives=copy.deepcopy(p['extensions']['parent_archives']);storage=read(s/'evidence/degree4-witness-storage.json')
archives.append({'role':'degree4_witness_seeds','sha256':storage['archive']['sha256'],'bytes':storage['archive']['bytes'],'object':storage['object'],'local_root':str(witness)})
p['extensions']={'cases':cases,'source_files':source_refs,'raw_prefix':'s3://setec-astronomy-artifacts/runs/n83-degree4-propagation-20261029-r1/local/','detailed_design':'DEGREE4_PROPAGATION_DESIGN.md','file_layout':'DEGREE4_PROPAGATION_FORMAT.json','parent_archives':archives,'limits':{'max_rows':65536,'max_new_operations':262144,'affine_free_variables':12},'backend_reason':'Fresh EKS inventory: no duplicate n83 execution; retained storage probe still failed with unconfirmed cause and other owners active. Existing-host fallback; no IAM/capacity change or timing claim.','retained_objective':'All n83 families,m2..9,FFD/LFD and elite factor bases remain open. This batch tests global upper certification on unchanged m3 binary systems.'}
out=s/'protocol-degree4-propagation.json';assert not out.exists(),'Never overwrite a frozen or previous protocol';out.write_text(json.dumps(p,indent=2)+'\n')
print(json.dumps({'source_commit':head,'binary_sha256':sha(binary),'cases':len(cases),'parent_archives':len(archives)}))
