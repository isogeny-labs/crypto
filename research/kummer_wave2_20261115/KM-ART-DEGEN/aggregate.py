#!/usr/bin/env python3
import json, sys, collections
src = sys.argv[1]
rows = [json.loads(l) for l in open(src)]
def key(r): return (r["family"], r["p"], r["draw"], r["t"])
main = [r for r in rows if r["kind"] == "main"]
cells = collections.defaultdict(dict)
for r in main: cells[key(r)][r["eps"]] = r
fmt = lambda r: "-" if r is None else (">=%d" % (r["cmax"] + 1) if r["dF"] is None and r["censored"] else ("skip" if r["dF"] is None else str(r["dF"])))
out = []
out.append("| family | t | p | draw | dF eps=1 | dF eps=0 | dF eps=5 | ffd 1/0/5 | eq(0,1) |")
out.append("|---|--:|--:|--:|--:|--:|--:|:--|:--|")
nd = neq = 0; maxdiff = 0
for k in sorted(cells, key=lambda k: (k[0], k[3], k[1], k[2])):
    c = cells[k]; r1, r0, r5 = c.get(1), c.get(0), c.get(5)
    eq = "-"
    if r1 and r0 and r1["dF"] is not None and r0["dF"] is not None:
        nd += 1; d = abs(r1["dF"] - r0["dF"]); maxdiff = max(maxdiff, d); eq = "yes" if d == 0 else "NO (%+d)" % (r0["dF"] - r1["dF"]); neq += (d == 0)
    ff = "/".join(str(x["ffd"]) if x else "-" for x in (r1, r0, r5))
    out.append(f"| {k[0]} | {k[3]} | {k[1]} | {k[2]} | {fmt(r1)} | {fmt(r0)} | {fmt(r5)} | {ff} | {eq} |")
out.append("")
out.append(f"Comparable (both finite) cells eps=0 vs eps=1: {nd}; equal: {neq}; max |diff|: {maxdiff}")
for kind in ("planted", "twist_eps_sK", "twist_ref_eps1_fs"):
    rs = [r for r in rows if r["kind"] == kind]
    if not rs: continue
    out.append(f"\n{kind}: {len(rs)} cells")
    for r in sorted(rs, key=lambda r: (r["p"], r["draw"], r["t"], r["eps"])):
        out.append(f"- t={r['t']} p={r['p']} draw={r['draw']} eps={r['eps']} dF={fmt(r)} ffd={r['ffd']} censored={r['censored']}" + (f" s={r['s']}" if "s" in r else ""))
print("\n".join(out))
