"""Amendment 17 reproduction gate: rebuilt rows vs the committed stage A summary (pfd-a, frozen image).

Compares every deterministic field the summary carries, per draw key (p, kind, m, control, t, target_index),
for t <= 8: x_r, closure last fall degree, ffd, falls, closure muladds, closure max cols, F4 D, F4 muladds.
Timing fields are ignored. Exit 0 iff no draw differs and every rebuilt draw has a reference.
"""
import json, sys, glob, os

ref = json.load(open(sys.argv[1]))["draws"]
here = sys.argv[2]
key = lambda r: (r["p"], r["kind"], r["m"], r["control"], r["t"], r["target_index"])
R = {key(d): d for d in ref if d["t"] <= 8}

hky, f4 = {}, {}
for f in glob.glob(os.path.join(here, "*.jsonl")):
    for ln in open(f):
        r = json.loads(ln)
        if "t" not in r or "target_index" not in r: continue
        (hky if os.path.basename(f).startswith("A-") else f4)[key(r)] = r

checks = diffs = 0
missing = []
for k, d in sorted(R.items()):
    h, g = hky.get(k), f4.get(k)
    if h is None or (d.get("D") is not None and g is None):
        missing.append(k); continue
    pairs = [("x_r", d["x_r"], h["x_r"]),
             ("closure_dF", d["closure_dF"], str(h["last_fall_degree"]) if h["last_fall_degree"] is not None else None),
             ("ffd", d["ffd"], h["first_fall_degree"]),
             ("falls", d["falls"], h["falls"]),
             ("closure_muladds", d["closure_muladds"], h["muladds"]),
             ("closure_max_cols", d["closure_max_cols"], max(c["cols"] for c in h["closures"]))]
    if g is not None:
        pairs += [("D", d["D"], g.get("solving_degree_max")), ("f4_muladds", d["f4_muladds"], g.get("muladds")),
                  ("x_r(F4)", d["x_r"], g["x_r"])]
    for name, a, b in pairs:
        checks += 1
        if a != b:
            diffs += 1; print("DIFF", k, name, "ref", a, "rebuilt", b)
print(f"draws {len(R) - len(missing)} / {len(R)} matched-key, checks {checks}, differences {diffs}, missing {len(missing)}")
for k in missing[:10]: print("MISSING", k)
sys.exit(0 if diffs == 0 and not missing else 1)
