"""Terminal exact-algebra receipts and immutable Git byte pins; no mathematics."""
from pathlib import Path
from datetime import datetime
import hashlib,json,re,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];rows=[];pins=[]
authored={'Jets59.fourth_annihilator','Jets59.fourth_idempotent','Jets59.fourth_orthogonal','Jets59.fourth_nilpotent_model','Jets59.left_inverse_basechange'}
for d in sorted((h/'runs').iterdir()):
 if not d.is_dir():continue
 rev=(d/'source-revision.txt').read_text().strip();row={'attempt':d.name,'source_revision':rev}
 if (d/'input.sing').exists():
  code=int((d/'exit.txt').read_text());log=(d/'native.log').read_text();source=(d/'input.sing').read_text()
  markers=re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)',source);assert len(markers)==1
  errors=[s for s in log.splitlines() if re.match(r'^\s*\?',s)]
  row.update(kind='Singular',native_exit=code,marker=markers[0],marker_present=markers[0] in log.splitlines(),error_count=len(errors),first_errors=errors[:8],native_results=[s for s in log.splitlines() if re.match(r'^(DATA59_RESULT )',s)],point_results=[s for s in log.splitlines() if re.match(r'^UNUSED_POINT ',s)])
  row['status']='accepted' if code==0 and not errors and row['marker_present'] else 'rejected'
  row['outcome']='time_cap' if code==124 else 'native_error' if errors else 'completed' if row['status']=='accepted' else 'missing_marker'
  start=(d/'started-utc.txt').read_text().strip();finish=(d/'finished-utc.txt').read_text().strip()
  row.update(started_utc=start,finished_utc=finish,native_wall_seconds=(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds())
  candidates=[]
  for p in h.glob('*.sing'):
   rel=str(p.relative_to(repo));old=subprocess.run(['git','show',rev+':'+rel],cwd=repo,capture_output=True)
   if old.returncode==0 and old.stdout==(d/'input.sing').read_bytes():candidates.append(str(p.relative_to(h)))
  assert len(candidates)==1,(d.name,candidates)
  mapping={'input.sing':candidates[0],'driver.sh':'NATIVE_DRIVER.sh','launcher.sh':'REPLAY_CAS.sh','PROTOCOL.md':'PROTOCOL.md','protocol.json':'protocol.json','donors.json':'donors.json'}
 elif (d/'COHORT59.rs').exists():
  code=int((d/'exit.txt').read_text());log=(d/'native.log').read_text()
  checker=d.name.startswith('check');marker='CHECKCOHORT59_ALL_CHECKS_PASS' if checker else 'COHORT59_ALL_CHECKS_PASS'
  bc=int((d/'build.exit').read_text()) if (d/'build.exit').exists() else None
  row.update(kind='Rust certificate checker' if checker else 'Rust producer',native_exit=code,build_exit=bc,marker=marker,marker_present=marker in log.splitlines(),native_results=[x for x in log.splitlines() if x.startswith(('COHORT59_RESULT ','CHECKCOHORT59_RESULT '))])
  row['status']='accepted' if code==0 and (checker or bc==0) and row['marker_present'] else 'rejected'
  mapping={n:n for n in [('CHECK_DRIVER.sh' if checker else 'RUST_DRIVER.sh'),('REPLAY_CHECK.sh' if checker else 'REPLAY_RUST.sh'),'COHORT59.rs','PROTOCOL.md','protocol.json','RUST-PROTOCOL.md','donors.json']}
  for n in ['data59.txt','cohort59']+(['certificate59.txt'] if checker else []):
   row.setdefault('copied_inputs',{})[n]=hashlib.sha256((d/n).read_bytes()).hexdigest()
  start=(d/'started-utc.txt').read_text().strip();finish=(d/'finished-utc.txt').read_text().strip()
  row.update(started_utc=start,finished_utc=finish,native_wall_seconds=(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds())
 else:
  cc=int((d/'compile.exit').read_text());ac=int((d/'audit.exit').read_text()) if (d/'audit.exit').exists() else None
  audit=(d/'audit.log').read_text() if (d/'audit.log').exists() else ''
  decls=re.findall(r'^ok\s+(Jets59\.\S+)',audit,re.M)
  row.update(kind='Lean',compile_exit=cc,audit_exit=ac,audited_declarations=decls,authored_claims=sorted(authored.intersection(decls)),generated_equation_declarations=sorted(set(decls)-authored))
  row['status']='accepted' if cc==0 and ac==0 and authored.issubset(decls) and set(decls)==authored and '0 failing, 0 trusted' in audit else 'rejected'
  mapping={'driver.sh':'LEAN_DRIVER.sh','launcher.sh':'REPLAY_LEAN.sh','Jets59.lean':'lean/Jets59.lean','LEAN-PROTOCOL.md':'LEAN-PROTOCOL.md','AuditAxioms.lean':None}
 for local,source in mapping.items():
  path=str(h.relative_to(repo)/source) if source else 'tools/lean/AuditAxioms.lean'
  old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo);actual=(d/local).read_bytes();assert old==actual,(d.name,local)
  pins.append({'attempt':d.name,'artifact':local,'revision':rev,'git_source':path,'sha256':hashlib.sha256(actual).hexdigest()})
 rows.append(row)
accepted={r['attempt']:r for r in rows if r['status']=='accepted'}
assert all(n in accepted for n in ['data59-02','cohort59-01','checkcohort59-01','jets-lean59-01'])
result={'schema':'koblitz.results/v1','round':59,'runs':rows,'input_pins':pins,'authored_proved_claims':sorted(authored),'audited_declarations':sorted(accepted['jets-lean59-01']['audited_declarations'])}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
protocols=[]
for row in rows:
 d=h/'runs'/row['attempt'];name='LEAN-PROTOCOL.md' if row['kind']=='Lean' else 'protocol.json';p=d/name
 protocols.append({'attempt':d.name,'path':'runs/'+d.name+'/'+name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'frozen_revision':row['source_revision']})
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R59','claim_class':'exact_computation','protocols':protocols,'results':{'path':'results.json','sha256':hashlib.sha256((h/'results.json').read_bytes()).hexdigest()},'raw_archive':'runs.tar.xz;resolved by .artifacts.json after custody publication','canonical_schema_validation':'unavailable in both selected repository indexes;manual exact contract and all historical source bytes verified','claims':{'normal_quotient':'640-coordinate normal presentation, complete 320-dimensional cokernel with four 80-dimensional labelled Jordan blocks','cohort':'all eighteen prescribed stable finite-support divisor spaces have zero kernel, with explicit left inverses','checker':'row-dot-column replay of producer certificates; no producer rerun','basechange':'native left inverses persist over F32 by audited generic theorem','premises':'proper genus61 source, R55/R56 reduced support and local DVR interpretation; no projective-saturation claim'}}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n')
print(json.dumps({'workflows':len(rows),'accepted':len(accepted),'rejected':len(rows)-len(accepted),'valid_historical_pins':len(pins),'authored_claims':len(authored),'audited_declarations':len(result['audited_declarations'])}))
