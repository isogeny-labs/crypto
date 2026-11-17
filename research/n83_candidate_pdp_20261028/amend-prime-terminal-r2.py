"""Retain failed R1; bind identical mathematical panel to repaired worker/image R2."""
from pathlib import Path
import json,hashlib,subprocess
s=Path('research/n83_candidate_pdp_20261028');r=Path('/home/ec2-user/n83-candidate-recovery');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
p=json.loads((s/'protocol-prime-terminal.json').read_text());old=p.pop('freeze');assert old['status']=='frozen'
image=json.loads((r/'prime-terminal-support/image-r2.json').read_text());commit=image['source_revision'];assert image['binary_sha256']==p['arms'][0]['instrument']['binary_sha256']
for name in ['src/bin/prime_terminal.rs','src/prime_terminal_math.rs','src/prime_first_fall_donor.rs','run-prime-terminal.py']:
 assert subprocess.check_output(['git','show',commit+':'+str(s/name)])==(s/name).read_bytes()
p['version']=2
for a in p['arms']:a['instrument']['commit']=commit
p['extensions'].update(image=image,job_name='n83-prime-terminal-20261030-r2',raw_prefix='s3://setec-astronomy-artifacts/runs/n83-prime-terminal-20261030-r2/eks/',prior_failed_job='n83-prime-terminal-20261030-r1',prior_protocol_sha256=old['protocol_sha256'])
for f in p['extensions']['source_files']:f['sha256']=sha(s/f['path'])
p['extensions']['source_files'].append({'path':'amend-prime-terminal-r2.py','sha256':sha(s/'amend-prime-terminal-r2.py')})
p['amendments']=[{'id':'A-worker-r2','filed':'2026-10-30','filed_before_affected_data':True,'reason':'R1 terminal Job exit2 before any native computation: UID1000 could not read root-owned mode600 worker. Submission also revealed legacy default results bucket. All R1 protocol/source/image/Job/logs retained.','changes':['Image COPY explicitly grants readable worker/build-info;container control opens and compiles worker underUID1000','Worker requires explicit permitted-bucket output URI;never uses legacy default for writes','New R2 Job and outputs;identical native binary,42inputs and mathematical method;zero automatic retries'],'supersedes_fields':['arms[].instrument.commit','extensions.image','extensions.job_name','extensions.raw_prefix','extensions.source_files']}]
p['freeze']={'status':'draft','protocol_sha256':None,'commit':None,'frozen_at':None,'filed_before_affected_data':None}
out=s/'protocol-prime-terminal-r2.json';assert not out.exists();out.write_text(json.dumps(p,indent=2)+'\n')
