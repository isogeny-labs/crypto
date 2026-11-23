import sys, json, time
from drv import *
n,arm,s=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]); k=(n+1)//2
sy=lib.system(n,k,s); t=time.time(); m=mac(sy[arm],2*k,4)
m.update(n=n,k=k,arm=arm,s=s,wall=round(time.time()-t,1))
open(os.path.join(HERE,'..','raw','n%d_%s_s%d.json'%(n,arm,s)),'w').write(json.dumps(m,indent=1)); print(json.dumps({x:m[x] for x in m if x!='log'}))
