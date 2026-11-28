#!/usr/bin/env python3
"""Independent dense GF(n) validation of the run57 signed-DSU coefficient rank."""
import random
from run57_rank_gate import SignedDSU,N

def dense_rank(rows,n):
    a=[[r.get(j,0)%N for j in range(n)] for r in rows]
    k=0
    for j in range(n):
        found=next((i for i in range(k,len(a)) if a[i][j]),None)
        if found is None:continue
        a[k],a[found]=a[found],a[k]
        inv=pow(a[k][j],N-2,N)
        for col in range(j,n):a[k][col]=a[k][col]*inv%N
        for i in range(len(a)):
            if i==k or not a[i][j]:continue
            factor=a[i][j]
            for col in range(j,n):
                a[i][col]=(a[i][col]-factor*a[k][col])%N
        k+=1
        if k==len(a):break
    return k

def main():
    rng=random.Random(0x570051)
    for n in (2,3,5,8,12):
        for case in range(100):
            d=SignedDSU(n);edges=[];extra=[]
            for _ in range(rng.randrange(n*3+1)):
                i=rng.randrange(n);j=rng.randrange(n)
                s=rng.choice((-1,1));t=rng.choice((-1,1))
                d.add(i,s,j,t)
                row={}
                for z,c in ((i,s),(j,t)):
                    row[z]=(row.get(z,0)+c)%N
                    if not row[z]:del row[z]
                edges.append(row)
            for _ in range(rng.randrange(n*2+1)):
                row={}
                for _ in range(rng.randrange(2,6)):
                    j=rng.randrange(n)
                    row[j]=(row.get(j,0)+rng.choice((-1,1)))%N
                    if not row[j]:del row[j]
                extra.append(row)
            encoded=[]
            for r in extra:
                pairs=[]
                for col,c in r.items():
                    cc=c if c<N//2 else c-N
                    pairs.extend((col,cc))
                encoded.append(pairs)
            assert d.free==n-dense_rank(edges,n),(n,case,'edges')
            assert d.deficiency(encoded,full=True)==n-dense_rank(edges+extra,n),(n,case,'joint')
    print('PASS 500 independent dense GF(n) signed-component rank tests')

if __name__=='__main__':main()
