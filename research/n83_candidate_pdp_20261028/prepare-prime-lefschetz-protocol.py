"""Full42-case draft; freeze and commit separately before production."""
from pathlib import Path
import hashlib,json,subprocess,sys
s=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
p=json.loads((s/'protocol-prime-linear.json').read_text())
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();binary=Path(sys.argv[1])
p.update(protocol_id='N83-PRIME-LEFSCHETZ-20261030',version=1,
 question='Do the source-bound Lefschetz/regular-sequence certificates and explicit terminal witness give conditional exact highest-form FFD17 for all42 retained higher-m prime cases?')
p['amendments']=[];p['links']={'design':'PRIME_LEFSCHETZ_ARGUMENT.md','controls':'LEFSCHETZ_CONTROLS.md','koszul_controls':'LEFSCHETZ_KOSZUL_CONTROLS.md'}
p['claims']=[{'id':'C1','class':'theorem','text':'Conditional exact highest-form FFD17 follows from the external block SLP theorem,the written chain/Koszul argument,verified source identities and explicit upper witness;independent review/formalization pending.','decided_by':[]},
 {'id':'C2','class':'out_of_scope','text':'No new LFD,solving degree,regularity,elite-base,security or speed claim. Arithmetic checks are not a formal proof.','decided_by':[]}]
for arm in p['arms']:
 arm['instrument'].update(commit=head,binary_sha256=sha(binary),build=[['cargo','build','--locked','--release','--manifest-path','research/n83_candidate_pdp_20261028/Cargo.toml','--bin','prime_source_audit']])
p['literature']=[{'ref':'https://arxiv.org/html/2112.05579','provenance':'retrieved','claim':'FFD is the highest-form nontrivial syzygy degree;LFD and regularity are distinct.','verified_by':'Definitions1.3–1.5 re-read before this design'},
 {'ref':'https://arxiv.org/html/1604.06820','provenance':'retrieved','claim':'For characteristic greater than block socle degree,the coordinate sum is strong Lefschetz. The chain proof is study-specific.','verified_by':'Theorem2.7 and Section3 re-read;PRIME_LEFSCHETZ_ARGUMENT.md'}]
p['variable_ledger']['factors']=[{'name':n,'treatment':t,'how':h} for n,t,h in [
 ('instrument','fixed','Frozen native source auditor plus Python structural/witness checker;one process at a time'),
 ('input','fixed','All42 exact original input,source and base-manifest hashes'),
 ('dataset','crossed','Three arms,two target kinds,seven m;shared structural argument,not42 independent theorem proofs'),
 ('host','measured','Existing host fallback after retained EKS read-only probe timed out;no clock claim'),
 ('limits','fixed','Four120second phases,2GiB address space each,720second outer,no retries'),
 ('analyst','fixed','Conditional exact17 only when all source,structural,witness and replay checks pass;failure invalidates affected result')]]
p['variable_ledger']['comparisons'][0]['threshold']='All42source identities plus structural hypotheses,explicit upper witness and fresh replay;no statistical inference'
p['controls']['positive']['not_applicable_reason']='Development controls outside production arms:18explicit block bases,40direct/sector maps with six Rust coefficient comparisons,12free-chain kernel/Koszul comparisons,18native test invocations and two arithmetic/witness unit tests.'
p['controls']['negative']['not_applicable_reason']='Bad characteristics,invalid leading weights,degree17 truncation boundary,changed Koszul sign,corrupted upper witness and incomplete/duplicate source keys are detected;all raw fixtures retained.'
p['sample_size']['preflight_runtime'].update(estimated_total_seconds=720,measured_on='Fixed resource cap,not a performance estimate')
p['sample_size']['power']['note']='All42existing sources;shared deterministic structure. No statistical theorem inference from controls.'
def metric(name,text):return {'id':name,'name':name,'unit':'degree','kind':'counted','definition':text,'source':'results/summary.json '+name,'aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}
p['metrics']={'primary':[metric('first_fall_degree','17 conditional on the written proof and all source/witness/replay checks;otherwise no valid result')],
 'secondary':[metric('first_fall_lower_bound','17 from external SLP theorem plus written no-truncation/Koszul argument'),metric('first_fall_upper_bound','17 from explicit source-bound nontrivial terminal witness')]}
p['measurement']['counters']=[x['id'] for a in p['metrics'].values() for x in a]
p['execution'].update(backend='local',queue='existing-spot-workspace',timeout_seconds=720)
p['execution']['placement'].update(cpus=1,memory='2Gi')
p['stopping_rules']=[{'kind':'fixed_n','preregistered':True,'rule':'All42cases,one producer and fresh replay for both native audit and Python certificate;no omitted arms or targets'},
 {'kind':'budget','preregistered':True,'rule':'Four120s phases,2GiB each,5s termination grace,720s outer,no automatic retries;upload source/inputs before compute and each phase output'}]
p['invalidation_rules']=['Any source,field,weight,structural,witness or replay mismatch invalidates affected results',
 'No inference from incomplete cases or failed phases','No empirical control establishes the universal proof by itself']
p['decision_rule'].update(text='Record conditional exactFFD17 only after all native/source/structural/upper checks and fresh replay;independent scientific review and formalization pending.',verdict_vocabulary=['conditional_certificate','invalid','error','incomplete'])
p['analysis_plan'].update(script={'path':'prime_lefschetz_certificate.py','sha256':sha(s/'prime_lefschetz_certificate.py')},
 inputs=['42unchanged input/source pairs','Native source audits','Source-matched prior upper summary','External SLP theorem and written argument'],
 outputs=['results/summary.json','results/structure.json','results/upper-witness.json','results/leading-mM.json','results/KEY.json','source-audit/summary.json'])
p['reporting']['graphs']=['Per-m source-matched lower/upper degree;all cases retained;proof boundary explicit']
cases=p['extensions']['cases']
for case in cases:
 directory=Path(case['directory']);assert sha(directory/'input.json')==case['input_sha256'];assert sha(directory/'system.sing')==case['source_sha256']
names=['src/bin/prime_source_audit.rs','src/prime_linear_source_donor.rs','src/prime_first_fall_donor.rs','src/prime_terminal_math.rs',
 'prime_lefschetz_certificate.py','lefschetz_controls.py','lefschetz_koszul_controls.py','test_lefschetz_certificate.py',
 'PRIME_LEFSCHETZ_ARGUMENT.md','PRIME_LEFSCHETZ_FORMAT.json','LEFSCHETZ_CONTROLS.md','LEFSCHETZ_KOSZUL_CONTROLS.md',
 'run-prime-lefschetz.sh','prepare-prime-lefschetz-protocol.py','evidence/prime-terminal/summary.json']
for name in names:assert subprocess.check_output(['git','show',head+':research/n83_candidate_pdp_20261028/'+name])==(s/name).read_bytes()
p['extensions']={'cases':cases,'file_layout':'PRIME_LEFSCHETZ_FORMAT.json','source_files':[{'path':n,'sha256':sha(s/n)} for n in names],
 'upper_summary_sha256':sha(s/'evidence/prime-terminal/summary.json'),'phase_timeout_seconds':120,'address_space_bytes':2147483648,
 'raw_prefix':'s3://setec-astronomy-artifacts/runs/n83-prime-lefschetz-20261030-r1/local/',
 'backend_reason':'Fresh read-only EKS probe n83-lefschetz-s3-probe-20261030-r1 UID2b6d0a19-d1c3-4e8f-902b-f12a54310506 terminal DeadlineExceeded;no retained Pod/log;no S3 access verdict. Retain established existing-host fallback without IAM/capacity changes.',
 'proof_boundary':'External SLP theorem,full written chain/Koszul proof and source-bound arithmetic;independent review/formalization pending'}
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
out=s/'protocol-prime-lefschetz.json';assert not out.exists();out.write_text(json.dumps(p,indent=2)+'\n')
