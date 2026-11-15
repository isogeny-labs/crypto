#!/usr/bin/env python3
"""Write the validator's compact evidence into validation/ (each file well under 1 MiB):
  native_work.json   per row of rows.jsonl: replay match, H, HashMap calls, rhodp canonicalisations,
                     step-index hashes, DP tests (validator instrumented replay of all 3960 rows)
  sens_ic_runs.json  validator sensitivity panel 1 (ic5rw x5 mu, ic5w, ic5ks x2 mu), counters only
  sens_rho_runs.json validator sensitivity panel 2 (rho2xks at kappa 0.05/0.1/0.25), counters only
  sens_dpu_runs.json validator sensitivity panel 3 (rhodpu: rhodp with distinct steps, rhodp's grid), counters only
usage: compact_evidence.py <scratch dir> <validation dir>"""
import json, sys, os
S, V = sys.argv[1:3]
ra = json.load(open(os.path.join(S, "replay_all.json")))
keep = ("i", "arm", "pb", "T", "idx", "seed", "param", "doff", "match", "ok", "targets", "H", "hop", "canon", "pidx", "dpt")
rr = sorted(ra["runs"], key=lambda r: r["i"])
nw = dict(schema="validator.native-work/v1", source="validation/scripts/replay.py all + patch_instr.py (instrumented build, adds no counted operation)",
          summary=ra["summary"], fields=list(keep), rows=[[r[k] for k in keep] for r in rr])
json.dump(nw, open(os.path.join(V, "native_work.json"), "w"), separators=(",", ":"))
for src, dst in (("sens.json", "sens_ic_runs.json"), ("sens_rho.json", "sens_rho_runs.json"), ("sens_dpu.json", "sens_dpu_runs.json")):
    d = json.load(open(os.path.join(S, src)))
    F = ["arm", "pb", "T", "idx", "seed", "param", "doff", "targets", "ok", "I", "M", "S", "H", "L", "I_targets", "walks", "abandoned", "d"]
    rows = [[r["phases"]["targets"][0] if k == "I_targets" else r.get(k) for k in F] for r in d["rows"]]
    json.dump(dict(meta=d["meta"], argv="[arm, instances/W<pb>-<idx>.json, T, seed, param] + ['4' for ic5* arms | doff for rhodpu]", fields=F, rows=rows),
              open(os.path.join(V, dst), "w"), separators=(",", ":"))
for f in ("native_work.json", "sens_ic_runs.json", "sens_rho_runs.json", "sens_dpu_runs.json"):
    print(f, os.path.getsize(os.path.join(V, f)))
