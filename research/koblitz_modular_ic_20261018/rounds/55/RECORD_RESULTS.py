"""Terminal exact-algebra receipts and immutable Git byte pins; no mathematics."""
from pathlib import Path
from datetime import datetime
import hashlib,json,re,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];rows=[];pins=[]
authored={'Quotient55.evaluation_surjective','Quotient55.kernel_iff_image','Quotient55.range_eq_kernel','Quotient55.scalar_mem_image'}
for d in sorted((h/'runs').iterdir()):
 if not d.is_dir():continue
 rev=(d/'source-revision.txt').read_text().strip();row={'attempt':d.name,'source_revision':rev}
 if (d/'input.sing').exists():
  code=int((d/'exit.txt').read_text());log=(d/'native.log').read_text();source=(d/'input.sing').read_text()
  markers=re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)',source);assert len(markers)==1
  errors=[s for s in log.splitlines() if re.match(r'^\s*\?',s)]
  row.update(kind='Singular',native_exit=code,marker=markers[0],marker_present=markers[0] in log.splitlines(),error_count=len(errors),first_errors=errors[:8],native_results=[s for s in log.splitlines() if re.match(r'^(RESULT55 |CHECK55_RESULT |DIAG55_)',s)],point_results=[s for s in log.splitlines() if re.match(r'^UNUSED_POINT ',s)])
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
 else:
  cc=int((d/'compile.exit').read_text());ac=int((d/'audit.exit').read_text()) if (d/'audit.exit').exists() else None
  audit=(d/'audit.log').read_text() if (d/'audit.log').exists() else ''
  decls=re.findall(r'^ok\s+(Quotient55\.\S+)',audit,re.M)
  row.update(kind='Lean',compile_exit=cc,audit_exit=ac,audited_declarations=decls,authored_claims=sorted(authored.intersection(decls)),generated_equation_declarations=sorted(set(decls)-authored))
  row['status']='accepted' if cc==0 and ac==0 and authored.issubset(decls) and set(decls)==authored and '0 failing, 0 trusted' in audit else 'rejected'
  mapping={'driver.sh':'LEAN_DRIVER.sh','launcher.sh':'REPLAY_LEAN.sh','Quotient55.lean':'lean/Quotient55.lean','LEAN-PROTOCOL.md':'LEAN-PROTOCOL.md','AuditAxioms.lean':None}
 for local,source in mapping.items():
  path=str(h.relative_to(repo)/source) if source else 'tools/lean/AuditAxioms.lean'
  old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo);actual=(d/local).read_bytes();assert old==actual,(d.name,local)
  pins.append({'attempt':d.name,'artifact':local,'revision':rev,'git_source':path,'sha256':hashlib.sha256(actual).hexdigest()})
 rows.append(row)
accepted={r['attempt']:r for r in rows if r['status']=='accepted'}
assert all(n in accepted for n in ['diag55-01','quotient55-02','quotient-lean55-01','check55-02'])
result={'schema':'koblitz.results/v1','round':55,'runs':rows,'input_pins':pins,'authored_proved_claims':sorted(authored),'audited_declarations':sorted(accepted['quotient-lean55-01']['audited_declarations'])}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
protocols=[]
for row in rows:
 d=h/'runs'/row['attempt'];name='protocol.json' if row['kind']=='Singular' else 'LEAN-PROTOCOL.md';p=d/name
 protocols.append({'attempt':d.name,'path':'runs/'+d.name+'/'+name,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'frozen_revision':row['source_revision']})
export={'schema':'experiment.export/v1','experiment_id':'KOBLITZ-R55','claim_class':'exact_computation','protocols':protocols,'results':{'path':'results.json','sha256':hashlib.sha256((h/'results.json').read_bytes()).hexdigest()},'raw_archive':'runs.tar.xz;resolved by .artifacts.json after custody publication','canonical_schema_validation':'unavailable in both selected repository indexes;manual exact contract and all historical source bytes verified','claims':{'source_annihilator':'both full polynomial products','complete_quotient':'dimension80;all kernel/image/recovery products','CRT_evaluator':'all32sourcevalues/528products/128pointbindings','global_F2_modular_factor_transfer':'open;not replaced by affine quotient'}}
(h/'export.json').write_text(json.dumps(export,indent=2)+'\n')
print(json.dumps({'workflows':len(rows),'accepted':len(accepted),'rejected':len(rows)-len(accepted),'valid_historical_pins':len(pins),'authored_claims':4,'audited_declarations':len(result['audited_declarations'])}))
