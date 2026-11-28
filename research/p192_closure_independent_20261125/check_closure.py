#!/usr/bin/env python3
"""Independent closure check of the P-192 split-tangent local run (PR #5123).

Written from the published result.json schema only; it does not import or call
the producer's C++ code. Checks, all exact:

  A. every projected-base point lies on NIST P-192 and has order n (n*P = O);
  B. every matrix row is an elliptic relation: sum_j c_j * B_j = O;
  C. column incidence: for each row with support on new columns (index >= 225),
     count columns used by no other row ("private" columns);
  D. ranks modulo n: full, rank of the new-column projection, closed rank =
     dim(rowspace intersect old coordinates) = full - new, baseline rank of
     rows with zero new support, and gain = closed - baseline;
  E. the closed space is spanned by the baseline rows (explicit check);
  F. planted controls: (i) a duplicate-sharing synthetic row that must produce
     gain 1 and break privacy; (ii) one corrupted coefficient that must fail B.

Lemma used for the support-size statement: if a row r has a column c that is
nonzero in r and zero in every other row, then any left-kernel vector of the
new-column projection has coefficient 0 on r. Hence if every added row has a
private new column, the closed space equals the span of the baseline rows,
whatever the support size of a candidate combination.
"""
import hashlib, json, sys, time

P = 2**192 - 2**64 - 1
A = -3 % P
B = 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1
N = 6277101735386680763835789423176059013767194773182842284081
OLD = 225

def inv(a, m): return pow(a, m - 2, m)

def add(p1, p2):
    if p1 is None: return p2
    if p2 is None: return p1
    x1, y1 = p1; x2, y2 = p2
    if x1 == x2:
        if (y1 + y2) % P == 0: return None
        l = (3 * x1 * x1 + A) * inv(2 * y1, P) % P
    else:
        l = (y2 - y1) * inv((x2 - x1) % P, P) % P
    x3 = (l * l - x1 - x2) % P
    return (x3, (l * (x1 - x3) - y1) % P)

def mul(k, pt, reduce=True):
    if reduce: k %= N
    r = None
    while k:
        if k & 1: r = add(r, pt)
        pt = add(pt, pt); k >>= 1
    return r

def on_curve(pt):
    x, y = pt
    return (y * y - (x * x * x + A * x + B)) % P == 0

def rank_mod(rows, m):
    rows = [r[:] for r in rows if any(r)]
    rk = 0; ncol = len(rows[0]) if rows else 0
    for c in range(ncol):
        piv = next((i for i in range(rk, len(rows)) if rows[i][c] % m), None)
        if piv is None: continue
        rows[rk], rows[piv] = rows[piv], rows[rk]
        iv = inv(rows[rk][c], m)
        rows[rk] = [v * iv % m for v in rows[rk]]
        for i in range(len(rows)):
            if i != rk and rows[i][c]:
                f = rows[i][c]
                rows[i] = [(a - f * b) % m for a, b in zip(rows[i], rows[rk])]
        rk += 1
    return rk

def analyse(M):
    R = len(M); C = len(M[0])
    newcols = range(OLD, C)
    added = [i for i in range(R) if any(M[i][c] for c in newcols)]
    base = [i for i in range(R) if i not in added]
    users = {c: [i for i in range(R) if M[i][c]] for c in newcols}
    private = {i: [c for c in newcols if users[c] == [i]] for i in added}
    full = rank_mod(M, N)
    new = rank_mod([[r[c] for c in newcols] for r in M], N)
    closed = full - new
    baseline = rank_mod([M[i] for i in base], N) if base else 0
    return dict(rows=R, cols=C, added_rows=len(added), baseline_rows=len(base),
                full_rank=full, new_rank=new, closed_rank=closed,
                baseline_rank=baseline, gain=closed - baseline,
                added_rows_with_private_col=sum(1 for i in added if private[i]),
                min_private_cols=min((len(v) for v in private.values()), default=0),
                max_private_cols=max((len(v) for v in private.values()), default=0),
                new_cols_unused=sum(1 for c in newcols if not users[c]),
                new_cols_shared=sum(1 for c in newcols if len(users[c]) > 1)), base, added

def main(path, out):
    t0 = time.perf_counter()
    raw = open(path, 'rb').read()
    d = json.loads(raw)
    M = [[int(v) % N for v in row] for row in d['matrix']]
    basepts = [None if q.get('infinity') else (int(q['x']), int(q['y'])) for q in d['projected_base']]
    res = {'schema': 'p192-closure-independent/v1', 'input': path,
           'input_sha256': hashlib.sha256(raw).hexdigest(), 'standing': 'unverified (independent producer)'}
    # A
    finite = [q for q in basepts if q is not None]
    res['A_points_on_curve'] = sum(on_curve(q) for q in finite)
    res['A_points_order_n'] = sum(mul(N, q, reduce=False) is None for q in finite)
    res['A_finite_points'] = len(finite); res['A_infinity_entries'] = len(basepts) - len(finite)
    t1 = time.perf_counter()
    # B
    def row_sum(r):
        acc = None
        for c, v in enumerate(r):
            if v and basepts[c] is not None: acc = add(acc, mul(v, basepts[c]))
        return acc
    res['B_rows_summing_to_O'] = sum(row_sum(r) is None for r in M)
    t2 = time.perf_counter()
    # C,D
    stats, base, added = analyse(M)
    res.update({'CD_' + k: v for k, v in stats.items()})
    # E: closed space == span(baseline rows): rank(baseline + closed combos) == baseline rank
    combos = [[int(v) % N for v in r] for r in d['closure']['combinations']]
    oldrows = [[int(v) % N for v in r] + [0] * (len(M[0]) - OLD) for r in d['closure']['old_rows']]
    res['E_recorded_old_rows'] = len(oldrows)
    res['E_old_rows_in_baseline_span'] = rank_mod([M[i] for i in base] + oldrows, N) == stats['baseline_rank']
    # combinations: vectors over rows; check each gives an old-only row
    ok = 0
    for cv in combos:
        if len(cv) != len(M): break
        v = [sum(cv[i] * M[i][c] for i in range(len(M))) % N for c in range(len(M[0]))]
        ok += all(x == 0 for x in v[OLD:]) and v in oldrows
    res['E_recorded_combinations_old_only'] = f"{ok}/{len(combos)}"
    t3 = time.perf_counter()
    # F(i): planted sharing row = added row + a fresh old-column unit vector
    # outside the baseline span; must give gain 1 and break privacy of that row.
    r0 = added[0]
    e = None
    for c in range(OLD):
        cand = [0] * len(M[0]); cand[c] = 1
        if rank_mod([M[i] for i in base] + [cand], N) > stats['baseline_rank']: e = cand; break
    planted = M + [[(a + b) % N for a, b in zip(M[r0], e)]]
    ps, _, _ = analyse(planted)
    res['F1_planted_gain'] = ps['gain']
    res['F1_planted_rows_with_private_col'] = f"{ps['added_rows_with_private_col']}/{ps['added_rows']}"
    # F(iii): order-check control: (n-1)*P must not be O
    res['F3_order_control_nonzero'] = mul(N - 1, finite[0], reduce=False) is not None
    # F(ii): corrupt one nonzero coefficient of an added row
    bad = [r[:] for r in M]; c = next(c for c, v in enumerate(bad[r0]) if v)
    bad[r0][c] = (bad[r0][c] + 1) % N
    res['F2_corrupted_row_sums_to_O'] = row_sum(bad[r0]) is None
    t4 = time.perf_counter()
    res['cost_seconds'] = {'A_points': round(t1 - t0, 3), 'B_row_sums': round(t2 - t1, 3),
                           'CDE_linear_algebra': round(t3 - t2, 3), 'F_controls': round(t4 - t3, 3)}
    passed = (res['A_points_on_curve'] == res['A_finite_points'] == res['A_points_order_n']
              and res['B_rows_summing_to_O'] == len(M)
              and stats['added_rows_with_private_col'] == stats['added_rows']
              and stats['gain'] == 0 and res['E_old_rows_in_baseline_span']
              and ps['gain'] == 1 and res['F3_order_control_nonzero'] and not res['F2_corrupted_row_sums_to_O'])
    res['verdict'] = 'PASS' if passed else 'FAIL'
    json.dump(res, open(out, 'w'), indent=2); print(json.dumps(res, indent=2))
    return 0 if passed else 1

if __name__ == '__main__':
    sys.exit(main(sys.argv[1], sys.argv[2]))
