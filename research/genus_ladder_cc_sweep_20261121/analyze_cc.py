#!/usr/bin/env python3
"""Analysis of the cc sweep (PROTOCOL.md). Reads the recorded counts only.

Usage: analyze_cc.py WORKDIR_WITH_JSONL ANALYZE_BIN OUT.json
Expects g3_cc<X>.jsonl and g4_cc<X>.jsonl, plus rho_g3.jsonl, rho_g4.jsonl (the earlier
studies' R_rho_neg rows, extracted by the run script). Runs genus_ladder_analyze on each
(cc file + rho rows), then derives per-cell tables, paired gains vs cc = 2.0, F1-F5.
"""
import json, random, subprocess, sys, os
from collections import defaultdict

CCS = ["1.0", "1.5", "2.0", "3.0", "4.0"]
SGE, UNM = "C_double_lp+pre+sge(8,3)", "C_double_lp+pre"
BOOT, SEED = 2000, 20261121
READ = {"low": (1, 1), "mid": (10, None), "high": (30, None)}


def fpm(r, kind):
    g = r["g"]
    I, k = {"low": (1, 1), "mid": (10, g * g), "high": (30, g * g)}[kind]
    return r["cantor_mul"] + I * r["cantor_inv"] + r["rel_mul"] + k * r["la_mul"]


def boot(pairs):
    by = defaultdict(list)
    for c, x in pairs:
        by[c].append(x)
    cs = sorted(by)
    mean = sum(x for _, x in pairs) / len(pairs)
    rng = random.Random(SEED)
    ms = []
    for _ in range(BOOT):
        s = n = 0
        for _ in cs:
            v = by[rng.choice(cs)]
            for _ in v:
                s += rng.choice(v); n += 1
        ms.append(s / n)
    ms.sort()
    return {"mean": mean, "lo": ms[int(0.025 * (BOOT - 1) + .5)], "hi": ms[int(0.975 * (BOOT - 1) + .5)]}


def load(path):
    return [json.loads(l) for l in open(path) if l.strip()]


def main():
    wd, abin, out = sys.argv[1:4]
    res = {"schema": "genus_ladder.cc_sweep/v1", "cells": [], "f1": {}, "f2": [], "f3": [], "f4": {}, "f5": []}
    runs = {}  # (fam, cc) -> list of run rows
    ana = {}
    for fam in ("g3", "g4"):
        rho = load(os.path.join(wd, f"rho_{fam}.jsonl"))
        for cc in CCS:
            fn = os.path.join(wd, f"{fam}_cc{cc}.jsonl")
            rows = load(fn)
            runs[(fam, cc)] = [r for r in rows if r.get("kind") == "run"]
            comb = os.path.join(wd, f"comb_{fam}_cc{cc}.jsonl")
            with open(comb, "w") as f:
                for r in rows + rho:
                    f.write(json.dumps(r, sort_keys=True) + "\n")
            aj = os.path.join(wd, f"an_{fam}_cc{cc}.json")
            subprocess.run([abin, comb, aj], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            ana[(fam, cc)] = json.load(open(aj))
    # per cell
    cells = {}
    for (fam, cc), rs in runs.items():
        for r in rs:
            if r["arm"] in (SGE, UNM):
                cells.setdefault((fam, r["p"]), {}).setdefault(cc, {}).setdefault(r["arm"], []).append(r)
    ratio = {}
    for (fam, cc), a in ana.items():
        for c in a["cells"]:
            if c["arm"] in (SGE, UNM):
                ratio[(fam, c["p"], cc, c["arm"])] = c
    allcert = all(r.get("certified") and r.get("ok") for rs in runs.values() for r in rs)
    res["f1"]["all_certified"] = allcert
    res["f1"]["runs"] = sum(len(rs) for rs in runs.values())
    mean = lambda xs: sum(xs) / len(xs)
    for (fam, p), byc in sorted(cells.items()):
        g = 3 if fam == "g3" else 4
        cell = {"family": fam, "g": g, "p": p, "by_cc": {}}
        for cc in CCS:
            rs = byc[cc][SGE]
            un = byc[cc][UNM]
            e = {"n": len(rs),
                 "trials": mean([r["trials"] for r in rs]), "relations": mean([r["relations"] for r in rs]),
                 "fb_cols": mean([r["fb_cols"] for r in rs]), "core_cols": mean([r["core_cols"] for r in rs]),
                 "cantor_mul": mean([r["cantor_mul"] for r in rs]), "cantor_inv": mean([r["cantor_inv"] for r in rs]),
                 "rel_mul": mean([r["rel_mul"] for r in rs]), "la_mul": mean([r["la_mul"] for r in rs]),
                 "la_mul_unmerged": mean([r["la_mul"] for r in un]),
                 "fpm_low": mean([fpm(r, "low") for r in rs]), "fpm_mid": mean([fpm(r, "mid") for r in rs]),
                 "fpm_high": mean([fpm(r, "high") for r in rs]),
                 "fpm_mid_unmerged": mean([fpm(r, "mid") for r in un])}
            c = ratio[(fam, p, cc, SGE)]
            for k in ("low", "mid", "high"):
                x = c[f"fpm_{k}_over_rho"]
                e[f"ratio_{k}"] = {"ratio": x["ratio"], "lo": x["lo"], "hi": x["hi"]}
            u = ratio[(fam, p, cc, UNM)]["fpm_mid_over_rho"]
            e["ratio_mid_unmerged"] = {"ratio": u["ratio"], "lo": u["lo"], "hi": u["hi"]}
            e["band_fpm"] = c.get("band_fpm")
            # paired vs 2.0
            base = {(r["curve_seed"], r["seed"]): r for r in byc["2.0"][SGE]}
            pr = [(r["curve_seed"], fpm(r, "mid") / fpm(base[(r["curve_seed"], r["seed"])], "mid")) for r in rs]
            e["paired_vs_2.0"] = boot(pr)
            cell["by_cc"][cc] = e
        means = {k: [cell["by_cc"][cc][f"fpm_{k}"] for cc in CCS] for k in ("low", "mid", "high")}
        cell["best_cc"] = {k: CCS[min(range(5), key=lambda i: means[k][i])] for k in means}
        cell["rank"] = {k: [CCS[i] for i in sorted(range(5), key=lambda i: means[k][i])] for k in means}
        b = cell["best_cc"]["mid"]
        cell["gain_best_vs_2.0"] = 1 - cell["by_cc"][b]["fpm_mid"] / cell["by_cc"]["2.0"]["fpm_mid"]
        tr = [cell["by_cc"][cc]["trials"] for cc in CCS]
        la = [cell["by_cc"][cc]["la_mul"] for cc in CCS]
        cell["f2"] = {"trials_nonincreasing": all(tr[i] >= tr[i + 1] for i in range(4)),
                      "la_nondecreasing": all(la[i] <= la[i + 1] for i in range(4)),
                      "trials": tr, "la_mul": la}
        cell["f5_same_ranking"] = cell["rank"]["low"] == cell["rank"]["mid"] == cell["rank"]["high"]
        cell["f5_same_best"] = len(set(cell["best_cc"].values())) == 1
        res["cells"].append(cell)
    # SGE effect at 2.0 genus 4
    for cell in res["cells"]:
        e = cell["by_cc"]["2.0"]
        cell["sge_effect_2.0"] = {"la_ratio": e["la_mul"] / e["la_mul_unmerged"], "fpm_mid_ratio": e["fpm_mid"] / e["fpm_mid_unmerged"]}
    json.dump(res, open(out, "w"), indent=1, sort_keys=True)


main()
