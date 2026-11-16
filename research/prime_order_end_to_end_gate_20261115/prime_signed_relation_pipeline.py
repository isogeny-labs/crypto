#!/usr/bin/env python3
"""Cold, complete prime-order ECDLP test for a fixed 8-sign relation oracle.

Enumerating 2^8 signed sums is NOT a sparse-PKM m=8 solver. This script
provides a faithful toy end-to-end control (independent rows, unknown logs,
public target only, complete source/target recovery) for relation economics.
It never receives the target's discrete logarithm during solve().
All results are producer-local [pcc:unverified].
"""
from dataclasses import dataclass
import itertools
import json
import math
import hashlib

P,A,B,R = 1019,46,158,977
G=(944,936)
BASE=[(243,525),(432,632),(225,527),(627,518),
      (387,878),(914,704),(764,816),(34,48)]

@dataclass
class Counter:
    ops: int = 0
    invs: int = 0

class Group:
    def __init__(self):
        self.ctr=Counter()
    def add(self, a, b):
        if a is None: return b
        if b is None: return a
        self.ctr.ops+=1
        x,y=a; u,v=b
        if x==u and (y+v)%P==0: return None
        if x==u:
            assert y==v
            if y==0: return None
            num=(3*x*x+A)%P; den=(2*y)%P
        else:
            num=(v-y)%P; den=(u-x)%P
        self.ctr.invs+=1
        lam=num*pow(den,-1,P)%P
        z=(lam*lam-x-u)%P
        return (z,(lam*(x-z)-y)%P)
    @staticmethod
    def neg(a):
        return None if a is None else (a[0],(-a[1])%P)
    def mul(self, k, q):
        assert k>=0
        out=None
        while k:
            if k&1: out=self.add(out,q)
            q=self.add(q,q)
            k//=2
        return out


def row_rank(rows):
    # Pure F_R matrix rank without calling the group arithmetic oracle.
    arr=[list(row) for row in rows]
    if not arr: return 0
    k=0; d=len(arr[0]); n=len(arr)
    for j in range(d):
        t=next((i for i in range(k,n) if arr[i][j]%R),None)
        if t is None: continue
        arr[k],arr[t]=arr[t],arr[k]
        inv=pow(arr[k][j],-1,R)
        arr[k]=[(a*inv)%R for a in arr[k]]
        for i in range(k+1,n):
            f=arr[i][j]
            if f: arr[i]=[(a-f*b)%R for a,b in zip(arr[i],arr[k])]
        k+=1
        if k==n: break
    return k


def solve_linear(rows, rhs):
    assert len(rows)==len(rhs)==len(BASE)
    m=[list(r)+[v] for r,v in zip(rows,rhs)]
    for j in range(len(BASE)):
        pivot=next((k for k in range(j,len(BASE)) if m[k][j]%R),None)
        assert pivot is not None
        m[j],m[pivot]=m[pivot],m[j]
        inv=pow(m[j][j],-1,R)
        m[j]=[(x*inv)%R for x in m[j]]
        for k in range(len(BASE)):
            if j!=k:
                v=m[k][j]
                if v: m[k]=[(x-v*y)%R for x,y in zip(m[k],m[j])]
    assert all(m[i][:len(BASE)]==[int(i==j) for j in range(len(BASE))]
               for i in range(len(BASE)))
    return [m[i][-1] for i in range(len(BASE))]


def make_table(group):
    two=[group.add(q,q) for q in BASE]
    state=None
    for pt in BASE: state=group.add(state,group.neg(pt))
    sign=[-1]*len(BASE)
    table={}
    prev=0
    for t in range(1<<len(BASE)):
        gray=t^(t>>1)
        if t:
            flip=(gray^prev).bit_length()-1
            sign[flip]*=-1
            state=group.add(state, two[flip] if sign[flip]==1 else group.neg(two[flip]))
        table.setdefault(state,[]).append(tuple(sign))
        prev=gray
    assert sum(map(len,table.values()))==256
    return table


def solve(group, target):
    assert target is None or target[1]*target[1]%P==(
        target[0]**3+A*target[0]+B)%P
    start=group.ctr.ops
    table=make_table(group)
    table_ops=group.ctr.ops-start
    rows=[]; rhs=[]; step=None; a=0
    relation_trials=0; verified_relation_count=0
    while len(rows)<len(BASE) and relation_trials<R:
        relation_trials+=1
        for s in table.get(step,()):
            verified_relation_count+=1
            if row_rank(rows+[s])>len(rows):
                rows.append(s); rhs.append(a)
        if len(rows)==len(BASE): break
        step=group.add(step,G); a+=1
    if len(rows)!=len(BASE):
        raise RuntimeError('rank-deficient factor-base relation set')
    logs=solve_linear(rows,rhs)
    if not all(group.mul(v,G)==pt for v,pt in zip(logs,BASE)):
        raise AssertionError("factor-base log certificate failed")
    prep_ops=group.ctr.ops-start
    state=target
    shift=0
    for j in range(R):
        signs=table.get(state,())
        if signs:
            d=(sum(e*v for e,v in zip(signs[0],logs))-shift)%R
            if group.mul(d,G)!=target: raise AssertionError("target replay failed")
            return d,dict(total_ops=group.ctr.ops-start,
                          table_ops=table_ops, relation_ops=prep_ops-table_ops,
                          witness_ops=group.ctr.ops-start-prep_ops,
                          support=len(table), independent_rank=len(rows),
                          verified_raw_relations=verified_relation_count,
                          relation_trials=relation_trials, target_trials=j+1)
        state=group.add(state,G)
        shift+=1
    raise AssertionError('full cyclic walk should encounter signed support')


def bsgs(group,target):
    before=group.ctr.ops
    m=math.isqrt(R)
    if m*m<R: m+=1
    baby={}; state=None
    for j in range(m):
        baby.setdefault(state,j)
        state=group.add(state,G)
    step=group.neg(group.mul(m,G))
    state=target
    for i in range(m+1):
        if state in baby:
            d=(i*m+baby[state])%R
            if group.mul(d,G)!=target: raise AssertionError("target replay failed")
            return d, group.ctr.ops-before
        state=group.add(state,step)
    raise RuntimeError('BSGS failure')


def main():
    g=Group()
    assert (4*A**3+27*B*B)%P and g.mul(R,G) is None
    assert g.mul(1,G)==G
    assert len(set(x for x,y in BASE))==8
    assert all(y*y%P==(x*x*x+A*x+B)%P for x,y in BASE)
    # The curve order is exhaustively counted independently of solver.
    n=1
    for x in range(P):
        v=(x**3+A*x+B)%P
        n+=1 if v==0 else (2 if pow(v,(P-1)//2,P)==1 else 0)
    assert n==R
    assert all(R%d for d in range(2,math.isqrt(R)+1))
    targets=range(R)
    records=[]
    for secret in targets:
        target=Group().mul(secret,G)
        gs=Group(); rec,stats=solve(gs,target)
        gb=Group(); ref,cost=bsgs(gb,target)
        assert rec==ref==secret
        records.append(dict(d=secret,signed=stats,bsgs_ops=cost))
    assert len(records)==R
    total_signed=[z['signed']['total_ops'] for z in records]
    total_bsgs=[z['bsgs_ops'] for z in records]
    trial_stream=json.dumps(records,sort_keys=True,separators=(',',':')).encode()
    return dict(standing='[pcc:unverified]',field=P,curve=[A,B],
        group_order=R,subgroup_order=R,base_size=8,targets=R,
        target_scalar_passed_to_solver=False,
        test_record_sha256=hashlib.sha256(trial_stream).hexdigest(),
        first_five_tests=records[:5],
        signed_op_min=min(total_signed),signed_op_max=max(total_signed),
        bsgs_op_min=min(total_bsgs),bsgs_op_max=max(total_bsgs),
        mean_signed_ops=sum(x['signed']['total_ops'] for x in records)/len(records),
        mean_bsgs_ops=sum(x['bsgs_ops'] for x in records)/len(records),
        complete_if_independent_rank=8,
        scope='toy enumeration-backed sign oracle, not PKM Gröbner oracle; affine nontrivial add counted; matrix field ops uncharged')

if __name__=='__main__':
    print(json.dumps(main(),indent=2,sort_keys=True))
