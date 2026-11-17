import json, sys, time
from l2lib import *
n=31; k=16
for arm,s in [tuple(x.split(':')) for x in sys.argv[1:]]:
    s=int(s); sy=system(n,k,s); t=time.time()
    x=call5('closure',2*k,5,sy[arm],cap=2400,progress_log=os.path.join(HERE,'..','raw','prog_d5_%s_s%d.log'%(arm,s)))
    x.update(n=n,k=k,arm=arm,s=s,d=5,wall=round(time.time()-t,1))
    print(json.dumps(x),flush=True)
