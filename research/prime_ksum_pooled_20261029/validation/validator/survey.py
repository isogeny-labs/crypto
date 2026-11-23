#!/usr/bin/env python3
"""Schema survey: per session x arm, which unpriced counters, phase natives,
work_native keys/markers, calibration and solver fields appear."""
import json, sys
from collections import defaultdict, Counter
from pathlib import Path
root = Path(sys.argv[1])
for s in sorted(p.name for p in root.iterdir() if p.is_dir()):
    info = defaultdict(lambda: defaultdict(Counter))
    with open(root / s / "records.jsonl") as f:
        for line in f:
            r = json.loads(line)
            a = r["arm"]; I = info[a]
            I["n"]["all"] += 1
            I["n"]["warmup" if r["warmup"] else "measured"] += 1
            I["status"][r["outcome"]["status"]] += 1
            for u in r["cost"]["unpriced"]: I["unpriced"][u] += 1
            I["lower_bound"][str(r["cost"].get("lower_bound"))] += 1
            for p in r["phases"]:
                for k in p.get("native", {}): I["phase_native"][k] += 1
                for k in p.get("work_native", {}) or {}: I["work_native"][k] += 1
                for k in p.keys(): I["phase_keys"][k] += 1
            I["detail_keys_calib"][str([k for k in (r.get("detail") or {}) if "calib" in k])] += 1
            I["solver"][str(type(r.get("solver")).__name__)] += 1
            I["method"][r["method"]["id"] + " " + json.dumps(r["method"]["params"], sort_keys=True)] += 1
    print("=====", s)
    for a, I in info.items():
        print(" arm", a)
        for k, c in I.items():
            print("   ", k, dict(c))
