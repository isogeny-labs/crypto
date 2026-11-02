#!/usr/bin/env python3
"""P1: the factor-base tower (Petit-Kosters-Messeng isogeny form) over F_p, p = ECCp-131's prime.

p - 1 = 2 * 5 * (129-bit prime), so the Kummer tower g*mu_{2^t} does not exist for t >= 2
(and 5^2 does not divide p - 1 either).  The repository's isogeny form needs no condition
on p (src/cryptanalysis/ic_framework/prime_tower.rs, `prime-isogeny`): take an auxiliary
curve E_0/F_p with a rational cyclic subgroup <Q> of order 2^t and a rational point R';
chain t Velu 2-isogenies E_0 -> E_1 -> ... -> E_t; the factor base abscissae are
V = x(R' + <Q>), the 2^t solutions of the tower system

    u_0 = x,  u_{j+1} * den_j(u_j) - num_j(u_j) = 0  (j = 0..t-1),  u_t = c = x(phi(R')),

each equation quadratic in u_j and linear in u_{j+1} (x-map of a 2-isogeny = num/den,
deg 2 over deg 1).  The tower depends on p and the seed only, not on the curve under
test: the same V is the factor-base family on every arm.

Output tower.json: per height t the stage polynomials (low -> high, hex), c, V (hex), and
the auxiliary curve; every claim is asserted here (|V| = 2^t, V on the chain, every
v in V has u_t = c).   python3 build_tower.py --out tower.json --tmax 5
"""
import argparse, hashlib, json, os, time
import cypari2

P_HEX = "0x48e1d43f293469e33194c43186b3abc0b"
SEED = 20261102


def h256(*parts):
    return hashlib.sha256("|".join(str(x) for x in parts).encode()).hexdigest()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "tower.json"))
    ap.add_argument("--tmax", type=int, default=5)
    args = ap.parse_args()
    pari = cypari2.Pari(); pari.allocatemem(10**9)
    p = int(P_HEX, 16)
    T = args.tmax
    t0 = time.time()
    tried = 0
    while True:
        A = int(h256(SEED, "aux-A", tried), 16) % p
        B = int(h256(SEED, "aux-B", tried), 16) % p
        tried += 1
        if (4 * A**3 + 27 * B * B) % p == 0:
            continue
        E = pari.ellinit([A, B], p)
        d = pari.ellgroup(E, None, 1)
        orders = [int(x) for x in d[1]]
        if orders and orders[0] % (1 << T) == 0:
            break
    order = int(pari.ellcard(E))
    G = d[2][0]
    d1 = orders[0]
    Q = pari.ellmul(E, G, d1 // (1 << T))
    assert pari.ellmul(E, Q, 1 << T) == pari("[0]") and pari.ellmul(E, Q, 1 << (T - 1)) != pari("[0]")
    # R': a rational point whose coset does not collapse (checked by |V| = 2^t below)
    j = 0
    while True:
        x = int(h256(SEED, "aux-R", j), 16) % p
        y = pari.ellordinate(E, x)
        j += 1
        if len(y):
            R = [x, int(pari.lift(y[0]))]
            break
    out = {"schema": "p1.tower/v1", "p": P_HEX, "seed": SEED, "formulation": "PKM isogeny tower, 2-isogeny chain",
           "aux_curve": {"A": hex(A), "B": hex(B), "order": order, "group_orders": orders, "curves_tried": tried,
                         "R": [hex(R[0]), hex(R[1])], "Q_order": 1 << T}, "towers": {}}
    for t in range(2, T + 1):
        Ej = E
        Qj = pari.ellmul(E, Q, 1 << (T - t))
        Rj = pari(R)
        stages = []
        for jj in range(t):
            K = pari.ellmul(Ej, Qj, 1 << (t - jj - 1))
            assert K != pari("[0]") and K[1] == 0
            I = pari.ellisogeny(Ej, K)
            f = I[1][0] / (I[1][2] ** 2)  # PARI Velu map [f, g, h]: x -> f / h^2
            num, den = pari.numerator(f), pari.denominator(f)
            assert (int(pari.poldegree(num)), int(pari.poldegree(den))) == (2, 1), (jj, pari.poldegree(num), pari.poldegree(den))
            # normalise den monic
            lc = pari.pollead(den)
            num, den = num / lc, den / lc
            # check: x-map agrees with ellisogenyapply on a random point of Ej
            Pj = None
            xx = int(h256(SEED, "chk", t, jj), 16) % p
            while True:
                yy = pari.ellordinate(Ej, xx)
                if len(yy):
                    Pj = [xx, int(pari.lift(yy[0]))]; break
                xx += 1
            img = pari.ellisogenyapply(I[1], Pj)
            assert pari.subst(num, "x", xx) / pari.subst(den, "x", xx) == img[0]
            stages.append({"num_low_to_high": [hex(int(pari.polcoef(num, i).lift())) for i in range(3)],
                           "den_low_to_high": [hex(int(pari.polcoef(den, i).lift())) for i in range(2)],
                           "curve": {"a": hex(int(Ej[3].lift())), "b": hex(int(Ej[4].lift()))}})
            Qj = pari.ellisogenyapply(I[1], Qj)
            Rj = pari.ellisogenyapply(I[1], Rj)
            Ej = pari.ellinit(I[0], p)
        c = int(Rj[0].lift()) if len(Rj) > 1 else None
        assert c is not None
        # V = x(R + <Q_t>)
        Qt = pari.ellmul(E, Q, 1 << (T - t))
        V = []
        cur = pari(R)
        for k in range(1 << t):
            V.append(int(cur[0].lift()) if len(cur) > 1 else None)
            cur = pari.elladd(E, cur, Qt)
        assert None not in V and len(set(V)) == (1 << t), "coset collapses; change R"
        for v in V:
            u = v
            for st in stages:
                nu = sum(int(st["num_low_to_high"][i], 16) * pow(u, i, p) for i in range(3)) % p
                de = sum(int(st["den_low_to_high"][i], 16) * pow(u, i, p) for i in range(2)) % p
                assert de != 0
                u = nu * pow(de, p - 2, p) % p
            assert u == c
        out["towers"][str(t)] = {"t": t, "size": 1 << t, "c": hex(c), "stages": stages,
                                 "V_sorted": [hex(v) for v in sorted(V)]}
        print("t=%d |V|=%d ok" % (t, len(V)), flush=True)
    out["wall_s"] = time.time() - t0
    json.dump(out, open(args.out, "w"), indent=1)
    print("wrote", args.out, "aux curves tried", tried)


if __name__ == "__main__":
    main()
