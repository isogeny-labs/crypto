"""Reproducible small prime-field fixtures and known-label separation audit.

The prime group orders are exact point counts from count_curves.cpp. These
are experimental small ordinary curves, not cryptographic-size challenges.
Every pool point is computed through public alpha*P + beta*Q. Validation
secrets and any scalar-label reference oracle remain in separate files.
"""
from __future__ import annotations
import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import random
import resource
import subprocess
import time

import sympy

HERE = Path(__file__).resolve().parent
OUTPUT = HERE.parent / "prime_fixtures"


class Curve:
    def __init__(self,p,a,b,r):
        self.p,self.a,self.b,self.r=p,a,b,r
        self.reset()
    def reset(self):
        self.counts=Counter()
    def mul(self,x,y):
        self.counts['field_multiplications']+=1
        return x*y % self.p
    def sq(self,x):
        self.counts['field_squarings']+=1
        return x*x % self.p
    def inv(self,x):
        self.counts['field_inversions']+=1
        return pow(x,-1,self.p)
    def neg(self,P):
        self.counts['group_negations']+=1
        return None if P is None else (P[0],-P[1] % self.p)
    def add(self,P,Q):
        self.counts['group_add_calls']+=1
        if P is None:return Q
        if Q is None:return P
        x,y=P;xx,yy=Q;p=self.p
        if x==xx:
            if (y+yy)%p==0:
                self.counts['inverse_or_2torsion_pairs']+=1
                return None
            self.counts['affine_doublings']+=1
            s=self.mul((3*self.sq(x)+self.a)%p,self.inv(2*y%p))
        else:
            self.counts['affine_additions']+=1
            s=self.mul((yy-y)%p,self.inv((xx-x)%p))
        z=(self.sq(s)-x-xx)%p
        return z,(self.mul(s,(x-z)%p)-y)%p
    def scale(self,n,P):
        self.counts['scalar_multiplications']+=1
        out=None
        while n:
            self.counts['scalar_bits']+=1
            if n&1:out=self.add(out,P)
            P=self.add(P,P);n>>=1
        return out
    def on_curve(self,P):
        return P is None or (P[1]*P[1]-P[0]**3-self.a*P[0]-self.b)%self.p==0
    def lift(self,x):
        value=(x**3+self.a*x+self.b)%self.p
        y=sympy.sqrt_mod(value,self.p)
        return None if y is None else (x,min(int(y),self.p-int(y)))


def dump(path,data):
    path.write_text(json.dumps(data,sort_keys=True,indent=2)+'\n')


def point_record(index,P,alpha,beta):
    return {'index':index,'x':P[0],'y':P[1],'alpha':alpha,'beta':beta}


def generate_public_pools(c,P,Q,B,seed):
    rng=random.Random(seed);c.reset()
    wall=time.perf_counter();cpu=time.process_time()
    seen=set();pools=[];draws=0;rejects=Counter()
    for slot in range(3):
        pool=[]
        while len(pool)<B:
            draws+=1
            alpha,beta=rng.randrange(c.r),rng.randrange(c.r)
            R=c.add(c.scale(alpha,P),c.scale(beta,Q))
            if R is None:
                rejects['infinity']+=1;continue
            if R[0] in seen:
                rejects['duplicate_abscissa']+=1;continue
            seen.add(R[0])
            pool.append(point_record(len(pool),R,alpha,beta))
        pools.append(pool)
    metrics={'cpu_seconds':time.process_time()-cpu,'wall_seconds':time.perf_counter()-wall,
             'counter_scope':'Public alpha*P+beta*Q pool construction, including rejected draws; Python modular inverse internals not expanded into multiplications.',
             'counters':dict(c.counts),'draws':draws,'accepted':3*B,'rejected':dict(rejects),
             'secret_used_for_pool_generation':False}
    return pools,metrics


def secret_reference(pools,k,r):
    """Validation-only scalar oracle, never an attack control or timed solver."""
    values=[[((z['alpha']+z['beta']*k)%r) for z in pool] for pool in pools]
    third={}
    for j,v in enumerate(values[2]):
        for sign in (1,-1):third.setdefault(sign*v%r,(j,sign))
    for i,u in enumerate(values[0]):
        for j,v in enumerate(values[1]):
            for s in (1,-1):
                for t in (1,-1):
                    match=third.get(-(s*u+t*v)%r)
                    if match is not None:
                        jj,ss=match
                        return {'indices':[i,j,jj],'signs':[s,t,ss]}
    return None


def verify_fixture(c,P,Q,pools,k):
    assert sympy.isprime(c.p) and sympy.isprime(c.r) and c.r!=c.p
    assert c.on_curve(P) and c.on_curve(Q)
    assert c.scale(c.r,P) is None and c.scale(k,P)==Q
    seen=set()
    for pool in pools:
        for z in pool:
            R=(z['x'],z['y'])
            assert c.on_curve(R)
            assert R[0] not in seen
            seen.add(R[0])
            assert c.add(c.scale(z['alpha'],P),c.scale(z['beta'],Q))==R
    witness=secret_reference(pools,k,c.r)
    if witness is not None:
        out=None
        for pool,i,sign in zip(pools,witness['indices'],witness['signs']):
            z=pool[i];R=(z['x'],z['y'])
            out=c.add(out,R if sign==1 else c.neg(R))
        assert out is None
    return witness


def structured_control(c,P,Q,B,k,case_id,measure_labels):
    # Fixed x interval. Actual liftable count is recorded, not declared as B.
    wall=time.perf_counter();cpu=time.process_time()
    points=[]
    for x in range(2*B):
        R=c.lift(x)
        if R is not None:points.append(R)
    setup={'cpu_seconds':time.process_time()-cpu,'wall_seconds':time.perf_counter()-wall,
           'interval_length':2*B,'liftable_abscissae':len(points),'signed_points':2*len(points)}
    control={'schema':'structured-label-separation/v1','id':case_id,'curve':{'p':c.p,'a':c.a,'b':c.b,'r':c.r,'P':P,'Q':Q},
             'policy':'0 <= x < 2*nominal_B; canonical ordinate; no mixed labels supplied',
             'nominal_B':B,'points':[{'x':x,'y':y} for x,y in points],
             'setup':setup,'mixed_labels_available':False,
             'random_label_draw_acceptance_probability':2*len(points)/c.r,
             'expected_uniform_public_draws_per_accepted_point':c.r/(2*len(points)) if points else None,
             'scope':'Rejection-sampling cost is for this explicit method, not a lower bound for all label-construction algorithms.'}
    if measure_labels:
        lookup={x:y for x,y in points};found={};rng=random.Random(71001+B)
        c.reset();wall=time.perf_counter();cpu=time.process_time();draws=0
        cap=100*c.r
        while len(found)<len(points) and draws<cap:
            draws+=1
            alpha,beta=rng.randrange(c.r),rng.randrange(c.r)
            R=c.add(c.scale(alpha,P),c.scale(beta,Q))
            if R is None or R[0] not in lookup:continue
            if R[1]!=lookup[R[0]]:alpha,beta=-alpha%c.r,-beta%c.r
            found.setdefault(R[0],(alpha,beta))
        measured={'cpu_seconds':time.process_time()-cpu,'wall_seconds':time.perf_counter()-wall,
                  'draws':draws,'counter_scope':'Actual public mixed-label rejection sampling; all misses and duplicate accepted points charged.',
                  'counters':dict(c.counts),'labelled_points':len(found),'total_points':len(points),
                  'complete':len(found)==len(points),'cap_draws':cap}
        for x,(alpha,beta) in found.items():assert c.add(c.scale(alpha,P),c.scale(beta,Q))==(x,lookup[x])
        control['charged_label_acquisition']=measured
    return control


def run(sizes,curves_per_size,seeds_per_curve):
    os.sched_setaffinity(0,{3});OUTPUT.mkdir(parents=True,exist_ok=True)
    binary=HERE/'count_curves'
    subprocess.run(['g++','-O3','-std=c++17',str(HERE/'count_curves.cpp'),'-o',str(binary)],check=True)
    manifest={'schema':'prime-pools-manifest/v1','cpu_affinity':[3],
              'scope':'Small exact ordinary prime-order fixtures; exploratory shared-host CPU measurements, no attack runtime claim.',
              'r_over_B_cubed_target':8,'natural_policy':'Three pools, each B; independent uniform public mixed-label draws conditioned only on no infinity or repeated abscissa.',
              'fixtures':[],'structured_controls':[],'curve_searches':[]}
    for B in sizes:
        p=int(sympy.nextprime(8*B**3))
        wall=time.perf_counter();cpu=time.process_time()
        result=subprocess.run([str(binary),str(p),str(curves_per_size),str(20261116+B)],check=True,capture_output=True,text=True)
        curves=json.loads(result.stdout)
        manifest['curve_searches'].append({'B':B,'p':p,'requested_curves':curves_per_size,
             'curve_search_wall_seconds':time.perf_counter()-wall,'python_orchestrator_cpu_seconds':time.process_time()-cpu,
             'scope':'Fixture preparation, excluded from all solver times; exact count C++ child CPU is not captured by Python process_time.',
             'curves':curves})
        for curve_index,entry in enumerate(curves):
            c=Curve(entry['p'],entry['a'],entry['b'],entry['r'])
            P=next(R for x in range(c.p) if (R:=c.lift(x)) is not None and R[1])
            private_rng=random.Random(9019111+1000*B+curve_index)
            k=private_rng.randrange(1,c.r)
            Q=c.scale(k,P)
            assert c.scale(c.r,P) is None and Q is not None
            for seed_index in range(seeds_per_curve):
                seed=1000000*B+1000*curve_index+seed_index
                pools,generation=generate_public_pools(c,P,Q,B,seed)
                case_id=f'b{B:03d}_c{curve_index}_natural_s{seed_index}'
                fixture={'schema':'prime-pools/v1','id':case_id,'B':B,'kind':'natural','public_seed':seed,
                         'curve':{'p':c.p,'a':c.a,'b':c.b,'r':c.r,'P':P,'Q':Q},
                         'pools':pools,'generation':generation,'r_over_B_cubed':c.r/B**3}
                witness=verify_fixture(c,P,Q,pools,k)
                validation={'schema':'prime-pools-validation/v1','id':case_id,'secret_k':k,
                            'all_points_and_mixed_labels_checked':True,'scalar_reference_scope':'Validation-only using k; not an attack control or runtime.',
                            'signed_zero_sum_exists':witness is not None,'witness':witness}
                path=OUTPUT/f'{case_id}.json';dump(path,fixture);dump(OUTPUT/f'{case_id}.validation.json',validation)
                manifest['fixtures'].append({'id':case_id,'B':B,'kind':'natural','path':str(path),
                    'validation_path':str(OUTPUT/f'{case_id}.validation.json'),'curve_index':curve_index})
                dump(OUTPUT/'manifest.json',manifest)
                print(json.dumps({'fixture':str(path),'B':B,'natural':True,'signed_relation_exists_validation_only':witness is not None}),flush=True)
            structured=structured_control(c,P,Q,B,k,f'b{B:03d}_c{curve_index}_structured',measure_labels=(B==8))
            structured_path=OUTPUT/f'b{B:03d}_c{curve_index}_structured.json';dump(structured_path,structured)
            manifest['structured_controls'].append(str(structured_path))
            dump(OUTPUT/'manifest.json',manifest)
    manifest['generator_source_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    manifest['point_count_source_sha256']=hashlib.sha256((HERE/'count_curves.cpp').read_bytes()).hexdigest()
    manifest['fixture_count']=len(manifest['fixtures'])
    manifest['peak_generator_rss_kib']=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    dump(OUTPUT/'manifest.json',manifest)
    return manifest


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--sizes',default='8,16,32,64,128')
    parser.add_argument('--curves',type=int,default=3)
    parser.add_argument('--seeds',type=int,default=4)
    args=parser.parse_args()
    data=run([int(x) for x in args.sizes.split(',')],args.curves,args.seeds)
    print(json.dumps({'manifest':str(OUTPUT/'manifest.json'),'fixtures':data['fixture_count'],'peak_rss_kib':data['peak_generator_rss_kib']},indent=2))
