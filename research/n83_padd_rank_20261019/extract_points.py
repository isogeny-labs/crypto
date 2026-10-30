#!/usr/bin/env python3
"""Write points.json (per-point recorded values) from a run.py output directory.
Usage: extract_points.py <results_dir> <out.json>. Records values only."""
import json, sys
from pathlib import Path

res, out = Path(sys.argv[1]), Path(sys.argv[2])
src = json.loads((res / "source.json").read_text())
rows = []
for p in range(1, 21):
    row = {"point": p, "order": src["schedule"][p - 1]["ic_order"],
           "rho_first": src["schedule"][p - 1]["rho_first"], "arms": {}, "rho_online_ms": []}
    for arm in ("cur_a", "cur_b", "padd"):
        d = res / "runs" / f"p{p}_{arm}"
        u = json.loads((d / "usage.json").read_text())
        s = next(json.loads(l) for l in (d / "stdout.jsonl").read_text().splitlines()
                 if l.startswith("{") and '"compact_orbit_dlp_summary"' in l)
        t = s["timing_ms"]
        row["arms"][arm] = {"cold_s": u["wall_seconds"], "rank_s": t["rank_stage"] / 1000,
                            "setup_s": t["setup_total"] / 1000, "peak_rss_bytes": u.get("peak_rss_bytes"),
                            "rank_solve": s.get("rank_solve", "s3 (field absent)")}
    for d in range(src["rho_draws"]):
        r = json.loads((res / "runs" / f"p{p}_rho{d}" / "receipt.json").read_text())
        row["rho_online_ms"].append(r["online_ms"])
    rows.append(row)
out.write_text(json.dumps({"source": {k: src[k] for k in ("k", "rank_seed", "binary_sha256", "rho_binary_sha256", "runner_sha256", "targets_sha256")}, "points": rows}, indent=1) + "\n")
print(len(rows), "points")
