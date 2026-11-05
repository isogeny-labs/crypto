# ecdb curve generators (SageMath). Writes JSONL records (schema
# ecdb.generated/v1) that `ecdb import generated` verifies and loads.
# Sage computes; the Rust importer re-checks every record it can (ICV1 slug,
# nonsingularity, Hasse bound, P on the curve and [N]P = O, [r]G = O, prime
# r by Miller-Rabin, genus-2 point counts for small p) and refuses a record
# that fails. Nothing here is a measurement.
#
#   sage generate.sage SET out=DIR [workers=W worker=I] [units=u1,u2]
#
# SET: census-fp | census-f2m | crypto-prime | pairing | genus2 | isogenies
# (isogenies reads the census-fp, crypto-prime and pairing output: in=DIR)
# Every set is split into units (one prime, one bit size, one family); a unit
# writes DIR/<set>/<unit>.jsonl and then <unit>.done, so a rerun resumes.
# Each unit seeds Sage's RNG from its name: reruns reproduce the same curves.

import hashlib, json, os, sys, time

SCHEMA = "ecdb.generated/v1"
import sage.version
SAGE_VERSION = sage.version.version

def arg(name, default=None):
    # key=value arguments: sage's own option parser consumes --flags.
    key = name.lstrip("-") + "="
    for a in sys.argv[2:]:
        if a.startswith(key):
            return a[len(key):]
    return default

def _plain(o):
    # Sage integers (the preparser makes every literal one) become JSON numbers;
    # anything else exotic is written as its string form.
    if o in ZZ:
        return int(o)
    return str(o)

def seed_for(unit):
    set_random_seed(int(hashlib.sha256(unit.encode()).hexdigest()[:15], 16))

def run_units(set_name, units, fn, out):
    workers, me = int(arg("--workers", 1)), int(arg("--worker", 0))
    d = os.path.join(out, set_name)
    os.makedirs(d, exist_ok=True)
    for i, unit in enumerate(units):
        if i % workers != me:
            continue
        done = os.path.join(d, unit + ".done")
        if os.path.exists(done):
            continue
        seed_for(set_name + "/" + unit)
        t0 = time.time()
        path = os.path.join(d, unit + ".jsonl")
        n = 0
        with open(path + ".part", "w") as fh:
            for rec in fn(unit):
                rec["schema"] = SCHEMA
                rec["set"] = set_name
                rec["unit"] = unit
                rec["sage"] = SAGE_VERSION
                fh.write(json.dumps(rec, sort_keys=True, separators=(",", ":"), default=_plain) + "\n")
                n += 1
        os.replace(path + ".part", path)
        open(done, "w").write(json.dumps({"records": int(n), "seconds": float(time.time() - t0)}) + "\n")
        print(f"{set_name}/{unit}: {n} records in {time.time() - t0:.1f}s", flush=True)

# ------------------------------------------------------------- helpers

_H = {}

def class_number(D):
    if D not in _H:
        _H[D] = int(pari(D).qfbclassno())
    return _H[D]

def cm_data(t, q, factor_limit=10**7, class_number_limit=10**12):
    """Frobenius discriminant t^2 - 4q, its fundamental part D_K, the conductor f of
    Z[pi], and h(D_K); fields are null when the factorization is not complete."""
    d = ZZ(t)**2 - 4 * ZZ(q)
    out = {"disc": str(d), "D_K": None, "f": None, "h_K": None}
    if d == 0:
        return out
    fac = factor(d, limit=factor_limit)
    if len(fac):
        last = fac[-1][0]
        if last > factor_limit and not is_pseudoprime(last):
            return out  # an unfactored composite remains
    D_K = fundamental_discriminant(d)
    quo = d // D_K
    f = quo.isqrt()
    if f**2 != quo:
        return out
    out["D_K"], out["f"] = str(D_K), str(f)
    if abs(D_K) <= class_number_limit:
        out["h_K"] = class_number(D_K)
    return out

def embedding(q, r, kmax=100):
    """Smallest k <= kmax with r | q^k - 1, else (None, kmax + 1)."""
    if r <= 2:
        return None, None
    x = Mod(q, r)
    y = x
    for k in range(1, kmax + 1):
        if y == 1:
            return k, None
        y *= x
    return None, kmax + 1

def pt(P):
    return [str(P[0]), str(P[1])]

def bin_int(x):
    return ZZ(x.polynomial().change_ring(ZZ)(2)) if x != 0 else ZZ(0)

def bin_hex(x):
    return hex(bin_int(x))

def binary_modulus(m):
    """ICV1's rule: the least x^m + low, low odd of weight <= 4, irreducible over GF(2)."""
    R = PolynomialRing(GF(2), "x")
    X = R.gen()
    if m == 1:
        return X + 1
    cands = [1]
    for i in range(1, m):
        cands.append(1 | (1 << i))
        for j in range(i + 1, m):
            cands.append(1 | (1 << i) | (1 << j))
            for k in range(j + 1, m):
                cands.append(1 | (1 << i) | (1 << j) | (1 << k))
    for low in sorted(cands):
        f = X**m + sum(X**i for i in range(m) if (low >> i) & 1)
        if f.is_irreducible():
            return f
    raise ValueError(m)

def prime_record(E, family, params=None, subgroup=None, with_cm=True, kmax=100, order=None):
    F = E.base_field()
    p = F.characteristic()
    a, b = E.a4(), E.a6()
    N = ZZ(order) if order is not None else E.order()
    t = p + 1 - N
    P = E.random_point()
    while P.is_zero():
        P = E.random_point()
    rec = {"genus": 1, "field": {"kind": "prime", "p": str(p)}, "a": str(a), "b": str(b), "order": str(N),
           "point": pt(P), "twist_order": str(2 * p + 2 - N), "family": family, "params": params or {}}
    if subgroup is not None:
        r, h = subgroup
        G = h * E.random_point()
        while G.is_zero():
            G = h * E.random_point()
        rec.update({"generator": pt(G), "subgroup_order": str(r), "cofactor": str(h),
                    "subgroup_prime": bool(is_prime(r))})
        k, lb = embedding(p, r, kmax)
        rec["embedding_degree"], rec["embedding_lower_bound"] = k, lb
    if with_cm:
        rec["cm"] = cm_data(t, p)
    return rec

def canonical_short(a, b, p, sq2, sq3):
    """Least (a w^2, b w^3) over the squares w = u^2: one model per F_p-isomorphism class."""
    best = None
    for w2, w3 in zip(sq2, sq3):
        c = (a * w2 % p, b * w3 % p)
        if best is None or c < best:
            best = c
    return best

# ------------------------------------------------------------- census over F_p, 5 <= p < 2^10

def census_fp(unit):
    p = ZZ(unit[1:])
    F = GF(p)
    squares = sorted({(u * u) % p for u in range(1, p)})
    sq2 = [w * w % p for w in squares]
    sq3 = [w * w * w % p for w in squares]
    classes = set()
    for j in F:
        if j == 0:
            reps = [(0, int(b)) for b in F if b != 0]
        elif j == 1728:
            reps = [(int(a), 0) for a in F if a != 0]
        else:
            E = EllipticCurve_from_j(j).short_weierstrass_model()
            E2 = E.quadratic_twist()
            reps = [(int(E.a4()), int(E.a6())), (int(E2.short_weierstrass_model().a4()), int(E2.short_weierstrass_model().a6()))]
        for a, b in reps:
            classes.add(canonical_short(a, b, p, sq2, sq3))
    for a, b in sorted(classes):
        E = EllipticCurve(F, [a, b])
        yield prime_record(E, "census-fp", {"isomorphism_class_representative": "least (a u^4, b u^6)"},
                           subgroup=None)

def census_fp_units():
    return ["p%04d" % p for p in prime_range(5, 1024)]

# ------------------------------------------------------------- census over GF(2^m), m <= 10

def census_f2m(unit):
    m = int(unit[1:])
    f = binary_modulus(m)
    F = GF(2**m, "z", modulus=f)
    mod_hex = hex(ZZ(f.change_ring(ZZ)(2)))
    els = sorted(F, key=bin_int)
    a1 = next(x for x in els if x.trace() == 1)
    for a in [F(0), a1]:
        for b in els:
            if b == 0:
                continue
            E = EllipticCurve(F, [1, a, 0, 0, b])
            N = E.order()
            P = E.random_point()
            while P.is_zero():
                P = E.random_point()
            t = 2**m + 1 - N
            yield {"genus": 1, "field": {"kind": "binary", "m": m, "modulus": mod_hex},
                   "a": bin_hex(a), "b": bin_hex(b), "order": str(N),
                   "point": [bin_hex(P[0]), bin_hex(P[1])], "twist_order": str(2 * 2**m + 2 - N),
                   "family": "census-f2m", "params": {"a_class": "0" if a == 0 else "trace-1 representative"},
                   "cm": cm_data(t, 2**m)}

def census_f2m_units():
    return ["m%02d" % m for m in range(1, 11)]

# ------------------------------------------------------------- cryptographic-size prime curves

CRYPTO_BITS = [32, 48, 64, 80, 96, 112, 128, 160, 192, 224, 256]
CM_D = [-3, -4, -7, -8, -11, -19, -43, -67, -163]

def per_size(bits):
    return 60 if bits <= 128 else 20

def prime_order_curve(p):
    F = GF(p)
    while True:
        a, b = F.random_element(), F.random_element()
        if 4 * a**3 + 27 * b**2 == 0:
            continue
        E = EllipticCurve(F, [a, b])
        n = ZZ(pari(E).ellsea(1))   # early abort: 0 when a small factor divides #E
        if n != 0 and n.is_prime():
            return E, n

def crypto_prime(unit):
    kind, bits = unit.split("-")
    bits = int(bits)
    if kind == "rand":
        for i in range(per_size(bits)):
            p = random_prime(2**bits - 1, lbound=2**(bits - 1))
            E, n = prime_order_curve(p)
            rec = prime_record(E, "crypto-prime", {"bits": bits, "index": i}, subgroup=(n, 1), order=n)
            yield rec
            T = E.quadratic_twist().short_weierstrass_model()
            yield prime_record(T, "crypto-prime-twist", {"bits": bits, "index": i, "twist_of_order": str(n)},
                               subgroup=None, order=2 * p + 2 - n)
    else:  # CM curves with class number one discriminants
        for D in CM_D:
            found = 0
            tries = 0
            while found < 2 and tries < 20000:
                tries += 1
                # 4p = t^2 + |D| y^2 with p of the target size
                y = ZZ.random_element(2**(bits // 2 - 4), 2**(bits // 2 - 1))
                t = ZZ.random_element(2**(bits // 2 - 2), 2**(bits // 2))
                if (t**2 + abs(D) * y**2) % 4:
                    continue
                p = (t**2 + abs(D) * y**2) // 4
                if p.nbits() != bits or not p.is_prime():
                    continue
                j = hilbert_class_polynomial(D).change_ring(GF(p)).roots()[0][0]
                E = EllipticCurve_from_j(j).short_weierstrass_model()
                for T in (E.twists() if j in (0, 1728) else [E, E.quadratic_twist()]):
                    T = T.short_weierstrass_model()
                    N = T.order()
                    h = prod(q**N.valuation(q) for q in prime_range(2, 1000))
                    r = N // h
                    sub_ = (r, h) if r > 1 and r.is_prime() else None
                    yield prime_record(T, "crypto-cm", {"bits": bits, "D": D}, subgroup=sub_, order=N)
                found += 1

def crypto_prime_units():
    return ["rand-%03d" % b for b in CRYPTO_BITS] + ["cm-%03d" % b for b in CRYPTO_BITS]

# ------------------------------------------------------------- pairing-friendly families

def find_cm_curve(p, n, D):
    """y^2 = x^3 + b (D=-3) or x^3 + a x (D=-4) over F_p with n * P = O."""
    F = GF(p)
    for c in range(1, 10**5):
        E = EllipticCurve(F, [0, c]) if D == -3 else EllipticCurve(F, [c, 0])
        if all((n * E.random_point()).is_zero() for _ in range(3)):
            return E
    return None

FAMILIES = {
    # name: (k, D, p(u), r(u), t(u), u-condition)
    "bn": (12, -3, lambda u: 36*u**4 + 36*u**3 + 24*u**2 + 6*u + 1, lambda u: 36*u**4 + 36*u**3 + 18*u**2 + 6*u + 1,
           lambda u: 6*u**2 + 1, lambda u: True),
    "bls12": (12, -3, lambda u: (u - 1)**2 * (u**4 - u**2 + 1) / 3 + u, lambda u: u**4 - u**2 + 1,
              lambda u: u + 1, lambda u: u % 3 == 1),
    "bls24": (24, -3, lambda u: (u - 1)**2 * (u**8 - u**4 + 1) / 3 + u, lambda u: u**8 - u**4 + 1,
              lambda u: u + 1, lambda u: u % 3 == 1),
    "kss16": (16, -4, lambda u: (u**10 + 2*u**9 + 5*u**8 + 48*u**6 + 152*u**5 + 240*u**4 + 625*u**2 + 2398*u + 3125) / 980,
              lambda u: (u**8 + 48*u**4 + 625) / 61250, lambda u: (2*u**5 + 41*u + 35) / 35,
              lambda u: u % 70 in (25, 45)),
    "kss18": (18, -3, lambda u: (u**8 + 5*u**7 + 7*u**6 + 37*u**5 + 188*u**4 + 259*u**3 + 343*u**2 + 1763*u + 2401) / 21,
              lambda u: (u**6 + 37*u**3 + 343) / 343, lambda u: (u**4 + 16*u + 7) / 7, lambda u: u % 42 == 14),
}
PAIRING_BITS = [160, 192, 224, 256, 320, 384, 448, 512]

def pairing(unit):
    fam, bits = unit.split("-")
    bits = int(bits)
    k, D, P, R, T, ok = FAMILIES[fam]
    deg, lead = {"bn": (4, 36), "bls12": (6, 1/3), "bls24": (10, 1/3), "kss16": (10, 1/980), "kss18": (8, 1/21)}[fam]
    found = 0
    u = ZZ(floor((RR(2)**(bits - 1) / lead)**(1 / deg))) - 2
    tries = 0
    while found < 3 and tries < 2 * 10**6:
        tries += 1
        u += 1
        for uu in (u, -u):
            if not ok(uu):
                continue
            p, r, t = QQ(P(uu)), QQ(R(uu)), QQ(T(uu))
            if not (p.is_integer() and r.is_integer() and t.is_integer()):
                continue
            p, r, t = ZZ(p), ZZ(r), ZZ(t)
            if p.nbits() > bits + 1 and uu > 0:
                tries = 10**9
            if p.nbits() != bits or not p.is_prime() or not r.is_prime():
                continue
            n = p + 1 - t
            if n % r:
                continue
            E = find_cm_curve(p, n, D)
            if E is None:
                continue
            yield prime_record(E, "pairing-" + fam, {"u": str(uu), "k": k, "D": D, "bits": bits},
                               subgroup=(r, n // r), with_cm=False, kmax=k, order=n)
            found += 1
            if found >= 3:
                break

def pairing_units():
    return ["%s-%03d" % (f, b) for f in FAMILIES for b in PAIRING_BITS]

# ------------------------------------------------------------- genus-2 Jacobians

G2_BITS = [5, 8, 11, 14, 17, 20, 24, 28, 32]

def genus2(unit):
    bits = int(unit[1:])
    count = 300 if bits <= 20 else 120
    for i in range(count):
        p = random_prime(2**bits - 1, lbound=2**(bits - 1))
        F = GF(p)
        R = PolynomialRing(F, "x")
        while True:
            f = R([F.random_element() for _ in range(5)] + [1])  # monic quintic: one point at infinity
            if f.is_squarefree() and f.degree() == 5:
                break
        H = HyperellipticCurve(f)
        cp = H.frobenius_polynomial()      # x^4 - s1 x^3 + s2 x^2 - p s1 x + p^2
        c = [ZZ(x) for x in cp.list()]
        L = [c[4 - i] for i in range(5)]   # L(T) = T^4 cp(1/T): 1, -s1, s2, -p s1, p^2
        jac = sum(L)
        npts = p + 1 + L[1]
        ic = [str(x) for x in H.igusa_clebsch_invariants()]
        yield {"genus": 2, "field": {"kind": "prime", "p": str(p)}, "f": [str(x) for x in f.list()], "h": [],
               "l_poly": [str(x) for x in L], "jacobian_order": str(jac), "curve_points": str(npts),
               "igusa_clebsch": ic, "family": "genus2-random", "params": {"bits": bits, "index": i}}

def genus2_units():
    return ["b%02d" % b for b in G2_BITS]

# ------------------------------------------------------------- isogeny graphs

ISOGENY_ELLS = (2, 3, 5)

def canonical_short_w(a, b, p, squares):
    """canonical_short, also returning the square w with (a w^2, b w^3) = the representative."""
    best, bw = None, None
    for w in squares:
        c = (a * w * w % p, b * w * w * w % p)
        if best is None or c < best:
            best, bw = c, w
    return best, bw

def isogeny_records(p, a, b, N, canonical, neighbour_of=None):
    """Every F_p-rational l-isogeny (l = 2, 3, 5) out of y^2 = x^3 + a x + b. Sage's
    codomain is Velu's; the stored target is the census representative of its class
    (canonical) or the Velu codomain itself, reached by (a w^2, b w^3)."""
    F = GF(p)
    E = EllipticCurve(F, [a, b])
    squares = sorted({(u * u) % p for u in range(1, p)}) if canonical else None
    for l in ISOGENY_ELLS:
        for psi in E.isogenies_prime_degree(l):
            # isogenies_prime_degree may compose Velu's map with an isomorphism
            # (e.g. at j = 1728); rebuild the plain Velu isogeny from its kernel so the
            # stored codomain is the one the importer recomputes.
            phi = E.isogeny(psi.kernel_polynomial())
            C = phi.codomain()
            if C.a1() or C.a2() or C.a3():
                C = C.short_weierstrass_model()
            ca, cb = int(C.a4()), int(C.a6())
            if canonical:
                (ta, tb), w = canonical_short_w(ca, cb, p, squares)
            else:
                (ta, tb), w = (ca, cb), 1
            if neighbour_of is not None:
                rec = prime_record(EllipticCurve(F, [ta, tb]), "isogeny-neighbour",
                                   {"source_family": neighbour_of["family"], "ell": l,
                                    "source": {"a": str(a), "b": str(b)}}, order=N, with_cm=False)
                rec["cm"] = neighbour_of.get("cm")   # same trace, same Frobenius discriminant
                yield rec
            yield {"record": "isogeny", "genus": 1, "field": {"kind": "prime", "p": str(p)}, "order": str(N),
                   "ell": l, "source": {"a": str(a), "b": str(b)},
                   "kernel": [str(c) for c in phi.kernel_polynomial().list()],
                   "codomain": {"a": str(ca), "b": str(cb)}, "w": str(w), "target": {"a": str(ta), "b": str(tb)}}

def isogenies(unit):
    src = arg("--in", arg("--out", "out"))
    kind, rest = unit.split("-", 1)
    if kind == "census":
        path = os.path.join(src, "census-fp", rest + ".jsonl")
        canonical, neighbours = True, False
    else:
        sset, sunit = rest.split(".", 1)
        path = os.path.join(src, sset, sunit + ".jsonl")
        canonical, neighbours = False, True
    for line in open(path):
        r = json.loads(line)
        if r.get("genus") != 1 or r["field"]["kind"] != "prime":
            continue
        p = ZZ(r["field"]["p"])
        if p.nbits() > 512:
            continue
        for rec in isogeny_records(p, ZZ(r["a"]), ZZ(r["b"]), ZZ(r["order"]), canonical,
                                   neighbour_of=r if neighbours else None):
            yield rec

def isogeny_units():
    src = arg("--in", arg("--out", "out"))
    units = ["census-" + u for u in census_fp_units()]
    for sset in ("crypto-prime", "pairing"):
        d = os.path.join(src, sset)
        if os.path.isdir(d):
            units += ["%s-%s.%s" % ("from", sset, f[:-5]) for f in sorted(os.listdir(d)) if f.endswith(".done")]
    return units

# ------------------------------------------------------------- main

SETS = {
    "census-fp": (census_fp_units, census_fp),
    "census-f2m": (census_f2m_units, census_f2m),
    "crypto-prime": (crypto_prime_units, crypto_prime),
    "pairing": (pairing_units, pairing),
    "genus2": (genus2_units, genus2),
    "isogenies": (isogeny_units, isogenies),
}

which = sys.argv[1]
out = arg("--out", "out")
units_fn, fn = SETS[which]
only = arg("--units")
run_units(which, units_fn() if not only else only.split(","), fn, out)
