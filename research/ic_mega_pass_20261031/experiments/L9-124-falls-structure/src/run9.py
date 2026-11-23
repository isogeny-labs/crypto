import sys, os, json, subprocess, time
from sys9 import *
HERE=os.path.dirname(os.path.abspath(__file__)); BIN=os.path.join(HERE,'macdump')
n,name,s,R=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]),int(sys.argv[4]); k=(n+1)//2
dd=os.environ.get('L9DUMP'); pref=os.path.join(dd,'n%d_%s_s%d'%(n,name,s)) if dd else None
eqs=variant(n,k,s,name)
inp="%d 4 %d\n"%(2*k,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
t=time.time(); r=subprocess.run([BIN,str(R)]+([pref] if pref else []),input=inp,capture_output=True,text=True)
o=json.loads(r.stdout); o.update(n=n,arm=name,s=s,maxround=R,wall=round(time.time()-t,1),log=r.stderr.strip().split('\n'))
print(json.dumps(o)); sys.stdout.flush()
