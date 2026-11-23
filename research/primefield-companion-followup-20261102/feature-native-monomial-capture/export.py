"""Convert a successful captured panel to the preserved canonical export schema."""
from pathlib import Path
import importlib.util,json,argparse,hashlib
D=Path(__file__).resolve().parent;R=D.parent

def load(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
protocol_tool=load('protocol_tool',R/'tools/experiment_protocol.py')
analyzer=load('matched_analyzer',R/'feature-native-monomial-matched/analyze.py')
def convert(rows,p,receipt):
 if p['freeze']['status']!='frozen' or p['freeze']['protocol_sha256']!=protocol_tool.protocol_hash(p):raise ValueError('frozen protocol required')
 if type(receipt['exit_code'])is not int or receipt['exit_code']!=0 or receipt.get('timed_out')is not False:raise ValueError('successful external process required')
 if receipt['protocol_sha256']!=p['freeze']['protocol_sha256']:raise ValueError('protocol binding mismatch')
 if {a['instrument']['binary_sha256']for a in p['arms']}!={receipt['binary_sha256']}:raise ValueError('binary binding mismatch')
 result=analyzer.analyze(rows);e=protocol_tool.export_skeleton(p,'protocol.json','manual','monomial-captured-panel')
 metrics=p['metrics']['primary']+p['metrics']['secondary']
 for pair in result['pairs']:
  for arm in analyzer.ARMS:
   values=pair['single_target_events'][arm]
   e['runs'].append(protocol_tool.run_row(run_id=f"{arm}-{pair['label']}",arm=arm,instance_id=str(pair['label']),seed=20261107,measurements={'counted':{m['id']:values[i]for i,m in enumerate(metrics)},'wall_valid':False},verification={'status':'verified','verifier':'native forward multiplication; strict saved-fixture parser, producer authored'},isolation={'tier':None,'blockers':['host isolation not supplied by process capture']}))
 e['flow']=protocol_tool.compute_flow(e['runs']);e['notes']=[{'kind':'observation','text':'Fixed costs including teardown added to each query. No clock claim. Raw stage vectors retained separately.','by':'monomial capture adapter'}]
 return e

def main():
 ap=argparse.ArgumentParser();ap.add_argument('directory',type=Path);ap.add_argument('--protocol',type=Path,required=True);a=ap.parse_args();raw=a.directory/'rows.jsonl';receipt=json.loads((a.directory/'producer-receipt.json').read_text());p=json.loads(a.protocol.read_text())
 if hashlib.sha256(raw.read_bytes()).hexdigest()!=receipt['raw_sha256']:raise ValueError('raw hash mismatch')
 rows=[json.loads(l,object_pairs_hook=analyzer.unique)for l in raw.read_text().splitlines()];e=convert(rows,p,receipt)
 # Copy frozen protocol into the fresh output directory exclusively; preserve all inputs.
 dest=a.directory/'protocol.json'
 with dest.open('xb')as f:f.write(a.protocol.read_bytes())
 e['files']=[protocol_tool.file_entry(a.directory,n,k)for n,k in [('rows.jsonl','raw'),('producer-receipt.json','summary'),('protocol.json','protocol')]]
 errors=protocol_tool.schema_errors(e,'experiment.export/v1')
 if errors:raise ValueError(errors)
 with(a.directory/'export.json').open('x')as f:json.dump(e,f,indent=2);f.write('\n')
if __name__=='__main__':main()
