#!/usr/bin/env python3
"""Tabulate recorded a3_s4_ladder JSON lines.  Reads measurements only.

Usage: summarise.py ladder.jsonl [--md]
"""
import json
import sys
from collections import defaultdict


def load(path):
    rows = []
    for line in open(path):
        line = line.strip()
        if not line:
            continue
        d = json.loads(line)
        s = d["ic_stages"]
        rows.append(
            {
                "bits": d["bits"],
                "curve": d["curve"],
                "class": d["curve_class"],
                "oracle": d["decomposition"]["oracle_class"],
                "fb": d["fb_size"],
                "seed": d["target_seed"],
                "rel": s["relations_collected"],
                "trials": s["trials_total"],
                "tpr": s["trials_per_relation_median"],
                "ic_ms": d["ic_ms"],
                "rel_ms": s["relations_ms"],
                "la_ms": s["linear_algebra_ms"],
                "fb_ms": s["factor_base_ms"],
                "ms_rel": d["ic_ms"] / max(1, s["relations_collected"]),
                "rho_ms": d["rho_ms"],
                "ok": bool(d["ic_ok"] and d["ic_matches_truth"] and d["ic_agrees_rho"]),
            }
        )
    return rows


def main():
    path = sys.argv[1]
    md = "--md" in sys.argv
    rows = load(path)
    if md:
        print("| bits | class | oracle | seed | relations | trials | median trials/rel | IC wall (s) | ms/relation | LA (ms) | verified |")
        print("|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|")
        for r in sorted(rows, key=lambda r: (r["bits"], r["class"], r["oracle"], r["seed"])):
            print(
                f"| {r['bits']} | {r['class']} | {r['oracle']} | {r['seed']} | {r['rel']} | "
                f"{r['trials']} | {r['tpr']:g} | {r['ic_ms']/1e3:.3f} | {r['ms_rel']:.3f} | "
                f"{r['la_ms']:.2f} | {'yes' if r['ok'] else 'NO'} |"
            )
    else:
        print(json.dumps(rows, indent=1))

    # Medians per (bits, class, oracle): the headline numbers.
    groups = defaultdict(list)
    for r in rows:
        groups[(r["bits"], r["class"], r["oracle"])].append(r)
    if md:
        print()
        print("| bits | class | oracle | seeds | median IC wall (s) | median ms/relation | median trials/rel | relations |")
        print("|--:|:--|:--|--:|--:|--:|--:|--:|")

        def med(xs):
            xs = sorted(xs)
            n = len(xs)
            return xs[n // 2] if n % 2 else (xs[n // 2 - 1] + xs[n // 2]) / 2

        for key in sorted(groups):
            g = groups[key]
            bits, cls, oracle = key
            print(
                f"| {bits} | {cls} | {oracle} | {len(g)} | {med([r['ic_ms'] for r in g])/1e3:.3f} | "
                f"{med([r['ms_rel'] for r in g]):.3f} | {med([r['tpr'] for r in g]):g} | "
                f"{min(r['rel'] for r in g)}-{max(r['rel'] for r in g)} |"
            )


if __name__ == "__main__":
    main()
