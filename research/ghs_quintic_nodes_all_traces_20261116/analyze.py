#!/usr/bin/env python3
"""Audit the complete geometry census and classify exact trace strata."""
from pathlib import Path
from collections import Counter,defaultdict
import hashlib
import json

here=Path(__file__).parent
source=here/'classify.py'
source_hash=hashlib.sha256(source.read_bytes()).hexdigest()
input_paths=[here.parent/'ghs_trace_minus14_quintic_nodes_20261116'/('FULL_%d.json'%i)
             for i in (0,1)]
input_hashes=[hashlib.sha256(p.read_bytes()).hexdigest() for p in input_paths]
inputs=[json.loads(p.read_text()) for p in input_paths]
rows=[json.loads(line) for line in (here/'FULL.jsonl').read_text().splitlines()]
assert len(rows)==1971
keys=set()
for row in rows:
    case,index=row['case'],row['index']
    assert row['source_sha256']==source_hash
    assert row['input_sha256']==input_hashes[case]
    assert row['coefficients']==inputs[case]['n4_candidates'][index]['coefficients']
    assert (case,index) not in keys
    keys.add((case,index))
assert keys=={(case,index) for case in (0,1)
              for index in range(len(inputs[case]['n4_candidates']))}

snap_path=here/'LMFDB_SNAPSHOT.json'
snapshot=json.loads(snap_path.read_text())
classes={-r['a5']:r for r in snapshot['rows']}
assert len(classes)==63 and set(classes)==set(range(-31,32))

def source_elliptic_possible(t):
    # Waterhouse trace criterion specialized to q=3^5; the mathematical
    # criterion is external, while this is its finite arithmetic filter.
    return t%3!=0 or t in (0,-27,27)

def factor_integer(n):
    out=[];d=2
    while d*d<=n:
        if n%d==0:
            k=0
            while n%d==0:n//=d;k+=1
            out.append([d,k])
        d+=1
    if n>1:out.append([n,1])
    return out

carriers=[r for r in rows if r['status']=='genus5_carrier']
by_trace=defaultdict(list)
for r in carriers:by_trace[r['trace_t']].append(r)
summary=[]
candidate_checks=[]
for t in sorted(by_trace):
    db=classes[t]
    rs=by_trace[t]
    representative=min(rs,key=lambda r:(len(r['sextic']),r['case'],r['index']))
    summary.append({'trace_t':t,'count':len(rs),
                    'LMFDB_label':db['label'],
                    'LMFDB_model_count':db['model_count'],
                    'LMFDB_has_jacobian_flag':db['has_jacobian'],
                    'source_elliptic_possible':source_elliptic_possible(t),
                    'group_order':244-t,'group_order_factorization':factor_integer(244-t),
                    'representative':{'case':representative['case'],
                                      'index':representative['index'],
                                      'sextic':representative['sextic']}})
    for r in rs:
        candidate_checks.append({'case':r['case'],'index':r['index'],
                                 'trace_t':t,'LMFDB_label':db['label'],
                                 'LMFDB_model_count':db['model_count'],
                                 'source_elliptic_possible':source_elliptic_possible(t),
                                 'exact_equation_publication_priority':'unresolved',
                                 'subgroup_transfer':'not_constructed',
                                 'full_GHS_vs_rho':'not_measured'})

eligible_unmodeled=[s for s in summary
                    if s['source_elliptic_possible'] and s['LMFDB_model_count']==0]
output={'schema':'ghs.quintic-node-all-traces-analysis/v1',
        'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        'classification_source_sha256':source_hash,
        'classification_sha256':hashlib.sha256((here/'FULL.jsonl').read_bytes()).hexdigest(),
        'LMFDB_snapshot_sha256':hashlib.sha256(snap_path.read_bytes()).hexdigest(),
        'input_sha256':input_hashes,
        'classified':len(rows),'status_counts':dict(sorted(Counter(r['status'] for r in rows).items())),
        'genus5_carriers':len(carriers),'represented_traces':len(summary),
        'elliptic_source_eligible_traces':sum(s['source_elliptic_possible'] for s in summary),
        'eligible_unmodeled_traces':[s['trace_t'] for s in eligible_unmodeled],
        'eligible_unmodeled_classes':len(eligible_unmodeled),
        'class_summary':summary,'candidate_prior_work_checks':candidate_checks}
print(json.dumps(output,sort_keys=True))
