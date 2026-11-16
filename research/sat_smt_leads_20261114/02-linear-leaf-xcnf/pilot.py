#!/usr/bin/env python3
"""Idea 2 pilot: linear-leaf XCNF for m=3 point decomposition (see PROTOCOL.md).

Usage: pilot.py OUT.json SEEDBASE DEADLINE_SECONDS L [L ...]
Self-contained: F_{2^23}, Koblitz curve, S3 chain encodings, CMS + Glucose arms.
"""
import sys, os, json, time, random, re, ctypes, tempfile, itertools

N = 23
POLY = (1 << 23) | (1 << 5) | 1
CA, CB = 1, 1
MASK = (1 << N) - 1


def red(x):
    while x >> N:
        x ^= POLY << (x.bit_length() - 1 - N)
    return x


def mul(a, b):
    r = 0
    while b:
        if b & 1:
            r ^= a
        b >>= 1
        a <<= 1
    return red(r)


def sq(a):
    return mul(a, a)


def inv(a):
    assert a
    r, t = 1, a
    for _ in range(N - 1):  # a^(2^N-2) = prod_{i=1}^{N-1} a^(2^i)
        t = sq(t)
        r = mul(r, t)
    return r


def tr(a):
    r, t = a, a
    for _ in range(N - 1):
        t = sq(t)
        r ^= t
    assert r in (0, 1)
    return r


def htr(c):
    r, t = c, c
    for _ in range((N - 1) // 2):
        t = sq(sq(t))
        r ^= t
    return r


def check_field():
    # irreducible: t^(2^N) == t, N prime, no roots in F2
    t = 2
    for _ in range(N):
        t = sq(t)
    assert t == 2
    assert POLY & 1 and bin(POLY).count("1") % 2 == 1
    for a in (3, 12345, 7654321):
        assert mul(a, inv(a)) == 1


def on_curve(P):
    if P is None:
        return True
    x, y = P
    return sq(y) ^ mul(x, y) == mul(sq(x), x) ^ mul(CA, sq(x)) ^ CB


def lift_x(x):
    if x == 0:
        return (0, 1)  # sqrt(b), b = 1
    c = x ^ CA ^ mul(CB, inv(sq(x)))
    if tr(c):
        return None
    z = htr(c)
    P = (x, mul(x, z))
    return P


def neg(P):
    return None if P is None else (P[0], P[0] ^ P[1])


def add(P, Q):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if y1 != y2 or x1 == 0:
            return None
        lam = x1 ^ mul(y1, inv(x1))
        x3 = sq(lam) ^ lam ^ CA
        y3 = sq(x1) ^ mul(lam ^ 1, x3)
        return (x3, y3)
    lam = mul(y1 ^ y2, inv(x1 ^ x2))
    x3 = sq(lam) ^ lam ^ x1 ^ x2 ^ CA
    y3 = mul(lam, x1 ^ x3) ^ x3 ^ y1
    return (x3, y3)


def base_points(l):
    out = []
    for x in range(1 << l):
        P = lift_x(x)
        if P is not None:
            out.append(P)
    return out


def lifts(xs, R):
    """Do x-coordinates lift to points with some signs summing to R?"""
    pts = [lift_x(x) for x in xs]
    if any(p is None for p in pts):
        return False
    for signs in itertools.product((0, 1), repeat=len(pts)):
        S = None
        for p, sg in zip(pts, signs):
            S = add(S, neg(p) if sg else p)
        if S == R:
            return True
    return False


# ---------------------------------------------------------------- native scan
def solve_lin(cols, rhs):
    piv = {}
    kernel = []
    for j, v in enumerate(cols):
        c = 1 << j
        while v:
            h = v.bit_length()
            if h in piv:
                pv, pc = piv[h]
                v ^= pv
                c ^= pc
            else:
                piv[h] = (v, c)
                break
        if not v:
            kernel.append(c)
    v, c = rhs, 0
    while v:
        h = v.bit_length()
        if h not in piv:
            return None, kernel
        pv, pc = piv[h]
        v ^= pv
        c ^= pc
    return c, kernel


def native_scan(l, R, base):
    """Linear 2+1 scan. Returns (decomposable, leaf_solves, witness)."""
    leaves = 0
    wit = None
    for P1 in base:
        us = []
        for Q in (add(R, P1), add(R, neg(P1))):
            if Q is not None and Q[0] not in us:
                us.append(Q[0])
        for u in us:
            leaves += 1
            u2 = sq(u)
            cols = [mul(u2, sq(1 << i)) for i in range(l)]
            cols += [sq(1 << k) ^ mul(u, 1 << k) for k in range(2 * l - 1)]
            part, ker = solve_lin(cols, CB)
            if part is None:
                continue
            assert len(ker) <= 12
            for kk in range(1 << len(ker)):
                sol = part
                for b, kv in enumerate(ker):
                    if (kk >> b) & 1:
                        sol ^= kv
                s = sol & ((1 << l) - 1)
                p = sol >> l
                for x2 in range(1 << l):
                    x3 = s ^ x2
                    if x2 <= x3 and mul(x2, x3) == p and lifts([P1[0], x2, x3], R):
                        if wit is None:
                            wit = (P1[0], x2, x3)
    return wit is not None, leaves, wit


def mitm_table(base):
    signed = base + [neg(p) for p in base if neg(p) != p]
    S2 = set()
    for a in signed:
        for b in signed:
            S2.add(add(a, b))
    return signed, S2


def mitm_decide(R, signed, S2):
    return any(add(R, c) in S2 for c in signed)


# ---------------------------------------------------------------- encodings
SQM = [sq(1 << j) for j in range(N)]
TP = [red(1 << k) for k in range(2 * N)]


def mulmat(c):
    return [mul(c, 1 << j) for j in range(N)]


def lin(M, e):
    out = [0] * N
    for j, f in enumerate(e):
        if f:
            m = M[j]
            k = 0
            while m:
                if m & 1:
                    out[k] ^= f
                m >>= 1
                k += 1
    return out


def exor(*es):
    out = [0] * N
    for e in es:
        for k in range(N):
            out[k] ^= e[k]
    return out


def econst(c):
    return [(c >> k) & 1 for k in range(N)]


class Enc:
    def __init__(self):
        self.nv = 0
        self.clauses = []
        self.xors = []
        self.andc = {}
        self.unsat = False

    def new(self, k):
        r = list(range(self.nv + 1, self.nv + k + 1))
        self.nv += k
        return r

    def evars(self, vs):
        return [1 << v for v in vs] + [0] * (N - len(vs))

    def and_(self, a, b):
        key = (a, b) if a < b else (b, a)
        t = self.andc.get(key)
        if t is None:
            t = self.new(1)[0]
            self.andc[key] = t
            self.clauses += [[-t, a], [-t, b], [t, -a, -b]]
        return t

    def prod(self, av, bv):
        out = [0] * N
        for i, a in enumerate(av):
            for j, b in enumerate(bv):
                t = self.and_(a, b)
                m = TP[i + j]
                k = 0
                while m:
                    if m & 1:
                        out[k] ^= 1 << t
                    m >>= 1
                    k += 1
        return out

    def zero_form(self, f):
        c = f & 1
        vs = [i for i in range(1, f.bit_length()) if (f >> i) & 1]
        if not vs:
            if c:
                self.unsat = True
            return
        self.xors.append((vs, bool(c)))

    def zero(self, e):
        for f in e:
            self.zero_form(f)


def enc_second(E, l, U_vars, u_const, variant, V):
    """S3(x2,x3,u)=0 with p (2l-1 bits). variant 'std' or 'sp'."""
    x2, x3 = E.new(l), E.new(l)
    p = E.new(2 * l - 1)
    V["x2"], V["x3"], V["p"] = x2, x3, p
    for k in range(2 * l - 1):
        f = 1 << p[k]
        for i in range(l):
            j = k - i
            if 0 <= j < l:
                f ^= 1 << E.and_(x2[i], x3[j])
        E.zero_form(f)
    P = E.evars(p)
    if variant == "sp":
        s = E.new(l)
        V["s"] = s
        for i in range(l):
            E.zero_form((1 << s[i]) ^ (1 << x2[i]) ^ (1 << x3[i]))
    if u_const is None:
        if variant == "sp":
            st = E.prod(s, U_vars)
        else:
            st = exor(E.prod(x2, U_vars), E.prod(x3, U_vars))
        eq = exor(lin(SQM, P), lin(SQM, st), E.prod(p, U_vars), econst(CB))
    else:
        assert variant == "sp"
        S = E.evars(s)
        eq = exor(lin(SQM, P), lin(mulmat(sq(u_const)), lin(SQM, S)),
                  lin(mulmat(u_const), P), econst(CB))
    E.zero(eq)


def build_chain(l, xR, variant, x1_const=None):
    """B0 (variant 'std'), T1/T2 ('sp'), T3 ('sp' + x1_const)."""
    E = Enc()
    V = {}
    MR = mulmat(xR)
    u = E.new(N)
    V["u"] = u
    U = E.evars(u)
    if x1_const is None:
        x1 = E.new(l)
        V["x1"] = x1
        X1 = E.evars(x1)
        w = E.prod(x1, u)
    else:
        X1 = econst(x1_const)
        w = lin(mulmat(x1_const), U)
    eqA = exor(lin(SQM, lin(MR, X1)), lin(SQM, w), lin(SQM, lin(MR, U)), lin(MR, w), econst(CB))
    E.zero(eqA)
    enc_second(E, l, u, None, variant, V)
    return E, V


def build_leaf(l, u_const):
    E = Enc()
    V = {}
    enc_second(E, l, None, u_const, "sp", V)
    return E, V


# ---------------------------------------------------------------- solvers
LIBC = ctypes.CDLL(None)
CAP = None  # file object capturing fd 1
CONF_CAP = 300000
TIME_CAP = 120.0


def start_capture():
    global CAP
    sys.stdout.flush()
    CAP = tempfile.TemporaryFile(mode="w+b")
    os.dup2(CAP.fileno(), 1)


def read_capture(pos):
    LIBC.fflush(None)
    end = os.lseek(CAP.fileno(), 0, os.SEEK_END)
    os.lseek(CAP.fileno(), pos, os.SEEK_SET)
    data = os.read(CAP.fileno(), end - pos).decode("ascii", "replace")
    return data, end


def cap_pos():
    LIBC.fflush(None)
    return os.lseek(CAP.fileno(), 0, os.SEEK_END)


SOLVE_RE = re.compile(r"c \[solve\] \d+ ret: (\S+) confl: (\d+)")
MAT_RE = re.compile(r"Using (\d+) matrices")


class CMS:
    def __init__(self, E):
        import pycryptosat
        self.s = pycryptosat.Solver(verbose=1, confl_limit=CONF_CAP, time_limit=TIME_CAP)
        self.E = E
        if E.clauses:
            self.s.add_clauses(E.clauses)
        for vs, rhs in E.xors:
            self.s.add_xor_clause(vs, rhs)
        self.matrices = None

    def solve(self, assumptions=()):
        if self.E.unsat:
            return False, None, 0
        pos = cap_pos()
        sat, model = self.s.solve(list(assumptions))
        data, _ = read_capture(pos)
        m = SOLVE_RE.findall(data)
        assert m, data[-500:]
        mm = MAT_RE.findall(data)
        if mm:
            self.matrices = int(mm[-1])
        return sat, model, int(m[-1][1])


def xors_to_cnf(E):
    cls = [list(c) for c in E.clauses]
    nv = E.nv
    for vs, rhs in E.xors:
        vs = list(vs)
        while len(vs) > 4:
            nv += 1
            a = nv
            chunk, vs = vs[:3], vs[3:] + [a]
            _direct(cls, chunk + [a], False)
        _direct(cls, vs, rhs)
    return cls, nv


def _direct(cls, vs, rhs):
    k = len(vs)
    for bits in range(1 << k):
        # forbid assignments with parity != rhs
        par = bin(bits).count("1") & 1
        if par != int(rhs):
            cls.append([(-v if (bits >> i) & 1 else v) for i, v in enumerate(vs)])


class GLU:
    def __init__(self, E):
        from pysat.solvers import Glucose4
        self.E = E
        cls, nv = xors_to_cnf(E)
        self.s = Glucose4(bootstrap_with=cls)
        self.last = {"conflicts": 0, "decisions": 0, "propagations": 0}

    def solve(self, assumptions=()):
        if self.E.unsat:
            return False, None, {"conflicts": 0, "decisions": 0, "propagations": 0}
        self.s.conf_budget(CONF_CAP)
        r = self.s.solve_limited(assumptions=list(assumptions))
        st = self.s.accum_stats()
        d = {k: st[k] - self.last[k] for k in self.last}
        self.last = {k: st[k] for k in self.last}
        model = None
        if r:
            mm = self.s.get_model()
            model = [None] + [x > 0 for x in mm]
        return r, model, d

    def close(self):
        self.s.delete()


def val(model, vs):
    return sum(1 << i for i, v in enumerate(vs) if model[v])


def assume(vs, x):
    return [v if (x >> i) & 1 else -v for i, v in enumerate(vs)]


# ---------------------------------------------------------------- arms
def arm_free(kind, variant, l, R):
    t0 = time.time()
    E, V = build_chain(l, R[0], variant)
    S = CMS(E) if kind == "cms" else GLU(E)
    sat, model, st = S.solve()
    row = {"calls": 1, "nvars": E.nv, "nxors": len(E.xors), "nclauses": len(E.clauses)}
    if kind == "cms":
        row["conflicts"] = st
        row["matrices"] = S.matrices
    else:
        row.update(st)
        S.close()
    row["status"] = "SAT" if sat else ("UNSAT" if sat is False else "CENSORED")
    if sat:
        xs = [val(model, V[k]) for k in ("x1", "x2", "x3")]
        row["model"] = xs
        row["lift_ok"] = lifts(xs, R)
    row["time"] = time.time() - t0
    return row


def arm_assume(kind, l, R):
    t0 = time.time()
    E, V = build_chain(l, R[0], "sp")
    S = CMS(E) if kind == "cms" else GLU(E)
    tot = {"conflicts": 0, "decisions": 0, "propagations": 0}
    calls = 0
    status = "UNSAT"
    row = {}
    prev = 0
    for x1 in range(1 << l):
        sat, model, st = S.solve(assume(V["x1"], x1))
        calls += 1
        if kind == "cms":
            tot["conflicts"] += st
        else:
            for k in tot:
                tot[k] += st[k]
        if sat is None:
            status = "CENSORED"
            break
        if sat:
            status = "SAT"
            xs = [val(model, V[k]) for k in ("x1", "x2", "x3")]
            row["model"] = xs
            row["lift_ok"] = lifts(xs, R)
            break
    if kind == "cms":
        row["conflicts"] = tot["conflicts"]
        row["matrices"] = S.matrices
    else:
        row.update(tot)
        S.close()
    row.update({"calls": calls, "status": status, "time": time.time() - t0})
    return row


def arm_subst_x1(l, R):
    """T3: fresh CMS per x1 constant; u left to the solver."""
    t0 = time.time()
    confl = calls = 0
    status = "UNSAT"
    row = {}
    for x1 in range(1 << l):
        E, V = build_chain(l, R[0], "sp", x1_const=x1)
        sat, model, c = CMS(E).solve()
        calls += 1
        confl += c
        if sat is None:
            status = "CENSORED"
            break
        if sat:
            status = "SAT"
            xs = [x1, val(model, V["x2"]), val(model, V["x3"])]
            row["model"] = xs
            row["lift_ok"] = lifts(xs, R)
            break
    row.update({"conflicts": confl, "calls": calls, "status": status, "time": time.time() - t0})
    return row


def arm_leaf(l, R, base):
    """T4: x1 over base and both roots u substituted natively; pure XOR leaf in (s,p)."""
    t0 = time.time()
    confl = calls = 0
    lvl0 = 0
    status = "UNSAT"
    row = {}
    done = False
    for P1 in base:
        us = []
        for Q in (add(R, P1), add(R, neg(P1))):
            if Q is not None and Q[0] not in us:
                us.append(Q[0])
        for u in us:
            E, V = build_leaf(l, u)
            sat, model, c = CMS(E).solve()
            calls += 1
            confl += c
            lvl0 += 1 if c == 0 else 0
            if sat is None:
                status = "CENSORED"
                done = True
                break
            if sat:
                xs = [P1[0], val(model, V["x2"]), val(model, V["x3"])]
                if lifts(xs, R):
                    status = "SAT"
                    row["model"] = xs
                    row["lift_ok"] = True
                    done = True
                    break
                row["spurious_leaf_models"] = row.get("spurious_leaf_models", 0) + 1
        if done:
            break
    row.update({"conflicts": confl, "calls": calls, "zero_conflict_leaves": lvl0,
                "status": status, "time": time.time() - t0})
    return row


# ---------------------------------------------------------------- driver
def make_target(l, base, rng, planted):
    lim = 1 << l
    while True:
        if planted:
            R = None
            xs = []
            for _ in range(3):
                P = rng.choice(base)
                xs.append(P[0])
                R = add(R, neg(P) if rng.getrandbits(1) else P)
        else:
            xs = None
            P = lift_x(rng.getrandbits(N))
            if P is None:
                continue
            R = neg(P) if rng.getrandbits(1) else P
        if R is None or R[0] < lim:
            continue
        assert on_curve(R)
        return R, xs


def main():
    out, seedbase, deadline = sys.argv[1], int(sys.argv[2]), float(sys.argv[3])
    ls = [int(a) for a in sys.argv[4:]]
    ntarg = int(os.environ.get("NTARG", "8"))
    free_arms = os.environ.get("ARMS", "B0,T1,T2,T3,T4,G-B0,G-T1,G-T2").split(",")
    check_field()
    start_capture()
    T0 = time.time()
    rows = []
    log = lambda *a: print(*a, file=sys.stderr, flush=True)
    for l in ls:
        base = base_points(l)
        assert all(on_curve(p) for p in base)
        signed, S2 = mitm_table(base) if l <= 7 else (None, None)
        for idx in range(2 * ntarg):
            planted = idx < ntarg
            seed = seedbase * 1000 + l * 100 + idx
            rng = random.Random(seed)
            R, plant = make_target(l, base, rng, planted)
            dec, leaves, wit = native_scan(l, R, base)
            if planted:
                assert dec, "planted target not found by native scan"
            if S2 is not None:
                assert mitm_decide(R, signed, S2) == dec, "MITM brute force disagrees with linear scan"
            common = {"l": l, "idx": idx, "seed": seed, "planted": planted, "decomposable": dec,
                      "R": list(R), "base_size": len(base)}
            rows.append(dict(common, arm="L", unit="leaf_solves", work=leaves, calls=leaves,
                             status="SAT" if dec else "UNSAT"))
            for arm in free_arms:
                if time.time() - T0 > deadline:
                    rows.append(dict(common, arm=arm, status="NOT_RUN"))
                    continue
                if arm == "B0":
                    r = arm_free("cms", "std", l, R)
                elif arm == "T1":
                    r = arm_free("cms", "sp", l, R)
                elif arm == "T2":
                    r = arm_assume("cms", l, R)
                elif arm == "T3":
                    r = arm_subst_x1(l, R)
                elif arm == "T4":
                    r = arm_leaf(l, R, base)
                elif arm == "G-B0":
                    r = arm_free("glu", "std", l, R)
                elif arm == "G-T1":
                    r = arm_free("glu", "sp", l, R)
                elif arm == "G-T2":
                    r = arm_assume("glu", l, R)
                else:
                    raise SystemExit("unknown arm " + arm)
                r.update(common, arm=arm)
                if r["status"] != "CENSORED":
                    assert (r["status"] == "SAT") == dec, ("answer mismatch", arm, common, r)
                    if r["status"] == "SAT":
                        assert r["lift_ok"], ("model does not lift", arm, common, r)
                rows.append(r)
                log(f"l={l} idx={idx} dec={int(dec)} {arm}: {r['status']} confl={r.get('conflicts')} "
                    f"calls={r.get('calls')} t={r['time']:.2f}")
            with open(out, "w") as f:
                json.dump({"n": N, "poly": POLY, "curve": [CA, CB], "seedbase": seedbase,
                           "conf_cap": CONF_CAP, "time_cap": TIME_CAP, "rows": rows}, f, indent=0)
    log("done", time.time() - T0)


if __name__ == "__main__":
    main()
