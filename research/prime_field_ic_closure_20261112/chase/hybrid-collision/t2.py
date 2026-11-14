import random,sys,statistics as st,math
sys.path.insert(0,'.')
from hyb import *
rng=random.Random(7)
res={False:[],True:[]}
for c in range(4):
    E,P=gen_curve(20,rng)
    for i in range(15):
        x=rng.randrange(1,E.n);Q=E.mul(x,P)
        for neg in (False,True):
            r=run_instance(E,P,Q,x,random.Random(i*31+c),0,0,neg=neg,use_table=False)
            res[neg].append(r['events']['rho']['gops']/math.sqrt(E.n))
for k,v in res.items(): print(k,len(v),round(st.mean(v),3),round(st.stdev(v)/math.sqrt(len(v)),3))
print('theory plain',math.sqrt(math.pi/2),'neg',math.sqrt(math.pi/4))
