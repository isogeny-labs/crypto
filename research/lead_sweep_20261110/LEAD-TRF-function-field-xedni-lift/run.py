#!/usr/bin/env python3
"""Exhaustive toy: lift probability exponent of linear-section shape on y^2=x^3+a(t)x+b(t) over F_p(t)."""
import sys, json, math, random
from collections import Counter

def slope(ps, vals):
    xs = [math.log(p) for p in ps]; ys = [math.log(v) for v in vals]
    n = len(xs); mx = sum(xs)/n; my = sum(ys)/n
    return -sum((x-mx)*(y-my) for x, y in zip(xs, ys))/sum((x-mx)**2 for x in xs)

def enumerate_p(p):
    """All (a0,a1,x0,x1,y0,y1) in F_p^6; b(t)=y^2-x^3-a x as a cubic polynomial; verify
    identity at t=0,1,2 using b's coefficients; return Counter of fibre-0 data (a0,x0,y0)."""
    c0 = Counter(); bad = 0
    for a0 in range(p):
        for a1 in range(p):
            for x0 in range(p):
                for x1 in range(p):
                    x2 = x0*x0 % p; xx = x0*x1*2 % p
                    # x^3 coefficients
                    x3 = [x0**3 % p, 3*x0*x0*x1 % p, 3*x0*x1*x1 % p, x1**3 % p]
                    ax = [a0*x0 % p, (a0*x1+a1*x0) % p, a1*x1 % p]
                    for y0 in range(p):
                        for y1 in range(p):
                            y2 = [y0*y0 % p, 2*y0*y1 % p, y1*y1 % p]
                            b = [(y2[0]-x3[0]-ax[0]) % p, (y2[1]-x3[1]-ax[1]) % p,
                                 (y2[2]-x3[2]-ax[2]) % p, (-x3[3]) % p]
                            t = 2
                            xv = (x0+x1*t) % p; yv = (y0+y1*t) % p; av = (a0+a1*t) % p
                            bv = (b[0]+b[1]*t+b[2]*t*t+b[3]*t**3) % p
                            if (yv*yv - xv**3 - av*xv - bv) % p: bad += 1
                            c0[(a0, x0, y0)] += 1
    return c0, bad

def main():
    smoke = "--smoke" in sys.argv
    primes = [3, 5] if smoke else [5, 7, 11, 13]
    seed = 20261110
    rng = random.Random(seed)
    res = {}; bad_total = 0
    A = {"main_t1": [], "main_t2": [], "planted": []}
    for p in primes:
        c0, bad = enumerate_p(p); bad_total += bad
        tot = p**6
        # two independent prescribed targets (A/A), y != 0
        def tgt():
            return (rng.randrange(p), rng.randrange(p), rng.randrange(1, p))
        t1, t2 = tgt(), tgt()
        A["main_t1"].append(c0[t1]/tot); A["main_t2"].append(c0[t2]/tot)
        # planted control: prescribe only (a0,x0) fixed -> probability over y0 given them = marginal count
        a0, x0 = t1[0], t1[1]
        planted = sum(c0[(a0, x0, y)] for y in range(p))/tot*p**2  # conditional on (a0,x0)... normalised below
        # P(y0=target | a0,x0 fixed to target) = 1/p exactly; measured from counts:
        A["planted"].append(c0[t1]/sum(c0[(a0, x0, y)] for y in range(p)))
    out = {"lead": "LEAD-TRF-function-field-xedni-lift", "seed": seed, "primes": primes,
           "shape": "a,x,y linear in t, b := y^2-x^3-ax (deg 3)", "identity_violations": bad_total}
    if all(v > 0 for k in A for v in A[k]):
        al = {k: slope(primes, A[k]) for k in A}
        out["alpha_hat"] = al; out["probabilities"] = A
        aa = abs(al["main_t1"]-al["main_t2"])
        pos_ok = abs(al["planted"]-1) < 0.2
        main = (al["main_t1"]+al["main_t2"])/2
        out["aa_gap"] = aa; out["positive_control_ok"] = pos_ok
        if smoke:
            out["verdict"] = "inconclusive"
        elif not pos_ok or aa > 0.3 or bad_total:
            out["verdict"] = "inconclusive"
        elif main < 0.5:
            out["verdict"] = "falsified"
        elif 2.5 <= main <= 3.5:
            out["verdict"] = "consistent_with_prediction"
        else:
            out["verdict"] = "inconclusive"
    else:
        out["verdict"] = "inconclusive"; out["note"] = "zero hit counts at these sizes (smoke)"
    print(json.dumps(out))
main()
