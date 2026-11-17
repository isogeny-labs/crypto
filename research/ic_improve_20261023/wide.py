import os
import run,sys,json
n=int(sys.argv[1]); K=int(sys.argv[2]); norm=int(sys.argv[3])
d=run.run(n,K,norm,None,seed=1,tg=os.environ.get('TG','tg71.txt'),extra={})
if 'err' in d: print(d['err']); sys.exit()
t=d['timing_ms']; a,b=run.ok()
print(n,'K',K,'norm',norm,'pts',d['factor_base_points'],'orb',d.get('orbit_representatives'),'cpu %.1f'%d['cpu_s'],{k:round(v,0) for k,v in t.items() if isinstance(v,(int,float))},'verified',a,b,flush=True)
