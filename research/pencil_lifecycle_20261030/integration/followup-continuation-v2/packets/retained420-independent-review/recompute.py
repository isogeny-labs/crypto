from pathlib import Path
import json,hashlib,collections
P=Path(__file__).parent;D=Path('/var/tmp/pencil-kummer-retained-duplicate-frozen-v2-20261031');H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();load=lambda n:json.loads((D/n).read_text());jl=lambda n:[json.loads(s)for s in(D/n).read_text().splitlines()]
m=load('pre-execution-manifest.json');hashes={n:H(D/n)==v['sha256'] and (D/n).stat().st_size==v['bytes'] for n,v in m['files'].items()};assert all(hashes.values())
refs=load('control-references.json');refchecks={n:H(Path(n))==h for n,h in refs.items()};assert all(refchecks.values())
admission=json.loads(Path('/var/tmp/pencil-retained-audit-root-admission-20261031.json').read_text());receipt=load('receipt.json');assert admission['producer_manifest_sha256']==H(D/'pre-execution-manifest.json')==receipt['preexecution_manifest_sha256'];assert receipt['exit_code']==0
archive=jl('archive.jsonl');idx=jl('input-index.jsonl');rows=jl('run.stdout');selected=[(i,v) for i,v in enumerate(archive) if v['p']==739];expected=[(i,c['case_index'])for i,v in selected for c in v['cases']];assert len(archive)==180 and len(selected)==60
assert expected==[(x['archive_row'],x['case_index'])for x in idx]==[(x['identity']['archive_row'],x['identity']['case_index'])for x in rows];assert len(set(expected))==420
status=collections.Counter();adapters=collections.Counter();graphs=collections.Counter();ledger={};tables={};post=0;per_target={};phases=collections.defaultdict(collections.Counter);scalar=collections.defaultdict(collections.Counter)
for x in rows:
 ident=x['identity'];res=x['result'];row=ident['archive_row'];case=ident['case_index'];v=archive[row];c=v['cases'][case];assert c['case_index']==case
 for name in ['target_index','workload_id','workload_sha256','algorithm_seed']:assert ident[name]==v[name]
 assert ident['p']==v['p']==739 and ident['r']==v['r']==34147
 assert ident['g']==v['cases'][1]['coordinate_witness']['jacobian_expected'] and ident['q']==v['cases'][2]['coordinate_witness']['jacobian_expected']
 assert res['divisor']==c['coordinate_witness']['jacobian_expected'] and (res['a'],res['b'])==(c['a'],c['b']) and res['native_evaluated']
 record={k:x[k]for k in ['constructor_field','setup_checks_field']};assert x['setup_count_once_per_target'] and x['setup_counter_scope'].startswith('nonadditive')
 if row in ledger:assert ledger[row]==record
 else:ledger[row]=record
 status[res['status']]+=1;per_target.setdefault(row,collections.Counter())[res['status']]+=1
 for k,z in res.items():
  if k.endswith('_field'):phases[k].update(z)
 d=res['divisor'];p=739;r=34147;disc=(d['u'][1]**2-4*d['u'][0])%p;eligible=d['degree']==2 and disc!=0 and pow(disc,(p-1)//2,p)==1
 assert eligible==(res['status']=='compared')
 if not eligible:continue
 def key(dd):return (dd['degree'],tuple(dd['u']),tuple(dd['v']))
 neg={'degree':d['degree'],'u':d['u'],'v':[(-a)%p for a in d['v']]};canon=min([d,neg],key=key);flip=canon!=d;labels=[(-res[k])%r if flip else res[k] for k in ['a','b']];a=res['adapter'];t=tables.setdefault(row,{})
 assert a['canonical_divisor']==canon and a['sign_flipped']==flip and a['normalized_labels']==labels
 kk=key(canon)
 if kk in t:
  old,first=t[kk];delta=[(u-w)%r for u,w in zip(labels,old)];expected_status='redundant' if delta==[0,0] else ('invariant_error' if delta[1]==0 else 'candidate_verified')
  assert a['delta']==delta and not a['emit_graph_edge']
 else:t[kk]=(labels,case);first=case;expected_status='first';assert a['delta'] is None and a['emit_graph_edge']
 assert a['status']==expected_status and a['stored_pairs']==len(t) and res['first_case_index']==first
 adapters[a['status']]+=1;post+=res['post_terminal_diagnostic'];g=res['graph'];graphs[g['status']]+=1
 assert a['answer'] is None and not a['candidate_verified']
 if a['status']=='first':
  assert g['status']=='inserted' and g['factor_sum_verified'] and g['answer']==g['dense_answer']
  assert g['equation']['a']==res['a'] and g['equation']['b']==res['b']
 else:assert g['status']=='duplicate_suppressed'
 for k,z in g.items():
  if k.endswith('_field'):phases['graph.'+k].update(z)
 phases['adapter.field'].update(a['field'])
 for name,before,after in [('adapter',a['scalar_before'],a['scalar_after']),('graph',g.get('ring_before',{}),g.get('ring_after',{}))]:
  for k,n in after.items():assert n>=before[k];scalar[name][k]+=n-before[k]
setup={phase:dict(sum((collections.Counter(v[phase])for v in ledger.values()),collections.Counter())) for phase in ['constructor_field','setup_checks_field']}
# Keep explicit zero keys for exact producer ledger comparison.
for phase in setup:
 for k in next(iter(ledger.values()))[phase]:setup[phase].setdefault(k,0)
analysis=load('analysis.json');assert setup==analysis['setup_accounting']['totals_count_each_target_once'];assert {str(k):v for k,v in ledger.items()}==analysis['setup_accounting']['per_target_ledger'];assert dict(status)==analysis['statuses'] and dict(adapters)==analysis['adapter_statuses']
result={'schema':'research.archival-recompute/v1','archive_records':len(archive),'selected_targets':len(selected),'dispositions':len(rows),'identity_order_exact':True,'statuses':dict(status),'adapter_statuses':dict(adapters),'informative':adapters['candidate_verified'],'graph_statuses':dict(graphs),'post_terminal_diagnostic_cases':post,'once_per_target_ledger_entries':len(ledger),'once_per_target_totals':setup,'row_phase_totals':{k:dict(v)for k,v in phases.items()},'scalar_phase_deltas':{k:dict(v)for k,v in scalar.items()},'per_target_counts':{str(k):dict(v)for k,v in per_target.items()},'all_source_hashes_match':all(hashes.values()),'source_entries':len(hashes),'control_references_checked':refchecks,'admission_manifest_matches':True,'native_replayed':False,'native_exit':receipt['exit_code'],'complete_cost_ratio':None,'natural_yield':None}
(P/'recomputed.json').write_text(json.dumps(result,indent=2)+'\n');(P/'input-bindings.json').write_text(json.dumps({str(D/n):H(D/n)for n in ['pre-execution-manifest.json','protocol.json','run.stdout','run.stderr','receipt.json','analysis.json','archive.jsonl','input-index.jsonl','src/check.rs','src/pruned.rs','src/duplicate.rs','src/graph.rs','native/src/three_leads.rs']},indent=2)+'\n');print(json.dumps({k:v for k,v in result.items()if k not in ['per_target_counts','control_references_checked']},indent=2))
