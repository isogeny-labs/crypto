"""Check: the top-degree form of S_R (in x_1..x_m, x_R fixed) is c * prod x_i^{2^{m-1}}, c != 0, for every
curve and many targets; hence D_ff = Delta + max(0, d - 2^{m-1}) exactly (monomial-algebra kernel).  [pcc:unverified]"""
import json, random, sys
import f4lab as L, icmodel as M
res = {}
for bits in (10, 12, 16, 20):
    p, a, b, n = L.get_curve(bits)
    for m in (2, 3, 4):
        rng = random.Random(f"tf-{bits}-{m}"); ok = 0; T = 40 if m < 4 else 10; forms = set()
        for _ in range(T):
            R = L.rand_point(p, a, b, rng)
            S = L.semaev(m, p, a, b, R[0])
            Sh, D = L.topform(S)
            forms.add((D, tuple(sorted(Sh))))
            ok += (D == m * 2 ** (m - 1) and list(Sh) == [tuple([2 ** (m - 1)] * m)])
        res[f"p{bits}_m{m}"] = {"ok": ok, "of": T, "forms": [str(f) for f in forms]}
# also the generic polynomial over Z: coefficient of prod x_i^{2^{m-1}} with x_{m+1}^0 a^0 b^0
print(json.dumps(res, indent=0))
