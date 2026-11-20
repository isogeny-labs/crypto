#!/usr/bin/env python3
"""EXP-PKM-RESULTANT-001 Amendment 1: A1-A4 from runs/aa/ and runs/amortised/ against the frozen runs/.
Writes results_amortised.json."""
import glob
import json
import os
from statistics import median

HERE = os.path.dirname(os.path.abspath(__file__))
P3 = 3221225473


def load(sub):
    out = {}
    for f in sorted(glob.glob(os.path.join(HERE, "runs", sub, "*.jsonl"))):
        for l in open(f):
            if l.strip():
                r = json.loads(l)
                out[(r["cell"], r["index"])] = r
    return out


def main():
    frozen, aa, am = load(""), load("aa"), load("amortised")
    # runs/aa/ also holds P3-m2-t4-res, which is not a frozen cell (a slip in run_amortised.sh): no reference.
    unmatched = sorted({k[0] for k in aa if k not in frozen})
    aa = {k: r for k, r in aa.items() if k in frozen}
    a1_counts = [k for k, r in aa.items() if r["resultant"]["field_ops"] != frozen[k]["resultant"]["field_ops"]
                 or r["target"] != frozen[k]["target"]]
    a1_agree = [k for k, r in am.items() if not r["resultant"]["agree"]]
    a1_target = [k for k, r in am.items() if r["target"] != frozen[k]["target"]]
    cells = {}
    for k, r in am.items():
        cells.setdefault(r["cell"], []).append(r)
    summ = []
    for name, rs in cells.items():
        pl = [r for r in rs if r["kind"] == "planted"]
        marg = median(r["resultant"]["field_ops"] for r in pl)
        full = median(frozen[(name, r["index"])]["resultant"]["field_ops"] for r in pl)
        em = median(r["enum_marginal_ops"] for r in pl)
        setup = rs[0]["resultant"]["setup_ops"]
        summ.append({"cell": name, "p": rs[0]["p"], "S": rs[0]["S"], "vkind": rs[0]["vkind"],
                     "marginal": marg, "frozen_full": full, "marginal_over_full": marg / full,
                     "enum_marginal": em, "enum_full": median(r["enum_field_ops"] for r in pl),
                     "marginal_over_enum_marginal": marg / em, "setup_ops": setup,
                     "setup_over_marginal": setup / marg,
                     "eval_share": median(r["resultant"]["eval_ops"] / r["resultant"]["field_ops"] for r in pl)})
    summ.sort(key=lambda c: (c["p"], c["vkind"], c["S"]))
    v = lambda bad, data: "falsified" if bad else ("held" if data else "no data")  # noqa: E731
    A2 = [(c["cell"], c["marginal_over_full"]) for c in summ if c["S"] >= 256]
    A3 = [(c["cell"], c["marginal_over_enum_marginal"]) for c in summ if c["S"] >= 64]
    p3 = {c["S"]: c for c in summ if c["p"] == P3 and c["vkind"] == "coset"}
    g = p3[65536]["marginal_over_enum_marginal"] / p3[256]["marginal_over_enum_marginal"] if 65536 in p3 and 256 in p3 else None
    out = {
        "study": "EXP-PKM-RESULTANT-001 Amendment 1",
        "A1": {"frozen_cells_rerun": len(aa), "rerun_cells_without_frozen_reference": unmatched, "count_or_target_mismatches": a1_counts,
               "amortised_disagreements": a1_agree, "amortised_target_mismatches": a1_target,
               "verdict": "held" if aa and not (a1_counts or a1_agree or a1_target) else "falsified"},
        "A2": {"cells": A2, "verdict": v([x for x in A2 if not 0.3 <= x[1] <= 0.7], A2)},
        "A3": {"cells": A3, "growth_2^8_to_2^16": g,
               "verdict": v([x for x in A3 if x[1] <= 1] or (g is not None and not g < 16), A3)},
        "A4": [(c["cell"], c["setup_over_marginal"]) for c in summ],
        "cells": summ,
        "note": "Labelled extension; frozen verdicts stand. No claim about cryptographic sizes or Pollard rho "
                "[pcc:unverified].",
    }
    json.dump(out, open(os.path.join(HERE, "results_amortised.json"), "w"), indent=1)
    for k in ("A1", "A2", "A3"):
        print(k, out[k]["verdict"])
    print("A1", out["A1"]["frozen_cells_rerun"], "rerun rows; growth", g)
    for c in summ:
        print(f'{c["cell"]:22} S={c["S"]:6} marg={c["marginal"]:.3g} full={c["frozen_full"]:.3g} '
              f'm/f={c["marginal_over_full"]:.2f} enum_m={c["enum_marginal"]:.3g} '
              f'm/enum_m={c["marginal_over_enum_marginal"]:.2f} setup/marg={c["setup_over_marginal"]:.2f} '
              f'eval%={c["eval_share"]:.2f}')


if __name__ == "__main__":
    main()
