#!/usr/bin/env python3
"""Fully counted Gaudry index calculus on E/F_{p^3} versus matched store-all +- Pollard rho.

Built on ../ext_field/ic_fp3.py (copied here as ic_fp3_orig.py, unchanged). Changes:
  * msolve is replaced by counted_gb.solve (F4 + sparse FGLM/Wiedemann + counted root finding +
    Horner eigenvector), so the whole decomposition is charged in F_p multiplications and the run
    is replayable from the seed (no external process).
  * The mod-r linear algebra (incremental sparse elimination, ic_fp3_orig.Elim) is counted in Z_r
    multiplications; a block-Wiedemann estimate 9 N K (3N matvecs of 3K nonzeros) is also recorded.
  * Cost is split by phase: walk, build (Weil descent of S4), f4, multmatrix, wiedemann (sequence +
    Berlekamp-Massey), rootfind (F_p roots of the degree-D eliminating polynomial), eigen (e1, e2 per
    root), post (cubic roots, factor-base tests, y lifts, sign additions), la (Z_r).
  * Modes on one relation stream: all (factor base = every x in F_p, up to sign), 2lp:B, 1lp:B with
    B = p/2. The run stops when all modes are done or at cap x the trials of the first solved mode.

usage: ic_fp3_full.py p seed ntargets nrho out.json [cap]
"""
import json, math, os, random, sys, time
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import ic_fp2 as F
import ic_fp3_orig as T
import counted_gb as G

C = F.C


def make_solver(rng):
    def proots_counted(f):
        m0, i0 = C[0], C[1]
        r = F.proots(F.ptrim(f), rng)
        G.charge('rootfind', C[0] - m0, C[1] - i0)
        C[0], C[1] = m0, i0          # moved from the generic counter to the phase table
        return r

    def solve_sys(polys, rng_, tag):
        t0 = time.time()
        pd = [{(i, j, k): c for i, j, k, c in P} for P in polys]
        sols, D, status, st = G.solve(pd, T.p, rng, proots_counted)
        dt = time.time() - t0
        p = T.p
        good = [e for e in sols if all(sum(c * pow(e[0], i, p) * pow(e[1], j, p) * pow(e[2], k, p)
                                           for i, j, k, c in P) % p == 0 for P in polys)]  # uncounted check
        if len(good) != len(sols):
            status = 'badsol'
        return good, D, dt, status
    return solve_sys


def snapshot(walkM, walkI, buildM, buildI):
    ph = {k: list(v) for k, v in G.PH.items()}
    ph['walk'] = [walkM, walkI]
    ph['build'] = [buildM, buildI]
    # everything else charged to the generic counter is post-processing of solutions
    ph['post'] = [C[0] - walkM - buildM, C[1] - walkI - buildI]
    return ph


def ic_run(P, Q, a, b, r, w, rng, cap=2.0):
    C[0] = C[1] = C[2] = 0
    G.PH.clear()
    mult = []
    for _ in range(20):
        c = rng.randrange(r); d = rng.randrange(r)
        mult.append((T.padd(T.smul(c, P, a), T.smul(d, Q, a), a), c, d))
    c = rng.randrange(r); d = rng.randrange(r)
    R = T.padd(T.smul(c, P, a), T.smul(d, Q, a), a)
    S = T.Sys(a, b)
    T.solve_sys = make_solver(rng)
    C[0] = C[1] = C[2] = 0
    G.PH.clear()
    p = T.p
    B = p // 2
    modes = {'all': 0, '2lp:B': 1, '1lp:B': 2}
    el = {m: T.Elim(r) for m in modes}
    zr = {m: 0 for m in modes}
    kept = {m: 0 for m in modes}
    cols = {m: set() for m in modes}
    done = {}
    stats = dict(msolve_sec=0.0, calls=0, deg=[], status={})
    seen = set(); trials = 0; rels = 0; t0 = time.time(); first = None
    walkM = walkI = buildM = buildI = 0
    orig_build = S.build

    def build(x4):
        nonlocal buildM, buildI
        m0, i0 = C[0], C[1]
        out = orig_build(x4)
        buildM += C[0] - m0; buildI += C[1] - i0
        return out
    S.build = build
    while len(done) < len(modes):
        M_, cc, dd = mult[rng.randrange(20)]
        m0, i0 = C[0], C[1]
        R = T.padd(R, M_, a); c = (c + cc) % r; d = (d + dd) % r; trials += 1
        walkM += C[0] - m0; walkI += C[1] - i0
        if R is None or R[0] in seen:
            continue
        seen.add(R[0])
        if R[0][1] == 0 and R[0][2] == 0:
            continue
        for rel in T.decompose(R, a, b, S, rng, '', stats):
            rels += 1
            nsmall = sum(1 for (x, s) in rel if x < B)
            for m, need in modes.items():
                if m in done or nsmall < need:
                    continue
                kept[m] += 1
                row = {}
                for x, s in rel:
                    row[x] = row.get(x, 0) + s
                cols[m].update(row)
                z0 = C[2]
                eq = el[m].add(row, c, d)
                zr[m] += C[2] - z0
                if eq is not None:
                    ww = F.solve_eq(eq[0], eq[1], r)
                    if ww is not None:
                        assert ww == w, 'IC wrong log'
                        N = len(cols[m]); K = kept[m]
                        done[m] = dict(trials=trials, rels_seen=rels, rels_kept=K, ncols=N,
                                       phases=snapshot(walkM, walkI, buildM, buildI),
                                       Zr=zr[m], Zr_wiedemann_est=9 * N * K,
                                       solver_calls=stats['calls'], solver_sec=stats['msolve_sec'],
                                       sec=time.time() - t0, log_ok=True)
        if done and first is None:
            first = trials
        if first and trials > cap * first:
            break
    for m in modes:
        done.setdefault(m, dict(censored=True, trials=trials))
    stats['deg_hist'] = {str(k): stats['deg'].count(k) for k in sorted(set(stats['deg']))}
    del stats['deg']
    stats['solver_sec'] = stats.pop('msolve_sec')
    return done, stats


def main():
    pp = int(sys.argv[1]); seed = int(sys.argv[2]); ntarg = int(sys.argv[3]); nrho = int(sys.argv[4]); out = sys.argv[5]
    cap = float(sys.argv[6]) if len(sys.argv) > 6 else 2.0
    T.setp(pp); assert pp % 4 == 3
    T.find_field()
    rng = random.Random(seed)
    t0 = time.time()
    a, b, r = T.find_curve(rng)
    Gp = T.rand_point(a, b, rng)
    res = dict(p=pp, n=3, seed=seed, cmod=T.cmod, a=a, b=b, r=r, log2r=math.log2(r), cap=cap,
               gen_sec=time.time() - t0, solver='counted_gb (F4 + Wiedemann + proots + Horner eigen)', targets=[])
    for k in range(ntarg):
        w = rng.randrange(1, r); Q = T.smul(w, Gp, a)
        rho = [T.rho_run(Gp, Q, a, r, w, rng) for _ in range(nrho)]
        ic, st = ic_run(Gp, Q, a, b, r, w, rng, cap)
        res['targets'].append(dict(w=w, rho=rho, ic=ic, decomposition=st))
        print(pp, seed, k, 'rho', [x['steps'] for x in rho],
              {m: (v['trials'], v.get('phases') and sum(x[0] for x in v['phases'].values())) for m, v in ic.items()},
              st['status'], round(st['solver_sec'], 1), flush=True)
    json.dump(res, open(out, 'w'), indent=1)


if __name__ == '__main__':
    main()
