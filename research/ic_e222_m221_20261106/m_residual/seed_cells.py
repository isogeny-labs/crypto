#!/usr/bin/env python3
"""Is there noise shared between instances that use the same solver seed (unvalidated, pre-gate)?
The relation sampler's (a, b) stream depends only on the solver seed and the subgroup order n, so instances of one
rung and one seed draw nearly the same uniform numbers.  For each (seed, rung) cell of the new Montgomery sample the
mean single-seed residual is compared with its model standard error; chi2 = sum ((cell - grand) / se)^2 should be
about (cells - 1) if the cells are independent.  Same for Edwards.
Usage: seed_cells.py OUT.json"""
import json, math, statistics, collections, sys
rows = [json.loads(l) for l in open("rows.jsonl")]
model = {r["name"]: r for r in json.load(open("model.json"))}
obs = collections.defaultdict(lambda: collections.defaultdict(dict))
for r in rows:
    obs[r["name"]][r["seed"]][r["arm"]] = r["trials"]
K = 34
out = {}
for fam in ("M", "E"):
    cells = collections.defaultdict(list)       # (seed, rung) -> [(resid, var)]
    for name, o in obs.items():
        m = model[name]
        if m["family"] != fam:
            continue
        for s, a in o.items():
            res = math.log(a["on"] / a["off"]) - math.log(m["P_off"] / m["P_on"]) - (m["P_on"] - m["P_off"]) / (2 * K)
            var = ((1 - m["P_on"]) + (1 - m["P_off"])) / K
            cells[(s, m["rung"])].append((res, var))
    allr = [x for v in cells.values() for x in v]
    grand = statistics.mean([x[0] for x in allr])
    chi = 0.0; table = {}
    for key in sorted(cells):
        v = cells[key]
        mean = statistics.mean([x[0] for x in v]); se = math.sqrt(sum(x[1] for x in v)) / len(v)
        chi += ((mean - grand) / se) ** 2
        table[f"seed{key[0]}_rung{key[1]}"] = dict(n=len(v), mean=mean, se=se, z=(mean - grand) / se)
    ncell = len(cells)
    # chi2 tail by Wilson-Hilferty
    k = ncell - 1; z = ((chi / k) ** (1 / 3) - (1 - 2 / (9 * k))) / math.sqrt(2 / (9 * k)); p = 0.5 * math.erfc(z / math.sqrt(2))
    # by seed only (pooled over rungs)
    byseed = {}
    for s in sorted({k_[0] for k_ in cells}):
        v = [x for key, vv in cells.items() if key[0] == s for x in vv]
        byseed[str(s)] = dict(mean=statistics.mean([x[0] for x in v]), se=math.sqrt(sum(x[1] for x in v)) / len(v))
    out[fam] = dict(grand_mean=grand, cells=ncell, chi2=chi, dof=k, chi2_p_upper=p, by_seed=byseed, cell_table=table)
    print(f"{fam}: grand mean {grand:+.4f}  cells {ncell}  chi2 {chi:.1f} on {k} dof  upper-tail p {p:.3f}   per-seed means (se):",
          {s: (round(v['mean'], 4), round(v['se'], 4)) for s, v in byseed.items()})
json.dump(out, open(sys.argv[1], "w"), indent=1)
