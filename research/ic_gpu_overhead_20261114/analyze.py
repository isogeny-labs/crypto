#!/usr/bin/env python3
"""Tabulate the ABBA overhead A/B (out_ab/<sweep>-<arm>-<k>.json under DIR) into RESULTS.md and
results.json. Identity first: counted S, calls and hit rate must be equal in every run of a sweep.
Then per arm: solver wall per call, worker round trip per call, the rest of the solver call, and the
whole-solve wall (practicality note, AGENTS.md section 6). Values are read from the bench JSON;
means and ratios are derived here.
    analyze.py DIR [OUTDIR ARM_A ARM_B [RESULTS_STEM]]   (defaults: out_ab old new RESULTS)
"""
import json, statistics, sys
from pathlib import Path

D = Path(sys.argv[1]); o = D / (sys.argv[2] if len(sys.argv) > 2 else "out_ab")
A, B = (sys.argv[3], sys.argv[4]) if len(sys.argv) > 4 else ("old", "new")
STEM = sys.argv[5] if len(sys.argv) > 5 else "RESULTS"
md, res = [], {}
md.append("GPU: " + next((l.split(":", 1)[1].strip() for l in (o / "nvidia-smi-q.txt").read_text().splitlines() if "Product Name" in l), "?"))
md.append("\n```\n" + (o / "revs.txt").read_text().strip() + "\n" + (o / "sha256.txt").read_text().strip() + "\n" + (o / "serve-smoke.txt").read_text().strip() + "\n```")
for s in ("koblitz-n23", "char2-n25", "char2-n29"):
    runs = sorted(o.glob(f"{s}-*-*.json"))
    if not runs:
        md.append(f"\n## {s}: no output\n"); continue
    per = {}  # (solver) -> arm -> list of dicts
    for f in runs:
        _, arm, k = f.stem.rsplit("-", 2)
        d = json.loads(f.read_text())
        seen = {}
        for r in d["rows"]:
            sol = (r.get("decomposition") or {}).get("solver") or {}
            name = sol.get("name")
            if not name:
                continue
            rep = seen.get(name, 0); seen[name] = rep + 1
            ex = sol.get("extra") or {}
            calls = sol.get("calls", 0)
            rt = ex.get("gpu_round_trip_ns")
            per.setdefault(name, {}).setdefault(arm, []).append({
                "run": int(k), "repeat": rep, "ops": sol.get("ops"), "priced_by": sol.get("priced_by"),
                "verified": r.get("verified"), "s": r.get("s"), "s_over_rho": r.get("s_over_rho"),
                "calls": calls, "hit": (r.get("decomposition") or {}).get("hit_rate"),
                "solver_ms_per_call": sol.get("wall_ns", 0) / max(1, calls) / 1e6,
                "round_trip_ms_per_call": (rt / max(1, calls) / 1e6) if rt is not None else None,
                "row_wall_s": r.get("wall_seconds"),
                "gpu_fallback": ex.get("gpu_fallback"), "gpu_capped": ex.get("gpu_capped"),
            })
    md.append(f"\n## {s}\n")
    md.append("| solver | arm | runs | verified | S (all runs) | calls | solver ms/call (each run) | round trip ms/call | rest ms/call | whole solve s (each run) |")
    md.append("|:--|:--|--:|:--|:--|:--|:--|:--|:--|:--|")
    res[s] = {}
    for name, arms in per.items():
        allruns = [x for a in arms.values() for x in a]
        # identity per repeat (each repeat is its own target): counted ops, calls, hit rate.
        reps = sorted({x["repeat"] for x in allruns})
        same = all(len({(x["ops"], x["calls"], round(x["hit"], 9)) for x in allruns if x["repeat"] == i}) == 1 for i in reps)
        res[s][name] = {"identical_across_runs": same, "arms": arms}
        for arm in (A, B):
            xs = sorted(arms.get(arm, []), key=lambda x: x["run"])
            if not xs:
                continue
            rts = [x["round_trip_ms_per_call"] for x in xs if x["round_trip_ms_per_call"] is not None]
            rest = [x["solver_ms_per_call"] - x["round_trip_ms_per_call"] for x in xs if x["round_trip_ms_per_call"] is not None]
            md.append(f"| `{name}` | {arm} | {len(xs)} | {'y' if all(x['verified'] for x in xs) else 'NO'} | "
                      f"{', '.join(f'{x['s']:.2f}' for x in xs)} | {', '.join(str(x['calls']) for x in xs)} | "
                      f"{', '.join(f'{x['solver_ms_per_call']:.3f}' for x in xs)} | {', '.join(f'{v:.3f}' for v in rts) or '—'} | "
                      f"{', '.join(f'{v:.3f}' for v in rest) or '—'} | {', '.join(f'{x['row_wall_s']:.2f}' for x in xs)} |")
        if A in arms and B in arms:
            # paired by repeat: mean over the runs of each arm, ratio per repeat, then over repeats
            def by_rep(a, key):
                return {i: statistics.mean(x[key] for x in arms[a] if x["repeat"] == i) for i in reps}
            so, sn = by_rep(A, "solver_ms_per_call"), by_rep(B, "solver_ms_per_call")
            wo, wn = by_rep(A, "row_wall_s"), by_rep(B, "row_wall_s")
            rto = {i: statistics.mean(x["round_trip_ms_per_call"] for x in arms[A] if x["repeat"] == i and x["round_trip_ms_per_call"] is not None) for i in reps} if all(x["round_trip_ms_per_call"] is not None for x in allruns) else None
            rtn = {i: statistics.mean(x["round_trip_ms_per_call"] for x in arms[B] if x["repeat"] == i and x["round_trip_ms_per_call"] is not None) for i in reps} if rto else None
            aa = max(max(x["solver_ms_per_call"] for x in arms[a] if x["repeat"] == i) /
                     min(x["solver_ms_per_call"] for x in arms[a] if x["repeat"] == i) for a in (A, B) for i in reps)
            ratios = [so[i] / sn[i] for i in reps]
            wr = [wo[i] / wn[i] for i in reps]
            s_spread = {a: (min(x["s"] for x in arms[a]), max(x["s"] for x in arms[a])) for a in (A, B)}
            res[s][name].update({"solver_ms_per_call_old": so, "solver_ms_per_call_new": sn, "row_wall_old": wo,
                                 "row_wall_new": wn, "speedup_per_repeat": ratios, "wall_speedup_per_repeat": wr,
                                 "round_trip_ms_per_call_a": rto, "round_trip_ms_per_call_b": rtn,
                                 "arms": [A, B], "within_arm_max_ratio": aa, "priced_by": sorted({x["priced_by"] for x in allruns})})
            md.append(f"\n- derived, `{name}`: counted ops, calls and hit rate {'identical per repeat in every run' if same else 'DIFFER'}"
                      f" (S priced `{', '.join(sorted({x['priced_by'] for x in allruns}))}`); solver per call, by repeat: "
                      + "; ".join(f"{so[i]:.3f} → {sn[i]:.3f} ms ({ratios[i]:.2f}×)" for i in reps)
                      + "; whole solve: " + "; ".join(f"{wo[i]:.2f} → {wn[i]:.2f} s ({wr[i]:.2f}×)" for i in reps)
                      + (("; round trip: " + "; ".join(f"{rto[i]:.3f} → {rtn[i]:.3f} ms" for i in reps)) if rto else "")
                      + f"; largest same-arm, same-repeat max/min {aa:.3f}.")
(D / f"{STEM.lower()}.json").write_text(json.dumps(res, indent=1))
(D / f"{STEM}.md").write_text("\n".join(md) + "\n")
print("\n".join(md))
