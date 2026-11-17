import json,sys,statistics as st,os
pairs=[('f6_ic_ecbench_ladder_20261005','n13'),('f6_ic_ecbench_ladder_20261005','n19'),('f6_ic_ecbench_ladder_20261005','n23'),
 ('f4_f5_f6_cross_family_20261011','K13'),('f4_f5_f6_cross_family_20261011','R13'),('f4_f5_f6_cross_family_20261011','K19'),('f4_f5_f6_cross_family_20261011','R19')]
def load(p):
    d={}
    for l in open(p+'/records.jsonl'):
        r=json.loads(l)
        if r['warmup']: continue
        d[(r['arm'],r['workload']['target_index'],r['round'])]=r
    return d
out=[]
for study,s in pairs:
    a=f'/home/user/crypto/research/{study}/sessions/{s}'; b=f'/home/user/rerun/{study}-{s}'
    if not os.path.exists(b+'/records.jsonl'): continue
    A,B=load(a),load(b)
    arms=sorted({k[0] for k in A})
    for arm in arms:
        ks=[k for k in A if k[0]==arm and k in B]
        sa=[A[k]['cost']['s'] for k in ks if A[k]['cost'].get('s') is not None]
        sb=[B[k]['cost']['s'] for k in ks if B[k]['cost'].get('s') is not None]
        ops_eq=sum(1 for k in ks if {p['name']:p for p in A[k]['phases']}.get('relations',{}).get('native',{}).get('solver_ops')=={p['name']:p for p in B[k]['phases']}.get('relations',{}).get('native',{}).get('solver_ops'))
        lb=(sorted({A[k]['cost']['lower_bound'] for k in ks}),sorted({B[k]['cost']['lower_bound'] for k in ks}))
        unp=sorted({u for k in ks for u in B[k]['cost']['unpriced'] if 'solver' in u})
        row=dict(study=study,session=s,arm=arm,n=len(ks),mean_s_archived=st.mean(sa) if sa else None,mean_s_main=st.mean(sb) if sb else None,
                 solver_ops_equal=ops_eq,lower_bound_archived=lb[0],lower_bound_main=lb[1],main_solver_unpriced=unp)
        out.append(row)
        print(s,arm,len(ks),'%.3f -> %.3f'%(row['mean_s_archived'] or -1,row['mean_s_main'] or -1),'ops_eq',ops_eq,lb,unp)
json.dump(out,open('/home/user/rerun/compare.json','w'),indent=1)
