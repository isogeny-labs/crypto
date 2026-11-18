#!/usr/bin/env python3
"""Independent checker for `cert` fields in prime_fast_j0_{sym,multi} rows.
Shares no code with the solvers. Checks: p prime, p=1 mod 3, n prime, G on
y^2=x^3+b (b from curve name), n*G=O, every Q = k*G, and every recovered
scalar r satisfies r*G = Q. Usage: verify_pf_cert.py rows.jsonl [...]"""
import json, sys

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def run(path):
    bad = rows = certs = 0
    for line in open(path):
        r = json.loads(line); c = r.get("cert")
        if not c: continue
        rows += 1
        p = c.get("p", r.get("p")); n = c.get("n", r.get("n"))
        b = int(r["curve"].split("-b")[1])
        def add(P, Q):
            if P is None: return Q
            if Q is None: return P
            (x1,y1),(x2,y2) = P,Q
            if x1==x2 and (y1+y2)%p==0: return None
            l = (3*x1*x1*pow(2*y1,-1,p) if P==Q else (y2-y1)*pow(x2-x1,-1,p)) % p
            x = (l*l-x1-x2) % p
            return (x, (l*(x1-x)-y1) % p)
        def mul(P, k):
            R = None
            while k:
                if k & 1: R = add(R, P)
                P = add(P, P); k >>= 1
            return R
        g = tuple(c["g"])
        ok = is_prime(p) and p%3==1 and is_prime(n) and (g[1]**2-g[0]**3-b)%p==0 and mul(g,n) is None
        if "k" in c and isinstance(c["k"], list):
            items = list(zip(c["q"], c["k"], c["sym_rec"], c["base_rec"] or [None]*len(c["k"])))
        else:
            if c.get("sym_rec") is None or c.get("plain3_rec") is None:
                ok = False  # a failed solve must not pass silently
            items = [(c["q"], c["k"], c.get("sym_rec"), c.get("plain3_rec"))]
        for q, k, *recs in items:
            q = tuple(q); ok &= mul(g,k) == q
            for x in recs:
                if x is not None: ok &= mul(g,x) == q; certs += 1
        bad += not ok
    print(f"{path}: rows_with_cert={rows} recovered_scalars_checked={certs} failing_rows={bad}")
    return bad

if __name__ == "__main__":
    sys.exit(1 if sum(run(p) for p in sys.argv[1:]) else 0)
