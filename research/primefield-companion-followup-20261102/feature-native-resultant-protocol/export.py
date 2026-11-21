"""Success-only canonical adapter; failed/partial attempts remain raw receipts, never fake cells."""
from pathlib import Path
import argparse,json,os
from common import canonical,analyzer,protocol,bindings,sha,require

def build(p,protocol_path,rows_path,receipt_path,base):
 b=bindings(p,protocol_path.parent);receipt=analyzer.read_json(receipt_path.read_text());raw=rows_path.read_bytes();ph=p['freeze']['protocol_sha256']
 require(type(receipt.get('exit_code'))is int and receipt['exit_code']==0,'attempt not successful')
 require(receipt.get('raw_sha256')==sha(rows_path),'raw receipt mismatch')
 require(receipt.get('protocol_sha256')==ph,'receipt protocol mismatch')
 binary=p['arms'][0]['instrument']['binary_sha256'];require(binary and receipt.get('binary_sha256')==binary,'binary receipt mismatch')
 require(receipt.get('input_sha256')==sha(b['inputs']) and receipt.get('metrics_sha256')==sha(b['metrics']),'input/metrics receipt mismatch')
 rows=[analyzer.read_json(x)for x in raw.decode().splitlines()];inputs=analyzer.fixtures(b['inputs'].read_text());names=b['metrics'].read_text().splitlines()
 analysis=analyzer.validate_rows(rows,inputs,names,ph,sha(b['inputs']))
 rel=lambda path:os.path.relpath(path,base)
 e=canonical.export_skeleton(p,rel(protocol_path),'isolab-k8s',p['protocol_id']+'-attempt-1')
 e['source'].update(commit=p['arms'][0]['instrument']['commit'],dirty=None)
 if isinstance(receipt.get('host'),dict):e['host']['summary'].update(receipt['host'])
 for arm in e['arms']:arm.update(binary_sha256=binary,commit=p['arms'][0]['instrument']['commit'])
 for row in rows:
  e['runs'].append(canonical.run_row(run_id=f"case-{row['case_id']}-{row['arm']}",arm=row['arm'],instance_id=str(row['case_id']),seed=20261107,measurements={'counted':row['metrics']},isolation={'tier':None,'contended':None,'blockers':['No timing claim; isolation not independently graded by this adapter']},verification={'status':'no_claim','verifier':'producer-authored direct-product oracle; not independent review'},backend_ref=receipt.get('job_uid')))
 e['flow']=canonical.compute_flow(e['runs'])
 e['files']=[canonical.file_entry(base,rel(f),kind)for f,kind in [(rows_path,'raw'),(receipt_path,'receipt'),(b['inputs'],'other'),(b['metrics'],'other'),(protocol_path,'protocol')]]
 e['analysis']['script']=canonical.file_entry(base,rel(b['analysis']),'analysis')
 e['notes']=[{'kind':'observation','text':'Complete panel passed producer oracle, A/A and planted-operation checks. Separate counted dimensions; no clock or independent-review claim.','by':'resultant-adapter'}]
 errors=canonical.schema_errors(e,'experiment.export/v1')or canonical.lint_export(e,p,base,True);require(not errors,str(errors))
 return e,analysis

def main():
 a=argparse.ArgumentParser()
 for name in ['protocol','expected-protocol','rows','receipt','out']:a.add_argument('--'+name,required=True)
 args=a.parse_args();pp=Path(args.protocol).resolve();p=protocol(pp,args.expected_protocol);out=Path(args.out).resolve();require(not out.exists(),'output directory exists')
 # Reserve a write-once attempt directory; a refusal may leave it empty.
 out.mkdir(parents=True,exist_ok=False)
 e,analysis=build(p,pp,Path(args.rows).resolve(),Path(args.receipt).resolve(),out)
 canonical.write_once(out/'comparison.json',analysis)
 e['analysis']['outputs']=[canonical.file_entry(out,'comparison.json','analysis')]
 canonical.write_once(out/'export.json',e)
 print(out/'export.json')
if __name__=='__main__':main()
