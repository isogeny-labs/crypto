import json
from .ctx import build_ctx
from .field import GateFailure
from .instance import Tables
from .pencil import run_arm
from .poly import smul, cantor, point, IDENT
from .rng import SM64, stream_seed, TAGS
from .zeta import lpoly, order_of_delta, random_divisor


def run_gate(g, p, out):
    ok_all = True
    curves = []
    with open(out, "w") as fo:
        def w(o):
            fo.write(json.dumps(o, sort_keys=True) + "\n")
            fo.flush()
        for c in (1, 2, 3):
            ctx = build_ctx(p, g, "root", c, need_odd_tab=True)
            om = ctx["omap"]
            tab_e, tab_o = ctx["tab_even"], ctx["tab_odd"]
            Le, Ne = lpoly(ctx["f"], p, g, tab_e.chi, 2)
            Lo, No = lpoly(om.Fodd, p, g, tab_o.chi, 1)
            Je, Jo = sum(Le), sum(Lo)
            rng = SM64(stream_seed(TAGS["GATE"], g, p, ctx["cs"], 0))
            jc = (Je == Jo)
            for _ in range(16):
                D = random_divisor(rng, tab_o, om.Fodd, g, p)
                if smul(D, Je, om.Fodd, g, p) != IDENT:
                    jc = False
            Wp = point(*om.map_point("inf+"), p)
            Dd = cantor(Wp, Wp, om.Fodd, g, p)
            od = order_of_delta(Dd, Je, om.Fodd, g, p)
            w({"type": "gate_curve", "g": g, "p": p, "curve_index": c, "curve_seed": ctx["cs"], "f": ctx["f"],
               "alpha": ctx["alpha"], "Fodd": om.Fodd, "s": ctx["s"], "L_even": Le, "L_odd": Lo,
               "J_even": Je, "J_odd": Jo, "jcheck_ok": jc, "ord_delta": od})
            curve_ok = jc
            for arm in ("E_0", "E_low", "E_1a", "E_g", "O"):
                try:
                    rec, pens = run_arm(arm, ctx, g, p, ctx["cs"], 1, 540, "full")
                except GateFailure as e:
                    w({"type": "failure", "stage": e.stage, "arm": arm, "message": e.msg,
                       "relation": e.relation, "counts": getattr(e, "counts", None), "curve_index": c})
                    curve_ok = False
                    continue
                rec.update({"class": "root", "curve_index": c, "curve_seed": ctx["cs"], "solver_seed": 1,
                            "f": ctx["f"], "alpha": ctx["alpha"], "Fodd": om.Fodd,
                            "setup_mul": ctx["setup"].mul, "setup_inv": ctx["setup"].inv})
                for pr in pens:
                    w(pr)
                w(rec)
                if any(rec[k] for k in ("v1_failed", "vinf_failed", "v2_failed", "m1_accepted", "m2_accepted")):
                    curve_ok = False
            curves.append({"curve_index": c, "pass": curve_ok})
            ok_all = ok_all and curve_ok
        w({"type": "gate_summary", "g": g, "p": p, "pass": ok_all, "curves": curves})
    return ok_all
