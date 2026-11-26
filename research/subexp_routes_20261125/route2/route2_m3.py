#!/usr/bin/env python3
"""
Route 2, m = 3 addendum -- the 3-summand relation join at p ~ 2^13.

Reuses route2_join.py as the instrument (curve search, Kummer/isogeny/interval
bases, lift, point arithmetic, sumset/energy measurements).

A relation is Q = P1 + P2 + P3 with Pi in F, F = {P : x(P) in V}.  The columns of
the relation matrix are the points of F, so the base size that enters the random
model |F|^m / p is |F|, not |V|.  Bases are chosen to give |F| ~ p^(1/3) = 23.2
at p = 12289; the |F| actually reached is reported as |F|/p^(1/3).

Relation counting: a triple {a,b,c} of distinct points has 6 ordered triples but
only 1 relation (the multiset), and 3 (unordered-pair, third) table hits.  The
"relations per target" used below is exact, from `threesum_census`, which
enumerates all |F|^3 ordered triples and all |F|(|F|+1)(|F|+2)/6 multisets.

Three joins, same base and target:
  j1 enumerative   : every unordered pair (P1,P2), test x(T-P1-P2) in V
                     cost/pass |F|(|F|+1)/2, relations/pass = census rate
  j2 pair table    : build all P1+P2, then scan the third summand
                     cost = |F|(|F|+1)/2 + (#targets)*|F|
  j3 random triple : uniform (P1,P2,P3) tested against K targets
                     cost = #tests + K, with #tests = |F|^3 (see stage C)

Outputs
  route2_m3_results.json     every measured table
  route2_m3_relations.jsonl  rank-growth series and sample supports (m = 2, 3)

Usage: python3 route2_m3.py [--json OUT] [--jsonl OUTL]
"""
import json
import math
import random
import sys
import time

import route2_join as R

BIGQ = 2 ** 61 - 1
EULER_GAMMA = 0.5772156649


# --------------------------------------------------------------- rank / matrix


def rank_mod(rows, n, Q=BIGQ):
    M = [r[:] for r in rows]
    rank = 0
    for c in range(n):
        piv = None
        for r in range(rank, len(M)):
            if M[r][c] % Q:
                piv = r
                break
        if piv is None:
            continue
        M[rank], M[piv] = M[piv], M[rank]
        invp = pow(M[rank][c], Q - 2, Q)
        M[rank] = [(x * invp) % Q for x in M[rank]]
        for r in range(len(M)):
            if r != rank and M[r][c] % Q:
                f = M[r][c]
                M[r] = [(x - f * y) % Q for x, y in zip(M[r], M[rank])]
        rank += 1
        if rank == n:
            break
    return rank


def row_of(support, n):
    v = [0] * n
    for i in support:
        v[i] += 1
    return v


def graph_components(supports, n):
    """components, bipartite components, colouring, adjacency."""
    adj = [[] for _ in range(n)]
    for (i, j) in supports:
        adj[i].append(j)
        adj[j].append(i)
    color = [-1] * n
    comps = bip = 0
    bip_comp = []
    for s0 in range(n):
        if color[s0] != -1 or not adj[s0]:
            continue
        comps += 1
        color[s0] = 0
        stack, ok, members = [s0], True, [s0]
        while stack:
            u = stack.pop()
            for w in adj[u]:
                if color[w] == -1:
                    color[w] = 1 - color[u]
                    stack.append(w)
                    members.append(w)
                elif color[w] == color[u]:
                    ok = False
        if ok:
            bip += 1
            bip_comp.append(set(members))
    return comps, bip, color, adj, bip_comp


# ------------------------------------------------------------------- bases


def make_interval_base(p, a, b, target_F, qr):
    """V = [1..X] with X chosen so |F| is as close to target_F as possible."""
    best = None
    for X in range(max(4, target_F // 2), min(p - 1, 4 * target_F + 8)):
        V = list(range(1, X + 1))
        F = R.lift_abscissae(p, a, b, V, qr)
        d = abs(len(F) - target_F)
        if best is None or d < best[0]:
            best = (d, V, F)
        if len(F) > target_F + 4 and X > target_F:
            break
    return best[1], best[2]


# --------------------------------------------------------------- m=3 census


def threesum_census(F, a, p, inv):
    """exact triple-sum census: distinct sums, ordered triples, distinct
    relations (column multisets), and the per-target relation rate."""
    nF = len(F)
    ordered = {}
    for i in range(nF):
        for j in range(nF):
            S = R.padd(F[i], F[j], a, p, inv)
            for k in range(nF):
                U = S if F[k] is None else R.padd(S, F[k], a, p, inv)
                ordered[U] = ordered.get(U, 0) + 1
    multis = {}
    for i in range(nF):
        for j in range(i, nF):
            S = R.padd(F[i], F[j], a, p, inv)
            for k in range(j, nF):
                U = S if F[k] is None else R.padd(S, F[k], a, p, inv)
                multis.setdefault(U, set()).add((i, j, k))
    tot_rel = sum(len(v) for v in multis.values())
    nT = len(ordered)
    return dict(sums=nT, ordered=sum(ordered.values()), relations=tot_rel,
                rel_per_target=tot_rel / nT if nT else 0.0, coverage=nT)


def pair_table(p, a, F, inv):
    tab, ops = {}, 0
    nF = len(F)
    for i in range(nF):
        for j in range(i, nF):
            S = R.padd(F[i], F[j], a, p, inv)
            ops += 1
            tab.setdefault(S, []).append((i, j))
    return tab, ops


def table_pass(T, F, tab, a, p, inv):
    hits = 0
    for P3 in F:
        S = R.padd(T, (P3[0], (-P3[1]) % p), a, p, inv)
        e = tab.get(S)
        if e:
            hits += len(e)
    return hits


def three_fold(F, a, p, inv):
    """|F+F+F| (returns count) and 3-sum energy sum_c r3(c)^2."""
    cnt = {}
    for P in F:
        for Q in F:
            S = R.padd(P, Q, a, p, inv)
            for Rt in F:
                U = S if Rt is None else R.padd(S, Rt, a, p, inv)
                cnt[U] = cnt.get(U, 0) + 1
    return len(cnt), sum(c * c for c in cnt.values())


# ------------------------------------------------------------- rank structure


def rank_need(n, m, trials, rng, cap_factor=6):
    """add random m-column relations until the matrix has full rank n."""
    if m == 2:
        pool = [(i, j) for i in range(n) for j in range(i, n)]
    else:
        pool = [(i, j, k) for i in range(n) for j in range(i, n)
                for k in range(j, n)]
    cap = int(cap_factor * n)
    out = []
    for _ in range(trials):
        order = pool[:]
        rng.shuffle(order)
        rows, done = [], None
        for t in range(1, min(len(order), cap) + 1):
            rows.append(row_of(order[t - 1], n))
            if t % 2 == 0 or t == len(order):     # rank every 2 rows
                if rank_mod(rows, n) == n:
                    done = t
                    break
        out.append(done if done else cap)
    return sum(out) / len(out), out


# -------------------------------------------------------------------- stages


def stage_join(p_list, out, jsonl):
    rows_out = []
    for p in p_list:
        qr, inv = R.make_qr(p), R.make_inv(p)
        a, b, N = R.find_curve(p, qr)
        B_ideal = N ** (1.0 / 3.0)
        rng = random.Random(424242 ^ p)
        print("\n== A. m=3 join at |F| ~ p^(1/3): p = %d, #E = %d, p^(1/3)=%.2f =="
              % (p, N, B_ideal))
        print("%-9s %3s %4s %4s %5s %6s %9s %8s %8s %9s %9s %6s %6s"
              % ("tower", "t", "|V|", "|F|", "B/Bid", "|F+F|", "|F+F+F|",
                 "rel/tgt", "ops/rel", "tot(j2)", "tot(j1)", "E3", "meas×"))
        jobs = [("interval", 0)] + [(tw, t) for t in (4, 5)
                                    for tw in ("kummer", "isogeny")]
        for (tower, t) in jobs:
            if tower == "interval":
                V, F = make_interval_base(p, a, b, int(round(B_ideal)), qr)
            else:
                V, F = R.build_base(p, tower, t, a, b, qr, inv)
            if V is None or not (10 <= len(F) <= 32):
                continue
            nF = len(F)
            tab, tab_ops = pair_table(p, a, F, inv)
            cen = threesum_census(F, a, p, inv)
            rel_pt = cen["relations"] / N            # relations per uniform target
            tot2 = tab_ops + (nF / rel_pt) * nF
            tot1 = (nF / rel_pt) * (nF * (nF + 1) / 2.0)
            # measured check of the hit count on the same base
            hits_exact = (nF * (nF + 1) / 2.0) * nF / N
            nt = max(300, min(4000, int(1500.0 / max(hits_exact, 1e-9))))
            h = 0
            for _ in range(nt):
                T = R.random_point(p, a, b, qr, rng)
                h += table_pass(T, F, tab, a, p, inv)
            meas_ratio = (h / nt) / hits_exact
            # structure: 3-sum energy vs matched control
            _, e3 = three_fold(F, a, p, inv)
            ctl = []
            for _ in range(2):
                xs = []
                while len(xs) < nF // 2:
                    x = rng.randrange(1, p)
                    if (x * x * x + a * x + b) % p in qr:
                        xs.append(x)
                G = R.lift_abscissae(p, a, b, xs, qr)[:nF]
                ctl.append(three_fold(G, a, p, inv)[1])
            e3r = e3 / (sum(ctl) / len(ctl))
            print("%-9s %3d %4d %4d %5.2f %6d %9d %8.3f %8.1f %9.0f %9.0f "
                  "%6.2f %5.2f"
                  % (tower, t, len(V), nF, nF / B_ideal, len(tab), cen["sums"],
                     rel_pt, nF / rel_pt, tot2, tot1, e3r, meas_ratio))
            if tower == "interval":
                seen = set()
                for _ in range(60):
                    T = R.random_point(p, a, b, qr, rng)
                    for k, P3 in enumerate(F):
                        S = R.padd(T, (P3[0], (-P3[1]) % p), a, p, inv)
                        for (i, j) in tab.get(S, ()):
                            key = tuple(sorted((i, j, k)))
                            if key in seen or len(seen) >= 10:
                                continue
                            seen.add(key)
                            ok = R.padd(R.padd(F[i], F[j], a, p, inv), F[k],
                                        a, p, inv) == T
                            jsonl.append(dict(kind="relation", p=p, tower=tower,
                                              a=a, b=b, F=nF, V=len(V),
                                              support=list(key),
                                              pts=[list(F[i]), list(F[j]),
                                                   list(F[k])],
                                              target=list(T), verified=ok))
            rows_out.append(dict(p=p, tower=tower, t=t, V=len(V), F=nF, N=N,
                                 B_ideal=B_ideal, table_entries=len(tab),
                                 table_ops=tab_ops, F3=cen["sums"],
                                 coverage3=cen["sums"] / N, relations=cen["relations"],
                                 rel_per_target=rel_pt, ops_per_relation=nF / rel_pt,
                                 total_j2=tot2, total_j1=tot1,
                                 energy3_ratio=e3r, measured_ratio=meas_ratio))
        out["m3_join"] = rows_out
    return rows_out


def stage_rank(p, out, jsonl):
    print("\n== B. relations needed for a full-rank relation matrix ==")
    print("   rows: one random relation.  columns: the |F| base points.")
    print("   %-4s %5s %11s %9s %18s %s"
          % ("m", "|F|", "mean R_full", "R_full/n", "coupon (n/m)(ln n+g)",
             "P(FULL RANK)"))
    rows = []
    rng = random.Random(20261125)
    for n in (16, 24, 32, 48):
        for m in (2, 3):
            mean, allr = rank_need(n, m, 40, rng)
            coupon = (n / m) * (math.log(n) + EULER_GAMMA)
            nfail = sum(1 for r in allr if r >= 6 * n)
            print("   %-4d %5d %11.1f %9.2f %18.1f %d/%d"
                  % (m, n, mean, mean / n, coupon, len(allr) - nfail, len(allr)))
            rows.append(dict(m=m, F=n, mean_R_full=mean, R_over_n=mean / n,
                             coupon=coupon, trials=len(allr), fails=nfail))
            for r in allr:
                jsonl.append(dict(kind="rank_need", m=m, F=n, R_full=r))
    # m=2 defect made explicit: rows e_i+e_j form a graph incidence matrix, so
    # rank = n - (#bipartite components), isolated vertices counting as trivial
    # (bipartite) components.  The kernel vector is the +-1 sign vector.
    print("   m=2 defect, 200 trials at n=24 with 24 relations:")
    n = 24
    pool = [(i, j) for i in range(n) for j in range(i, n) if i != j]
    ok_rank = ok_kern = with_bip = one_t = 0

    def comps_all(supp, n):
        adj = [[] for _ in range(n)]
        for (i, j) in supp:
            adj[i].append(j)
            adj[j].append(i)
        col = [-1] * n
        bip, groups = 0, []
        for s0 in range(n):
            if col[s0] != -1:
                continue
            col[s0] = 0
            stack, ok, mem = [s0], True, [s0]
            while stack:
                u = stack.pop()
                for w in adj[u]:
                    if col[w] == -1:
                        col[w] = 1 - col[u]
                        stack.append(w)
                        mem.append(w)
                    elif col[w] == col[u]:
                        ok = False
            if ok:
                bip += 1
                groups.append(mem)
        return bip, col, groups

    for _ in range(200):
        supp = rng.sample(pool, 24)
        bip, col, groups = comps_all(supp, n)
        r = rank_mod([row_of(s, n) for s in supp], n)
        if r == n - bip:
            ok_rank += 1
        nontriv = [m for m in groups if len(m) > 1]
        if nontriv:
            with_bip += 1
            w = [0] * n
            for mem in nontriv:
                for i in mem:
                    w[i] = 1 if col[i] == 0 else -1
            good = all(w[i] + w[j] == 0 for (i, j) in supp if w[i] and w[j])
            # 1^T is never in the left kernel: every row is e_i + e_j, sum 2
            good = good and all(sum(row_of(s, n)) == 2 for s in supp)
            if good:
                ok_kern += 1
        if all(sum(row_of(s, n)) == 2 for s in supp):
            one_t += 1
    print("      rank = n - #bip comps (isolated vertices count as trivial "
          "components): %d/200" % ok_rank)
    print("      %d/200 graphs have a non-trivial bipartite component (>=2 "
          "columns); on it the +-1 sign vector is in the left kernel in %d of "
          "them; 1^T is in none (%d/200 rows sum to 2)" %
          (with_bip, ok_kern, one_t))
    print("      -> m=2 kernel = a bipartition sign vector on a bipartite "
          "component, never 1^T")
    for m in (2, 3):
        nn = 24
        pool = ([(i, j) for i in range(nn) for j in range(i, nn)] if m == 2 else
                [(i, j, k) for i in range(nn) for j in range(i, nn)
                 for k in range(j, nn)])
        order = pool[:]
        rng.shuffle(order)
        rows_ = []
        for t in range(1, 61):
            rows_.append(row_of(order[t - 1], nn))
            jsonl.append(dict(kind="rank_growth", m=m, F=nn, n_relations=t,
                              rank=rank_mod(rows_, nn)))
    out["rank"] = dict(rows=rows, m2_rank_formula_ok=ok_rank,
                       m2_bipartite=with_bip, m2_kernel_ok=ok_kern)
    return rows


def stage_terms(p, out):
    qr, inv = R.make_qr(p), R.make_inv(p)
    a, b, N = R.find_curve(p, qr)
    B = N ** (1.0 / 3.0)
    V, F = make_interval_base(p, a, b, int(round(B)), qr)
    nF = len(F)
    rng = random.Random(5 ^ p)
    tab, tab_ops = pair_table(p, a, F, inv)
    cen = threesum_census(F, a, p, inv)
    rel_pt = cen["relations"] / N
    systems = nF / rel_pt                 # targets = algebraic systems
    scan = systems * nF
    join_total = tab_ops + scan
    T_log = math.log2(nF)
    C_route1 = 2.0 ** T_log
    N_var = 3.0 * T_log
    C_f4 = 127.4e6 * (15.72 ** ((N_var - 10.0) / 2.0))
    term_r1 = systems * C_route1
    term_f4 = systems * C_f4
    rho = 1.25 * math.sqrt(N)
    print("\n== C. the two m=3 cost terms at B = p^(1/3) ==")
    print("   p=%d #E=%d |V|=%d |F|=%d (B/p^(1/3)=%.2f)  rel/target=%.3f  "
          "targets=%.0f" % (p, N, len(V), nF, nF / B, rel_pt, systems))
    print("   3-sum coverage |F+F+F|/N = %d/%d = %.3f" % (cen["sums"], N,
                                                          cen["sums"] / N))
    print("   join A (pair table %d + scan %0.f)          = %7.0f ops = %5.2f "
          "p^(2/3) = %5.1f x rho" % (tab_ops, scan, join_total,
                                     join_total / N ** (2.0 / 3.0),
                                     join_total / rho))
    print("   join B (algebraic, Route-1 target 2^T=%.1f)  = %7.0f ops = %5.2f "
          "p^(2/3) = %5.1f x rho" % (C_route1, term_r1,
                                     term_r1 / N ** (2.0 / 3.0), term_r1 / rho))
    print("   join B (algebraic, measured F4 %.3g MA/call)  = %.3g ops = %.1e x rho"
          % (C_f4, term_f4, term_f4 / rho))
    C3 = nF * (nF + 1) * (nF + 2) / 6.0
    tot_j3 = C3 + nF * N / C3
    best3 = (None, None)
    for k in range(2, 2000):
        c3 = k * (k + 1) * (k + 2) / 6.0
        t3 = c3 + k * N / c3
        if best3[0] is None or t3 < best3[0]:
            best3 = (t3, k)
    print("   join C (random triple + target list, |F|=%d)     = %7.0f ops = %5.2f "
          "p^(2/3) = %5.1f x rho" % (nF, tot_j3, tot_j3 / N ** (2.0 / 3.0),
                                     tot_j3 / rho))
    print("   best m=3 join at this p (join C at |F|=%d)     = %7.0f ops = %5.2f "
          "p^(2/3) = %5.1f x rho" % (best3[1], best3[0],
                                     best3[0] / N ** (2.0 / 3.0), best3[0] / rho))
    ex = {}
    for name, tot in (("table", join_total), ("route1", term_r1), ("f4", term_f4)):
        ex[name] = math.log(tot) / math.log(N)
    print("   single-point exponent ln(ops)/ln(p): table %.3f, Route1 %.3f, F4 %.3f"
          % (ex["table"], ex["route1"], ex["f4"]))
    print("   (asymptotic laws: table c*p^(2/3) with c=%.2f; Route1 c*p^(2/3) "
          "with c=%.2f; F4 p^{2.6})"
          % (join_total / N ** (2.0 / 3.0), term_r1 / N ** (2.0 / 3.0)))
    out["m3_terms"] = dict(p=p, N=N, B_ideal=B, V=len(V), F=nF,
                           rel_per_target=rel_pt, coverage3=cen["sums"] / N,
                           systems=systems, table_ops=tab_ops, scan=scan,
                           total_table=join_total, T_log=T_log,
                           C_route1=C_route1, term_route1=term_r1, N_var=N_var,
                           C_f4=C_f4, term_f4=term_f4, rho_ops=rho,
                           j3_total=tot_j3, best_m3_join=best3[0],
                           best_m3_F=best3[1], exponents=ex)
    return out["m3_terms"]


def stage_strategies(p_list, out):
    """exact closed forms (validated against the census in stage A):
       rel/target = C(|F|+2,3)/N   (m=3)
       j1 = (|F|/rel) * |F|(|F|+1)/2        enumerative pairs
       j2 = |F|(|F|+1)/2 + (|F|/rel)*|F|    pair table + scan
       j3 = C(|F|+2,3) + |F|*N/C(|F|+2,3)   random triple + target list
    """
    print("\n== D. m=3: three joins vs base size, and the best exponent ==")
    print("   %-6s %4s %6s %9s %9s %9s"
          % ("p", "|F|", "|F|/Bid", "j1 enum", "j2 table", "j3 rndtriple"))
    res = {}
    for p in p_list:
        qr, inv = R.make_qr(p), R.make_inv(p)
        a, b, N = R.find_curve(p, qr)
        B_id = N ** (1.0 / 3.0)
        rows = []
        for nF in range(4, 65, 2):
            C3 = nF * (nF + 1) * (nF + 2) / 6.0
            pairs = nF * (nF + 1) / 2.0
            rel = C3 / N
            tot1 = (nF / rel) * pairs
            tot2 = pairs + (nF / rel) * nF
            tot3 = C3 + nF * N / C3
            rows.append(dict(F=nF, j1=tot1, j2=tot2, j3=tot3,
                             best=min(tot1, tot2, tot3)))
            if nF in (8, 16, 24, 32, 48, 64):
                print("   %-6d %4d %6.2f %9.0f %9.0f %9.0f"
                      % (p, nF, nF / B_id, tot1, tot2, tot3))
        mins = {}
        for k in ("j1", "j2", "j3"):
            mm = min(rows, key=lambda r: r[k])
            mins[k] = dict(total=mm[k], F=mm["F"])
        print("   minima over |F|=4..64: j1 %.0f@%d  j2 %.0f@%d  j3 %.0f@%d"
              % (mins["j1"]["total"], mins["j1"]["F"], mins["j2"]["total"],
                 mins["j2"]["F"], mins["j3"]["total"], mins["j3"]["F"]))
        res[str(p)] = dict(p=p, N=N, B_ideal=B_id, rows=rows, minima=mins)
    ps = sorted(res, key=lambda k: res[k]["p"])
    if len(ps) >= 2:
        fits = {}
        for k in ("j1", "j2", "j3"):
            y0 = res[ps[0]]["minima"][k]["total"]
            y1 = res[ps[-1]]["minima"][k]["total"]
            fits[k] = math.log(y1 / y0) / math.log(res[ps[-1]]["p"] / res[ps[0]]["p"])
        print("   fitted exponent of each minimum, p=%d..%d: %s"
              % (res[ps[0]]["p"], res[ps[-1]]["p"],
                 ", ".join("%s %.3f" % (k, v) for k, v in fits.items())))
        res["fitted_exponents"] = fits
    # where the asymptotes live (same closed forms, larger N; model, not measured)
    print("   the same closed forms at larger N (model projection):")
    print("   %-9s %6s %10s %10s %10s" %
          ("N", "|F|*", "j2/p", "j3/p", "j3 N-exp"))
    proj = []
    for N in (10 ** 3, 10 ** 4, 10 ** 5, 10 ** 6, 10 ** 8, 10 ** 10, 10 ** 12):
        best2 = best3 = None
        for nF in range(2, 4000):
            C3 = nF * (nF + 1) * (nF + 2) / 6.0
            pairs = nF * (nF + 1) / 2.0
            rel = C3 / N
            t2 = pairs + (nF / rel) * nF
            t3 = C3 + nF * N / C3
            if best2 is None or t2 < best2[0]:
                best2 = (t2, nF)
            if best3 is None or t3 < best3[0]:
                best3 = (t3, nF)
        proj.append(dict(N=N, F2=best2[1], j2=best2[0], j3=best3[0],
                         F3=best3[1]))
        print("   %-9.0e %6d %10.3g %10.3g" % (N, best3[1], best2[0] / N,
                                               best3[0] / N))
    xs = [math.log(r["N"]) for r in proj]
    ys = [math.log(r["j3"]) for r in proj]
    n = len(xs)
    mx = sum(xs) / n
    my = sum(ys) / n
    slope3 = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) /         sum((x - mx) ** 2 for x in xs)
    print("   projected exponent of the m=3 best (j3) join over N=1e3..1e12: "
          "%.3f (law 3/5 = 0.600)" % slope3)
    res["projection"] = proj
    out["m3_strategies"] = res
    return res


def stage_compare(p, out):
    qr, inv = R.make_qr(p), R.make_inv(p)
    a, b, N = R.find_curve(p, qr)
    print("\n== E. m=2 vs m=3 at the same base (p=%d) ==" % p)
    print("   %4s %4s %11s %11s %8s" % ("|V|", "|F|", "m=2 table", "m=3 table",
                                        "m3/m2"))
    rows = []
    for target in (12, 16, 20, 24, 32):
        V, F = make_interval_base(p, a, b, target, qr)
        nF = len(F)
        if nF < 6:
            continue
        tab_ops = nF * (nF + 1) / 2.0
        cen = threesum_census(F, a, p, inv)
        rel2 = nF * nF / (2.0 * N)
        rel3 = cen["relations"] / N
        tot2 = tab_ops + nF / rel2
        tot3 = tab_ops + (nF / rel3) * nF
        print("   %4d %4d %11.0f %11.0f %8.2f" % (len(V), nF, tot2, tot3,
                                                  tot3 / tot2))
        rows.append(dict(V=len(V), F=nF, m2_total=tot2, m3_total=tot3,
                         ratio=tot3 / tot2, rel2_per_target=rel2,
                         rel3_per_target=rel3))
    out["m2_m3"] = rows
    return rows


def main():
    outp, outl = "route2_m3_results.json", "route2_m3_relations.jsonl"
    if "--json" in sys.argv:
        outp = sys.argv[sys.argv.index("--json") + 1]
    if "--jsonl" in sys.argv:
        outl = sys.argv[sys.argv.index("--jsonl") + 1]
    out, jsonl = {}, []
    t0 = time.time()
    plist = [12289] + ([16381] if R.is_prime(16381) else [])
    stage_join(plist, out, jsonl)
    stage_rank(12289, out, jsonl)
    stage_terms(12289, out)
    stage_strategies([1009, 4099, 12289], out)
    stage_compare(12289, out)
    print("\ntotal wall %.1f s" % (time.time() - t0))
    with open(outp, "w") as f:
        json.dump(out, f, indent=1, sort_keys=True)
    with open(outl, "w") as f:
        for rec in jsonl:
            f.write(json.dumps(rec, sort_keys=True) + "\n")
    print("wrote %s and %s (%d records)" % (outp, outl, len(jsonl)))


if __name__ == "__main__":
    main()
