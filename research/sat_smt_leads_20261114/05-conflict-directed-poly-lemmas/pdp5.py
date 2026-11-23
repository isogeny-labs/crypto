"""Self-contained m=3 binary PDP generator, ANF Macaulay lemma generator and
Glucose outer loop for idea 5 (conflict-directed polynomial lemmas). Pilot."""
import random, itertools, time
from pysat.solvers import Glucose4

IRRED = {11: (1 << 11) | 0b101, 13: (1 << 13) | 0b11011, 17: (1 << 17) | 0b1001,
         19: (1 << 19) | 0b100111}


class Field:
    def __init__(self, n):
        self.n = n
        self.mod = IRRED[n]
        self.mask = (1 << n) - 1

    def mul(self, a, b):
        r = 0
        n, mod = self.n, self.mod
        while b:
            if b & 1:
                r ^= a
            b >>= 1
            a <<= 1
            if a >> n:
                a ^= mod
        return r

    def sq(self, a):
        return self.mul(a, a)

    def pow(self, a, e):
        r = 1
        while e:
            if e & 1:
                r = self.mul(r, a)
            a = self.mul(a, a)
            e >>= 1
        return r

    def inv(self, a):
        assert a
        return self.pow(a, (1 << self.n) - 2)

    def trace(self, a):
        t = 0
        for _ in range(self.n):
            t ^= a
            a = self.sq(a)
        assert t in (0, 1)
        return t

    def halftrace(self, a):  # n odd: z with z^2+z = a when Tr(a)=0
        h = 0
        x = a
        for i in range(self.n):
            if i % 2 == 0:
                h ^= x
            x = self.sq(x)
        return h

    def irreducible_check(self):
        # x^(2^n) == x and gcd conditions (n prime: enough that x^(2^n)=x, x^2 != x)
        x = 2
        y = x
        for _ in range(self.n):
            y = self.sq(y)
        return y == x


class Curve:
    """y^2 + xy = x^3 + a x^2 + b over F_{2^n}; points (x,y) or None."""

    def __init__(self, F, a, b):
        self.F, self.a, self.b = F, a, b

    def lift(self, x):
        F = self.F
        if x == 0:
            return None
        rhs = x ^ self.a ^ F.mul(self.b, F.inv(F.sq(x)))
        if F.trace(rhs):
            return None
        z = F.halftrace(rhs)
        assert F.sq(z) ^ z == rhs
        return (x, F.mul(z, x))

    def on(self, P):
        F = self.F
        x, y = P
        return F.sq(y) ^ F.mul(x, y) == F.mul(F.sq(x), x) ^ F.mul(self.a, F.sq(x)) ^ self.b

    def neg(self, P):
        return None if P is None else (P[0], P[0] ^ P[1])

    def add(self, P, Q):
        F = self.F
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if y1 != y2 or x1 == 0:
                return None
            lam = x1 ^ F.mul(y1, F.inv(x1))
            x3 = F.sq(lam) ^ lam ^ self.a
            y3 = F.sq(x1) ^ F.mul(lam ^ 1, x3)
            return (x3, y3)
        lam = F.mul(y1 ^ y2, F.inv(x1 ^ x2))
        x3 = F.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a
        y3 = F.mul(lam, x1 ^ x3) ^ x3 ^ y1
        return (x3, y3)


def S3(F, b, x1, x2, x3):
    e = F.mul(x1, x2) ^ F.mul(x1, x3) ^ F.mul(x2, x3)
    return F.sq(e) ^ F.mul(F.mul(x1, x2), x3) ^ b


# ---------------------------------------------------------------- instances
def make_instance(n, l, idx, planted):
    seed = 100000 * n + 1000 * l + idx
    rng = random.Random(seed)
    F = Field(n)
    while True:
        b = rng.randrange(1, 1 << n)
        E = Curve(F, 1, b)
        # random independent basis of V
        basis = []
        red = []
        while len(basis) < l:
            v = rng.randrange(1, 1 << n)
            r = v
            for q in red:
                r = min(r, r ^ q)
            if r:
                red.append(r)
                red.sort(reverse=True)
                basis.append(v)
        span = [0]
        for v in basis:
            span += [x ^ v for x in span]
        pts = [E.lift(x) for x in span]
        pts = [P for P in pts if P is not None]
        if len(pts) < 3:
            continue
        if planted:
            ok = False
            for _ in range(100):
                Ps = [rng.choice(pts) for _ in range(3)]
                Ps = [P if rng.random() < .5 else E.neg(P) for P in Ps]
                R = E.add(E.add(Ps[0], Ps[1]), Ps[2])
                if R is not None and R[0] != 0 and len({P[0] for P in Ps}) == 3:
                    ok = True
                    break
            if not ok:
                continue
            sol = [P[0] for P in Ps]
        else:
            while True:
                R = E.lift(rng.randrange(1, 1 << n))
                if R is not None:
                    break
            sol = None
        return dict(n=n, l=l, idx=idx, seed=seed, planted=planted, F=F, E=E, b=b,
                    basis=basis, span=span, R=R, XR=R[0], sol=sol)


def brute_force(inst):
    """All (x1,x2,x3) in V^3 (unordered x1,x2 counted twice) with exists t:
    S3(x1,x2,t)=0 and S3(x3,XR,t)=0. Returns count of (i,j,k) index triples."""
    F, b, span, XR = inst['F'], inst['b'], inst['span'], inst['XR']
    n = F.n
    T3 = {}
    for k, x3 in enumerate(span):
        for t in range(1 << n):
            if S3(F, b, x3, XR, t) == 0:
                T3.setdefault(t, []).append(k)
    cnt = 0
    for i, x1 in enumerate(span):
        for j, x2 in enumerate(span):
            p = F.mul(x1, x2)
            s2 = F.sq(x1 ^ x2)
            c0 = F.sq(p) ^ b
            for t in T3:
                if F.mul(s2, F.sq(t)) ^ F.mul(p, t) ^ c0 == 0:
                    cnt += len(T3[t])
    return cnt


# ---------------------------------------------------------------- ANF system
class FPoly:
    """field-valued multilinear polynomial: dict monomial(tuple) -> coef."""

    def __init__(self, F, d=None):
        self.F = F
        self.d = d or {}

    def __add__(self, o):
        d = dict(self.d)
        for m, c in o.d.items():
            c2 = d.get(m, 0) ^ c
            if c2:
                d[m] = c2
            else:
                d.pop(m, None)
        return FPoly(self.F, d)

    def __mul__(self, o):
        F = self.F
        d = {}
        for m1, c1 in self.d.items():
            for m2, c2 in o.d.items():
                m = tuple(sorted(set(m1) | set(m2)))
                c = d.get(m, 0) ^ F.mul(c1, c2)
                if c:
                    d[m] = c
                else:
                    d.pop(m, None)
        return FPoly(F, d)

    def sq(self):
        return FPoly(self.F, {m: self.F.sq(c) for m, c in self.d.items()})

    def bits(self):
        out = []
        for k in range(self.F.n):
            out.append(frozenset(m for m, c in self.d.items() if (c >> k) & 1))
        return out


def build_system(inst):
    F, l, n = inst['F'], inst['l'], inst['n']
    A = list(range(0, l)); B = list(range(l, 2 * l)); C = list(range(2 * l, 3 * l))
    S = list(range(3 * l, 4 * l)); P = list(range(4 * l, 4 * l + n))
    T = list(range(4 * l + n, 4 * l + 2 * n))
    N = 4 * l + 2 * n
    V = inst['basis']
    lin = lambda vs, cs: FPoly(F, {(v,): c for v, c in zip(vs, cs)})
    const = lambda c: FPoly(F, {(): c} if c else {})
    x1, x2, x3, s = lin(A, V), lin(B, V), lin(C, V), lin(S, V)
    p = lin(P, [1 << k for k in range(n)]); t = lin(T, [1 << k for k in range(n)])
    XR, b = inst['XR'], inst['b']
    E0 = p + x1 * x2
    E1 = p.sq() + s.sq() * t.sq() + p * t + const(b)
    XR2 = const(F.sq(XR))
    E2 = x3.sq() * XR2 + x3.sq() * t.sq() + XR2 * t.sq() + const(XR) * x3 * t + const(b)
    eqs = []
    for E in (E0, E1, E2):
        eqs += [e for e in E.bits() if e]
    for i in range(l):
        eqs.append(frozenset([(S[i],), (A[i],), (B[i],)]))
    monos = set()
    for e in eqs:
        for m in e:
            if len(m) == 2:
                monos.add(m)
    return dict(N=N, eqs=eqs, monos=monos, groups=dict(A=A, B=B, C=C, S=S, P=P, T=T))


def eval_anf(e, assign):
    v = 0
    for m in e:
        v ^= all(assign[x] for x in m)
    return v


def solution_assignment(inst, sysd, xs):
    """assignment of ANF variables for x1,x2,x3 given (field elts in V)."""
    F, l, n = inst['F'], inst['l'], inst['n']
    span, basis = inst['span'], inst['basis']

    def coords(x):
        for c in range(1 << l):
            v = 0
            for i in range(l):
                if (c >> i) & 1:
                    v ^= basis[i]
            if v == x:
                return [(c >> i) & 1 for i in range(l)]
        raise ValueError
    a, b_, c = coords(xs[0]), coords(xs[1]), coords(xs[2])
    p = F.mul(xs[0], xs[1])
    # find t
    ts = [t for t in range(1 << n) if S3(F, inst['b'], xs[0], xs[1], t) == 0 and
          S3(F, inst['b'], xs[2], inst['XR'], t) == 0]
    out = []
    for t in ts:
        out.append(a + b_ + c + [x ^ y for x, y in zip(a, b_)] +
                   [(p >> k) & 1 for k in range(n)] + [(t >> k) & 1 for k in range(n)])
    return out


# ---------------------------------------------------------------- Macaulay
class LemmaGen:
    def __init__(self, sysd):
        self.sys = sysd
        self.allowed = set(sysd['monos'])
        self.gidx = {}       # allowed mono -> global column
        self.gbasis = {}     # leading bit -> row
        self.ops = 0         # 64-bit word XORs
        self.lemmas = []     # frozensets
        for e in sysd['eqs']:
            self._insert(e)

    def _col(self, m):
        i = self.gidx.get(m)
        if i is None:
            i = self.gidx[m] = len(self.gidx)
        return i

    def _insert(self, e):
        r = 0
        for m in e:
            r ^= 1 << self._col(m)
        while r:
            h = r.bit_length() - 1
            q = self.gbasis.get(h)
            if q is None:
                self.gbasis[h] = r
                return r
            r ^= q
            self.ops += (h >> 6) + 1
        return 0

    def generate(self, mult, lmax=None, eq_idx=None):
        """multipliers: list of variable ids. Returns list of new lemmas."""
        eqs = self.sys['eqs'] if eq_idx is None else [self.sys['eqs'][i] for i in eq_idx]
        rows = [set(e) for e in self.sys['eqs']]
        rows += [set(e) for e in self.lemmas]
        base = list(rows)
        for v in mult:
            for e in (eqs + self.lemmas):
                r = set()
                for m in e:
                    if v in m:
                        mm = m
                    else:
                        mm = tuple(sorted(m + (v,)))
                    if mm in r:
                        r.discard(mm)
                    else:
                        r.add(mm)
                if r:
                    rows.append(r)
        allmon = set()
        for r in rows:
            allmon |= r
        allowed = self.allowed

        def cls(m):
            if len(m) >= 3:
                return 2
            if len(m) == 2 and m not in allowed:
                return 1
            return 0
        order = sorted(allmon, key=lambda m: (cls(m), len(m), m))
        cidx = {m: i for i, m in enumerate(order)}
        n_allowed = sum(1 for m in order if cls(m) == 0)
        piv = {}
        ops = 0
        for r in rows:
            x = 0
            for m in r:
                x |= 1 << cidx[m]
            while x:
                h = x.bit_length() - 1
                q = piv.get(h)
                if q is None:
                    piv[h] = x
                    break
                x ^= q
                ops += (h >> 6) + 1
        self.ops += ops
        cand = []
        for h, x in piv.items():
            if h < n_allowed:
                e = frozenset(order[i] for i in range(h + 1) if (x >> i) & 1)
                cand.append(e)
        cand.sort(key=lambda e: (len(e), sorted(e)))
        new = []
        for e in cand:
            if lmax is not None and len(new) >= lmax:
                break
            if self._insert(e):
                new.append(e)
                self.lemmas.append(e)
        self.last_stats = dict(rows=len(rows), cols=len(order), n_allowed=n_allowed,
                               cand=len(cand), new=len(new))
        return new

    def augment(self, S):
        """declare all pairs of S as allowed monomials (aug variant)."""
        added = []
        for x, y in itertools.combinations(sorted(S), 2):
            if (x, y) not in self.allowed:
                self.allowed.add((x, y))
                added.append((x, y))
        return added


# ---------------------------------------------------------------- CNF + loop
class Enc:
    def __init__(self, sysd):
        self.N = sysd['N']
        self.nv = self.N
        self.mono_var = {}
        self.var_mono = {}
        self.clauses = []
        for m in sorted(sysd['monos']):
            self.mono(m)
        for e in sysd['eqs']:
            self.xor(e)

    def new(self):
        self.nv += 1
        return self.nv

    def mono(self, m):
        v = self.mono_var.get(m)
        if v is None:
            assert len(m) == 2
            v = self.new()
            self.mono_var[m] = v
            self.var_mono[v] = m
            x, y = m[0] + 1, m[1] + 1
            self.clauses += [[-v, x], [-v, y], [v, -x, -y]]
        return v

    def xor(self, e):
        """e: set of monos, XOR == 0. Returns nothing; appends clauses."""
        rhs = 0
        lits = []
        for m in sorted(e, key=lambda m: (len(m), m)):
            if len(m) == 0:
                rhs ^= 1
            elif len(m) == 1:
                lits.append(m[0] + 1)
            else:
                lits.append(self.mono(m))
        # XOR(lits) = rhs
        if not lits:
            if rhs:
                self.clauses.append([])
            return
        while len(lits) > 4:
            y = self.new()
            self._xor_small(lits[:3] + [y], 0)
            lits = [y] + lits[3:]
        self._xor_small(lits, rhs)

    def _xor_small(self, lits, rhs):
        k = len(lits)
        for mask in range(1 << k):
            # forbid assignments with parity != rhs: clause negating assignment
            if bin(mask).count('1') % 2 != rhs:
                self.clauses.append([(-lits[i] if (mask >> i) & 1 else lits[i]) for i in range(k)])

    def take(self):
        c = self.clauses
        self.clauses = []
        return c


def run_arm(inst, sysd, arm, B, k=8, lmax=64, aug=False, replay_counts=None,
            max_rounds=10**6, time_cap=280.0, static_mult=None):
    """arm in {'A0','A1','A2','A3'}. Returns dict of counters."""
    t0 = time.time()
    enc = Enc(sysd)
    gen = LemmaGen(sysd)
    rng = random.Random(inst['seed'] * 7 + 3)
    N = sysd['N']
    lemma_counts = []
    lemma_len = []
    gen_time = 0.0
    if arm == 'A1':
        g0 = time.time()
        mult = list(range(N)) if static_mult is None else static_mult
        new = gen.generate(mult, None)
        for e in new:
            enc.xor(e)
        lemma_counts.append(len(new))
        lemma_len += [len(e) for e in new]
        gen_time += time.time() - g0
    s = Glucose4(bootstrap_with=enc.take(), with_proof=(arm == 'A2'))
    status = None
    rounds = 0
    proof_seen = 0
    while True:
        rounds += 1
        s.conf_budget(B)
        r = s.solve_limited(expect_interrupt=False)
        if r is not None:
            status = 'SAT' if r else 'UNSAT'
            break
        if rounds >= max_rounds or time.time() - t0 > time_cap:
            status = 'TIMEOUT'
            break
        if arm in ('A2', 'A3'):
            g0 = time.time()
            if arm == 'A2':
                pr = s.get_proof()
                if len(pr) >= proof_seen:
                    lines = pr[proof_seen:]
                    proof_seen = len(pr)
                else:  # non-cumulative
                    lines = pr
                cnt = [0] * N
                for ln in lines:
                    if ln.startswith('d'):
                        continue
                    for tok in ln.split():
                        v = abs(int(tok))
                        if v == 0:
                            continue
                        if v <= N:
                            cnt[v - 1] += 1
                        else:
                            m = enc.var_mono.get(v)
                            if m:
                                cnt[m[0]] += 1
                                cnt[m[1]] += 1
                S = sorted(range(N), key=lambda v: (-cnt[v], v))[:k]
                if aug:
                    gen.augment(S)
                new = gen.generate(S, lmax)
            else:
                if replay_counts:
                    target = replay_counts[rounds - 1] if rounds - 1 < len(replay_counts) \
                        else int(round(sum(replay_counts) / len(replay_counts)))
                else:
                    target = lmax
                new = []
                for _ in range(5):
                    if len(new) >= target:
                        break
                    S = rng.sample(range(N), k)
                    if aug:
                        gen.augment(S)
                    new += gen.generate(S, target - len(new))
            for e in new:
                enc.xor(e)
            for c in enc.take():
                s.add_clause(c)
            lemma_counts.append(len(new))
            lemma_len += [len(e) for e in new]
            gen_time += time.time() - g0
    st = s.accum_stats()
    model = s.get_model() if status == 'SAT' else None
    s.delete()
    return dict(arm=arm, status=status, conflicts=st['conflicts'], decisions=st['decisions'],
                propagations=st['propagations'], rounds=rounds, lemmas=sum(lemma_counts),
                lemma_counts=lemma_counts,
                lemma_mean_len=(sum(lemma_len) / len(lemma_len) if lemma_len else 0),
                gen_word_xors=gen.ops, gen_time=gen_time, wall=time.time() - t0,
                nvars=enc.nv, model=model, _lemmas=gen.lemmas)


def check_model(inst, sysd, model):
    """returns (algebraic_ok, lifts)."""
    F, E, l, n = inst['F'], inst['E'], inst['l'], inst['n']
    N = sysd['N']
    assign = [1 if model[i] > 0 else 0 for i in range(N)]
    alg = all(eval_anf(e, assign) == 0 for e in sysd['eqs'])
    xs = []
    for g in ('A', 'B', 'C'):
        x = 0
        for i, v in enumerate(sysd['groups'][g]):
            if assign[v]:
                x ^= inst['basis'][i]
        xs.append(x)
    t = sum(assign[v] << i for i, v in enumerate(sysd['groups']['T']))
    alg = alg and S3(F, inst['b'], xs[0], xs[1], t) == 0 and S3(F, inst['b'], xs[2], inst['XR'], t) == 0
    Ps = [E.lift(x) for x in xs]
    lifts = False
    if all(P is not None for P in Ps):
        for sg in itertools.product((0, 1), repeat=3):
            Q = None
            for P, g in zip(Ps, sg):
                Q = E.add(Q, E.neg(P) if g else P)
            if Q is not None and Q[0] == inst['XR']:
                lifts = True
    return alg, lifts, xs
