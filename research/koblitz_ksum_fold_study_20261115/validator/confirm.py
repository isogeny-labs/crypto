#!/usr/bin/env python3
"""Validator's pre-declared confirmation test on F4-F6 (confirm-predeclared.txt)."""
import json, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
import recompute as rc
SCR = pathlib.Path("/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/kf")
labels = ["F4", "F5", "F6"]
recs = []
for l in labels:
    recs += rc.load(l, SCR / "fresh" / l)
out = {"band_pooled": rc.band(recs), "band_per_session": {l: rc.band([r for r in recs if r["session"] == l]) for l in labels}, "rows": {}}
for wi, w in enumerate(["0", "0.1", "1"]):
    row = {}
    for a in ["gen-layout-c4", "gen-lazy-c4-sr", "lazy-c4", "rho-strong", "rho-frob-v5", "rho-strong-v2"]:
        row[f"kf3_over_{a}"] = rc.ratio(recs, a, "kf3-c1g-sr", wi=wi, seed=301 + wi)
        row[f"kf3_over_{a}_per_session"] = {l: rc.ratio(recs, a, "kf3-c1g-sr", sessions={l}, wi=wi, seed=311 + wi) for l in labels}
    row["aa"] = rc.ratio(recs, "rho-strong", "rho-strong-aa", wi=wi, seed=321 + wi)
    row["rho-frob-v5_over_rho-strong"] = rc.ratio(recs, "rho-strong", "rho-frob-v5", wi=wi, seed=331 + wi)
    row["gen-layout-c4_over_lazy-c4"] = rc.ratio(recs, "lazy-c4", "gen-layout-c4", wi=wi, seed=341 + wi)
    out["rows"][w] = row
pathlib.Path(sys.argv[1]).write_text(json.dumps(out, indent=1, default=list))
for a, v in out["band_pooled"].items():
    print(f"  {a:16s} {v['verified']}/{v['runs']} S={['%.3f' % x for x in v['mean_S']]}")
for w, row in out["rows"].items():
    for k, r in row.items():
        if k.endswith("_per_session"):
            print(f"  w={w} {k}", {l: (round(x['ratio'], 3), round(x['ci95_linearised_t'][0], 3), round(x['ci95_linearised_t'][1], 3)) for l, x in r.items()})
        else:
            print(f"w={w} {k:32s} {r['ratio']:.4f} [{r['ci95_linearised_t'][0]:.3f}, {r['ci95_linearised_t'][1]:.3f}] pairs={r['pairs']} same_seeds={r['same_seeds']}")
