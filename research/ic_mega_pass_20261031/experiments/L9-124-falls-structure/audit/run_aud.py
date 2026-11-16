import sys, os, json, subprocess, time
sys.dont_write_bytecode=True
E='/Users/adamburan/ic-work/crypto/research/ic_mega_pass_20261031/experiments/L9-124-falls-structure/src'
sys.path.insert(0,E)
from sys9 import *
n,name,s,R=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]),int(sys.argv[4]); k=(n+1)//2
pref=os.path.join(os.environ['DUMP'],'n%d_%s_s%d'%(n,name,s))
eqs=variant(n,k,s,name)
inp="%d 4 %d\n"%(2*k,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
t=time.time(); r=subprocess.run(['./macdump_aud',str(R),pref],input=inp,capture_output=True,text=True)
o=json.loads(r.stdout); o.update(n=n,arm=name,s=s,log=r.stderr.strip().split('\n')); print(json.dumps(o))
