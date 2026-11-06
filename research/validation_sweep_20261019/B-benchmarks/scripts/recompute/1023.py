import json, os
BASE = "/home/user/crypto/research/isogeny_loop_glv_20261011"
d = json.load(open(os.path.join(BASE, "glvgls_lattice.json")))
for r in d["rows"]:
    if r["D"] == -619 and r.get("embedding") in ("s", "-s"):
        print(f"D=-619 embedding={r['embedding']} model.ratio={r['model']['ratio']!r} gls2_M={r['model']['gls2_M']} glvgls4_M={r['model']['glvgls4_M']} c_dbl={r['model']['c_dbl']}")
dd = json.load(open(os.path.join(BASE, "degree_bound.json")))
c_dbl = dd["c_dbl"]
print("degree_bound.json c_dbl:", c_dbl)
print("README formula 2(c_dbl+2)/c_dbl =", 2 * (c_dbl + 2) / c_dbl)
