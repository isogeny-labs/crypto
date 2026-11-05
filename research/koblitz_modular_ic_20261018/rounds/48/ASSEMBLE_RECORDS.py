from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent;rows=[];logs={};failed=json.loads((h/'failed-launches.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
markers={'signed48-shift1-01':'SIGNED48','signed48-shift2-01':'SIGNED48','cofactor48-shift1-01':'COFACTOR48','cofactor48-shift2-01':'COFACTOR48','split48-01':'SPLIT48','compare48-01':'COMPARE48'}
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 if p.name in failed:
  f=failed[p.name];rows.append({'attempt':p.name,'kind':'Lean','stage':'precompiler_launch','status':'rejected','wrapper_exit':f['wrapper_exit'],'compile_exit':None,'audit_exit':None,'compiler_started':False,'source_revision':f['source_revision'],'audited_declarations':0});continue
 row={'attempt':p.name,'source_revision':(p/'source-revision.txt').read_text().strip()}
 if 'lean' in p.name:
  c=int((p/'compile.exit').read_text());a=int((p/'audit.exit').read_text())if(p/'audit.exit').exists()else None;log=(p/'audit.log').read_text()if(p/'audit.log').exists()else '';ok=c==0 and a==0 and '0 failing, 0 trusted' in log
  row.update(kind='Lean',compile_exit=c,audit_exit=a,status='accepted'if ok else'rejected',audited_declarations=int(re.search(r'audited (\d+) theorems',log).group(1))if ok else 0,audit_log=log)
 else:
  c=int((p/'build.exit').read_text());a=int((p/'exit.txt').read_text())if(p/'exit.txt').exists()else None;log=(p/'native.log').read_text()if(p/'native.log').exists()else '';logs[p.name]=log;ok=c==0 and a==0 and markers[p.name]+'_ALL_CHECKS_PASS'in log
  bins=[q for q in p.iterdir()if q.name in ['signed48','cofactor48','split48','compare48']];row.update(kind='native',build_exit=c,native_exit=a,status='accepted'if ok else'rejected',full_generator_quotients=len(re.findall(r'^QUOT36_QUOTIENT ',log,re.M)),binary_sha256=sha(bins[0])if bins else None)
 rows.append(row)
def values(line):
 out={}
 for k,v in re.findall(r'(\w+)=(\S+)',line):out[k]=True if v=='true'else False if v=='false'else int(v)if v.isdigit()else v
 return out
def result(name,prefix):return values(next(l for l in logs[name].splitlines()if l.startswith(prefix+'_RESULT ')))
split=result('split48-01','SPLIT48');comparison=result('compare48-01','COMPARE48')if'compare48-01'in logs else None
comparisons=[]
if comparison:
 for l in logs['compare48-01'].splitlines():
  if l.startswith('SIGNED37_EQUALITY label=new7-old7-multiple-')or l.startswith('SIGNED37_EQUALITY label=new8-old8-multiple-'):comparisons.append(values(l))
 assert len(comparisons)==22
 for component in [7,8]:
  matches=[x for x in comparisons if x['label'].startswith(f'new{component}-')and x['equivalent']];assert len(matches)<=1
  target=comparison[f'component{component}_old_coordinate'];assert target==(int(matches[0]['label'].rsplit('-',1)[1])if matches else'none')
matched=sum(comparison[f'component{k}_old_coordinate']!='none'for k in [7,8])if comparison else None
r={'schema':'koblitz.results/v1','round':48,'runs':rows,'shifted_signed_classes':[result(f'signed48-shift{i}-01','SIGNED48')for i in [1,2]],'shifted_cofactors':[result(f'cofactor48-shift{i}-01','COFACTOR48')for i in [1,2]],'split':split,'comparison':comparison,'old_line_comparisons':comparisons,'failed_launches':failed,'construction_counts':{'character_components_tested':2 if split['gate']else 0,'old_line_scalar_comparisons':len(comparisons),'matched_old_coordinates':matched,'crossed_source_operator_images':2 if comparison else None},'new_class_in_old_tested_component_span':comparison['coordinates_recover']if comparison else None,'chosen_Koblitz_model_map_evaluated':False,'native_arrays_formalized':False,'proper_geometry_formalized':False,'full_integral_Hom_lattice_recovered':False,'IC_relation_rank':None,'larger_n_transfer':False,'performance_measured':False,'independent_scientific_replay':False,'independent_semantic_review':False}
(h/'results.json').write_text(json.dumps(r,indent=2)+'\n');print('Recorded',len(rows),'attempts;',len(comparisons),'complete line comparisons; matched coordinates:',matched)
