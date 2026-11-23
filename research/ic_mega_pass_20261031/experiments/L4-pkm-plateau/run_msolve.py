import sys,json,subprocess,os,re,time,gzip
from gen import draw
from mk_ms import ms
kind,p,t,s0,n,tl=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5]),int(sys.argv[6])
T=os.environ['TMPDIR']; os.makedirs('raw/msolve_logs',exist_ok=True)
for s in range(s0,s0+n):
    d=draw(kind,p,t,s)
    if d is None: continue
    if d['grid_zeros']>0 and kind!='planted': d['excluded']='decomposable'; print(json.dumps(d),flush=True); continue
    f=f"{T}/ms_{kind}_{p}_{t}_{s}"; open(f+'.ms','w').write(ms(d))
    t0=time.time()
    try:
        r=subprocess.run(['msolve','-f',f+'.ms','-o',f+'.out','-t','1','-v','2','-g','2'],capture_output=True,text=True,timeout=tl); out=r.stdout; to=False
    except subprocess.TimeoutExpired as e: out=(e.stdout or b'').decode() if isinstance(e.stdout,bytes) else (e.stdout or ''); to=True
    degs=[int(m.group(1)) for m in re.finditer(r'^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+',out,re.M)]
    d['D_msolve']=max(degs) if degs else None; d['timeout']=to; d['wall']=round(time.time()-t0,2)
    d['no_solution']=('No solution' in out) or ('single element' in out)
    m=re.search(r'max\. matrix data\s+(\d+ x \d+)',out); d['max_matrix']=m.group(1) if m else None
    gzip.open(f"raw/msolve_logs/{kind}_{p}_t{t}_s{s}.log.gz",'wt').write(out)
    for e in ('.ms','.out'):
        try: os.remove(f+e)
        except: pass
    print(json.dumps(d),flush=True)
