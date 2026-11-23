#!/usr/local/bin/python3.13
"""Cube-and-conquer over b=9 CNFs: classes hi/lo/one/rnd, 3 bits -> 8 cubes, kissat --conflicts=CAP each. Resumable. usage: cube_driver.py [workers]"""
import json, os, re, subprocess, sys, random, itertools, glob, tempfile
from concurrent.futures import ThreadPoolExecutor
CAP=100000; S=3
HERE=os.path.dirname(os.path.abspath(__file__)); os.chdir(HERE)
OUT=os.environ.get('CUBE_OUT','raw/cube_results.jsonl')
SEEDS=[int(x) for x in os.environ.get('CUBE_SEEDS','3,4,5,8,10,13,16,21,24,25').split(',')]
done=set()
if os.path.exists(OUT):
    for l in open(OUT):
        d=json.loads(l); done.add((d['seed'],d['run'],d['cls'],d['cube']))
def kissat(cnf_text_path):
    r=subprocess.run(['kissat','--conflicts=%d'%CAP,'-n',cnf_text_path],capture_output=True,text=True)
    st='UNKNOWN'; c=pr=None
    for ln in r.stdout.splitlines():
        if ln.startswith('s SATISFIABLE'): st='SAT'
        elif ln.startswith('s UNSATISFIABLE'): st='UNSAT'
        m=re.match(r'c\s+conflicts:\s+(\d+)',ln)
        if m and c is None: c=int(m.group(1))
        m=re.match(r'c\s+propagations:\s+(\d+)',ln)
        if m and pr is None: pr=int(m.group(1))
    return st,c,pr
def classes(win,rng):
    x1,x2,x3=win
    allbits=[x1[i] for i in range(S)]+[x2[i] for i in range(S)]+[x3[i] for i in range(S)]
    return {'hi':[x1[S-1],x2[S-1],x3[S-1]],'lo':[x1[0],x2[0],x3[0]],'one':[x3[0],x3[1],x3[2]],'rnd':rng.sample(allbits,3)}
def work(job):
    seed,run,cls,vars_,cube,cnf=job
    lits=[v if (cube>>i)&1 else -v for i,v in enumerate(vars_)]
    txt=open(cnf).read().split('\n',1)
    h=txt[0].split(); nv,nc=int(h[2]),int(h[3])
    fd,path=tempfile.mkstemp(suffix='.cnf',dir=HERE+'/tmp'); os.close(fd)
    with open(path,'w') as f:
        f.write('p cnf %d %d\n'%(nv,nc+3)+txt[1]+('' if txt[1].endswith('\n') else '\n')+''.join('%d 0\n'%l for l in lits))
    st,c,pr=kissat(path); os.remove(path)
    return dict(seed=seed,run=run,cls=cls,cube=cube,vars=vars_,status=st,conflicts=c,propagations=pr)
W=int(sys.argv[1]) if len(sys.argv)>1 else 1
jobs=[]
for seed in SEEDS:
    for jf in sorted(glob.glob('cnf/s%d-r*.json'%seed)):
        j=json.load(open(jf)); run=j['run']; cnf=jf[:-5]+'.cnf'
        rng=random.Random(seed*1000+run)
        for cls,vs in classes((j['x1'],j['x2'],j['x3']),rng).items():
            assert all(abs(v)>1 for v in vs)
            for cube in range(8):
                if (seed,run,cls,cube) not in done: jobs.append((seed,run,cls,vs,cube,cnf))
print(len(jobs),'cube solves pending',flush=True)
with ThreadPoolExecutor(W) as ex, open(OUT,'a') as out:
    for r in ex.map(work,jobs):
        out.write(json.dumps(r)+'\n'); out.flush()
