#!/usr/bin/env python3
"""L25: matched-ideal-degree k=30 window vs pullback comparison.

Self-contained copy of the S4 + membership builder from
scripts/ic_semaev_msolve.py (L13, PR #5137), plus target selection by ordered
decomposition-triple count and a matched-degree timing run.

Usage:
    python3 scripts/l25_matched_count.py scan   # degree table, -t 2, 4 jobs
    python3 scripts/l25_matched_count.py runx   # matched x-space count 30, -t 8
    python3 scripts/l25_matched_count.py run    # matched degree-60 control, -t 8
"""
import collections
import concurrent.futures
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import time

P, A, B = 4099, 1, 1
MATCHED = [("window", 2037), ("pullback", 2575)]
MATCHED_XSPACE = [("window", 3106), ("pullback", 3106)]


# ── dense polynomials over F_p ───────────────────────────────────────
def add(u, v):
    w = dict(u)
    for e, c in v.items():
        w[e] = (w.get(e, 0) + c) % P
        if w[e] == 0:
            del w[e]
    return w


def mul(u, v):
    w = {}
    for e1, c1 in u.items():
        for e2, c2 in v.items():
            e = tuple(a + b for a, b in zip(e1, e2))
            w[e] = (w.get(e, 0) + c1 * c2) % P
    return {e: c for e, c in w.items() if c}


def scale(u, s):
    return {e: (c * s) % P for e, c in u.items() if (c * s) % P}


def const(c, n=3):
    return {tuple([0] * n): c % P}


def var(i, n=3):
    e = [0] * n
    e[i] = 1
    return {tuple(e): 1}


def s3_abc(u, v):
    diff = add(u, scale(v, -1))
    a_ = mul(diff, diff)
    s = add(u, v)
    pr = mul(u, v)
    inner = add(pr, const(A))
    t = add(mul(s, inner), const(2 * B))
    b_ = scale(t, -2)
    uu = add(pr, const(-A))
    c_ = add(mul(uu, uu), scale(s, -4 * B))
    return a_, b_, c_


def s4(x1, x2, x3, x4):
    a1, b1, c1 = s3_abc(x1, x2)
    a2, b2, c2 = s3_abc(x3, x4)
    ac = add(mul(a1, c2), scale(mul(a2, c1), -1))
    ab = add(mul(a1, b2), scale(mul(a2, b1), -1))
    bc = add(mul(b1, c2), scale(mul(b2, c1), -1))
    return add(mul(ac, ac), scale(mul(ab, bc), -1))


def membership(xs, i, n=3):
    h = const(1, n)
    for w in xs:
        h = mul(h, add(var(i, n), const(-w, n)))
    return h


def render(poly, names=("x1", "x2", "x3")):
    terms = []
    for e, c in sorted(poly.items()):
        mono = []
        for v, k in zip(names, e):
            if k == 1:
                mono.append(v)
            elif k > 1:
                mono.append(f"{v}^{k}")
        mono = "*".join(mono) if mono else "1"
        terms.append(f"{c}*{mono}" if mono != "1" else f"{c}")
    return "+".join(terms) if terms else "0"


# ── curve arithmetic ─────────────────────────────────────────────────
def curve_affine():
    from sympy import sqrt_mod
    pts = []
    for x in range(P):
        r = (x * x * x + A * x + B) % P
        y = sqrt_mod(r, P)
        if y is not None:
            pts.append((x, y))
            if y:
                pts.append((x, (-y) % P))
    return pts


def eadd(p1, p2):
    if p1 is None:
        return p2
    if p2 is None:
        return p1
    x1, y1 = p1
    x2, y2 = p2
    if x1 == x2:
        if (y1 + y2) % P == 0:
            return None
        lam = (3 * x1 * x1 + A) * pow(2 * y1, -1, P) % P
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, P) % P
    x3 = (lam * lam - x1 - x2) % P
    return (x3, (lam * (x1 - x3) - y1) % P)


def base_sets(k):
    by_x = {}
    for q in curve_affine():
        by_x.setdefault(q[0], q)
    xs_all = sorted(by_x)
    win = xs_all[:k]
    tset, pull = set(), []
    for x in (v for v in xs_all if v):
        tset.add(x * x % P)
        pre = [y for y in xs_all if (y * y % P) in tset]
        if len(pre) >= k:
            pull = pre[:k]
            break
    if len(pull) < k:
        raise SystemExit(f"pullback k={k}: only {len(pull)} on-curve x-values")
    return by_x, win, pull


def triple_counts(by_x, xset):
    pts = {x: by_x[x] for x in xset}
    cnt = collections.Counter()
    for i in xset:
        for j in xset:
            pij = eadd(pts[i], pts[j])
            if pij is None:
                continue
            for l in xset:
                s = eadd(pij, pts[l])
                if s is not None:
                    cnt[s[0]] += 1
    return cnt


# ── msolve ───────────────────────────────────────────────────────────
def build_input(xset, xq):
    polys = [render(s4(var(0), var(1), var(2), const(xq))),
             render(membership(xset, 0)),
             render(membership(xset, 1)),
             render(membership(xset, 2))]
    return "x1,x2,x3\n%d\n" % P + ",\n".join(polys) + "\n"


def solve(xset, xq, threads, timeout=300):
    src = build_input(xset, xq)
    sha = hashlib.sha256(src.encode()).hexdigest()
    with tempfile.TemporaryDirectory() as d:
        inp, out = os.path.join(d, "in.ms"), os.path.join(d, "out.ms")
        open(inp, "w").write(src)
        t0 = time.time()
        try:
            r = subprocess.run(
                ["msolve", "-f", inp, "-o", out, "-t", str(threads), "-v", "2"],
                capture_output=True, text=True, timeout=timeout)
        except subprocess.TimeoutExpired:
            return dict(family=None, xq=xq, timeout=True, wall=timeout, sha256=sha)
        wall = time.time() - t0
        m = re.search(r"degree of ideal\s+(\d+)", r.stderr)
        mt = re.search(r"msolve overall time\s+([\d.]+) sec \(elapsed\) / ([\d.]+) sec \(cpu\)",
                       r.stderr)
        return dict(timeout=False, wall=round(wall, 2), rc=r.returncode, sha256=sha,
                    degree=int(m.group(1)) if m else None,
                    msolve_elapsed=float(mt.group(1)) if mt else None,
                    msolve_cpu=float(mt.group(2)) if mt else None)


def scan(k=30):
    by_x, win, pull = base_sets(k)
    sets = {"window": win, "pullback": pull}
    jobs = []
    for name, xs in sets.items():
        cnt = triple_counts(by_x, xs)
        for target_count in (6, 13, 27, 48):
            for xq, c in cnt.items():
                if c == target_count:
                    jobs.append((name, xq, target_count))
                    break

    def work(job):
        name, xq, c = job
        r = solve(sets[name], xq, threads=2)
        r.update(family=name, xq=xq, triples=c, threads=2)
        print(r, flush=True)
        return r

    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as ex:
        rows = list(ex.map(work, jobs))
    json.dump(rows, open("scan.json", "w"), indent=1)
    return rows


def run(k=30, reps=2, threads=8, pair=None, out="matched60.json"):
    by_x, win, pull = base_sets(k)
    sets = {"window": win, "pullback": pull}
    rows = []
    for rep in range(reps):
        for name, xq in (pair or MATCHED):
            r = solve(sets[name], xq, threads=threads)
            r.update(family=name, xq=xq, rep=rep, threads=threads)
            print(r, flush=True)
            rows.append(r)
    json.dump(rows, open(out, "w"), indent=1)
    return rows


def runx(k=30, reps=2, threads=8):
    return run(k=k, reps=reps, threads=threads, pair=MATCHED_XSPACE,
               out="matched_xspace30.json")


if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "run"
    if mode == "scan":
        scan()
    elif mode == "runx":
        runx()
    elif mode == "run":
        run()
    else:
        raise SystemExit("usage: l25_matched_count.py [scan|runx|run]")
