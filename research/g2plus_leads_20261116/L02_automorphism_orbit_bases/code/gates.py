import argparse, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ff, jac, aut
from ff import uncounted
from order import *
from seeds import rng

def check_curve(d):
    C = Curve(d)
    ff.set_p(C.p)
    res = {"curve_id": C.curve_id}
    N, info = jacobian_order(C)
    res["G1_order_unique"] = (N is not None and N == C.N)
    res["G1_r_prime"] = is_prime(C.r) and C.N == C.h * C.r
    with uncounted():
        rnd = rng("gate:G1:" + C.curve_id)
        divs = [jac.random_divisor(C, rnd) for _ in range(32)]
        res["G1_annihilate32"] = all(jac.is_ident(jac.smul(C.N, D, C)) for D in divs)
    cm = C.family in ("F5", "F7")
    if cm:
        lam = find_lambda(C)
        res["G1_lambda_unique"] = (lam is not None and lam == C.lam)
        res["G1_r_mod_q"] = (C.r % C.q == 1)
        with uncounted():
            rnd = rng("transport:" + C.curve_id)
            ok = []
            for _ in range(20):
                P = jac.point_divisor(*jac.random_point(C, rnd))
                lhs = jac.smul(C.h, aut.rho_apply(P, C), C)
                rhs = jac.smul(C.lam, jac.smul(C.h, P, C), C)
                ok.append(lhs == rhs)
            res["G2_transport"] = all(ok)
    else:
        res["G1_lambda_unique"] = True; res["G1_r_mod_q"] = True; res["G2_transport"] = True
    res["pass"] = all(v for k, v in res.items() if k != "curve_id")
    return res

if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--instances", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    with open(a.out, "w") as fh:
        for path in a.instances.split(","):
            for l in open(path):
                d = json.loads(l)
                if d.get("rejected"):
                    continue
                res = check_curve(d)
                fh.write(json.dumps(res, sort_keys=True) + "\n"); fh.flush()
                print(res["curve_id"], res["pass"], flush=True)
