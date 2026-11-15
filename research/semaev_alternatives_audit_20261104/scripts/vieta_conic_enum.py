"""Idea 2 check: is the Vieta-conic + square-test route already O(B)?

Q_R(e1,e2) = 0 is quadratic in e2 for each e1, so enumerating all conic points
with e1 <= 2B costs O(B) field ops.  Then a decomposition is exactly a point
whose e2 has a small integer representative and whose discriminant is a square.
So the 'sieve' needs no lattice at all -- unless one wants sub-B.
"""
import random, time
import sympy as sp

def run(pbits, delta, trials=5, a=0, b=7, seed=3):
    random.seed(seed)
    p = int(sp.nextprime(2**pbits))
    B = int(p**delta)
    # base points
    base = []
    for x in range(1, B+1):
        rhs = (x*x*x + a*x + b) % p
        if pow(rhs, (p-1)//2, p) == 1:
            y = sp.sqrt_mod(rhs, p, all_roots=False)
            if y is not None:
                base.append((x, int(y) % p))
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        X1, Y1 = P; X2, Y2 = Q
        if X1 == X2 and (Y1+Y2) % p == 0: return None
        lam = ((3*X1*X1+a)*pow(2*Y1, -1, p) if P == Q else (Y2-Y1)*pow(X2-X1, -1, p)) % p
        X3 = (lam*lam - X1 - X2) % p
        return (X3, (lam*(X1-X3) - Y1) % p)
    ok = 0; steps = 0
    t0 = time.time()
    for _ in range(trials):
        P, Q = random.sample(base, 2)
        R = add(P, Q)
        xR = R[0]
        found = None
        # e1 runs over [2, 2B]; quadratic in e2
        for e1 in range(2, 2*B+1):
            steps += 1
            A2 = 1 % p
            B2 = (-4*xR*xR - 2*xR*e1 - 2*a) % p
            C2 = (xR*xR*e1*e1 - 2*xR*(a*e1 + 2*b) + a*a - 4*b*e1) % p
            disc = (B2*B2 - 4*A2*C2) % p
            if pow(disc, (p-1)//2, p) != 1 and disc != 0:
                continue
            s = sp.sqrt_mod(disc, p, all_roots=False)
            if s is None: continue
            for e2 in {((-B2 + s) * pow(2, -1, p)) % p, ((-B2 - s) * pow(2, -1, p)) % p}:
                e2i = e2 if e2 <= p//2 else e2 - p          # integer rep
                u2 = e1*e1 - 4*e2i
                if u2 < 0: continue
                u = sp.integer_nthroot(u2, 2)[0]
                if u*u != u2: continue
                for su in ({u, -u}):
                    if (e1 + su) % 2: continue
                    x1 = (e1 + su)//2; x2 = (e1 - su)//2
                    if 1 <= x1 <= B and 1 <= x2 <= B:
                        if {x1, x2} == {P[0], Q[0]}:
                            found = (x1, x2); break
                if found: break
            if found: break
        ok += found is not None
    return ok, trials, steps, time.time()-t0, len(base), B

for pbits, delta in ((20, 0.5), (24, 0.5), (28, 0.5)):
    ok, tr, steps, dt, nb, B = run(pbits, delta)
    print(f"p=2^{pbits} delta=1/2 B={B} base={nb}: found {ok}/{tr} in {steps} e1-steps ({dt:.2f}s)")
