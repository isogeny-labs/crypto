#!/usr/bin/env python3
"""Census of hyperelliptic Jacobians y^2=f(x) over F_3 in the isogeny class of Res_{F_{3^n}/F_3} E.
stdlib only. --smoke: n=2 control on a subset. Full: n=2 control + exhaustive n=3 hyperelliptic census."""
import sys, json, random, itertools
P = 3
SEED = 20261110

def polymulmod(a, b, m, k):
    r = [0] * (2 * k - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] = (r[i + j] + x * y) % P
    for d in range(2 * k - 2, k - 1, -1):
        c = r[d]
        if c:
            for i in range(k + 1):
                r[d - k + i] = (r[d - k + i] - c * m[i]) % P
    return r[:k]

def enc(v): return sum(d * P ** i for i, d in enumerate(v))

class GF:
    def __init__(self, k):
        self.k = k; self.q = P ** k; q = self.q
        for tail in itertools.product(range(P), repeat=k):
            m = list(tail) + [1]
            if m[0] == 0: continue
            x = [0, 1] + [0] * (k - 2) if k > 1 else [(-m[0]) % P]
            cur = [1] + [0] * (k - 1); seen = []
            for _ in range(q - 1):
                seen.append(enc(cur)); cur = polymulmod(cur, x, m, k)
            if len(set(seen)) == q - 1 and enc(cur) == 1:
                self.exp = seen; break
        else:
            raise RuntimeError("no primitive poly")
        self.log = {e: i for i, e in enumerate(self.exp)}
        dig = lambda a: [(a // P ** i) % P for i in range(k)]
        self.add = [[enc([(x + y) % P for x, y in zip(dig(a), dig(b))]) for b in range(q)] for a in range(q)]
    def mul(self, a, b):
        if a == 0 or b == 0: return 0
        return self.exp[(self.log[a] + self.log[b]) % (self.q - 1)]
    def chi(self, a):  # 0 -> 0, square -> 1, nonsquare -> -1
        if a == 0: return 0
        return 1 if self.log[a] % 2 == 0 else -1

def count(F, f):  # f: coeff list low->high over F_3, ints 0..2 embed as themselves
    n = 1; deg = len(f) - 1
    for x in range(F.q):
        acc = 0
        for c in reversed(f):
            acc = F.add[F.mul(acc, x)][c]
        n += 1 + F.chi(acc)
    # points at infinity of the smooth model
    if deg % 2 == 1: n += 1
    else: n += 1 + F.chi(f[-1])  # f[-1] in F_3 embeds
    return n  # NB n started at 1 for infinity slot replaced below

def count_pts(F, f):
    # affine + infinity
    deg = len(f) - 1; aff = 0
    for x in range(F.q):
        acc = 0
        for c in reversed(f):
            acc = F.add[F.mul(acc, x)][c]
        aff += 1 + F.chi(acc)
    inf = 1 if deg % 2 == 1 else 1 + F.chi(f[-1])
    return aff + inf

def pgcd(a, b):
    def trim(p):
        while p and p[-1] == 0: p.pop()
        return p
    a = trim(a[:]); b = trim(b[:])
    while b:
        inv = pow(b[-1], P - 2, P)
        while len(a) >= len(b) and a:
            c = a[-1] * inv % P; s = len(a) - len(b)
            for i, y in enumerate(b): a[s + i] = (a[s + i] - c * y) % P
            a = trim(a)
        a, b = b, a
    return a

def squarefree(f):
    d = [(i * f[i]) % P for i in range(1, len(f))]
    if not any(d): return False
    return len(pgcd(f, d)) <= 1

def adm(t, k):  # Waterhouse admissibility for E/F_{3^k}
    if t * t > 4 * 3 ** k: return False
    if t % 3: return True
    if k % 2 == 0: return abs(t) in (0, 3 ** (k // 2), 2 * 3 ** (k // 2)) 
    return t in (0, 3 ** ((k + 1) // 2), -3 ** ((k + 1) // 2))

def census(n, degs, fields, limit=None):
    q = P
    res = {"curves": 0, "hits": 0, "ts": {}, "null_hits": 0, "weil_viol": 0}
    ex = []
    for g in (range(0, 0),):
        pass
    for deg in degs:
        for lead in (1, 2):
            for tail in itertools.product(range(P), repeat=deg):
                f = list(tail) + [lead]
                if not squarefree(f): continue
                res["curves"] += 1
                N = [count_pts(F, f) for F in fields]
                gen = (deg - 1) // 2
                for k, Nk in enumerate(N, 1):
                    if abs(Nk - q ** k - 1) > 2 * gen * q ** (k / 2) + 1e-9: res["weil_viol"] += 1
                if N[0] == q + 1 and N[1] == q * q + 1:
                    s = [q ** k + 1 - N[k - 1] for k in range(1, n + 1)]
                    if n == 2:  # s1=0 (N1=q+1); s2 = 2t
                        s2 = q * q + 1 - N[1]
                        ok = s2 % 2 == 0 and adm(s2 // 2, 2) if False else False
                    # n=2 uses only N1; handled below
                if n == 3 and N[0] == q + 1 and N[1] == q * q + 1:
                    s3 = q ** 3 + 1 - N[2]
                    if s3 % 3 == 0 and adm(s3 // 3, 3):
                        res["hits"] += 1; t = s3 // 3; res["ts"][t] = res["ts"].get(t, 0) + 1
                        if len(ex) < 3: ex.append(f)
                if n == 2:
                    if N[0] == q + 1:
                        s2 = q * q + 1 - N[1]
                        # s1=0,s2=2t ; need c1=0 -> char poly x^4 - t x^2 + q^2
                        if s2 % 2 == 0 and adm(s2 // 2, 2):
                            res["hits"] += 1; t = s2 // 2; res["ts"][t] = res["ts"].get(t, 0) + 1
                            if len(ex) < 3: ex.append(f)
                    if N[0] == q + 2: res["null_hits"] += 1
                else:
                    if N[0] == q + 2 and N[1] == q * q + 1: res["null_hits"] += 1
    res["ts"] = {str(k): v for k, v in sorted(res["ts"].items())}
    res["examples_low_to_high"] = ex
    return res

def aa_check(F1, f_list):
    """A/A: independent recount of N1 by explicit y enumeration over F_3."""
    bad = 0
    for f in f_list:
        deg = len(f) - 1; c = 0
        for x in range(3):
            v = sum(co * x ** i for i, co in enumerate(f)) % 3
            c += sum(1 for y in range(3) if (y * y) % 3 == v)
        c += 1 if deg % 2 == 1 else (2 if f[-1] == 1 else 0)
        if c != count_pts(F1, f): bad += 1
    return bad

def main():
    smoke = "--smoke" in sys.argv
    rnd = random.Random(SEED)
    F1, F2 = GF(1), GF(2)
    F3 = None if smoke else GF(3)
    # control n=2 (genus 2, deg 5,6)
    ctrl = census(2, (5, 6) if not smoke else (5,), [F1, F2])
    sample = []
    for _ in range(30 if smoke else 300):
        deg = rnd.choice([5, 6, 7]); f = [rnd.randrange(3) for _ in range(deg)] + [rnd.choice([1, 2])]
        if squarefree(f): sample.append(f)
    aa_bad = aa_check(F1, sample)
    out = {"lead": "LEAD-TRF-weil-restriction-jacobian-class", "mode": "smoke" if smoke else "full", "seed": SEED,
           "q": 3, "control_n2_genus2": ctrl, "aa_recount_mismatches": aa_bad, "aa_sample": len(sample)}
    if smoke:
        out["verdict"] = "inconclusive"
    else:
        c3 = census(3, (7, 8), [F1, F2, F3])
        out["census_n3_genus3_hyperelliptic"] = c3
        if ctrl["hits"] == 0 or aa_bad or ctrl["weil_viol"] or c3["weil_viol"]:
            out["verdict"] = "inconclusive"
        elif c3["hits"] >= 1:
            out["verdict"] = "consistent_with_prediction"
        else:
            out["verdict"] = "falsified"
    print(json.dumps(out))
    return 0

if __name__ == "__main__":
    sys.exit(main())
