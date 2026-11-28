#!/usr/bin/env python3
"""C3 curve selection and C2 (per seed): coverage from the committed search files at COMMIT.

Each file's seed list is reconstructed from the script that wrote it (the config row does not record
--curve-seeds):
  search.sh P N           -> search_p{P}_1_{i}: seeds 1..N with s % 4 == i (N = 2000 at 16411, 1800 at 32771)
  search_chunks.sh P LO HI RES -> search_p{P}_r{LO}-{HI}-{RES}_w{w}_c{j}: S = LO..HI (residue RES or all),
                              worker w takes S[w::4], chunk j is mine[j:j+28]
A file covers its list only if it has exactly one cell row with tried == len(list) and
admitted + sum(rejects) == tried.  For every seed the validator's own DDF (polyfp.py) decides
reducibility; per file, its reducible count must equal rejects.f_reducible_even_order, and every
reducible seed below the fifth chosen seed carries a checked certificate (w | f, 1 <= deg w <= 4).
Writes out/select_check.json and out/seed_reducibility.json (per seed, for the Rust comparison)."""
import re, sys, os, random
sys.path.insert(0, os.path.dirname(__file__))
from common import *
from jac import random_f, is_prime, hasse_weil_bounds
from polyfp import reducible_certificate, check_certificate

N_SEARCH = {16411: 2000, 32771: 1800}
EXPECT = {16411: [373, 455, 548, 752, 875], 32771: [355, 882, 1461, 1623, 2374]}
KNOWN_REASONS = {"f_reducible_even_order", "order_not_near_prime"}


def seed_list(name):
    m = re.fullmatch(r"search_p(\d+)_1_(\d)\.jsonl\.gz", name)
    if m:
        P, i = int(m.group(1)), int(m.group(2))
        return P, [x for x in range(1, N_SEARCH[P] + 1) if x % 4 == i]
    m = re.fullmatch(r"search_p(\d+)_r(\d+)-(\d+)-(\w+)_w(\d)_c(\d+)\.jsonl\.gz", name)
    P, lo, hi, res, w, j = int(m.group(1)), int(m.group(2)), int(m.group(3)), m.group(4), int(m.group(5)), int(m.group(6))
    S = [x for x in range(lo, hi + 1) if res == "all" or x % 4 == int(res)]
    return P, S[w::4][j:j + 28]


def main():
    out = {"commit": COMMIT, "files": {}, "fail": [], "primes": {}}
    files = [p for p in raw_files() if p.split("/")[-1].startswith("search_p")]
    red_cache = {}
    rnd = random.Random("val4po-select")

    def reducible(P, s):
        if (P, s) not in red_cache:
            f = random_f(P, 4, s)
            c = reducible_certificate(f, P, rnd)
            red_cache[(P, s)] = (f, c)
        return red_cache[(P, s)]

    covered = {16411: {}, 32771: {}}
    admitted = {16411: {}, 32771: {}}
    for path in files:
        name = path.split("/")[-1]
        P, lst = seed_list(name)
        rows, h = rows_of(path)
        cfg = [r for r in rows if r["kind"] == "config"]
        cells = [r for r in rows if r["kind"] == "cell"]
        inst = [r for r in rows if r["kind"] == "instance"]
        e = {"sha256": h, "p": P, "n_list": len(lst), "first": lst[0], "last": lst[-1], "cells": len(cells),
             "instances": [r["curve_seed"] for r in inst]}
        c0 = cfg[0]
        e["config_ok"] = (len(cfg) == 1 and c0["primes"] == [P] and c0["genera"] == [4] and c0["max_h"] == 1
                          and c0["arms"] == "n" and c0["curves"] >= len(lst) and c0["targets"] == 1 and c0["seeds"] == 1)
        if not e["config_ok"]:
            out["fail"].append(f"config {name}")
        for r in inst:
            s = r["curve_seed"]
            f_ok = r["f"] == random_f(P, 4, s)
            ok = (s in lst and r["p"] == P and r["g"] == 4 and r["h"] == 1 and r["order"] == r["r"] and f_ok
                  and is_prime(r["r"]))
            if not ok:
                out["fail"].append(f"instance {name} {s}")
            if s in admitted[P] and admitted[P][s] != r["order"]:
                out["fail"].append(f"instance order differs across files {P} {s}")
            admitted[P][s] = r["order"]
        mine_red = [s for s in lst if reducible(P, s)[1] is not None]
        mine_irr = [s for s in lst if reducible(P, s)[1] is None]
        e["validator_reducible"] = len(mine_red)
        e["validator_irreducible"] = len(mine_irr)
        if len(cells) == 1:
            c = cells[0]
            rej = c["rejects"]
            e.update(tried=c["tried"], admitted=c["admitted"], rejects=rej)
            complete = (c["tried"] == len(lst) and c["admitted"] == len(inst)
                        and c["admitted"] + sum(rej.values()) == c["tried"] and c["p"] == P and c["g"] == 4)
            e["complete"] = complete
            e["prefilter_count_match"] = rej.get("f_reducible_even_order", 0) == len(mine_red)
            e["irreducible_accounted"] = len(mine_irr) == c["admitted"] + sum(v for k, v in rej.items() if k != "f_reducible_even_order")
            e["unknown_reasons"] = sorted(set(rej) - KNOWN_REASONS)
            if not complete:
                out["fail"].append(f"cell incomplete {name}")
            if not (e["prefilter_count_match"] and e["irreducible_accounted"]):
                out["fail"].append(f"prefilter count {name}")
            if complete:
                for s in lst:
                    covered[P].setdefault(s, []).append(name)
        else:
            e["complete"] = False
            e["note"] = "no cell row: partial file, its seeds are not counted as covered"
        out["files"][name] = e

    for P in (16411, 32771):
        cov = covered[P]
        top = 0
        while top + 1 in cov:
            top += 1
        adm = sorted(admitted[P])
        chosen = [s for s in adm if not (P == 16411 and s == 2640)][:5]
        need = 2000 if P == 16411 else 2600
        fifth = chosen[-1]
        below = [s for s in range(1, fifth) if s not in admitted[P]]
        red_below = [s for s in below if red_cache[(P, s)][1] is not None]
        cert_ok = all(check_certificate(red_cache[(P, s)][0], P, red_cache[(P, s)][1]) for s in red_below)
        lo, hi = hasse_weil_bounds(P, 4)
        out["primes"][str(P)] = {
            "contiguous_covered_1_to": top, "covered_required_to": need, "coverage_ok": top >= need,
            "seeds_covered_twice": sorted(s for s, v in cov.items() if len(v) > 1),
            "admitted": adm, "chosen": chosen, "expected": EXPECT[P], "chosen_ok": chosen == EXPECT[P],
            "fifth_chosen_within_coverage": fifth <= top,
            "seeds_below_fifth_not_admitted": len(below),
            "of_which_reducible_with_checked_certificate": len(red_below), "certificates_ok": cert_ok,
            "of_which_irreducible_rejected_after_point_count": len(below) - len(red_below),
            "admitted_orders_in_hasse_weil": all(lo <= o <= hi for o in admitted[P].values()),
        }
        if not (top >= need and chosen == EXPECT[P] and fifth <= top and cert_ok):
            out["fail"].append(f"selection {P}")
    cj = json.loads(blob(f"{D}/curves.json"))
    out["curves_json_matches"] = {P: cj[P]["admitted"] == out["primes"][P]["admitted"] and cj[P]["chosen"] == out["primes"][P]["chosen"]
                                  and all(cj[P]["orders"][str(s)] == admitted[int(P)][s] for s in out["primes"][P]["chosen"])
                                  for P in ("16411", "32771")}
    if not all(out["curves_json_matches"].values()):
        out["fail"].append("curves.json")
    out["reasons_seen"] = sorted({k for e in out["files"].values() for k in e.get("rejects", {})})
    out["ok"] = not out["fail"]
    dump("select_check.json", out)
    dump("seed_reducibility.json", {f"{P}:{s}": (0 if c is None else 1) for (P, s), (f, c) in sorted(red_cache.items())})
    print(json.dumps(out["primes"], indent=1))
    print("curves.json", out["curves_json_matches"], "reasons", out["reasons_seen"], "fail", out["fail"])
    print("partial files", [n for n, e in out["files"].items() if not e["complete"]])


if __name__ == "__main__":
    main()
