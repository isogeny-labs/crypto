#!/usr/bin/env python3
"""Summarise run.sh output into a markdown table (stdout) and summary.json."""
import json
import math
import pathlib
import sys

out_dir = pathlib.Path(sys.argv[1])
rows = []
for f in sorted(out_dir.glob("*.json")):
    if f.name == "summary.json":
        continue
    d = json.loads(f.read_text())
    rho = d.get("rho_reference") or {}
    for r in d["rows"]:
        dec = r["decomposition"]
        nat = dec["cost"]["native"]
        work = dec["cost"].get("work_native", {})
        fibers = nat.get("genroot_fibers")
        muls = work.get("genroot_field_muls")
        rows.append({
            "config": f.stem,
            "instance": d["instance"] if isinstance(d["instance"], str) else d["instance"].get("name"),
            "log2_r": round(r["log2_r"], 2),
            "oracle": dec["name"],
            "m": dec["summands"],
            "fb_abscissae": r["factor_base"].get("abscissae"),
            "columns": r["factor_base"].get("columns"),
            "verified": r["verified"],
            "relations": dec["relations_found"],
            "hit_rate": dec["hit_rate"],
            "S_counted": r["s"],
            "rho_S": rho.get("mean_s"),
            "S_over_rho": r["s_over_rho"],
            "counted_S_over_rho": (r["s"] / rho["mean_s"]) if rho.get("mean_s") else None,
            "wall_s": r["wall_seconds"],
            "genroot_fibers": fibers,
            "genroot_field_muls_per_fiber": (muls / fibers) if fibers else None,
            "genroot_unknown": nat.get("genroot_unknown"),
            "cost_qualification": r.get("cost_qualification"),
        })
(out_dir / "summary.json").write_text(json.dumps(rows, indent=1))


def fmt(v, nd=2):
    if v is None:
        return "–"
    if isinstance(v, bool):
        return "yes" if v else "NO"
    if isinstance(v, float):
        return f"{v:.{nd}f}" if abs(v) < 1e5 else f"{v:.3g}"
    return str(v)


print("| config | log2 r | oracle | m | |F_x| | verified | rel. | S counted | rho S | S/rho (priced) | counted S/rho | wall s | fibers | field muls/fiber | unknown |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for x in rows:
    print("| " + " | ".join(fmt(v) for v in [
        x["config"], x["log2_r"], x["oracle"], x["m"], x["fb_abscissae"], x["verified"],
        x["relations"], x["S_counted"], x["rho_S"], x["S_over_rho"], x["counted_S_over_rho"],
        x["wall_s"], x["genroot_fibers"], x["genroot_field_muls_per_fiber"], x["genroot_unknown"],
    ]) + " |")
