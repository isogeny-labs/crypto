#!/usr/bin/env python3
"""Complete cold prime-field toy DLP with explicit basis-log costs.

Shows the effect of optimizing the signed-table search stride. The stride
optimizer gets an uncharged full scan of 976 group-space permutations, so
signed cost is deliberately optimistic. Producer-local [pcc:unverified].
"""
import json
from itertools import combinations
from math import isqrt

P, A, B = 1019, 46, 158
POINTS = ((944,936),(243,525),(432,632),(225,527),
          (627,518),(387,878),(914,704),(764,816))
G = POINTS[0]
ORDER = 977

class Meter:
    def __init__(self): self.calls = 0
    def add(self, x, y):
        self.calls += 1
        return add(x, y)

def neg(q):
    return None if q is None else (q[0], -q[1] % P)

def add(q, r):
    if q is None: return r
    if r is None: return q
    x,y = q; u,v = r
    if x == u and (y+v)%P == 0: return None
    if x == u:
        assert y != 0
        lam = (3*x*x + A)*pow(2*y, -1, P) % P
    else:
        lam = (v-y)*pow(u-x, -1, P) % P
    xx = (lam*lam-x-u)%P
    yy = (lam*(x-xx)-y)%P
    return (xx,yy)

def mul(q,k,m=None):
    acc=None
    assert k >= 0
    while k:
        if k&1: acc = m.add(acc,q) if m else add(acc,q)
        k >>= 1
        if k: q = m.add(q,q) if m else add(q,q)
    return acc

def group_size():
    return 1+sum(1 if ((x*x*x+A*x+B)%P)==0 else
                 2 if pow((x*x*x+A*x+B)%P,(P-1)//2,P)==1 else 0
                 for x in range(P))

def bsgs_setup():
    m = isqrt(ORDER)+1
    t = Meter()
    baby = {}
    q=None
    for i in range(m):
        baby.setdefault(q,i)
        q=t.add(q,G)
    # q=m*G already computed; no redundant scalar multiplication.
    return baby,neg(q),m,t.calls

def log_bsgs(q,baby,minus_step,m):
    target=q
    t=Meter()
    for j in range(m+1):
        if q in baby:
            d = (j*m+baby[q])%ORDER
            assert mul(G,d)==target
            return d,t.calls
        q=t.add(q,minus_step)
    raise RuntimeError('BSGS failed')

def lookup_build(points,logs):
    t=Meter()
    acc=None; scalar=0
    for pt,d in zip(points,logs):
        acc=t.add(acc,pt)
        scalar=(scalar+d)%ORDER
    double=[t.add(pt,pt) for pt in points]
    table={acc:scalar}; prev=0
    for i in range(1,256):
        gray=i^(i>>1)
        switched=gray^prev
        assert switched>0 and switched&(switched-1)==0
        bit=switched.bit_length()-1
        if gray&switched:
            acc=t.add(acc,neg(double[bit]))
            scalar=(scalar-2*logs[bit])%ORDER
        else:
            acc=t.add(acc,double[bit])
            scalar=(scalar+2*logs[bit])%ORDER
        if acc in table: assert table[acc]==scalar
        table[acc]=scalar
        prev=gray
    return table,t.calls

def stride_stats(keys,s):
    inv=pow(s,-1,ORDER)
    places=sorted((k*inv)%ORDER for k in keys)
    gap=[(places[(i+1)%len(places)]-places[i])%ORDER
         for i in range(len(places))]
    assert sum(gap)==ORDER
    total = sum(d*(d-1)//2 for d in gap)
    return total,max(gap)-1

def stride_search(table):
    support=list(table.values())
    best=None
    for s in range(1,ORDER):
        total,worst=stride_stats(support,s)
        candidate=(total,worst,s)
        if best is None or candidate<best: best=candidate
    return best

def sign_recover(target,table,step,stride):
    meter=Meter()
    q=target
    for j in range(ORDER):
        if q in table:
            d=(table[q]-j*stride)%ORDER
            assert mul(G,d)==target
            return d,meter.calls
        q=meter.add(q,step)
    raise RuntimeError('signed search failed')


def lift(z):
    f=(z*z*z+A*z+B)%P
    if not f: return None
    y=pow(f,(P+1)//4,P)
    return (z,min(y,P-y)) if y*y%P == f else None

def block_rank(z):
    xs=[x for x,y in POINTS[:7]]+[z]
    left=list(combinations(range(4),2))
    right=list(combinations(range(4,8),2))
    def vander4_squared(row_indices):
        ans=1
        for i,j in combinations(row_indices,2):
            ans=ans*(xs[i]-xs[j])**2 % P
        return ans
    mat=[[vander4_squared(a+b) for b in right] for a in left]
    rank=0
    for j in range(6):
        idx=next((k for k in range(rank,6) if mat[k][j]),None)
        if idx is None:continue
        mat[rank],mat[idx]=mat[idx],mat[rank]
        inv=pow(mat[rank][j],-1,P)
        mat[rank]=[t*inv%P for t in mat[rank]]
        for k in range(rank+1,6):
            coefficient=mat[k][j]
            mat[k]=[(a-coefficient*b)%P for a,b in zip(mat[k],mat[rank])]
        rank+=1
    return rank

def control_support_scan():
    support={None}
    for pt in POINTS[:7]:
        support={add(q,signed) for q in support for signed in (pt,neg(pt))}
    assert len(support)==128
    differences={add(u,neg(v)) for u in support for v in support}
    assert len(differences)==879
    controls=[]; maximal=[]
    for z in range(P):
        point=lift(z)
        if not point or z in [x for x,y in POINTS]:continue
        two=add(point,point)
        full_support=two not in differences
        controls.append(z)
        if full_support:
            assert len({add(q,signed) for q in support for signed in (point,neg(point))})==256
            maximal.append((z,point))
    assert len(controls)==480 and len(maximal)==48
    assert add(POINTS[7],POINTS[7]) not in differences
    return support,differences,controls,maximal

def main():
    assert group_size()==ORDER and all(ORDER%d for d in range(2,isqrt(ORDER)+1))
    assert all(y*y%P == (x*x*x+A*x+B)%P for x,y in POINTS)
    assert mul(G,ORDER)==None
    baby,minus_step,m,baby_cost=bsgs_setup()
    logs=[1]
    extra_log_calls=0
    conditional_log_cost=0
    for original in POINTS[1:]:
        d,work=log_bsgs(original,baby,minus_step,m)
        assert mul(G,d)==original
        logs.append(d)
        extra_log_calls+=work
        if original == POINTS[7]: conditional_log_cost=work
    table,table_cost=lookup_build(POINTS,logs)
    assert len(table)==256
    seven,differences,eligible_controls,maximal_controls=control_support_scan()
    assert block_rank(764)==4
    assert all(block_rank(z)==6 for z in eligible_controls)
    best=stride_search(table)
    optimal_total,optimal_max,stride=best
    step_cost=Meter()
    step=mul(G,stride,step_cost)
    assert step is not None
    raw_total,raw_worst=stride_stats(table.values(),1)
    # Verify all 977 public targets with both complete algorithms.
    baseline_online=0; signed_online=0
    q=None
    for d in range(ORDER):
        original=q
        got_b,work_b=log_bsgs(original,baby,minus_step,m)
        got_s,work_s=sign_recover(q,table,step,stride)
        assert got_b == got_s == d
        baseline_online+=work_b
        signed_online+=work_s
        q=add(q,G)
    assert q is None
    assert signed_online==optimal_total
    # Compare a rank-6 control having identical maximal 256 target support.
    control_z,control_point = maximal_controls[0]
    control_log,control_log_cost=log_bsgs(control_point,baby,minus_step,m)
    control_table,control_table_cost=lookup_build(POINTS[:7]+(control_point,),logs[:7]+[control_log])
    assert len(control_table)==256 and block_rank(control_z)==6
    control_online_total,control_worst,control_stride=stride_search(control_table)
    control_step_meter=Meter()
    control_step=mul(G,control_stride,control_step_meter)
    control_online=0
    q=None
    for d in range(ORDER):
        got,work=sign_recover(q,control_table,control_step,control_stride)
        assert got==d
        control_online+=work
        q=add(q,G)
    assert control_online==control_online_total
    control_setup=baby_cost+(extra_log_calls-conditional_log_cost)+control_log_cost+control_table_cost+control_step_meter.calls
    baseline_setup=baby_cost
    signed_setup=baby_cost+extra_log_calls+table_cost+step_cost.calls
    improvement=baseline_online-signed_online
    crossover=(ORDER*(signed_setup-baseline_setup)+improvement-1)//improvement if improvement>0 else None
    result = dict(standing='[pcc:unverified]',p=P,curve=[A,B],group_order=ORDER,
                  all_logs_recovered=ORDER, signed_table_size=len(table),
                  basis_logs=logs,
                  structural_null=dict(seven_point_support=len(seven),
                    seven_difference_set_size=len(differences),
                    eligible_nonconditioned_eighth_abscissae=len(eligible_controls),
                    full_support_rank6_controls=len(maximal_controls),
                    full_support_iff_twice_eighth_point_outside_difference_set=True,
                    conditioned_rank=block_rank(764)),
                  matched_rank6_control=dict(z=control_z,point=control_point,
                    rank=block_rank(control_z),support=len(control_table),
                    optimized_stride=control_stride,optimized_mean_online_add_calls=control_online/ORDER,
                    optimized_worst_add_calls=control_worst,
                    cold_setup_add_calls=control_setup,
                    mean_cold_add_calls=control_setup+control_online/ORDER),
                  baseline_bsgs=dict(baby_entries=len(baby),setup_add_calls=baseline_setup,
                    mean_online_add_calls=baseline_online/ORDER,mean_cold_add_calls=baseline_setup+baseline_online/ORDER),
                  rank_selected_signed=dict(factor_base_log_add_calls=extra_log_calls,
                    table_build_add_calls=table_cost,optimized_stride=stride,
                    stride_multiply_add_calls=step_cost.calls,
                    uncharged_stride_scan_candidates=ORDER-1,
                    unoptimized_stride_mean_online_add_calls=raw_total/ORDER,
                    unoptimized_stride_worst_add_calls=raw_worst,
                    optimized_stride_mean_online_add_calls=signed_online/ORDER,
                    optimized_stride_worst_add_calls=optimal_max,
                    cold_setup_add_calls=signed_setup,
                    mean_cold_add_calls=signed_setup+signed_online/ORDER,
                    optimistic_amortized_break_even_targets=crossover),
                  caveat='Optimized signed stride selected with uncharged O(r|S|log|S|) integer operations. Both omit field-operation pricing, point verification, curve/factor-base/locus selection, memory and CI PCC; favors signed search.')
    print(json.dumps(result,indent=2,sort_keys=True))

if __name__=='__main__':main()
