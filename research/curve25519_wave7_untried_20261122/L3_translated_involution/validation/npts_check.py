import numpy as np, math, json
from vcore import FAM
ROOT='/home/user/crypto/research/curve25519_wave7_untried_20261122/L3_translated_involution'
ARMS=['E.x','E.x_noself','E.iota_even','E.iota_T0','E.iota_odd','Ep.x','Ep.iota_2E','Ep.iota_c01','Ep.iota_c10','Ep.iota_c11','Ep.huff_x','Ep.huff_y','Ep.ji_s','Ep.ji_c','Ep.ji_d']
zs=[]; out=[]
for k in (16,18,20):
    p,A,n=FAM[k]
    for arm in ARMS:
        z=np.load(f'{ROOT}/data/k{k}/{arm}.npz')
        for t in (4,5,6,7,8):
            a=z[f't{t}_n_points'].astype(float)
            # finite-valued non-O points: x-frames n-1; iota arms n-2 (poles O,T -> T excluded); huff n-3; JI: ~n-1-#poles
            c={'E.x':n-1,'E.x_noself':n-1,'Ep.x':n-1}.get(arm, n-2 if 'iota' in arm else (n-3 if 'huff' in arm else n-1))
            mu=(2**t)*c/(p+1)
            zz=(a.mean()-mu)/(a.std(ddof=1)/math.sqrt(a.size))
            zs.append(zz); out.append((k,arm,t,round(a.mean(),3),round(mu,3),round(zz,2)))
zs=np.array(zs)
print('n cells',len(zs),'mean z',zs.mean().round(3),'sd z',zs.std().round(3),'|z|>2:',int((abs(zs)>2).sum()),'|z|>3:',int((abs(zs)>3).sum()))
for o in sorted(out,key=lambda o:-abs(o[-1]))[:8]: print(o)
