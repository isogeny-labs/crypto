"""D6: coverage against decision cost for each measured family (all measured F), gap at w = 0.1."""
import json, math, pathlib
D = pathlib.Path(__file__).resolve().parents[1]; r = 131203; sr = math.sqrt(r); RHO01 = 1.716
d1 = json.load(open(D / "data/d1_m2.json")); d5 = json.load(open(D / "data/d5_m3.json"))
out = []
for x in d1['rows']:
    F, C = x['F'], x['C']; T = (F + 1) * r / C; base = (3 * F + 0.1 * (25 * F + 3 * F * F)) / sr
    out.append(dict(family="D1 m=2", F=F, C_over_r=C / r, trials=T, gapPN=(base + T * (1 + 0.1 * x['c_add']) / sr) / RHO01,
                    gapPG=(base + T * (1 + 2 * F) / sr) / RHO01))
for x in d5:
    F, C = x['F'], x['C']; T = (F + 1) * r / C; base = (3 * F + 0.1 * (25 * F + 4 * F * F)) / sr
    out.append(dict(family="D5 m=3 MITM", F=F, C_over_r=C / r, trials=T,
                    gapPN=(base + 0.1 * 7 * F * (F + 1) / sr + T * (1 + 0.1 * x['c_mitm']) / sr) / RHO01,
                    gapPG=(base + F * (F + 1) / sr + T * (1 + 2 * F) / sr) / RHO01))
for o in out: print({k: (round(v, 3) if isinstance(v, float) else v) for k, v in o.items()})
json.dump(out, open(D / "data/d6_tradeoff.json", "w"), indent=1)
