import sys, json, time, random, itertools, math
from fractions import Fraction as Fr
import numpy as np
from n3lib import fpoly, ksum, degsum
P = 2147483647  # 2^31-1, products fit in int64

def rank_mod(A, p=P):
    A = A.copy() % p; r = 0; rows, cols = A.shape
    for c in range(cols):
        if r == rows: break
        nz = np.nonzero(A[r:, c])[0]
        if nz.size == 0: continue
        i = r + nz[0]
        if i != r: A[[r, i]] = A[[i, r]]
        inv = pow(int(A[r, c]), -1, p); A[r] = A[r] * inv % p
        col = A[:, c].copy(); col[r] = 0
        nzr = np.nonzero(col)[0]
        if nzr.size: A[nzr] = (A[nzr] - (col[nzr, None] * A[r][None, :]) % p) % p
        r += 1
    return r

def pmulp(f, g, p=P):
    h = {}
    for e1, c1 in f.items():
        for e2, c2 in g.items():
            e = tuple(a+b for a, b in zip(e1, e2)); h[e] = (h.get(e, 0) + c1*c2) % p
    return {e: c for e, c in h.items() if c}

class Dcalc:
    def __init__(self, f, m, d, lmax):
        self.m, self.d = m, d; self.fp = [{(0,)*m: 1}]
        for j in range(lmax): self.fp.append(pmulp(self.fp[-1], f))
    def D(self, Mset, j, R):
        """dim{h : supp(f^j h) ⊆ M}, h supported in [0,R-jd]^m (Newton-polytope necessary region)."""
        m, d = self.m, self.d; H = list(itertools.product(range(R - j*d + 1), repeat=m))
        if not H: return 0
        out = {}  # monomial outside M -> row
        fj = self.fp[j]; cols = []
        for ci, h in enumerate(H):
            for e, c in fj.items():
                s = tuple(a+b for a, b in zip(h, e))
                if s not in Mset: out.setdefault(s, {})[ci] = c
        if not out: return len(H)
        A = np.zeros((len(out), len(H)), dtype=np.int64)
        for ri, row in enumerate(out.values()):
            for ci, c in row.items(): A[ri, ci] = c
        return len(H) - rank_mod(A)
    def bound(self, M, l, R):
        Ms = set(M); Ds = [self.D(Ms, j, R) for j in range(1, l+1)]
        Ns = [sum(1 for s in M if min(s) >= j*self.d) for j in range(1, l+1)]
        return Ds, Ns

def staircase_opt(l, R, d=2):
    """exact max over 2D downsets in [0,R]^2 of sum k / sum deg (Dinkelbach + DP over nonincreasing column heights)."""
    def k(x, y): return min(l, min(x, y)//d)
    lam = 0.0; best = None
    for it in range(60):
        # column x, height h: value v(x,h)=sum_{y<h} (k - lam*(x+y))
        V = [[0.0]*(R+2) for _ in range(R+1)]
        for x in range(R+1):
            acc = 0.0
            for h in range(1, R+2):
                y = h-1; acc += k(x, y) - lam*(x+y); V[x][h] = acc
        # DP: heights nonincreasing in x
        F = [V[0][h] for h in range(R+2)]; arg = [[(h,) ] for h in range(R+2)]
        choice = [[None]*(R+2) for _ in range(R+1)]
        for x in range(1, R+1):
            G = [0.0]*(R+2); ch = [0]*(R+2)
            # G[h] = V[x][h] + max_{h' >= h} F[h']
            suf = [0.0]*(R+3); sufa = [0]*(R+3); suf[R+2] = -1e300
            for h in range(R+1, -1, -1):
                if F[h] >= suf[h+1]: suf[h], sufa[h] = F[h], h
                else: suf[h], sufa[h] = suf[h+1], sufa[h+1]
            for h in range(R+2): G[h] = V[x][h] + suf[h]; ch[h] = sufa[h]
            choice[x] = ch; F = G
        h = max(range(R+2), key=lambda h: F[h]); hs = [h]
        for x in range(R, 0, -1): h = choice[x][h]; hs.append(h)
        hs = hs[::-1]
        M = [(x, y) for x in range(R+1) for y in range(hs[x])]
        num = sum(k(*s) for s in M); den = sum(sum(s) for s in M)
        if den == 0 or F[max(range(R+2), key=lambda h: F[h])] <= 1e-12: break
        new = num/den
        if best is not None and abs(new - lam) < 1e-13: break
        lam = new; best = (Fr(num, den), hs)
    return best

if __name__ == "__main__":
    mode = sys.argv[1]; res = {}; T0 = time.time()
    if mode == "B0":
        for l in (5, 10, 30):
            for R in (2*l, 2*l+2, 2*l+4, 2*l+8, 3*l):
                v, hs = staircase_opt(l, R); res[f"l{l}_R{R}"] = dict(val=float(v), frac=str(v), heights=hs)
                print(l, R, float(v), flush=True)
        # box closed form check
        def boxval(m, d, l, t=0):
            M = list(itertools.product(range(d*l+t+1), repeat=m)); return Fr(ksum(M, d, l), degsum(M))
        res["box_m2"] = {l: float(boxval(2, 2, l)) for l in (1, 2, 3, 5, 10)}
        res["box_m3"] = {l: float(boxval(3, 4, l)) for l in (1, 2, 3)}
        # B1 per-monomial LP: max sum k(s) y_s s.t. sum deg(s) y_s = 1, y>=0  -> max_s k/deg
        from scipy.optimize import linprog
        for m, d, l in ((2, 2, 5), (3, 4, 3), (4, 8, 2)):
            M = list(itertools.product(range(d*l+3), repeat=m)) if m < 4 else list(itertools.product(range(0, d*l+1, 2), repeat=m))
            M = [s for s in M if sum(s) > 0]
            c = [-min(l, min(s)//d) for s in M]; A = [[sum(s) for s in M]]
            r = linprog(c, A_eq=A, b_eq=[1], bounds=(0, None), method="highs")
            res[f"B1_m{m}_l{l}"] = dict(lp=-r.fun, one_over_md=1/(m*d)); print("B1", m, l, -r.fun, 1/(m*d), flush=True)
    elif mode in ("B2", "B3"):
        rng = random.Random(0)
        m, d = (2, 2) if mode == "B2" else (3, 4)
        f, crv = fpoly(m, P, rng)
        ls = (3, 5) if mode == "B2" else (1, 2)
        for l in ls:
            R = d*l + 4 if m == 2 else d*l + 2
            dc = Dcalc(f, m, d, l)
            def score(M):
                if not M: return 0.0, None
                Ds, Ns = dc.bound(M, l, R); assert all(a <= b for a, b in zip(Ds, Ns)), (Ds, Ns)
                return sum(Ds)/degsum(M), (Ds, Ns)
            fam = {}
            for t in range(0, R - d*l + 1):
                fam[f"box+{t}"] = list(itertools.product(range(d*l+t+1), repeat=m))
            if m == 2:
                for a in range(d*l, R+1):
                    for b in range(d*l, R+1): fam[f"rect{a}x{b}"] = [(x, y) for x in range(a+1) for y in range(b+1)]
                for c in range(1, 4): fam[f"cutcorner{c}"] = [s for s in fam["box+0"] if not (s[0]+s[1] < c)]
                for c in range(1, 4): fam[f"cutTop{c}"] = [s for s in fam["box+0"] if not (s[0]+s[1] > 2*d*l - c)]
                for r in (0.6, 0.8): fam[f"diamond{r}"] = [s for s in itertools.product(range(R+1), repeat=2) if abs(s[0]-s[1]) <= r*R]
                v, hs = staircase_opt(l, R); fam["stair_opt"] = [(x, y) for x in range(R+1) for y in range(hs[x])]
                fam["rect_hollow"] = [s for s in fam["box+0"] if not (0 < s[0] < d*l and 0 < s[1] < d*l and min(s) < d)]
            for nm, M in fam.items():
                sc, info = score(M); res[f"l{l}_{nm}"] = dict(rho_D=sc, rho_k=ksum(M, d, l)/degsum(M), D=info[0] if info else None, N=info[1] if info else None)
                print(l, nm, sc, ksum(M, d, l)/degsum(M), flush=True)
            if m == 2:
                # simulated annealing over arbitrary subsets of [0,R]^2, seed 0, 2000 moves, start = staircase optimum
                cells = list(itertools.product(range(R+1), repeat=2)); cur = set(fam["stair_opt"])
                cs, _ = score(sorted(cur)); best = (cs, sorted(cur)); T = 0.003; t1 = time.time()
                for it in range(2000):
                    c = rng.choice(cells); nxt = set(cur); nxt ^= {c}
                    ns, _ = score(sorted(nxt))
                    if ns >= cs or rng.random() < math.exp((ns - cs)/T): cur, cs = nxt, ns
                    if cs > best[0]: best = (cs, sorted(cur))
                    T *= 0.998
                bs, info = score(best[1])
                down = all(((x-1, y) in set(best[1]) or x == 0) and ((x, y-1) in set(best[1]) or y == 0) for x, y in best[1])
                res[f"l{l}_SA"] = dict(best=bs, stair_opt=float(v), exceeds=bs > float(v) + 1e-9, is_downset=down, M=best[1], D=info[0], N=info[1], secs=time.time()-t1)
                print("SA", l, bs, float(v), down, time.time()-t1, flush=True)
    res["secs"] = time.time() - T0
    json.dump(res, open(f"raw/arm{mode}.json", "w"), indent=1)
