import re,glob,sys,collections
import numpy as np
rows=[]
for f in sorted(glob.glob('raw/m*_f*.txt')):
    for l in open(f):
        if not l.startswith('INST'): continue
        d=dict(re.findall(r'(\w+)=(\S+)',l))
        d['sat']=' SAT ' in l; d['m']=int(d['m']);d['s']=int(d['s']);d['cap']=int(d['cap']);d['filt']=int(d['filt'])
        d['b']=int(d['bstar']);d['ub']=int(d['ub']);d['curve']=int(d['curve']);d['sb']=int(d['sound_bad'])
        d['prof']=re.search(r'prof=(\S+)',l).group(1); rows.append(d)
tt={2.5:4.303,3:3.182,4:2.776,5:2.571,6:2.447,7:2.365,8:2.306,9:2.262,10:2.228,11:2.201,12:2.179}
cells=collections.defaultdict(list)
for d in rows: cells[(d['m'],d['cap'],d['filt'],d['s'])].append(d)
print("m cap filt s | n  ub_ok  sound_bad | B* min/med/max | offset (m-1)s-B*: min/med/max | per-curve mean offset | SAT-med UNSAT-med")
for k in sorted(cells):
    c=cells[k]; m,cap,fl,s=k; B=np.array([d['b'] for d in c]); off=(m-1)*s-B
    pc=[np.mean([d['b'] for d in c if d['curve']==q]) for q in sorted(set(d['curve'] for d in c))]
    ubok=sum(d['ub']>=0 for d in c); sb=sum(d['sb'] for d in c)
    bs=np.median([d['b'] for d in c if d['sat']]);bu=np.median([d['b'] for d in c if not d['sat']])
    print(m,cap,fl,s,'|',len(c),ubok,sb,'|',B.min(),np.median(B),B.max(),'|',off.min(),np.median(off),off.max(),'|',[round((m-1)*s-x,2) for x in pc],'|',bs,bu)
print("\nslope of per-curve mean B* vs s (OLS, 95% t-CI, n=3 curves x cells)")
for m in (3,4):
  for cap in (2,4,8):
    for fl in (0,1):
      xs=[];ys=[]
      for k,c in cells.items():
        if k[0]==m and k[1]==cap and k[2]==fl:
          for q in sorted(set(d['curve'] for d in c)): xs.append(k[3]); ys.append(np.mean([d['b'] for d in c if d['curve']==q]))
      if len(set(xs))<2: continue
      xs=np.array(xs,float);ys=np.array(ys);n=len(xs)
      A=np.vstack([xs,np.ones(n)]).T; co,res,_,_=np.linalg.lstsq(A,ys,rcond=None)
      r=ys-A@co; se=np.sqrt((r@r)/(n-2)/((xs-xs.mean())**2).sum()); t=tt.get(n-2,2.0 if n-2>12 else 2.2)
      print(f"m={m} cap={cap} filt={fl}: slope={co[0]:.3f} +- {t*se:.3f} (CI [{co[0]-t*se:.3f},{co[0]+t*se:.3f}]), intercept={co[1]:.2f}, n={n}; m-1={m-1}")
print("\ncommon profiles at B* (cap=4,filt=1): first/top profile string per cell, representative instance")
seen=set()
for d in rows:
    k=(d['m'],d['cap'],d['filt'],d['s'])
    if d['cap']==4 and d['filt']==1 and k not in seen: seen.add(k); print(k,d['prof'])
