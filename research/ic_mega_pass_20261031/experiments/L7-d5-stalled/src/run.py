import json, sys, time, os
from l2lib import *
n,arm,s,cap=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]),int(sys.argv[4]); k=(n+1)//2
sy=system(n,k,s); t=time.time()
x=call5('closure',2*k,5,sy[arm],cap=cap,progress_log=os.path.join(HERE,'..','raw','prog_n%d_%s_s%d.log'%(n,arm,s)))
x.update(n=n,k=k,arm=arm,s=s,d=5,wall=round(time.time()-t,1))
print(json.dumps(x),flush=True)
