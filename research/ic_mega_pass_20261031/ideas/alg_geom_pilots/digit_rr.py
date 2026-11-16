"""Pilot for MP-alg_geom-6: prime-field digit presentation of the m=2 decomposition in the
Riemann-Roch/collinearity form (P1, P2, -R collinear; y_i kept as unknowns) vs the Semaev S3
digit presentation.  Base-2 digits b_{ij}^2 = b_{ij}, x_i = sum_j b_ij 2^j (+ offset so cosets are disjoint).
Records msolve max F4 step degree for s = 2..S.
"""
import random, subprocess, re, os, sys, json, time
import cypari2
P = cypari2.Pari()
TMP = os.environ.get("TMPDIR", "/tmp")
HERE = os.path.dirname(os.path.abspath(__file__))

def legendre(a, p): return pow(a % p, (p - 1) // 2, p)

def prime_order_curve(p, rng):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4 * a ** 3 + 27 * b ** 2) % p == 0: continue
        E = P.ellinit([a, b], p)
        N = int(P.ellcard(E))
        if P.isprime(N): return a, b, E, N

def lift(a, b, p, x):
    v = (x ** 3 + a * x + b) % p
    if v == 0: return 0
    if legendre(v, p) != 1: return None
    return int(P.sqrt(P.Mod(v, p)).lift()) if hasattr(P.sqrt(P.Mod(v, p)), 'lift') else int(P.lift(P.sqrt(P.Mod(v, p))))

def run_msolve(path, out, timeout=300):
    cmd = ["msolve", "-v", "2", "-g", "2", "-f", path, "-o", out]
    t0 = time.time()
    try:
        pr = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout); txt = pr.stdout + pr.stderr; st = "ok"
    except subprocess.TimeoutExpired as e:
        txt = ""; st = "timeout"
    degs = [int(m.group(1)) for m in (re.match(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)", l) for l in txt.splitlines()) if m]
    gb = open(out).read() if st == "ok" and os.path.exists(out) else ""
    nb = len([l for l in gb.splitlines() if l.strip() and not l.startswith('#')])
    return {"status": st, "time": round(time.time() - t0, 2), "maxdeg": max(degs) if degs else None, "gb_len": nb}

def build(p, a, b, s, R, offs, pres):
    """pres in {'rr','semaev'}; two summands with digit cosets x_i = offs[i] + sum_j b_ij 2^j."""
    names = []
    xs = []
    for i in range(2):
        terms = []
        for j in range(s):
            names.append(f"b{i}_{j}"); terms.append(f"{2**j}*b{i}_{j}")
        xs.append(f"({offs[i]}+" + "+".join(terms) + ")")
    polys = []
    xR, yR = R
    if pres == "rr":
        names += ["y0", "y1"]
        for i in range(2):
            polys.append(f"y{i}^2-({xs[i]})^3-{a}*({xs[i]})-{b}")
        # collinear(P1,P2,-R): x1(y2-(-yR)) + x2((-yR)-y1) + xR(y1-y2)
        polys.append(f"({xs[0]})*(y1+{yR})+({xs[1]})*(-{yR}-y0)+{xR}*(y0-y1)")
    else:
        # S3(x1,x2,xR) char != 2 with curve y^2=x^3+ax+b:
        # (x1-x2)^2 x3^2 - 2((x1+x2)(x1x2+a)+2b)x3 + (x1x2-a)^2 - 4b(x1+x2)
        X1, X2, X3 = xs[0], xs[1], str(xR)
        polys.append(f"({X1}-{X2})^2*{X3}^2-2*(({X1}+{X2})*({X1}*{X2}+{a})+2*{b})*{X3}+({X1}*{X2}-{a})^2-4*{b}*({X1}+{X2})")
    for v in names:
        if v.startswith("b"):
            polys.append(f"{v}^2-{v}")
    # msolve's parser needs expanded polynomials: expand with sympy over GF(p)
    import sympy as sp
    syms = {n: sp.Symbol(n) for n in names}
    out = []
    for q in polys:
        e = sp.Poly(sp.expand(sp.sympify(q.replace("^", "**"), locals=syms)), *[syms[n] for n in names], modulus=p)
        terms = []
        for mono, c in e.terms():
            c = int(c) % p
            if c == 0: continue
            m = "*".join(f"{names[i]}^{k}" for i, k in enumerate(mono) if k)
            terms.append(f"{c}" + (f"*{m}" if m else ""))
        out.append("+".join(terms) if terms else "0")
    return names, out

def main():
    out = []
    rng = random.Random(11)
    primes = [4099, 65537, 1048583]
    for p in primes:
        a, b, E, N = prime_order_curve(p, rng)
        for s in range(2, 8):
            offs = [1 << (s + 1), 1 << (s + 3)]  # disjoint digit cosets, nonzero
            for want in ("unsat", "sat"):
                # choose R
                if want == "sat":
                    while True:
                        x1 = offs[0] + rng.randrange(1 << s); x2 = offs[1] + rng.randrange(1 << s)
                        y1 = lift(a, b, p, x1); y2 = lift(a, b, p, x2)
                        if y1 is None or y2 is None: continue
                        Rp = P.elladd(E, [x1, y1], [x2, y2])
                        if Rp == [0]: continue
                        R = (int(Rp[0]), int(Rp[1])); break
                else:
                    # random point, check no decomposition by brute force
                    while True:
                        x = rng.randrange(p); y = lift(a, b, p, x)
                        if y is None or y == 0: continue
                        R = (x, y)
                        ok = True
                        for m1 in range(1 << s):
                            x1 = offs[0] + m1; y1 = lift(a, b, p, x1)
                            if y1 is None: continue
                            for sg in (1, -1):
                                Q = P.ellsub(E, [R[0], R[1]], [x1, sg * y1 % p])
                                if Q != [0] and (int(Q[0]) - offs[1]) in range(1 << s):
                                    ok = False; break
                            if not ok: break
                        if ok: break
                row = {"p": p, "s": s, "want": want}
                for pres in ("rr", "semaev"):
                    names, polys = build(p, a, b, s, R, offs, pres)
                    path = os.path.join(TMP, f"digit_{pres}_{p}_{s}_{want}.ms"); o = os.path.join(TMP, f"digit_{pres}_{p}_{s}_{want}.out")
                    with open(path, "w") as f:
                        f.write(",".join(names) + "\n" + str(p) + "\n" + ",\n".join(polys) + "\n")
                    r = run_msolve(path, o, timeout=int(os.environ.get("PILOT_TIMEOUT", "240")))
                    r["nvars"] = len(names); row[pres] = r
                print(json.dumps(row), flush=True); out.append(row)
                json.dump(out, open(os.path.join(HERE, "RESULTS_digit_rr.json"), "w"), indent=1)

if __name__ == "__main__":
    main()
