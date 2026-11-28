#!/usr/bin/env python3
"""Independent exact direct-Gaussian verification of run59 quotient-target counts."""
import random
from run59_free_row_target import Graph, free_span_p, N

def rank(rows):
    pivots = {}
    for original in rows:
        row = {i: v % N for i,v in original.items() if v % N}
        while row:
            j = min(row)
            c = row[j]
            if j not in pivots:
                z = pow(c, -1, N)
                pivots[j] = {i: v*z % N for i,v in row.items()}
                break
            for k,v in pivots[j].items():
                new = (row.get(k,0)-c*v) % N
                if new: row[k]=new
                else:row.pop(k,None)
    return len(pivots)

def main():
    total_cases=0; total_pairs=0
    for seed in range(101):
        rng=random.Random(0x596fa+seed)
        b=rng.randrange(3,12)
        graph=Graph(b); free=[]; all_rows=[]
        for _ in range(rng.randrange(1,2*b)):
            i,j=rng.randrange(b),rng.randrange(b)
            a,c=rng.choice((-1,1)),rng.choice((-1,1))
            if i==j and a!=c:c=a
            graph.add(i,a,j,c)
            r={i:a};r[j]=r.get(j,0)+c
            all_rows.append(r)
        for _ in range(rng.randrange(1,b+1)):
            ids=rng.sample(range(b),min(b,rng.randrange(1,5)))
            r={};raw=[]
            for i in ids:
                x=rng.choice((-2,-1,1,2));raw.extend((i,x));r[i]=x
            free.append(raw);all_rows.append(r)
        actual,_=free_span_p(graph,free)
        base_rank=rank(all_rows)
        good=0
        for i in range(b):
            for j in range(i,b):
                for a in (-1,1):
                    for c in (-1,1):
                        if i==j and a!=c:continue
                        pair={i:a};pair[j]=pair.get(j,0)+c
                        good += (rank(all_rows+[pair]) == base_rank)
                        total_pairs+=1
        assert total_pairs>0
        assert good == round(actual*2*b*b),(seed,b,good,actual)
        total_cases+=1
    print(f"PASS: {total_cases} independent exact modular Gaussian test systems, {total_pairs} signed-pair rank checks.")

if __name__=="__main__":
    main()
