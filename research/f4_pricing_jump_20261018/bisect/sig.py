import json,sys
recs=[json.loads(l) for l in open(sys.argv[1]+'/records.jsonl')]
out=[]
for r in recs:
    if not r['arm'].startswith('ic-f4'): continue
    ph={p['name']:p for p in r['phases']}
    rel=ph['relations']
    out.append(dict(arm=r['arm'],round=r['round'],warmup=r['warmup'],target=r['workload'].get('target_index',r['workload'].get('target')),
      total_gae=r['cost']['total_gae'],lower_bound=r['cost']['lower_bound'],unpriced=r['cost']['unpriced'],
      phase_gae={k:v['gae'] for k,v in ph.items()},solver_ops=rel['native'].get('solver_ops'),
      solver_calls=rel['native'].get('solver_calls'),trials=rel['native'].get('trials'),relations=rel['native'].get('relations'),
      adds=rel['adds'],doubles=rel['doubles'],outcome=r['outcome'].get('status') if isinstance(r['outcome'],dict) else r['outcome']))
out.sort(key=lambda d:(d['arm'],d['round'],d['warmup'],str(d['target'])))
json.dump(out,sys.stdout,sort_keys=True)
