"""A reproducible low-memory source-group Pollard-rho baseline."""
import math,random
from time import perf_counter


def solve(P,Q,r,seed=20261107,max_restarts=12):
    start=perf_counter();rng=random.Random(seed);adds=0
    if Q.is_zero():return {'scalar':0,'seconds':perf_counter()-start,'group_additions':0,'algorithm':'Pollard rho'}
    def key(V):return () if V.is_zero() else (V[0],V[1])
    for restart in range(max_restarts):
        table=[]
        for _ in range(16):
            a,b=rng.randrange(r),rng.randrange(r);table.append((a*P+b*Q,a,b))
        def step(state):
            nonlocal adds
            V,a,b=state;W,c,d=table[hash(key(V))%16];adds+=1
            return V+W,(a+c)%r,(b+d)%r
        a,b=rng.randrange(r),rng.randrange(r);slow=(a*P+b*Q,a,b);fast=slow
        for _ in range(20*math.isqrt(r)+100):
            slow=step(slow);fast=step(step(fast))
            if slow[0]==fast[0]:
                den=(fast[2]-slow[2])%r
                if not den:break
                answer=(slow[1]-fast[1])*pow(den,-1,r)%r
                assert answer*P==Q
                return {'scalar':answer,'seconds':perf_counter()-start,'group_additions':adds,
                        'restarts':restart,'seed':seed,'walk_table_size':16,
                        'algorithm':'16-adding Pollard rho with Floyd collision detection',
                        'count_scope':'Walk additions only; wall time also includes table setup and verification'}
    raise RuntimeError('Pollard-rho restart budget exhausted')
