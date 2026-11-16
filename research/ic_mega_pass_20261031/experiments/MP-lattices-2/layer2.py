import json, time, math, sys, random
from common import *
P = int(pari.nextprime(2**256))
def poly_str(g):  # dict (i,j)->c  -> PARI poly in x,y
    return "+".join(f"({c})*x^{i}*y^{j}" for (i, j), c in g.items()) or "0"
def instance(kind, delta, seed, p=P):
    rnd = random.Random(seed); X = int(round(2**(delta*math.log2(p))))
    while True:
        a, b = rnd.randrange(p), rnd.randrange(p); x1, x2 = rnd.randrange(X), rnd.randrange(X)
        if kind == "unplanted":
            xR = rnd.randrange(p); f = S3_specialised(a, b, xR, p); return f, None, X
        if kind == "generic":
            f = {e: rnd.randrange(p) for e in [(2,2),(2,1),(1,2),(2,0),(0,2),(1,1),(1,0),(0,1)]}; f[(2,2)] = 1
            f[(0,0)] = (-sum(c*x1**i*x2**j for (i,j), c in f.items())) % p; return f, (x1, x2), X
        # planted: solve S3(x1,x2,X)=0 mod p for xR
        A2 = (x1-x2)**2 % p; B1 = -2*((x1+x2)*(x1*x2+a)+2*b) % p; C0 = ((x1*x2-a)**2 - 4*b*(x1+x2)) % p
        disc = (B1*B1 - 4*A2*C0) % p
        if A2 == 0 or pari.kronecker(disc, p) != 1: continue
        s = int(pari.Mod(disc, p).sqrt().lift()); xR = (-B1 + s) * pow(2*A2, -1, p) % p
        f = S3_specialised(a, b, xR, p); assert sum(c*x1**i*x2**j for (i,j), c in f.items()) % p == 0
        return f, (x1, x2), X
def reduce_arm(f, l, X, delta, arm, p=P):
    rows = build_rows(f, p, l, 2)
    if arm in ("B", "C"): rows = [r for r in rows if delta*sum(r[0]) - r[1] <= 1e-12]
    mons, idx = deg_order(2, 2*l + 2); used = sorted({e for _, _, g in rows for e in g}, key=lambda e: idx[e])
    ci = {e: j for j, e in enumerate(used)}; r = len(rows)
    M = pari.matrix(len(used), r, [0]*(len(used)*r))
    for j, (_, _, g) in enumerate(rows):
        for e, c in g.items(): M[ci[e], j] = c * X**sum(e)
    R = M * pari.qflll(M)
    vecs = []
    for j in range(R.ncols()):
        v = {used[i]: int(R[i, j]) // X**sum(used[i]) for i in range(len(used)) if R[i, j] != 0}
        nrm = 0.5*math.log(max(1, sum(int(R[i, j])**2 for i in range(len(used)))))  # ln norm
        vecs.append((nrm, v))
    vecs.sort(key=lambda t: t[0]); return vecs, r
def antisym(v): return all(v.get((j, i), 0) == -c for (i, j), c in v.items())
def evaluate(f, root, X, l, delta, arm, p=P, kmax=6):
    vecs, r = reduce_arm(f, l, X, delta, arm)
    lbound = l*math.log(p) - 0.5*math.log(r); lb = math.log(p)
    cand = [t for t in vecs if not (arm == "C" and antisym(t[1]))]
    hg = len(cand) >= 2 and cand[1][0] < lbound
    margin2 = (lbound - cand[1][0]) / lb if len(cand) >= 2 else None
    heur = False; fp = False
    sel = cand[:kmax]
    for i in range(len(sel)):
        for j in range(i+1, len(sel)):
            g1, g2 = sel[i][1], sel[j][1]
            if root is not None and not all(sum(c*root[0]**a*root[1]**b for (a,b), c in g.items()) == 0 for g in (g1, g2)): continue
            res = pari.polresultant(pari(poly_str(g1)), pari(poly_str(g2)), 'y')
            if res == 0: continue
            if root is not None:
                if pari.subst(res, 'x', root[0]) == 0: heur = True
            else:
                for fac in pari.factor(res)[0]:
                    if pari.poldegree(fac, 'x') == 1:
                        x1 = -fac.polcoef(0, 'x') / fac.polcoef(1, 'x')
                        if pari.type(x1) == "t_INT" and abs(int(x1)) < X:
                            fy = pari.subst(pari(poly_str(g1)), 'x', x1)
                            for fy2 in (pari.factor(fy)[0] if fy != 0 else []):
                                if pari.poldegree(fy2, 'y') == 1:
                                    x2 = -fy2.polcoef(0, 'y')/fy2.polcoef(1, 'y')
                                    if pari.type(x2) == "t_INT" and sum(c*int(x1)**a*int(x2)**b for (a,b), c in f.items()) % p == 0: fp = True
            if heur or fp: break
        if heur or fp: break
    return {"hg": hg, "margin_logp": margin2, "heur": heur, "unplanted_recovery": fp, "r": r, "n_vec_root_over_Z": None if root is None else sum(1 for t in cand if all(True for _ in [0]) and sum(c*root[0]**a*root[1]**b for (a,b),c in t[1].items()) == 0)}
if __name__ == "__main__":
    t0 = time.time(); out = []
    grid = {2: [0.06, 0.08, 0.09, 0.10, 0.11, 0.12, 0.14, 0.16, 0.18, 0.20], 3: [0.08, 0.10, 0.11, 0.12, 0.13, 0.14, 0.16, 0.18, 0.20]}
    for l in (2, 3):
        for d in grid[l]:
            for s in range(100, 104):
                f, root, X = instance("planted", d, s)
                for arm in ("A", "B", "C"):
                    rec = {"l": l, "delta": d, "seed": s, "arm": arm, **evaluate(f, root, X, l, d, arm)}
                    out.append(rec); print(json.dumps(rec), flush=True)
    # controls at l=2
    for d in (0.08, 0.10, 0.12, 0.45):
        for s in range(200, 204):
            for kind in ("generic", "unplanted", "planted"):
                if kind == "planted" and d != 0.45: continue
                f, root, X = instance(kind, d, s)
                for arm in ("A", "B", "C"):
                    rec = {"control": kind, "l": 2, "delta": d, "seed": s, "arm": arm, **evaluate(f, root, X, 2, d, arm)}
                    out.append(rec); print(json.dumps(rec), flush=True)
    json.dump({"p": str(P), "runtime_s": time.time()-t0, "records": out}, open("raw/layer2.json", "w"), indent=0)
