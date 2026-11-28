#!/usr/bin/env python3
"""Validator's n29 characterisation (session G1)."""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import recompute as rc
SCR = pathlib.Path("/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/kf")
recs = rc.load("G1", SCR / "fresh" / "G1")
out = {"band": rc.band(recs), "rows": {}}
for wi, w in enumerate(["0", "0.1", "1"]):
    row = {}
    for a, b in [("lazy-c1", "kf3-c1g-sr"), ("gen-layout-c1", "kf3-c1g-sr"), ("gen-lazy-c1-sr", "kf3-c1g-sr"),
                 ("rho-strong", "lazy-c1"), ("rho-frob-v5", "lazy-c1"), ("rho-strong", "rho-frob-v5"),
                 ("rho-frob-v5", "gen-layout-c1"), ("rho-strong", "rho-strong-aa")]:
        row[f"{b}_over_{a}"] = rc.ratio(recs, a, b, wi=wi, seed=401 + wi)
    out["rows"][w] = row
pathlib.Path(sys.argv[1]).write_text(json.dumps(out, indent=1, default=list))
for a, v in out["band"].items():
    print(f"  {a:16s} {v['verified']}/{v['runs']} S={['%.3f' % x for x in v['mean_S']]} trials={v['mean_trials']:.1f}")
for w, row in out["rows"].items():
    for k, r in row.items():
        print(f"w={w} {k:30s} {r['ratio']:.3f} [{r['ci95_linearised_t'][0]:.3f}, {r['ci95_linearised_t'][1]:.3f}] (single session: within-workload clusters K={r['clusters']})")
