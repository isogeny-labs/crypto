"""Parity screen for algebraic relation oracles at k = 20 (r = 131,203).
Model S(w) = (F+1) * (r/C) * (1 + w*c) / sqrt(r) (setup ignored: favours IC).
Rho reference: measured rho-neg S at w = 0, 0.1, 1 (T13 band_pooled.txt)."""
import json, math
r = 131203; sr = math.sqrt(r); RHO = {0: 1.600, 0.1: 1.716, 1: 2.764}
fam = [  # name, F, C, c (native ops per trial), source
 ("isogeny t4 G3 s1193 + trace (T13)", 8, 52, 17592 * 0.4740, "measured: L10 mpt 17,592; T13 trace ratio 0.4740"),
 ("isogeny t5 G4 s1307 + trace (L12)", 22, 210, 24912, "measured: L12 engine 24,912 mul-adds/target"),
 ("isogeny t5 G4 s1307, exhaustive grid (L13)", 22, 210, 1248, "measured: L13, 1,248 F_p mults/target"),
 ("digit set t5 G4 (L15)", 22, 522, 1375663, "measured: L15 mpt 1,375,663"),
 ("isogeny t4, m = 3 (T3)", 8, 8 * math.comb(10, 3), 17592 * 2297, "measured cost ratio 2,297x (T3); coverage derived upper bound 2^3*C(F+2,3)"),
 ("m = 2 full coverage, F = 256, root-find per base point (rootfind-subtract style)", 256, 2 * 256 ** 2, 256 * 25, "derived: 2F^2 >= r; per trial one quadratic root (~25 F_p mults) and one hash test per base point"),
 ("m = 2 full coverage, F = 256, exhaustive pair scan", 256, 2 * 256 ** 2, 256 ** 2 // 2 * 6, "derived: F^2/2 pairs x ~6 mults (grid count, L13)"),
 ("m = 3 full coverage, F = 46, F4 on S4 (T3 cost scaled from F = 8)", 46, 8 * math.comb(48, 3), 17592 * 2297, "derived coverage 2^3 C(F+2,3) >= r; cost = T3 measured at F = 8, a lower bound for F = 46"),
 ("mu_d base, d = 2,394 (L11 design), scan oracle", 1197, 2 * 1197 ** 2, 1197 * 20, "derived: F ~ d/2, C <= 2F^2 (capped at r), c >= F * 20 (one sqrt + test per candidate)"),
]
rows = []
for name, F, C, c, src in fam:
    C = min(C, r); trials = (F + 1) * r / C
    S = {w: trials * (1 + w * c) / sr for w in RHO}
    gap = {w: S[w] / RHO[w] for w in RHO}
    cmax = {w: max(0.0, (RHO[w] * sr / trials - 1) / w) for w in (0.1, 1)}
    rows.append(dict(family=name, F=F, C=C, trials=trials, c=c, S=S, gap=gap, c_max=cmax, source=src))
    print(f"{name:48s} F={F:5d} C={C:7.0f} trials={trials:9.1f} c={c:11.0f} gap w0={gap[0]:8.3g} w0.1={gap[0.1]:8.3g} w1={gap[1]:8.3g} cmax(0.1)={cmax[0.1]:.3g}")
json.dump(dict(r=r, rho=RHO, rows=rows), open("data/screen.json", "w"), indent=1, default=str)
