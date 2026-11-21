#!/usr/bin/env python3
import sys, json, time, os
from hky_eps import *
OUT = "/tmp/claude-0/-home-user/eb02eff3-ee5a-5113-a25c-b6cc0a65fdff/scratchpad/wave2/KM-ART-DEGEN/cells.jsonl"
PRIMES = (40961, 12289)

_CACHE = {}
def draw_inst(p, family, draw):
    k = (p, family, draw)
    if k not in _CACHE: _CACHE[k] = _draw_inst(p, family, draw)
    return _CACHE[k]

def _draw_inst(p, family, draw):
    for tr in range(100000):
        v = rng(p, draw, tr, "inst" + family)
        if family == "tower":
            a, b, g, xR, _ = v
            if g == 0: continue
            f = S3_nf(p, a, b, g, xR); par = dict(a=a, b=b, g=g, xR=xR)
        else:
            c = v[:4]
            f = box1(p, c); par = dict(c=c)
            if f[(0,0)] == 0 or f[(1,1)] == 0 or f[(1,0)]==0 or f[(0,1)]==0: continue
        if no_zero_on_grid(p, f, 8): return f, par, tr
    raise RuntimeError("no instance")

def planted(p, draw):
    for tr in range(100000):
        a, b, g, _, X = rng(p, draw, tr, "plant")
        if g == 0: continue
        x1, x2 = g, (-g) % p
        c2 = (x1 - x2) ** 2 % p
        c1 = (-2 * (((x1 + x2) * (x1 * x2 + a) + 2 * b))) % p
        c0 = ((x1 * x2 - a) ** 2 - 4 * b * (x1 + x2)) % p
        disc = (c1 * c1 - 4 * c2 * c0) % p
        if pow(disc, (p - 1) // 2, p) != 1: continue
        # sqrt via brute (p small)
        from math import isqrt
        s = next(s for s in range(p) if s * s % p == disc)
        xR = (-c1 + s) * pow(2 * c2, p - 2, p) % p
        f = S3_nf(p, a, b, g, xR)
        return f, dict(a=a, b=b, g=g, xR=xR), tr

def emit(row):
    with open(OUT, "a") as fh: fh.write(json.dumps(row) + "\n")
    print(json.dumps({k: row[k] for k in row if k not in ("rows",)}), flush=True)

def _done():
    d = set()
    if os.path.exists(OUT):
        for l in open(OUT):
            r = json.loads(l); d.add((r["kind"], r["family"], r["p"], r["draw"], r["t"], r["eps"], r.get("s")))
    return d
DONE = _done()

def cell(kind, family, p, draw, t, eps, f, par, cmax, extra=None):
    if (kind, family, p, draw, t, eps, (extra or {}).get("s")) in DONE: return
    t0 = time.time()
    unit = None
    if eps == 0: unit = (f.get((0, 0), 0) % p != 0)
    if eps == 0 and not unit:
        r = dict(dF=None, ffd=None, censored=False, rows=[], note="f not a unit in R_0 (skipped)")
    else:
        r = dF(p, t, eps, f, cmax)
    row = dict(kind=kind, family=family, p=p, draw=draw, t=t, eps=eps, cmax=cmax, unit_R0=unit, params=par,
               wall_s=round(time.time() - t0, 2)); row.update(r)
    if extra: row.update(extra)
    emit(row)

if __name__ == "__main__":
    which = sys.argv[1]
    if which == "main":
        tmin, tmax = int(sys.argv[2]), int(sys.argv[3]); ndraw = int(sys.argv[4])
        for t in range(tmin, tmax + 1):
            cmax = 7 if t <= 7 else 6
            for fam in ("tower", "w1"):
                for p in PRIMES:
                    for d in range(ndraw):
                        f, par, tr = draw_inst(p, fam, d)
                        for eps in (1, 0, 5):
                            cell("main", fam, p, d, t, eps, f, par, cmax, dict(tries=tr))
    elif which == "planted":
        for t in range(2, 6):
            for p in PRIMES:
                for d in range(3):
                    f, par, tr = planted(p, d)
                    for eps in (1, 0, 5):
                        cell("planted", "tower", p, d, t, eps, f, par, 7)
    elif which == "twist":
        import random
        for t in range(2, 6):
            for p in PRIMES:
                for d in range(3):
                    f, par, tr = draw_inst(p, "tower", d)
                    s = 2 + (rng(p, d, t, "twist")[0] % (p - 3))
                    eps = pow(s, 1 << t, p)
                    fs = {k: c * pow(s, k[0] + k[1], p) % p for k, c in f.items()}
                    cell("twist_eps_sK", "tower", p, d, t, eps, f, par, 7, dict(s=s))
                    cell("twist_ref_eps1_fs", "tower", p, d, t, 1, fs, par, 7, dict(s=s))
