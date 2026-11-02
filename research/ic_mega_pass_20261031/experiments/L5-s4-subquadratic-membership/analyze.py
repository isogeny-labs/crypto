import json, collections, math
rows=[json.loads(l) for l in __import__('gzip').open('raw/grid.jsonl.gz','rt')]
I2M=3
g=collections.defaultdict(list)
for r in rows: g[(r['p'],r['n'],r['mode'])].append(r)
out=[]; name={0:'RAND/A',1:'TCL/Q_T',2:'E2CL/Q_E2'}
for (p,n,m),rs in sorted(g.items()):
    a=lambda k: sum(r[k] for r in rs)/len(rs)
    nX3=a('nX3'); S=a('S_pairtable'); npts_x=p/2  # ~#x-values of points
    hit=a('hits_random')/rs[0]['targets']
    onA=a('onA_M_per')+I2M*a('onA_I_per'); onB=a('onB_M_per')+I2M*a('onB_I_per')
    build=a('build_M')+I2M*a('build_I')
    D_lookups = 2*n  # x(R+-P3) per shift * shifts = 2n for all modes
    out.append(dict(p=p,n=n,arm=name[m],nX3=nX3,nX3_over_n3=nX3/n**3,S=S,D=D_lookups,SD_over_X3=S*D_lookups/nX3,
      hit_rate_random=hit, hit_pred=nX3/npts_x, build_Mequiv=build, onlineA_Mequiv_per_test=onA,
      onlineB_Mequiv_per_test=onB if m==0 else None, B_over_A=onB/onA if m==0 else None,
      online_per_relation=onA/(nX3/npts_x),
      mism_A=sum(r['mismatch_A'] for r in rs), mism_B=sum(r['mismatch_B'] for r in rs),
      pos=sum(r['positives'] for r in rs), verified=sum(r['verified'] for r in rs)))
for o in out:
    base=[x for x in out if x['p']==o['p'] and x['n']==o['n'] and x['arm']=='RAND/A'][0]
    o['X3_ratio_vs_RAND']=o['nX3']/base['nX3']; o['per_relation_ratio_vs_RAND']=o['online_per_relation']/base['online_per_relation']
    o['table_ratio_vs_RAND']=o['S']/base['S']
json.dump(out,open('RESULTS_grid.json','w'),indent=1)
print("%-10s %4s %-10s %9s %6s %8s %6s %7s %7s %8s %7s %7s %5s/%s"%('p','n','arm','|X3|','X3/n3','S','SD/X3','X3rat','tblrat','onA/test','B/A','rel_rat','mis','ver'))
for o in out: print("%-10d %4d %-10s %9d %6.3f %8d %6.2f %7.3f %7.3f %8.0f %7s %7.2f %5d %d/%d"%(o['p'],o['n'],o['arm'],o['nX3'],o['nX3_over_n3'],o['S'],o['SD_over_X3'],o['X3_ratio_vs_RAND'],o['table_ratio_vs_RAND'],o['onlineA_Mequiv_per_test'],'%.0f'%o['B_over_A'] if o['B_over_A'] else '-',o['per_relation_ratio_vs_RAND'],o['mism_A']+o['mism_B'],o['verified'],o['pos']))
