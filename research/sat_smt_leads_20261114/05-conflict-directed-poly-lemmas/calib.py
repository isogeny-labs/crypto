import sys, time, json
from pdp5 import *
n=13
out=[]
for l in (2,3,4):
    for idx,pl in ((900,True),(901,False),(902,False)):
        inst=make_instance(n,l,idx,pl); sysd=build_system(inst)
        r=run_arm(inst,sysd,'A0',B=10**9); r.pop('model'); r.pop('_lemmas')
        print(l,idx,pl,r['status'],r['conflicts'],round(r['wall'],1),flush=True)
        out.append(dict(l=l,idx=idx,**r))
        if idx==901:
            B=max(50,r['conflicts']//8)
            for k in (8,12,16):
                q=run_arm(inst,sysd,'A2',B=B,k=k); 
                print('   A2 k',k,q['status'],q['conflicts'],'lemmas',q['lemma_counts'],'gen_t',round(q['gen_time'],1),flush=True)
                q.pop('model'); q.pop('_lemmas'); out.append(dict(l=l,idx=idx,k=k,B=B,**q))
json.dump(out,open('calibration.json','w'),indent=1)
