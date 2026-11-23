#!/usr/bin/env python3
"""R4: distinct pair-sum keys vs identical seeded half-table, toy curve only."""
import argparse, json, math, random, hashlib, time
import run as base

def candidate(g, reference):
    seed=reference['seed']; n=reference['B']; scalars=reference['factor_base_scalars']
    fbm=base.Counter(); fb_start=time.perf_counter_ns();fb=[base.mul(s,g,fbm) for s in scalars];fb_ns=time.perf_counter_ns()-fb_start
    assert fb == reference['factor_base_points']
    k=reference['target'];q=base.mul(k,g,base.Counter())
    indices=[(i,j) for i in range(n) for j in range(i,n)]
    random.Random(seed^0x65f3).shuffle(indices)
    M=math.ceil(0.5*len(indices));tabc=base.Counter();table={};tested=0
    t=time.perf_counter_ns()
    for i,j in indices:
        tested+=1
        p=base.add(fb[i],fb[j],tabc)
        if p not in table:table[p]=(i,j)
        if len(table)==M:break
    tab_ns=time.perf_counter_ns()-t
    if len(table)<M:
        raise RuntimeError(f'unable to reach {M} distinct keys with {len(indices)} candidates')
    walk_c=base.Counter();t=time.perf_counter_ns()
    a=reference['start'];walk=base.mul(a,g,walk_c)
    rnk=base.Rank(n);rows=[];attempts=0;dependent=0;seen_rows=set()
    while rnk.free and attempts<base.R:
        ij=table.get(walk)
        if ij is not None:
            i,j=ij;key=(i,j,a)
            if key not in seen_rows:
                seen_rows.add(key);before=n-rnk.free;after=rnk.update(i,j)
                dependent+=int(before==after);rows.append((i,j,a))
        attempts+=1
        if rnk.free:
            walk=base.add(walk,g,walk_c);a=(a+1)%base.R
    coll_ns=time.perf_counter_ns()-t
    lc=base.Counter();dc=base.Counter();build_ns=la_ns=desc_ns=0
    desc_trials=0;pair=None;recovered=None;err=None
    if rnk.free==0:
        try:
            logs,build_ns,la_ns=base.graph_solve(rows,n,lc)
            if logs!=scalars: raise ValueError('factor base log mismatch')
            t=time.perf_counter_ns();p=q
            for d in range(base.R):
                desc_trials+=1
                ij=table.get(p)
                if ij is not None:
                    i,j=ij;pair=[i,j];recovered=(logs[i]+logs[j]-d)%base.R
                    if recovered!=k or base.mul(recovered,g,base.Counter())!=q:
                        raise ValueError('target recovery failed')
                    break
                if d+1<base.R:p=base.add(p,g,dc)
            desc_ns=time.perf_counter_ns()-t
            if recovered is None:err='descent exhausted'
        except Exception as e: err=repr(e)
    else:err=f'no full rank: {n-rnk.free}/{n}'
    group=fbm.g+tabc.g+walk_c.g+dc.g
    return {'status':'ok' if err is None else 'censored','error':err,
      'fraction':.50,'target_distinct_keys':M,'pair_entries':tested,'tested_pairs':tested,
      'pair_collisions':tested-len(table),'distinct_pair_sums':len(table),
      'rank':n-rnk.free,'rows':rows,'rows_sha256':hashlib.sha256(json.dumps(rows).encode()).hexdigest(),
      'relation_attempts':attempts,'relations':len(rows),'dependent_rows':dependent,
      'descent_trials':desc_trials,'descent_pair':pair,'recovered_log':recovered,
      'phases':{'factor_base':{'group_add':fbm.g,'wall_ns':fb_ns},
                'pair_table':{'group_add':tabc.g,'wall_ns':tab_ns,'tested_pairs':tested,'collisions':tested-len(table)},
                'collection':{'group_add':walk_c.g,'wall_ns':coll_ns},
                'matrix':{'wall_ns':build_ns,'rows':len(rows),'columns':n},
                'linear_algebra':{'mod_ops':lc.field(),'wall_ns':la_ns,**lc.dict()},
                'descent':{'group_add':dc.g,'wall_ns':desc_ns}},
      'total_group_add':group,'total_proxy_w01':group+0.1*lc.field(),'total_proxy_w1':group+lc.field()}

def main():
    p=argparse.ArgumentParser();p.add_argument('--base',type=int,required=True)
    p.add_argument('--seeds',type=int,required=True);p.add_argument('--seed0',type=lambda s:int(s,0),required=True)
    p.add_argument('--out',required=True);a=p.parse_args()
    assert a.base in [24,32,40] and a.seeds<=24
    g=base.generator();start_ns=time.perf_counter_ns();records=[]
    for idx in range(a.seeds):
        seed=a.seed0+idx
        try:
            old=base.trial(g,seed,a.base)
            old['arms']={k:old['arms'][k] for k in ['full','0.5','aa_full']}
            old['arms']['0.5_unique']=candidate(g,old)
            x=old['arms']['full'];y=old['arms']['aa_full']
            assert all(x[k]==y[k] for k in ['status','rank','rows_sha256','relation_attempts','total_group_add'])
            records.append(old)
        except Exception as e:records.append({'seed':seed,'status':'failed','error':repr(e)})
    out={'protocol':'EXP-IC-PAIRKEY-20261118-R4','curve':{'p':base.P,'a':base.A,'b':base.B,'r':base.R,'G':g},
      'base':a.base,'seed0':a.seed0,'seeds':a.seeds,'total_python_ns':time.perf_counter_ns()-start_ns,
      'records':records}
    with open(a.out,'w') as f:json.dump(out,f,sort_keys=True,indent=2)
    print(json.dumps({'out':a.out,'cases':len(records),'failures':[x for x in records if 'error' in x],
      'censored':[{'seed':r['seed'],'arm':name} for r in records if 'arms' in r for name,arm in r['arms'].items() if arm['status']!='ok']}))
if __name__=='__main__':main()
