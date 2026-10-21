#!/usr/bin/env python3
"""Smoke workload for tools/expspec: deterministic, seconds long, no claims.

  run.py host --out DIR          record the host
  run.py measure --n N --out F   triangular number of N as JSON
  run.py summarize --dir DIR --out F
"""
import argparse, json, os, platform, sys
from pathlib import Path

p = argparse.ArgumentParser(); s = p.add_subparsers(dest="cmd", required=True)
h = s.add_parser("host"); h.add_argument("--out", required=True)
m = s.add_parser("measure"); m.add_argument("--n", type=int, required=True); m.add_argument("--out", required=True)
z = s.add_parser("summarize"); z.add_argument("--dir", required=True); z.add_argument("--out", required=True)
a = p.parse_args()
if a.cmd == "host":
    Path(a.out).mkdir(parents=True, exist_ok=True)
    (Path(a.out) / "host.json").write_text(json.dumps({"python": sys.version.split()[0], "machine": platform.machine(), "cpus": os.cpu_count()}) + "\n")
elif a.cmd == "measure":
    Path(a.out).parent.mkdir(parents=True, exist_ok=True)
    Path(a.out).write_text(json.dumps({"n": a.n, "value": a.n * (a.n + 1) // 2, "ok": True}) + "\n")
    print(json.dumps({"cell_done": a.n}))
else:
    rows = [json.loads(f.read_text()) for f in sorted(Path(a.dir).glob("cell-*.json"))]
    Path(a.out).write_text(json.dumps({"cells": len(rows), "all_ok": all(r["ok"] for r in rows)}) + "\n")
