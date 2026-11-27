#!/usr/bin/env python3
"""
route1_tower_solver.py  --  Route 1 of the subexp ECDLP program.

A T-independent (triangular / recursive-tree) solver for the PKM m=2 *tower
membership system*, benchmarked against the measured F4 wall
(127.4 M / 2.003 G / 79.75 G multiply-adds per solver call at T = 5 / 6 / 7).

THE SYSTEM (matches `pkm_semaev.rs::build_system_fixed`, mode=full, m=2,
x_R specialised to the target abscissa c):

    variables   y_{i,0}, ..., y_{i,t-1}   for i = 1,2          -> 2t variables
    equations   y_{i,j}^2 - y_{i,j+1} = 0        (j < t-1)     -> t-1 per summand
                y_{i,t-1}^2 - 1       = 0                      -> 1   per summand
                S_3(g y_{1,0}, g y_{2,0}, c) = 0                -> 1
                                                               -> 2t+1 equations

    S_3(x1,x2,x3) = (x1-x2)^2 x3^2
                    - 2[(x1+x2)(x1 x2 + a) + 2b] x3
                    + (x1 x2 - a)^2 - 4b(x1+x2)

So 2t variables / 2t+1 equations: 10/11, 12/13, 14/15 at t = 5,6,7 -- exactly
the shapes in the F4 panel.

TOWER IS EXACT FOR THE KUMMER ARM.  For `prime-kummer` the step is y -> y^2,
so the tower's zero set is g * mu_{2^t} exactly; no proxy is involved.
For `prime-isogeny` the step is the x-map of a Velu 2-isogeny, which the
harness also encodes as one quadratic per level (`SquareRule`,
`pkm_semaev.rs::tower_rule_roots`: y_{j+1}^2 - xi_j y_{j+1} + tau_j = 0).
The solver below uses ONLY (i) the branching factor 2 of a level, and (ii) an
explicit listing of V, so every cost number reported here is
coefficient-independent: it is the same for the isogeny arm.  That is the
"structural proxy" claim, made explicit.

WHY THIS IS T-INDEPENDENT.  The tower equations are triangular and each level
is a quadratic, so a value at level j fixes at most 2 values at level j+1: a
depth-t binary tree with 2^t leaves and *no Macaulay matrix*.  And because S_3
is *quadratic in x2*, the second summand is never searched: once x1 is fixed,
the equation in x2 is a scalar quadratic solved in closed form.  So the system
costs 2^t leaves, not (2^t)^2 = 2^{2t}.

Two ways to reach the 2^t leaves are measured, because the difference is the
whole cost story:
  * `sqrt_chain`  -- the literal back-substitution: y_{t-1} in {+1,-1}, and at
                     each level take both square roots.  2^t square roots.
  * `generator`   -- the same leaf set walked as the cyclic group mu_{2^t}:
                     2^t field multiplies by a fixed primitive 2^t-th root.

Usage:
    python3 route1_tower_solver.py            # full benchmark + JSON
    python3 route1_tower_solver.py --quick    # t = 5,6 only
"""

import json
import random
import sys
import time

# --------------------------------------------------------------------------
# counted F_p arithmetic.  Ledger.mul counts field multiplications
# (squarings included, each squaring = 1); Ledger.add counts field additions.
# mul+add is the analogue of F4's "multiply-add" unit.
# --------------------------------------------------------------------------


class Ledger:
    __slots__ = ("mul", "add", "inv", "sqrt", "pow")

    def __init__(self):
        self.mul = self.add = self.inv = self.sqrt = self.pow = 0

    def reset(self):
        self.mul = self.add = self.inv = self.sqrt = self.pow = 0

    def ma(self):
        return self.mul + self.add

    def copy(self):
        l = Ledger()
        l.mul, l.add, l.inv, l.sqrt, l.pow = (
            self.mul, self.add, self.inv, self.sqrt, self.pow)
        return l

    def scaled(self, k):
        l = Ledger()
        l.mul, l.add = int(self.mul * k), int(self.add * k)
        return l


class Ctx:
    """Field + curve, carrying the op ledger."""

    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b
        self.L = Ledger()

    def mul(self, x, y):
        self.L.mul += 1
        return (x * y) % self.p

    def sqr(self, x):
        self.L.mul += 1
        return (x * x) % self.p

    def add(self, x, y):
        self.L.add += 1
        return (x + y) % self.p

    def sub(self, x, y):
        self.L.add += 1
        return (x - y) % self.p

    def neg(self, x):
        self.L.add += 1
        return (-x) % self.p

    def dbl(self, x):
        self.L.add += 1
        return (x + x) % self.p

    def pow(self, x, e):
        """x^e mod p with exact op accounting (squarings + multiplies)."""
        self.L.pow += 1
        if e == 0:
            return 1
        self.L.mul += e.bit_length() - 1
        self.L.mul += bin(e).count("1") - 1
        return pow(x, e, self.p)

    def inv(self, x):
        self.L.inv += 1
        return self.pow(x, self.p - 2)

    # -- S_3, the m=2 Semaev polynomial ------------------------------------
    def s3(self, x1, x2, x3):
        add, sub, mul, sqr = self.add, self.sub, self.mul, self.sqr
        a, b = self.a, self.b
        d = sub(x1, x2)
        t = mul(sqr(d), sqr(x3))
        s = add(x1, x2)
        pr = mul(x1, x2)
        v = mul(s, add(pr, a))
        v = self.dbl(add(v, self.dbl(b)))
        t = sub(t, mul(v, x3))
        t = add(t, sqr(sub(pr, a)))
        t = sub(t, mul(b, self.dbl(self.dbl(s))))
        return t

    # -- the quadratic in x2 at fixed x1 = X, x3 = Y -----------------------
    def quad_in_x2(self, X, Y):
        """
        S_3(X, x2, Y) = A x2^2 + B x2 + C, in closed form.  Derived once
        symbolically, O(1) and independent of t:
            A = (X - Y)^2
            B = -2 X Y^2 - 2 Y (X^2 + a) - 2 X a - 4 b
            C = X^2 Y^2 - 2 X a Y - 4 b Y + a^2 - 4 b X
        Verified against `s3` on random points in `self_test`.
        """
        add, sub, mul, sqr = self.add, self.sub, self.mul, self.sqr
        a, b = self.a, self.b
        X2 = sqr(X)
        Y2 = sqr(Y)
        XY = mul(X, Y)
        A = sqr(sub(X, Y))
        B = sub(sub(sub(self.neg(mul(self.dbl(XY), Y)),
                        self.dbl(mul(Y, add(X2, a)))),
                    self.dbl(mul(X, a))),
                self.dbl(self.dbl(b)))
        C = sub(sub(sub(add(sqr(XY), sqr(a)),
                        self.dbl(mul(mul(X, a), Y))),
                    self.dbl(self.dbl(mul(b, Y)))),
                self.dbl(self.dbl(mul(b, X))))
        return A, B, C

    # -- square root mod p (Tonelli-Shanks, counted) -----------------------
    def sqrt(self, n):
        p = self.p
        n %= p
        if n == 0:
            return 0
        if self.pow(n, (p - 1) // 2) != 1:      # Legendre / Euler test
            return None
        self.L.sqrt += 1
        if p % 4 == 3:
            return self.pow(n, (p + 1) // 4)
        q, s = p - 1, 0
        while q % 2 == 0:
            q //= 2
            s += 1
        z = 2
        while self.pow(z, (p - 1) // 2) == 1:
            z += 1
        m, c = s, self.pow(z, q)
        t, r = self.pow(n, q), self.pow(n, (q + 1) // 2)
        while t != 1:
            i, t2 = 0, t
            while t2 != 1:
                t2 = self.sqr(t2)
                i += 1
            bq = self.pow(c, 1 << (m - i - 1))
            m, c = i, self.sqr(bq)
            t = self.mul(t, c)
            r = self.mul(r, bq)
        return r


# --------------------------------------------------------------------------
# the factor base V = g * mu_{2^t}
# --------------------------------------------------------------------------

def primitive_2t_root(ctx, t):
    p = ctx.p
    if (p - 1) % (1 << t) != 0:
        return None
    e = (p - 1) >> t
    h = 2
    while h < 1000:
        z = pow(h, e, p)
        if pow(z, 1 << (t - 1), p) != 1:
            return z
        h += 1
    return None


def leaves_generator(ctx, t, g=1):
    """V = g*mu_{2^t} walked as a cyclic group: 2^t multiplies by zeta."""
    zeta = primitive_2t_root(ctx, t)
    if zeta is None:
        raise SystemExit(f"2^{t} does not divide p-1 = {ctx.p - 1}")
    out, z = [], g % ctx.p
    for _ in range(1 << t):
        out.append(z)
        z = ctx.mul(z, zeta)
    return out


def leaves_sqrt_chain(ctx, t, g=1):
    """
    V walked by literal tower back-substitution: y_{t-1} in {+1,-1}, then at
    each of the t-1 lower levels take both square roots.  2^t square roots.
    """
    level = [1 % ctx.p, (ctx.p - 1) % ctx.p]
    for _ in range(t - 1):
        nxt = []
        for y in level:
            r = ctx.sqrt(y)
            if r is None:
                raise AssertionError("tower level is not a square")
            nxt.append(r)
            nxt.append(ctx.neg(r))
        level = nxt
    return [ctx.mul(g, y) if g % ctx.p != 1 else y for y in level]


# --------------------------------------------------------------------------
# the solver
# --------------------------------------------------------------------------

def leaf_roots(ctx, X, Y, Vset, test):
    """
    One leaf: x1 = X fixed, solve S_3(X, x2, Y) = 0 for x2, keep roots in V.
    """
    A, B, C = ctx.quad_in_x2(X, Y)
    out = []
    if A == 0:                                    # X == Y: S_3 is linear in x2
        if B != 0:
            v = ctx.neg(ctx.mul(C, ctx.inv(B)))
            if test(v):
                out.append(v)
        return out
    disc = ctx.sub(ctx.sqr(B), ctx.mul(ctx.dbl(ctx.dbl(A)), C))
    s = ctx.sqrt(disc)
    if s is None:
        return out
    inv2A = ctx.inv(ctx.dbl(A))
    for w in (ctx.add(ctx.neg(B), s), ctx.sub(ctx.neg(B), s)):
        v = ctx.mul(w, inv2A)
        if test(v):
            out.append(v)
    return out


def solve_tree(ctx, V, c, t, early_exit=False, membership="hash"):
    """
    The T-independent solver: walk the 2^t leaves of the first summand's tower,
    solve the closed-form quadratic for the second.  Returns (solutions as
    (x1, x2) pairs, leaves visited).

    membership="hash"  -> O(1) set lookup (the base is listed anyway)
    membership="chain" -> no table: fresh t-step squaring chain per candidate
    """
    Vset = set(V)
    if membership == "hash":
        test = lambda x: x in Vset
    else:
        def test(x):
            y = x
            for _ in range(t):
                y = ctx.sqr(y)
            return y == 1
    sols, leaves = [], 0
    for x1 in V:
        leaves += 1
        for x2 in leaf_roots(ctx, x1, c, Vset, test):
            sols.append((x1, x2))
            if early_exit:
                return sols, leaves
    return sols, leaves


def solve_naive(ctx, V, c):
    """Reference grid over V x V: Theta(2^{2t}).  The exactness oracle."""
    return [(u, v) for u in V for v in V if ctx.s3(u, v, c) == 0]


def batch_inv(ctx, xs):
    """Montgomery batch inversion: one inversion + 3k muls for k values."""
    if not xs:
        return []
    pre = [xs[0]]
    for x in xs[1:]:
        pre.append(ctx.mul(pre[-1], x))
    acc = ctx.inv(pre[-1])
    out = [0] * len(xs)
    for i in range(len(xs) - 1, 0, -1):
        out[i] = ctx.mul(acc, pre[i - 1])
        acc = ctx.mul(acc, xs[i])
    out[0] = acc
    return out


def solve_tree_batched(ctx, V, c, t, membership="hash"):
    """
    Full scan with the per-leaf denominators inverted in one batch.  This is the
    refutation cost: every leaf is visited, no early exit.
    """
    Vset = set(V)
    if membership == "hash":
        test = lambda x: x in Vset
    else:
        def test(x):
            y = x
            for _ in range(t):
                y = ctx.sqr(y)
            return y == 1
    pre = []
    for x1 in V:
        A, B, C = ctx.quad_in_x2(x1, c)
        disc = ctx.sub(ctx.sqr(B), ctx.mul(ctx.dbl(ctx.dbl(A)), C))
        pre.append((x1, B, disc, ctx.dbl(A)))
    invs = batch_inv(ctx, [q[3] for q in pre])
    sols = []
    for (x1, B, disc, _), i2A in zip(pre, invs):
        s = ctx.sqrt(disc)
        if s is None:
            continue
        for w in (ctx.add(ctx.neg(B), s), ctx.sub(ctx.neg(B), s)):
            v = ctx.mul(w, i2A)
            if test(v):
                sols.append((x1, v))
    return sols, len(V)


# --------------------------------------------------------------------------
# self test
# --------------------------------------------------------------------------

def self_test(ctx, t=4):
    p = ctx.p
    rnd = random.Random(12345)
    for _ in range(300):                       # closed form == direct S_3
        X, Y, v = (rnd.randrange(p) for _ in range(3))
        A, B, C = ctx.quad_in_x2(X, Y)
        lhs = ctx.add(ctx.add(ctx.mul(A, ctx.sqr(v)), ctx.mul(B, v)), C)
        assert lhs == ctx.s3(X, v, Y), (X, Y, v)
    Vg = leaves_generator(ctx, t)
    Vs = leaves_sqrt_chain(ctx, t)
    assert sorted(Vg) == sorted(Vs), "generator walk != square-root walk"
    for c in [rnd.randrange(p) for _ in range(10)]:
        got = sorted(solve_tree(ctx, Vg, c, t)[0])
        want = sorted(solve_naive(ctx, Vg, c))
        assert got == want, (c, got, want)
        got = sorted(solve_tree(ctx, Vg, c, t, membership="chain")[0])
        assert got == want
        got = sorted(solve_tree_batched(ctx, Vg, c, t)[0])
        assert got == want
        got = sorted(solve_tree_batched(ctx, Vg, c, t, membership="chain")[0])
        assert got == want
    print(f"  self-test ok (t={t}): closed form == S_3; generator walk == "
          f"sqrt-chain walk; tree == naive grid (plain/batched x hash/chain)")


# --------------------------------------------------------------------------
# relation targets c that really do have a (x1,x2) in V x V
# --------------------------------------------------------------------------

def relation_targets(ctx, V, k, rnd):
    out = []
    n = len(V)
    for _ in range(20000):
        if len(out) >= k:
            break
        i, j = rnd.randrange(n), rnd.randrange(n)
        if i == j:
            continue
        x1, x2 = V[i], V[j]
        A = ctx.sqr(ctx.sub(x1, x2))
        Th = ctx.add(ctx.mul(ctx.add(x1, x2), ctx.add(ctx.mul(x1, x2), ctx.a)),
                     ctx.dbl(ctx.b))
        Ps = ctx.sub(ctx.sqr(ctx.sub(ctx.mul(x1, x2), ctx.a)),
                     ctx.mul(ctx.dbl(ctx.dbl(ctx.b)), ctx.add(x1, x2)))
        s = ctx.sqrt(ctx.sub(ctx.sqr(Th), ctx.mul(A, Ps)))
        if s is None:
            continue
        inv = ctx.inv(A)
        for sg in (1, -1):
            out.append(ctx.mul(ctx.add(Th, s if sg > 0 else ctx.neg(s)), inv))
            if len(out) >= k:
                break
    return out


# --------------------------------------------------------------------------
# benchmark
# --------------------------------------------------------------------------

def benchmark(p, a, b, t_list, n_random=200, n_hit=25, n_full=40, seed=20261185):
    rnd = random.Random(seed)
    rows = []
    for t in t_list:
        ctx = Ctx(p, a, b)
        n_v = 1 << t

        # cost of the leaf-set build, both ways
        ctx.L.reset()
        V = leaves_generator(ctx, t)
        build_gen = ctx.L.copy()
        ctx.L.reset()
        Vs = leaves_sqrt_chain(ctx, t)
        build_sqrt = ctx.L.copy()
        assert sorted(V) == sorted(Vs)
        Vset = set(V)

        # cost of one closed-form quadratic (the whole "symbolic setup")
        ctx.L.reset()
        ctx.quad_in_x2(1234 % p, 5678 % p)
        setup = ctx.L.copy()

        # decision on random (mostly refuted) targets, early exit
        ctx.L.reset()
        t0 = time.perf_counter()
        verdicts = []
        for _ in range(n_random):
            c = rnd.randrange(p)
            sols, leaves = solve_tree(ctx, V, c, t, early_exit=True)
            verdicts.append((len(sols) > 0, leaves))
        wall_rand = (time.perf_counter() - t0) / n_random
        ops_rand = ctx.L.copy()
        hits = sum(1 for h, _ in verdicts if h)
        leaves_rand = sum(l for _, l in verdicts) / n_random

        # decision on targets that DO have a solution (best case: early exit)
        real_c = relation_targets(ctx, V, n_hit, rnd)
        ctx.L.reset()
        t0 = time.perf_counter()
        for c in real_c:
            solve_tree(ctx, V, c, t, early_exit=True)
        wall_hit = (time.perf_counter() - t0) / max(1, len(real_c))
        ops_hit = ctx.L.copy()

        # full scan = the refutation cost; averaged over several targets
        tgts = [rnd.randrange(p) for _ in range(n_full)]
        ctx.L.reset()
        t0 = time.perf_counter()
        for c in tgts:
            solve_tree_batched(ctx, V, c, t)
        wall_full = (time.perf_counter() - t0) / n_full
        ops_batch = ctx.L.copy().scaled(1 / n_full)
        ctx.L.reset()
        for c in tgts:
            solve_tree(ctx, V, c, t, early_exit=False)
        ops_plain = ctx.L.copy().scaled(1 / n_full)
        ctx.L.reset()
        for c in tgts:
            solve_tree_batched(ctx, V, c, t, membership="chain")
        ops_chain = ctx.L.copy().scaled(1 / n_full)

        # reference grid cost, measured on one column and scaled exactly
        ctx.L.reset()
        solve_naive(ctx, V[:1], rnd.randrange(p))
        per = ctx.L.copy()
        ops_naive = per.scaled(n_v * n_v)
        if t <= 6:
            ctx.L.reset()
            solve_naive(ctx, V, rnd.randrange(p))
            real_naive = ctx.L.copy()
            assert abs(real_naive.ma() / max(1, ops_naive.ma()) - 1) < 0.02

        row = dict(
            t=t, n_vars=2 * t, n_eqs=2 * t + 1, base_size=n_v,
            base_points=2 * n_v,
            coverage=n_v * n_v / p,
            hit_fraction=hits / n_random,
            tree_leaves_avg=leaves_rand,
            setup_mul=setup.mul, setup_add=setup.add,
            build_gen_mul=build_gen.mul, build_gen_add=build_gen.add,
            build_sqrt_mul=build_sqrt.mul, build_sqrt_add=build_sqrt.add,
            build_sqrt_sqrt_calls=build_sqrt.sqrt,
            ops_tree_random_mul=ops_rand.mul / n_random,
            ops_tree_random_ma=ops_rand.ma() / n_random,
            ops_tree_hit_mul=ops_hit.mul / max(1, len(real_c)),
            ops_full_batch_mul=ops_batch.mul, ops_full_batch_ma=ops_batch.ma(),
            ops_full_plain_mul=ops_plain.mul, ops_full_plain_ma=ops_plain.ma(),
            ops_full_chain_mul=ops_chain.mul, ops_full_chain_ma=ops_chain.ma(),
            ops_per_leaf_mul=ops_batch.mul / n_v,
            ops_naive_ma=ops_naive.ma(),
            wall_tree_random_s=wall_rand, wall_tree_hit_s=wall_hit,
            wall_full_s=wall_full,
            solutions_found=len(sols),
        )
        rows.append(row)
        print(f"  t={t}: vars={2*t} eqs={2*t+1} |V|={n_v} "
              f"| full {row['ops_full_batch_mul']:.0f} mul "
              f"({row['ops_per_leaf_mul']:.0f}/leaf, {wall_full*1e6:.0f} us) "
              f"| avg decision {row['ops_tree_random_mul']:.0f} mul "
              f"(hit {hits/n_random*100:.0f}%) "
              f"| naive {row['ops_naive_ma']:.3g} ma")
    return rows


# --------------------------------------------------------------------------
# relation-join cost: the full pair table
# --------------------------------------------------------------------------

def join_cost(p, a, b, t, seed=1):
    """
    Enumerate every unordered abscissa pair (x1,x2) in V x V, compute both
    x-sums, and count the distinct group elements covered, with exact op cost.
    This is the *entire* m=2 relation set.
    """
    ctx = Ctx(p, a, b)
    ctx.L.reset()
    V = leaves_generator(ctx, t)
    build = ctx.L.copy()
    n = len(V)
    seen, pairs = set(), 0
    for i in range(n):
        x1 = V[i]
        for j in range(i + 1, n):
            x2 = V[j]
            pairs += 1
            A = ctx.sqr(ctx.sub(x1, x2))
            Th = ctx.add(ctx.mul(ctx.add(x1, x2), ctx.add(ctx.mul(x1, x2), a)),
                         ctx.dbl(b))
            Ps = ctx.sub(ctx.sqr(ctx.sub(ctx.mul(x1, x2), a)),
                         ctx.mul(ctx.dbl(ctx.dbl(b)), ctx.add(x1, x2)))
            s = ctx.sqrt(ctx.sub(ctx.sqr(Th), ctx.mul(A, Ps)))
            if s is None:
                continue
            inv = ctx.inv(A)
            for sg in (1, -1):
                seen.add(ctx.mul(ctx.add(Th, s if sg > 0 else ctx.neg(s)), inv))
    L = ctx.L.copy()
    return dict(
        t=t, base_points=2 * n, abscissa_pairs=pairs,
        distinct_x_sums=len(seen), coverage_of_group=len(seen) / p,
        build_mul=build.mul,
        ops_total_mul=L.mul, ops_total_ma=L.ma(),
        ops_per_pair_ma=L.ma() / max(1, pairs),
        pair_count_theory=(2 * n) ** 2 / 2,
    )


# --------------------------------------------------------------------------

F4_BASELINE = {5: 127.4e6, 6: 2002.9e6, 7: 79748e6}   # multiply-adds / solver call


def main():
    quick = "--quick" in sys.argv
    p, a, b = 12289, 1, 1
    print("route1: T-independent triangular/tree solver, PKM m=2 tower system")
    print(f"  F_p: p = {p} ({p.bit_length()} bits, "
          f"v2(p-1) = {((p-1) & -(p-1)).bit_length()-1}), "
          f"E: y^2 = x^3 + {a}x + {b}")
    ctx = Ctx(p, a, b)
    self_test(ctx, t=4)

    t_list = [5, 6, 7, 8] if not quick else [5, 6]
    rows = benchmark(p, a, b, t_list)

    print("\n  relation join (full pair table, m=2):")
    joins = [join_cost(p, a, b, t) for t in t_list]
    for j in joins:
        print(f"    t={j['t']}: |V|={j['base_points']} pts  pairs={j['abscissa_pairs']}  "
              f"distinct sums={j['distinct_x_sums']} "
              f"({j['coverage_of_group']*100:.0f}% of F_p)  "
              f"{j['ops_total_ma']:.3g} ma ({j['ops_per_pair_ma']:.0f}/pair)")

    print("\n  vs the measured F4 wall (per solver call):")
    print(f"    {'T':>2} {'tree full mul':>14} {'F4 mul-adds':>13} "
          f"{'F4/tree':>10} {'ratio vs prev T':>16}")
    prev_tree = None
    for r in rows:
        f4 = F4_BASELINE.get(r["t"])
        if prev_tree is None:
            step = "-"
        elif f4 and F4_BASELINE.get(r["t"] - 1):
            step = (f"{r['ops_full_batch_mul']/prev_tree:.2f}x "
                    f"(F4 {f4/F4_BASELINE[r['t']-1]:.2f}x)")
        else:
            step = f"{r['ops_full_batch_mul']/prev_tree:.2f}x"
        if f4:
            print(f"    {r['t']:>2} {r['ops_full_batch_mul']:>14.0f} "
                  f"{f4:>13.4g} {f4/r['ops_full_batch_mul']:>9.3g}x "
                  f"{step:>16}")
        else:
            print(f"    {r['t']:>2} {r['ops_full_batch_mul']:>14.0f} "
                  f"{'(outside panel)':>13} {'-':>10} {step:>16}")
        prev_tree = r["ops_full_batch_mul"]

    # ---- scaling summary: per-system exponent in p ------------------------
    print("\n  scaling summary (per-leaf constant, and the p-exponent it implies)")
    c_leaf = rows[-1]["ops_per_leaf_mul"]
    print(f"    per-leaf constant: {', '.join(f'T={r['t']}: {r['ops_per_leaf_mul']:.0f}' for r in rows)} mul")
    print(f"    per system  = c*2^T  with c = {c_leaf:.0f} field mults, T-independent")
    print(f"    at T = 0.5 log2 p:  per system = {c_leaf:.0f} * p^0.5")
    print(f"    relation collection = |V| decisions = p^0.5 * ({c_leaf:.0f} p^0.5)"
          f" = {c_leaf:.0f} p^1.0")
    f4_exp = 2.66
    print(f"    for comparison: F4 per call = p^{f4_exp} (measured fit), "
          f"rho = p^0.5")

    # ---- the only p-dependence: log p per leaf (second modulus) -----------
    print("\n  second modulus (per-leaf constant should scale as log p, not p)")
    for pp, ts in ((40961, [5, 6, 7]), (65537, [5, 6, 7, 8])):
        cc = Ctx(pp, 1, 1)
        # sanity: modulus prime and 2^max(ts) | p-1
        assert all(pp % d for d in range(2, int(pp ** 0.5) + 1)), f"{pp} not prime"
        assert (pp - 1) % (1 << max(ts)) == 0
        sub = benchmark(pp, 1, 1, ts, n_random=40, n_hit=8, n_full=8, seed=7)
        print(f"    p={pp} ({pp.bit_length()} bits): per-leaf mul = "
              f"{', '.join(f'T={r['t']}: {r['ops_per_leaf_mul']:.0f}' for r in sub)}")

    out = dict(p=p, a=a, b=b, rows=rows, joins=joins,
               f4_baseline=F4_BASELINE, per_leaf_mul=c_leaf)
    with open("route1_results.json", "w") as f:
        json.dump(out, f, indent=1)
    print("\n  wrote route1_results.json")


if __name__ == "__main__":
    main()
