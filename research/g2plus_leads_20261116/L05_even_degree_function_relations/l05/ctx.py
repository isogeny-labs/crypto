from .field import Counter, F
from .instance import select_curve, Tables, random_f
from .models import sqrt_part, OddMap


def build_ctx(p, g, cls, c, need_even=True, need_odd_tab=False):
    sel = select_curve(p, g, cls, c)
    if sel is None:
        raise SystemExit("no curve")
    cs, f, alpha = sel
    setup = Counter()
    F_ = F(p, setup)
    ctx = {"p": p, "g": g, "cls": cls, "cs": cs, "f": f, "alpha": alpha, "setup": setup}
    if cls == "oddrand":
        ctx["tab_odd"] = Tables(f, p, F_)
        ctx["odd_h"] = f
        ctx["Fodd"] = f
        return ctx
    ctx["tab_even"] = Tables(f, p, F_)
    ctx["s"] = sqrt_part(f, g, p, F_)
    if cls == "root":
        om = OddMap(f, alpha, g, p)
        ctx["omap"] = om
        ctx["odd_h"] = om.Fodd
        ctx["Fodd"] = om.Fodd
        if need_odd_tab:
            ctx["tab_odd"] = Tables(om.Fodd, p, F_)
    return ctx
