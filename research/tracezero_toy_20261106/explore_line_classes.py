#!/usr/bin/env python3
"""Post hoc exploration (README 'Explaining the k=2, d=1 residual'): per-line classes.

For k = 2, d = 1 every V = F_p*g is one of p+1 lines of F_{p^2} over F_p. Classes:
'rational' (g in F_p), 'pure' (g^p = -g), 'generic'. Mean small-base cov_2/|S| of the
trace arm minus a sign-symmetric random null, per class. Run: python3 -I explore_line_classes.py
"""
import importlib.util, json, os, random, sys
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("tz", os.path.join(here, "tracezero.py"))
tz = importlib.util.module_from_spec(spec); spec.loader.exec_module(tz)

def run(p, ncurves, reps):
    F = tz.Field(p, 2)
    lines = [1] + [c0 + p for c0 in range(p)]            # g = 1 (F_p) and g = c0 + t
    def cls(g):
        if g < p: return "rational"
        return "pure" if F.frob(g) == F.neg(g) else "generic"
    out = {}
    for a, b in tz.curves_for(p, 2, ncurves):
        curve = tz.Curve(p, a, b); rnd = random.Random(tz.seed("explore", p, a, b))
        for g in lines:
            V = sorted({F.mul(t, g) if t else 0 for t in range(p)})
            n0, tr = tz.lifts_trace(F, curve, V, a, b)
            for _ in range(reps):
                r0 = [rnd.randrange(curve.n) for _ in range(n0 // 2)]
                r0 += [curve.neg[i] for i in r0]
                st, sr = curve.sub_stats(tr, 3, rnd), curve.sub_stats(r0, 3, rnd)
                if st is None or sr is None: continue
                d = out.setdefault(cls(g), [0, 0.0, 0.0, 0])
                d[0] += 1; d[1] += st[1] - sr[1]; d[2] += st[0] - sr[0]
    return {c: {"n": v[0], "d_cov2": v[1] / v[0], "d_size": v[2] / v[0]} for c, v in out.items()}, len(lines)

res = {}
for p in (23, 37, 53):
    r, nl = run(p, 60, 4)
    # exposure: each class's share when g is uniform in F_{p^2}\{0}
    res[p] = r
    w = {"rational": (p - 1) / (p * p - 1), "pure": (p - 1) / (p * p - 1)}
    w["generic"] = 1 - 2 * w["rational"]
    mix = sum(w[c] * r[c]["d_cov2"] for c in r)
    print(p, json.dumps(r, sort_keys=True), "weighted_mix_d_cov2=%.4f" % mix)
