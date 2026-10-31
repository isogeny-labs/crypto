"""Read-only integrity/arithmetic audit of frozen metadata; no group operations."""
from pathlib import Path
import hashlib,json,math,collections
ROOT=Path(__file__).resolve().parents[6]
OUT=Path(__file__).resolve().parent
INDEX=ROOT/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/continuation-reconciliation/input-reconciliation.json'
def sha(b): return hashlib.sha256(b).hexdigest()
rows=[]; receipts=[]; seen=set(); groups={}
for entry in json.loads(INDEX.read_text())['rows']:
    data=Path(entry['path']).read_bytes()
    if sha(data)!=entry['sha256']: raise ValueError('input hash mismatch: '+entry['name'])
    plan=json.loads(data); workloads=plan['workloads']
    if len(workloads)!=entry['workloads']: raise ValueError('workload count mismatch')
    lane='separate_gate' if entry['name']=='gate-p186379-target0.json' else 'cohort'
    receipts.append({'name':entry['name'],'sha256':sha(data),'workloads':len(workloads),'lane':lane})
    for w in workloads:
        s=w['curve_spec']; c=w['curve']; p=s['p']; r=s['r']; n=s['group_order']
        assert all(type(v) is int and v>0 for v in (p,r,n))
        assert s['n']==2 and s['family']=='scholten'
        q=p*p; h,rem=divmod(n,r)
        if lane=='cohort':
            assert w['workload_id'] not in seen
            seen.add(w['workload_id'])
        identity={k:s[k] for k in ('p','n','modulus','a','b','group_order','r','family','parameter')}
        ident=sha(json.dumps(identity,sort_keys=True,separators=(',',':')).encode())
        row={'lane':lane,'input':entry['name'],'workload_id':w['workload_id'],'workload_sha256_recorded':w['workload_sha256'],'curve_metadata_sha256':ident,'p':p,'order':n,'r':r,'cofactor_derived':h,'order_divisible_by_r':rem==0,'supplied_cofactor_matches':c['cofactor']==h,'duplicate_curve_fields_match':c['group_order']==n and c['r']==r,'gcd_cofactor_r':math.gcd(h,r),'within_hasse_interval':(p-1)**2<=n<=(p+1)**2,'r_greater_hasse_width':r>4*p}
        rows.append(row)
        if lane=='cohort':
            if ident not in groups: groups[ident]={'curve_metadata_sha256':ident,**identity,'cofactor':h,'gcd_cofactor_r':math.gcd(h,r),'targets':0}
            groups[ident]['targets']+=1
assert len(seen)==1260
checks=('order_divisible_by_r','supplied_cofactor_matches','duplicate_curve_fields_match','within_hasse_interval','r_greater_hasse_width')
summary={'schema':'research.metadata-audit/v1','scope':'frozen input metadata only','cohort_workloads':len(seen),'cohort_curve_metadata_groups':len(groups),'separate_gate_workloads':sum(x['lane']=='separate_gate' for x in rows),'checks':{k:sum(not x[k] for x in rows) for k in checks},'noncoprime_rows':sum(x['gcd_cofactor_r']!=1 for x in rows),'curve_cardinality_recomputed':False,'prime_checks_run':False,'transfer_checks_run':False,'scientific_executions':0,'cost_ratio':None,'index_sha256':sha(INDEX.read_bytes()),'plan_sha256':sha((OUT/'PLAN.md').read_bytes())}
(OUT/'rows.jsonl').write_text(''.join(json.dumps(x,sort_keys=True)+'\n' for x in rows))
for name,obj in [('summary.json',summary),('input-receipts.json',receipts),('curves.json',list(groups.values()))]: (OUT/name).write_text(json.dumps(obj,indent=2)+'\n')
print(json.dumps(summary,indent=2))
