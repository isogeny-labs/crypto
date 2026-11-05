import sys, json, subprocess, os, time
from gen import draw
BIN=os.environ['TMPDIR']+'/closure'
kind,p,t,s0,n,cmax,tl=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5]),int(sys.argv[6]),float(sys.argv[7])
for s in range(s0,s0+n):
    d=draw(kind,p,t,s)
    if d is None: continue
    if d['grid_zeros']>0 and kind not in ('planted',):
        d['excluded']='decomposable'; print(json.dumps(d),flush=True); continue
    co=' '.join(str(x) for r in d['coeffs'] for x in r)
    t0=time.time()
    out=subprocess.run([BIN,str(p),str(t),str(cmax),str(tl)],input=co,capture_output=True,text=True).stdout
    d['closure_log']=out.strip().split('\n'); d['result']=d['closure_log'][-1]; d['wall']=round(time.time()-t0,2)
    print(json.dumps(d),flush=True)
