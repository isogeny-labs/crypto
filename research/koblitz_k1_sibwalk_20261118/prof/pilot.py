import sys,collections
sys.path.insert(0,sys.argv[1]); from model import *
S="/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad"
src={23:(f"{S}/klc-sessions-a1/n23/records.jsonl.gz",{"sp-c4-d5":(4,5),"sp-c5-d5":(5,5),"sp-c4-d3":(4,3)}),
     29:(f"{S}/n29lv-sessions/n29/records.jsonl.gz",{"gen-c1":(1,0)}),
     37:(f"{S}/klc-sessions-a1/n37/records.jsonl.gz",{"sp-c8-d5":(8,5),"sp-c9-d5":(9,5),"sp-c8-d3":(8,3)})}
import os
for n,(path,arms) in src.items():
    if not os.path.exists(path): path=path[:-3]
    by=collections.defaultdict(list)
    for x in recs(path):
        if x["arm"] in arms: by[x["arm"]].append(x)
    inc=list(arms)[0]; ki,di=arms[inc]
    base=[model(x,n,ki,di) for x in by[inc]]
    def S_(L,w): return sum(g+w*u for g,u in L)/len(L)
    print(f"n{n} pilot N={len(base)} base S(w)=",[round(S_(base,w),1) for w in (0,.1,1)])
    for arm,(k,d) in arms.items():
        for m in (0,1,2,3,4,6):
            for lad,mk in ((False,False),(True,True)):
                L=[model(x,n,k,d,m,lad,mk) for x in by[arm]]
                print(f"  {arm:9s} m={m} ladder+minkey={int(lad)}: "+" ".join(f"{S_(L,w)/S_(base,w):.3f}" for w in (0,.1,1)))
