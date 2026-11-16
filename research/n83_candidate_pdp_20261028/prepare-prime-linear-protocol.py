"""Prepare a new draft; the protocol tool freezes it before production."""
from pathlib import Path
import hashlib,json,subprocess,sys
s=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
p=json.loads((s/'protocol-prime-terminal-r3.json').read_text())
commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()
binary=Path(sys.argv[1]);digest=sha(binary)
p.update(protocol_id='N83-PRIME-LINEAR-20261030',version=1,created='2026-10-30',question='Does complete degree-five injectivity of a five-selector template, together with covering projections, exclude degree-five nontrivial syzygies in all42 original prime m3-9 systems?')
p['amendments']=[];p['links']={'design':'PRIME_LINEAR_DESIGN.md'}
p['claims']=[{'id':'C1','class':'exact_computation','text':'Full column rank of projected degree-five maps and complete selector coverage certify original highest-form FFD>=6 under the written quotient argument. Deficient projections are inconclusive.','decided_by':[]},{'id':'C2','class':'out_of_scope','text':'No exact FFD, LFD, elite-base improvement or runtime comparison.','decided_by':[]}]
for arm in p['arms']:
 arm['instrument'].update(commit=commit,binary_sha256=digest,build=[['cargo','build','--locked','--release','--manifest-path','research/n83_candidate_pdp_20261028/Cargo.toml','--bin','prime_linear']])
p['variable_ledger']['factors']=[{'name':n,'treatment':t,'how':h} for n,t,h in [('instrument','fixed','One Rust binary, one thread, exact coefficients'),('input','fixed','All42 original source and point-base hashes, no new targets'),('dataset','crossed','Original/baseline/spike40 times planted/natural; seven shared templates, not42 independent samples'),('host','measured','Existing-host fallback after retained EKS shared-bucket403; no clock claim'),('limits','fixed','2GiB address space,300s producer,300s replay,720s outer,no retries'),('analyst','fixed','Full rank implies bound6 under coverage proof; deficient projection gives null, not original syzygy')]]
p['variable_ledger']['comparisons'][0]['threshold']='All source,field,weight inverse,coverage and exact coefficient-map rank checks pass; replay equality required'
p['controls']['positive']['not_applicable_reason']='No additional production arm: small-prime five-selector injection, independent dense ranks and six-term expansion are native unit controls;21 test invocations include16 duplicated donor tests.'
p['controls']['negative']['not_applicable_reason']='Three-selector annihilator and deficient map detected; invalid coverage and selector-square products checked; inherited source rejection and field controls.'
p['sample_size']['power']['note']='All42 retained input identities;seven shared deterministic template computations. No statistical inference.'
p['sample_size']['preflight_runtime'].update(estimated_total_seconds=720,measured_on='Development controls only; budget is not predicted runtime')
def metric(name,unit,text):return {'id':name,'name':name,'unit':unit,'kind':'counted','definition':text,'source':'results/summary.json '+name,'aggregation':'per_row','resolution':'1','uncertainty':'none_exact'}
p['metrics']={'primary':[metric('first_fall_lower_bound','degree','6 only after source-bound projection injectivity;otherwise null')],'secondary':[metric('projected_rank','rows','Exact rank of projected degree-five map'),metric('projected_columns','columns','Full linear multiplier domain dimension')]}
p['measurement']['counters']=[x['id'] for v in p['metrics'].values() for x in v]
p['measurement']['record']=[x for x in p['measurement']['record'] if x!='image_digest']
p['stopping_rules']=[{'kind':'fixed_n','preregistered':True,'rule':'All42 cases and seven shared templates once;retain deficiency,errors and partial outputs'},{'kind':'budget','preregistered':True,'rule':'2GiB,300s producer,300s fresh replay,5s grace,720s outer;zero retries'}]
p['invalidation_rules']=['Source,point-base,field,normalization,coverage or replay mismatch invalidates','Projection kernel is inconclusive for the original system','No LFD,exactFFD or timing claims']
p['decision_rule'].update(text='Record bound6 only for full column rank under complete covering projections;otherwise null.',verdict_vocabulary=['certified_lower','inconclusive','error','incomplete'])
p['analysis_plan'].update(script={'path':'src/bin/prime_linear.rs','sha256':sha(s/'src/bin/prime_linear.rs')},outputs=['results/summary.json','results/template-mM.json','results/KEY.json','results/primality.json'])
p['reporting']['graphs']=['Per-m lower-bound certificates and projected rank/column counts;nulls explicit']
p['literature']=[{'ref':'https://arxiv.org/html/2112.05579','provenance':'retrieved','claim':'Definition1.3 and Section4 distinguish highest-form nontrivial syzygies from LFD;covering argument is study-specific','verified_by':'Read before design;PRIME_LINEAR_DESIGN.md'}]
cases=p['extensions']['cases']
files=['src/bin/prime_linear.rs','src/prime_linear_source_donor.rs','src/prime_terminal_math.rs','src/prime_first_fall_donor.rs','PRIME_LINEAR_DESIGN.md','PRIME_LINEAR_FORMAT.json','run-prime-linear.sh','prepare-prime-linear-protocol.py']
for name in files:
 tracked='research/n83_candidate_pdp_20261028/'+name
 assert subprocess.check_output(['git','show',commit+':'+tracked])==(s/name).read_bytes()
p['extensions']={'cases':cases,'detailed_design':'PRIME_LINEAR_DESIGN.md','file_layout':'PRIME_LINEAR_FORMAT.json','source_files':[{'path':name,'sha256':sha(s/name)} for name in files],'raw_prefix':'s3://setec-astronomy-artifacts/runs/n83-prime-linear-20261030-r1/local/','raw_upload_seconds':30,'retry_policy':'none','backend_reason':'Fresh inventory has no duplicate panel. Prior prime-terminal R2 Job failed shared-bucket HeadObject403; retain established existing-host fallback without IAM/capacity changes.','prior_failed_job':'n83-prime-terminal-20261030-r2','proof_boundary':'Exact arithmetic plus written covering/regular-sequence argument; independent review and full formalization pending'}
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
out=s/'protocol-prime-linear.json';assert not out.exists();out.write_text(json.dumps(p,indent=2)+'\n')
