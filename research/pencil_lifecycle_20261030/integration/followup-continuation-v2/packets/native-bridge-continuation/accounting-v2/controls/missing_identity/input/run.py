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
# Setup copies are nonadditive row metadata. Build one ledger entry per target,
# requiring every repeated copy to agree before permitting aggregation.
setup_ledger={};setup_consistency_errors=[];failed_dispositions=[]
archive_failure_statuses={'partition_binding_failed','partition_poisoned','native_case_panic','invalid_label_or_divisor'}
adapter_failure_statuses={'invariant_error','candidate_rejected','noncanonical_label'}
for x in rows:
 ident=x.get('identity',x);result=x.get('result',x);key=str(ident.get('archive_row'))
 if 'setup_checks_field' in x or 'constructor_field' in x:
  entry={'setup_checks_field':x.get('setup_checks_field'),'constructor_field':x.get('constructor_field')}
  if key in setup_ledger and setup_ledger[key]!=entry:
   setup_consistency_errors.append({'archive_row':ident.get('archive_row'),'observed':entry,'first':setup_ledger[key]})
  else:setup_ledger.setdefault(key,entry)
 nested=result.get('adapter',{}).get('status');top=result.get('status')
 if top in archive_failure_statuses or nested in adapter_failure_statuses:
  failed_dispositions.append({'archive_row':ident.get('archive_row'),'case_index':ident.get('case_index'),'status':top,'adapter_status':nested})
setup_totals={}
for entry in setup_ledger.values():
 for phase,counters in entry.items():
  if not isinstance(counters,dict):
   setup_consistency_errors.append({'malformed_phase':phase,'value':counters});continue
  totals=setup_totals.setdefault(phase,{})
  for metric,value in counters.items():totals[metric]=totals.get(metric,0)+value
complete=r.returncode==0 and not parse_errors and len(ids)==420 and len(set(ids))==420 and set(ids)==expected
any_failed=bool(failed_dispositions or setup_consistency_errors)
analysis={'complete':complete,'audit_pass':complete and not any_failed,'native_exit':r.returncode,'dispositions':len(rows),'statuses':dict(statuses),'adapter_statuses':dict(adapter),'missing':sorted(expected-set(ids)),'unexpected':list(set(ids)-expected),'parse_errors':parse_errors,'all_rows_preserved':True,'natural_yield':None,'complete_cost_ratio':None,'any_failed_dispositions':bool(failed_dispositions),'failed_dispositions':failed_dispositions,'failed_disposition_count':len(failed_dispositions),'setup_accounting':{'row_copies_are_nonadditive':True,'ledger_key':'physical archive_row','per_target_ledger':setup_ledger,'consistency_errors':setup_consistency_errors,'totals_count_each_target_once':setup_totals,'totals_valid':not setup_consistency_errors}}
(D/'analysis.json').write_text(json.dumps(analysis,indent=2)+'\n');print(json.dumps(analysis));sys.exit(0 if analysis['audit_pass'] else 1)
