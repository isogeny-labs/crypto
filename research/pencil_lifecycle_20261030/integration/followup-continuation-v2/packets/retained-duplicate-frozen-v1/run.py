from pathlib import Path
import subprocess,json,time,hashlib,os,collections,sys
D=Path(__file__).parent
for name in ['run.stdout','run.stderr','receipt.json','analysis.json']:
 if (D/name).exists():raise SystemExit('Refusing overwrite: '+name)
m=json.loads((D/'pre-execution-manifest.json').read_text())
for name,b in m['files'].items():
 assert hashlib.sha256((D/name).read_bytes()).hexdigest()==b['sha256'],name
cmd=['timeout','-k','5s','120s','cargo','run','--locked','--offline','--manifest-path',str(D/'Cargo.toml'),'--bin','equivalence']
env=os.environ.copy();env['CARGO_TARGET_DIR']=str(D/'target');t=time.time()
with (D/'run.stdout').open('w') as out,(D/'run.stderr').open('w') as err:r=subprocess.run(cmd,cwd=D,env=env,stdout=out,stderr=err)
receipt={'command':cmd,'exit_code':r.returncode,'start':t,'end':time.time(),'preexecution_manifest_sha256':hashlib.sha256((D/'pre-execution-manifest.json').read_bytes()).hexdigest()}
(D/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
index=[json.loads(s) for s in (D/'input-index.jsonl').read_text().splitlines()];expected={(x['archive_row'],x['case_index']) for x in index};rows=[];parse_errors=[]
for i,s in enumerate((D/'run.stdout').read_text().splitlines()):
 try:rows.append(json.loads(s))
 except Exception as e:parse_errors.append({'line':i,'error':str(e)})
ids=[];statuses=collections.Counter();adapter=collections.Counter()
for x in rows:
 z=x.get('identity',x);ids.append((z.get('archive_row'),z.get('case_index')));result=x.get('result',x);statuses[result.get('status','missing')]+=1
 if 'adapter' in result:adapter[result['adapter']['status']]+=1
complete=r.returncode==0 and not parse_errors and len(ids)==420 and len(set(ids))==420 and set(ids)==expected
analysis={'complete':complete,'native_exit':r.returncode,'dispositions':len(rows),'statuses':dict(statuses),'adapter_statuses':dict(adapter),'missing':sorted(expected-set(ids)),'unexpected':list(set(ids)-expected),'parse_errors':parse_errors,'all_rows_preserved':True,'natural_yield':None,'complete_cost_ratio':None,'any_failed_dispositions':any(k in statuses for k in ['partition_binding_failed','partition_poisoned','native_case_panic','invalid_label_or_divisor'])}
(D/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n');print(json.dumps(analysis));sys.exit(0 if complete else 1)
