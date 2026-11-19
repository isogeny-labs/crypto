#!/usr/bin/env python3
"""CT240: F103 norm-orbit reduced bases with exact signed two-LP elimination.

Finite public known-answer *feasibility* screen, NOT cold native timings.
Source fixture and roster are CT238; raw Cantor/E group arithmetic is CT223/237.
Precommitted protocol: CT240_F103_REDUCED_NORM_LP_PROTOCOL.md.
No ECDLP table, scalar oracle, or secret input is used to form any equation.
"""
import json
import sys

import ct238_p103_norm_fiber_pilot as N

P, R, B, G, Q = N.P, N.R, N.B, N.G, N.Q
T, C = N.T, N.C
BASES = (12, 16, 18, 20, 22)
CAP = 60 * R

def require(ok, msg):
    if not ok:
        raise ValueError(msg)

def norm_point(x):
    ys = [y for y in range(P) if y*y % P == (pow(x, 3, P) + x + B) % P]
    require(len(ys) == 2, 'missing/nonpaired E norm point')
    return (x, min(ys))

def norm_relation_check(row, xs, rhs, pts):
    """Independent E group evaluation of the complete norm coefficient row."""
    lhs = None
    for x, a in zip(xs, row):
        if a % R:
            lhs = T.eadd(lhs, T.emul(a % R, pts[x]))
    return lhs == T.emul(rhs % R, G)

def make_stream():
    require((P, R, B, G, Q) == (103, 109, 24, (1, 51), (42, 71)), 'wrong fixture')
    require(N.emul_raw(R, G) is None and N.emul_raw(73, G) == Q, 'wrong subgroup')
    all_u, first, ys, xs = N.roster()
    require(len(xs) == len(set(xs)) == 22, 'duplicate norm orbit')
    points = {x: norm_point(x) for x in xs}
    os = T.offsets(first)[:60]
    gdiv = T.dimage(G)
    stream = []
    candidates = 0
    for off_id, offset in enumerate(os):
        cur = C.ID
        off = {x: 0 for x in xs}
        for u, y in offset:
            cur = C.cantor(cur, T.dpoint((u, y)))
            off[pow(u, 3, P)] += 1 if y < P-y else -1
        for k in range(R):
            if candidates == CAP:
                break
            candidates += 1
            signed = N.support(cur, all_u)
            if signed is not None:
                # CT237 independent group equation before compression.
                T.validate_raw(signed, offset, k, ys)
                row = dict.fromkeys(xs, 0)
                for u, sgn in signed.items():
                    row[pow(u, 3, P)] += sgn
                coefficients = [row[x] - off[x] for x in xs]
                rhs = 3*k % R
                require(norm_relation_check(coefficients, xs, rhs, points),
                        'compressed norm coefficient/group identity drift')
                stream.append((candidates, off_id, k, coefficients, rhs))
            if k < R-1 and candidates < CAP:
                cur = C.cantor(cur, gdiv)
        if candidates == CAP:
            break
    require(candidates == CAP, 'incomplete precommitted relation budget')
    return all_u, xs, points, os, stream, candidates

def target_solve(base_size, xs, points, offsets, all_u, fb, lp):
    """Recover from public Q using only determined FB and LP logs."""
    gdiv = T.dimage(G)
    qdiv = T.dimage(Q)
    known = fb + lp
    attempts = 0
    for pts in offsets:
        cur = qdiv
        off = {x: 0 for x in xs}
        for u, y in pts:
            cur = C.cantor(cur, T.dpoint((u, y)))
            off[pow(u, 3, P)] += 1 if y < P-y else -1
        for a in range(R):
            if attempts >= CAP:
                return None, attempts, None
            attempts += 1
            signed = N.support(cur, all_u)
            if signed is not None:
                row = dict.fromkeys(xs, 0)
                for u, sign in signed.items():
                    row[pow(u, 3, P)] += sign
                coeff = [row[x] - off[x] for x in xs]
                # Do not accept a relation involving unknown excluded columns.
                if all(v % R == 0 or l is not None for v, l in zip(coeff, known)):
                    require(norm_relation_check(coeff, xs, (3*a + 3*73) % R, points),
                            'target raw elliptic norm equation mismatch')
                    total = sum(v*l for v, l in zip(coeff, known) if v % R)
                    log = (total*pow(3, -1, R) - a) % R
                    require(T.emul(log, G) == Q, 'incorrect target scalar')
                    require(C.sd(T.dimage(G), log) == qdiv, 'incorrect divisor pullback')
                    return log, attempts, a
            if a < R-1 and attempts < CAP:
                cur = C.cantor(cur, gdiv)
    return None, attempts, None

def evaluate(base_size, all_u, xs, points, offsets, stream):
    require(0 < base_size <= len(xs), 'bad base size')
    fb_pivots = [None] * base_size
    lp_pivots = [None] * (len(xs)-base_size)
    full = one_lp = two_lp = rejected = b_only = rank = 0
    used = 0
    for cand, off_id, k, row, rhs in stream:
        nlp = sum(v % R != 0 for v in row[base_size:])
        if nlp > 2:
            rejected += 1
            continue
        if nlp == 0:
            full += 1
        elif nlp == 1:
            one_lp += 1
        else:
            two_lp += 1
        residual = T.lp_insert(lp_pivots, row[:base_size], row[base_size:], rhs)
        if residual is not None:
            b_only += 1
            b_row, b_rhs = residual
            if T.reduce_row(fb_pivots, b_row, b_rhs):
                rank += 1
                if rank == base_size:
                    used = cand
                    break
    else:
        used = CAP
    fb = lp = None
    log = None
    target_attempts = 0
    target_offset = None
    recovered_lp = 0
    if rank == base_size:
        fb = T.solve(fb_pivots)
        for x, a in zip(xs[:base_size], fb):
            require(T.emul(a, G) == points[x], 'wrong base discrete logarithm')
        lp = T.recover_lp(lp_pivots, fb)
        for x, a in zip(xs[base_size:], lp):
            if a is not None:
                recovered_lp += 1
                require(T.emul(a, G) == points[x], 'wrong recovered LP logarithm')
        log, target_attempts, target_offset = target_solve(
            base_size, xs, points, offsets, all_u, fb, lp)
    return dict(schema='ct240-f103-reduced-norm-lp-feasibility-v1',
                basis=base_size, excluded_columns=22-base_size,
                rank=rank, relation_candidates=used,
                full=full, single_lp=one_lp, double_lp=two_lp,
                rejected_multi_lp=rejected, b_only_equations=b_only,
                lp_pivots=sum(z is not None for z in lp_pivots),
                recovered_lp=recovered_lp, target_attempts=target_attempts,
                target_offset=target_offset, recovered_log=log,
                complete=log is not None, model='exact_python_public_toy',
                cold_wall='not_measured', peak_memory='not_measured',
                pcc='unverified')

def mutation_tests(xs, stream, points):
    require(len(xs) == len(set(xs)), 'duplicate norm symbol accepted')
    require(N.emul_raw(74, G) != Q, 'invalid public known-answer control')
    r = next(row for _, _, _, row, _ in stream if any(v % R for v in row))
    rhs = next(rhs for _, _, _, row, rhs in stream if any(v % R for v in row))
    require(norm_relation_check(r, xs, rhs, points), 'test witness invalid')
    mutant = r[:]
    mutant[0] += 1
    require(not norm_relation_check(mutant, xs, rhs, points),
            'corrupt norm-coefficient control unexpectedly passed')
    require(not norm_relation_check(r, xs, (rhs+1) % R, points),
            'corrupt source-right-hand-side control unexpectedly passed')
    require(len(xs+[xs[0]]) != len(set(xs+[xs[0]])),
            'duplicate roster mutant unexpectedly passed')

def main():
    all_u, xs, points, offsets, stream, candidates = make_stream()
    mutation_tests(xs, stream, points)
    print('CT240_STREAM', json.dumps(dict(candidates=candidates,
          split_relations=len(stream), norm_columns=len(xs),
          verified_raw_relations=len(stream), mutation_controls=4),
          sort_keys=True), flush=True)
    rows = []
    for b in BASES:
        row = evaluate(b, all_u, xs, points, offsets, stream)
        print('CT240_RESULT', json.dumps(row, sort_keys=True), flush=True)
        rows.append(row)
    control = rows[-1]
    require((control['rank'], control['relation_candidates'], control['full'],
             control['recovered_log'], control['target_attempts']) ==
            (22, 381, 27, 73, 27), 'CT238 unchanged B22 control mismatch')
    require(all(z['relation_candidates'] <= CAP for z in rows), 'unbounded work')
    print('CT240_COMPLETE 5 fixed arms; negative arms preserved; no native timing/PCC',
          flush=True)

if __name__ == '__main__':
    try:
        main()
    except Exception as exc:
        print('CT240_FAIL', type(exc).__name__, str(exc), file=sys.stderr, flush=True)
        raise
