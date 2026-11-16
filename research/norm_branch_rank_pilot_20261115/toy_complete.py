#!/usr/bin/env python3
"""Complete toy DLP recovery on the rank-defect-selected F_1019 base.

This deliberately uses BSGS for factor-base logs and a 256-entry generic
signed-sum table. It validates correctness and charges setup separately;
it is NOT a cheaper cold ECDLP attack or a new exponent.
"""
import json
from math import isqrt
from signed_support import (P,A,B,POINTS,add,neg,group_size)
from cubic_factor_fast import select


def multiply(base, n):
    acc=None
    while n:
        if n&1: acc=add(acc,base)
        base=add(base,base)
        n>>=1
    return acc


def bsgs_logs(gen,targets,order):
    m=isqrt(order)+1
    table={}; point=None
    for i in range(m):
        table.setdefault(point,i)
        point=add(point,gen)
    neg_m=neg(multiply(gen,m))
    out=[];giant_steps=0
    for target in targets:
        t=target
        for j in range(m+1):
            if t in table:
                d=(j*m+table[t])%order
                assert multiply(gen,d)==target
                out.append(d)
                break
            t=add(t,neg_m);giant_steps+=1
        else: raise AssertionError('missing subgroup logarithm')
    return out,dict(baby_entries=len(table),giant_steps=giant_steps)


def gray_signed_table(points,logs,order):
    current=None
    current_scalar=0
    for pt,scalar in zip(points,logs):
        current=add(current,pt);current_scalar=(current_scalar+scalar)%order
    initial_scalar=current_scalar
    doubles=[add(x,x) for x in points]
    result={current:current_scalar}
    prev=0
    for i in range(1,1<<len(points)):
        gray=i^(i>>1)
        delta=gray^prev
        assert delta&(delta-1)==0
        bit=delta.bit_length()-1
        if gray&delta:  # +P => -P: subtract 2P
            current=add(current,neg(doubles[bit]))
            current_scalar=(current_scalar-2*logs[bit])%order
        else:           # -P => +P: add 2P
            current=add(current,doubles[bit])
            current_scalar=(current_scalar+2*logs[bit])%order
        if current in result and result[current]!=current_scalar:
            raise AssertionError('collision with inconsistent public logarithms')
        result[current]=current_scalar
        prev=gray
    assert initial_scalar==sum(logs)%order
    return result


def run():
    order=group_size()
    assert order==977 and all(order%d for d in range(2,isqrt(order)+1))
    gen=POINTS[0]
    assert multiply(gen,order) is None and gen is not None
    check=select(P,[x for x,_ in POINTS[:7]])
    assert POINTS[7][0] in check['noncollision_roots']
    assert check['ranks'][check['noncollision_roots'].index(POINTS[7][0])]==4
    logs,bsgs=bsgs_logs(gen,POINTS,order)
    lookup=gray_signed_table(POINTS,logs,order)
    assert len(lookup)==256
    # Check every possible target, including O, without ever passing a scalar
    # to recover(). Enumerate only to build an independent known-answer set.
    def recover(target):
        shifted=target
        for t in range(order):
            if shifted in lookup:
                d=(lookup[shifted]-t)%order
                assert multiply(gen,d)==target
                return d,t
            shifted=add(shifted,gen)
        raise AssertionError('recovery failed')
    total_shift=max_shift=0
    target=None
    for d in range(order):
        k,t=recover(target)
        assert k==d
        total_shift+=t
        max_shift=max(max_shift,t)
        target=add(target,gen)
    return {'curve':f'F_{P}: y^2=x^3+{A}x+{B}',
            'group_order':order,'rank_defect_6x6':4,
            'factor_base_logs':logs,'bsgs':bsgs,
            'lookup_entries':len(lookup),'gray_table_transitions':255,
            'targets_recovered':order,'wrong_recoveries':0,
            'total_online_shift_adds':total_shift,
            'mean_online_shift_adds':round(total_shift/order,6),
            'max_online_shift_adds':max_shift,
            'interpretation':'full toy solver with reusable generic precomputation; not a cold breakthrough'}


if __name__=='__main__': print(json.dumps(run(),sort_keys=True,indent=2))
