import json, glob, os, statistics as st
BASE = "/home/user/crypto/research/isogeny_loop_glv_20261012"
rows = {}
for f in ["bench_D23.json", "bench_D3019.json", "bench_D431.json", "bench_D71.json", "bench_run1.json", "bench_run2.json"]:
    d = json.load(open(os.path.join(BASE, f)))
    arms = {a["name"]: a for a in d["arms"]}
    med = {k: st.median(a["rounds_ns"]) for k, a in arms.items()}
    gls2 = med["gls2"]; proj = med["glvgls4 (projective loop)"]
    rows[f] = (d["d"], gls2, proj, gls2 / proj)
    print(f"{f} d={d['d']} gls2_median_ns={gls2:.3f} glvgls4_proj_median_ns={proj:.3f} ratio_gls2_over_proj={gls2/proj:.4f} aa_ratio={med['wnaf']/med['wnaf (A/A)']:.4f}")
r1 = rows["bench_run1.json"]
print("CLAIM run1 D=-619 proj median ns:", round(r1[2]), " GLS-2 median ns:", round(r1[1]), " ratio:", round(r1[3], 3))
ratios = [v[3] for v in rows.values()]
print("across-D projective ratios min/max (6 files):", round(min(ratios), 4), round(max(ratios), 4))
