#!/usr/bin/env python3
"""Conditional full-rank two-point relation sampling, Prime31. NOT an ECDLP solver."""
from pathlib import Path
import json,random,statistics

N=2147446831
FIX=json.loads((Path(__file__).parent/'run57_basis.json').read_text())

class SignedDSU:
    def __init__(self,n):
        self.parent=list(range(n));self.sign=[1]*n;self.anchor=[False]*n;self.free=n
    def root(self,i):
        p=self.parent[i]
        if p==i:return i,1
        j,s=self.root(p);self.sign[i]*=s;self.parent[i]=j
        return j,self.sign[i]
    def add(self,i,a,j,b):
        u,x=self.root(i);v,y=self.root(j)
        if u==v:
            if a*x+b*y and not self.anchor[u]:self.anchor[u]=True;self.free-=1
            return
        old=(not self.anchor[u])+(not self.anchor[v])
        self.parent[u]=v;self.sign[u]=-a*x*b*y
        self.anchor[v]|=self.anchor[u]
        self.free+=int(not self.anchor[v])-old
    def deficiency(self,free_rows,full=False):
        if not full and self.free>len(free_rows):return self.free-len(free_rows)
        piv={}
        for encoded in free_rows:
            row={}
            for i,c in zip(encoded[::2],encoded[1::2]):
                u,s=self.root(i)
                if self.anchor[u]:continue
                z=(row.get(u,0)+s*c)%N
                if z:row[u]=z
                else:row.pop(u,None)
            while row:
                i=min(row)
                if i not in piv:
                    scale=pow(row[i],-1,N)
                    piv[i]={k:v*scale%N for k,v in row.items()}
                    break
                a=row[i]
                for k,v in piv[i].items():
                    z=(row.get(k,0)-a*v)%N
                    if z:row[k]=z
                    else:row.pop(k,None)
        return self.free-len(piv)

def deficiency_at(n,free_rows,pairs,k,full=False):
    d=SignedDSU(n)
    for i,a,j,b in pairs[:k]:d.add(i,a,j,b)
    return d.deficiency(free_rows,full)

def experiment(n,free_rows,seed):
    rng=random.Random(seed);pairs=[]
    for _ in range(8000):
        i=rng.randrange(n);j=rng.randrange(n)
        a=1 if rng.getrandbits(1) else -1
        b=1 if rng.getrandbits(1) else -1
        if i==j and a==-b:b=a
        pairs.append((i,a,j,b))
    assert deficiency_at(n,free_rows,pairs,8000)==0
    lo=0;hi=8000
    while lo+1<hi:
        mid=(lo+hi)//2
        if deficiency_at(n,free_rows,pairs,mid):lo=mid
        else:hi=mid
    assert deficiency_at(n,free_rows,pairs,hi-1)>0
    return hi

def main():
    results={}
    for ix,name in enumerate(['dickson','hybrid']):
        n=FIX['base_points'][ix];rows=FIX['rows'][name]
        hits=[experiment(n,rows,0x13579bdf+i*7919) for i in range(256)]
        naive=n-len(rows);residual=[]
        for i in range(256):
            rng=random.Random(0x2468ace0+i*6073);pairs=[]
            for _ in range(naive):
                x=rng.randrange(n);y=rng.randrange(n)
                a=1 if rng.getrandbits(1) else -1
                b=1 if rng.getrandbits(1) else -1
                if x==y and a==-b:b=a
                pairs.append((x,a,y,b))
            residual.append(deficiency_at(n,rows,pairs,naive,full=True))
        nonzero=FIX['distinct_sums_including_identity'][ix]-1
        results[name]={'points':n,'free_rank':len(rows),'positive_hits_required':hits,
            'mean_positive_hits':statistics.mean(hits),'median_positive_hits':statistics.median(hits),
            'mean_rank_deficiency_at_naive_bound':statistics.mean(residual),
            'mean_uniform_calls':statistics.mean(hits)*N/nonzero,
            'naive_calls':naive*N/nonzero}
    a=results['dickson']['mean_uniform_calls'];b=results['hybrid']['mean_uniform_calls']
    results['comparison']={'hybrid_calls_vs_dickson':b/a,'max_hybrid_oracle_cost_multiplier':a/b}
    print(json.dumps(results,indent=2))
    (Path(__file__).parent/'run57_compact_results.json').write_text(json.dumps(results,indent=2)+'\n')
if __name__=='__main__':main()
