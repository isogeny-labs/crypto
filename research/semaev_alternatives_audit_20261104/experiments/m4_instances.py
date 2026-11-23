"""Frozen m=4 (S_5) instance set, as proposed in Idea 15 but corrected.

Design notes
------------
* Arity m = 4 is the smallest arity that can beat rho at all (see memo S3:
  m=3 ties only with an O(1) oracle).
* Base B = floor(p^(1/5)), the T=1 optimum of the cost model
  C = (m!/2^m) p T / B^(m-1) + m B^2.
* At that B a *random* target decomposes with probability ~ B^4/p = p^(-1/5)
  (1/256 at 40 bits), so every instance is PLANTED: pick four base points and
  sum them.  Unplanted targets at this B are negative controls, not instances.
* Each record carries the witness, a group-law check, and (for the first few)
  a direct S_5 evaluation by resultant recursion.

Output: JSONL, one record per instance.
"""
import json, random, sys, time
import sympy as sp

def curve(a, b, p):
    def on(P):
        x, y = P
        return (y*y - (x*x*x + a*x + b)) % p == 0
    def add(P, Q):
        if P is None: return Q
        if Q is None: return P
        X1, Y1 = P; X2, Y2 = Q
        if X1 == X2 and (Y1 + Y2) % p == 0: return None
        if P == Q:
            lam = (3*X1*X1 + a) * pow(2*Y1, -1, p) % p
        else:
            lam = (Y2 - Y1) * pow(X2 - X1, -1, p) % p
        X3 = (lam*lam - X1 - X2) % p
        return (X3, (lam*(X1 - X3) - Y1) % p)
    return on, add

def base_points(p, a, b, B):
    out = []
    for x in range(1, B + 1):
        rhs = (x*x*x + a*x + b) % p
        if rhs == 0:
            continue
        if pow(rhs, (p - 1)//2, p) != 1:
            continue
        y = sp.sqrt_mod(rhs, p, all_roots=False)
        if y is not None:
            out.append((x, int(y) % p))
    return out

def S3_poly(u, v, a, b, var):
    # S_3(u, v, var) as a degree-2 poly in var
    A = (u - v)**2
    Bc = -2*((u + v)*(u*v + a) + 2*b)
    C = (u*v - a)**2 - 4*b*(u + v)
    return sp.Poly(A*var**2 + Bc*var + C, var, domain=sp.GF(sp.Integer(_P)))

_P = None

def S5_value(xs, a, b, p):
    """S_5(x1,x2,x3,x4,x5) mod p via Res_T(S_4(x1..x3,T), S_3(x4,x5,T))."""
    T = sp.Symbol('T'); U = sp.Symbol('U')
    def S3e(u, v, w):
        return ((u - v)**2*w**2 - 2*((u + v)*(u*v + a) + 2*b)*w
                + (u*v - a)**2 - 4*b*(u + v))
    s4 = sp.resultant(S3e(xs[0], xs[1], U), S3e(xs[2], T, U), U)
    s5 = sp.resultant(sp.expand(s4), S3e(xs[3], xs[4], T), T)
    return int(s5) % p

def make(pbits, a, b, n_inst=20, seed=1):
    random.seed(seed)
    p = int(sp.nextprime(2**pbits))
    B = int(round(p ** sp.Rational(1, 5)))
    on, add = curve(a, b, p)
    t0 = time.time()
    base = base_points(p, a, b, B)
    assert len(base) >= 8, (p, B, len(base))
    recs = []
    for i in range(n_inst):
        Ps = random.sample(base, 4)
        R = None
        for P in Ps:
            R = add(R, P)
        xs = [P[0] for P in Ps]
        rec = {
            "p_bits": pbits, "p": str(p), "a": a, "b": b, "B": B,
            "arity": 4, "semaev": "S_5",
            "x": xs, "x_R": R[0],
            "witness_ok_group_law": all(on(P) for P in Ps) and on(R),
            "relation_density_log10": round(float(sp.log(10, sp.Rational(B**4, p))), 6),
        }
        if i < 3:
            rec["S5_value"] = S5_value(xs + [R[0]], a, b, p)
        recs.append(rec)
    return recs, {"p": p, "B": B, "base_size": len(base), "gen_seconds": round(time.time()-t0, 2)}

if __name__ == "__main__":
    out = []
    for pbits in (40, 48):
        recs, meta = make(pbits, 0, 7, n_inst=20)
        print(f"p=2^{pbits} B={meta['B']} |F|={meta['base_size']} "
              f"({meta['gen_seconds']}s) S5={[r.get('S5_value') for r in recs[:3]]}")
        out += recs
    with open(sys.argv[1] if len(sys.argv) > 1 else "m4_instances.jsonl", "w") as f:
        for r in out:
            f.write(json.dumps(r) + "\n")
    print("wrote", len(out), "records")
