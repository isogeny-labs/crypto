import run,sys,json
n=int(sys.argv[1]); seeds=int(sys.argv[2]); cfgs=[tuple(map(int,c.split(','))) for c in sys.argv[3:]]
for K,norm in cfgs:
    c=[];ok=True;parts=[]
    for seed in range(1,seeds+1):
        d=run.run(n,K,norm,None,seed=seed); 
        if 'err' in d: print(K,norm,d['err']); ok=False; break
        t=d['timing_ms']; c.append(d['cpu_s']); a,b=run.ok(); ok&=(a==b); parts.append((t['index_build'],t['rank_stage'],t['setup_total'],t['process_total']))
    if c: print(n,'K',K,'norm',norm,'pts',d['factor_base_points'],'orb',d.get('orbit_representatives'),'cpu mean %.2f min %.2f'%(sum(c)/len(c),min(c)),'index %.0f rank %.0f setup %.0f total %.0f ms'%tuple(sum(p[i] for p in parts)/len(parts) for i in range(4)),'verified',ok,flush=True)
