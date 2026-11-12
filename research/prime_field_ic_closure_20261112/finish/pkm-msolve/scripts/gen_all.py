#!/usr/bin/env python3
"""Generate every system named in PRESET.txt section 4 into OUT/systems/.

Usage: gen_all.py OUTDIR
"""
import hashlib
import json
import os
import random
import shutil
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pkmgen as G  # noqa: E402

BASE_SEED = 0x504B4D53  # "PKMS"
REPO_SYS = "/home/user/crypto/research/ic_next10_20261014/03-pkm-degree/systems"
C0 = ["pkm_kummer_p2013265921_m4_t2_x1102336260", "pkm_kummer_p2013265921_m4_t2_x92234565",
      "pkm_kummer_p2013265921_m4_t3_x1710525890", "pkm_kummer_p2013265921_m4_t3_x259303215"]
P1 = 2013265921
P1_CURVE = (1373482479, 486145570)


def seed_of(*parts):
    h = hashlib.sha256(("%d|" % BASE_SEED + "|".join(map(str, parts))).encode()).hexdigest()
    return int(h[:16], 16)


class PrimeSetup:
    def __init__(self, p, curve=None, tmax=4):
        self.p = p
        rng = random.Random(seed_of("curve", p))
        if curve is None:
            A, B, n = G.prime_order_curve(p, rng)
        else:
            A, B = curve
            assert (4 * A ** 3 + 27 * B ** 2) % p != 0
            n = None
        self.A, self.B, self.n = A, B, n
        self.g = G.choose_g(p, A, B, random.Random(seed_of("g", p)))
        self.z = G.root_of_unity(p, tmax, random.Random(seed_of("zeta", p)))
        self.tmax = tmax

    def V(self, t):
        zt = pow(self.z, 1 << (self.tmax - t), self.p)  # exact order 2^t, nested V_t in V_{t+1}
        return [self.g * pow(zt, k, self.p) % self.p for k in range(1 << t)]


def random_target(S, rng):
    p, A, B = S.p, S.A, S.B
    while True:
        x = rng.randrange(p)
        f = (x ** 3 + A * x + B) % p
        if f and G.legendre(f, p) == 1:
            return x, G.sqrt_mod(f, p)


def planted_target(S, m, t, rng):
    p, A, B = S.p, S.A, S.B
    F = G.Fp2(p)
    V = S.V(t)
    onE = [v for v in V if G.legendre(v ** 3 + A * v + B, p) == 1]
    assert onE, "no E-abscissa in V"
    for _ in range(10000):
        xs = [rng.choice(onE) for _ in range(m)]
        pts = []
        for x in xs:
            y = G.sqrt_mod((x ** 3 + A * x + B) % p, p)
            if rng.randrange(2):
                y = (-y) % p
            pts.append(((x, 0), (y, 0)))
        partial, Ssum, ok = [], None, True
        for P in pts:
            Ssum = G.ec_add(F, A, Ssum, P)
            partial.append(Ssum)
        if any(q is None for q in partial[1:]):
            continue
        xR, yR = Ssum[0][0], Ssum[1][0]
        # planted point
        sol = {}
        for i, x in enumerate(xs):
            yv = x * G.inv(S.g, p) % p
            for j in range(t):
                sol[f"y{i}_{j}"] = yv
                yv = yv * yv % p
        for k in range(m - 2):
            sol[f"u{k}"] = partial[k + 1][0][0]
        return xR, yR, sol, xs
    raise RuntimeError("planted failed")


def write_system(outdir, name, meta, S, m, t, xR):
    names, eqs, text = G.build_system(S.p, S.A, S.B, S.g, m, t, xR)
    path = os.path.join(outdir, name + ".ms")
    with open(path, "w") as fh:
        fh.write(text)
    meta.update(name=name, p=S.p, A=S.A, B=S.B, n_E=S.n, g=S.g, zeta_order=1 << t, V=S.V(t), m=m, t=t,
                N=m * t, nvars=len(names), neqs=len(eqs), xR=xR,
                sha256=hashlib.sha256(text.encode()).hexdigest())
    if "planted" in meta:
        # check the planted point solves every generator
        point = [(meta["planted"][nm], 0) for nm in names]
        F = G.Fp2(S.p)
        import msparse as MP
        vals = [MP.eval_poly(e, point, F) for e in eqs]
        meta["planted_ok_gen"] = all(v == (0, 0) for v in vals)
        assert meta["planted_ok_gen"], name
    with open(os.path.join(outdir, name + ".json"), "w") as fh:
        json.dump(meta, fh)
    return meta


def main():
    out = os.path.join(sys.argv[1], "systems")
    os.makedirs(out, exist_ok=True)
    index = []
    # C0: repo systems, copied verbatim
    for nm in C0:
        src = os.path.join(REPO_SYS, nm + ".ms")
        dst = os.path.join(out, "C0_" + nm + ".ms")
        shutil.copyfile(src, dst)
        h = hashlib.sha256(open(dst, "rb").read()).hexdigest()
        m = int(nm.split("_m")[1].split("_")[0])
        t = int(nm.split("_t")[1].split("_")[0])
        meta = dict(arm="C0", name="C0_" + nm, p=P1, m=m, t=t, N=m * t, kind="random",
                    xR=int(nm.split("_x")[1]), sha256=h, source=src)
        with open(os.path.join(out, "C0_" + nm + ".json"), "w") as fh:
            json.dump(meta, fh)
        index.append(meta)
    setups = {40961: PrimeSetup(40961), 769: PrimeSetup(769), 12289: PrimeSetup(12289),
              P1: PrimeSetup(P1, curve=P1_CURVE)}
    for p, S in setups.items():
        print("prime", p, "curve", S.A, S.B, "#E", S.n, "g", S.g, "zeta16", S.z)
    cells = []
    # arm A
    for m in (4, 5, 6, 7):
        for t in (1, 2, 3):
            cells.append(("A", 40961, m, t, 2, 1 if t <= 2 or (m == 4 and t == 3) else 0))
    cells.append(("A", 40961, 4, 4, 2, 0))
    # arm B (P1, outside 10-16 bits)
    for t in (2, 3):
        for m in (5, 6, 7):
            cells.append(("B", P1, m, t, 2, 0))
    # arm C
    for p in (769, 12289):
        for m in (4, 5, 6, 7):
            cells.append(("C", p, m, 2, 2, 0))
    for arm, p, m, t, nrand, nplant in cells:
        S = setups[p]
        for idx in range(nrand):
            rng = random.Random(seed_of(arm, p, m, t, "random", idx))
            xR, yR = random_target(S, rng)
            name = f"{arm}_p{p}_m{m}_t{t}_r{idx}"
            meta = write_system(out, name, dict(arm=arm, kind="random", idx=idx, yR=yR), S, m, t, xR)
            index.append(meta)
        for idx in range(nplant):
            rng = random.Random(seed_of(arm, p, m, t, "planted", idx))
            xR, yR, sol, xs = planted_target(S, m, t, rng)
            name = f"{arm}_p{p}_m{m}_t{t}_pl{idx}"
            meta = write_system(out, name, dict(arm=arm, kind="planted", idx=idx, yR=yR, planted=sol,
                                                planted_xs=xs), S, m, t, xR)
            index.append(meta)
    with open(os.path.join(sys.argv[1], "index.jsonl"), "w") as fh:
        for meta in index:
            fh.write(json.dumps({k: v for k, v in meta.items() if k not in ("planted", "V")}) + "\n")
    print("systems:", len(index))


if __name__ == "__main__":
    main()
