"""Amendment 17 gate part 2: rebuilt stage C rows (t = 2, m = 3..6) vs committed pfdC_summary.json draws.
Deterministic fields: x_r, curve, d_F lower value, ffd, falls, max cols, muladds. Exit 0 iff all present and equal."""
import json, sys, glob, os
ref = [d for d in json.load(open(sys.argv[1]))["draws"] if d["t"] == 2 and 3 <= d["m"] <= 6]
key = lambda p, kind, m, t, k: (p, kind, m, t, k)
R = {key(d["p"], d["kind"], d["m"], d["t"], d["k"]): d for d in ref}
got = {}
for f in glob.glob(os.path.join(sys.argv[2], "C-t2-m*.jsonl")):
    for ln in open(f):
        r = json.loads(ln)
        if "t" in r and "target_index" in r: got[key(r["p"], r["kind"], r["m"], r["t"], r["target_index"])] = r
checks = diffs = 0; missing = []
for k, d in sorted(R.items()):
    h = got.get(k)
    if h is None: missing.append(k); continue
    dF = h["last_fall_degree"]
    pairs = [("x_r", d["x_r"], h["x_r"]), ("curve", d["curve"], h["curve"]), ("ffd", d["ffd"], h["first_fall_degree"]),
             ("falls", d["falls"], h["falls"]), ("cols_max", d["cols_max"], max(c["cols"] for c in h["closures"])),
             ("muladds", d["muladds"], h["muladds"]),
             ("d_F_lo", d["d_F_lo"], dF if dF is not None else (h["closures"][-1]["c"] + (0 if h["closures"][-1].get("censored") else 1)))]
    for n, a, b in pairs:
        checks += 1
        if a != b: diffs += 1; print("DIFF", k, n, "ref", a, "rebuilt", b)
print(f"draws {len(R) - len(missing)} / {len(R)}, checks {checks}, differences {diffs}, missing {len(missing)}")
for k in missing: print("MISSING", k)
sys.exit(0 if diffs == 0 and not missing else 1)
