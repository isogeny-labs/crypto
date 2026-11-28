#!/usr/bin/env python3
"""Prime31 Run58: target-specific signed graph admission, NOT an ECDLP solve."""
from pathlib import Path
from collections import Counter
import json, random, statistics

HERE=Path(__file__).resolve().parent
FIX=json.loads((HERE/"run57_basis.json").read_text())
N=2147446831
M=(0,100,200,300,400,500,600,700,800,900,1000,1200,1500,1800,2300,2800,3500)
TARGETS=(1,32,128,1024)
TRIALS=256
SEED=0x58ac16e3

class Graph:
    def __init__(self,n):
        self.parent=list(range(n))
        self.sign=[1]*n
        self.anchor=[False]*n
    def root(self,i):
        p=self.parent[i]
        if i==p:return i,1
        r,s=self.root(p)
        self.sign[i]*=s
        self.parent[i]=r
        return r,self.sign[i]
    def add(self,i,a,j,b):
        u,x=self.root(i)
        v,y=self.root(j)
        if u==v:
            if a*x+b*y:self.anchor[u]=True
        else:
            self.parent[u]=v
            self.sign[u]=-a*x*b*y
            self.anchor[v]|=self.anchor[u]
    def count(self):
        n=len(self.parent)
        sizes=Counter()
        known=0
        for i in range(n):
            r,_=self.root(i)
            if self.anchor[r]:known+=1
            else:sizes[r]+=1
        favorable=2*known*known+sum(s*(s-1) for s in sizes.values())
        return favorable,2*n*n,known

def draw(rng,n):
    i=rng.randrange(n);j=rng.randrange(n)
    a=1 if rng.getrandbits(1) else -1
    b=1 if rng.getrandbits(1) else -1
    if i==j and a==-b:b=a
    return i,a,j,b

def brute_check(n,seed):
    rng=random.Random(seed)
    g=Graph(n)
    for _ in range(2*n):g.add(*draw(rng,n))
    expected,den,_=g.count()
    good=total=0
    for i in range(n):
        for j in range(i,n):
            for a in (-1,1):
                for b in (-1,1):
                    if i==j and a!=b:continue
                    u,x=g.root(i);v,y=g.root(j)
                    good+=(g.anchor[u] and g.anchor[v]) or (u==v and not g.anchor[u] and a*x+b*y==0)
                    total+=1
    assert total==den==2*n*n
    assert expected==good,(n,seed,expected,good)

def arm(name,n,support,offset):
    probs={m:[] for m in M}
    for trial in range(TRIALS):
        rng=random.Random(SEED+0x9e3779b1*trial+offset)
        g=Graph(n);last=0
        for m in M:
            for _ in range(last,m):g.add(*draw(rng,n))
            good,total,_=g.count()
            probs[m].append(good/total)
            last=m
    rows=[]
    for m in M:
        p=probs[m]
        # E[1/p] is correct for a reused shared base graph;
        # no silent deletion of trials with zero sufficient-event probability.
        wait=(statistics.mean([1/z for z in p]) if all(p) else None)
        rows.append({"base_hits":m,"mean_p":statistics.mean(p),
                     "zero_probability_trials":sum(z==0 for z in p),
                     "target_positive_wait_mean":wait})
    best={}
    for t in TARGETS:
        candidates=[( (r["base_hits"]+t*r["target_positive_wait_mean"])*N/support,r)
                    for r in rows if r["target_positive_wait_mean"] is not None]
        cost,row=min(candidates,key=lambda x:x[0])
        best[str(t)]={"base_positive_hits":row["base_hits"],
                      "estimated_uniform_calls":cost,
                      "target_positive_success_probability":row["mean_p"]}
    return {"base_points":n,"nonidentity_group_support":support,
            "conditional_uniform_hit_probability":support/N,
            "trials":TRIALS,"grid":rows,"best":best}

def main():
    for n in range(4,21):
        for trial in range(3):brute_check(n,SEED+37*n+trial)
    assert FIX["base_points"]==[509,677]
    assert FIX["distinct_sums_including_identity"]==[518085,916469]
    result={"status":"conditional synthetic relation oracle; not a Prime31 ECDLP solve",
            "seed":SEED,"small_exhaustive_graph_checks":51,
            "arms":{"dickson":arm("dickson",509,518084,0),
                    "hybrid":arm("hybrid",677,916468,104729)}}
    for name,data in result["arms"].items():
        for count,info in data["best"].items():
            print(name,count,info)
    (HERE/"RUN58_RESULTS.json").write_text(json.dumps(result,indent=2)+"\n")
    print("PASS 51 exhaustive small-graph checks and 512 deterministic Prime31-support trials")

if __name__=="__main__":main()
