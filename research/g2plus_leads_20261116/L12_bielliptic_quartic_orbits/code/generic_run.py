"""Generic quartic control run (SPEC.md section 9.3). Collection only."""
import sys, os, json, random, argparse
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fp import Ops, Fp, pmul, trim, divroot
from ic_run import stream_pairs


def build_sqrt(F):
    F.build_sqrt()


def trial(F, g, p, x1, y1, x2, y2):
    """Return True iff the residual of the line section splits."""
    if x1 == x2:
        S = [0] * 5
        for jj in range(5):
            acc = 0
            xp = 1
            for i in range(5 - jj):
                acc = F.add(acc, F.mul(g[i][jj], xp))
                if i < 4 - jj:
                    xp = F.mul(xp, x1)
            S[jj] = acc
        S = trim(S)
        S, _ = divroot(F, S, y1)
        S, _ = divroot(F, S, y2)
    else:
        m = F.mul(F.sub(y2, y1), F.inv(F.sub(x2, x1)))
        c = F.sub(y1, F.mul(m, x1))
        Y = [[1], [c, m]]
        Y.append(pmul(F, Y[1], Y[1]))
        Y.append(pmul(F, Y[2], Y[1]))
        Y.append(pmul(F, Y[3], Y[1]))
        S = [0] * 5
        for jj in range(5):
            gp = pmul(F, [g[i][jj] for i in range(5 - jj)], Y[jj])
            for k in range(len(gp)):
                S[k] = F.add(S[k], gp[k])
        S = trim(S)
        S, _ = divroot(F, S, x1)
        S, _ = divroot(F, S, x2)
    S = trim(S)
    d = len(S) - 1
    if d == 2:
        e0, e1, e2 = S
        D = F.sub(F.mul(e1, e1), F.times4(F.mul(e2, e0)))
        if D == 0:
            F.inv(F.dbl(e2)); F.mul(F.neg(e1), 1)
            return True
        if F.sqrt(D) > 0:
            s = F.sqrt(D)
            t = F.inv(F.dbl(e2))
            F.mul(F.add(F.neg(e1), s), t)
            F.mul(F.sub(F.neg(e1), s), t)
            return True
        return False
    if d == 1:
        F.mul(F.neg(S[0]), F.inv(S[1]))
        return True
    return True  # degree 0: other intersections at infinity


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instance-json", required=True)
    ap.add_argument("--s", type=int, default=1)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    inst = json.loads(a.instance_json)
    p, g, cs = inst["p"], inst["g"], inst["cs"]
    # affine points (uncounted)
    import generic
    pts, _ = generic.affine_points(g, p)
    N = len(pts)
    ops = Ops()
    F = Fp(p, ops)
    tmp = Ops()
    Ft = Fp(p, tmp)
    Ft.build_sqrt()
    F.SQRT = Ft.SQRT
    ss = 7919 * cs + a.s
    split = 0
    n = 2 * N
    for t, (i, j) in enumerate(stream_pairs(ss, N, n)):
        if trial(F, g, p, pts[i][0], pts[i][1], pts[j][0], pts[j][1]):
            split += 1
    out = {"family": "G", "p": p, "j": inst["j"], "k": inst["k"], "s": a.s, "N_aff": N,
           "trials": n, "split_trials": split, "yield": split / n,
           "collect": ops.as_dict(), "fpm_per_trial": ops.fpm() / n}
    with open(a.out, "w") as f:
        json.dump(out, f, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
