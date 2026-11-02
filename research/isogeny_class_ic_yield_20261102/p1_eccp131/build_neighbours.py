#!/usr/bin/env python3
"""P1 builder: the 27 Elkies neighbours of ECCp-131, verified, with a planted DLP.

Construction only (no cryptanalytic measurement).  PARI 2.17 through cypari2;
there is no gp binary and no Sage.  Usage:

    . $VENV/bin/activate
    python3 build_neighbours.py --out neighbours.json [--lmax 150] [--only 5,7]

For every prime l <= lmax with Phi_l(j, Y) having a root mod p (an Elkies
prime) and every Frobenius eigenvalue lam of E[l] over F_p:

  kernel polynomial  h_lam = gcd(psi_l(X), X^p - x_lam(X))  mod p
                     (Schoof: Frobenius acts on the eigenline as [lam]),
                     degree (l-1)/2, found without factoring psi_l;
  isogeny            phi = ellisogeny(E0, h_lam)  (Velu), codomain E' with
                     j' that must be a root of Phi_l(j, Y);
  checks             deg of the x-map (l over l-1), phi(P+Q) = phi(P)+phi(Q)
                     on random points, #E'(F_p) = n (SEA, ellcard), the image
                     of the generator has order n, and the planted DLP
                     Q = k P maps to phi(Q) = k phi(P).

Also writes the isomorphic-model control of E0 ((a u^4, b u^6)).
Everything random is derived from SHA-256(seed || label).
"""
import argparse, hashlib, importlib.util, json, os, sys, time
import cypari2

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", ".."))

P_HEX = "0x48e1d43f293469e33194c43186b3abc0b"
A_HEX = "0x41cb121ce2b31f608a76fc8f23d73cb66"
B_HEX = "0x2f74f717e8dec90991e5ea9b2ff03da58"
N_HEX = "0x48e1d43f293469e317f7ed728f6b8e6f1"
TRACE = 29529377007934231835
SEED = 20261102
EXPECTED_ELKIES = {5: 2, 7: 2, 13: 2, 17: 2, 29: 2, 37: 2, 61: 2, 67: 1, 73: 2,
                   83: 2, 89: 2, 107: 2, 131: 2, 149: 2}


def load_curve_id():
    spec = importlib.util.spec_from_file_location("curve_id", os.path.join(REPO, "scripts", "curve_id.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def h256(*parts):
    return hashlib.sha256("|".join(str(x) for x in parts).encode()).hexdigest()


class Ctx:
    def __init__(self):
        self.pari = cypari2.Pari()
        self.pari.allocatemem(6 * 10**9)
        self.p = int(P_HEX, 16)
        self.a = int(A_HEX, 16)
        self.b = int(B_HEX, 16)
        self.n = int(N_HEX, 16)
        self.cid = load_curve_id()

    def ell(self, a, b):
        return self.pari.ellinit([a, b], self.p)

    def point(self, E, label):
        """Deterministic point of E from the label (first x >= hash with a rational y)."""
        pari = self.pari
        x = int(h256(SEED, label), 16) % self.p
        while True:
            y = pari.ellordinate(E, x)
            if len(y):
                return pari.vector(2, [x, int(y[0].lift())]) if False else [x, int(pari.lift(y[0]))]
            x = (x + 1) % self.p


def split_kernel(pari, g, l, a, b, p, X, which):
    """g = K1*K2, two kernel polynomials sharing x_lam (mu = -lam).  Take an irreducible
    factor f of g, form K_f = prod_{k=1}^{(l-1)/2} (X - x([k]P_f)) over F_p[x]/f by x-only
    arithmetic (its coefficients lie in F_p exactly when P_f spans an eigenline), then
    K1 = K_f and K2 = g/K1.  `which` (0/1) takes the smaller / larger by coefficient
    list, so the two eigenvalue slots get different kernels (the slot is then ordinal)."""
    f0 = pari.subst(pari.factormod(g, p)[0][0].lift(), "x", "y") * pari.Mod(1, p)
    Y = pari("y")
    x1 = pari.Mod(Y, f0)
    F1 = x1**3 + a * x1 + b
    xs = [None, x1, (x1**4 - 2 * a * x1**2 - 8 * b * x1 + a * a) / (4 * F1)]
    for k in range(2, (l - 1) // 2):
        xk = xs[k]
        num = (xk * x1 - a) ** 2 - 4 * b * (xk + x1)
        xs.append(num / (xk - x1) ** 2 / xs[k - 1])
    K = pari(1)
    for k in range(1, (l - 1) // 2 + 1):
        K = K * (X - xs[k])
    cs = [pari.lift(c) for c in pari.Vec(K)]
    assert all(c == pari.polcoef(c, 0, "y") for c in cs), "factor not in an eigenline"
    K1 = pari.Polrev([pari.polcoef(c, 0, "y") for c in reversed(cs)])
    K1 = K1 / pari.pollead(K1)
    q, r = pari.divrem(g, K1)
    assert r == 0
    K2 = q / pari.pollead(q)
    pair = sorted([K1, K2], key=lambda z: [int(c.lift()) for c in pari.Vec(z)])
    return pair[which]


def to_pt(pari, P):
    return pari([int(P[0]), int(P[1])]) if P is not None else pari("[0]")


def hexs(v):
    return hex(int(v))


def pt_hex(P):
    if len(P) == 1:
        return "O"
    return [hexs(P[0].lift()), hexs(P[1].lift())]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(HERE, "neighbours.json"))
    ap.add_argument("--lmax", type=int, default=150)
    ap.add_argument("--only", default="", help="comma list of l (smoke / debugging)")
    args = ap.parse_args()
    C = Ctx()
    pari, p, a, b, n = C.pari, C.p, C.a, C.b, C.n
    only = {int(x) for x in args.only.split(",") if x}
    T0 = time.time()

    E0 = C.ell(a, b)
    assert int(pari.ellcard(E0)) == n and p + 1 - n == TRACE and pari.isprime(n)
    j0 = int(E0[12].lift())
    idinfo = C.cid.prime_id(p, a, b, n)
    ident0 = {"slug": idinfo["slug"], "icv1": idinfo["icv1"], "model_sha256": idinfo["model_sha256"]}
    X = pari("x")
    k_dlp = int(h256(SEED, "dlp-k"), 16) % (n - 2) + 1
    P0 = C.point(E0, "planted-P")
    Q0 = [int(pari.lift(c)) for c in pari.ellmul(E0, P0, k_dlp)]
    assert pari.ellmul(E0, P0, n) == pari("[0]")

    out = {
        "schema": "p1.neighbours/v1", "seed": SEED, "pari": str(pari.version()),
        "p": P_HEX, "n": N_HEX, "trace": TRACE,
        "E0": {"a": A_HEX, "b": B_HEX, "j": hexs(j0), **ident0, "role": "E0"},
        "planted_dlp": {"k": hexs(k_dlp), "P": [hexs(P0[0]), hexs(P0[1])], "Q": [hexs(Q0[0]), hexs(Q0[1])],
                        "rule": "P = first rational point at x >= SHA256(seed|planted-P); k = SHA256(seed|dlp-k) mod (n-2) + 1"},
        "neighbours": [], "controls": [], "elkies_roots_per_l": {}, "timing_s": {},
    }

    # ---- isomorphic-model control of E0 ------------------------------------
    u = int(h256(SEED, "iso-u"), 16) % (p - 2) + 2
    ai, bi = a * pow(u, 4, p) % p, b * pow(u, 6, p) % p
    Ei = C.ell(ai, bi)
    assert int(Ei[12].lift()) == j0 and int(pari.ellcard(Ei)) == n
    idi = C.cid.prime_id(p, ai, bi, n)
    out["controls"].append({
        "role": "iso_control", "a": hexs(ai), "b": hexs(bi), "u": hexs(u), "j": hexs(j0),
        "slug": idi["slug"], "icv1": idi["icv1"], "model_sha256": idi["model_sha256"],
        "map": "(x,y) -> (u^2 x, u^3 y)",
        "planted_Q_image": [hexs(Q0[0] * pow(u, 2, p) % p), hexs(Q0[1] * pow(u, 3, p) % p)],
        "check_on_curve": bool(pari.ellisoncurve(Ei, [Q0[0] * pow(u, 2, p) % p, Q0[1] * pow(u, 3, p) % p]))})

    # ---- Elkies primes ------------------------------------------------------
    primes = [int(q) for q in pari.primes([1, args.lmax])]
    jmod = pari.Mod(j0, p)
    roots_by_l = {}
    t0 = time.time()
    for l in primes:
        if l <= 3 or (only and l not in only):
            continue
        Phi = pari.polmodular(l)
        Pj = pari.subst(Phi, "x", jmod)
        r = pari.polrootsmod(Pj, p)
        rr = sorted(int(c.lift()) for c in r)
        if rr:
            roots_by_l[l] = rr
    out["timing_s"]["phi_roots_all_l"] = time.time() - t0
    out["elkies_roots_per_l"] = {str(l): [hexs(r) for r in v] for l, v in roots_by_l.items()}
    if not only:
        got = {l: len(v) for l, v in roots_by_l.items()}
        assert got == EXPECTED_ELKIES, (got, EXPECTED_ELKIES)

    # ---- kernel polynomials and isogenies ----------------------------------
    F = X**3 + a * X + b
    for l, roots in roots_by_l.items():
        tl = time.time()
        psi = pari.elldivpol(E0, l)
        Xp = pari.Mod(X, psi) ** p
        shared = time.time() - tl
        disc = pari(f"x^2-{TRACE}*x+{p}")
        lams = sorted(int(c.lift()) for c in pari.polrootsmod(disc, l))
        found = {}
        for lam in lams:
            tk = time.time()
            k = min(lam, l - lam)
            fm, f0, fp1 = (pari.elldivpol(E0, i) for i in (k - 1, k, k + 1))
            if k % 2 == 0:
                num, den = fm * fp1 * 4 * F, f0**2
            else:
                num, den = fm * fp1, 4 * F * f0**2
            xk = X - pari.Mod(num, psi) / pari.Mod(den, psi)
            g = pari.gcd(psi, (Xp - xk).lift())
            if int(pari.poldegree(g)) == (l - 1) // 2:
                h = g / pari.pollead(g)
            else:
                # mu = -lam: both eigenlines share x_lam, so g = K1*K2; split it.
                assert int(pari.poldegree(g)) == l - 1 and len(lams) == 2, (l, lam)
                h = split_kernel(pari, g, l, a, b, p, X, which=lams.index(lam))
                assert int(pari.poldegree(h)) == (l - 1) // 2
            t_kernel = time.time() - tk
            tv = time.time()
            I = pari.ellisogeny(E0, h)
            Ep = pari.ellinit(I[0], p)
            phi = I[1]
            t_isog = time.time() - tv
            jp = int(Ep[12].lift())
            assert jp in roots, (l, lam)
            ap_, bp_ = int(I[0][3].lift()), int(I[0][4].lift())
            assert all(int(c) == 0 for c in I[0][:3])
            coeffs = [int(c.lift()) for c in pari.Vec(h)]  # high -> low
            # verification --------------------------------------------------
            tc = time.time()
            # PARI's Velu map is [f, g, h]: x -> f/h^2, y -> g/h^3 with h the kernel polynomial.
            nume, dene = phi[0], phi[2] ** 2
            deg_ok = ((int(pari.poldegree(nume)), int(pari.poldegree(dene))) == (l, l - 1)
                      and phi[2] / pari.pollead(phi[2]) == h)
            hom_ok, oncurve_ok = True, True
            for i in range(6):
                A_ = to_pt(pari, C.point(E0, f"hom-{l}-{lam}-{i}-a"))
                B_ = to_pt(pari, C.point(E0, f"hom-{l}-{lam}-{i}-b"))
                S_ = pari.elladd(E0, A_, B_)
                fa, fb = pari.ellisogenyapply(phi, A_), pari.ellisogenyapply(phi, B_)
                fs = pari.ellisogenyapply(phi, S_)
                oncurve_ok &= bool(pari.ellisoncurve(Ep, fa))
                hom_ok &= (pari.elladd(Ep, fa, fb) == fs)
            card_ok = int(pari.ellcard(Ep)) == n
            fP = pari.ellisogenyapply(phi, to_pt(pari, P0))
            fQ = pari.ellisogenyapply(phi, to_pt(pari, Q0))
            order_ok = (fP != pari("[0]")) and pari.ellmul(Ep, fP, n) == pari("[0]")
            dlp_ok = (pari.ellmul(Ep, fP, k_dlp) == fQ)
            t_check = time.time() - tc
            idn = C.cid.prime_id(p, ap_, bp_, n)
            chk = {"x_map_degrees_f_over_h2": [int(pari.poldegree(nume)), int(pari.poldegree(dene))],
                   "degree_ok": deg_ok, "homomorphism_6_pairs": bool(hom_ok), "images_on_curve": bool(oncurve_ok),
                   "ellcard_equals_n": bool(card_ok), "image_generator_order_n": bool(order_ok),
                   "planted_dlp_transports": bool(dlp_ok), "j_prime_is_root_of_Phi_l": True,
                   "j_prime_not_j0": jp != j0}
            ok = all(chk.values())
            rec = {
                "role": "neighbour", "l": l, "lambda": lam, "kernel_degree": (l - 1) // 2,
                "a": hexs(ap_), "b": hexs(bp_), "j": hexs(jp),
                "slug": idn["slug"], "icv1": idn["icv1"], "model_sha256": idn["model_sha256"],
                "id_note": "ICV1-style slug from scripts/curve_id.py prime_id; not yet in docs/curves/registry.json",
                "kernel_poly_high_to_low": [hexs(c) for c in coeffs],
                "kernel_poly_sha256": h256("kernel", l, ",".join(str(c) for c in coeffs)),
                "phi_P": pt_hex(fP), "phi_Q": pt_hex(fQ), "checks": chk, "all_checks_pass": ok,
                "timing_s": {"kernel_gcd": t_kernel, "shared_psi_Xp_per_l": shared, "velu_isogeny": t_isog,
                             "verify": t_check, "build_total": t_kernel + t_isog + shared / len(lams)},
            }
            found[lam] = rec
            out["neighbours"].append(rec)
            print(f"l={l:3d} lam={lam:3d} {idn['slug']} ok={ok} kernel={t_kernel:.2f}s isog={t_isog:.2f}s "
                  f"verify={t_check:.2f}s shared={shared:.2f}s", flush=True)
        js = [r["j"] for r in found.values()]
        assert len(set(js)) == len(js) == len(roots), (l, js)
        assert sorted(int(x, 16) for x in js) == roots, l
    out["neighbours"].sort(key=lambda r: (r["l"], r["lambda"]))
    out["summary"] = {"count": len(out["neighbours"]), "all_checks_pass": all(r["all_checks_pass"] for r in out["neighbours"]),
                      "build_total_s_sum": sum(r["timing_s"]["build_total"] for r in out["neighbours"]),
                      "wall_s": time.time() - T0}
    with open(args.out, "w") as f:
        json.dump(out, f, indent=1)
    print("wrote", args.out, out["summary"])
    if not out["summary"]["all_checks_pass"] or (not only and out["summary"]["count"] != 27):
        sys.exit(1)


if __name__ == "__main__":
    main()
