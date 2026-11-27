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

by_x0 = {}


def decomposition_counts(xset):
    """Ordered triple counts for every reachable sum over this base."""
    pts = [by_x0[x] for x in xset]
    counts = {}
    for P in pts:
        for Q in pts:
            for R in pts:
                s = eadd(eadd(P, Q), R)
                if s is not None:
                    counts[s] = counts.get(s, 0) + 1
    return counts

def run(k, family="window", timeout=900, threads=8, want_count=None, force_xq=None):
    aff = curve_affine()
    by_x = {}
    for q in aff: by_x.setdefault(q[0], q)
    by_x0.clear(); by_x0.update(by_x)
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
    if want_count is None:
        for i in range(len(pts)):
            for j in range(len(pts)):
                for l in range(len(pts)):
                    s = eadd(eadd(pts[i], pts[j]), pts[l])
                    if s is not None: target = s; break
                if target: break
            if target: break
    else:
        counts = decomposition_counts(xset)
        cands = [t for t, c in counts.items() if c == want_count and t is not None]
        if not cands:
            return dict(k=k, family=family, actual_k=len(xset), error=f"no target with count {want_count}")
        target = cands[0]
    xq = force_xq if force_xq is not None else target[0]
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

def s4_eval(x1, x2, x3, x4):
    """S4 at concrete field elements, via the two-quadratic resultant."""
    def abc(u, v):
        A_ = (u - v) * (u - v) % P
        B_ = (-2 * (((u + v) * (u * v + A) + 2 * B) % P)) % P
        C_ = (((u * v - A) % P) ** 2 - 4 * B * (u + v)) % P
        return A_, B_, C_
    a1, b1, c1 = abc(x1, x2)
    a2, b2, c2 = abc(x3, x4)
    ac = (a1 * c2 - a2 * c1) % P
    ab = (a1 * b2 - a2 * b1) % P
    bc = (b1 * c2 - b2 * c1) % P
    return (ac * ac - ab * bc) % P

def xspace_count(xset, xq):
    """Ordered triples of base x-values satisfying S4(x1,x2,x3,xq)=0."""
    n = 0
    for a in xset:
        for b in xset:
            for c in xset:
                if s4_eval(a, b, c, xq) == 0:
                    n += 1
    return n

def xspace_counts_fast(xset, xqs):
    """Vectorised x-space counts for a list of target x-coordinates."""
    import numpy as np
    xs = np.array(xset, dtype=object)
    out = {}
    for xq in xqs:
        total = 0
        for a in xset:
            for b in xset:
                u, v = a, b
                A1 = (u - v) * (u - v) % P
                B1 = (-2 * (((u + v) * (u * v + A) + 2 * B) % P)) % P
                C1 = (((u * v - A) % P) ** 2 - 4 * B * (u + v)) % P
                c = xs
                A2 = ((c - xq) % P) ** 2 % P
                B2 = (-2 * (((c + xq) * (c * xq + A) + 2 * B) % P)) % P
                C2 = (((c * xq - A) % P) ** 2 - 4 * B * (c + xq)) % P
                ac = (A1 * C2 - A2 * C1) % P
                ab = (A1 * B2 - A2 * B1) % P
                bc = (B1 * C2 - B2 * C1) % P
                total += int(np.count_nonzero((ac * ac - ab * bc) % P == 0))
        out[xq] = total
    return out

def matched_targets_xspace(k, pool=120):
    """One target per family with the SAME x-space solution count."""
    sets = {}
    for fam in ("window", "pullback"):
        aff = curve_affine(); by_x = {}
        for q in aff: by_x.setdefault(q[0], q)
        xs_all = sorted(by_x)
        if fam == "window":
            xset = xs_all[:k]
        else:
            Tset, xset = set(), []
            for x in (v for v in xs_all if v):
                Tset.add(x * x % P)
                pre = [y for y in xs_all if (y * y % P) in Tset]
                if len(pre) >= k:
                    xset = pre[:k]; break
        sets[fam] = xset
    pool_t = {}
    for fam, xset in sets.items():
        pts = [by_x0[x] for x in xset] if by_x0 else None
        # reachable x-coordinates (cheap: point triples on that base)
        aff = curve_affine(); by_x = {}
        for q in aff: by_x.setdefault(q[0], q)
        pts = [by_x[x] for x in xset]
        cand = {}
        for i in range(0, len(pts), max(1, len(pts)//pool)):
            for j in range(len(pts)):
                s = eadd(eadd(pts[i], pts[j]), pts[0])
                if s is not None:
                    cand.setdefault(s[0], xspace_count(xset, s[0]))
        pool_t[fam] = cand
    best = None
    for xq_w, c in pool_t["window"].items():
        if pool_t["pullback"].get(xq_w) == c and c > 0:
            best = (c, xq_w); break
    return sets, best

def matched_targets(k):
    """Pick one target per family with the same ordered decomposition count."""
    sets = {}
    for fam in ("window", "pullback"):
        aff = curve_affine(); by_x = {}
        for q in aff: by_x.setdefault(q[0], q)
        by_x0.clear(); by_x0.update(by_x)
        xs_all = sorted(by_x)
        if fam == "window":
            xset = xs_all[:k]
        else:
            Tset, xset = set(), []
            for x in (v for v in xs_all if v):
                Tset.add(x * x % P)
                pre = [y for y in xs_all if (y * y % P) in Tset]
                if len(pre) >= k:
                    xset = pre[:k]; break
        sets[fam] = xset
    common = None
    for c in range(2, 40):
        ok = all(any(v == c for v in decomposition_counts(sets[f]).values()) for f in sets)
        if ok:
            common = c; break
    return sets, common

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "matchedxf":
        k = int(sys.argv[2]) if len(sys.argv) > 2 else 40
        aff = curve_affine(); by_x = {}
        for q in aff: by_x.setdefault(q[0], q)
        xs_all = sorted(by_x)
        sets = {}
        for fam in ("window", "pullback"):
            if fam == "window":
                xset = xs_all[:k]
            else:
                Tset, xset = set(), []
                for x in (v for v in xs_all if v):
                    Tset.add(x * x % P)
                    pre = [y for y in xs_all if (y * y % P) in Tset]
                    if len(pre) >= k:
                        xset = pre[:k]; break
            sets[fam] = xset
        wpts = [by_x[x] for x in sets["window"]]
        cand = []
        for i in range(12):
            for j in range(12):
                t = eadd(wpts[i], wpts[j])
                if t and t[0] not in cand: cand.append(t[0])
        cand = cand[:16]
        cw = xspace_counts_fast(sets["window"], cand)
        cp = xspace_counts_fast(sets["pullback"], cand)
        hit = [(c, xq) for xq, c in cw.items() if cp.get(xq) == c and c > 0]
        print("common (count, xq):", hit[:5], flush=True)
        if hit:
            c, xq = hit[0]
            for fam in ("window", "pullback"):
                print(run(k, fam, force_xq=xq), flush=True)
        sys.exit(0)
    if len(sys.argv) > 1 and sys.argv[1] == "matchedx":
        k = int(sys.argv[2]) if len(sys.argv) > 2 else 20
        sets, best = matched_targets_xspace(k)
        print("x-space matched:", best, flush=True)
        if best:
            c, xq = best
            for fam in ("window", "pullback"):
                print(run(k, fam, want_count=None, force_xq=xq), flush=True)
        sys.exit(0)
    if len(sys.argv) > 1 and sys.argv[1] == "matched":
        k = int(sys.argv[2]) if len(sys.argv) > 2 else 30
        sets, c = matched_targets(k)
        print("common decomposition count:", c, flush=True)
        for fam in ("window", "pullback"):
            print(run(k, fam, want_count=c), flush=True)
    else:
        for k in [int(a) for a in sys.argv[1:]] or [12, 20]:
            for fam in ("window", "pullback"):
                print(run(k, fam), flush=True)
