#!/usr/bin/env python3
"""Summarise `ecbench verify --replay-all` reports: usage python3 analysis/verify_summary.py <dir with *.verify.json> > out.txt"""
import json, sys, glob, os
for f in sorted(glob.glob(os.path.join(sys.argv[1], "*.verify.json"))):
    d = json.load(open(f)); rp = d.get("replays", [])
    print(os.path.basename(f), "ok" if d.get("ok") else "FAIL", f"records={d.get('records')}", f"replays={len(rp)}",
          f"reproduced={sum(1 for r in rp if r.get('reproduced'))}", f"problems={len(d.get('problems', []))}")
