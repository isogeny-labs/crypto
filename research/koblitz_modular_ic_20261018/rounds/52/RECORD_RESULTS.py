"""Read terminal receipts and literal native counters; no mathematical execution."""
from pathlib import Path
from collections import Counter
import json,re,subprocess,hashlib
h=Path(__file__).resolve().parent;repo=h.parents[3]
rows=[]
for d in sorted((h/'runs').iterdir()):
 if not d.is_dir():continue
 row={'attempt':d.name,'source_revision':(d/'source-revision.txt').read_text().strip()}
 if (d/'input.sing').exists():
  assert (d/'exit.txt').exists(),f'nonterminal {d}'
  log=(d/'native.log').read_text();code=int((d/'exit.txt').read_text());source=(d/'input.sing').read_text()
  markers=re.findall(r'print\("([A-Z0-9_]+_ALL_CHECKS_PASS)"\)',source);assert len(markers)==1
  errors=[x for x in log.splitlines() if re.match(r'^ *\?',x)]
  row.update(kind='Singular',native_exit=code,marker=markers[0],marker_present=markers[0] in log.splitlines(),error_count=len(errors),first_errors=errors[:8])
  row['status']='accepted' if code==0 and not errors and row['marker_present'] else 'rejected'
  for prefix in ['CLOSURE52_RESULT','ANNIHILATOR52_RESULT','FILTRATION52_RESULT','SEMIGROUP52_RESULT','FIBERS23_52_RESULT','LARGEFIBERS23_52_RESULT','FRESH52_RESULT','NORMLINE52_RESULT']:
   matches=[x for x in log.splitlines() if x.startswith(prefix+' ')];
   if matches:row['native_result']=matches[-1]
  if d.name=='fibers23-52-01':row['manual_stop']='SIGTERM sent to verified native PID for diagnosed nonadvancing continue loop; handler exit0,required marker absent'
 else:
  assert (d/'compile.exit').exists(),f'nonterminal {d}'
  compile_exit=int((d/'compile.exit').read_text());audit_exit=int((d/'audit.exit').read_text()) if (d/'audit.exit').exists() else None
  audit=(d/'audit.log').read_text() if (d/'audit.log').exists() else ''
  declarations=re.findall(r'^ok\s+((?:Closure52|FibreIrred52|RectBridge52)\.\w+)',audit,re.M)
  row.update(kind='Lean',compile_exit=compile_exit,audit_exit=audit_exit,declarations=declarations)
  row['status']='accepted' if compile_exit==0 and audit_exit==0 and len(declarations)==(2 if (d/'FibreIrred52.lean').exists() else 1 if (d/'RectBridge52.lean').exists() else 6) and '0 failing, 0 trusted' in audit else 'rejected'
 rows.append(row)
# Bind every actually executed input and runner to its historical Git source.
pins=[]
for row in rows:
 d=h/'runs'/row['attempt'];rev=row['source_revision']
 is_fibre=(d/'FibreIrred52.lean').exists();is_rect=(d/'RectBridge52.lean').exists()
 actual_sources=(['FibreIrred52.lean','Closure52.lean','AuditAxioms.lean','runner.sh'] if is_fibre else ['RectBridge52.lean','AuditAxioms.lean','runner.sh'] if is_rect else ['Closure52.lean','AuditAxioms.lean','runner.sh']) if row['kind']=='Lean' else ['input.sing','runner.sh']
 for name in actual_sources:
  if name=='runner.sh':relative=('REPLAY_FIBRE_LEAN.sh' if is_fibre else 'REPLAY_RECT_LEAN.sh' if is_rect else 'REPLAY_LEAN.sh') if row['kind']=='Lean' else 'REPLAY_CAS.sh'
  elif name.endswith('.lean'):relative='lean/'+name
  else:
   runner=(d/'input.sing').read_bytes()
   candidates=[]
   for p in h.glob('*.sing'):
    path=str(p.relative_to(repo))
    out=subprocess.run(['git','show',rev+':'+path],cwd=repo,capture_output=True)
    if out.returncode==0 and out.stdout==runner:candidates.append(path)
   assert len(candidates)==1,(row['attempt'],candidates)
   relative=str(Path(candidates[0]).relative_to(h.relative_to(repo)))
  path='tools/lean/AuditAxioms.lean' if name=='AuditAxioms.lean' else str(h.relative_to(repo)/relative)
  old=subprocess.check_output(['git','show',rev+':'+path],cwd=repo)
  actual=(d/name).read_bytes();assert old==actual,(row['attempt'],name,rev,path)
  pins.append({'attempt':row['attempt'],'artifact':name,'git_source':path,'revision':rev,'sha256':hashlib.sha256(actual).hexdigest()})
counts={'audited_unique_theorems':len(set(x for row in rows if row['status']=='accepted' for x in row.get('declarations',[])))}
accepted={x['attempt']:x for x in rows if x['status']=='accepted'}
required=['closure52-02','annihilator52-01','reload52-02','normline52-01','filtration52-02','closure-lean52-02','fresh52-01']
assert all(x in accepted for x in required)
def numeric_fields(attempt):return {k:int(v) for k,v in re.findall(r'(\w+)=(\d+)',accepted[attempt]['native_result'])}
closure=numeric_fields('closure52-02');ann=numeric_fields('annihilator52-01');filtration=numeric_fields('filtration52-02');fresh=numeric_fields('fresh52-01')
counts.update(normal_chart_rank=closure['rank'],source_index_length=closure['initial_length'],nu_adjoined_index_length=closure['final_length'],nu_closure_extensions=closure['extensions'],least_zero_exponent=ann['zero_exponent'],least_one_exponent=ann['one_exponent'],local_filtration_levels=filtration['local_filtration_levels'])
for attempt,stem in [('fibers23-52-02','small'),('largefibers23-52-01','large')]:
 if attempt in accepted:
  line=accepted[attempt]['native_result']
  for key,value in re.findall(r'(\w+)=(\d+)',line):counts[stem+'_'+key]=int(value)
if 'fresh52-01' in accepted:counts.update(fresh_verified_packet_fibers=fresh['complete_specializations'],generic_norm_line_maps=1)
if 'semigroup52-01' in accepted:counts.update(candidate_semigroup_gaps=numeric_fields('semigroup52-01')['gaps'],candidate_semigroup_conductor=numeric_fields('semigroup52-01')['conductor'])
result={'schema':'koblitz.results/v1','round':52,'date':'2026-10-31','status_counts':dict(Counter(x['status'] for x in rows)),'construction_counts':counts,'runs':rows,'interpretation':'Exact native certificates; normalization,PID classification,DVR specialization and source-field identification premises are recorded separately in PROOF_MAP.md.'}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
(h/'receipt-validation.json').write_text(json.dumps({'all_attempts_terminal':True,'strict_native_gate':True,'historical_input_runner_pins':len(pins),'pins':pins},indent=2)+'\n')
print(json.dumps({'attempts':len(rows),'status_counts':result['status_counts'],'historical_input_runner_pins':len(pins),'construction_counts':counts}))
