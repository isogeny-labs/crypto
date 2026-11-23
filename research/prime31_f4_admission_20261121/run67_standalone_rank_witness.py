#!/usr/bin/env python3
"""Prime31 run67: direct 677-coordinate rank test for frozen signed 3-sum witnesses.

Usage: python3 run67_standalone_rank_witness.py run57_basis.json hybrid_witness.csv
The witness CSV is a generic grader's output, never a non-generic solver oracle.
The 0/100/400/900 added pair equations are synthetic rank-model probes, not
actual curve relations; they have no supplied true scalar right-hand sides.
"""
import csv
import json
import random
import sys

N=2_147_446_831
B=677

def insert(piv,row):
    row={k:v%N for k,v in row.items() if v%N}
    while row:
        k=min(row);a=row[k]
        if k not in piv:
            z=pow(a,-1,N)
            piv[k]={j:(z*v)%N for j,v in row.items()}
            return True
        for j,v in piv[k].items():
            t=(row.get(j,0)-a*v)%N
            if t:row[j]=t
            else:row.pop(j,None)
    return False

def member(piv,row):
    row={k:v%N for k,v in row.items() if v%N}
    while row:
        k=min(row);a=row[k]
        if k not in piv:return False
        for j,v in piv[k].items():
            t=(row.get(j,0)-a*v)%N
            if t:row[j]=t
            else:row.pop(j,None)
    return True

def coefficients(w):
    d={}
    for x in w:
        if x==0 or abs(x)>B:raise ValueError('Invalid factor-base witness')
        i=abs(x)-1
        d[i]=(d.get(i,0)+(1 if x>0 else -1))%N
    return {j:v for j,v in d.items() if v}

def evaluate(fixture,witness,m):
    assert fixture['base_points']==[509,677]
    assert len(fixture['rows']['hybrid'])==88
    assert fixture['curve']['p']==2_147_483_647 and fixture['curve']['n']==N
    piv={}
    for encoded in fixture['rows']['hybrid']:
        d={}
        for i,c in zip(encoded[::2],encoded[1::2]):
            d[i]=(d.get(i,0)+c)%N
        insert(piv,d)
    assert len(piv)==88
    rng=random.Random(0x58ac16e3+104729)
    for _ in range(m):
        i=rng.randrange(B);j=rng.randrange(B)
        a=1 if rng.getrandbits(1) else -1
        b=1 if rng.getrandbits(1) else -1
        if i==j and a==-b:b=a
        d={i:a%N};d[j]=(d.get(j,0)+b)%N
        insert(piv,d)
    hits={'sha256_hash_to_curve':0,'frozen_8leaf_corpus':0}
    positives={'sha256_hash_to_curve':0,'frozen_8leaf_corpus':0}
    rows={'sha256_hash_to_curve':0,'frozen_8leaf_corpus':0}
    for r in witness:
        co=r['cohort'];rows[co]+=1
        if int(r['hit3']):
            positives[co]+=1
            vec=coefficients([int(r[f'w{i}']) for i in range(3)])
            hits[co]+=member(piv,vec)
    assert all(v==4096 for v in rows.values())
    assert positives=={'sha256_hash_to_curve':711,'frozen_8leaf_corpus':763}
    return {'m':m,'rank':len(piv),'positive_witnesses':positives,'rank_visible_first_witness':hits}

def main():
    if len(sys.argv)!=3:
        raise SystemExit('usage: script.py run57_basis.json hybrid_witness.csv')
    fixture=json.load(open(sys.argv[1]))
    with open(sys.argv[2],newline='') as f:witness=list(csv.DictReader(f))
    result=[evaluate(fixture,witness,m) for m in (0,100,400,900)]
    assert [r['rank_visible_first_witness']['sha256_hash_to_curve'] for r in result]==[0,0,76,573]
    assert [r['rank_visible_first_witness']['frozen_8leaf_corpus'] for r in result]==[0,0,108,642]
    assert [r['rank'] for r in result]==[88,188,488,642]
    print(json.dumps({'classification':'synthetic quotient-span control; no blind oracle, no ECDLP log', 'results':result,'status':'PASS'},indent=2))

if __name__=='__main__':main()
