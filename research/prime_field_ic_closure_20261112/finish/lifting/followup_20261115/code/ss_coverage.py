#!/usr/bin/env python3
"""Silverman-Suzuki coverage toy.  [pcc:unverified]

For a fixed E'/Q of rank s (generators from PARI ellrank, saturated), and primes p of good
reduction with #E'(F_p) = q prime:
  B_X = { n in Z^s : n^T G n <= X }   (G = Neron-Tate Gram, Silverman normalisation = PARI/2)
  image(X) = { sum n_i l_i mod q }    (l_i = dlog of red_p(gen_i))
  X*(q) = least X with |image(X)| >= sqrt(q)   (the 'lift table beats sqrt(q)' threshold)
Recorded per (curve, p): s, lambda, X*, N(X*) = |B_X*|, |image|, packing bound
T(1+2 sqrt(X*/lambda))^s, Gaussian count V_s X*^(s/2)/sqrt(det G).
"""
import sys, json, math, time, argparse, random
import cypari2

pari = cypari2.Pari()
pari.allocatemem(2 * 10 ** 9)
pari.default("parisizemax", 4 * 10 ** 9)

CURVES = {  # coefficients recalled from memory; rank is re-derived by ellrank below
    "c37a": [0, 0, 1, -1, 0], "c389a": [0, 1, 1, -2, 0], "c5077a": [0, 0, 1, -7, 6],
    "r4": [1, -1, 0, -79, 289], "r5": [0, 0, 1, -79, 342], "r6": [1, 1, 0, -2582, 48720],
    "r8": [1, -1, 0, -106384, 13075804], "r12": [0, 0, 1, -6349808647, 193146346911036],
}


def curve_data(a, sat_bound=200):
    E = pari.ellinit(a)
    R = pari.ellrank(E)
    rlo, rhi, gens = int(R[0]), int(R[1]), R[3]
    assert rlo == rhi == len(gens), ("rank not determined", rlo, rhi, len(gens))
    gens = pari.ellsaturation(E, gens, sat_bound)
    G = pari.ellheightmatrix(E, gens) / 2
    s = len(gens)
    mn = pari.qfminim(G, None, None, 2)
    lam = float(mn[1])
    tors = int(pari.elltors(E)[0])
    detG = float(pari.matdet(G))
    return dict(E=E, gens=gens, G=G, s=s, lam=lam, T=tors, detG=detG,
                N=str(pari.ellglobalred(E)[0]), disc=str(E[11]))


def lattice_vectors(G, X, cap):
    """all nonzero n with n^T G n <= X (both signs), with norms; cap on half-count."""
    res = pari.qfminim(G, X, cap, 2)
    cnt = int(res[0])
    V = res[2]
    out = []
    ncols = int(pari.matsize(V)[1]) if cnt else 0
    for j in range(ncols):
        col = [int(V[i, j]) for i in range(int(pari.matsize(V)[0]))]
        out.append(col)
    return cnt, out


def coverage(cd, p, Xmax_cap=1500000):
    E, gens, G, s = cd["E"], cd["gens"], cd["G"], cd["s"]
    Ep = pari.ellinit(cd["E"][:5], p)
    q = int(pari.ellcard(Ep))
    gp = pari.ellgenerators(Ep)[0]
    ls = []
    for P in gens:
        if int(pari.denominator(P[0])) % p == 0:
            ls.append(0)          # reduces to O
            continue
        Pp = [pari.Mod(P[0], p), pari.Mod(P[1], p)]
        ls.append(int(pari.elllog(Ep, Pp, gp, q)))
    target = math.isqrt(q) + 1
    # grow X geometrically until image >= target
    X = cd["lam"] * 1.01
    while True:
        cnt, vecs = lattice_vectors(G, X, Xmax_cap)
        if 2 * cnt + 1 >= 3 * target or cnt >= Xmax_cap:
            break
        X *= 1.6
    # exact threshold: sort vectors by norm, add both signs
    Gf = [[float(G[i, j]) for j in range(s)] for i in range(s)]
    def nrm(v):
        return sum(v[i] * Gf[i][j] * v[j] for i in range(s) for j in range(s))
    vecs.sort(key=nrm)
    img = {0}
    xstar = None
    nvec = 1
    for v in vecs:
        val = sum(vi * li for vi, li in zip(v, ls)) % q
        img.add(val)
        img.add((-val) % q)
        nvec += 2
        if len(img) >= target:
            xstar = nrm(v)
            break
    if xstar is None:
        return dict(p=p, q=q, status="cap")
    lam, T = cd["lam"], cd["T"]
    pack = T * (1 + 2 * math.sqrt(xstar / lam)) ** s
    Vs = math.pi ** (s / 2) / math.gamma(s / 2 + 1)
    gauss = Vs * xstar ** (s / 2) / math.sqrt(cd["detG"])
    return dict(p=p, q=q, status="ok", sqrtq=target, Xstar=xstar, Xstar_over_lam=xstar / lam,
                N_at_Xstar=nvec, image=len(img), collisions=nvec - len(img),
                pack_bound=pack, gauss=gauss)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--curves", default=",".join(CURVES))
    ap.add_argument("--bits", default="12,14,16,18,20,22,24")
    ap.add_argument("--per", type=int, default=4)
    ap.add_argument("--seed", type=int, default=20261115)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    pari.setrand(a.seed)
    with open(a.out, "a") as fo:
        for nm in a.curves.split(","):
            t0 = time.time()
            cd = curve_data(CURVES[nm])
            head = dict(kind="curve", name=nm, a=CURVES[nm], s=cd["s"], lam=cd["lam"], T=cd["T"],
                        detG=cd["detG"], N=cd["N"], log_absDisc=math.log(abs(int(cd["disc"]))),
                        gens=str(cd["gens"]), sec=round(time.time() - t0, 2))
            fo.write(json.dumps(head) + "\n"); fo.flush()
            for b in [int(x) for x in a.bits.split(",")]:
                got = 0
                tries = 0
                while got < a.per and tries < 20000:
                    tries += 1
                    p = int(pari.randomprime([2 ** (b - 1), 2 ** b]))
                    if int(pari.Mod(int(cd["disc"]), p)) == 0:
                        continue
                    ap_ = int(pari.ellap(cd["E"], p))
                    q = p + 1 - ap_
                    if not pari.isprime(q):
                        continue
                    t1 = time.time()
                    rec = coverage(cd, p)
                    rec.update(kind="cov", name=nm, s=cd["s"], bits=b, sec=round(time.time() - t1, 2))
                    fo.write(json.dumps(rec) + "\n"); fo.flush()
                    got += 1


if __name__ == "__main__":
    main()
