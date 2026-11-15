"""Preserve EKS failures; freeze unchanged native panel on established host fallback."""
from pathlib import Path
import json,hashlib,subprocess
s=Path('research/n83_candidate_pdp_20261028');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();p=json.loads((s/'protocol-prime-terminal-r2.json').read_text());old=p.pop('freeze');assert old['status']=='frozen'
commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();p['version']=3
for a in p['arms']:a['instrument']['commit']=commit
p['execution'].update(backend='local',queue='existing-spot-workspace',timeout_seconds=720);p['execution']['placement'].update(cpus=1,memory='2Gi')
p['sample_size']['preflight_runtime']['estimated_total_seconds']=720
for factor in p['variable_ledger']['factors']:
 if factor['name']=='host':factor['how']='Established existing-host fallback after terminal EKS R2 source HeadObject403;no IAM/capacity changes'
 if factor['name']=='limits':factor['how']='60s controls,300sproducer,300sfreshreplay,+5skillgrace,2GiBaddressspace,720souter,no retries'
 if factor['name']=='instrument':factor['how']='Unchanged pinned native Rust binary and source;one thread;EKS images retained as failed-attempt evidence'
p['stopping_rules'][1]['rule']='60scontrols,300sproducer,300sreplay,+5skillgrace,2GiBaddressspace,720souter;no retries'
p['extensions']['prior_image']=p['extensions'].pop('image');p['extensions'].pop('job_name',None)
p['extensions'].update(raw_prefix='s3://setec-astronomy-artifacts/runs/n83-prime-terminal-20261030-r3/local/',prior_failed_jobs=['n83-prime-terminal-20261030-r1','n83-prime-terminal-20261030-r2'],prior_protocol_sha256=old['protocol_sha256'],backend_reason='R1 image worker permission failure and R2 shared-bucket HeadObject403 both terminal before native production. Preserve both;use established existing-host fallback with samebinary/42inputs/math and bounded contract.')
for f in p['extensions']['source_files']:f['sha256']=sha(s/f['path'])
for n in ['run-prime-terminal-r3.sh','amend-prime-terminal-r3.py']:p['extensions']['source_files'].append({'path':n,'sha256':sha(s/n)})
p['amendments'].append({'id':'A-host-r3','filed':'2026-10-30','filed_before_affected_data':True,'reason':'R2 EKS Job terminated exit1 on source HeadObject403 before native computation;no access escalation. Preserve allfailedattempts and use established existing-host fallback.','changes':['Unchanged binary,42inputs,exact algebra and controls','Local launcher with original budgets,sourceidentity checks,periodicS3 andterminalupload','NewR3outputprefix;bothEKSprotocols/images/sourcebundles/receipts retained'],'supersedes_fields':['arms[].instrument.commit','execution','extensions.raw_prefix','extensions.source_files']})
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None};out=s/'protocol-prime-terminal-r3.json';assert not out.exists();out.write_text(json.dumps(p,indent=2)+'\n')
