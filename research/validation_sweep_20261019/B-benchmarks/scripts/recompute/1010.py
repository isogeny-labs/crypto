import json, glob, os
D = "research/pdp_split_dreg_20261011/"
a = json.load(open(D + "ANALYSIS.json"))
print("cells", len(a["cells"]), "replica_runs_compared", a["replica_runs_compared"], "replica_runs_mismatched", a["replica_runs_mismatched"], "projection_rule", a["projection_rule"][:200] if isinstance(a["projection_rule"], str) else a["projection_rule"])
rows = []
for c in a["cells"]:
    for v in c["variants"]:
        p = v.get("projection")
        if not p: continue
        rows.append((c["file"], c["curve_slug"], v["plan"], v["g"], p["s_relations"], c["rho_strong"]["mean_s"], c["rho_signed_frobenius"]["mean_s"], p["s_relations"] / c["rho_strong"]["mean_s"], p["s_relations"] / c["rho_signed_frobenius"]["mean_s"], p["over_strong_rho_s"], p["over_signed_frobenius_rho_s"]))
for r in rows:
    print([round(x, 4) if isinstance(x, float) else x for x in r])
print("min ratio over strong rho:", round(min(r[7] for r in rows), 3), "at", [r[0] for r in rows if abs(r[7]-min(x[7] for x in rows))<1e-12])
print("min ratio over signed-frobenius rho:", round(min(r[8] for r in rows), 3), "at", [r[0] for r in rows if abs(r[8]-min(x[8] for x in rows))<1e-12])
# the card row for 3-1-11-4 unsplit g=4 (table in README)
print("runs-v2 jsonl record counts:")
tot = 0
for f in sorted(glob.glob(D + "runs-v2/*.jsonl")):
    n = sum(1 for _ in open(f)); tot += n
    print(os.path.basename(f), n)
print("total runs-v2 records", tot)
