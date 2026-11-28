import csv, json, sys, os, glob
orig, rep = sys.argv[1], sys.argv[2]
TIMING = {"wall_ms"}
def rows(path):
    with open(path, newline="") as f:
        r = list(csv.reader(f))
    if not r: return [], []
    hdr = r[0]
    keep = [i for i, h in enumerate(hdr) if h not in TIMING]
    return [hdr[i] for i in keep], [[row[i] if i < len(row) else None for i in keep] for row in r[1:]]
report = {"csv": {}, "json": None}
names = sorted(set(os.path.basename(p) for p in glob.glob(os.path.join(rep, "*.csv"))))
for n in names:
    po, pr = os.path.join(orig, n), os.path.join(rep, n)
    if not os.path.exists(po):
        report["csv"][n] = "MISSING_IN_ORIGINAL"; continue
    if n.startswith("controls_"):
        # controls: raw text compare, timing lines removed if any
        a = [l for l in open(po) if "wall" not in l]; b = [l for l in open(pr) if "wall" not in l]
        report["csv"][n] = {"rows_orig": len(a), "rows_rep": len(b), "equal": a == b}
        continue
    ho, ro = rows(po); hr, rr = rows(pr)
    report["csv"][n] = {"rows_orig": len(ro), "rows_rep": len(rr), "header_equal": ho == hr, "equal": ho == hr and ro == rr,
        "first_diff_row": next((i for i, (x, y) in enumerate(zip(ro, rr)) if x != y), None)}
def canon(o):
    if isinstance(o, dict): return {k: canon(v) for k, v in o.items() if not any(t in k.lower() for t in ("wall", "_ms", "time"))}
    if isinstance(o, list): return [canon(v) for v in o]
    return o
jo = json.load(open(os.path.join(orig, "results.json"))); jr = json.load(open(os.path.join(rep, "results.json")))
co, cr = canon(jo), canon(jr)
report["json"] = {"equal_ignoring_timing": co == cr, "items_orig": len(jo["items"]), "items_rep": len(jr["items"])}
if co != cr:
    diffs = [i for i, (x, y) in enumerate(zip(co["items"], cr["items"])) if x != y]
    report["json"]["differing_item_indices"] = diffs[:20]
    report["json"]["n_differing"] = len(diffs)
report["md_equal_ignoring_timing_lines"] = None
ro_md = [l for l in open(os.path.join(orig,"RESULTS.md")) if "wall" not in l.lower()]
rr_md = [l for l in open(os.path.join(rep,"RESULTS.md")) if "wall" not in l.lower()]
report["md_equal_ignoring_timing_lines"] = ro_md == rr_md
print(json.dumps(report, indent=1))
