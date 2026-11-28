"""Reproducible, strict Semaev-only correctness gates and diagnostic GB runs.

Run: python -m summation_split.harness --out summation_split --samples 50
No Sage installed: uses SymPy lex Buchberger, never claims to measure F4.
"""
import argparse,csv,os,random,time,resource,multiprocessing as mp,sys
import sympy as sp
from .core import admissible,specialized_summation,eval_poly as evaluate
from .field import witness_roots
from .projective import projective_chain_roots
from .solver import direct_gb,chained_gb,symmetric_s4_gb,linear_gb
from .symmetric import terminal_s4

PARAMS=[(5,0,1,3),(7,1,1,3),(11,1,1,3),(11,1,1,4),(19,2,3,4),(19,2,3,5)]
STRATEGIES={'monolithic':direct_gb,'projective_chain':chained_gb,
            'linear':linear_gb,'symmetric_S4':symmetric_s4_gb}

def elementary(xs):
    e=[1]+[0]*len(xs)
    for i,x in enumerate(xs):
        for j in range(i+1,0,-1):e[j]+=x*e[j-1]
    return e[1:]

def gate(p,a,b,m,samples,seed):
    if not admissible(p,a,b):raise AssertionError('invalid curve')
    rr=random.Random(seed+p*10007+m*733)
    cases=[];solutions=0
    sym_fn = None
    if m == 3:
        fs,subs,_=terminal_s4(a,b)
        sym_fn=sp.lambdify([sy for sy,_ in subs],fs,"math")
    for case in range(samples):
        xs=tuple(rr.randrange(p) for _ in range(m))
        f=specialized_summation(p,a,b,xs)
        baseline={X for X in range(p) if evaluate(f,X,p)==0}
        chained=projective_chain_roots(p,a,b,xs)
        group=witness_roots(p,a,b,xs)
        if chained!=baseline or group!=baseline:
            raise AssertionError(f'gate fail cell {(p,a,b,m)} case={case} xs={xs} base={baseline} chain={chained} group={group}')
        if m==3:
            for X in range(p):
                lhs=int(sym_fn(*elementary((*xs,X))))%p
                rhs=evaluate(f,X,p)
                if lhs!=rhs:raise AssertionError(f'symmetric S4 mismatch {(p,a,b,m)} case={case}, X={X}')
        # Linear invertible differences reconstruct each tuple exactly.
        for X in range(p):
            seq=(*xs,X)
            diffs=[seq[j]-seq[j+1] for j in range(len(seq)-1)]+[seq[-1]]
            rec=[sum(diffs[i:]) for i in range(len(diffs))]
            if tuple(rec)!=seq:raise AssertionError('linear invertibility')
        # S_r symmetry includes the target coordinate; check a cyclic swap.
        j=case%(m+1)
        perm=(*xs,rr.randrange(p)); perm=perm[j:]+perm[:j]
        if evaluate(specialized_summation(p,a,b,perm[:-1]),perm[-1],p)!=evaluate(specialized_summation(p,a,b,perm[1:]),perm[0],p):
            raise AssertionError('symmetric permutation mismatch')
        solutions+=len(baseline)
        cases.append((xs,sorted(baseline)))
    return cases,solutions

def _worker(q,strategy,cell,xs):
    try:
        start=time.perf_counter()
        roots,basis_degree,basis_count=STRATEGIES[strategy](*cell[:3],xs)
        wall=time.perf_counter()-start
        peak=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        q.put(dict(status='finished',roots=sorted(roots),wall_s=wall,peak_rss_kib=peak,
                   gb_basis_max_degree=basis_degree,gb_basis_size_or_branches=basis_count))
    except BaseException as e:q.put(dict(status='error',reason=repr(e)))

def timed(strategy,cell,xs,timeout):
    if strategy=='symmetric_S4' and cell[-1]!=3:
        return dict(status='not_implemented',reason='full symmetric terminal S5 and above not derived')
    ctx=mp.get_context('fork');q=ctx.Queue(maxsize=1)
    proc=ctx.Process(target=_worker,args=(q,strategy,cell,xs));begin=time.perf_counter();proc.start()
    proc.join(timeout)
    if proc.is_alive():proc.terminate();proc.join(2);return dict(status='censored_timeout',wall_s=time.perf_counter()-begin)
    if q.empty():return dict(status='error',reason=f'no result, exit={proc.exitcode}')
    return q.get()

def fmt(v):return '' if v is None else v

def rows_for_cell(cell,cases,total,timeout):
    p,a,b,m=cell;numvars={'monolithic':1,'projective_chain':1+2*(m-2),
        'linear':2,'symmetric_S4':11 if m==3 else None}
    maxraw={'monolithic':2**(m-1),'projective_chain':4,
        'linear':2**(m-1),'symmetric_S4':4 if m==3 else None}
    ranked=sorted(cases,key=lambda c: not bool(c[1]))
    subset=ranked[:2]
    for sample_id,(xs,truth) in enumerate(subset):
        for strat in STRATEGIES:
            r=timed(strat,cell,xs,timeout)
            checked=(r.get('roots')==truth) if r['status']=='finished' else None
            if checked is False:raise AssertionError(f'GB wrong {strat} {cell} {xs} {r} vs {truth}')
            yield dict(p=p,a=a,b=b,m=m,strategy=strat,sample_id=sample_id,
                       fixed_xs=':'.join(map(str,xs)),gate_instances=len(cases),
                       gate_projected_solutions_total=total,
                       gate_chain_exact=True,gate_group_exact=True,
                       gate_symmetric_identity=(m==3),gate_linear_exact=True,
                       max_generator_degree_no_field_eq=maxraw[strat],
                       max_generator_degree_with_field_eq=max(p,maxraw[strat]) if maxraw[strat] is not None else None,
                       variables=numvars[strat],projected_solutions=len(truth),
                       gb_roots_exact=checked,backend='sympy_buchberger_lex',
                       first_fall_degree=None,d_F4=None,
                       **{k:r.get(k) for k in ('status','wall_s','peak_rss_kib','gb_basis_max_degree','gb_basis_size_or_branches','reason')})

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--samples',type=int,default=50)
    ap.add_argument('--seed',type=int,default=12042026)
    ap.add_argument('--timeout',type=float,default=5)
    ap.add_argument('--out',default='summation_split');args=ap.parse_args()
    if args.samples<50:raise SystemExit('at least 50 independent instances per parameter required before any timings')
    os.makedirs(args.out,exist_ok=True)
    gates=[];cases_by_cell={}
    for cell in PARAMS:
        cases,count=gate(*cell,args.samples,args.seed)
        print('gate',cell,len(cases),count,'PASS',flush=True)
        cases_by_cell[cell]=(cases,count)
        gates.append(dict(p=cell[0],a=cell[1],b=cell[2],m=cell[3],instances=len(cases),
                          projected_solution_total=count,chain=True,group=True,symmetric_S4=(cell[-1]==3),linear=True))
    rows=[]
    for cell,(cases,total) in cases_by_cell.items():
        for row in rows_for_cell(cell,cases,total,args.timeout):
            rows.append(row)
            print('GB',cell,row['strategy'],row['sample_id'],row['status'],row['wall_s'],flush=True)
    with open(os.path.join(args.out,'bench.csv'),'w',newline='') as fh:
        writer=csv.DictWriter(fh,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
    with open(os.path.join(args.out,'gate.csv'),'w',newline='') as fh:
        writer=csv.DictWriter(fh,fieldnames=list(gates[0]));writer.writeheader();writer.writerows(gates)
    svgplot(os.path.join(args.out,'solving_degree_vs_m.svg'))
    print('DONE',len(rows),'timing records; no measured F4/first-fall degrees')

def svgplot(path):
    points_mono=' '.join(f'{120+(m-3)*120},{280-(2**(m-1))*10}' for m in (3,4,5))
    points_split=' '.join(f'{120+(m-3)*120},{280-4*10}' for m in (3,4,5))
    svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="700" height="400" viewBox="0 0 700 400">
<rect width="700" height="400" fill="white"/><text x="30" y="29" font-family="sans-serif" font-size="18">Generator degree by m — NOT F4 solving degree</text>
<line x1="85" y1="300" x2="470" y2="300" stroke="#333"/><line x1="85" y1="65" x2="85" y2="300" stroke="#333"/>
<polyline points="{points_mono}" fill="none" stroke="#c25135" stroke-width="3"/><polyline points="{points_split}" fill="none" stroke="#26748a" stroke-width="3"/>
{''.join(f'<text x="{120+(m-3)*120-4}" y="318" font-size="12">{m}</text>' for m in (3,4,5))}
{''.join(f'<text x="57" y="{280-d*10+4}" font-size="12">{d}</text>' for d in (4,8,16))}
<text x="250" y="343" font-size="13">m (number of input summands)</text>
<line x1="490" y1="100" x2="515" y2="100" stroke="#c25135" stroke-width="3"/><text x="520" y="105" font-size="12">S_(m+1) univariate X</text>
<line x1="490" y1="135" x2="515" y2="135" stroke="#26748a" stroke-width="3"/><text x="520" y="139" font-size="12">S3 chart equations</text>
<text x="490" y="200" font-family="sans-serif" font-size="15" fill="#8b2510">d_F4: unmeasured</text>
<text x="490" y="224" font-family="sans-serif" font-size="13">First-fall degree: unmeasured</text>
<text x="490" y="250" font-family="sans-serif" font-size="12">Field equations degree p</text>
<text x="490" y="265" font-family="sans-serif" font-size="12">when included in GB</text>
</svg>'''
    with open(path,'w') as fh:fh.write(svg)

if __name__=='__main__':main()
