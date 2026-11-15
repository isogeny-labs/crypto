#!/usr/bin/env python3
"""See PROTOCOL.md. Run: python3 -I analyse.py > summary.txt"""
import json, os, random
here = os.path.dirname(os.path.abspath(__file__)); R = os.path.join(here, "..")
SD = os.path.join(R, "curve25519_isogeny_seeds_20261114"); DD = os.path.join(R, "curve25519_miniature_descent_class_20261111", "tower_dumps")
p, A = 1048573, 926
leg = lambda v: 0 if v % p == 0 else (1 if pow(v % p, (p-1)//2, p) == 1 else -1)
shift = A * pow(3, p-2, p) % p
rhs = lambda x: (x*x*x + A*x*x + x) % p
def arm(s, L):
    return json.load(open(os.path.join(SD, "runs", ("iso_L%d.json" % L) if s == 0 else "s%d_L%d.json" % (s, L))))
tc = {json.loads(l)["seed"]: json.loads(l) for l in open(os.path.join(SD, "tower_cost.jsonl"))}
rows = []
for s in range(21):
    a0, a1 = arm(s, 0), arm(s, 1)
    r = (a0["hits_verified"] + a1["hits_verified"]) / ((a0["f4_muladds"] + a1["f4_muladds"]) / 1e9)
    v = json.load(open(os.path.join(DD, "v_t5_seed%d.json" % s)))["v"]
    on = [(x - shift) % p for x in v if leg(rhs((x - shift) % p)) >= 0]
    plus = sum(1 for x in on if (leg(x) if x else leg(A*A-4)) == 1)
    n = len(on); mino = min(plus, n - plus) / n if n else 0
    ao = tc[s]["aux_order"]; v2 = (ao & -ao).bit_length() - 1
    rows.append(dict(seed=s, rate=r, V=len(v), onfrac=n/len(v), minority=mino, base_points=a0["base_points"],
                     cov_all=a0["covered_points_all_cosets"], columns=a0["columns"], v2=v2, trace=abs(ao-(p+1)), tried=tc[s]["curves_tried"]))
def rank(v):
    o = sorted(range(len(v)), key=lambda i: v[i]); x = [0.0]*len(v); i = 0
    while i < len(o):
        j = i
        while j+1 < len(o) and v[o[j+1]] == v[o[i]]: j += 1
        for k in range(i, j+1): x[o[k]] = (i+j)/2
        i = j+1
    return x
def spear(a, b):
    x, y = rank(a), rank(b); m = (len(x)-1)/2
    c = sum((i-m)*(j-m) for i, j in zip(x, y)); vx = sum((i-m)**2 for i in x); vy = sum((j-m)**2 for j in y)
    return c / (vx*vy) ** 0.5 if vx and vy else float("nan")
rate = [r["rate"] for r in rows]; rnd = random.Random(2026111501)
print("seed rate  |V| onfrac minority base_pts cov_all columns v2 |trace| tried")
for r in rows: print("%3d %.3f %4d %.3f %.3f %4d %5d %4d %2d %5d %4d" % (r["seed"], r["rate"], r["V"], r["onfrac"], r["minority"], r["base_points"], r["cov_all"], r["columns"], r["v2"], r["trace"], r["tried"]))
print("\nfeature            spearman  perm p (20000)   primary")
res = {}
for name, key, prim in [("F1 |V|", "V", 1), ("F2 on-curve frac", "onfrac", 1), ("F3 descent minority", "minority", 1), ("F4 base_points", "base_points", 1),
                        ("F5 cov_all_cosets", "cov_all", 1), ("F5b columns", "columns", 0), ("F6a v2(aux_order)", "v2", 1), ("F6b |aux-(p+1)|", "trace", 1), ("F7 curves_tried", "tried", 0)]:
    f = [r[key] for r in rows]; obs = spear(f, rate); res[key] = obs
    if obs != obs: print("%-20s undefined (constant)" % name); continue
    ge = 0
    for _ in range(20000):
        sh = rate[:]; rnd.shuffle(sh)
        if abs(spear(f, sh)) >= abs(obs) - 1e-12: ge += 1
    print("%-20s %+.3f   %.4f   %s" % (name, obs, (ge+1)/20001, "primary" if prim else "control/aux"))
