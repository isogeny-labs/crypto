#!/usr/bin/env python3
"""EXP-PKM-IC-001: K0-K3 from runs/ (frozen PROTOCOL.md). Writes results.json."""
import glob
import json
import os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(HERE, "runs", "*.jsonl"))) for l in open(f) if l.strip()]
    k0_bad = [(r["bits"], r["t"], r["arm"], r["target"]) for r in rows if not (r["solved"] and r["correct"])]
    cells = {}
    for r in rows:
        cells.setdefault((r["bits"], r["t"]), {}).setdefault(r["arm"], []).append(r)
    summ = []
    for (bits, t), arms in sorted(cells.items()):
        c = {"bits": bits, "t": t, "S": arms["kummer"][0]["S"], "M": arms["kummer"][0]["M"],
             "n": arms["kummer"][0]["n"], "rho_reference": arms["kummer"][0]["rho_reference"]}
        for arm, rs in arms.items():
            ok = [r for r in rs if r["solved"]]
            c[arm] = {"ops_median": median(r["charged_group_ops"] for r in ok),
                      "trials_per_relation_median": median(r["trials_total"] / r["relations"] for r in ok),
                      "over_rho_median": median(r["ratio_over_rho"] for r in ok),
                      "wall_ms_median": median(r["wall_ms"] for r in ok)}
        base = c["small-x"]
        for arm in ("kummer", "random-x"):
            c[arm]["ops_over_small"] = c[arm]["ops_median"] / base["ops_median"]
            c[arm]["trials_over_small"] = c[arm]["trials_per_relation_median"] / base["trials_per_relation_median"]
        summ.append(c)
    v = lambda bad, data: "falsified" if bad else ("held" if data else "no data")  # noqa: E731
    K1 = [(c["bits"], c["t"], arm, c[arm]["ops_over_small"]) for c in summ for arm in ("kummer", "random-x")]
    K2 = [(c["bits"], c["t"], c["kummer"]["trials_over_small"]) for c in summ]
    K3 = [(c["bits"], c["t"], arm, c[arm]["over_rho_median"]) for c in summ for arm in ("kummer", "small-x", "random-x")]
    out = {
        "study": "EXP-PKM-IC-001",
        "K0": {"rows": len(rows), "failed_or_wrong": k0_bad, "verdict": "held" if rows and not k0_bad else "falsified"},
        "K1": {"cells": K1, "verdict": v([x for x in K1 if not 0.8 < x[3] < 1.25], K1)},
        "K2": {"cells": K2, "verdict": v([x for x in K2 if not 0.8 < x[2] < 1.25], K2)},
        "K3": {"cells": K3, "verdict": v([x for x in K3 if x[3] <= 1], K3)},
        "cells": summ,
        "note": "Charged group operations of the pair-sum IC (table build, trials, verification); elimination work is "
                "counted by the harness as *_uncharged and not included. Rho reference is the theoretical sqrt(pi n/2), "
                "not a measured matched rho. No claim about cryptographic sizes [pcc:unverified].",
    }
    json.dump(out, open(os.path.join(HERE, "results.json"), "w"), indent=1)
    for k in ("K0", "K1", "K2", "K3"):
        print(k, out[k]["verdict"])
    for c in summ:
        print(f'{c["bits"]}-bit t={c["t"]} M={c["M"]:4}: ops kummer/small {c["kummer"]["ops_over_small"]:.3f} '
              f'random/small {c["random-x"]["ops_over_small"]:.3f}  trials k/s {c["kummer"]["trials_over_small"]:.3f}  '
              f'over rho: small {c["small-x"]["over_rho_median"]:.1f} kummer {c["kummer"]["over_rho_median"]:.1f} '
              f'random {c["random-x"]["over_rho_median"]:.1f}')


if __name__ == "__main__":
    main()
