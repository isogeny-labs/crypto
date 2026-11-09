"""Floyd Pollard-rho on the exact order-277 source pair used by the cover."""

import json
import statistics
import time
from pathlib import Path

root=Path(__file__).resolve().parent
K=GF(512,'w',modulus='conway')
w=K.gen()
b=K.from_integer(105)
E=EllipticCurve(K,[K(1),K(1),K(0),K(0),b])
G=E([w^478,w^433])
Q=E([w^77,w^119])
r=277
assert E.cardinality()==554 and r*G==E(0) and Q==7*G

def step(state):
    P,a,c=state
    bucket=0 if P==E(0) else int(P[0].to_integer())%3
    if bucket==0:
        return P+G,(a+1)%r,c
    if bucket==1:
        return P+Q,a,(c+1)%r
    return P+P,(2*a)%r,(2*c)%r

records=[]
for trial in range(32):
    a0=(11*trial+1)%r
    c0=(37*trial+3)%r
    init=(a0*G+c0*Q,a0,c0)
    t0=time.perf_counter()
    tortoise=step(init)
    hare=step(step(init))
    operations=3
    while tortoise[0]!=hare[0] and operations<10000:
        tortoise=step(tortoise)
        hare=step(step(hare))
        operations+=3
    assert tortoise[0]==hare[0]
    denom=(hare[2]-tortoise[2])%r
    if denom==0:
        continue
    answer=((tortoise[1]-hare[1])*pow(denom,-1,r))%r
    assert answer==7 and answer*G==Q
    records.append({'trial':int(trial),'operations':int(operations),
                    'seconds':float(time.perf_counter()-t0),'answer':int(answer)})

assert len(records)>=24
result={'schema':'ghs-q8-554-rho-baseline/v1','source_b':int(105),'prime':int(r),
        'trials':records,'successes':len(records),
        'median_operations':statistics.median(row['operations'] for row in records),
        'median_seconds':statistics.median(row['seconds'] for row in records)}
(root/'rho_baseline.json').write_text(json.dumps(result,indent=2)+'\n')
print('successes',result['successes'],'median_operations',result['median_operations'],
      'median_seconds',result['median_seconds'])
