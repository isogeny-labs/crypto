"""Partial attempt export: preserve canonical incompleteness findings, never fabricate cells."""
from pathlib import Path
import sys,json,os,argparse
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'feature-native-resultant-protocol'))
from common import canonical,analyzer,protocol,bindings,sha,require

def prefix(data):
 lines=data.splitlines(keepends=True);records=[];tail=b''
 for i,line in enumerate(lines):
  if not line.endswith(b'\n'):
   require(i==len(lines)-1,'interior unterminated record');tail=line;break
  records.append(analyzer.read_json(line.decode()))
 return records,tail

def classify(events,rows,inputs,names,ph,ih):
 started=[];completed=[];pending=None
 for e in events:
  require(set(e)=={'ordinal','phase','protocol_sha256','input_sha256'},'event columns')
  require(type(e['ordinal'])is int and 0<=e['ordinal']<240,'event ordinal')
  require(e['protocol_sha256']==ph and e['input_sha256']==ih,'event custody')
  if e['phase']=='start':
   require(pending is None and e['ordinal']==len(started),'start order');pending=e['ordinal'];started.append(pending)
  elif e['phase']=='complete':
   require(pending==e['ordinal'],'completion without matching start');completed.append(pending);pending=None
  else:raise ValueError('event phase')
 require(len(completed)<=len(rows)<=len(started),'row/journal mismatch')
 for ordinal,r in enumerate(rows):
  c=inputs[ordinal//5];arm=analyzer.ARMS[(ordinal%5+c['case_id'])%5]
  require(set(r)=={'case_id','arm','ordinal','p','kind','n','m','value','expected','protocol_sha256','input_sha256','metrics'},'row columns')
  require(all(analyzer.integer(r[k])for k in ['case_id','ordinal','p','n','m','value','expected']),'row integer')
  require(r['ordinal']==ordinal and r['arm']==arm,'row order')
  require(all(r[k]==c[k]for k in ['case_id','p','kind','n','m','expected'])and r['value']==c['expected'],'oracle/metadata mismatch')
  require(r['protocol_sha256']==ph and r['input_sha256']==ih,'row custody')
  m=r['metrics'];require(type(m)is dict and set(m)==set(names)and all(analyzer.integer(v)for v in m.values()),'metric shape')
  require(all(m[k]==0 for k in ['live_bytes','allocation_failures','accounting_invalid','counter_overflow'])and m['Projection']==1,'invalid accounting row')
 return {'started':started,'completed':completed,'completion_unconfirmed':[]if pending is None else[pending],'never_started':list(range(len(started),240)),'raw_rows_without_completion_marker':list(range(len(completed),len(rows)))}

def build(p,pp,raw_path,event_path,receipt_path,out):
 b=bindings(p,pp.parent);r=analyzer.read_json(receipt_path.read_text());ph=p['freeze']['protocol_sha256'];ih=sha(b['inputs']);binary=p['arms'][0]['instrument']['binary_sha256']
 require(r.get('status')in ['failed','timeout','infra_error','interrupted'],'not a failed attempt')
 require(r.get('exit_code')is None or (type(r['exit_code'])is int and r['exit_code']!=0),'contradictory exit status')
 require(binary and r.get('binary_sha256')==binary and r.get('protocol_sha256')==ph,'receipt binary/protocol')
 require(r.get('input_sha256')==ih and r.get('metrics_sha256')==sha(b['metrics']),'receipt inputs/metrics')
 for path,key in [(raw_path,'raw_sha256'),(event_path,'events_sha256')]:require((sha(path)if path.exists()else None)==r.get(key),'receipt file digest')
 raw=raw_path.read_bytes()if raw_path.exists()else b'';ev=event_path.read_bytes()if event_path.exists()else b''
 rows,rt=prefix(raw);events,et=prefix(ev);inputs=analyzer.fixtures(b['inputs'].read_text());names=b['metrics'].read_text().splitlines();flow=classify(events,rows,inputs,names,ph,ih)
 require(not et or len(flow['started'])<240 or flow['completion_unconfirmed'],'unexpected trailing event fragment')
 require(not rt or (flow['completion_unconfirmed'] and len(rows)==len(flow['completed'])),'raw fragment without pending cell')
 rel=lambda x:os.path.relpath(x,out)
 e=canonical.export_skeleton(p,rel(pp),'isolab-k8s',p['protocol_id']+'-failed-attempt-1');e['source'].update(commit=p['arms'][0]['instrument']['commit'],dirty=None)
 for a in e['arms']:a.update(binary_sha256=binary,commit=p['arms'][0]['instrument']['commit'])
 for ordinal in flow['started']:
  cid=ordinal//5;arm=analyzer.ARMS[(ordinal%5+cid)%5];done=ordinal in flow['completed']
  e['runs'].append(canonical.run_row(run_id=f'case-{cid}-{arm}',arm=arm,instance_id=str(cid),seed=20261107,status='succeeded'if done else'interrupted',measurements={'counted':rows[ordinal]['metrics']if done else{}},verification={'status':'no_claim','verifier':'producer completion journal and oracle only'},exclusion={'excluded':True,'rule':'Incomplete/failed panel; no comparative inference'},backend_ref=r.get('job_uid')))
 e['flow']=canonical.compute_flow(e['runs']);e['files']=[canonical.file_entry(out,rel(x),kind)for x,kind in [(pp,'protocol'),(raw_path,'raw'),(event_path,'log'),(receipt_path,'receipt'),(b['inputs'],'other'),(b['metrics'],'other')]if x.exists()]
 e['analysis']['script']=canonical.file_entry(out,rel(b['analysis']),'analysis')
 e['notes']=[{'kind':'observation','by':'failure-adapter','text':'Failed attempt. Only journal-started cells exported. Every exported row excluded from comparison; unstarted cells listed in failure-flow.json. Canonical completeness findings are retained, not waived.'}]
 require(not canonical.schema_errors(e,'experiment.export/v1'),'export schema failure')
 require(not canonical.lint_export(e,None,out,True),'export internal/file failure')
 flow.update(schema='primefield-resultant-failure-flow/v1',valid_comparison=False,protocol_sha256=ph,receipt_sha256=sha(receipt_path),raw_tail_bytes=len(rt),event_tail_bytes=len(et),canonical_protocol_findings=canonical.lint_export(e,p,out,True))
 return e,flow

def main():
 a=argparse.ArgumentParser()
 for k in ['protocol','expected-protocol','raw','events','receipt','out']:a.add_argument('--'+k,required=True)
 args=a.parse_args();pp=Path(args.protocol).resolve();p=protocol(pp,args.expected_protocol);out=Path(args.out).resolve();out.mkdir(parents=True,exist_ok=False)
 e,flow=build(p,pp,Path(args.raw).resolve(),Path(args.events).resolve(),Path(args.receipt).resolve(),out)
 canonical.write_once(out/'failure-flow.json',flow);e['analysis']['outputs']=[canonical.file_entry(out,'failure-flow.json','analysis')];canonical.write_once(out/'export.json',e)
 print(json.dumps({'valid_comparison':False,'attempted_cells':len(flow['started']),'unstarted_cells':len(flow['never_started']),'canonical_protocol_findings':flow['canonical_protocol_findings']}))
if __name__=='__main__':main()
