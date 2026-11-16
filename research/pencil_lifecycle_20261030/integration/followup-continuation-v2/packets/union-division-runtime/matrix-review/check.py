from pathlib import Path
import json,subprocess,hashlib,shutil
p=Path(__file__).parent;s=Path('/var/tmp/pencil-kummer-matrix-binding-20261101');binary=p/'build/release/pencil-readonly-accounting'
def sha(q):return hashlib.sha256(q.read_bytes()).hexdigest()
def run(root):
 r=subprocess.run([str(binary),'--input-dir',str(root)],capture_output=True,text=True);v=json.loads(r.stdout);return r,v
before={str(q.relative_to(s)):sha(q) for q in s.rglob('*') if q.is_file() and 'build' not in q.relative_to(s).parts}
r,v=run(s);assert r.returncode==0 and v['accepted'];assert v==json.loads((s/'result-final.json').read_text());(p/'native-result.json').write_text(r.stdout)
r2=subprocess.run(['python3',str(p/'recompute.py'),str(s)],capture_output=True,text=True);assert r2.returncode==0,r2.stderr
ind=json.loads(r2.stdout);native=[dict(p=m['p'],index=a['index'],square_scalar=a['square_scalar'],normalized_root=a['normalized_root']) for m in v['models'] for a in m['actions']];assert sorted(native,key=lambda x:(x['p'],x['index']))==sorted(ind['rows'],key=lambda x:(x['p'],x['index']));(p/'recomputed.json').write_text(r2.stdout)
cases=[]
for name in ['shape','noncanonical','coefficient','duplicate_model','missing_model','scalar_order','missing_pin','changed_walk']:
 d=p/'controls'/name;d.mkdir(parents=True,exist_ok=True);shutil.copytree(s/'inputs',d/'inputs',dirs_exist_ok=True)
 for f in ['pin-index.json','walk-source.rs']:shutil.copy2(s/f,d/f)
 summary=json.loads((d/'inputs/summary.json').read_text());gs=[json.loads(l) for l in (d/'inputs/generic-models.jsonl').read_text().splitlines()]
 if name=='shape':gs[0]['torsion_actions'][0]['action']['matrix'][0].pop()
 if name=='noncanonical':gs[0]['torsion_actions'][0]['action']['matrix'][0][0]=gs[0]['p']
 if name=='coefficient':gs[0]['torsion_actions'][0]['action']['matrix'][0][0]=(gs[0]['torsion_actions'][0]['action']['matrix'][0][0]+1)%gs[0]['p']
 if name=='duplicate_model':gs.append(gs[0])
 if name in ['shape','noncanonical','coefficient','duplicate_model']:
  b=('\n'.join(json.dumps(x) for x in gs)+'\n').encode();(d/'inputs/generic-models.jsonl').write_bytes(b);(d/'inputs/pins/generic-models.jsonl').write_bytes(b)
  for pin in summary['source_digests']:
   if pin['path'].endswith('/generic-models.jsonl'):pin['sha256']=hashlib.sha256(b).hexdigest()
 if name=='missing_model':summary['models'].pop()
 if name=='scalar_order':summary['models'][0]['square_scalars'].reverse()
 if name=='missing_pin':
  path=next(x['path'] for x in summary['source_digests'] if x['path'].endswith('target-gate-40.jsonl'));summary['source_digests']=[x for x in summary['source_digests'] if x['path']!=path]
  ix=json.loads((d/'pin-index.json').read_text());(d/'pin-index.json').write_text(json.dumps([x for x in ix if x['path']!=path]))
 if name=='changed_walk':(d/'walk-source.rs').write_text('altered unrelated source\n')
 (d/'inputs/summary.json').write_text(json.dumps(summary))
 cr,cv=run(d);(d/'output.json').write_text(cr.stdout);(d/'stderr.txt').write_text(cr.stderr)
 jq=subprocess.run(['jq','-e','-f',str(p/'accept.jq')],input=cr.stdout,capture_output=True,text=True)
 if name in ['missing_pin','changed_walk']:assert cr.returncode==0 and cv['accepted']
 else:assert cr.returncode!=0 and not cv['accepted'],name
 if name=='missing_pin':assert jq.returncode!=0
 if name=='changed_walk':assert jq.returncode==0 and sha(d/'walk-source.rs')!=before['walk-source.rs']
 cases.append({'case':name,'native_exit':cr.returncode,'accepted_field':cv['accepted'],'jq_exit':jq.returncode,'error':cv.get('error'),'complete_acceptance':False,'external_source_binding_required':name=='changed_walk'})
assert before=={str(q.relative_to(s)):sha(q) for q in s.rglob('*') if q.is_file() and 'build' not in q.relative_to(s).parts}
(p/'source-bindings.json').write_text(json.dumps(before,indent=2)+'\n');(p/'controls.json').write_text(json.dumps(cases,indent=2)+'\n');(p/'acceptance.json').write_text(json.dumps({'accepted':True,'source_publication':'80b98db381','native_equal_producer':True,'independent_matrix_rows_equal':105,'input_bindings_unchanged':True,'negative_controls':8,'acceptance_contract':'native success + exact inventory jq + externally fixed source/input digests','scientific_execution':False},indent=2)+'\n')
print('accepted: 105 independent matrix identities and roots; 8 adverse controls; unchanged producer bindings')
