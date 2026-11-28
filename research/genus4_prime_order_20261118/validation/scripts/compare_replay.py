#!/usr/bin/env python3
"""C4b: compare the validator's replays (out/replay/, validator rebuild) with the committed rows (git blobs at
COMMIT).  Run rows by (arm, curve_seed, target_seed, seed), instance rows by curve_seed, cell rows per search
chunk; every field must be equal except wall_ns / search_wall_s / wall_s.  The expected row count of every
planned cell is checked, so a missing replay row fails instead of passing vacuously.
Also C2: the library's f_irreducible on every seed (h_irr.jsonl) against the validator's DDF.
Writes out/replay_compare.json and out/prefilter_rust_vs_validator.json."""
import os, sys, random
sys.path.insert(0, os.path.dirname(__file__))
from common import *
from select_check import seed_list
from jac import random_f
from polyfp import reducible_certificate

R = os.path.join(OUT, "replay")
WALL = {"wall_ns", "search_wall_s", "wall_s"}
plan = json.load(open(os.path.join(OUT, "plan.json")))
res = {"cells": [], "fail": []}


def strip(r):
    return {k: v for k, v in r.items() if k not in WALL}


def jl(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def committed(name):
    return rows_of(f"{D}/{name}")[0]


def cmp_runs(tag, replay_rows, comm_rows, expect_keys):
    rk = lambda r: (r["arm"], r["curve_seed"], r["target_seed"], r["seed"])
    rep = {rk(r): r for r in replay_rows if r["kind"] == "run"}
    com = {rk(r): r for r in comm_rows if r["kind"] == "run"}
    ri = {r["curve_seed"]: r for r in replay_rows if r["kind"] == "instance"}
    ci = {r["curve_seed"]: r for r in comm_rows if r["kind"] == "instance"}
    diffs = []
    for k, r in rep.items():
        if k not in com:
            diffs.append(f"{k} not in committed")
        elif strip(r) != strip(com[k]):
            diffs.append(f"{k}: " + ", ".join(f for f in set(r) | set(com[k]) if f not in WALL and r.get(f) != com[k].get(f)))
    inst_eq = all(ri[c] == ci[c] for c in ri) and len(ri) == 1
    cell = {"tag": tag, "rows_replayed": len(rep), "rows_expected": expect_keys, "identical_rows": len(rep) - len(diffs),
            "instance_rows_identical": inst_eq, "diffs": diffs,
            "ok": len(rep) == expect_keys and not diffs and inst_eq}
    res["cells"].append(cell)
    if not cell["ok"]:
        res["fail"].append(tag)


for a in plan["A"]:
    tag = f"a_cc{a['cc']}_{a['cs']}_k{a['k']}"
    cmp_runs(tag, jl(os.path.join(R, tag + ".jsonl")), committed(f"a_cc{a['cc']}.jsonl.gz"), 8)
for c in plan["B"]:
    tag = f"b_p{c['p']}_{c['arm']}_{c['cs']}_k{c['k0']}"
    cmp_runs(tag, jl(os.path.join(R, tag + ".jsonl")), committed(f"b_p{c['p']}_{c['arm']}.jsonl.gz"), 2 * c["targets"])
q = plan["Q"]
cmp_runs(f"q_{q['cs']}_k0", jl(os.path.join(R, f"q_{q['cs']}_k0.jsonl")), committed("b_p16411_q.jsonl.gz"), 1)
for name in plan["S"]:
    stem = name[:-len(".jsonl.gz")]
    rep = jl(os.path.join(R, stem + ".jsonl"))
    com = committed(name)
    pick = lambda rows, kind: [strip(r) for r in rows if r["kind"] == kind]
    ok = (pick(rep, "cell") == pick(com, "cell") and len(pick(rep, "cell")) == 1
          and pick(rep, "instance") == pick(com, "instance"))
    cell = {"tag": stem, "cell_row": pick(rep, "cell"), "instances": [r["curve_seed"] for r in rep if r["kind"] == "instance"],
            "ok": ok}
    res["cells"].append(cell)
    if not ok:
        res["fail"].append(stem)
res["run_rows_identical"] = sum(c.get("identical_rows", 0) for c in res["cells"])
res["run_rows_expected"] = sum(c.get("rows_expected", 0) for c in res["cells"])
res["search_cells_identical"] = sum(1 for c in res["cells"] if "cell_row" in c and c["ok"])
res["ok"] = not res["fail"]
dump("replay_compare.json", res)
print({k: v for k, v in res.items() if k != "cells"})
for c in res["cells"]:
    print(c["tag"], c["ok"], c.get("rows_replayed"), c.get("rows_expected"), c.get("diffs", c.get("cell_row")))

# ---- C2: the library's f_irreducible per seed against the validator's DDF ----
mine = json.load(open(os.path.join(OUT, "seed_reducibility.json")))
rnd = random.Random("val4po-irr")
pf = {"compared": 0, "agree": 0, "disagree": [], "f_equal_validator_random_f": True, "by_p": {}}
for r in jl(os.path.join(R, "h_irr.jsonl")):
    p, cs = r["p"], r["cs"]
    f = random_f(p, 4, cs)
    if r["f"] != f:
        pf["f_equal_validator_random_f"] = False
    key = f"{p}:{cs}"
    red = mine[key] if key in mine else (0 if reducible_certificate(f, p, rnd) is None else 1)
    pf["compared"] += 1
    b = pf["by_p"].setdefault(str(p), {"seeds": 0, "rust_irreducible": 0, "validator_irreducible": 0})
    b["seeds"] += 1
    b["rust_irreducible"] += r["irr"]
    b["validator_irreducible"] += red == 0
    if r["irr"] == (red == 0):
        pf["agree"] += 1
    else:
        pf["disagree"].append(key)
pf["ok"] = pf["compared"] > 0 and not pf["disagree"] and pf["f_equal_validator_random_f"]
dump("prefilter_rust_vs_validator.json", pf)
print({k: v for k, v in pf.items() if k != "disagree"}, "disagree", pf["disagree"][:10])

# compact, committed form of h_irr.jsonl (521 KB, a data file over 64 KiB, kept out of Git): one flag per seed
raw_irr = open(os.path.join(R, "h_irr.jsonl"), "rb").read()
flags = {"source": "out/replay/h_irr.jsonl (not committed; regenerate: printf 'irr 16411 1 2000\\nirr 32771 1 3000\\n' | val_po)",
         "source_sha256": sha(raw_irr), "meaning": "character i (0-based) is 1 iff the library's f_irreducible(p, random_f(p, 4, i + 1)) is true"}
for p, hi in ((16411, 2000), (32771, 3000)):
    rows_p = {r["cs"]: r["irr"] for r in jl(os.path.join(R, "h_irr.jsonl")) if r["p"] == p}
    flags[str(p)] = "".join("1" if rows_p[cs] else "0" for cs in range(1, hi + 1))
dump("prefilter_rust_flags.json", flags)
print("flags written", {p: flags[p].count("1") for p in ("16411", "32771")})
