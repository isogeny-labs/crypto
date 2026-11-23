#!/usr/bin/env python3
"""Parse terminal native/proof receipts and hashes only; no field arithmetic."""
from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[];cases=[]
for p in sorted((h/'runs').iterdir()):
 if not p.is_dir():continue
 if p.name.startswith('lean'):
  ce=int((p/'compile.exit').read_text()); ae=int((p/'audit.exit').read_text())if(p/'audit.exit').exists()else None
  log=(p/'audit.log').read_text()if(p/'audit.log').exists()else ''
  accepted=ce==0 and ae==0 and '0 failing, 0 trusted' in log
  rows.append({'attempt':p.name,'kind':'Lean','compile_exit':ce,'audit_exit':ae,'status':'accepted'if accepted else'rejected','audited_declarations':int(re.search(r'audited (\d+) theorems',log).group(1)) if accepted else 0,'audit_log':log})
 else:
  be=int((p/'build.exit').read_text());rc=int((p/'exit.txt').read_text())if(p/'exit.txt').exists()else None
  log=(p/'native.log').read_text()if(p/'native.log').exists()else ''
  accepted=be==0 and rc==0 and 'COLLECT46_ALL_CHECKS_PASS' in log
  row={'attempt':p.name,'kind':'native','build_exit':be,'native_exit':rc,'status':'accepted'if accepted else'rejected','source_revision':(p/'source-revision.txt').read_text().strip(),'source_sha256':sha(p/'COLLECT46.rs')}
  if(p/'collect45').exists():row['binary_sha256']=sha(p/'collect45')
  rows.append(row)
  if accepted:
   result=next(x for x in log.splitlines()if x.startswith('COLLECT46_RESULT'))
   data=dict(re.findall(r'(\w+)=(\S+)',result));degree=int(data['conjugates'])
   pointfile=p/'points/closed-points.txt';lines=pointfile.read_text().splitlines()
   assert len(lines)==int(data['closed_points'])
   cases.append({'attempt':p.name,'status':'accepted','role':'control','extension_degree':degree,**{k:int(data[k])for k in ['field','base_points','chart_base_points','lambda_candidates','source_passes','rational_source_points','closed_points','deck_orbits','jacobian_rank2_points']},'points_sha256':sha(pointfile),'degree_partition':[dict((k,int(v))for k,v in re.findall(r'(\w+)=(\d+)',line))for line in log.splitlines()if line.startswith('COLLECT46_DEGREE')]})
comparisons=[]
for row in cases:
 d=row['extension_degree'];name='control7-01'if d==7 else f'collect{d}-01'
 reference=h/'../45/runs'/name/'points/closed-points.txt'; actual=h/'runs'/row['attempt']/'points/closed-points.txt'
 assert actual.read_bytes()==reference.read_bytes()
 comparisons.append({'degree':d,'candidate_attempt':row['attempt'],'reference_attempt':name,'byte_identical':True,'reference_sha256':sha(reference)})
(h/'results.json').write_text(json.dumps({'schema':'koblitz.results/v1','round':46,'runs':rows,'collector_cases':cases,'point_set_comparisons':comparisons,'proper_geometry_formalized':False,'Picard_classes_evaluated':False,'target_images_evaluated':False,'larger_n_transfer':False,'independent_scientific_replay':False,'independent_semantic_review':False},indent=2)+'\n')
print('Recorded',len(rows),'terminal attempts and',len(comparisons),'complete point-set equalities')
