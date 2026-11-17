import argparse
import json
import sys
from l05.ctx import build_ctx
from l05.field import GateFailure
from l05.pencil import run_arm, ARM_NAMES


def main():
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd")
    g_ = sub.add_parser("gate")
    g_.add_argument("--g", type=int, required=True)
    g_.add_argument("--p", type=int, required=True)
    g_.add_argument("--out", required=True)
    r = sub.add_parser("relphase")
    r.add_argument("--g", type=int, required=True)
    r.add_argument("--p", type=int, required=True)
    r.add_argument("--class", dest="cls", required=True, choices=["root", "rootless", "oddrand"])
    r.add_argument("--curve-index", type=int, required=True)
    r.add_argument("--seed", type=int, required=True)
    r.add_argument("--arm", required=True)
    r.add_argument("--out", required=True)
    r.add_argument("--cpu-budget", type=float, default=540)
    a = ap.parse_args()
    if a.cmd == "gate":
        from l05.gate import run_gate
        ok = run_gate(a.g, a.p, a.out)
        return 0 if ok else 2
    if a.cmd != "relphase":
        return 1
    if a.arm not in ARM_NAMES:
        return 1
    odd_arm = a.arm in ("O", "O_T")
    if (odd_arm and a.cls == "rootless") or ((not odd_arm) and a.cls == "oddrand"):
        return 1
    ctx = build_ctx(a.p, a.g, a.cls, a.curve_index, need_odd_tab=(odd_arm and a.cls == "root"))
    vmode = "sample" if a.cls == "root" else ("v2odd" if odd_arm else "none")
    try:
        rec, pens = run_arm(a.arm, ctx, a.g, a.p, ctx["cs"], a.seed, a.cpu_budget, vmode)
    except GateFailure as e:
        with open(a.out, "w") as fo:
            fo.write(json.dumps({"type": "failure", "stage": e.stage, "arm": a.arm, "message": e.msg,
                                 "relation": e.relation, "counts": getattr(e, "counts", None),
                                 "g": a.g, "p": a.p, "class": a.cls, "curve_index": a.curve_index,
                                 "solver_seed": a.seed}, sort_keys=True) + "\n")
        return 2
    rec.update({"class": a.cls, "curve_index": a.curve_index, "curve_seed": ctx["cs"], "solver_seed": a.seed,
                "f": ctx["f"], "alpha": ctx["alpha"], "Fodd": ctx.get("Fodd"),
                "setup_mul": ctx["setup"].mul, "setup_inv": ctx["setup"].inv})
    with open(a.out, "w") as fo:
        for pr in pens:
            fo.write(json.dumps(pr, sort_keys=True) + "\n")
        fo.write(json.dumps(rec, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
