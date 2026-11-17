"""Export immutable Git source/JSON metadata; no comparison or proof verdicts."""
from pathlib import Path
import subprocess,json,hashlib,argparse
ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,default=Path(__file__).resolve().parents[2]);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--snapshot',default='325abd3fc4d3aaa5a32044f39fa4f1b43e84e451');args=ap.parse_args();r=args.repo.resolve();o=args.out.resolve();o.mkdir(parents=True,exist_ok=True)
snapshot=args.snapshot;base='967ff57c3814186044e95d6b6b4864cd91dfe289'
def git(*args):return subprocess.check_output(['git',*args],cwd=r)
assert git('rev-parse',snapshot+'^{tree}').decode().strip()=='d8b68301d79629e900e6d1d3a0a87d26ffbe4455','Replay snapshot must have the originally audited tree'
paths=git('diff','--name-only',base,snapshot,'--','research/*/sources.json').decode().splitlines();cases=[];inventory=[]
for i,path in enumerate(paths):
 raw=git('show',snapshot+':'+path);x=json.loads(raw);f=x.get('formal',{});name=f.get('theorem') or f.get('binding') if isinstance(f,dict)else None
 entry=dict(plan=path,plan_sha256=hashlib.sha256(raw).hexdigest(),schema=x.get('schema'),formal_keys=list(f)if isinstance(f,dict)else [])
 if not isinstance(name,str):
  entry['case_id']=None;entry['reason']='No named formal binding in this source-plan schema';inventory.append(entry);continue
 case_id=f'case-{i+1:03d}';entry['case_id']=case_id;inventory.append(entry)
 project=f.get('project');source=f.get('source');short=name.split('.')[-1]
 if project:
  candidates=[p for p in git('ls-tree','-r','--name-only',snapshot,'--',project).decode().splitlines()if p.endswith('.lean')and '.lake/'not in p and 'lakefile'not in p]
 else:candidates=[source]if source else []
 candidates_out=[]
 for original in candidates:
  b=git('show',snapshot+':'+original)
  if short.encode()not in b:continue
  target=o/case_id/original;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(b)
  declared=next((z.get('sha256')for z in f.get('sources',[])if isinstance(z,dict)and z.get('path')==original),None)
  if original==source:declared=f.get('source_sha256',declared)
  candidates_out.append(dict(source_path=original,export_path=str(target),bytes=len(b),sha256=hashlib.sha256(b).hexdigest(),declared_source_sha256=declared))
 cases.append(dict(case_id=case_id,source_plan=path,plan_sha256=entry['plan_sha256'],project=project,theorem=name,short_name=short,expected_decl_sha256=f.get('decl_sha256'),recorded_statement=f.get('statement'),candidates=candidates_out))
x=dict(schema='immutable-statement-binding-inputs/v1',snapshot_commit=snapshot,snapshot_tree=git('rev-parse',snapshot+'^{tree}').decode().strip(),comparison_base=base,source_plan_inventory=inventory,cases=cases,scope='65 source plans added to current stacked feature. Explicit/alternate formal binding schemas exported; native/catalog-only plans listed, not silently counted as theorem passes. Existing metadata is input, not a verdict.')
p=o/'manifest.json';p.write_text(json.dumps(x,indent=2,ensure_ascii=False)+'\n');print(json.dumps({'plans':len(paths),'named_formal_bindings':len(cases),'candidate_files':sum(len(c['candidates'])for c in cases),'manifest_bytes':p.stat().st_size,'no_recorded_statement':sum(c['recorded_statement']is None for c in cases)}))
