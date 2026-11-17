"""L04 run_cell.py: runs one cell / arm group, writes JSON lines (SPEC section 7)."""
import argparse
import hashlib
import json
import random
import sys

import l04
from l04 import Ops, fpm, fpm_lab


def lin(r, p):
    return [(-r) % p, 1]


def make_planted(g, p, S, inS, chi, seed_planted, n=256):
    rng = random.Random(seed_planted)
    mod = 8 if g == 3 else 6
    us, exp = [], []

    def IN(used):
        while True:
            x = rng.choice(S)
            if x not in used:
                return x

    def OUT(used):
        while True:
            x = rng.randrange(p)
            if not inS[x] and x not in used:
                return x

    def irrq():
        while True:
            c0, c1 = rng.randrange(p), rng.randrange(p)
            if chi[(c1 * c1 - 4 * c0) % p] == -1:
                return [c0, c1, 1]

    def irrc():
        while True:
            u = [rng.randrange(p) for _ in range(3)] + [1]
            if all(l04.peval(u, x, p) != 0 for x in range(p)):
                return u

    def prod(*ps):
        r = [1]
        for q in ps:
            r = l04.umul(r, q, p)
        return r

    for i in range(n):
        t = i % mod
        used = set()

        def i_():
            x = IN(used); used.add(x); return x

        def o_():
            x = OUT(used); used.add(x); return x

        if g == 3:
            if t == 0:
                a, b, c = i_(), i_(), i_(); u = prod(lin(a, p), lin(b, p), lin(c, p)); e = 1
            elif t == 1:
                a, b, c = i_(), i_(), o_(); u = prod(lin(a, p), lin(b, p), lin(c, p)); e = 1
            elif t == 2:
                a, b, c = i_(), o_(), o_(); u = prod(lin(a, p), lin(b, p), lin(c, p)); e = 1
            elif t == 3:
                a, b, c = o_(), o_(), o_(); u = prod(lin(a, p), lin(b, p), lin(c, p)); e = 0
            elif t == 4:
                a, b = i_(), i_(); u = prod(lin(a, p), lin(a, p), lin(b, p)); e = 0
            elif t == 5:
                a = i_(); u = prod(lin(a, p), irrq()); e = 0
            elif t == 6:
                u = irrc(); e = 0
            else:
                a = o_(); b = i_(); u = prod(lin(a, p), lin(a, p), lin(b, p)); e = 0
        else:
            if t == 0:
                a, b = i_(), i_(); u = prod(lin(a, p), lin(b, p)); e = 1
            elif t == 1:
                a, b = i_(), o_(); u = prod(lin(a, p), lin(b, p)); e = 1
            elif t == 2:
                a, b = o_(), o_(); u = prod(lin(a, p), lin(b, p)); e = 1
            elif t == 3:
                a = o_(); u = prod(lin(a, p), lin(a, p)); e = 0
            elif t == 4:
                u = irrq(); e = 0
            else:
                a = i_(); u = prod(lin(a, p), lin(a, p)); e = 0
        us.append(u)
        exp.append(e)
    return us, exp


def oracle(u, g, p, inS):
    rts = [x for x in range(p) if l04.peval(u, x, p) == 0]
    return 1 if (len(rts) == g and sum(1 for x in rts if not inS[x]) <= 2) else 0


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cell", required=True)
    ap.add_argument("--g", type=int, required=True)
    ap.add_argument("--p", type=int, required=True)
    ap.add_argument("--T", type=int, required=True)
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--nfb", type=int, default=None)
    ap.add_argument("--arms", required=True, choices=["AB", "C", "Cpf", "all"])
    ap.add_argument("--msweep", required=True)
    ap.add_argument("--planted", action="store_true")
    ap.add_argument("--oracle", action="store_true")
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    g, p, seed = a.g, a.p, a.seed
    prime_ok = l04.is_prime(p)
    if not prime_ok:
        print("G1 fail: p not prime", file=sys.stderr)
        sys.exit(2)
    msweep = [int(x) for x in a.msweep.split(",")]
    setup = Ops()
    chi = l04.chi_table(p, setup)
    f = l04.make_f(p, g, seed)
    N = l04.compute_N(p, g, a.nfb)
    S, inS = l04.make_S(p, g, f, chi, N, setup)
    FS = l04.build_FS(S, p, setup)
    G8 = (len(FS) == N + 1 and len(S) == N and all(l04.peval(FS, s, p) == 0 for s in S))
    seed_u, seed_root, seed_aa, seed_pl = seed + 50, seed + 70, seed + 71, seed + 90
    rng = random.Random(seed_u)
    us = [[rng.randrange(p) for _ in range(g)] + [1] for _ in range(a.T)]
    T_planted = 0
    expected = None
    if a.planted:
        pu, expected = make_planted(g, p, S, inS, chi, seed_pl)
        us = us + pu
        T_planted = len(pu)
    T_cell = len(us)
    orc = [oracle(u, g, p, inS) for u in us] if a.oracle else None
    code_sha = hashlib.sha256(open(l04.__file__, "rb").read()).hexdigest()
    ctx = l04.Ctx(g, p, S, inS, FS, chi)

    groups = {"AB": [("A_pf", None), ("A_nopf", None), ("A_pf_aa", None), ("B", None), ("B_pf", None)],
              "C": [("C_m%d" % m, m) for m in msweep],
              "Cpf": [("Cpf_m%d" % m, m) for m in msweep]}
    arms = groups["AB"] + groups["C"] + groups["Cpf"] if a.arms == "all" else groups[a.arms]

    with open(a.out, "w") as fo:
        fo.write(json.dumps({"type": "cell", "lead": "L04", "cell": a.cell, "g": g, "p": p, "N": N,
                             "T_random": a.T, "T_planted": T_planted, "seed": seed, "seed_f": seed,
                             "seed_u": seed_u, "seed_root": seed_root, "seed_root_aa": seed_aa,
                             "seed_planted": seed_pl, "setup": setup.d(), "FS_deg": len(FS) - 1,
                             "G8_ok": bool(G8), "prime_ok": prime_ok, "python": sys.version,
                             "code_sha256": code_sha}) + "\n")
        for name, m in arms:
            print("arm", a.cell, name, file=sys.stderr, flush=True)
            pf = name in ("A_pf", "A_pf_aa", "B_pf") or name.startswith("Cpf")
            if name == "A_pf":
                r = l04.run_arm_A(ctx, us, True, seed_root)
            elif name == "A_pf_aa":
                r = l04.run_arm_A(ctx, us, True, seed_aa)
            elif name == "A_nopf":
                r = l04.run_arm_A(ctx, us, False, seed_root)
            elif name == "B":
                r = l04.run_arm_B(ctx, us, False, seed_root)
            elif name == "B_pf":
                r = l04.run_arm_B(ctx, us, True, seed_root)
            else:
                r = l04.run_arm_C(ctx, us, m, name.startswith("Cpf"), seed_root)
            bits = r["dec_bits"]
            dstr = "".join(str(b) for b in bits)
            # recompose
            rec = True
            for idx, rts in r["rts"].items():
                prod = [1]
                for x in rts:
                    prod = l04.umul(prod, lin(x, p), p)
                if prod != us[idx]:
                    rec = False
            lp = r["lp_ok"]
            for idx, rts in r["rts"].items():
                if sum(1 for x in rts if not inS[x]) > 2:
                    lp = False
            fp = lambda o: fpm(o) / T_cell
            tot = fpm(r["pre"]) + fpm(r["dec"]) + fpm(r["roots"])
            totlab = fpm_lab(r["pre"], p) + fpm_lab(r["dec"], p) + fpm_lab(r["roots"], p)
            oracle_ok = (bits == orc) if orc is not None else None
            planted_ok = (bits[a.T:] == expected) if expected is not None else None
            line = {"type": "arm", "cell": a.cell, "arm": name, "m": m, "prefilter": pf, "T_cell": T_cell,
                    "pre": r["pre"].d(), "dec": r["dec"].d(),
                    "tprod": r["tprod"].d() if r["tprod"] else None,
                    "trem": r["trem"].d() if r["trem"] else None,
                    "post": r["post"].d() if r["post"] else None,
                    "roots": r["roots"].d(),
                    "fpm_pre_per_cand": fp(r["pre"]), "fpm_dec_per_cand": fp(r["dec"]),
                    "fpm_roots_per_cand": fp(r["roots"]),
                    "fpm_total_per_cand": tot / T_cell, "fpm_lab_total_per_cand": totlab / T_cell,
                    "accepted": sum(bits), "splits_or_prefilter_pass": r["pass"],
                    "cz_fail": r["cz_fail"], "batch_sizes": r["batch_sizes"],
                    "decisions": dstr, "decisions_sha256": hashlib.sha256(dstr.encode()).hexdigest(),
                    "recompose_ok": rec, "lp_ok": lp, "oracle_ok": oracle_ok, "planted_ok": planted_ok}
            fo.write(json.dumps(line) + "\n")
            fo.flush()


if __name__ == "__main__":
    main()
