#!/usr/bin/env python3
"""Ladder summary: per (scale, target, arm) status, F4 degree reached, wall, peak RSS, relations."""
import json, glob, sys, re, collections
D = sys.argv[1]
rows = []
for p in sorted(glob.glob(f"{D}/s*-*.json")):
    if p.endswith(".meta.json"): continue
    m = re.match(r"s(\d+)-(\w+?)-(\w+)\.json", p.split("/")[-1])
    scale, target, arm = int(m[1]), m[2], m[3]
    meta = json.load(open(p[:-5] + ".meta.json"))
    try:
        d = json.load(open(p))
    except Exception:
        rows.append(dict(scale=scale, target=target, arm=arm, status=f"crash exit={meta['exit']}", degree=None,
                         wall_s=meta["process_wall_s"], rss_mib=meta["max_rss_kib"] / 1024, relations=0, detail="no JSON"))
        continue
    r = d["result"]
    if arm == "coupled":
        status, degree = r["status"], (r.get("traversal") or {}).get("solving_degree")
        rel = r["diagnostics"]["accepted_relations"]; detail = r["reason"][:80]
    elif arm == "sectors":
        secs = r["sectors"]
        status = "+".join(s["status"] for s in secs)
        degree = [s.get("solving_degree") for s in secs]
        rel = sum(len(s.get("relations", [])) for s in secs)
        detail = "; ".join((s.get("resource_limit") or s.get("reason") or "")[:40] for s in secs)
    else:
        status, degree = r["status"], r.get("solving_degree")
        rel = len(r.get("relations", [])); detail = (r.get("resource_limit") or r.get("reason") or "")[:80]
    rows.append(dict(scale=scale, target=target, arm=arm, status=status, degree=degree,
                     variables=r.get("variables") or (r.get("diagnostics") or {}).get("input_variables"),
                     wall_s=round(d["arm_wall_s"], 3), rss_mib=round(meta["max_rss_kib"] / 1024, 1),
                     relations=rel, matrix_cap=d["limits"]["max_matrix_entries"], detail=detail,
                     control=r.get("planted_witness_control")))
print(json.dumps(rows, indent=1))
