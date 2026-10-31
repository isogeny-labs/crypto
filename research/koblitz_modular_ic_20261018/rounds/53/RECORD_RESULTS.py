"""Terminal receipts and historical byte custody; no mathematical computation."""
from pathlib import Path
from datetime import datetime
import hashlib,json,re,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
rows=[];pins=[]
for d in sorted((h/'runs').iterdir()):
 if not d.is_dir():continue
 rev=(d/'source-revision.txt').read_text().strip()
 row={'attempt':d.name,'source_revision':rev}
 if (d/'interrupted.json').exists():
  row.update(kind='Singular',status='rejected',native_exit=None,error_count=0,
             interruption=json.loads((d/'interrupted.json').read_text()),
             input_pin_status='invalid intermediate custody; retained files only',
             retained_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in d.iterdir() if p.is_file()})
  rows.append(row)
  continue
 if (d/'input.sing').exists():
  code=int((d/'exit.txt').read_text());log=(d/'native.log').read_text()
  markers=re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)',(d/'input.sing').read_text());assert len(markers)==1
  errors=[s for s in log.splitlines() if re.match(r'^\s*\?',s)]
  row.update(kind='Singular',native_exit=code,marker=markers[0],marker_present=markers[0] in log.splitlines(),error_count=len(errors),first_errors=errors[:8])
  row['status']='accepted' if code==0 and not errors and row['marker_present'] else 'rejected'
  row['started_utc']=(d/'started-utc.txt').read_text().strip();row['finished_utc']=(d/'finished-utc.txt').read_text().strip()
  row['native_wall_seconds']=(datetime.fromisoformat(row['finished_utc'])-datetime.fromisoformat(row['started_utc'])).total_seconds()
  sources=['input.sing','runner.sh']
 else:
  cc=int((d/'compile.exit').read_text());ac=int((d/'audit.exit').read_text()) if (d/'audit.exit').exists() else None
  log=(d/'audit.log').read_text() if (d/'audit.log').exists() else ''
  declarations=re.findall(r'^ok\s+(Transfer53\.\w+)',log,re.M)
  row.update(kind='Lean',compile_exit=cc,audit_exit=ac,declarations=declarations)
  row['status']='accepted' if cc==0 and ac==0 and len(declarations)==3 and '0 failing, 0 trusted' in log else 'rejected'
  sources=['Transfer53.lean','AuditAxioms.lean','runner.sh']
 if d.name=='discrim53-01':row['manual_stop']='SIGTERM for diagnosed invalid continuation; native handler exit0; strict errors/marker gate rejects'
 for name in sources:
  if name=='input.sing':
   candidates=[]
   for p in h.glob('*.sing'):
    path=str(p.relative_to(repo));out=subprocess.run(['git','show',rev+':'+path],cwd=repo,capture_output=True)
    if out.returncode==0 and out.stdout==(d/name).read_bytes():candidates.append(path)
   assert len(candidates)==1,(d.name,candidates);path=candidates[0]
  elif name=='runner.sh':path=str(h.relative_to(repo)/('REPLAY_CAS.sh' if row['kind']=='Singular' else 'REPLAY_LEAN.sh'))
  elif name=='AuditAxioms.lean':path='tools/lean/AuditAxioms.lean'
  else:path=str(h.relative_to(repo)/'lean'/name)
  old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo);actual=(d/name).read_bytes();assert old==actual,(d.name,name)
  pins.append({'attempt':d.name,'artifact':name,'revision':rev,'git_source':path,'sha256':hashlib.sha256(actual).hexdigest()})
 rows.append(row)
result={'schema':'koblitz.results/v1','round':53,'runs':rows,'input_pins':pins,
        'audited_unique_declarations':sorted(set(x for r in rows if r['status']=='accepted' for x in r.get('declarations',[])))}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'attempts':len(rows),'accepted':sum(r['status']=='accepted' for r in rows),'audited_unique_declarations':len(result['audited_unique_declarations']),'executed_source_pins':len(pins)}))
