"""Supplement: split-coordinate modular compositions for implicit incidence.

The third pool carries y coordinates. Interpolate Y3(Z) modulo F3, then
evaluate F2 at x(P1 +/- P3) by native FLINT modular composition. This uses
no pair table. It does not assert a new modular-composition complexity bound.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import time
from candidate import Ring,product_poly,monic,nmod_poly,recover_witness


def incidence_compose(fixture):
    curve=fixture['curve'];p=int(curve['p']);pools=fixture['pools']
    xs=[[int(row['x']) for row in pool] for pool in pools]
    if len(pools)!=3 or len({x for pool in xs for x in pool})!=sum(map(len,xs)):
        raise ValueError('Requires three globally disjoint coordinate pools')
    start=time.process_time();allstart=start;wall=time.perf_counter();phases={}
    f2=product_poly(xs[1],p);f3=product_poly(xs[2],p);Z=nmod_poly([0,1],p)
    derivative=f3.derivative();Y=nmod_poly([],p)
    for row in pools[2]:
        quotient,remainder=divmod(f3,Z-int(row['x']))
        if remainder:raise AssertionError('Interpolation division failed')
        weight=int(row['y'])*pow(int(derivative(int(row['x']))),-1,p)%p
        Y+=quotient*weight
    ring=Ring(p,f3);ring.counts['interpolation_scalar_inversions']=len(xs[2])
    phases['preprocessing']=time.process_time()-start;start=time.process_time()
    norm=ring.c(1);curve_poly=(Z**3+int(curve['a'])*Z+int(curve['b']))%f3
    for row in pools[0]:
        x=int(row['x']);y=int(row['y']);fx=int(f3(x))
        quotient,remainder=divmod(f3-fx,Z-x)
        if remainder:raise AssertionError('Inverse construction division failed')
        inv=quotient*pow(fx,-1,p)
        ring.counts['denominator_scalar_inversions']+=1
        inv2=ring.mul(inv,inv)
        base=ring.mul(curve_poly+y*y,inv2)-x-Z
        cross=ring.mul(Y*(2*y%p),inv2)
        tplus=(base+cross)%f3;tminus=(base-cross)%f3
        vplus=f2.compose_mod(tplus,f3);vminus=f2.compose_mod(tminus,f3)
        ring.counts['native_modular_compositions']+=2
        norm=ring.mul(norm,ring.mul(vplus,vminus))
    phases['modular_composition_incidence']=time.process_time()-start;start=time.process_time()
    G=monic(f3.gcd(norm));phases['incidence_gcd']=time.process_time()-start
    result=recover_witness(fixture,G)
    phases['witness_and_scalar_recovery']=result.pop('recovery_cpu_seconds')
    result.update({'arm':'split_compose','gcd_degree':int(G.degree()),
        'gcd_coefficients':[int(c) for c in G.coeffs()], 'phase_cpu_seconds':phases,
        'solver_core_bracket_cpu_seconds':time.process_time()-allstart,
        'solver_total_wall_seconds':time.perf_counter()-wall,
        'counters':dict(ring.counts),'dimensions':{'quotient_dimension':ring.dimension,
        'first_pool_iterations':len(xs[0]),'largest_stored_univariate_degree':max(3,ring.dimension),
        'backend_internal_workspace':'Native FLINT compose_mod workspace; not a pair table; peak process RSS measured.'},
        'peak_process_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        'complexity_scope':'B iterations, 2B native degree-B modular compositions, 5B explicit quotient products, O(B^2) classical interpolation coefficient work. compose_mod internals uncounted; no subquadratic claim.'})
    return result


def main():
    parser=argparse.ArgumentParser();parser.add_argument('--fixture',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True);parser.add_argument('--min-batch-wall',type=float,default=.08)
    args=parser.parse_args();os.sched_setaffinity(0,{1});resource.setrlimit(resource.RLIMIT_CPU,(30,31))
    raw=args.fixture.read_bytes();fixture=json.loads(raw);samples=[]
    wall=time.perf_counter();cpu=time.process_time()
    while True:
        samples.append(incidence_compose(fixture))
        if time.perf_counter()-wall>=args.min_batch_wall or len(samples)>=512:break
    batchcpu=time.process_time()-cpu;batchwall=time.perf_counter()-wall;result=samples[-1];n=len(samples)
    result.update({'schema':'incidence-compose-result/v1','fixture_id':fixture['id'],'B':fixture['B'],
        'kind':fixture['kind'],'fixture_sha256':hashlib.sha256(raw).hexdigest(),
        'fixture_generation':fixture['generation'],'solver_total_cpu_seconds':batchcpu/n,
        'batch_cpu_seconds':batchcpu,'batch_wall_seconds':batchwall,'timing_repetitions':n,
        'phase_cpu_seconds':{k:sum(s['phase_cpu_seconds'][k] for s in samples)/n for k in result['phase_cpu_seconds']},
        'solver_total_wall_seconds':sum(s['solver_total_wall_seconds'] for s in samples)/n,
        'cpu_affinity':[1],'flint_version':'0.9.0',
        'phase_timing_scope':'Short inner CPU intervals unreliable/undersum; outer batch complete-call CPU is primary.'})
    args.output.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))


if __name__=='__main__':main()
