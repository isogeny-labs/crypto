"""Instance selection (SPEC.md section 5)."""
import sys, os, json, random, argparse
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fp import Ops, Fp
from curve import Curve, setup_points
import counts
import generic

F1_PRIMES = [263, 523, 1031, 2063]
F0_PRIMES = [269, 521, 1033, 2069]
G_PRIMES = [263, 523, 1031, 2063]


def factor_largest(n):
    best = 1
    m = n
    d = 2
    while d * d <= m:
        while m % d == 0:
            best = d
            m //= d
        d += 1
    if m > 1:
        best = max(best, m)
    return best


class InstrumentError(Exception):
    pass


def coeffs(family, p, cs):
    rng = random.Random(cs)
    if family == "F1":
        return [rng.randrange(p) for _ in range(5)]  # a1, a3, a2, a4, a6
    a4 = rng.randrange(p)
    a6 = rng.randrange(p)
    return [0, 0, 0, a4, a6]


def check(family, p, k, cs):
    """Return (instance dict or None, reject reason or None)."""
    a1, a3, a2, a4, a6 = coeffs(family, p, cs)
    if family == "F1" and a1 == 0 and a3 == 0:
        return None, "h_zero"
    if family == "F0" and (a4 == 0 or a6 == 0):
        return None, "a46_zero"
    cv = Curve(p, a1, a3, a2, a4, a6)
    if cv.disc_f == 0:
        return None, "disc_f"
    if cv.DeltaE == 0:
        return None, "deltaE"
    if cv.c4 == 0:
        return None, "j0"
    if cv.c6 == 0:
        return None, "j1728"
    F = Fp(p, Ops())
    S = setup_points(F, cv)
    nE = S.nE
    r = factor_largest(nE)
    if r < 5:
        return None, "r_small"
    if nE % (r * r) == 0:
        return None, "r_squared"
    if nE // r > 16:
        return None, "cofactor"
    if S.N_aff < 8:
        return None, "few_points"
    nE1, nC1 = counts.count_Fp(cv)
    if nC1 != S.nC or nE1 != nE:
        raise InstrumentError("count_mismatch")
    cn = counts.full_counts(cv)
    if not cn["echeck"]:
        raise InstrumentError("count_mismatch (E check)")
    if not cn["parity"]:
        raise InstrumentError("parity")
    if not cn["weil"]:
        raise InstrumentError("weil")
    if cn["nP"] % r == 0:
        return None, "r_divides_P"
    inst = {"family": family, "p": p, "k": k, "cs": cs, "a1": a1, "a3": a3, "a2": a2,
            "a4": a4, "a6": a6, "nE": nE, "r": r, "c": nE // r, "N_aff": S.N_aff,
            "N_ram": S.N_ram, "nC1": nC1, "nE2": cn["nE2"], "nC2": cn["nC2"],
            "s1": cn["s1"], "s2": cn["s2"], "nP": cn["nP"], "nJ": nE * cn["nP"]}
    return inst, None


def find(family, p, want):
    base = {"F1": 1000003, "F0": 2000003, "G": 3000003}[family]
    out = []
    rejects = []
    for k in range(10000):
        cs = base * p + k
        if family == "G":
            res, reason = generic.accept(cs, p)
            if res is None:
                rejects.append((family, p, k, reason))
                continue
            g, pts = res
            inst = {"family": "G", "p": p, "k": k, "cs": cs, "g": g, "N_aff": len(pts)}
        else:
            inst, reason = check(family, p, k, cs)
            if inst is None:
                rejects.append((family, p, k, reason))
                continue
        inst["j"] = len(out)
        out.append(inst)
        if len(out) == want:
            break
    return out, rejects


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--rejects", required=True)
    a = ap.parse_args()
    allrej = []
    with open(a.out, "w") as fo:
        for family, primes, want in (("F1", F1_PRIMES, 4), ("F0", F0_PRIMES, 2), ("G", G_PRIMES, 2)):
            for p in primes:
                try:
                    insts, rej = find(family, p, want)
                except InstrumentError as e:
                    print("INSTRUMENT ERROR", family, p, e)
                    sys.exit(2)
                allrej += rej
                if len(insts) < want:
                    print("only %d instances for %s p=%d" % (len(insts), family, p))
                    sys.exit(3)
                for i in insts:
                    fo.write(json.dumps(i, sort_keys=True) + "\n")
                    print(family, p, "j=%d k=%d r=%d c=%d N_aff=%d" %
                          (i["j"], i["k"], i.get("r", 0), i.get("c", 0), i["N_aff"]))
                sys.stdout.flush()
    with open(a.rejects, "w") as fr:
        fr.write("family\tp\tk\treason\n")
        for row in allrej:
            fr.write("%s\t%d\t%d\t%s\n" % row)


if __name__ == "__main__":
    main()
