#!/usr/bin/env python3
"""Validator comparison of the E31 replays with the committed records (stdlib only).

End to end: every replay/e2e/e2e_<solver>_p<p>s<s>.json against experiments/31_solve_cost/ (E31
solvers) or experiments/30_gls_v4/ (the macaulay A/A control against E25), leaf by leaf, integers
and floats exactly; wall_ms and solve_ms are wall-clock, reported and not compared.
Stage A: replay/stage_a/stage_a_replay.json against experiments/31_solve_cost/stage_a.json, every
per-call array leaf exactly (wall_us and the wall summaries reported, not compared).
Writes replay/replay_comparison.json.

    python3 benchmarks/20261024-e31-chain-solve/replay/replay_compare.py
"""
import glob, hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.normpath(os.path.join(HERE, "..", "..", ".."))
TIMING = {"wall_ms", "solve_ms"}
SA_TIMING = {"wall_us", "wall_us_median", "wall_us_mean"}


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def leaves(x, pre=""):
    if isinstance(x, dict):
        for k, v in x.items():
            yield from leaves(v, f"{pre}.{k}" if pre else k)
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from leaves(v, f"{pre}[{i}]")
    else:
        yield pre, x


def top(path):
    return re.split(r"[.\[]", path.split("].", 1)[-1] if path.startswith("[") else path)[0]


def last_key(path):
    return re.sub(r"\[\d+\]", "", path).split(".")[-1]


runs = []
for f in sorted(glob.glob(os.path.join(HERE, "e2e", "e2e_*_p*s*.json"))):
    m = re.search(r"e2e_(macaulay|chain-sparsest|chain)_p(\d+)s(\d+)\.json$", f)
    solver, p, s = m.group(1), int(m.group(2)), int(m.group(3))
    committed = (f"experiments/30_gls_v4/p{p}s{s}.json" if solver == "macaulay"
                 else f"experiments/31_solve_cost/e2e_{solver}_p{p}s{s}.json")
    crec = json.load(open(os.path.join(REPO, committed)))
    rrec = json.load(open(f))
    for c, r in zip(crec, rrec):
        lc, lr = dict(leaves(c)), dict(leaves(r))
        compared = [k for k in lc if last_key(k) not in TIMING]
        mism = [{"field": k, "committed": lc[k], "replay": lr.get(k, "<absent>")} for k in compared if lr.get(k, object()) != lc[k]]
        added = sorted(set(lr) - set(lc))
        runs.append({
            "experiment": "e31" if solver != "macaulay" else "e25 A/A control through the E31 dispatch",
            "solver": solver, "p": p, "seed": s, "arm": c["arm"],
            "committed": {"path": committed, "sha256": sha(os.path.join(REPO, committed))},
            "replay_output": {"path": os.path.relpath(f, os.path.dirname(HERE)), "sha256": sha(f)},
            "replayed": True, "fields_compared": len(compared), "fields_identical": len(compared) - len(mism),
            "mismatches": mism, "fields_added_in_replay": added,
            "timing_not_compared": {k: {"committed": c.get(k), "replay": r.get(k)} for k in sorted(TIMING)},
            "matched": not mism and not added and len(crec) == len(rrec),
            "logarithm": {"replay_solved": r["folded"].get("solved"), "replay_correct": r["folded"]["correct"],
                          "walks_correct": all(w["correct"] for w in r["rho"])},
            "key_scalars": {k: r[k] for k in ("total_muls", "ratio", "residuals", "unsolved", "reruns", "decompositions",
                                              "macaulay_entries", "rho_matched_muls", "solver_fallbacks") if k in r},
        })

stage_a = None
fa = os.path.join(HERE, "stage_a", "stage_a_replay.json")
if os.path.exists(fa):
    com = {(c["p"], c["seed"]): c for c in json.load(open(os.path.join(REPO, "experiments/31_solve_cost/stage_a.json")))}
    rep = json.load(open(fa))
    curves = []
    for r in rep:
        c = com[(r["p"], r["seed"])]
        lc, lr = dict(leaves(c)), dict(leaves(r))
        compared = [k for k in lc if last_key(k) not in SA_TIMING]
        mism = [k for k in compared if lr.get(k, object()) != lc[k]]
        wall = {a["solver"]: {"committed_median": cs["wall_us_median"], "replay_median": rs["wall_us_median"]}
                for a, cs, rs in zip(c["arms"], c["summaries"], r["summaries"])}
        curves.append({"p": r["p"], "seed": r["seed"], "calls": r["calls"], "fields_compared": len(compared),
                       "fields_identical": len(compared) - len(mism), "mismatches": mism[:50],
                       "agree": r["agree"], "quotient_agree": r["quotient_agree"], "quotient_checked": r["quotient_checked"],
                       "fallbacks": [a["fallbacks"] for a in r["arms"]],
                       "C_median": [s["call_median"] for s in r["summaries"]],
                       "wall_not_compared": wall, "matched": not mism})
    stage_a = {"committed": {"path": "experiments/31_solve_cost/stage_a.json",
                             "sha256": sha(os.path.join(REPO, "experiments/31_solve_cost/stage_a.json"))},
               "replay_output": {"path": "replay/stage_a/stage_a_replay.json", "sha256": sha(fa)},
               "curves_replayed": len(curves), "curves_matched": sum(c["matched"] for c in curves), "curves": curves}

out = {"e2e_runs_total": len(runs), "e2e_runs_matched": sum(r["matched"] for r in runs), "e2e_runs": runs, "stage_a": stage_a}
with open(os.path.join(HERE, "replay_comparison.json"), "w") as f:
    json.dump(out, f, indent=1, ensure_ascii=False)
    f.write("\n")
print(f"e2e: {out['e2e_runs_matched']}/{out['e2e_runs_total']} arm records identical (timing excluded)")
for r in runs:
    print(f"  {r['solver']:>14} p={r['p']} s={r['seed']} arm {r['arm']}: {r['fields_identical']}/{r['fields_compared']}"
          f"{'' if r['matched'] else '  MISMATCH ' + json.dumps(r['mismatches'][:3]) + ' added ' + str(r['fields_added_in_replay'])}")
if stage_a:
    print(f"stage A: {stage_a['curves_matched']}/{stage_a['curves_replayed']} curves identical (wall excluded)")
    for c in stage_a["curves"]:
        print(f"  p={c['p']} s={c['seed']}: {c['fields_identical']}/{c['fields_compared']} agree {c['agree']} quotient {c['quotient_agree']}/{c['quotient_checked']}"
              f"{'' if c['matched'] else '  MISMATCH ' + str(c['mismatches'][:5])}")
sys.exit(0 if all(r["matched"] for r in runs) and (stage_a is None or stage_a["curves_matched"] == stage_a["curves_replayed"]) else 1)
