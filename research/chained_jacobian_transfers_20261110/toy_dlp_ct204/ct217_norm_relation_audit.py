#!/usr/bin/env python3
"""CT217: exact norm of cubic-cover split relations and a deliberately fully priced
toy norm-log recovery. This is NOT a genus-4 index-calculus solver.

Uses the separately written CT216 polynomial/Cantor implementation for divisor
arithmetic, but checks its output through elliptic norm sums. All auxiliary
elliptic discrete logarithms are solved explicitly by cold BSGS and charged;
they are never treated as free factor-base logarithms.
"""
import argparse
import json
import math
from collections import Counter

import ct216_genus4_coset_screen as c

P, R = c.P, c.R
G = (2, 37)


def ecneg(q):
    return None if q is None else (q[0], (-q[1]) % P)


def ecmul(k, q):
    out = None
    while k:
        if k & 1:
            out = c.ecadd(out, q)
        q = c.ecadd(q, q)
        k >>= 1
    return out


def ecsum(points):
    out = None
    for pt in points:
        out = c.ecadd(out, pt)
    return out


def norm_point(pt):
    """Pushforward a rational place (x,y) of C via (x,y)->(x^3,y)."""
    q = (pow(pt[0], 3, P), pt[1])
    assert (q[1] ** 2 - q[0] ** 3 - q[0] - 46) % P == 0
    return q


def split_support(d):
    roots, complete = c.roots_count(d[0])
    if not (complete and len(roots) == 4 and len(set(roots)) == 4):
        return None
    pts = [(x, c.evalp(d[1], x)) for x in roots]
    assert all((y*y - c.evalp(c.F, x)) % P == 0 for x, y in pts)
    return pts


def roster():
    first = []
    for x in range(P):
        for y in range(P):
            if (y*y - c.evalp(c.F, x)) % P == 0:
                first.append((x, y))
                break
    assert first[:8] == [(4,19),(6,28),(7,31),(16,33),
                          (17,4),(20,6),(21,25),(23,47)]
    first = first[:8]
    offsets = [('deg1_'+str(i),[first[i]]) for i in range(8)]
    offsets += [('deg2_'+str(i),first[2*i:2*i+2]) for i in range(4)]
    offsets += [('deg3_'+str(i),first[3*i:3*i+3]) for i in range(2)]
    offsets += [('deg4_'+str(i),first[4*i:4*i+4]) for i in range(2)]
    return offsets


def rank_mod(rows, p):
    if not rows:
        return 0
    mat = [[int(a) % p for a in row] for row in rows]
    pivot = 0
    for col in range(len(mat[0])):
        pos = next((i for i in range(pivot, len(mat)) if mat[i][col]), None)
        if pos is None:
            continue
        mat[pivot], mat[pos] = mat[pos], mat[pivot]
        inv = pow(mat[pivot][col], -1, p)
        mat[pivot] = [(x*inv)%p for x in mat[pivot]]
        for i in range(len(mat)):
            if i == pivot or not mat[i][col]:
                continue
            scale = mat[i][col]
            mat[i] = [(x-scale*y)%p for x,y in zip(mat[i],mat[pivot])]
        pivot += 1
        if pivot == len(mat):
            break
    return pivot


def cold_bsgs(q):
    """No target-log oracle, no pre-existing table; charge E additions."""
    m = math.isqrt(R-1) + 1
    babies = {}
    v = None
    additions = 0
    for j in range(m):
        babies[v] = j
        v = c.ecadd(v, G)
        additions += 1
    step = None
    ng = ecneg(G)
    for _ in range(m):
        step = c.ecadd(step, ng)
        additions += 1
    v = q
    for i in range(m+1):
        if v in babies:
            ans = i*m+babies[v]
            assert ans < R and ecmul(ans,G) == q
            return ans, additions
        v = c.ecadd(v, step)
        additions += 1
    raise AssertionError("ECDLP not found")


def perform():
    offsets = roster()
    j = [c.IDENTITY]
    for k in range(1,R):
        j.append(c.cantor(j[-1],c.image(G)))
    assert all(j[k] == c.image(ecmul(k,G)) for k in range(R))
    assert c.cantor(j[-1], c.image(G)) == c.IDENTITY

    # The degree-3 norm/pullback identity gives a *global split projector*
    # on J(C)(F101): beta*alpha=[3], and 3 is invertible on E(F101)
    # because its order is 109. No guessed #J cofactor is required.
    inv_degree = pow(3, -1, R)
    projector_coset_checks = 0
    nonzero_complements = 0
    for _,pts in offsets:
        d0 = c.sums(pts)
        delta = ecsum(map(norm_point, pts))
        projected_point = ecmul(inv_degree, delta)
        projected_div = c.image(projected_point)
        complement = c.cantor(d0, c.neg(projected_div))
        nonzero_complements += int(complement != c.IDENTITY)
        assert c.cantor(complement, projected_div) == d0
        for k,img in enumerate(j):
            # Full equality of divisors, not only equal norms.
            projected_coset = c.image(c.ecadd(projected_point, ecmul(k,G)))
            assert c.cantor(complement, projected_coset) == c.cantor(d0,img)
            projector_coset_checks += 1

    witnesses = []
    omitted_offset_detected = 0
    for name, pts in offsets:
        d0 = c.sums(pts)
        delta = ecsum(map(norm_point,pts))
        for k, img in enumerate(j):
            d = c.cantor(d0,img)
            support = split_support(d)
            if support is None:
                continue
            lhs = c.ecadd(delta,ecmul(3*k,G))
            rhs = ecsum(map(norm_point,support))
            assert lhs == rhs, ("failed norm identity",name,k)
            if delta is not None and rhs != ecmul(3*k,G):
                omitted_offset_detected += 1
            witnesses.append((name,pts,k,support))
    assert len(witnesses) == 33
    assert omitted_offset_detected > 0
    assert not any(name.startswith('deg1_') for name,_,_,_ in witnesses)

    factor_points = sorted({point for _,_,_,support in witnesses for point in support})
    offset_names = sorted({name for name,_,_,_ in witnesses})
    variables = [('factor',p) for p in factor_points] + [('offset',name) for name in offset_names]
    ix = {variable:i for i,variable in enumerate(variables)}
    relations = []
    for name,pts,k,support in witnesses:
        row = [0]*len(variables)
        for point in support:
            row[ix[('factor',point)]] += 1
        row[ix[('offset',name)]] -= 1
        relations.append([v % R for v in row])
    # Solve neither the factor-base logs nor the offset logs by assuming them.
    raw_rank = rank_mod(relations,R)
    assert raw_rank <= len(witnesses)

    # Offsets are explicit sums of cover points, not independent free
    # generators. Eliminate them in a second matrix, substituting their
    # actual support. This prevents inflating a rank deficiency by leaving
    # artificial offset columns unconstrained.
    all_points = sorted({pt for _,pts,_,supp in witnesses
                         for pt in (pts + supp)})
    full_index = {pt:i for i,pt in enumerate(all_points)}
    eliminated_rows = []
    for _,pts,_,support in witnesses:
        row = [0]*len(all_points)
        for pt in support:
            row[full_index[pt]] += 1
        for pt in pts:
            row[full_index[pt]] -= 1
        eliminated_rows.append([v % R for v in row])
    eliminated_rank = rank_mod(eliminated_rows,R)
    assert eliminated_rank <= len(witnesses)

    # Genuine full-target recovery uses only the public T and explicit elliptic
    # BSGS for every factor-base/offset norm logarithm; cost is NOT amortized away.
    target_scalar = 37  # verification fixture ONLY; solver below never reads it
    target = ecmul(target_scalar,G)
    jt = c.image(target)
    candidates = 0
    jac_additions = 0
    chosen = None
    for name,pts in offsets:
        if name.startswith('deg1_'):
            continue
        d0 = c.sums(pts)
        start = c.cantor(d0,jt)
        jac_additions += len(pts)+1
        for a in range(R):
            candidates += 1
            trial = c.cantor(start,j[a])
            jac_additions += 1
            support = split_support(trial)
            if support is not None:
                chosen = (name,pts,a,support)
                break
        if chosen:
            break
    assert chosen is not None
    name,pts,a,support = chosen
    # No free logs; each distinct norm image is solved from scratch by cold BSGS.
    needed = [norm_point(z) for z in support] + [norm_point(z) for z in pts]
    cache = {}
    elliptic_additions = 0
    for q in needed:
        if q not in cache:
            cache[q],cost = cold_bsgs(q)
            elliptic_additions += cost
    numerator = sum(cache[norm_point(z)] for z in support)
    numerator -= sum(cache[norm_point(z)] for z in pts)
    recovered = ((numerator - 3*a)*pow(3,-1,R)) % R
    assert ecmul(recovered,G) == target and recovered == target_scalar
    direct_log, direct_additions = cold_bsgs(target)
    assert direct_log == recovered

    return {
        'schema':'ct217-cubic-norm-relation-audit-v1',
        'pcc':'unverified',
        'field':P, 'source_order':R,
        'norm_pullback_multiplier':3,
        'projector_coset_identities_verified':projector_coset_checks,
        'nonzero_kernel_complements_in_offsets':nonzero_complements,
        'offset_cosets':len(offsets),
        'squarefree_split_witnesses':len(witnesses),
        'offset_omission_detected':omitted_offset_detected,
        'factor_base_points_in_witnesses':len(factor_points),
        'offset_variables_in_witnesses':len(offset_names),
        'relation_unknowns':len(variables),
        'relation_row_rank_mod_109':raw_rank,
        'relation_nullity':len(variables)-raw_rank,
        'eliminated_offset_factor_symbols':len(all_points),
        'eliminated_offset_rank_mod_109':eliminated_rank,
        'eliminated_offset_nullity':len(all_points)-eliminated_rank,
        'toy_target':list(target),
        'toy_recovered_scalar':recovered,
        'toy_offset':name, 'toy_walk_coefficient':a,
        'toy_candidates_tested':candidates,
        'toy_jacobian_additions_lower_bound':jac_additions,
        'toy_factor_norm_logs_solved_by_cold_ec_bsgs':len(cache),
        'toy_factor_norm_bsgs_ec_additions':elliptic_additions,
        'toy_direct_bsgs_ec_additions':direct_additions,
        'standing':'complete toy norm-identity recovery; factor norm logs themselves require elliptic DLP; no IC or speedup',
    }


if __name__=='__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    result = perform()
    if args.self_test:
        assert result['toy_recovered_scalar'] == 37
        assert result['relation_nullity'] > 0
        assert result['eliminated_offset_nullity'] > 0
        assert result['projector_coset_identities_verified'] == 16*R
        assert result['toy_factor_norm_bsgs_ec_additions'] >= result['toy_direct_bsgs_ec_additions']
        print('CT217_SELFTEST_PASS',result['squarefree_split_witnesses'],
              'norm witnesses, rank',result['relation_row_rank_mod_109'])
    else:
        print(json.dumps(result, sort_keys=True, indent=2))
