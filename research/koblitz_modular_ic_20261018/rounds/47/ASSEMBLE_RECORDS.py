from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent;rows=[];logs={}
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 row={'attempt':p.name,'source_revision':(p/'source-revision.txt').read_text().strip()}
 if 'lean' in p.name:
  c=int((p/'compile.exit').read_text());a=int((p/'audit.exit').read_text())if(p/'audit.exit').exists()else None
  log=(p/'audit.log').read_text()if(p/'audit.log').exists()else ''
  ok=c==0 and a==0 and '0 failing, 0 trusted' in log
  row.update(kind='Lean',compile_exit=c,audit_exit=a,status='accepted'if ok else'rejected',audited_declarations=int(re.search(r'audited (\d+) theorems',log).group(1))if ok else 0,audit_log=log)
 else:
  c=int((p/'build.exit').read_text());a=int((p/'exit.txt').read_text())if(p/'exit.txt').exists()else None
  log=(p/'native.log').read_text()if(p/'native.log').exists()else '';logs[p.name]=log
  prefix={'jets47-01':'JET47','couple47-01':'COUPLE47','signed47-shift0-01':'SIGNED47','cofactor47-shift0-01':'COFACTOR47'}[p.name]
  ok=c==0 and a==0 and prefix+'_ALL_CHECKS_PASS' in log
  binaries=[q for q in p.iterdir()if q.name in ['jet47','couple47','signed47','cofactor47']]
  row.update(kind='native',build_exit=c,native_exit=a,status='accepted'if ok else'rejected',full_generator_quotients=len(re.findall(r'^QUOT36_QUOTIENT ',log,re.M)),binary_sha256=sha(binaries[0])if binaries else None)
 rows.append(row)
def values(line):
 d={}
 for k,v in re.findall(r'(\w+)=(\S+)',line):
  d[k]=True if v=='true'else False if v=='false'else int(v)if v.isdigit()else v
 return d
jets=[values(l)for l in logs['jets47-01'].splitlines()if l.startswith('JET47_POINT ')]
coupled=[values(l)for l in logs['couple47-01'].splitlines()if l.startswith('COUPLE47_POINT ')]
signed=values(next(l for l in logs['signed47-shift0-01'].splitlines()if l.startswith('SIGNED47_RESULT ')))
cofactor=values(next(l for l in logs['cofactor47-shift0-01'].splitlines()if l.startswith('COFACTOR47_RESULT ')))
assert len(jets)==len(coupled)==10
for i in range(10):
 for label,n in [('small',68),('large',196)]:
  f=h/f'runs/couple47-01/divisors/{label}-o1-d{i}-basis.txt';lines=f.read_text().splitlines();assert len(lines)==n
  for k,l in enumerate(lines):parts=l.split();assert len(parts)==155 and int(parts[0])==k+1
assert signed['independent_expression_equal'] and cofactor['independent_cofactor_expressions_equal']
order=11 if not cofactor['cofactor_class_principal'] and cofactor['eleven_cofactor_principal'] else None
r={'schema':'koblitz.results/v1','round':47,'runs':rows,'jet_cases':jets,'complete_divisor_cases':coupled,'signed_class':signed,'cofactor_class':cofactor,'source_order_under_retained_Picard_bridge':order,'construction_counts':{'full_jet_cases':len(jets),'complete_divisor_representations':len(coupled),'signed_class_cases':1},'chosen_target_image_evaluated':False,'diamond_character_evaluated':False,'proper_geometry_formalized':False,'native_arrays_formalized':False,'IC_relation_rank':None,'larger_n_transfer':False,'performance_measured':False,'independent_scientific_replay':False,'independent_semantic_review':False}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n');print('Recorded',len(rows),'terminal attempts; source order under retained Picard bridge:',order)
