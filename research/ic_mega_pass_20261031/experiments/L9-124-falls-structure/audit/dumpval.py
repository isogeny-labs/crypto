import sys, os, json, subprocess; sys.dont_write_bytecode=True
sys.path.insert(0,'/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L9-124-falls-structure/src')
from sys9 import *
import numpy as np
from aud_check import load, rank_basis
os.makedirs('../dumpv',exist_ok=True)
tot=0;bad=0
for n in (11,13,15):
  k=(n+1)//2; N=2*k
  for name in ('A','C1','A_V1V2'):
    for s in range(4):
      eqs=variant(n,k,s,name)
      inp="%d 4 %d\n"%(N,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
      pref='../dumpv/v_%d_%s_%d'%(n,name,s)
      r1=json.loads(subprocess.run(['./macdump_aud','99',pref],input=inp,capture_output=True,text=True).stdout)
      r2=json.loads(subprocess.run(['./macclose_aud'],input=inp,capture_output=True,text=True).stdout)
      same=all(r1[x]==r2[x] for x in ('dim','low','one'))
      # solutions by brute force
      pts=np.arange(1<<N,dtype=np.uint64); ok=np.ones(1<<N,bool)
      for e in eqs:
        v=np.zeros(1<<N,bool)
        for mm in e: v^=((pts&np.uint64(mm))==np.uint64(mm))
        ok&=~v
      sols=[int(x) for x in pts[ok]]
      rounds=r1['rounds']; vanish=True; nrows=0
      if not r1['one']:
        monos,rows=load(pref,rounds); nrows=len(rows)
        for row in rows:
          for x in sols:
            par=0; rr=row
            while rr:
              lb=rr&-rr; mm=monos[lb.bit_length()-1]
              par^= ((x&mm)==mm); rr^=lb
            if par: vanish=False
        lowok = nrows==r1['low']
      else:
        lowok=True
      tot+=1; bad+= (not same) or (not vanish) or (not lowok)
      print(n,name,s,'same_as_macclose',same,'nsol',len(sols),'rows',nrows,'vanish_on_sols',vanish,'one',r1['one'])
print('cells',tot,'bad',bad)
