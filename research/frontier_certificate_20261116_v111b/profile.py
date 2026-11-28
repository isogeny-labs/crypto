import json,time,random,itertools,collections,hashlib
from pathlib import Path
from .common import P,finish
from .kernel import solve,guarded_charts
from research.residue_quotient_20261116_v99b.kernel import apply,basis
from research.syndrome_frontier_20261116_v111.kernel import Widths,quotient_map
from research.guarded_local_search_20261116_v101.kernel import matrix,decode,valid_guard
START=time.monotonic();out={'original_complete':False,'toy_cases':0,'toy_assignments':0};error=None
try:
    d=json.loads((P/'DATA.json').read_text());rng=random.Random(d['seed'])
    for case in range(64):
        n=5+case%3;rows=[(rng.randrange(1<<n),rng.randrange(2))for _ in range(case%4)]
        flags=[(rng.randrange(1<<n),rng.randrange(2))for _ in range(3)]+[(1<<(n-2),0),(1<<(n-1),0)]
        charts=guarded_charts(rows,n,*flags);actual=set()
        for chart in charts:
            if not chart['feasible']:continue
            points={chart['particular']^apply(chart['columns'],z)for z in range(1<<chart['dimension'])}
            assert not(actual&points);actual|=points
        literal=set()
        for theta in range(1<<n):
            out['toy_assignments']+=1
            if any((mask&theta).bit_count()%2!=rhs for mask,rhs in rows):continue
            u,v,w,q1,q2=[b^((mask&theta).bit_count()%2)for mask,b in flags]
            if (u,v,w)in[(0,0,0),(1,1,0),(0,0,1)]and q2==w and(w or not q1):literal.add(theta)
        assert actual==literal;out['toy_cases']+=1
    code=json.loads(Path(d['code']).read_text());obs=json.loads(Path(d['observations']).read_text())
    assert len(code['blocks'])==14
    W=Widths(code['blocks'],d['rank_cap']);hist=collections.Counter();best=None;visited=0
    for tail in itertools.combinations(range(1,14),6):
        mask=1
        for i in tail:mask|=1<<i
        width=W.cut(mask);hist[width]+=1;visited+=1
        if best is None or(width,mask)<tuple(best):best=[width,mask]
    assert visited==1716
    equations=[];offset=0;local_dimensions=[]
    for width,h in zip(code['widths'],code['hulls']):
        piv,K,free,q=quotient_map(h['columns'],width)
        columns=[q((v>>offset)&((1<<width)-1))for v in obs['columns']]
        rhs=q(((obs['affine']>>offset)&((1<<width)-1))^h['anchor'])
        for j in range(len(free)):equations.append((sum(((v>>j)&1)<<i for i,v in enumerate(columns)),(rhs>>j)&1))
        offset+=width
    linear=solve(equations,76);assert linear['feasible'];assert linear['dimension']==sum(len(h['columns'])for h in code['hulls'])-code['syndrome_rank']+len(code['observation_kernel'])
    cfg=json.loads(Path(d['config']).read_text());rows=matrix(cfg['matrix'])
    def flag(i,bit):return sum(((v[i]>>bit)&1)<<j for j,v in enumerate(rows[1:])),(rows[0][i]>>bit)&1
    charts=guarded_charts(equations,76,flag(3,34),flag(2,51),flag(1,68),(1<<74,0),(1<<75,0))
    controls=0
    for chart in charts:
        if not chart['feasible']:continue
        for _ in range(64):
            theta=chart['particular']^apply(chart['columns'],rng.randrange(1<<chart['dimension']))
            assert valid_guard(decode(rows,theta),theta>>73)
            assert all((mask&theta).bit_count()%2==rhs for mask,rhs in equations);controls+=1
    payload={'local_hull_equations':equations,'linear':linear,'charts':charts,'old_dimension':76,'original_complete':False}
    b=json.dumps(payload,separators=(',',':')).encode()
    with(P/'charts.json').open('xb')as f:f.write(b)
    out.update({'balanced_partitions':visited,'balanced_width_histogram':dict(sorted(hist.items())),'minimum_balanced_width':best[0],'minimum_mask':best[1],'universal_ordering_peak_lower_bound':best[0],'rank_evaluations':W.calls,'local_linear_equations':len(equations),'local_linear_rank':linear['rank'],'local_linear_dimension':linear['dimension'],'chart_dimensions':[c.get('dimension')for c in charts],'chart_feasible':[c['feasible']for c in charts],'chart_parameter_count':sum(1<<c['dimension']for c in charts if c['feasible']),'actual_guard_controls':controls,'charts_sha256':hashlib.sha256(b).hexdigest(),'no_frontier_allocated':True,'worker_seconds':time.monotonic()-START})
except Exception as e:error=repr(e)
finish(out,error,START,'Exhaustive balanced-cut affine lower bound and independently validated guarded primary-coordinate charts.','Compose all original predicates with each exact smaller chart in a new variant; verify complete sets on tiny systems and all original lift bits before changed hard search.')
