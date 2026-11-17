#!/usr/bin/env python3
"""Analysis plumbing (python3 stdlib only): raw.jsonl of run_perf.sh -> RESULT.json.

    analyze.py RUN_DIR [COMMITTED_CALLGRIND_RAW.jsonl]

I_col = instructions / columns (closure), I_step = instructions / walk_steps
(rho), K = I_col / I_step, per draw and per (cell, degree) as median, min, max.
Controls of PROTOCOL_AMENDMENT_A1_perf.md section 5 are evaluated here:
A/A spread within 0.5%, degree-3 perf instructions within 5% of the committed
Callgrind Ir, interval strictly below the whole process, no multiplexing.
A run is `reportable` only if every control passes and it is not a selftest.
"""
import json, os, re, statistics as st, sys
from collections import defaultdict

run = sys.argv[1]
cg_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "raw.jsonl")
recs = [json.loads(l) for l in open(os.path.join(run, "raw.jsonl")) if l.strip()]
host = next((r for r in recs if r.get("kind") == "host"), {})
if host.get("status") == "unsupported":
    json.dump({"status": "unsupported", "host": host, "reportable": False}, open(os.path.join(run, "RESULT.json"), "w"), indent=1)
    print("unsupported host: no RESULT")
    sys.exit(3)
ok = [r for r in recs if r.get("kind") != "host" and r.get("status") == "ok"]
failed = [r for r in recs if r.get("kind") != "host" and r.get("status") != "ok"]
selftest = any(r.get("selftest") for r in ok)

def spread(v):
    return (max(v) - min(v)) / st.median(v) if v and st.median(v) else None

def stats(v):
    return dict(median=st.median(v), min=min(v), max=max(v), n=len(v)) if v else None

# ---- rho: I_step per field, per seed (median over repeats), then over seeds ----
rho_jobs = defaultdict(list)  # (n, seed) -> [instr/step]
for r in ok:
    if r["mode"] == "rho":
        rho_jobs[(r["n"], r["seed"])].append(r["instructions"] / r["walk_steps"])
I_step = {}
for n in (61, 83):
    per_seed = {s: v for (nn, s), v in rho_jobs.items() if nn == n}
    if per_seed:
        meds = [st.median(v) for v in per_seed.values()]
        I_step[f"n{n}"] = dict(per_seed={str(s): st.median(v) for s, v in per_seed.items()},
                               aa_spread={str(s): spread(v) for s, v in per_seed.items()}, **stats(meds))

# ---- closure: I_col per draw (median over repeats) ----
jobs = defaultdict(list)  # (label, degree, draw) -> records
for r in ok:
    if r["mode"] == "closure":
        jobs[(r["label"], r["degree"], r["draw"])].append(r)
draws, cells = [], defaultdict(list)
for (lab, deg, d), v in sorted(jobs.items()):
    cols = {x["columns"] for x in v}
    ops = {x["closure_word_ops"] for x in v}
    ins = [x["instructions"] for x in v]
    row = dict(label=lab, degree=deg, draw=d, columns=v[0]["columns"], n_vars=v[0]["n_vars"],
               engine_counters_constant_over_repeats=len(cols) == 1 and len(ops) == 1,
               closure_word_ops=v[0]["closure_word_ops"], refuted=v[0]["refuted"],
               instructions=ins, cycles=[x["cycles"] for x in v], secs=[x["secs"] for x in v],
               vmhwm_kb=max(x["vmhwm_kb"] for x in v), aa_spread=spread(ins),
               I_col=st.median(ins) / v[0]["columns"], I_word=st.median(ins) / v[0]["closure_word_ops"])
    for den, s in I_step.items():
        row[f"K_{den}"] = row["I_col"] / s["median"]
    draws.append(row)
    cells[(lab, deg)].append(row)
cell_out = []
for (lab, deg), rows in sorted(cells.items()):
    c = dict(label=lab, degree=deg, columns=sorted({r["columns"] for r in rows}), draws=[r["draw"] for r in rows],
             I_col=stats([r["I_col"] for r in rows]))
    for den in I_step:
        c[f"K_{den}"] = stats([r[f"K_{den}"] for r in rows])
    cell_out.append(c)

# ---- controls ----
aa = [r["aa_spread"] for r in draws if r["aa_spread"] is not None] + \
     [x for s in I_step.values() for x in s["aa_spread"].values() if x is not None]
ctl = {"aa_max_spread": max(aa) if aa else None, "aa_limit": 0.005}
ctl["aa_pass"] = bool(aa) and max(aa) <= 0.005 and len(aa) > 0
ctl["engine_counters_constant"] = all(r["engine_counters_constant_over_repeats"] for r in draws)

# positive control: degree-3 perf instructions against the committed Callgrind Ir
cg = {}
if os.path.exists(cg_path):
    for l in open(cg_path):
        o = json.loads(l)
        if o["kind"] == "closure" and o["toggle"] and o["rep"] == 0:
            cg[(o["cell"], o["draw"])] = o
ratios = []
for r in draws:
    c = cg.get((r["label"], r["draw"]))
    if r["degree"] == 3 and c:
        ratios.append(dict(label=r["label"], draw=r["draw"], perf_instructions=st.median(r["instructions"]),
                           callgrind_ir=c["ir"], ratio=st.median(r["instructions"]) / c["ir"],
                           columns_match=c["columns"] == r["columns"], word_ops_match=c["word_ops"] == r["closure_word_ops"]))
ctl["deg3_vs_callgrind"] = ratios
ctl["pos_pass"] = bool(ratios) and all(abs(x["ratio"] - 1) <= 0.05 and x["columns_match"] and x["word_ops_match"] for x in ratios)
rho_cg = defaultdict(list)
for l in open(cg_path) if os.path.exists(cg_path) else []:
    o = json.loads(l)
    if o["kind"] == "rho" and o["rep"] == 0:
        rho_cg[o["cell"]].append(o["ir"] / o["walk_steps"])
ctl["rho_perf_over_callgrind_info"] = {k: I_step[k]["median"] / st.median(v) for k, v in rho_cg.items() if k in I_step}

# null control: interval strictly inside the whole process (tools/perf_ipc.sh runs)
null = []
wp = os.path.join(run, "null_whole.jsonl")
if os.path.exists(wp):
    for l in open(wp):
        o = json.loads(l)
        if o.get("kind") != "run" or "whole-" not in o.get("label", ""):
            continue
        lab = o["label"].split("whole-")[1]  # <cell>-d<draw>-deg<degree>
        cell, draw, deg = re.match(r"(.+)-d(\d+)-deg(\d+)$", lab).groups()
        mine = [y["instructions"] for y in jobs.get((cell, int(deg), int(draw)), [])]
        iv = st.median(mine) if mine else None
        whole = (o.get("counters") or {}).get("instructions")
        null.append(dict(label=lab, interval=iv, whole=whole, interval_over_whole=(iv / whole) if iv and whole else None,
                         strictly_inside=bool(iv and whole and iv < whole)))
ctl["null_whole_process"] = null
ctl["null_pass"] = bool(null) and all(x["strictly_inside"] for x in null)
ctl["no_multiplexing"] = not any(r.get("multiplexed") for r in ok)
ctl["no_failed_jobs"] = not failed
ctl["all_pass"] = all(ctl[k] for k in ("aa_pass", "pos_pass", "null_pass", "no_multiplexing", "no_failed_jobs", "engine_counters_constant"))

deg4 = sorted(({"label": r["label"], "draw": r["draw"], "columns": r["columns"], "I_col": r["I_col"],
                **{k: v for k, v in r.items() if k.startswith("K_")}} for r in draws if r["degree"] == 4), key=lambda x: x["columns"])
out = dict(status="ok", selftest=selftest, reportable=bool(ctl["all_pass"] and not selftest), host=host,
           unit="instructions:u (perf_event_open interval counters); cycles are in raw.jsonl and are not used in K",
           I_step=I_step, cells=cell_out, draws=draws, deg4_growth_by_columns=deg4, controls=ctl,
           failed=[r.get("tag") for r in failed])
json.dump(out, open(os.path.join(run, "RESULT.json"), "w"), indent=1)
print(f"RESULT.json written: reportable={out['reportable']} selftest={selftest}; controls: " +
      ", ".join(f"{k}={ctl[k]}" for k in ("aa_pass", "pos_pass", "null_pass", "no_multiplexing", "no_failed_jobs")))
for c in cell_out:
    k = c.get("K_n83") or c.get("K_n61")
    if k:
        print(f"  {c['label']} deg {c['degree']} cols {c['columns']}: K median {k['median']:.3f} [{k['min']:.3f}, {k['max']:.3f}]")
