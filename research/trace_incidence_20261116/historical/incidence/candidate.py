"""Exact modular-incidence GCD for three labelled point pools.

The candidate never constructs or scans a table of pair sums.  It loops over
the first pool only. For each x it reduces F2(Y) modulo the normalized
quadratic S3(x,Y,Z) over R=Fp[Z]/F3(Z), takes the quadratic norm, and multiplies
the norm factors in R. The final gcd detects candidate third coordinates.

Horner uses O(B^2) quotient multiplications of dimension B, not subquadratic
total field work. The blocked arm uses baby/giant evaluation in the quadratic
algebra and a native matrix product for block linear combinations. Its
classical coefficient/matrix work is still potentially cubic in B overall.
No ECDLP exponent or global novelty claim is made by this implementation.
"""

from __future__ import annotations
from collections import Counter
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import resource
import sys
import time

os.environ.setdefault('OMP_NUM_THREADS','1')
os.environ.setdefault('OPENBLAS_NUM_THREADS','1')
os.environ.setdefault('MKL_NUM_THREADS','1')
sys.path.insert(0,'/workspace/ecdlp_research/math_vendor')
from flint import ctx, nmod_mat, nmod_poly
ctx.threads=1


class Ring:
    def __init__(self,p,modulus):
        self.p=p
        self.h=modulus
        self.dimension=int(modulus.degree())
        self.counts=Counter()

    def c(self,a):return nmod_poly([a % self.p],self.p)

    def mul(self,a,b):
        self.counts['quotient_multiplications']+=1
        self.counts['polynomial_product_coefficient_pairs']+=len(a)*len(b)
        if not a or not b:
            self.counts['zero_product_shortcuts']+=1
            return self.c(0)
        return (a*b) % self.h

    def qmuly(self,pair,beta,gamma):
        a,b=pair
        self.counts['quadratic_times_generator']+=1
        return -self.mul(gamma,b),a-self.mul(beta,b)

    def qmul(self,left,right,beta,gamma):
        a,b=left;c,d=right
        self.counts['quadratic_products']+=1
        u=self.mul(a,c)
        v=self.mul(b,d)
        w=self.mul(a+b,c+d)-u-v
        return u-self.mul(gamma,v),w-self.mul(beta,v)


def monic(f):
    if not f:return f
    return f*pow(int(f.leading_coefficient()),-1,int(f.modulus()))


def product_poly(xs,p):
    factors=[nmod_poly([-int(x)%p,1],p) for x in xs]
    if not factors:return nmod_poly([1],p)
    while len(factors)>1:
        factors=[factors[i]*factors[i+1] if i+1<len(factors) else factors[i]
                 for i in range(0,len(factors),2)]
    return factors[0]


def normalized_s3(x,curve,ring):
    """Coefficients beta,gamma of Y²+beta*Y+gamma; x is disjoint from F3."""
    p=ring.p;a=curve['a'];b=curve['b']
    Z=nmod_poly([0,1],p)
    denominator=(Z-x)**2
    g,s,_=denominator.xgcd(ring.h)
    ring.counts['quotient_inversions']+=1
    if int(g.degree())!=0:
        raise ValueError('Disjoint-coordinate precondition violated')
    inverse=(s*pow(int(g[0]),-1,p)) % ring.h
    B=-2*((Z+x)*(Z*x+a)+2*b)
    C=(Z*x-a)**2-4*b*(Z+x)
    beta=ring.mul(B % ring.h,inverse)
    gamma=ring.mul(C % ring.h,inverse)
    return beta,gamma


def evaluate_horner(coefficients,beta,gamma,ring):
    r0=r1=ring.c(0)
    for c in reversed(coefficients):
        r0,r1=ring.c(c)-ring.mul(gamma,r1),r0-ring.mul(beta,r1)
    return r0,r1


def block_plan(coefficients,p):
    m=math.isqrt(max(1,len(coefficients)-1))+1
    blocks=(len(coefficients)+m-1)//m
    rows=[[int(coefficients[i*m+j]) if i*m+j<len(coefficients) else 0
           for j in range(m)] for i in range(blocks)]
    return m,nmod_mat(rows,p)


def evaluate_blocked(coefficients,beta,gamma,ring,plan):
    m,weights=plan
    babies=[(ring.c(1),ring.c(0))]
    for _ in range(m):babies.append(ring.qmuly(babies[-1],beta,gamma))
    d=ring.dimension
    baby_rows=[]
    for u,v in babies[:m]:
        baby_rows.append([int(u[j]) for j in range(d)]+[int(v[j]) for j in range(d)])
    block_values=(weights*nmod_mat(baby_rows,ring.p)).tolist()
    ring.counts['block_matrix_products']+=1
    ring.counts['block_matrix_classical_scalar_products']+=weights.nrows()*weights.ncols()*(2*d)
    blocks=[(nmod_poly(row[:d],ring.p),nmod_poly(row[d:],ring.p)) for row in block_values]
    out=(ring.c(0),ring.c(0))
    for block in reversed(blocks):
        u,v=ring.qmul(out,babies[m],beta,gamma)
        out=u+block[0],v+block[1]
    return out


def incidence_gcd(fixture,arm='blocked'):
    curve=fixture['curve'];p=int(curve['p'])
    pools=fixture['pools']
    xs=[[int(row['x']) for row in pool] for pool in pools]
    if len(pools)!=3 or len({x for pool in xs for x in pool})!=sum(map(len,xs)):
        raise ValueError('Requires three globally disjoint coordinate pools')
    phase={};start=time.process_time()
    f2=product_poly(xs[1],p);f3=product_poly(xs[2],p)
    ring=Ring(p,f3)
    coefficients=[int(c) for c in f2.coeffs()]
    plan=block_plan(coefficients,p) if arm=='blocked' else None
    phase['preprocessing']=time.process_time()-start
    started=time.process_time()
    norm=ring.c(1)
    factor_degrees=[]
    for x in xs[0]:
        beta,gamma=normalized_s3(x,curve,ring)
        r0,r1=(evaluate_blocked(coefficients,beta,gamma,ring,plan)
               if arm=='blocked' else evaluate_horner(coefficients,beta,gamma,ring))
        # Norm(r0+r1*Y) in the quadratic algebra Y²+beta*Y+gamma=0.
        cross=ring.mul(r0,r1)
        factor=ring.mul(r0,r0)-ring.mul(beta,cross)+ring.mul(gamma,ring.mul(r1,r1))
        norm=ring.mul(norm,factor)
        factor_degrees.append(int(factor.degree()))
    phase['modular_norm']=time.process_time()-started
    started=time.process_time()
    G=monic(f3.gcd(norm))
    phase['incidence_gcd']=time.process_time()-started
    return G,phase,dict(ring.counts),{'quotient_dimension':ring.dimension,
        'quadratic_algebra_dimension':2*ring.dimension,
        'first_pool_iterations':len(xs[0]),'expanded_norm_degree_bound':2*len(xs[0])*len(xs[1]),
        'largest_stored_univariate_degree':max(ring.dimension,int(f2.degree())),
        'factor_degree_min':min(factor_degrees),'factor_degree_max':max(factor_degrees),
        'block_width':plan[0] if plan else None,
        'largest_block_matrix_entries':max(plan[1].nrows()*plan[1].ncols(),
            plan[0]*(2*ring.dimension),plan[1].nrows()*(2*ring.dimension)) if plan else 0,
        'block_baby_matrix_shape':[plan[0],2*ring.dimension] if plan else None,
        'block_output_matrix_shape':[plan[1].nrows(),2*ring.dimension] if plan else None,
        'coefficient_pair_counter_scope':'Schoolbook coefficient-product proxy, not actual FLINT multiplication count.',
        'matrix_scalar_counter_scope':'Classical matrix-product proxy, not actual native operation count.'}


class Group:
    def __init__(self,curve):
        self.p=int(curve['p']);self.a=int(curve['a']);self.r=int(curve['r'])
        self.counts=Counter()
    def neg(self,P):return None if P is None else (P[0],-P[1]%self.p)
    def add(self,P,Q):
        self.counts['group_addition_calls']+=1
        if P is None:return Q
        if Q is None:return P
        p=self.p;x,y=P;u,v=Q
        if x==u:
            if (y+v)%p==0:return None
            numerator=3*x*x+self.a;denominator=2*y
        else:numerator=v-y;denominator=u-x
        self.counts['field_inversions']+=1
        slope=numerator*pow(denominator%p,-1,p)%p
        z=(slope*slope-x-u)%p
        return z,(slope*(x-z)-y)%p
    def scale(self,k,P):
        result=None
        while k:
            if k&1:result=self.add(result,P)
            P=self.add(P,P);k>>=1
        return result


def recover_witness(fixture,G):
    curve=fixture['curve'];group=Group(curve);p=group.p;r=group.r
    start=time.process_time()
    lookup={}
    for row in fixture['pools'][1]:
        P=(int(row['x']),int(row['y']))
        lookup[P]=(row,1);lookup[group.neg(P)]=(row,-1)
    # Roots of G are selected from the known third-pool coordinates. This is
    # one O(B deg(G)) scan, not a pair-sum scan. The count is retained.
    candidates=[];evaluations=0
    for row in fixture['pools'][2]:
        evaluations+=1
        if int(G(int(row['x'])))==0:candidates.append(row)
    uninformative=None;uninformative_matches=0
    for third in candidates:
        target=group.neg((int(third['x']),int(third['y'])))
        for first in fixture['pools'][0]:
            P=(int(first['x']),int(first['y']))
            for sign in (1,-1):
                wanted=group.add(target,group.neg(P if sign==1 else group.neg(P)))
                if wanted not in lookup:continue
                second,second_sign=lookup[wanted]
                witness=[{'pool':0,'index':int(first['index']),'sign':sign},
                         {'pool':1,'index':int(second['index']),'sign':second_sign},
                         {'pool':2,'index':int(third['index']),'sign':1}]
                rows=(first,second,third);signs=(sign,second_sign,1)
                A=sum(s*int(row['alpha']) for row,s in zip(rows,signs))%r
                B=sum(s*int(row['beta']) for row,s in zip(rows,signs))%r
                total=None
                for row,s in zip(rows,signs):
                    P=(int(row['x']),int(row['y']))
                    total=group.add(total,P if s==1 else group.neg(P))
                if total is not None:raise AssertionError('Witness does not sum to identity')
                if B==0:
                    uninformative=witness;uninformative_matches+=1
                    continue
                k=(-A*pow(B,-1,r))%r
                if group.scale(k,tuple(curve['P']))!=tuple(curve['Q']):
                    raise AssertionError('Scalar verification failed')
                return {'status':'relation','witness':witness,'recovery':{'A':A,'B':B,'k':k},
                        'root_candidates':len(candidates),'gcd_evaluations':evaluations,
                        'uninformative_matches_skipped':uninformative_matches,
                        'recovery_cpu_seconds':time.process_time()-start,
                        'group_counters':dict(group.counts)}
    if uninformative is not None:
        return {'status':'uninformative','witness':uninformative,'recovery':None,
                'root_candidates':len(candidates),'gcd_evaluations':evaluations,
                'uninformative_matches_skipped':uninformative_matches,
                'recovery_cpu_seconds':time.process_time()-start,
                'group_counters':dict(group.counts)}
    if candidates:raise AssertionError('Incidence root had no point witness')
    return {'status':'none','witness':None,'recovery':None,
            'root_candidates':0,'gcd_evaluations':evaluations,
            'recovery_cpu_seconds':time.process_time()-start,'group_counters':dict(group.counts)}


def solve(fixture,arm='blocked'):
    started=time.process_time();wall=time.perf_counter()
    G,phases,counters,dimensions=incidence_gcd(fixture,arm)
    result=recover_witness(fixture,G)
    phases['witness_and_scalar_recovery']=result.pop('recovery_cpu_seconds')
    result.update({'arm':arm,'gcd_degree':int(G.degree()),'gcd_coefficients':[int(c) for c in G.coeffs()],
        'phase_cpu_seconds':phases,'solver_total_cpu_seconds':time.process_time()-started,
        'solver_total_wall_seconds':time.perf_counter()-wall,
        'counters':counters,'dimensions':dimensions,
        'peak_process_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
        'complexity_scope':'No pair table. B first-pool iterations; Horner uses B² quotient products. Blocked reduces quotient products but may retain cubic coefficient work. No sub-B^1.5 claim.'})
    return result


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--fixture',type=Path,required=True)
    parser.add_argument('--arm',choices=('horner','blocked'),default='blocked')
    parser.add_argument('--output',type=Path)
    parser.add_argument('--cpu-cap',type=int,default=30)
    parser.add_argument('--min-batch-wall',type=float,default=0.0)
    parser.add_argument('--max-repetitions',type=int,default=512)
    args=parser.parse_args()
    if hasattr(os,'sched_setaffinity'):os.sched_setaffinity(0,{1})
    if args.cpu_cap:resource.setrlimit(resource.RLIMIT_CPU,(args.cpu_cap,args.cpu_cap+1))
    raw=args.fixture.read_bytes();fixture=json.loads(raw)
    batch_start=time.perf_counter();batch_cpu=time.process_time()
    samples=[]
    while True:
        result=solve(fixture,args.arm)
        samples.append(result)
        if len(samples)>1:
            if (samples[0]['status'],samples[0]['gcd_coefficients'])!=(result['status'],result['gcd_coefficients']):
                raise AssertionError('Deterministic replay mismatch')
        if time.perf_counter()-batch_start>=args.min_batch_wall or len(samples)>=args.max_repetitions:
            break
    repetitions=len(samples)
    batch_cpu=time.process_time()-batch_cpu
    batch_wall=time.perf_counter()-batch_start
    result['phase_cpu_seconds']={key:sum(row['phase_cpu_seconds'][key] for row in samples)/repetitions
                                 for key in result['phase_cpu_seconds']}
    result['solver_core_bracket_cpu_seconds']=sum(row['solver_total_cpu_seconds'] for row in samples)/repetitions
    result['solver_total_cpu_seconds']=batch_cpu/repetitions
    result['solver_total_wall_seconds']=sum(row['solver_total_wall_seconds'] for row in samples)/repetitions
    result['timing_repetitions']=repetitions
    result['batch_cpu_seconds']=batch_cpu
    result['batch_wall_seconds']=batch_wall
    result['batch_scope']='Every repetition rebuilds input polynomials and all candidate state; process/library startup is separately charged by benchmark runner. CPU-pinned, shared host, exploratory timings.'
    result['unallocated_batch_cpu_seconds_per_call']=max(0.0,batch_cpu/repetitions-sum(result['phase_cpu_seconds'].values()))
    result.update({'schema':'incidence-gcd-result/v1','fixture_id':fixture['id'],
        'B':fixture['B'],'kind':fixture['kind'],'fixture_sha256':hashlib.sha256(raw).hexdigest(),
        'fixture_generation':fixture.get('generation'),'flint_version':'0.9.0',
        'cpu_affinity':sorted(os.sched_getaffinity(0))})
    text=json.dumps(result,indent=2)
    if args.output:args.output.write_text(text+'\n')
    print(text,flush=True)


if __name__=='__main__':main()
