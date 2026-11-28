#!/usr/bin/env python3
"""Tabulate the LA / factor-base sweep from recorded JSON lines."""
import json
import sys

rows = [json.loads(l) for l in open(sys.argv[1] if len(sys.argv) > 1 else "sweep.jsonl") if l.strip()]
print("| bits | B | rows | cols | rank | nnz_in | nnz_peak | LA ms | rel s | IC s | LA | ok |")
print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|")
for r in sorted(rows, key=lambda r: (r["bits"], r["fb_size"], r["la_engine"])):
    s = r["ic_stages"]
    la = s["la"]
    ok = r["ic_ok"] and r["ic_matches_truth"] and r["ic_agrees_rho"]
    print(
        f"| {r['bits']} | {r['fb_size']} | {la['rows']} | {la['cols']} | {la['rank']} | "
        f"{la['nnz_in']} | {la['nnz_peak']} | {s['linear_algebra_ms']:.2f} | "
        f"{s['relations_ms']/1e3:.2f} | {r['ic_ms']/1e3:.2f} | "
        f"{'sparse' if 'sparse' in r['la_engine'] else 'dense'} | {'yes' if ok else 'NO'} |"
    )
