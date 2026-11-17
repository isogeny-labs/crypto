import sys, os, json, subprocess, time
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'))
import lib
BIN=os.path.join(HERE,'..','..','L9-124-falls-structure','src','macdump')
n,arm,s=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]); k=(n+1)//2
eqs=lib.system(n,k,s)[arm]
inp="%d 4 %d\n"%(2*k,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in sorted(e))+"\n" for e in eqs)
t=time.time(); r=subprocess.run([BIN,'3'],input=inp,capture_output=True,text=True)
o=json.loads(r.stdout); log=r.stderr.strip().split('\n')
low={int(l.split()[1]):int(l.split()[5]) for l in log if l.startswith('round')}
o.update(n=n,arm=arm,s=s,maxround=3,low_by_round=low,falls_r3=low.get(3,0)-low.get(2,0),wall=round(time.time()-t,1),log=log)
print(json.dumps(o),flush=True)
