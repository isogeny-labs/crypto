"""Success-only producer export. Schema/receipt consistency is not independent review."""
from pathlib import Path
import importlib.util,json,hashlib,os,argparse,re
import analyze
D=Path(__file__).resolve().parent;A=D.parent/'feature-native-resultant-protocol'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def require(ok,message):
 if not ok:raise ValueError(message)
authority=json.loads((A/'authority.json').read_text())
for item in authority['files']:require(sha(A/'authority'/item['path'])==item['sha256'],'authority changed')
spec=importlib.util.spec_from_file_location('canonical',A/'authority/tools/experiment_protocol.py');canonical=importlib.util.module_from_spec(spec);spec.loader.exec_module(canonical)
def read(p):return json.loads(Path(p).read_text(),object_pairs_hook=analyze.unique)
def build(pp,raw,receipt,base):
 p=read(pp);errors=canonical.schema_errors(p,'experiment.protocol/v1')or canonical.lint_protocol(p,pp.parent);require(not errors,str(errors))
 require(p['freeze']['status']=='frozen'and p['freeze']['filed_before_affected_data']is True,'unfrozen protocol')
 for item in p['extensions']['bindings'].values():require(sha(pp.parent/item['path'])==item['sha256'],'binding mismatch')
 r=read(receipt);require(type(r.get('exit_code'))is int and r['exit_code']==0,'unsuccessful producer')
 require(r.get('raw_sha256')==sha(raw),'raw hash mismatch');require(r.get('protocol_sha256')==p['freeze']['protocol_sha256'],'protocol receipt mismatch')
 ins=p['arms'][0]['instrument'];require(ins['commit']and ins['binary_sha256'],'unbound production instrument')
 require(r.get('binary_sha256')==ins['binary_sha256']and r.get('source_commit')==ins['commit'],'instrument receipt mismatch')
 require(isinstance(r.get('execution_admission_uri'),str) and r['execution_admission_uri'].strip() and isinstance(r.get('execution_admission_sha256'),str) and re.fullmatch('[0-9a-f]{64}',r['execution_admission_sha256']),'missing execution admission reference')
 require(r.get('source_bindings')=={k:v['sha256'] for k,v in p['extensions']['bindings'].items()},'source binding receipt mismatch')
 rows=[json.loads(x,object_pairs_hook=analyze.unique)for x in raw.read_text().splitlines()];result=analyze.analyze(rows)
 rel=lambda f:os.path.relpath(f,base)
 e=canonical.export_skeleton(p,rel(pp),p['execution']['backend'],p['protocol_id']+'-attempt-1');e['source'].update(commit=ins['commit'],dirty=False)
 for arm in e['arms']:arm.update(commit=ins['commit'],binary_sha256=ins['binary_sha256'])
 for i in range(0,len(rows),2):
  initial,doubled=rows[i:i+2];counts={phase+'.'+name:row['counts'][j]for phase,row in [('initial',initial),('doubled',doubled)]for j,name in enumerate(analyze.NAMES)}
  e['runs'].append(canonical.run_row(run_id=f"case-{initial['case']}-{initial['arm']}",arm=initial['arm'],instance_id=str(initial['case']),seed=20261107,measurements={'counted':counts},verification={'status':'no_claim','verifier':'producer native dense oracle; no independent signed review'},backend_ref=r.get('job_uid')))
 e['flow']=canonical.compute_flow(e['runs']);e['files']=[canonical.file_entry(base,rel(f),kind)for f,kind in[(pp,'protocol'),(raw,'raw'),(receipt,'receipt')]]
 e['analysis']['script']=canonical.file_entry(base,rel(D/'analyze.py'),'analysis')
 e['notes']=[{'kind':'observation','text':'Two incremental phases per execution cell; no clock or independent-verification claim. Admission URI/digest are producer assertions requiring external review.','by':'interval-width-adapter'}]
 errors=canonical.schema_errors(e,'experiment.export/v1')or canonical.lint_export(e,p,base,True);require(not errors,str(errors));return e,result
if __name__=='__main__':
 parser=argparse.ArgumentParser()
 for n in ['protocol','rows','receipt','out']:parser.add_argument('--'+n,required=True)
 args=parser.parse_args();out=Path(args.out).resolve();out.mkdir(parents=True,exist_ok=False)
 e,result=build(Path(args.protocol).resolve(),Path(args.rows).resolve(),Path(args.receipt).resolve(),out)
 canonical.write_once(out/'comparison.json',result);e['analysis']['outputs']=[canonical.file_entry(out,'comparison.json','analysis')];canonical.write_once(out/'export.json',e)
