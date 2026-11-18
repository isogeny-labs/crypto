import argparse, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ff
from order import *
from seeds import rng

def jdump(fh, d):
    fh.write(json.dumps(d, sort_keys=True) + "\n"); fh.flush()

def select_cm(family, bits, out):
    q = 5 if family == "F5" else 7
    nprimes = 30 if family == "F5" else 15
    with open(out, "w") as fh:
        for k in bits:
            admitted = 0
            p = 2 ** k - 1
            for _ in range(nprimes):
                p = next_prime_cong(p + 1, 2 * q, 1)
                gam = prim_root(p)
                for i in range(2 * q):
                    a = pow(gam, i, p)
                    cid = "%s:%d:%d" % (family, p, a)
                    f = [a] + [0] * (q - 1) + [1]
                    C, reason = build_curve(family, p, q, f, {"curve_id": cid, "a": a}, bits=k)
                    if C is None:
                        jdump(fh, {"rejected": True, "curve_id": cid, "p": p, "reason": reason, "bits": k})
                    else:
                        jdump(fh, C.todict()); admitted += 1
                        print("admitted", cid, "N=%d h=%d r=%d" % (C.N, C.h, C.r), flush=True)
                        if admitted == 3:
                            break
                if admitted == 3:
                    break
            print("bits", k, "admitted", admitted, flush=True)

def select_ctrl(family, cm_path, out):
    q = 5 if family == "C5" else 7
    cms = [json.loads(l) for l in open(cm_path) if l.strip()]
    cms = [c for c in cms if not c.get("rejected")]
    used = {}
    with open(out, "w") as fh:
        for cm in cms:
            p = cm["p"]
            j = used.get(p, 0)
            while True:
                rnd = rng("ctrl:%d:%d:%d" % (q, p, j))
                c = [rnd.randrange(p) for _ in range(q)]
                f = c + [1]
                cid = "%s:%d:j%d" % (family, p, j)
                if all(ci == 0 for ci in c[1:q]):
                    jdump(fh, {"rejected": True, "curve_id": cid, "p": p, "reason": "trivial_ctrl", "bits": cm["bits"]})
                    j += 1; continue
                C, reason = build_curve(family, p, q, f, {"curve_id": cid, "j": j}, bits=cm["bits"])
                j += 1
                if C is None:
                    jdump(fh, {"rejected": True, "curve_id": cid, "p": p, "reason": reason, "bits": cm["bits"]})
                    continue
                jdump(fh, C.todict())
                print("admitted", cid, "N=%d h=%d r=%d" % (C.N, C.h, C.r), flush=True)
                break
            used[p] = j

if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--family", required=True)
    ap.add_argument("--bits")
    ap.add_argument("--cm")
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    if a.family in ("F5", "F7"):
        select_cm(a.family, [int(x) for x in a.bits.split(",")], a.out)
    else:
        select_ctrl(a.family, a.cm, a.out)
