import random, time, sys
sys.path.insert(0,'.')
from hyb import *
rng=random.Random(1)
t=time.time(); E,P=gen_curve(20,rng); print('curve',E.p,E.a,E.b,E.n,time.time()-t)
x=rng.randrange(E.n); Q=E.mul(x,P)
for (V,m,inc) in [(0,0,False),(40,2,False),(60,2,False),(8,3,False),(1,200,True)]:
    t=time.time()
    r=run_instance(E,P,Q,x,rng,V,m,include_P=inc,use_table=(V>0))
    print(V,m,inc,{k:v for k,v in r.items() if k!='events'}, {k:(v['gops'],v['hits'],v['steps']) for k,v in r['events'].items()}, round(time.time()-t,2))
