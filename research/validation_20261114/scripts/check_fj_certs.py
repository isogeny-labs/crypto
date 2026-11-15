#!/usr/bin/env python3
"""Independent certificate check for research/full_jacobian_ic_20261113.

Validator V, 2026-11-14. Input: the JSON lines printed by val_fj_dump (curve
data, planted logs, every recovered log and per-run counter) and the committed
full_jacobian.json. Own arithmetic only (no producer code):

  * E1: y^2 = x^3 + a2 x^2 + a4 x + a6 over F_p; affine group law; #E1 and #E2
    by Legendre-symbol point counting; r prime, r || #E1, r not | #E2,
    r^2 not | #E1 #E2; j(E1) not in {0, 1728} (so the negation map is the
    strongest automorphism fold on a prime-field curve); G on E1 of order r;
    T = [x]G with x from the planted formula; [k]G == T for every recovered
    log of every IC arm and every rho run.
  * J_C: Cantor's algorithm on the imaginary quintic model y^2 = f(t)
    (H64_f, lowest degree first); every divisor reduced and on the curve;
    [r]G' = 0 != G'; [x]G' == T'; [k]G' == T' for every IC log;
    [#J]D = 0 for G', Z and random divisors; N1(C) gives a1(C) = a1(E1)+a1(E2);
    for small p also the full L-polynomial: #J = L(1) = #E1 #E2.
  * Per-prime means of every per-run counter recomputed and compared with
    the committed json (exact).

Usage: check_fj_certs.py DUMP.jsonl FULL_JACOBIAN.json OUT.json [--lpoly-max-p 3203]
"""
import json, random, sys

dump_path, fj_path, out_path = sys.argv[1:4]
LPOLY_MAX = int(sys.argv[sys.argv.index("--lpoly-max-p") + 1]) if "--lpoly-max-p" in sys.argv else 3203
committed = {r["p"]: r for r in json.load(open(fj_path))["rows"]}
M64 = (1 << 64) - 1


def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True


def factor(n):
    f, d = {}, 2
    while d * d <= n:
        while n % d == 0:
            f[d] = f.get(d, 0) + 1; n //= d
        d += 1
    if n > 1: f[n] = f.get(n, 0) + 1
    return f


# ---------------- elliptic curve ----------------
class EC:
    def __init__(s, p, a2, a4, a6): s.p, s.a2, s.a4, s.a6 = p, a2 % p, a4 % p, a6 % p
    def on(s, P):
        if P is None: return True
        x, y = P; p = s.p
        return (y * y - (x ** 3 + s.a2 * x * x + s.a4 * x + s.a6)) % p == 0
    def add(s, P, Q):
        p = s.p
        if P is None: return Q
        if Q is None: return P
        (x1, y1), (x2, y2) = P, Q
        if x1 == x2:
            if (y1 + y2) % p == 0: return None
            lam = (3 * x1 * x1 + 2 * s.a2 * x1 + s.a4) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - s.a2 - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)
    def mul(s, P, n):
        R = None
        while n:
            if n & 1: R = s.add(R, P)
            P = s.add(P, P); n >>= 1
        return R
    def count(s):
        p, n = s.p, 1
        for x in range(p):
            v = (x ** 3 + s.a2 * x * x + s.a4 * x + s.a6) % p
            n += 1 if v == 0 else (2 if pow(v, (p - 1) // 2, p) == 1 else 0)
        return n
    def j(s):
        p = s.p
        b2, b4, b6 = 4 * s.a2 % p, 2 * s.a4 % p, 4 * s.a6 % p
        b8 = (4 * s.a2 * s.a6 - s.a4 * s.a4) % p
        c4 = (b2 * b2 - 24 * b4) % p
        disc = (-b2 * b2 * b8 - 8 * b4 ** 3 - 27 * b6 * b6 + 9 * b2 * b4 * b6) % p
        return disc, (c4 ** 3 * pow(disc, -1, p) % p if disc else None)


# ---------------- polynomials over F_p (lists, lowest degree first) ----------------
def trim(a):
    while a and a[-1] == 0: a.pop()
    return a

def padd(a, b, p):
    n = max(len(a), len(b)); return trim([((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0)) % p for i in range(n)])

def psub(a, b, p):
    n = max(len(a), len(b)); return trim([((a[i] if i < len(a) else 0) - (b[i] if i < len(b) else 0)) % p for i in range(n)])

def pmul(a, b, p):
    if not a or not b: return []
    c = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b): c[i + j] = (c[i + j] + x * y) % p
    return trim(c)

def pdivmod(a, b, p):
    a = a[:]; q = [0] * max(len(a) - len(b) + 1, 1)
    inv = pow(b[-1], -1, p)
    while len(a) >= len(b) and a:
        c = a[-1] * inv % p; d = len(a) - len(b); q[d] = c
        for i, y in enumerate(b): a[i + d] = (a[i + d] - c * y) % p
        trim(a)
    return trim(q), a

def pmod(a, b, p): return pdivmod(a, b, p)[1]

def monic(a, p):
    inv = pow(a[-1], -1, p); return [x * inv % p for x in a]

def xgcd(a, b, p):
    # returns (d, s, t) with s a + t b = d, d monic
    r0, r1, s0, s1, t0, t1 = a[:], b[:], [1], [], [], [1]
    while r1:
        q, r = pdivmod(r0, r1, p)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(q, s1, p), p)
        t0, t1 = t1, psub(t0, pmul(q, t1, p), p)
    inv = pow(r0[-1], -1, p)
    return [x * inv % p for x in r0], [x * inv % p for x in s0], [x * inv % p for x in t0]


class Jac:
    """Genus-2 Jacobian of y^2 = f(t), deg f = 5, Mumford (u monic, deg v < deg u)."""
    def __init__(s, p, f): s.p, s.f, s.g = p, trim([c % p for c in f]), 2
    ID = ([1], [])
    def valid(s, D):
        u, v = D; p = s.p
        if not u or u[-1] != 1 or len(u) - 1 > s.g or (v and len(v) >= len(u)): return False
        return not pmod(psub(pmul(v, v, p), s.f, p), u, p)
    def add(s, D1, D2):
        p = s.p; (u1, v1), (u2, v2) = D1, D2
        d1, e1, e2 = xgcd(u1, u2, p)
        d, c1, c2 = xgcd(d1, padd(v1, v2, p), p)
        s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
        dinv_num = pmul(u1, u2, p)
        u = pdivmod(dinv_num, pmul(d, d, p), p)[0]
        num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
                   pmul(s3, padd(pmul(v1, v2, p), s.f, p), p), p)
        v = pmod(pdivmod(num, d, p)[0], u, p)
        while len(u) - 1 > s.g:
            u = monic(pdivmod(psub(s.f, pmul(v, v, p), p), u, p)[0], p)
            v = pmod([(-c) % p for c in v], u, p)
        return (monic(u, p), trim(v))
    def mul(s, D, n):
        R = s.ID
        while n:
            if n & 1: R = s.add(R, D)
            D = s.add(D, D); n >>= 1
        return R
    def eq(s, A, B): return trim(A[0][:]) == trim(B[0][:]) and trim(A[1][:]) == trim(B[1][:])
    def rand(s, rng):
        p = s.p
        D = s.ID
        for _ in range(2):
            while True:
                x = rng.randrange(p)
                fx = 0
                for c in reversed(s.f): fx = (fx * x + c) % p
                if fx and pow(fx, (p - 1) // 2, p) == 1:
                    y = sqrt_mod(fx, p); break
            D = s.add(D, ([(-x) % p, 1], [y]))
        return D


def sqrt_mod(a, p):
    a %= p
    if p % 4 == 3: return pow(a, (p + 1) // 4, p)
    # Tonelli-Shanks
    q, s_ = p - 1, 0
    while q % 2 == 0: q //= 2; s_ += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1: z += 1
    m, c, t, r = s_, pow(z, q, p), pow(a, q, p), pow(a, (q + 1) // 2, p)
    while t != 1:
        i, tt = 0, t
        while tt != 1: tt = tt * tt % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def div_of(d):
    return ([c for c in d["u"]], trim([c for c in d["v"]]))


def lpoly_order(p, f):
    """#J = L(1) from N1 and N2 of y^2 = f(t) (deg 5): N_k = p^k + sum_t (1 + chi(f(t))) over F_{p^k}, + 1 at infinity."""
    # N1
    s = 0
    for t in range(p):
        v = 0
        for c in reversed(f): v = (v * t + c) % p
        s += 0 if v == 0 else (1 if pow(v, (p - 1) // 2, p) == 1 else -1)
    N1 = p + 1 + s
    # F_{p^2} = F_p(sqrt(n)), n a non-residue; chi_{p^2}(z) = chi_p(N(z))
    n = 2
    while pow(n, (p - 1) // 2, p) != p - 1: n += 1
    s2 = 0
    for a in range(p):
        for b in range(p):
            # Horner in F_{p^2}: (va + vb w)
            va, vb = 0, 0
            for c in reversed(f):
                va, vb = (va * a + vb * b * n + c) % p, (va * b + vb * a) % p
            nm = (va * va - n * vb * vb) % p
            s2 += 0 if nm == 0 else (1 if pow(nm, (p - 1) // 2, p) == 1 else -1)
    N2 = p * p + 1 + s2
    s1 = p + 1 - N1
    ss2 = p * p + 1 - N2
    e2 = (s1 * s1 - ss2) // 2
    return N1, N2, 1 - s1 + e2 - p * s1 + p * p


out = {"primes": [], "failures": []}
fail = out["failures"]
tot = {"ic_logs": 0, "ic_logs_ok_E1": 0, "ic_logs_ok_J": 0, "rho_logs": 0, "rho_logs_ok": 0, "means_compared": 0, "means_mismatch": 0}
for line in open(dump_path):
    if not line.strip(): continue
    D = json.loads(line)
    p, r = D["p"], D["r"]
    rep = {"p": p, "r": r}
    E1 = EC(p, *D["E1"]); E2 = EC(p, *D["E2"])
    disc, j = E1.j()
    rep["E1"] = D["E1"]; rep["j_E1"] = j
    if disc == 0 or j in (0, 1728 % p): fail.append(f"p{p}: E1 singular or j in {{0,1728}}: j={j}")
    n1, n2 = E1.count(), E2.count()
    rep.update({"n1": n1, "n2": n2})
    C = committed[p]
    for k_, v_ in (("e1_order", n1), ("e2_order", n2), ("jac_order", n1 * n2), ("r", r)):
        if C[k_] != v_: fail.append(f"p{p}: committed {k_}={C[k_]} != recomputed {v_}")
    fr = factor(n1)
    if not is_prime(r) or fr.get(r) != 1: fail.append(f"p{p}: r={r} not a prime exactly dividing #E1 {fr}")
    lcp = max([q for q, e in fr.items() if e == 1] or [1])
    if lcp != r: fail.append(f"p{p}: largest exactly-dividing prime {lcp} != r {r}")
    if n2 % r == 0 or (n1 * n2) % (r * r) == 0: fail.append(f"p{p}: r divides #E2 or r^2 | #J")
    # planted logs
    xs = []
    for k in range(len(D["instances"])):
        seed = D["seed0"] ^ (k * 0x1000)
        xs.append(((seed * 2246822519 + 7) & M64) % r)
    if xs != C["x"][:len(xs)] or [I["x"] for I in D["instances"]] != xs:
        fail.append(f"p{p}: planted x mismatch {xs} {C['x']}")
    if D["seed0"] != C["seed0"] or D["candidate"] != C["candidate"]:
        fail.append(f"p{p}: seed0/candidate mismatch")
    if D["seed0"] != 0xC0FFEE + D["i"] * 101 + D["candidate"]:
        fail.append(f"p{p}: seed0 formula")
    g = D["G_E1"]
    if g[0][1] or g[1][1]: fail.append(f"p{p}: G not rational")
    G = (g[0][0], g[1][0])
    if not E1.on(G) or G is None or E1.mul(G, r) is not None: fail.append(f"p{p}: G not of order r on E1")
    Ts = []
    for k, I in enumerate(D["instances"]):
        t = I["T_E1"]
        T = None if t is None else (t[0][0], t[1][0])
        if T != E1.mul(G, xs[k]): fail.append(f"p{p} k{k}: T != [x]G")
        Ts.append(T)
    # Jacobian
    J = Jac(p, D["H64_f"])
    rep["H64_f"] = D["H64_f"]
    if len(J.f) - 1 != 5: fail.append(f"p{p}: f not quintic")
    rng = random.Random(p)
    njac = n1 * n2
    GJ = div_of(D["instances"][0]["G_J"])
    for k, I in enumerate(D["instances"]):
        Gk, Tk, Zk = div_of(I["G_J"]), div_of(I["T_J"]), div_of(I["Z"])
        for nm, Dv in (("G'", Gk), ("T'", Tk), ("Z", Zk)):
            if not J.valid(Dv): fail.append(f"p{p} k{k}: {nm} not a reduced divisor on C")
        if not J.eq(Gk, GJ): fail.append(f"p{p} k{k}: G' differs between instances")
        if J.eq(Gk, J.ID) or not J.eq(J.mul(Gk, r), J.ID): fail.append(f"p{p} k{k}: G' not of order r")
        if not J.eq(J.mul(Gk, xs[k]), Tk): fail.append(f"p{p} k{k}: T' != [x]G'")
        if not J.eq(J.mul(Zk, njac), J.ID): fail.append(f"p{p} k{k}: [#J]Z != 0")
        if not J.eq(J.mul(Zk, I["z_order"]), J.ID): fail.append(f"p{p} k{k}: [ord Z]Z != 0")
        if I["z_order"] / njac != C["z_order_over_jac_order"][k]: fail.append(f"p{p} k{k}: z coverage mismatch")
        if J.eq(J.mul(Zk, C["h"]), J.ID): fail.append(f"p{p} k{k}: hZ == 0")
        if I["jac_order_inst"] != njac or I["h_inst"] != C["h"]: fail.append(f"p{p} k{k}: instance order/h mismatch")
        if I["seed"] != D["seed0"] ^ (k * 0x1000): fail.append(f"p{p} k{k}: instance seed")
    for _ in range(6):
        Rd = J.rand(rng)
        if not J.valid(Rd) or not J.eq(J.mul(Rd, njac), J.ID): fail.append(f"p{p}: random divisor not killed by #E1#E2")
    # trace check a1(C) = a1(E1) + a1(E2)
    s = 0
    for t in range(p):
        v = 0
        for c in reversed(J.f): v = (v * t + c) % p
        s += 0 if v == 0 else (1 if pow(v, (p - 1) // 2, p) == 1 else -1)
    NC1 = p + 1 + s
    rep["NC1"] = NC1
    if (p + 1 - NC1) != (p + 1 - n1) + (p + 1 - n2): fail.append(f"p{p}: a1(C) != a1(E1)+a1(E2)")
    if p <= LPOLY_MAX:
        N1, N2, LJ = lpoly_order(p, J.f)
        rep["L1"] = LJ
        if LJ != njac: fail.append(f"p{p}: L(1)={LJ} != #E1#E2={njac}")
    # IC logs
    per = {}
    for arm, runs in D["arms"].items():
        okE = okJ = 0
        for run in runs:
            k = run["k"]; tot["ic_logs"] += 1
            kk = run["recovered"]
            I = D["instances"][k]
            if kk is None or not run["certified"] or run["exhausted"]:
                fail.append(f"p{p} {arm} k{k} s{run['s']}: no certified log"); continue
            if E1.mul(G, kk) == Ts[k] and kk % r == xs[k]: okE += 1; tot["ic_logs_ok_E1"] += 1
            else: fail.append(f"p{p} {arm} k{k} s{run['s']}: [k]G != T on E1 (k={kk}, x={xs[k]})")
            if J.eq(J.mul(div_of(I["G_J"]), kk), div_of(I["T_J"])): okJ += 1; tot["ic_logs_ok_J"] += 1
            else: fail.append(f"p{p} {arm} k{k} s{run['s']}: [k]G' != T' on J_C")
        # means vs committed
        n = len(runs)
        cm = C["arms"][arm]
        pairs = [("group_ops", "group_ops"), ("rel_mul", "rel_mul"), ("rel_add", "rel_add"), ("la_mul", "la_mul"),
                 ("la_add", "la_add"), ("trials", "trials"), ("relations", "relations"), ("setup_ops", "setup_ops"),
                 ("walk_ops", "walk_ops"), ("verify_ops", "verify_ops"), ("attempts", "attempts"), ("fb_cols", "columns")]
        mm = []
        for a_, b_ in pairs:
            mine = sum(float(x[a_]) for x in runs) / n
            tot["means_compared"] += 1
            if mine != cm[b_]:
                tot["means_mismatch"] += 1; mm.append((b_, cm[b_], mine))
        if n != cm["runs"]: mm.append(("runs", cm["runs"], n))
        mx = max(x["group_ops"] for x in runs); mn = min(x["group_ops"] for x in runs)
        if mx != cm["group_ops_max"] or mn != cm["group_ops_min"]: mm.append(("group_ops_min/max", [cm["group_ops_min"], cm["group_ops_max"]], [mn, mx]))
        if arm in ("A", "B"):
            rates = [x["smooth_rate"] for x in runs]
            mine = sum(rates) / n
            if abs(mine - cm["smooth_rate_deg2"]) > 1e-12: mm.append(("smooth_rate_deg2", cm["smooth_rate_deg2"], mine))
            per.setdefault("smooth_rate_per_run", {})[arm] = [min(rates), max(rates)]
        if mm: fail.append(f"p{p} {arm}: means differ from committed json: {mm}")
        per[arm] = {"runs": n, "logs_ok_E1": okE, "logs_ok_J": okJ}
    # rho logs and means
    for neg in (True, False):
        rr = [x for x in D["rho_E"] if x["neg"] == neg]
        okc = 0
        for x in rr:
            tot["rho_logs"] += 1
            k = x["k"]
            if x["recovered"] is not None and x["certified"] and E1.mul(G, x["recovered"]) == Ts[k]:
                okc += 1; tot["rho_logs_ok"] += 1
            else: fail.append(f"p{p} rho neg={neg} k{k} s{x['s']}: log not certified on E1")
        key = "negation" if neg else "plain"
        cm = C["rho_E"][key]
        for a_ in ("total_ops", "native"):
            mine = sum(float(x[a_]) for x in rr) / len(rr)
            tot["means_compared"] += 1
            if mine != cm[a_]:
                tot["means_mismatch"] += 1; fail.append(f"p{p} rho {key} {a_}: committed {cm[a_]} != dump {mine}")
        per["rho_" + key] = {"runs": len(rr), "logs_ok_E1": okc}
    rep["arms"] = per
    out["primes"].append(rep)
    print(f"p={p} r={r} j={j} n1={n1} n2={n2} L1={rep.get('L1')} arms={json.dumps(per)}", flush=True)

out["totals"] = tot
out["n_failures"] = len(fail)
json.dump(out, open(out_path, "w"), indent=1)
print(json.dumps(tot), "failures:", len(fail))
for f_ in fail[:40]: print("FAIL", f_)
