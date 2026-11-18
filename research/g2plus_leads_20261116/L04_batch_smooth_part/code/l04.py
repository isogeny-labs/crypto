"""L04 library: batch smooth-part test of Mumford u-polynomials (SPEC.md sections 1-6)."""
import math
import random


class Ops:
    __slots__ = ("mul", "inv", "add")

    def __init__(self):
        self.mul = 0
        self.inv = 0
        self.add = 0

    def d(self):
        return {"mul": self.mul, "inv": self.inv, "add": self.add}


def fpm(o):
    return o.mul + 10 * o.inv


def fpm_lab(o, p):
    return o.mul + o.inv * ((p - 2).bit_length() + bin(p - 2).count("1") - 2)


def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def inverse(a, p):
    return pow(a, p - 2, p)


def is_prime(n):
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def peval(a, x, p):  # uncounted Horner
    r = 0
    for c in reversed(a):
        r = (r * x + c) % p
    return r


def umul(a, b, p):  # uncounted multiply
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % p
    return trim(out)


# ---------------- setup ----------------
def chi_table(p, ops):
    chi = [-1] * p
    chi[0] = 0
    for y in range(1, (p - 1) // 2 + 1):
        chi[y * y % p] = 1
    ops.mul += (p - 1) // 2
    return chi


def make_f(p, g, seed_f):
    rng = random.Random(seed_f)
    return [rng.randrange(p) for _ in range(2 * g + 1)] + [1]


def make_S(p, g, f, chi, N, ops):
    S = []
    x = 0
    while len(S) < N and x < p:
        fx = 0
        for cf in reversed(f):
            fx = (fx * x + cf) % p
        ops.mul += len(f)
        ops.add += len(f)
        if chi[fx] == 1:
            S.append(x)
        x += 1
    inS = bytearray(p)
    for s in S:
        inS[s] = 1
    return S, inS


def compute_N(p, g, nfb=None):
    if nfb is not None:
        return nfb
    return min(math.ceil(1.0 * p ** (1.0 - 1.0 / g)), p)


def build_FS(S, p, ops):
    F = [1]
    for s in S:
        new = [0] + F  # x*F
        # new[i] -= s*F[i]
        new[:len(F)] = [(a - s * b) % p for a, b in zip(new[:len(F)], F)]
        new = [v % p for v in new]
        ops.mul += len(F)
        F = new
    return F


# ---------------- ModU ----------------
def modu_mul(a, b, uu, d, p, ops):
    t = [0] * (2 * d)
    for i in range(d):
        for j in range(d):
            t[i + j] += a[i] * b[j]
    ops.mul += d * d
    ops.add += d * d
    for k in range(2 * d - 2, d - 1, -1):
        c = t[k] % p
        for j in range(d):
            t[k - d + j] -= uu[j] * c
        ops.mul += d
        ops.add += d
    return [t[i] % p for i in range(d)]


def modu_mul_xpa(a, aa, uu, d, p, ops):
    top = a[d - 1]
    out = [0] + a[:d - 1]
    for j in range(d):
        out[j] = (out[j] - uu[j] * top + aa * a[j]) % p
    w = d if aa == 0 else 2 * d
    ops.mul += w
    ops.add += w
    return out


def modu_base(aa, uu, d, p):
    if d == 1:
        return [(aa - uu[0]) % p]
    return [aa, 1] + [0] * (d - 2)


def pow_xpa(aa, e, uu, d, p, ops):
    res = modu_base(aa, uu, d, p)
    bits = e.bit_length()
    for i in range(bits - 2, -1, -1):
        res = modu_mul(res, res, uu, d, p, ops)
        if (e >> i) & 1:
            res = modu_mul_xpa(res, aa, uu, d, p, ops)
    return res


def splits_completely(u, p, ops):
    d = len(u) - 1
    uu = u[:d]
    xp = pow_xpa(0, p, uu, d, p, ops)
    return xp == modu_base(0, uu, d, p)


# ---------------- Euclid ----------------
def vrem(a, m, p, ops):
    r = list(a)
    trim(r)
    dm = len(m) - 1
    lead = m[-1]
    if lead != 1:
        li = inverse(lead, p)
        ops.inv += 1
    else:
        li = 1
    while len(r) > dm:
        top = len(r) - 1
        c = r[top]
        if li != 1:
            c = c * li % p
            ops.mul += 1
        if dm:
            s = top - dm
            r[s:s + dm] = [(x - c * mj) % p for x, mj in zip(r[s:s + dm], m)]
        ops.mul += dm
        ops.add += dm
        r.pop()
        trim(r)
    return r


def vgcd_monic(a, b, p, ops):
    a = trim(list(a))
    b = trim(list(b))
    while b:
        r = vrem(a, b, p, ops)
        a, b = b, r
    if a and a[-1] != 1:
        il = inverse(a[-1], p)
        ops.inv += 1
        a = [x * il % p for x in a]
        ops.mul += len(a)
    return a


def vdiv_monic(a, m, p, ops):
    dm = len(m) - 1
    r = list(a)
    q = [0] * (len(a) - dm)
    for top in range(len(a) - 1, dm - 1, -1):
        c = r[top]
        q[top - dm] = c
        for j in range(dm):
            r[top - dm + j] = (r[top - dm + j] - c * m[j]) % p
        ops.mul += dm
        ops.add += dm
    return q


def poly_mul(a, b, p, ops):
    la, lb = len(a), len(b)
    out = [0] * (la + lb - 1)
    for i, ai in enumerate(a):
        out[i:i + lb] = [o + ai * bj for o, bj in zip(out[i:i + lb], b)]
    ops.mul += la * lb
    return trim([v % p for v in out])


def disc_character(u, chi, p, ops):
    d = len(u) - 1
    a = list(u)
    b = [u[i] * (i % p) % p for i in range(1, d + 1)]
    ops.mul += d
    trim(b)
    sign = chi[p - 1] if (d * (d - 1) // 2) % 2 == 1 else 1
    while True:
        if not b:
            return 0
        m = len(a) - 1
        nn = len(b) - 1
        if nn == 0:
            return sign * chi[b[0]] if m % 2 == 1 else sign
        lb = b[nn]
        delta = m - nn + 1
        r = list(a)
        for step in range(delta):
            top = m - step
            c = r[top]
            for i in range(top):
                r[i] = r[i] * lb % p
            for j in range(nn):
                r[top - nn + j] = (r[top - nn + j] - c * b[j]) % p
            ops.mul += top + nn
            ops.add += nn
            del r[top:]
        trim(r)
        if not r:
            return 0
        dr = len(r) - 1
        if (m + dr + delta * nn) % 2 == 1:
            sign *= chi[lb]
        if (m * nn) % 2 == 1:
            sign *= chi[p - 1]
        a, b = b, r


def roots_of(u, p, rng, ops):
    d = len(u) - 1
    if d == 0:
        return []
    if d == 1:
        return [(-u[0]) % p]
    uu = u[:d]
    for _ in range(64):
        aa = rng.randrange(p)
        h = pow_xpa(aa, (p - 1) // 2, uu, d, p, ops)
        hm1 = list(h)
        hm1[0] = (hm1[0] - 1) % p
        trim(hm1)
        if not hm1:
            continue
        gg = vgcd_monic(u, hm1, p, ops)
        dg = len(gg) - 1
        if dg in (0, d):
            continue
        q = vdiv_monic(u, gg, p, ops)
        r1 = roots_of(gg, p, rng, ops)
        if r1 is None:
            return None
        r2 = roots_of(q, p, rng, ops)
        if r2 is None:
            return None
        return r1 + r2
    return None


# ---------------- post-test ----------------
def post_S(u, R, g, chi, inS, p, ops):
    """Return (accept, G, c)."""
    if not R:
        return True, list(u), [1]
    G = vgcd_monic(u, R, p, ops)
    k = len(G) - 1
    if k < g - 2:
        return False, G, None
    c = vdiv_monic(u, G, p, ops) if k >= 1 else list(u)
    dc = len(c) - 1
    if dc == 0:
        return True, G, c
    if dc == 1:
        r = (-c[0]) % p
        if inS[r]:
            return False, G, c
        return True, G, c
    # dc == 2
    disc = (c[1] * c[1] - 4 * c[0]) % p
    ops.mul += 2
    if chi[disc] != 1:
        return False, G, c
    if k >= 1:
        h = vgcd_monic(c, G, p, ops)
        if len(h) - 1 >= 1:
            return False, G, c
    return True, G, c


# ---------------- arms ----------------
class Ctx:
    def __init__(self, g, p, S, inS, FS, chi):
        self.g, self.p, self.S, self.inS, self.FS, self.chi = g, p, S, inS, FS, chi


def new_result():
    return {"pre": Ops(), "dec": Ops(), "roots": Ops(), "tprod": None, "trem": None,
            "post": None, "dec_bits": None, "rts": {}, "cz_fail": 0, "pass": None,
            "batch_sizes": None, "split": {}, "lp_ok": True}


def run_arm_A(ctx, us, prefilter, seed_root):
    g, p, chi, inS = ctx.g, ctx.p, ctx.chi, ctx.inS
    res = new_result()
    rng = random.Random(seed_root)
    dec = []
    reached = 0
    for idx, u in enumerate(us):
        decided = False
        if prefilter:
            ch = disc_character(u, chi, p, res["pre"])
            if ch != 1:
                dec.append(0)
                continue
            if g == 2:
                decided = True
        if not decided:
            if not splits_completely(u, p, res["dec"]):
                dec.append(0)
                continue
        reached += 1
        rts = roots_of(u, p, rng, res["roots"])
        if rts is None:
            res["cz_fail"] += 1
            dec.append(0)
            continue
        out = sum(1 for r in rts if not inS[r])
        if out <= 2:
            dec.append(1)
            res["rts"][idx] = rts
        else:
            dec.append(0)
    res["dec_bits"] = dec
    res["pass"] = reached
    return res


def _finish_accept(ctx, res, rng, idx, u, G, c):
    inS = ctx.inS
    rg = roots_of(G, ctx.p, rng, res["roots"]) if len(G) > 1 else []
    rc = roots_of(c, ctx.p, rng, res["roots"]) if len(c) > 1 else []
    if rg is None or rc is None:
        res["cz_fail"] += 1
        return False
    if not all(inS[r] for r in rg) or any(inS[r] for r in rc):
        res["lp_ok"] = False
    if sum(1 for r in rg + rc if not inS[r]) > 2:
        res["lp_ok"] = False
    res["rts"][idx] = rg + rc
    return True


def run_arm_B(ctx, us, prefilter, seed_root):
    g, p = ctx.g, ctx.p
    res = new_result()
    rng = random.Random(seed_root)
    dec = []
    survived = 0
    for idx, u in enumerate(us):
        if prefilter:
            if disc_character(u, ctx.chi, p, res["pre"]) != 1:
                dec.append(0)
                continue
            survived += 1
        R = vrem(ctx.FS, u, p, res["dec"])
        ok, G, c = post_S(u, R, g, ctx.chi, ctx.inS, p, res["dec"])
        if not ok:
            dec.append(0)
            continue
        dec.append(1 if _finish_accept(ctx, res, rng, idx, u, G, c) else 0)
    res["dec_bits"] = dec
    res["pass"] = survived if prefilter else None
    return res


def run_arm_C(ctx, us, m, prefilter, seed_root):
    g, p = ctx.g, ctx.p
    res = new_result()
    res["tprod"], res["trem"], res["post"] = Ops(), Ops(), Ops()
    rng = random.Random(seed_root)
    dec = [0] * len(us)
    if prefilter:
        L = []
        for idx, u in enumerate(us):
            if disc_character(u, ctx.chi, p, res["pre"]) == 1:
                L.append(idx)
        res["pass"] = len(L)
    else:
        L = list(range(len(us)))
    sizes = []
    for b0 in range(0, len(L), m):
        batch = L[b0:b0 + m]
        sizes.append(len(batch))
        levels = [[us[i] for i in batch]]
        while len(levels[-1]) > 1:
            cur = levels[-1]
            nxt = [poly_mul(cur[2 * j], cur[2 * j + 1], p, res["tprod"]) for j in range(len(cur) // 2)]
            if len(cur) % 2 == 1:
                nxt.append(cur[-1])
            levels.append(nxt)
        top = len(levels) - 1
        rems = [None] * len(levels)
        rems[top] = [vrem(ctx.FS, levels[top][0], p, res["trem"])]
        for lv in range(top - 1, -1, -1):
            cur = levels[lv]
            rl = []
            for j in range(len(cur)):
                parent_rem = rems[lv + 1][j // 2]
                if len(cur) % 2 == 1 and j == len(cur) - 1:
                    rl.append(parent_rem)
                else:
                    rl.append(vrem(parent_rem, cur[j], p, res["trem"]))
            rems[lv] = rl
        for k, idx in enumerate(batch):
            u = us[idx]
            ok, G, c = post_S(u, rems[0][k], g, ctx.chi, ctx.inS, p, res["post"])
            if not ok:
                continue
            if _finish_accept(ctx, res, rng, idx, u, G, c):
                dec[idx] = 1
    res["dec_bits"] = dec
    res["batch_sizes"] = sizes
    # dec = sum of the three sub-counters
    for f in ("mul", "inv", "add"):
        setattr(res["dec"], f, getattr(res["tprod"], f) + getattr(res["trem"], f) + getattr(res["post"], f))
    return res
