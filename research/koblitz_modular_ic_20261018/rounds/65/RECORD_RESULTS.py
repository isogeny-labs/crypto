"""Record immutable R65 terminal receipts and historical source bytes only."""
from datetime import datetime
from pathlib import Path
import hashlib,json,re,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];rows=[];pins=[]
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
for raw in sorted((h/'runs').iterdir()):
 if not raw.is_dir():continue
 rev=(raw/'source-revision.txt').read_text().strip();code=int((raw/'exit.txt').read_text());log=(raw/'native.log').read_text()
 errors=[s for s in log.splitlines() if re.match(r'^\s*\?',s)]
 input_name='FIRST_FROBENIUS65.sing' if raw.name.startswith('first-frobenius') else 'LATTICE65.sing'
 marker='FIRST_FROBENIUS65_ALL_CHECKS_PASS' if raw.name.startswith('first-frobenius') else 'LATTICE65_ALL_CHECKS_PASS'
 accepted=code==0 and not errors and marker in log.splitlines()
 mapping={'input.sing':input_name,'driver.sh':'NATIVE_DRIVER.sh','launcher.sh':'REPLAY_CAS.sh','PROTOCOL.md':'PROTOCOL.md','protocol.json':'protocol.json','donors.json':'donors.json'}
 for local,source in mapping.items():
  path=str((h/source).relative_to(repo));old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo);actual=(raw/local).read_bytes();assert old==actual,(raw.name,local)
  pins.append({'attempt':raw.name,'artifact':local,'git_source':path,'revision':rev,'sha256':hashlib.sha256(actual).hexdigest()})
 start=(raw/'started-utc.txt').read_text().strip();finish=(raw/'finished-utc.txt').read_text().strip()
 rows.append({'attempt':raw.name,'source_revision':rev,'kind':'Singular','status':'accepted' if accepted else 'rejected','native_exit':code,'marker':marker,'marker_present':marker in log.splitlines(),'error_count':len(errors),'first_errors':errors[:8],'result_lines_accepted':accepted,'result_lines':[s for s in log.splitlines() if s.startswith(('FROB65_RESULT ','LATTICE65_RESULT '))],'started_utc':start,'finished_utc':finish,'native_wall_seconds':(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds(),'artifacts':[{'path':str(p.relative_to(h)),'sha256':digest(p),'bytes':p.stat().st_size} for p in sorted(raw.iterdir()) if p.is_file()]})
assert [r['attempt'] for r in rows]==['first-frobenius65-01','lattice65-01']
assert all(r['status']=='accepted' and len(r['result_lines'])==1 for r in rows)
def fields(line):return dict(part.split('=',1) for part in line.split()[1:])
f=fields(rows[0]['result_lines'][0]);g=fields(rows[1]['result_lines'][0])
assert {k:f[k] for k in ('order_rank','trace_dual_quotient','square_columns','binary_rows','square_rank','square_survivor_dimension','exact_source_discriminant','changed_kernel_rejected')}=={'order_rank':'32','trace_dual_quotient':'160','square_columns':'160','binary_rows':'241','square_rank':'80','square_survivor_dimension':'80','exact_source_discriminant':'1','changed_kernel_rejected':'1'}
assert {k:g[k] for k in ('old_trace_dual_length','survivor_dimension','new_trace_dual_length','basis_columns','module_inclusions','inverse_products','basis_square_checks','changed_inverse_rejected')}=={'old_trace_dual_length':'160','survivor_dimension':'80','new_trace_dual_length':'80','basis_columns':'32','module_inclusions':'2','inverse_products':'2','basis_square_checks':'32','changed_inverse_rejected':'1'}
assert all((h/'runs/first-frobenius65-01'/n).exists() for n in ['quotient-basis65.sing','square-kernel65.sing','survivor-functions65.sing'])
assert all((h/'runs/lattice65-01'/n).exists() for n in ['first-lattice-polynomial65.sing','first-lattice-field65.sing'])
result={'schema':'koblitz.results/v1','round':65,'runs':rows,'input_pins':pins,'protocol_metadata_disclosure':'Version1 complete quotient producer accepted; version2 freezes fresh lattice reconstruction from saved accepted output before affected data.'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
protocols=[]
for r in rows:
 p=h/'runs'/r['attempt']/'protocol.json';protocols.append({'attempt':r['attempt'],'path':str(p.relative_to(h)),'sha256':digest(p),'frozen_revision':r['source_revision']})
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R65-FIRST-FROBENIUS','claim_class':'exact_computation','protocols':protocols,'raw_archive':'runs.tar.xz; content address in .artifacts.json after custody publication','results':{'path':'results.json','sha256':digest(h/'results.json')},'flow':{status:sum(r['status']==status for r in rows) for status in ['accepted','rejected','interrupted']},'canonical_schema_validation':'unavailable in selected Git index and disk; manual source-byte and exact contract checks only','notes':'Complete first trace-dual Frobenius step: quotient160, square-map rank80/survivors80, first PID lattice rank32 and quotient80; further steps required for equality.'}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n')
print(json.dumps({'attempts':len(rows),**export['flow'],'historical_byte_pins':len(pins),'binary_rows':f['binary_rows'],'survivors':f['square_survivor_dimension']}))
