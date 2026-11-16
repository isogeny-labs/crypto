"""Terminal receipts and historical byte checks only; no algebra is computed."""
from pathlib import Path
from datetime import datetime
import hashlib,json,re,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];rows=[];pins=[]
authored={'Scaling61.'+s for s in ['balanced_first_bound','balanced_degree_bound','minor_pattern_count','cubic_patterns_818','coefficient_inverse_square','eigenfunctional_blocks_surjectivity']}
for d in sorted((h/'runs').iterdir()):
 if not d.is_dir():continue
 rev=(d/'source-revision.txt').read_text().strip();row={'attempt':d.name,'source_revision':rev}
 if (d/'input.rs').exists():
  bc=int((d/'build.exit').read_text());code=int((d/'exit.txt').read_text()) if (d/'exit.txt').exists() else None;log=(d/'native.log').read_text() if (d/'native.log').exists() else ''
  row.update(kind='Rust',build_exit=bc,native_exit=code,marker='RUSTCHECK61_ALL_CHECKS_PASS',marker_present='RUSTCHECK61_ALL_CHECKS_PASS' in log.splitlines(),native_results=[s for s in log.splitlines() if re.match(r'^[A-Z0-9_]+_RESULT ',s)])
  row['status']='accepted' if bc==0 and code==0 and row['marker_present'] else 'rejected';row['result_lines_accepted']=row['status']=='accepted';row['outcome']='completed' if row['status']=='accepted' else 'build_or_run_failure'
  if (d/'finished-utc.txt').exists():
   start=(d/'started-utc.txt').read_text().strip();finish=(d/'finished-utc.txt').read_text().strip();row.update(started_utc=start,finished_utc=finish,native_wall_seconds=(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds())
  mapping={'input.rs':'CHECK61.rs','driver.sh':'RUST_DRIVER.sh','launcher.sh':'REPLAY_RUST.sh','PROTOCOL.md':'PROTOCOL.md','protocol.json':'protocol.json','RUST-PROTOCOL.md':'RUST-PROTOCOL.md','donors.json':'donors.json'}
 elif (d/'input.sing').exists():
  source=(d/'input.sing').read_text();markers=re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)',source);assert len(markers)==1
  if (d/'interrupted.json').exists():
   assert not (d/'native.log').exists() and not (d/'exit.txt').exists()
   row.update(kind='Singular-launch',status='interrupted',outcome='interrupted_before_native_start',interruption=json.loads((d/'interrupted.json').read_text()))
  else:
   code=int((d/'exit.txt').read_text());log=(d/'native.log').read_text();errors=[s for s in log.splitlines() if re.match(r'^\s*\?',s)]
   row.update(kind='Singular',native_exit=code,marker=markers[0],marker_present=markers[0] in log.splitlines(),error_count=len(errors),first_errors=errors[:8],native_results=[s for s in log.splitlines() if re.match(r'^[A-Z0-9_]+_RESULT ',s)])
   row['status']='accepted' if code==0 and not errors and row['marker_present'] else 'rejected'
   row['result_lines_accepted']=row['status']=='accepted'
   row['outcome']='manual_resource_stop' if d.name in ('check61-01','check61-03') else 'time_cap' if code==124 else 'native_error' if errors else 'completed' if row['status']=='accepted' else 'missing_marker'
   start=(d/'started-utc.txt').read_text().strip();finish=(d/'finished-utc.txt').read_text().strip();row.update(started_utc=start,finished_utc=finish,native_wall_seconds=(datetime.fromisoformat(finish.replace('Z','+00:00'))-datetime.fromisoformat(start.replace('Z','+00:00'))).total_seconds())
  candidates=[]
  for p in h.glob('*.sing'):
   old=subprocess.run(['git','show',rev+':'+str(p.relative_to(repo))],cwd=repo,capture_output=True)
   if old.returncode==0 and old.stdout==(d/'input.sing').read_bytes():candidates.append(str(p.relative_to(h)))
  assert len(candidates)==1,(d.name,candidates)
  mapping={'input.sing':candidates[0],'driver.sh':'NATIVE_DRIVER.sh','launcher.sh':'REPLAY_CAS.sh','PROTOCOL.md':'PROTOCOL.md','protocol.json':'protocol.json','donors.json':'donors.json'}
 else:
  cc=int((d/'compile.exit').read_text());ac=int((d/'audit.exit').read_text()) if (d/'audit.exit').exists() else None;audit=(d/'audit.log').read_text() if (d/'audit.log').exists() else ''
  decls=re.findall(r'^ok\s+(Scaling61\.\S+)',audit,re.M)
  row.update(kind='Lean',compile_exit=cc,audit_exit=ac,audited_declarations=decls,authored_claims=sorted(authored.intersection(decls)))
  row['status']='accepted' if cc==0 and ac==0 and authored.issubset(decls) and '0 failing, 0 trusted' in audit else 'rejected'
  mapping={'driver.sh':'LEAN_DRIVER.sh','launcher.sh':'REPLAY_LEAN.sh','Scaling61.lean':'lean/Scaling61.lean','LEAN-PROTOCOL.md':'LEAN-PROTOCOL.md','AuditAxioms.lean':None}
 for local,source in mapping.items():
  path=str(h.relative_to(repo)/source) if source else 'tools/lean/AuditAxioms.lean';old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo);actual=(d/local).read_bytes();assert old==actual,(d.name,local)
  pins.append({'attempt':d.name,'artifact':local,'revision':rev,'git_source':path,'sha256':hashlib.sha256(actual).hexdigest()})
 row['artifacts']=[{'path':str(p.relative_to(h)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size} for p in sorted(d.iterdir()) if p.is_file()]
 rows.append(row)
accepted={r['attempt']:r for r in rows if r['status']=='accepted'}
required=['trace61-01','cubic61-02','field61-01','fix61-02','cohort61-01','rustcheck61-01','scaling-lean61-01']
assert all(n in accepted for n in required),(required,list(accepted))
result={'schema':'koblitz.results/v1','round':61,'runs':rows,'input_pins':pins,'authored_proved_claims':sorted(authored),'audited_declarations':accepted['scaling-lean61-01']['audited_declarations'],'protocol_metadata_disclosure':'PROTOCOL-METADATA61.md; historical versions immutable'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
protocols=[]
for row in rows:
 d=h/'runs'/row['attempt'];name='LEAN-PROTOCOL.md' if row['kind']=='Lean' else 'protocol.json';p=d/name;protocols.append({'attempt':d.name,'path':str(p.relative_to(h)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'frozen_revision':row['source_revision']})
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R61','claim_class':'exact_computation','protocols':protocols,'raw_archive':'runs.tar.xz; content address in .artifacts.json after custody publication','results':{'path':'results.json','sha256':hashlib.sha256((h/'results.json').read_bytes()).hexdigest()},'flow':{s:sum(r['status']==s for r in rows) for s in ['accepted','rejected','interrupted']},'canonical_schema_validation':'unavailable in selected Git index and disk; manual source-byte and exact contract checks only','protocol_metadata_disclosure':'PROTOCOL-METADATA61.md; v5 descriptive repair, v6+ before affected data','notes':'Native exact certificates and six abstract Lean claims have distinct scope; proper-source/RR/normalization premises retained. Global holomorphic forms, Cartier matrix and order11 action remain construction tasks. Injected negative controls are changed inverse/source row; malformed sparse-input guards were not separately fuzz-tested.'}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n');print(json.dumps({'workflows':len(rows),**export['flow'],'historical_byte_pins':len(pins),'authored_claims':len(authored)}))
