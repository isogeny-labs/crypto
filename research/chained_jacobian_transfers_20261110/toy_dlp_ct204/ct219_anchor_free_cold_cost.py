#!/usr/bin/env python3
"""CT219: anchor-free complete *toy* cubic-cover index-calculus cost diagnostic.

F_101 cubic cover C: y^2=x^9+x^3+46 -> E: y^2=z^3+z+46.
54 signed rational place symbols are solved solely from Cantor smoothness
relations and known walk coefficients, with NO source-curve DLP subroutine in
index calculus. Source E arithmetic is used only to validate results and
run a charged cold direct BSGS comparator. No real-world speedup claim.
"""
import argparse
import json
import math
import statistics
import time
import tracemalloc
from itertools import combinations

import ct216_genus4_coset_screen as c

P, R, G = c.P, c.R, (2, 37)


def ec_mul(k, pt):
    out = None
    while k:
        if k & 1:
            out = c.ecadd(out, pt)
        pt = c.ecadd(pt, pt)
        k >>= 1
    return out


def cold_bsgs(q):
    m = math.isqrt(R-1) + 1
    babies = {}
    v = None
    additions = 0
    for j in range(m):
        babies[v] = j
        v = c.ecadd(v, G)
        additions += 1
    step = None
    neg_g = (G[0], -G[1] % P)
    for _ in range(m):
        step = c.ecadd(step, neg_g)
        additions += 1
    v = q
    for i in range(m+1):
        if v in babies:
            ans = i*m + babies[v]
            assert ans < R and ec_mul(ans,G) == q
            return ans, additions, len(babies)
        v = c.ecadd(v, step)
        additions += 1
    raise AssertionError('BSGS failure')


def variables():
    pts = [(x,y) for x in range(P) for y in range(P)
           if (y*y-c.evalp(c.F,x))%P == 0]
    assert len(pts) == 108
    xs = sorted({x for x,_ in pts})
    assert len(xs) == 54 and all(sum(p[0]==x for p in pts)==2 for x in xs)
    first = [(x,min(y for xx,y in pts if xx==x)) for x in xs[:8]]
    return xs, first


def signed_row(pts, ix, size):
    row = [0]*size
    for x,y in pts:
        assert y and 2*y != P
        row[ix[x]] += 1 if y < P-y else -1
    return row


def eliminate(pivots, row, rhs, size):
    row = [a%R for a in row]+[rhs%R]
    for i,prow in sorted(pivots.items()):
        if row[i]:
            f = row[i]
            row = [(a-f*b)%R for a,b in zip(row,prow)]
    for i in range(size):
        if row[i]:
            inv = pow(row[i],-1,R)
            pivots[i] = [(a*inv)%R for a in row]
            return True
    assert row[-1] == 0, 'inconsistent norm relation'
    return False


def solve(pivots, size):
    assert len(pivots) == size
    v = [0]*size
    for i,pr in sorted(pivots.items(),reverse=True):
        v[i] = (pr[-1]-sum(pr[j]*v[j] for j in range(i+1,size)))%R
    return v


def get_support(div):
    if c.degree(div[0]) != 4:
        return None
    roots,complete = c.roots_count(div[0])
    if not complete or len(roots)!=4 or len(set(roots))!=4:
        return None
    pts = [(x,c.evalp(div[1],x)) for x in roots]
    assert all((y*y-c.evalp(c.F,x))%P==0 for x,y in pts)
    return pts


def recover_public_target(target, fb, ix, offset, max_trials=R):
    """No answer lookup, no source E DLP routine, fresh Jacobian walk."""
    pts = list(offset)
    cur = c.cantor(c.sums(pts),c.image(target))
    steps = len(pts)+1
    gdiv = c.image(G)
    trials = 0
    for a in range(max_trials):
        trials += 1
        support = get_support(cur)
        if support is not None:
            row = signed_row(support,ix,len(fb))
            off = signed_row(pts,ix,len(fb))
            scalar = ((sum((x-y)*z for x,y,z in zip(row,off,fb))*pow(3,-1,R))-a)%R
            assert ec_mul(scalar,G) == target, 'invalid recovered scalar'
            return scalar,trials,steps
        cur = c.cantor(cur,gdiv)
        steps += 1
    raise AssertionError('target coset lacks a split witness')


def run(order="candidate-first", verify_all=False):
    start = time.perf_counter_ns()
    tracemalloc.start()
    if order == "direct-first":
        td0 = time.perf_counter_ns()
        direct_scalar,direct_additions,baby_size = cold_bsgs((64,19))
        td1 = time.perf_counter_ns()
    else:
        td0 = td1 = None
    xs,first = variables()
    n = len(xs)
    ix = {x:i for i,x in enumerate(xs)}
    all_offsets = [pair for degree in (2,3,4)
                   for pair in combinations(first,degree)]
    pivots = {}
    witness_count = candidate_count = jac_additions = used_offsets = 0
    exact_relations = []
    for pts in all_offsets:
        used_offsets += 1
        cur = c.sums(pts)
        jac_additions += len(pts)
        for k in range(R):
            candidate_count += 1
            support = get_support(cur)
            if support is not None:
                witness_count += 1
                lhs = signed_row(support,ix,n)
                rhs = signed_row(pts,ix,n)
                relation = [x-y for x,y in zip(lhs,rhs)]
                exact_relations.append((relation,3*k%R))
                eliminate(pivots,relation,3*k,n)
                if len(pivots) == n:
                    break
            if k<R-1:
                cur = c.cantor(cur,c.image(G))
                jac_additions += 1
        if len(pivots) == n:
            break
    t_rel = time.perf_counter_ns()
    logs = solve(pivots,n)
    for row,rhs in exact_relations:
        assert sum(a*b for a,b in zip(row,logs))%R == rhs
    t_la = time.perf_counter_ns()
    target = (64,19)  # public target; scalar 37 is not read by solver
    scalar,trials,rec_additions = recover_public_target(target,logs,ix,all_offsets[0])
    t_recovery = time.perf_counter_ns()
    assert scalar == 37 and ec_mul(scalar,G) == target
    for x,val in zip(xs,logs):
        y = next(y for xx,y in first if xx==x) if x in dict(first) else None
        if y is None:
            y = min(yy for yy in range(P) if (yy*yy-c.evalp(c.F,x))%P==0)
        assert ec_mul(val,G) == (pow(x,3,P),y)
    t_verify = time.perf_counter_ns()
    if order == "candidate-first":
        td0 = time.perf_counter_ns()
        direct_scalar,direct_additions,baby_size = cold_bsgs(target)
        td1 = time.perf_counter_ns()
    assert direct_scalar == scalar
    all_targets_validated = 0
    if verify_all:
        for expected in range(R):
            q = ec_mul(expected,G)
            ans,_,_ = recover_public_target(q,logs,ix,all_offsets[0])
            assert ans == expected
            all_targets_validated += 1
        try:
            eliminate({0:[1]+[0]*(n-1)+[0]},[0]*n,1,n)
        except AssertionError:
            pass
        else:
            raise AssertionError('tampered relation RHS was accepted')
    peak = tracemalloc.get_traced_memory()[1]
    tracemalloc.stop()
    return {
      'schema':'ct219-anchor-free-cold-toy-v1', 'order':order, 'pcc':'unverified',
      'field':P, 'curve':'y2=x9+x3+46', 'source_group_order':R,
      'factor_base_signed_x_variables':n,
      'offsets_used':used_offsets, 'relation_candidates':candidate_count,
      'smooth_relations':witness_count,'relation_rank':len(pivots),
      'relation_cantor_additions':jac_additions,
      'source_ec_dlp_oracles_in_ic':0, 'extra_test_targets_verified':all_targets_validated,
      'target':list(target), 'target_scalar_recovered':scalar,
      'target_candidates':trials,'target_jacobian_additions':rec_additions,
      'direct_cold_bsgs_additions':direct_additions,
      'direct_bsgs_baby_table':baby_size,
      'timing_ns':{
        'relation_collection':t_rel-(td1 if order=='direct-first' else start),
        'linear_algebra':t_la-t_rel,
        'target_recovery':t_recovery-t_la,
        'all_factor_log_independent_verification':t_verify-t_recovery,
        'direct_cold_bsgs':td1-td0,
      },
      'python_tracemalloc_peak_bytes':peak,
      'standing':'complete finite toy transferred relation solve, not scalable or advantageous',
    }


if __name__=='__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test',action='store_true')
    parser.add_argument('--paired',action='store_true')
    args = parser.parse_args()
    if args.paired:
        trials = []
        for i in range(7):
            obs = run('candidate-first' if i%2==0 else 'direct-first')
            total = sum(obs['timing_ns'][k] for k in (
                'relation_collection','linear_algebra','target_recovery',
                'all_factor_log_independent_verification'))
            base = obs['timing_ns']['direct_cold_bsgs']
            trials.append({'round':i+1,'order':obs['order'],
                           'candidate_ns':total,'baseline_ns':base,
                           'baseline_over_candidate':base/total})
        print(json.dumps({
            'schema':'ct219-seven-paired-toy-cost-v1','pcc':'unverified',
            'trials':trials,
            'median_baseline_over_candidate':statistics.median(
                t['baseline_over_candidate'] for t in trials)
        },sort_keys=True,indent=2))
        raise SystemExit(0)
    outcome = run(verify_all=args.self_test)
    assert outcome['relation_rank'] == 54
    assert outcome['source_ec_dlp_oracles_in_ic'] == 0
    assert outcome['target_scalar_recovered'] == 37
    if args.self_test:
        assert outcome['extra_test_targets_verified'] == R
        print('CT219_SELFTEST_PASS',outcome['relation_rank'],
              outcome['smooth_relations'],outcome['relation_candidates'])
    else:
        print(json.dumps(outcome,sort_keys=True,indent=2))
