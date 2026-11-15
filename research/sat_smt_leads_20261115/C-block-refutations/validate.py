#!/usr/bin/env python3
"""Validation of the fixed encoding against an independent brute force.  Usage: validate.py [l ...]
Writes validation.json.  Own arithmetic below shares nothing with lib.py except the field polynomial."""
import sys, os, json, random, itertools
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import lib

lib.CMS_OPTS.update({"autodisablegauss": "0", "sls": "0"})
H = os.path.dirname(os.path.abspath(__file__))


# ---------------------------------------------------------------- independent arithmetic
class Own:
    def __init__(self, n, poly, B, l):
        self.n, self.poly, self.B, self.l = n, poly, B, l
        q = 1 << n
        # y^2 + xy = x^3 + x^2 + B: brute-force y for every x in V \ {0}
        self.pts = []
        for x in range(1, 1 << l):
            rhs = self.mul(self.mul(x, x), x) ^ self.mul(x, x) ^ B
            for y in range(q):
                if self.mul(y, y) ^ self.mul(x, y) == rhs:
                    self.pts.append((x, y))
        self.pts = sorted(set(self.pts))

    def mul(self, a, b):
        r = 0
        for i in range(self.n):
            if (b >> i) & 1:
                r ^= a << i
        for i in range(2 * self.n - 2, self.n - 1, -1):
            if (r >> i) & 1:
                r ^= self.poly << (i - self.n)
        return r

    def inv(self, a):
        r = 1
        for _ in range((1 << self.n) - 2 if self.n <= 0 else 0):
            r = self.mul(r, a)
        # a^(2^n - 2) by square and multiply
        e, base, r = (1 << self.n) - 2, a, 1
        while e:
            if e & 1:
                r = self.mul(r, base)
            base = self.mul(base, base)
            e >>= 1
        return r

    def add(self, P, Q):
        if P is None:
            return Q
        if Q is None:
            return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if y1 != y2 or x1 == 0:
                return None
            lam = x1 ^ self.mul(y1, self.inv(x1))
        else:
            lam = self.mul(y1 ^ y2, self.inv(x1 ^ x2))
        x3 = self.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ 1
        return (x3, self.mul(lam, x1 ^ x3) ^ x3 ^ y1)

    def decomps(self, R):
        """x1 values (first summand) of all decompositions R = P1 + P2 + P3 over signed base points;
        also the subset whose pair P2 + P3 has x != 0."""
        pair = {}
        for a in self.pts:
            for b in self.pts:
                s = self.add(a, b)
                pair.setdefault(s, True)
        allx, nzx = set(), set()
        for a in self.pts:
            na = (a[0], a[0] ^ a[1])
            rest = self.add(R, na)  # R - a
            if rest in pair:
                allx.add(a[0])
                if rest is not None and rest[0] != 0:
                    nzx.add(a[0])
        return allx, nzx


def decide(cell, R, enc="dual", fix=True, basis=None):
    S = lib.System(cell, R, basis, fix_u0=fix)
    sat, model, _ = lib.CMS(S.render(enc)).solve()
    ok_model = None
    if sat:
        ok_model = cell.lifts(S.x_of(model), R)
    return sat, ok_model, S


out = {"cells": []}
for l in [int(x) for x in sys.argv[1:]] or [3, 4]:
    seed = 2026111500 + l
    cell = lib.Cell(l, seed)
    own = Own(cell.n, cell.F.poly, cell.B, l)
    assert own.pts == sorted(set(cell.signed)), "factor base mismatch"
    non, dec = [], []
    tg = [cell.random_target() for _ in range(24)] + [cell.planted_target() for _ in range(8)]
    res = {"l": l, "n": cell.n, "B": cell.B, "targets": len(tg), "decomposable": 0, "free_agree": 0,
           "models_lifted": 0, "leaf_answers": 0, "dfs_agree": 0, "shuffle_agree": 0, "u0_true": 0}
    for ti, R in enumerate(tg):
        allx, nzx = own.decomps(R)
        truth = bool(allx)
        assert truth == cell.decomposable(R) == cell.brute(R)
        res["decomposable"] += truth
        u0 = cell.u0_decomposable(R)
        res["u0_true"] += u0
        for enc in ("dual", "plain"):
            sat, okm, S = decide(cell, R, enc)
            assert (sat or u0) == truth, (l, ti, enc, sat, u0, truth)
            if sat:
                assert okm
                res["models_lifted"] += 1
            res["free_agree"] += 1
        # per-x1 leaf answers, fixed dual encoding, one incremental solver
        S = lib.System(cell, R)
        rd = S.render("dual")
        inc = lib.CMS(rd)
        for x in range(1 << l):
            sat, model, _ = inc.solve([S.c[k] if (x >> k) & 1 else -S.c[k] for k in range(l)])
            assert sat == (x in nzx), (l, ti, x, sat)
            if sat:
                assert cell.lifts(S.x_of(model), R) and S.x_of(model)[0] == x
            res["leaf_answers"] += 1
        # recording DFS, budgets 0 and 3, canonical and shuffled numbering
        for bud in (0, 3):
            for shuf in (None, 5):
                r2, perm = (rd, None) if shuf is None else lib.shuffle_rendered(rd, shuf)
                cv = S.c if perm is None else [perm[v] for v in S.c]
                st, nodes = lib.rec_dfs(S, lib.CMS(r2, {"seed": "3"} if shuf else None), list(range(l)), bud, cv)
                assert st["sat"] == bool(nzx)
                if not st["sat"]:
                    root = nodes[-1]
                    assert root[0] == 0 and root[6] == st["conflicts"]
                    cov = sum(1 << (l - nd[0]) for nd in nodes if nd[2] == 0)
                    assert cov == 1 << l
                res["dfs_agree" if shuf is None else "shuffle_agree"] += 1
        # leaf classification is consistent with the enumeration
        for x in range(1 << l):
            c = lib.leaf_class(cell, R, x)
            if x in nzx:
                assert c == 2
    out["cells"].append(res)
    print(res, file=sys.stderr)

# ---------------------------------------------------------------- the u = 0 defect
rng = random.Random(20261115)
tw = {"old_false_sat": 0, "new_correct": 0, "instances": [], "genuine_u0": 0, "genuine_u0_correct": 0}
for l in (3, 4):
    n = lib.CELLS[l]
    found = gen = tries = 0
    extra = [1281] if l == 4 else []
    while (found < 6 or gen < 3) and tries < 4000:
        tries += 1
        B = extra.pop() if extra else rng.randrange(2, 1 << n)
        cell = lib.Cell(l, 0, B)
        F = cell.F
        sB = F.sqrt(B)
        prs = [(x2, F.mul(sB, F.inv(x2))) for x2 in range(1, 1 << l)]
        prs = [(a, b) for a, b in prs if 0 < b < (1 << l)]
        if not prs:
            continue
        own = Own(n, F.poly, B, l)
        for P1 in cell.base:
            R = cell.E.add(P1, cell.T)
            if not cell.ok_target(R):
                continue
            allx, nzx = own.decomps(R)
            truth = bool(allx)
            assert truth == cell.brute(R)
            old, oldok, _ = decide(cell, R, fix=False)
            new, newok, _ = decide(cell, R, fix=True)
            u0 = cell.u0_decomposable(R)
            assert (new or u0) == truth, (l, B, R)
            if new:
                assert newok
            if old and not truth:
                tw["old_false_sat"] += 1
                tw["new_correct"] += 1
                found += 1
                if len(tw["instances"]) < 8:
                    tw["instances"].append({"l": l, "n": n, "B": B, "R": list(R), "old": "SAT", "old_model_lifts": oldok,
                                            "new": "UNSAT", "brute": "non-decomposable"})
            if truth and u0:
                tw["genuine_u0"] += 1
                tw["genuine_u0_correct"] += 1
                gen += 1
    tw["tries_l%d" % l] = tries
assert tw["old_false_sat"] >= 6
out["u0_defect"] = tw
print(tw, file=sys.stderr)

# ---------------------------------------------------------------- conflict counter cross-check
# the conflict count is parsed from CMS's verbose line; check it against the conflict limit:
# a fresh solver with limit L must return "unknown" when L < reported count - slack and UNSAT when L >= count.
cell = lib.Cell(4, 2026111504)
non, _ = cell.targets(2, 0)
chk = {"leaves": 0, "min_limit_equals_reported": 0, "max_abs_diff": 0}
for R in non:
    S = lib.System(cell, R)
    rd = S.render("dual")
    for x in range(1, 1 << 4, 2):
        asm = [S.c[k] if (x >> k) & 1 else -S.c[k] for k in range(4)]
        sat, _, cf = lib.CMS(rd).solve(asm)
        assert sat is False
        lo, hi = 0, max(cf + 8, 8)
        while lo < hi:
            mid = (lo + hi) // 2
            s2, _, _ = lib.CMS(rd).solve(asm, mid)
            if s2 is False:
                hi = mid
            else:
                lo = mid + 1
        chk["leaves"] += 1
        chk["min_limit_equals_reported"] += lo == cf
        chk["max_abs_diff"] = max(chk["max_abs_diff"], abs(lo - cf))
out["conflict_counter_check"] = chk
print(chk, file=sys.stderr)
json.dump(out, open(os.path.join(H, "validation.json"), "w"), indent=1)
print("VALIDATION OK", file=sys.stderr)
