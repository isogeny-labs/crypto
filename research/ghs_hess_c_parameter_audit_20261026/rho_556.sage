"""Deterministic same-source Pollard rho trials for the order-139 subgroup."""

import json
import random
import statistics
import time
from pathlib import Path

root=Path(__file__).resolve().parent
K=GF(512,'w',modulus='conway')
E=EllipticCurve(K,[K(1),K(0),K(0),K(0),K.from_integer(32)])
G=E([K.from_integer(255),K.from_integer(59)])
Q=E([K.from_integer(206),K.from_integer(467)])
r=139
assert E.cardinality()==556 and (r*G).is_zero() and not G.is_zero()

def step(state):
    P,a,b=state
    cls=0 if P.is_zero() else int(P[0].to_integer())%3
    if cls==0: return (P+G,(a+1)%r,b)
    if cls==1: return (P+Q,a,(b+1)%r)
    return (2*P,(2*a)%r,(2*b)%r)

trials=[]
for trial in range(32):
    seed=20261026+trial
    rng=random.Random(int(seed))
    start=time.perf_counter()
    found=None
    restarts=0
    evaluations=0
    for attempt in range(100):
        a=rng.randrange(r)
        b=rng.randrange(r)
        state=(a*G+b*Q,a,b)
        tortoise=step(state)
        hare=step(step(state))
        evaluations+=3
        for iteration in range(1,10000):
            if tortoise[0]==hare[0]:
                denominator=(hare[2]-tortoise[2])%r
                if denominator:
                    found=((tortoise[1]-hare[1])*inverse_mod(denominator,r))%r
                break
            tortoise=step(tortoise)
            hare=step(step(hare))
            evaluations+=3
        if found is not None: break
        restarts+=1
    elapsed=time.perf_counter()-start
    assert found is not None and found*G==Q
    trials.append({'trial':int(trial),'seed':int(seed),'answer':int(found),
                   'seconds':float(elapsed),'step_evaluations':int(evaluations),
                   'restarts':int(restarts)})

result={'schema':'ghs-hess-c-556-same-target-rho/v1','source_b':int(32),
        'source_order':int(556),'prime':int(r),'trial_count':int(len(trials)),
        'median_seconds':float(statistics.median(t['seconds'] for t in trials)),
        'median_step_evaluations':float(statistics.median(t['step_evaluations'] for t in trials)),
        'trials':trials}
(root/'rho_556.json').write_text(json.dumps(result,indent=2)+'\n')
print('median_seconds',result['median_seconds'],'median_steps',result['median_step_evaluations'],flush=True)
