#!/usr/bin/env python3
"""Reads saved comparisons only; writes analysis/summary.json, prints the C1 table (PROTOCOL.md)."""
import json, glob, os
os.chdir(os.path.join(os.path.dirname(__file__), ".."))
W = ["0", "0.1", "1"]
out = {}
for c in ["p16", "p18", "p36", "p37"]:
    d = f"sessions/D-{c}/comparisons/"
    cells = {}
    for b in (2, 4, 8):
        main = json.load(open(d + f"res{b}__res{b}dp.json"))["ops"]
        pts = main["band"]["ratio_b_over_a"]          # point ratios at w = 0, 0.1, 1
        cell = {"pairs": main["pairs"], "order": main["band"]["order"], "w": {}}
        cell["w"]["0"] = {"ratio": main["ratio_b_over_a"], "ci": main["ci95"]}
        for p in glob.glob(d + f"res{b}__res{b}dp__*.json"):
            o = json.load(open(p))["ops"]
            for k, w in ((1, "0.1"), (2, "1")):
                if abs(o["ratio_b_over_a"] - pts[k]) < 1e-9:
                    cell["w"][w] = {"ratio": o["ratio_b_over_a"], "ci": o["ci95"]}
        cell["below_1_everywhere"] = all(w in cell["w"] and cell["w"][w]["ci"][1] < 1 for w in W)
        cells[f"res{b}"] = cell
    aa = json.load(open(d + "res4__res4aa.json"))["ops"]
    rho = {}
    for a in ("res2", "res4", "res8", "res2dp", "res4dp", "res8dp"):
        j = json.load(open(d + f"rho__{a}.json"))["ops"]
        rho[a] = {"w0": j["ratio_b_over_a"], "band": j["band"]["ratio_b_over_a"]}
    out[c] = {"dp_vs_base": cells, "aa": [aa["ratio_b_over_a"], aa["ci95"]], "vs_rho": rho}
json.dump(out, open("analysis/summary.json", "w"), indent=1)
for c, v in out.items():
    for b, r in v["dp_vs_base"].items():
        s = "  ".join("w=%s %.3f [%.3f,%.3f]" % (w, r["w"][w]["ratio"], *r["w"][w]["ci"]) if w in r["w"] else "w=%s ?" % w for w in W)
        print(c, b, s, "| all<1:", r["below_1_everywhere"])
    print(c, "A/A", v["aa"])
    for a in ("res4", "res4dp"):
        print(c, "rho->", a, "w0 %.2f  w0/0.1/1 %s" % (v["vs_rho"][a]["w0"], [round(x, 2) for x in v["vs_rho"][a]["band"]]))
c1 = all(out[c]["dp_vs_base"][b]["below_1_everywhere"] for c in ("p36", "p37") for b in ("res2", "res4", "res8"))
print("C1 (all six cells below 1 at w=0,0.1,1):", c1)
