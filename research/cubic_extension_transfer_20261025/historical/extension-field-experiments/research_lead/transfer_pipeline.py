"""Reusable, bounded end-to-end cubic ECDLP transfer and benchmarks.

The solver accepts E,P,Q,r; it never accepts the target logarithm. Exceptional
evaluation charts trigger common-scalar retries. No claim that all exceptional
geometric cases have been eliminated is made; exhausted budgets are explicit.
"""
import argparse,json,random,statistics,sys
from pathlib import Path
from time import perf_counter
from sage.all import GF,EllipticCurve,Integer,set_random_seed
from inverse_map import setup
from polynomial_inverse import (symmetric_sections,interpolate,image_coordinates,invert,
                                divisor_images,ChartFailure,effective_divisor)
from target_index_calculus import solve as basic_solver
from line_index_calculus import solve as line_solver
from large_prime_index_calculus import solve as large_prime_solver
from pollard_rho import solve as rho_solver
from quotient_sweep import parse_curve
ROOT=Path(__file__).resolve().parent;sys.path.insert(0,str(ROOT.parent))
from common import point_data,field_data,bsgs_ec


def solve(E,P,Q,r,seed=20261108,target_solver='line',chart_attempts=12,max_isogeny_steps=2,
          compare_basic=False):
    start=perf_counter();r=int(r)
    if not Integer(r).is_prime() or P.is_zero() or not (r*P).is_zero() or not (r*Q).is_zero():
        raise ValueError('A nonzero prime-order generator and a target in its subgroup are required')
    for m in [0,1,r-1]:
        if m*P==Q:return {'status':'passed','scalar':m,'seconds':perf_counter()-start,
                         'algorithm':'trivial target identity','target_solver_used':False}
    if int(E.cardinality())!=r:raise ValueError('This implementation requires prime full group order')
    p=int(E.base_field().characteristic())
    if E.base_field().degree()!=3 or p%3!=1 or r%9!=4 or (p**3+1-r)%p==0 or E.j_invariant()**p==E.j_invariant():
        raise ValueError('Source does not meet the ordinary non-subfield cubic torsion gate')
    ctx=setup(tangent_coordinates=True,indices=[8,16,24,32,40,48,56],source_curve=E,
              max_isogeny_steps=max_isogeny_steps)
    setup_seconds=perf_counter()-start;stage=perf_counter()
    zeta,B,certificate=symmetric_sections(ctx['g'],seed=seed)
    transform,holdouts=interpolate(ctx,zeta,B,seed=seed+1)
    sections_seconds=perf_counter()-stage;rng=random.Random(seed);failures=[];timings=[]
    group_context=None
    for attempt in range(chart_attempts):
        multiplier=1 if attempt==0 else rng.randrange(1,r)
        coordinates=[image_coordinates(ctx,transform,multiplier*R) for R in [P,Q,P+Q]]
        if any(c is None for c in coordinates):
            failures.append({'attempt':attempt,'multiplier':multiplier,'reason':'evaluation pole'});continue
        lifted=[];attempt_timings=[]
        try:
            for c in coordinates:
                solutions,ring,timing=invert(ctx['g'],zeta,c)
                images,group_context=divisor_images(ctx['g'],ctx,solutions,ring,group_context)
                lifted.append(images);attempt_timings.append(timing)
            S=lifted[0][0]
            candidates=[T for T in lifted[1] if any(S+T==U for U in lifted[2])]
            if not candidates:raise ChartFailure('Three images do not admit a common sign choice')
            T=candidates[0];timings=attempt_timings;break
        except (ChartFailure,ZeroDivisionError) as exc:
            failures.append({'attempt':attempt,'multiplier':multiplier,'reason':str(exc)})
    else:raise ChartFailure('Common-scalar chart retry budget exhausted')
    F,G,basepoint,base,places=group_context
    assert not S==G.zero() and r*S==G.zero() and r*T==G.zero()
    if target_solver=='line':result=line_solver(S,T,r,base,places,ctx['g'],seed=seed+2)
    elif target_solver=='large-prime':result=large_prime_solver(S,T,r,base,places,ctx['g'],seed=seed+2)
    elif target_solver=='basic':result=basic_solver(S,T,r,base,places,seed=seed+2)
    else:raise ValueError('Unknown target solver')
    assert result['scalar']*P==Q
    total=perf_counter()-start
    out={'status':'passed','scalar':result['scalar'],'p':p,'r':r,'seconds':total,
         'source_curve':str(E),'source_field':field_data(E.base_field()),
         'source_generator':point_data(P),'source_target':point_data(Q),
         'quartic':str(ctx['g']),'isogeny_steps':len(ctx['isogeny_chain']),
         'isogeny_kernels':[str(phi.kernel_polynomial()) for phi in ctx['isogeny_chain']],
         'obstruction_walk':ctx['obstruction_walk'],'setup_seconds':setup_seconds,
         'sections_seconds':sections_seconds,'section_certificate':certificate,
         'interpolation_holdouts':holdouts,'chart_failures':failures,
         'source_randomization_multiplier':multiplier,'inversions':timings,
         'target_solver':result,'target_solver_used':True,
         'target_group_enumeration_used':False,'source_log_used_in_transfer':False,
         'all_image_coordinates_in_base_field':True,
         'limits':['Bounded retries, not a universal exceptional-chart proof.',
                   'These experimental target solvers use dense linear algebra, without a new exponent claim.',
                   'Small-field experiments do not establish an asymptotic or practical advantage.']}
    # Independent controls run only after successful transfer recovery.
    baselines=[rho_solver(P,Q,r,seed=seed+i) for i in range(5)]
    assert all(a['scalar']==result['scalar'] for a in baselines)
    out['source_pollard_rho']={'runs':baselines,'median_seconds':statistics.median(a['seconds'] for a in baselines)}
    out['source_bsgs']=bsgs_ec(P,Q,r)
    if compare_basic:
        comparison=basic_solver(S,T,r,base,places,seed=seed+2);assert comparison['scalar']==result['scalar']
        out['basic_target_comparison']=comparison
    return out


def instance(p,seed):
    set_random_seed(seed);K=GF(p**3,name='z');z=K.gen()
    if p==7:
        E=EllipticCurve(K,[4*z*z+3*z+3,4*z*z+5*z+2]);r=373
    else:
        for _ in range(100000):
            a,b=K.random_element(),K.random_element()
            if not 4*a**3+27*b*b:continue
            E=EllipticCurve(K,[a,b]);j=E.j_invariant()
            if j**p==j:continue
            xs=E.division_polynomial(3).roots(multiplicities=False)
            if len(xs) not in [1,4] or (xs[0]**3+a*xs[0]+b).is_square():continue
            r=int(E.cardinality())
            if r%9==4 and (p**3+1-r)%p and Integer(r).is_prime():break
        else:raise RuntimeError('Instance-generation budget exhausted')
    P=E.random_point()
    while P.is_zero():P=E.random_point()
    # Benchmark generation knows a scalar; the solve() API receives only Q.
    secret=random.Random(seed+99).randrange(2,r-1);Q=secret*P
    return E,P,Q,r,secret


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--primes',default='7,13,19,31')
    ap.add_argument('--solver',choices=['line','large-prime','basic'],default='line')
    ap.add_argument('--compare-basic',action='store_true');ap.add_argument('--seed',type=int,default=20261108)
    args=ap.parse_args();rows=[]
    path=ROOT/('results/four_track_benchmarks.json' if args.solver=='line' else f'results/four_track_{args.solver}_benchmarks.json')
    if path.exists():
        old=json.loads(path.read_text())
        if old.get('seed')==args.seed:rows=old['experiments']
    for p in map(int,args.primes.split(',')):
        if any(row['p']==p and row['status']=='passed' for row in rows):continue
        E,P,Q,r,expected=instance(p,args.seed+p)
        print('INSTANCE',p,r,flush=True)
        result=solve(E,P,Q,r,seed=args.seed+p,compare_basic=args.compare_basic,target_solver=args.solver)
        assert result['scalar']==expected
        result['generation_scalar_verified_only_after_solve']=True;rows.append(result)
        path.write_text(json.dumps({'status':'running','seed':args.seed,'experiments':rows},indent=2)+'\n')
        print('PIPELINE COMPLETE',p,result['seconds'],'rho',result['source_pollard_rho']['median_seconds'],flush=True)
    data=json.loads(path.read_text());data['status']='complete';path.write_text(json.dumps(data,indent=2)+'\n')


if __name__=='__main__':main()
