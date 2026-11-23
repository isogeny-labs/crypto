"""Pollard rho reference on E (SPEC.md section 9.2)."""
import sys, os, json, random, argparse
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fp import Ops, Fp
from curve import Curve, eadd, emul, free_field


def E_points(cv):
    p = cv.p
    T = {}
    for w in range(p):
        T.setdefault(w * w % p, []).append(w)
    pts = []
    for x in range(p):
        hx, fx = cv.h(x), cv.f(x)
        for w in range(p):
            if (w * w + hx * w - fx) % p == 0:
                pts.append((x, w))
    pts.sort()
    return pts


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instance-json", required=True)
    ap.add_argument("--w", type=int, required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    inst = json.loads(a.instance_json)
    p, r, c, cs = inst["p"], inst["r"], inst["c"], inst["cs"]
    cv = Curve(p, inst["a1"], inst["a3"], inst["a2"], inst["a4"], inst["a6"])
    FF = free_field(p)
    G = None
    for Q0 in E_points(cv):
        G = emul(FF, cv, Q0, c)
        if G is not None:
            break
    k_true = random.Random(cs + 777).randrange(1, r)
    T = emul(FF, cv, G, k_true)
    ops = Ops()
    F = Fp(p, ops)
    nadd = [0]

    def add(P, Q):
        nadd[0] += 1
        return eadd(F, cv, P, Q)
    restart = 0
    k_found = None
    iterations = 0
    while restart <= 10:
        rng = random.Random(cs + 900 + a.w + 1000 * restart)
        al = []
        be = []
        M = []
        for _ in range(20):
            x_, y_ = rng.randrange(r), rng.randrange(r)
            al.append(x_); be.append(y_)
            M.append(add(emul(F, cv, G, x_), emul(F, cv, T, y_)))
        a0, b0 = rng.randrange(r), rng.randrange(r)
        X0 = add(emul(F, cv, G, a0), emul(F, cv, T, b0))

        def step(st):
            X, aa, bb = st
            j = X[0] % 20 if X is not None else 0
            return (add(X, M[j]), (aa + al[j]) % r, (bb + be[j]) % r)
        tor = (X0, a0, b0)
        har = (X0, a0, b0)
        while True:
            iterations += 1
            tor = step(tor)
            har = step(step(har))
            if tor[0] == har[0]:
                break
        if tor[2] % r == har[2] % r:
            restart += 1
            continue
        ops.inv += 1
        ops.mul += 1
        k_found = (tor[1] - har[1]) * pow((har[2] - tor[2]) % r, r - 2, r) % r
        break
    correct = (k_found is not None and emul(FF, cv, G, k_found) == T)
    out = {"family": inst["family"], "p": p, "j": inst["j"], "w": a.w, "r": r,
           "k_true": k_true, "k_found": k_found, "correct": bool(correct),
           "restarts": restart, "iterations": iterations, "group_ops": nadd[0],
           "fpm": ops.fpm(), "mul": ops.mul, "inv": ops.inv}
    with open(a.out, "w") as f:
        json.dump(out, f, sort_keys=True)
        f.write("\n")


if __name__ == "__main__":
    main()
