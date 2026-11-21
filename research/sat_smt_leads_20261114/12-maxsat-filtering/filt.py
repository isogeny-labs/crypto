#!/usr/bin/env python3
"""Idea 12 pilot: MaxSAT relation filtering vs greedy pipeline. See PROTOCOL.md."""
import sys, json, time, math, random, itertools, multiprocessing as mp, threading
from pysat.formula import WCNF, IDPool
from pysat.card import CardEnc, EncType
from pysat.examples.rc2 import RC2
from pysat.solvers import Glucose4

K = 1
DENS_CAP = 16  # SGE stops when average row weight exceeds this (protocol deviation D1)
W = 4


def gen(N, R, seed):
    rng = random.Random(seed)
    return [tuple(sorted(rng.sample(range(N), W))) for _ in range(R)]


# ---------------------------------------------------------------- matrix
class Mat:
    def __init__(self, rows):
        self.rows = {i: set(r) for i, r in enumerate(rows)}
        self.cols = {}
        for i, r in self.rows.items():
            for c in r:
                self.cols.setdefault(c, set()).add(i)

    def copy(self):
        m = Mat.__new__(Mat)
        m.rows = {i: set(r) for i, r in self.rows.items()}
        m.cols = {c: set(s) for c, s in self.cols.items()}
        return m

    def excess(self):
        return len(self.rows) - len(self.cols)

    def del_row(self, i):
        """delete row i and cascade singleton removal"""
        stack = [i]
        while stack:
            i = stack.pop()
            r = self.rows.pop(i, None)
            if r is None:
                continue
            for c in r:
                s = self.cols[c]
                s.discard(i)
                if not s:
                    del self.cols[c]
                elif len(s) == 1:
                    stack.append(next(iter(s)))

    def singleton(self):
        for c in [c for c, s in self.cols.items() if len(s) == 1]:
            s = self.cols.get(c)
            if s and len(s) == 1:
                self.del_row(next(iter(s)))

    def sim(self, i):
        """simulate del_row(i): return (rows_removed, cols_removed)"""
        dec = {}
        gone = set()
        stack = [i]
        nc = 0
        while stack:
            i = stack.pop()
            if i in gone:
                continue
            gone.add(i)
            for c in self.rows[i]:
                d = dec.get(c, 0) + 1
                dec[c] = d
                left = len(self.cols[c]) - d
                if left == 0:
                    nc += 1
                elif left == 1:
                    for j in self.cols[c]:
                        if j not in gone:
                            stack.append(j)
                            break
        return len(gone), nc

    def score(self, i):
        return sum(1.0 / (len(self.cols[c]) - 1) for c in self.rows[i])


def greedy(mat, mode, rng=None, ncand=48, deadline=None):
    m = mat.copy()
    m.singleton()
    while m.excess() > K:
        ids = list(m.rows)
        if rng is None:
            sc = {i: m.score(i) for i in ids}
        else:
            sc = {i: m.score(i) * (0.5 + rng.random()) for i in ids}
        if mode == 1:
            best = max(ids, key=lambda i: (sc[i], -i))
        else:
            if rng is None and len(ids) <= 300:
                cand = ids
            else:
                cand = sorted(ids, key=lambda i: (-sc[i], i))[:ncand]
            best, bk = None, None
            for i in cand:
                nr, nc = m.sim(i)
                delta = nc - nr  # >= -1
                key = (1, nc) if delta >= 0 else (0, nc, sc[i])
                if bk is None or key > bk:
                    best, bk = i, key
        m.del_row(best)
        if deadline and time.time() > deadline:
            return None
    return m


def verify(rows, sel_rows, sel_cols):
    cs = set(sel_cols)
    used = set()
    for i in sel_rows:
        assert set(rows[i]) <= cs, "row outside columns"
        used |= set(rows[i])
    assert len(set(sel_rows)) - len(cs) >= K, "excess violated"
    return True


# ---------------------------------------------------------------- SGE (Markowitz, structural)
def sge(rows_sets):
    """rows_sets: list of sets. Trim to cols+K rows first. Returns (best_cost, N_at_best, W_at_best, cost0)."""
    rows = {i: set(r) for i, r in enumerate(rows_sets)}
    cols = {}
    for i, r in rows.items():
        for c in r:
            cols.setdefault(c, set()).add(i)
    wt = sum(len(r) for r in rows.values())
    cost0 = len(cols) * wt
    best = (cost0, len(cols), wt)
    import heapq
    heap = [(len(s), c) for c, s in cols.items()]
    heapq.heapify(heap)
    while heap and len(cols) > 1:
        l, c = heapq.heappop(heap)
        if c not in cols or len(cols[c]) != l:
            continue
        if l == 0:
            del cols[c]
            continue
        s = cols[c]
        p = min(s, key=lambda i: (len(rows[i]), i))
        pr = rows[p]
        touched = set()
        for j in list(s):
            if j == p:
                continue
            old = rows[j]
            new = (old | pr)
            new.discard(c)
            wt += len(new) - len(old)
            for d in new - old:
                cols[d].add(j)
                touched.add(d)
            rows[j] = new
        for d in pr:
            if d != c:
                cols[d].discard(p)
                touched.add(d)
        wt -= len(pr)
        del rows[p]
        del cols[c]
        for d in touched:
            heapq.heappush(heap, (len(cols[d]), d))
        cost = len(cols) * wt
        if not rows or wt > DENS_CAP * len(rows):
            break
        if cost < best[0]:
            best = (cost, len(cols), wt)
    return best[0], best[1], best[2], cost0


def finish(rows, sel_rows):
    """trim selection to cols+K rows without losing excess, then SGE."""
    sel = sorted(sel_rows)
    cs = set()
    for i in sel:
        cs |= set(rows[i])
    # trimming extra rows: only happens for MaxSAT arms with surplus; drop from the end
    m = Mat([rows[i] for i in sel])
    while m.excess() > K:
        m.del_row(max(m.rows))
    rs = [m.rows[i] for i in m.rows]
    c, n, w, c0 = sge(rs)
    return dict(cols=len(m.cols), rows=len(m.rows), cost_sel=len(m.cols) * W * len(m.rows),
                cost_sge=c, sge_N=n, sge_W=w)


# ---------------------------------------------------------------- SAT / MaxSAT
def hard_clauses(rrows, cols):
    """rrows: dict id->set(cols). returns (clauses, xvar, yvar, top)"""
    xv = {i: n + 1 for n, i in enumerate(sorted(rrows))}
    yv = {c: len(xv) + n + 1 for n, c in enumerate(sorted(cols))}
    cl = []
    for i, r in rrows.items():
        for c in r:
            cl.append([-xv[i], yv[c]])
    top = len(xv) + len(yv)
    lits = [-v for v in xv.values()] + list(yv.values())
    enc = CardEnc.atmost(lits=lits, bound=len(xv) - K, top_id=top, encoding=EncType.cardnetwrk)
    cl += enc.clauses
    top = max(top, enc.nv)
    return cl, xv, yv, top


def _rc2_child(q, rrows, cols):
    t = time.time()
    cl, xv, yv, top = hard_clauses(rrows, cols)
    w = WCNF()
    for c in cl:
        w.append(c)
    for v in yv.values():
        w.append([-v], weight=1)
    tenc = time.time() - t
    rc = RC2(w)
    model = rc.compute()
    st = rc.oracle.accum_stats()
    ms = set(l for l in model if l > 0)
    q.put(dict(cost=rc.cost, sel_rows=[i for i, v in xv.items() if v in ms],
               sel_cols=[c for c, v in yv.items() if v in ms],
               conflicts=st['conflicts'], decisions=st['decisions'], props=st['propagations'],
               enc_s=tenc, nclauses=len(cl), wall=time.time() - t))


def run_rc2(rrows, cols, T):
    q = mp.Queue()
    p = mp.Process(target=_rc2_child, args=(q, rrows, cols))
    t = time.time()
    p.start()
    try:
        res = q.get(timeout=T)
        p.join()
        res['status'] = 'OPT'
        return res
    except Exception:
        p.kill()
        p.join()
        return dict(status='TIMEOUT', wall=time.time() - t)


def run_any(rrows, cols, start_rows, T):
    """model-improving search from greedy solution. returns dict."""
    t0 = time.time()
    cl, xv, yv, top = hard_clauses(rrows, cols)
    best_rows = set(start_rows)
    best_cols = set()
    for i in best_rows:
        best_cols |= rrows[i]
    B = len(best_cols) - 1
    conf = dec = 0
    iters = 0
    improved = 0
    status = 'TIMEOUT'
    ylits = list(yv.values())
    while True:
        left = T - (time.time() - t0)
        if left <= 0:
            break
        if B < 1:
            status = 'OPT'
            break
        enc = CardEnc.atmost(lits=ylits, bound=B, top_id=top, encoding=EncType.cardnetwrk)
        s = Glucose4(bootstrap_with=cl + enc.clauses)
        s.set_phases([(v if i in best_rows else -v) for i, v in xv.items()] +
                     [(v if c in best_cols else -v) for c, v in yv.items()])
        left = T - (time.time() - t0)
        if left <= 0:
            s.delete()
            break
        tm = threading.Timer(left, lambda: s.interrupt())
        tm.start()
        r = s.solve_limited(expect_interrupt=True)
        tm.cancel()
        st = s.accum_stats()
        conf += st['conflicts']
        dec += st['decisions']
        iters += 1
        if r is True:
            ms = set(l for l in s.get_model() if l > 0)
            # rows fully inside chosen columns
            cs = set(c for c, v in yv.items() if v in ms)
            rs = set(i for i, r_ in rrows.items() if r_ <= cs)
            # shrink: singleton/cascade + drop unused columns
            m = Mat.__new__(Mat)
            m.rows = {i: set(rrows[i]) for i in rs}
            m.cols = {}
            for i, r_ in m.rows.items():
                for c in r_:
                    m.cols.setdefault(c, set()).add(i)
            m.singleton()
            best_rows = set(m.rows)
            best_cols = set(m.cols)
            improved += 1
            B = len(best_cols) - 1
            s.delete()
        elif r is False:
            status = 'OPT'
            s.delete()
            break
        else:
            s.delete()
            break
    return dict(status=status, sel_rows=sorted(best_rows), ncols=len(best_cols), conflicts=conf,
                decisions=dec, iters=iters, improved=improved, wall=time.time() - t0)


# ---------------------------------------------------------------- brute force
def brute(rows, N):
    best = None
    for s in range(1, N + 1):
        for cs in itertools.combinations(range(N), s):
            cset = set(cs)
            inside = [r for r in rows if set(r) <= cset]
            used = set()
            for r in inside:
                used |= set(r)
            if len(inside) - len(used) >= K and inside:
                return len(used)
    return None


def validate():
    ok = 0
    out = []
    for t in range(20):
        N = 10 + t % 3
        R = N + 4 + t % 4
        rows = gen(N, R, 7000 + t)
        b = brute(rows, N)
        m = Mat(rows)
        m.singleton()
        if m.excess() < K or not m.rows:
            out.append(dict(t=t, N=N, R=R, brute=b, note='no excess after singleton'))
            assert b is None, (t, b)
            ok += 1
            continue
        rr = {i: set(r) for i, r in m.rows.items()}
        r1 = run_rc2(rr, set(m.cols), 30)
        g = greedy(Mat(rows), 2)
        r2 = run_any(rr, set(m.cols), list(g.rows), 30)
        verify(rows, r1['sel_rows'], r1['sel_cols'])
        out.append(dict(t=t, N=N, R=R, brute=b, rc2=r1['cost'], any=r2['ncols'], any_status=r2['status'],
                        g2=len(g.cols)))
        assert r1['cost'] == b and r2['ncols'] == b and r2['status'] == 'OPT', out[-1]
        ok += 1
    # planted
    rng = random.Random(99)
    N = 60
    rows = [tuple(sorted(rng.sample(range(9), W))) for _ in range(12)]
    rows = list(dict.fromkeys(rows))
    while len(rows) < 12:
        r = tuple(sorted(rng.sample(range(9), W)))
        if r not in rows:
            rows.append(r)
    rows += [tuple(sorted(rng.sample(range(N), W))) for _ in range(56)]
    m = Mat(rows)
    m.singleton()
    rr = {i: set(r) for i, r in m.rows.items()}
    r1 = run_rc2(rr, set(m.cols), 60)
    g = greedy(Mat(rows), 2)
    r2 = run_any(rr, set(m.cols), list(g.rows), 60)
    verify(rows, r1['sel_rows'], r1['sel_cols'])
    planted = dict(rc2=r1.get('cost'), any=r2['ncols'], any_status=r2['status'], g2=len(g.cols), g0=len(m.cols))
    assert r1['cost'] <= 9 and r2['ncols'] <= 9, planted
    return dict(brute_ok=ok, brute_rows=out, planted=planted)


# ---------------------------------------------------------------- main experiment
def one(N, e, rep, T, full=True):
    seed = int(1000 * math.log2(N)) + 10 * e + rep
    R = math.ceil((1 + e / 100) * N)
    rows = gen(N, R, seed)
    row = dict(N=N, excess_pct=e, rep=rep, seed=seed, R=R)
    m0 = Mat(rows)
    m0.singleton()
    row['g0_rows'], row['g0_cols'] = len(m0.rows), len(m0.cols)
    if m0.excess() < K:
        row['skip'] = 'excess<k after singleton removal'
        return row
    t = time.time(); g1 = greedy(Mat(rows), 1); row['g1_wall'] = time.time() - t
    verify(rows, g1.rows, g1.cols)
    row['g1'] = finish(rows, g1.rows)
    t = time.time(); g2 = greedy(Mat(rows), 2); row['g2_wall'] = time.time() - t
    verify(rows, g2.rows, g2.cols)
    row['g2'] = finish(rows, g2.rows)
    bestg = g2 if len(g2.cols) <= len(g1.cols) else g1
    if not full:
        return row
    # G3
    rng = random.Random(seed + 555)
    t = time.time(); n3 = 0; g3 = bestg
    while time.time() - t < T:
        g = greedy(Mat(rows), 2, rng=rng, ncand=16, deadline=t + T)
        if g is None:
            break
        n3 += 1
        if len(g.cols) < len(g3.cols):
            g3 = g
    verify(rows, g3.rows, g3.cols)
    row['g3'] = finish(rows, g3.rows); row['g3_restarts'] = n3; row['g3_wall'] = time.time() - t
    bestg = g3
    row['greedy_cols'] = len(bestg.cols)
    rr = {i: set(r) for i, r in m0.rows.items()}
    r1 = run_rc2(rr, set(m0.cols), T)
    if r1['status'] == 'OPT':
        verify(rows, r1['sel_rows'], r1['sel_cols'])
        f = finish(rows, r1['sel_rows'])
        assert f['cols'] == r1['cost'], (f, r1['cost'])
        row['rc2'] = dict(status='OPT', conflicts=r1['conflicts'], decisions=r1['decisions'],
                          props=r1['props'], wall=r1['wall'], enc_s=r1['enc_s'], nclauses=r1['nclauses'], **f)
    else:
        row['rc2'] = r1
    r2 = run_any(rr, set(m0.cols), list(bestg.rows), T)
    cs = set()
    for i in r2['sel_rows']:
        cs |= set(rows[i])
    verify(rows, r2['sel_rows'], cs)
    f = finish(rows, r2['sel_rows'])
    row['any'] = dict(status=r2['status'], conflicts=r2['conflicts'], decisions=r2['decisions'],
                      iters=r2['iters'], improved=r2['improved'], wall=r2['wall'], **f)
    return row


if __name__ == '__main__':
    out = sys.argv[2] if len(sys.argv) > 2 else None
    if sys.argv[1] == 'validate':
        v = validate()
        print(json.dumps(v['planted']), v['brute_ok'])
        json.dump(v, open(out, 'w'), indent=1)
    elif sys.argv[1] == 'smoke':
        r = one(int(sys.argv[2]), int(sys.argv[3]), 0, float(sys.argv[4]))
        print(json.dumps(r, indent=1))
    elif sys.argv[1] == 'run':
        T = 5.0
        plan = [(32, 6), (64, 4), (128, 4), (256, 3), (512, 3), (1024, 2)]
        res = []
        for N, reps in plan:
            for rep in range(reps):
                for e in (5, 20, 100):
                    r = one(N, e, rep, T)
                    res.append(r)
                    print(json.dumps({k: (v if not isinstance(v, dict) else {a: b for a, b in v.items() if a in ('status', 'cols', 'cost_sge', 'conflicts')}) for k, v in r.items()}), flush=True)
                    json.dump(dict(T=T, rows=res), open(out, 'w'), indent=1)
        for N in (2048, 4096):
            for e in (5, 20, 100):
                r = one(N, e, 0, T, full=False)
                res.append(r)
                print(json.dumps(r), flush=True)
                json.dump(dict(T=T, rows=res), open(out, 'w'), indent=1)
