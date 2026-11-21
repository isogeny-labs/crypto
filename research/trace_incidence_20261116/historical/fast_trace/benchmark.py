"""Fresh-process, CPU-pinned complete-translation benchmark.

All input divisor construction is fixture generation, outside both methods:
the algorithm input in the paper is (u,v), not a list of its roots. Every
timed call includes input conversion, checks, maps, image recovery, collision
lifting and exceptional support points. Native helper compilation is a build
step, done before runs. Each worker loads it before the timed algorithm call.
Fresh-process user+system CPU, wall and absolute peak RSS are also retained.
This is a shared container with pinned cores, not a dedicated-host claim.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import random
import resource
import statistics
import subprocess
import sys
import time

HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent))


def make_case(p,a,b,d,kind,seed):
    from fast_trace.kernel import point_add
    from paper_trace_kernel import product_roots,interpolate
    rng=random.Random(seed)
    def lift(x):
        value=(x*x*x+a*x+b)%p
        if value==0:return x,0
        if pow(value,(p-1)//2,p)!=1:return None
        # Fixture generation uses sympy only here; never in either kernel.
        from sympy.ntheory.residue_ntheory import sqrt_mod
        return x,int(sqrt_mod(value,p))
    Q=next(P for x in range(p) if (P:=lift(x)) is not None and P[1]!=0)
    def neg(P):return None if P is None else (P[0],-P[1]%p)
    twoQ=point_add(p,a,Q,Q);minus2Q=neg(twoQ)
    selected=[];seen=set();images=set()
    if kind.startswith('exception'):
        selected.append(Q if kind=='exception_plus' else neg(Q));seen.add(Q[0])
    while len(selected)<d:
        x=rng.randrange(p);P=lift(x)
        if P is None or x in seen or x==Q[0]:continue
        if kind=='collision':
            PP=point_add(p,a,minus2Q,neg(P))
            if PP is None or PP[0] in seen or PP[0] in (P[0],Q[0]):continue
            selected.extend([P,PP]);seen.update((P[0],PP[0]))
        else:
            image=point_add(p,a,P,Q)
            if image is None or image[0] in images:continue
            selected.append(P);seen.add(P[0]);images.add(image[0])
    xs=[P[0]for P in selected];ys=[P[1]for P in selected]
    u=product_roots(xs,p);v=interpolate(xs,ys,p)
    input_infinity=kind.startswith('exception')
    points=[point_add(p,a,P,Q)for P in selected]+([Q]if input_infinity else [])
    return {'id':f'p{p}-d{d}-{kind}','p':p,'a':a,'b':b,'u':u,'v':v,'Q':list(Q),
            'input_infinity':input_infinity,'kind':kind,'degree':d,
            'expected_points':[list(P)if P is not None else None for P in points]}


def worker(path,method):
    os.sched_setaffinity(0,{0})
    from fast_trace.kernel import translate,initialize
    initialize()
    case=json.loads(Path(path).read_text())
    cpu=time.process_time();wall=time.perf_counter()
    answer=translate(**{k:case[k]for k in ('p','a','b','u','v','Q','input_infinity')},method=method)
    elapsed=time.process_time()-cpu;duration=time.perf_counter()-wall
    # Validation is after the complete timing, retained separately.
    validation_start=time.process_time()
    p=case['p'];actual=[]
    expected=case['expected_points'];xs={P[0] for P in expected if P is not None}
    from fast_trace.kernel import Poly
    for component in answer['components']:
        u=Poly(component['u'],p);v=Poly(component['v'],p)
        roots=[x for x in xs if int(u(x))==0]
        if len(roots)!=u.degree():raise RuntimeError('component degree/roots mismatch')
        actual.extend((x,int(v(x)))for x in roots)
    if answer['infinity']:actual.append(None)
    key=lambda P:(-1,-1)if P is None else tuple(P)
    if sorted(actual,key=key)!=sorted([None if P is None else tuple(P)for P in expected],key=key):
        raise RuntimeError('translated point multiset mismatch')
    result={'case':case['id'],'method':method,'complete_call_cpu_seconds':elapsed,
            'complete_call_wall_seconds':duration,'stats':answer['stats'],'verified':True,
            'validation_cpu_seconds':time.process_time()-validation_start,
            'fresh_process_peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
            'affinity':sorted(os.sched_getaffinity(0))}
    print(json.dumps(result))


def main():
    p=argparse.ArgumentParser();p.add_argument('--worker');p.add_argument('--method')
    p.add_argument('--max-degree',type=int,default=256);p.add_argument('--repeats',type=int,default=3)
    args=p.parse_args()
    if args.worker:worker(args.worker,args.method);return
    from fast_trace.kernel import initialize,flint
    initialize()
    out=HERE/'results';out.mkdir(exist_ok=True);fixtures=out/'fixtures';fixtures.mkdir(exist_ok=True)
    cases=[]
    for prime in (1000003,1000033):
        for d in (8,16,32,64,128,256):
            if d>args.max_degree:continue
            for kind in ('generic','collision','exception_plus','exception_minus'):
                case=make_case(prime,2,3,d,kind,20261116+d+prime)
                path=fixtures/(case['id']+'.json');path.write_text(json.dumps(case)+'\n');cases.append((case,path))
    rows=[];failures=[]
    for rep in range(args.repeats):
        for number,(case,path)in enumerate(cases):
            methods=('trace','dense')if (number+rep)%2 else ('dense','trace')
            for method in methods:
                before=resource.getrusage(resource.RUSAGE_CHILDREN)
                start=time.perf_counter()
                env=dict(os.environ,OMP_NUM_THREADS='1',OPENBLAS_NUM_THREADS='1',MKL_NUM_THREADS='1',PYTHONPATH=str(HERE.parent))
                command=[sys.executable,str(Path(__file__)), '--worker',str(path),'--method',method]
                try:
                    cp=subprocess.run(command,text=True,capture_output=True,env=env,timeout=90)
                    after=resource.getrusage(resource.RUSAGE_CHILDREN)
                    if cp.returncode:raise RuntimeError(cp.stderr)
                    row=json.loads(cp.stdout);row.update({'repeat':rep,'fresh_child_user_system_cpu_seconds':after.ru_utime+after.ru_stime-before.ru_utime-before.ru_stime,'fresh_child_wall_seconds':time.perf_counter()-start})
                    rows.append(row)
                except Exception as error:
                    failures.append({'case':case['id'],'method':method,'repeat':rep,'error':repr(error)})
                (out/'raw.json').write_text(json.dumps({'rows':rows,'failures':failures},indent=2)+'\n')
            print(json.dumps({'repeat':rep,'case':case['id'],'completed_rows':len(rows),'failures':len(failures)}),flush=True)
    summary=[]
    for case,_ in cases:
        item={'case':case['id'],'p':case['p'],'degree':case['degree'],'kind':case['kind']}
        for method in ('trace','dense'):
            values=[r for r in rows if r['case']==case['id']and r['method']==method]
            item[method]={'samples':len(values),'median_cpu_seconds':statistics.median(r['complete_call_cpu_seconds']for r in values)if values else None,'max_fresh_process_peak_rss_kib':max((r['fresh_process_peak_rss_kib']for r in values),default=None)}
        if item['trace']['median_cpu_seconds']and item['dense']['median_cpu_seconds']:
            item['dense_over_trace']=item['dense']['median_cpu_seconds']/item['trace']['median_cpu_seconds']
        summary.append(item)
    metadata={'platform':sys.platform,'python':sys.version,'python_flint':flint.__version__,'affinity':[0],'flint_threads':1,'repeats':args.repeats,'source_sha256':{f.name:hashlib.sha256(f.read_bytes()).hexdigest()for f in (HERE/'kernel.py',HERE/'recurrence.cpp',Path(__file__))},'scope':'Pinned CPU in shared container. Complete algorithm calls; per-fresh-process RSS includes interpreter/native libraries. Fixture generation and native-helper compilation are disclosed build/input preparation, excluded from both translation methods. Trace BM remains quadratic; no optimal modular-composition or ECDLP complexity claim.'}
    (out/'summary.json').write_text(json.dumps({'metadata':metadata,'summary':summary,'failures':failures},indent=2)+'\n')
    print(json.dumps({'rows':len(rows),'failures':len(failures),'summary':str(out/'summary.json')}))


if __name__=='__main__':main()
