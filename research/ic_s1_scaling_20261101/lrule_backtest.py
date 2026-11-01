#!/usr/bin/env python3
"""S1 l-rule (written before any S1 run) back-tested on R2W's recorded whole-run WDSat curves (A config, median Ir/target).
Rule (per cell, per arm/config): start l0, evaluate outward in both directions one l at a time; a direction stops when two successive l are
dearer than the best seen so far (failed/invalid l counts as dearer; l outside [LMIN, n-1] ends the direction); iterate to a fixed point.
Guard (WDSat only): l = 3 and 4 (where valid) must be evaluated even when outward search would not reach them (whole-solve or one-target trial exclusion)."""
INF = float('inf')
R2W = {  # whole run, median Ir/target; INF = failed 0/8 or invalid
 9:  {'K0': {3:INF,4:2.05e9,5:4.41e9,6:1.01e9,7:1.42e9,8:1.96e9},
     'K1': {3:4.81e8,4:5.63e8,5:6.55e8,6:8.17e8,7:1.02e9,8:1.40e9}},
 11: {'K1': {3:INF,4:8.23e9,5:1.19e10,6:3.11e10,7:1.65e10,8:3.85e9,9:6.53e9,10:1.03e10}},
 13: {'K0': {3:6.68e9,4:9.82e9,5:5.36e10,6:1.21e11,7:8.76e10,8:5.77e9,9:9.16e9,10:1.61e10,11:2.63e10,12:2.74e10}},
 15: {'K0': {3:INF,4:8.27e10,5:9.41e11,6:7.78e11,7:1.05e12,8:2.41e12,9:7.92e11,10:1.34e10,11:1.98e10,12:2.57e10,13:3.42e10}},
}
def rule(curve, n, l0, guard=(3, 4), lmin=3):
    seen = {}
    def ev(l):
        if l < lmin or l > n - 1: return None
        seen[l] = curve.get(l, INF); return seen[l]
    ev(l0)
    def best(): return min(seen.values()) if seen else INF
    changed = True
    while changed:
        changed = False
        for d in (-1, +1):
            l = l0 + d; worse = 0
            while True:
                if l < lmin or l > n - 1: break
                if l not in seen: ev(l); changed = True
                if seen[l] > best(): worse += 1
                else: worse = 0
                if worse >= 2: break
                l += d
    for g in guard:
        if g not in seen and lmin <= g <= n - 1: ev(g)
    b = min(seen, key=lambda k: seen[k])
    return b, sorted(seen)
if __name__ == '__main__':
    exp = {(9,'K0'):6,(9,'K1'):3,(11,'K1'):8,(13,'K0'):7+1,(15,'K0'):10}
    for n, d in R2W.items():
        for fam, cv in d.items():
            b, ev = rule(cv, n, n - 4)
            print(f'n={n} {fam}: start l0=n-4={n-4}; rule picks l={b} (R2W best {exp[(n,fam)]}); evaluated l={ev}', 'OK' if b == exp[(n,fam)] else 'MISMATCH')
