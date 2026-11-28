#!/usr/bin/env python3
"""Reproducible toy rho comparator, not optimized negation-map rho.

Runs on the prime-order curve in prime_signed_relation_pipeline.py. Exactly 977 public DLP instances checked. Producer-local only.
"""
import json
import random
import statistics
from prime_signed_relation_pipeline import Group, G, R


def next_state(group, state, target):
    X,a,b = state
    branch = 0 if X is None else X[0]%3
    if branch==0:
        return group.add(X,G),(a+1)%R,b
    if branch==1:
        return group.add(X,X),(2*a)%R,(2*b)%R
    return group.add(X,target),a,(b+1)%R


def rho(group,target):
    seed=4188+(0 if target is None else target[0])
    rng=random.Random(seed)
    restarts=0
    for restart in range(128):
        restarts=restart
        a=rng.randrange(R);b=rng.randrange(R)
        X=group.add(group.mul(a,G),group.mul(b,target))
        tortoise=(X,a,b)
        hare=tortoise
        for i in range(8*R):
            tortoise=next_state(group,tortoise,target)
            hare=next_state(group,next_state(group,hare,target),target)
            if tortoise[0]!=hare[0]:continue
            da=(tortoise[1]-hare[1])%R
            db=(hare[2]-tortoise[2])%R
            if db==0:break
            d=da*pow(db,-1,R)%R
            if group.mul(d,G)==target:
                return d,restart,i+1
            break
    raise RuntimeError('rho restarts exhausted')


def main():
    records=[]
    for d in range(R):
        q=Group().mul(d,G)
        group=Group()
        got,restarts,steps=rho(group,q)
        assert got==d
        records.append((d,group.ctr.ops,restarts,steps))
    return dict(standing='[pcc:unverified]',curve='E/F_1019: y²=x³+46x+158',
       group_order=R,verified_targets=len(records),
       measured_baseline='three-way Floyd rho with public-target-determined starts; not optimized negation rho',
       mean_point_ops=statistics.mean(r[1] for r in records),
       median_point_ops=statistics.median(r[1] for r in records),
       max_point_ops=max(r[1] for r in records),
       total_restarts=sum(r[2] for r in records),
       max_restarts=max(r[2] for r in records),
       mean_rho_iterations=statistics.mean(r[3] for r in records),
       note='per-target cold start; counts initialization and verification, not hashing or field elimination')

if __name__=='__main__':print(json.dumps(main(),sort_keys=True,indent=2))
