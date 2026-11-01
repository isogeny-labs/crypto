#!/usr/bin/env python3
"""Generate and validate matched PDP inputs for the three new fixed bases."""
import hashlib,json,os,subprocess,time
from pathlib import Path
study=Path(__file__).resolve().parents[3]; attempt=Path(__file__).resolve().parent
protocol=study/'protocol-model-full-domain-v5.json'; protocol_sha='3a041db64c7d6d70aa99200fe46500770998de5faffaf2acb7a2e366c855eb57'
binary=study/'target/release/pdp'; binary_sha='2ebd29a31146755f8a79b454b4d6d1673b2ed979ffedfc807218d2d8605b37d4'
assert hashlib.sha256(protocol.read_bytes()).hexdigest()==protocol_sha
assert hashlib.sha256((study/'src/bin/pdp.rs').read_bytes()).hexdigest()=='320e7b462486e7637edf6027bc568e30641876860c450503bd4b443352cf2353'
assert hashlib.sha256(binary.read_bytes()).hexdigest()==binary_sha
matched=study/'evidence/pdp/baseline'; outroot=attempt/'inputs'; assert not outroot.exists(); outroot.mkdir()
records=[]; env=dict(os.environ,AWS_DEFAULT_REGION='us-east-2')
for bit in (35,36,37):
 candidate=f'spike{bit}'; bases=Path('/tmp/n83-full-domain-bases-attempt07')/candidate; out=outroot/candidate; log=attempt/f'generate-{candidate}.log'; started=time.time()
 proc=subprocess.run([str(binary),'generate',str(out),str(bases),str(matched),candidate],cwd=study.parent.parent,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
 log.write_bytes(proc.stdout); rec={'candidate_id':candidate,'coordinate_bit':bit,'exit_code':proc.returncode,'elapsed_seconds_diagnostic_only':time.time()-started,'log_sha256':hashlib.sha256(log.read_bytes()).hexdigest()}
 if proc.returncode==0:
  rows=json.loads((out/'inputs.json').read_text()); assert len(rows)==48 and all(x['candidate_id']==candidate for x in rows)
  assert all(len(x['factor_base_membership'])==x['m'] and all(p['present'] for p in x['factor_base_membership']) for x in rows)
  assert all(len(x['selected_base_selectors'])==x['m'] for x in rows if x['target_kind']=='planted')
  saved=attempt/'base-sources'/candidate; dest=out/'base-source'; dest.mkdir()
  for name in ['index.json','s3-file-index.json','source-manifest.json','koblitz-manifest.json','binary-manifest.json','prime-manifest.json']:(dest/name).write_bytes((saved/name).read_bytes())
  rec.update({'input_rows':len(rows),'input_index_sha256':hashlib.sha256((out/'inputs.json').read_bytes()).hexdigest(),'all_membership_checks_passed':True})
 else: rec['failure']='generator failed; partial outputs retained; no overwrite/retry'
 (attempt/f'status-{candidate}.json').write_text(json.dumps(rec,indent=2)+'\n'); records.append(rec); print(json.dumps(rec),flush=True)
 if proc.returncode: break
summary={'schema':'n83.full-domain-new-base-input-generation/v1','protocol_sha256':protocol_sha,'pdp_binary_sha256':binary_sha,'planned_systems':144,'completed_bases':sum(x.get('input_rows')==48 for x in records),'completed_systems':48*sum(x.get('input_rows')==48 for x in records),'all_inputs_verified':len(records)==3 and all(x.get('all_membership_checks_passed') for x in records),'records':records}
(attempt/'generation-summary.json').write_text(json.dumps(summary,indent=2)+'\n'); print(json.dumps({k:v for k,v in summary.items() if k!='records'},indent=2))
if not summary['all_inputs_verified']:raise SystemExit(1)
