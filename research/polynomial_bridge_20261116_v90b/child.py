import json,time,sys,resource,os,subprocess
from pathlib import Path
START=time.monotonic()
from .codec import encode,field,evaluate
from .binary import anf,decode
from .block import solve
p=Path(sys.argv[1]);d=json.loads(p.read_text());c=d['cases'][int(sys.argv[2])];method=sys.argv[3];name=sys.argv[4];g=encode(c);f=field(c);stats={};native=None
if method=='block':rows,stats=solve(c)
else:
 b=anf(g);inp=p.parent/(name+'_neutral.json')
 with inp.open('x')as w:json.dump(b,w,separators=(',',':'))
 env=dict(os.environ);env['RAYON_NUM_THREADS']='1';cmd=[d['baseline_binary'],'--algorithm',method,'--input',str(inp),'--max-degree',str(d['max_degree']),'--node-budget',str(d['nodes']),'--max-solutions',str(d['max_solutions'])];r=subprocess.run(cmd,capture_output=True,text=True,timeout=d['baseline_seconds'],env=env)
 native={'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr};v=json.loads(r.stdout);assert r.returncode==0 and v['complete']and v['verified']and v['algorithm']==method and v['source_revision']==d['baseline_revision'];rows=sorted(decode(int(x,16),f.n,len(g['variables']))for x in v['roots_hex']);stats=v['stats'];assert len(rows)==v['root_count']
assert len(rows)==len(set(map(tuple,rows)))
for row in rows:assert all(evaluate(e,row,f)==0 for e in g['equations'])
out={'complete':True,'roots':rows,'exact_root_count':len(rows),'stats':stats,'method':method,'native':native,'cpu_affinity':sorted(os.sched_getaffinity(0)),'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss};trial=json.dumps(out,separators=(',',':'))
if time.monotonic()-START>=7:out['complete']=False;out['exact_root_count']=None
print(json.dumps(out,separators=(',',':')))
