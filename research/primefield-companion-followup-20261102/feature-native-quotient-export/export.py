"""Complete paired captures only; never synthesizes cells for failed attempts."""
from pathlib import Path
import argparse,os
from common import canonical,analyzer,protocol,bindings,profile,sha,require,pair,FILE_LIMIT

def build(p,pp,attempt,out):
 profile(p);b=bindings(p,pp.parent);receipt_path=attempt/'receipt.json';r=analyzer.read_json(receipt_path.read_text());ph=p['freeze']['protocol_sha256'];binary=p['arms'][0]['instrument']['binary_sha256']
 require(r.get('status')=='succeeded' and type(r.get('exit_code'))is int and r['exit_code']==0 and r.get('attempts')==1,'not successful single attempt')
 require(r.get('protocol_sha256')==ph and binary and r.get('binary_sha256')==binary,'receipt protocol/binary mismatch')
 require(r.get('source_commit')==p['arms'][0]['instrument']['commit'] and r.get('file_limit_bytes')==FILE_LIMIT,'source/output ceiling mismatch')
 require([x['arm']for x in r['processes']]==['trace','aa'],'missing/extra process')
 raw=[]
 for arm,x in zip(['trace','aa'],r['processes']):
  require(x.get('executed')is True and x.get('status')=='succeeded' and type(x.get('exit_code'))is int and x['exit_code']==0,'unsuccessful process')
  for kind,filename in [('stdout',arm+'.csv'),('stderr',arm+'.stderr')]:
   path=attempt/filename;require(x.get(kind)==filename and x.get(kind+'_sha256')==sha(path) and x.get(kind+'_bytes')==path.stat().st_size<=FILE_LIMIT,'output receipt mismatch');raw.append(path)
 result=pair(attempt/'trace.csv',attempt/'aa.csv');rel=lambda path:os.path.relpath(path,out)
 e=canonical.export_skeleton(p,rel(pp),'isolab-k8s',p['protocol_id']+'-attempt-1');e['source'].update(commit=p['arms'][0]['instrument']['commit'],dirty=None)
 if isinstance(r.get('host'),dict):e['host']['summary'].update(r['host'])
 for arm in e['arms']:arm.update(binary_sha256=binary,commit=p['arms'][0]['instrument']['commit'])
 for arm in ['trace','aa']:
  for i,t in enumerate(result['targets']):
   e['runs'].append(canonical.run_row(run_id=f'{arm}-{i}',arm=arm,instance_id=str(i),seed=0,measurements={'counted':{'quotients':t['quotients'],'products':t['quotient_products'],'inversions':t['quotient_inversions']}},isolation={'tier':None,'contended':None,'blockers':['Counted source quotient stages only; no valid wall clock']},verification={'status':'no_claim','verifier':'producer target equality and transcript parser; not independent review'},backend_ref=r.get('job_uid')))
 e['flow']=canonical.compute_flow(e['runs']);e['files']=[canonical.file_entry(out,rel(f),kind)for f,kind in [(x,'raw')for x in raw]+[(receipt_path,'receipt'),(pp,'protocol')]];e['analysis']['script']=canonical.file_entry(out,rel(b['pair']),'analysis');e['notes']=[{'kind':'observation','text':'Two complete identical fixed-panel transcripts. Coordinate-field source quotient stages only; no whole-source or timing result.','by':'quotient-export'}]
 errors=canonical.schema_errors(e,'experiment.export/v1')or canonical.lint_export(e,p,out,True);require(not errors,str(errors));return e,result

def main():
 ap=argparse.ArgumentParser()
 for k in ['protocol','expected-protocol','attempt','out']:ap.add_argument('--'+k,required=True)
 args=ap.parse_args();pp=Path(args.protocol).resolve();p=protocol(pp,args.expected_protocol);out=Path(args.out).resolve();out.mkdir(parents=True,exist_ok=False)
 e,result=build(p,pp,Path(args.attempt).resolve(),out);canonical.write_once(out/'analysis.json',result);e['analysis']['outputs']=[canonical.file_entry(out,'analysis.json','analysis')];canonical.write_once(out/'export.json',e);print(out/'export.json')
if __name__=='__main__':main()
