#!/usr/bin/env python3
"""L13: emit the Semaev S4 + membership system for msolve and time it.

System (m = 3 decomposition of a target Q with x-coordinate xQ):
    S4(x1,x2,x3,xQ) = 0,   h(x1) = h(x2) = h(x3) = 0
with h the membership polynomial of the base x-set (degree k), so this is the
same object as `crypto/examples/ic_semaev_gb.rs`, but solved by msolve's F4
instead of the in-repo dense F4.
"""
import subprocess, sys, time, tempfile, os, random

P = 4099
A, B = 1, 1

# ── dense polynomials over F_p as dict[exponent tuple] = coeff ──────
def add(u, v, p=P):
    w = dict(u)
    for e, c in v.items():
        w[e] = (w.get(e, 0) + c) % p
        if w[e] == 0: del w[e]
    return w

def mul(u, v, p=P):
    w = {}
    for e1, c1 in u.items():
        for e2, c2 in v.items():
            e = tuple(a + b for a, b in zip(e1, e2))
            w[e] = (w.get(e, 0) + c1 * c2) % p
    return {e: c for e, c in w.items() if c}

def scale(u, s, p=P):
    return {e: (c * s) % p for e, c in u.items() if (c * s) % p}

def const(c, n=3): return {tuple([0]*n): c % P}
def var(i, n=3):
    e = [0]*n; e[i] = 1
    return {tuple(e): 1}

def s3_abc(u, v):
    """S3(p,q,D) coefficients A,B,C, with p,q polynomials in 3 vars."""
    diff = add(u, scale(v, -1))
    A_ = mul(diff, diff)
    s = add(u, v); pr = mul(u, v)
    inner = add(pr, const(A))
    t = add(mul(s, inner), const(2*B))
    B_ = scale(t, -2)
    uu = add(pr, const(-A))
    C_ = add(mul(uu, uu), scale(s, -4*B))
    return A_, B_, C_

def s4(x1, x2, x3, x4):
    a1, b1, c1 = s3_abc(x1, x2)
    a2, b2, c2 = s3_abc(x3, x4)
    ac = add(mul(a1, c2), scale(mul(a2, c1), -1))
    ab = add(mul(a1, b2), scale(mul(a2, b1), -1))
    bc = add(mul(b1, c2), scale(mul(b2, c1), -1))
    return add(mul(ac, ac), scale(mul(ab, bc), -1))

def membership(xs, i):
    h = const(1)
    for w in xs:
        h = mul(h, add(var(i), const(-w)))
    return h

def render(poly, names=("x1","x2","x3")):
    terms = []
    for e, c in sorted(poly.items()):
        mono = []
        for v, k in zip(names, e):
            if k == 1: mono.append(v)
            elif k > 1: mono.append(f"{v}^{k}")
        mono = "*".join(mono) if mono else "1"
        terms.append(f"{c}*{mono}" if mono != "1" else f"{c}")
    return "+".join(terms) if terms else "0"

# ── curve arithmetic for a decomposable target ─────────────────────
def curve_affine(p=P, a=A, b=B):
    from sympy import sqrt_mod
    pts = []
    for x in range(p):
        r = (x*x*x + a*x + b) % p
        y = sqrt_mod(r, p)
        if y is not None:
            pts.append((x, y))
            if y: pts.append((x, (-y) % p))
    return pts

def eadd(P1, P2, p=P, a=A):
    if P1 is None: return P2
    if P2 is None: return P1
    x1, y1 = P1; x2, y2 = P2
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = (3*x1*x1 + a) * pow(2*y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam*lam - x1 - x2) % p
    return (x3, (lam*(x1 - x3) - y1) % p)

def run(k, family="window", timeout=900, threads=8):
    aff = curve_affine()
    by_x = {}
    for q in aff: by_x.setdefault(q[0], q)
    xs_all = sorted(by_x)
    if family == "window":
        xset = xs_all[:k]
    else:
        Tset = set()
        xset = []
        for x in (v for v in xs_all if v):
            Tset.add(x * x % P)
            pre = [y for y in xs_all if (y * y % P) in Tset]
            if len(pre) >= k:
                xset = pre[:k]
                break
        if len(xset) < k:
            print(f"  [warn] pullback k={k}: only {len(xset)} on-curve x-values", file=sys.stderr)
    pts = [by_x[x] for x in xset]
    target = None
    for i in range(len(pts)):
        for j in range(len(pts)):
            for l in range(len(pts)):
                s = eadd(eadd(pts[i], pts[j]), pts[l])
                if s is not None: target = s; break
            if target: break
        if target: break
    xq = target[0]
    polys = [render(s4(var(0), var(1), var(2), const(xq))),
             render(membership(xset, 0)),
             render(membership(xset, 1)),
             render(membership(xset, 2))]
    src = "x1,x2,x3\n%d\n" % P + ",\n".join(polys) + "\n"
    with tempfile.TemporaryDirectory() as d:
        inp = os.path.join(d, "in.ms"); out = os.path.join(d, "out.ms")
        open(inp, "w").write(src)
        t0 = time.time()
        try:
            r = subprocess.run(["msolve", "-f", inp, "-o", out, "-t", str(threads), "-v", "2"],
                               capture_output=True, text=True, timeout=timeout)
            rc = r.returncode
        except subprocess.TimeoutExpired:
            return dict(k=k, family=family, actual_k=len(xset), timeout=True, wall=timeout,
                        stderr="timeout", out="")
        wall = time.time() - t0
        o = open(out).read() if os.path.exists(out) else ""
        # compact summary of msolve's verbose steps
        lines = [l for l in r.stderr.splitlines() if any(s in l for s in
                 ("deg", "matrix", "pairs", "basis", "dim", "time"))][-6:]
        return dict(k=k, family=family, actual_k=len(xset), timeout=False, wall=round(wall, 2), rc=rc,
                    n_terms=len(polys[0].split("+")), out_head=o[:120],
                    stderr="\n".join(lines))

if __name__ == "__main__":
    for k in [int(a) for a in sys.argv[1:]] or [12, 20]:
        for fam in ("window", "pullback"):
            print(run(k, fam), flush=True)
