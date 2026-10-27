#!/usr/bin/env python3
"""EXPLORATORY (PROTOCOL.md Amendment 3, post hoc; outside every gate).

Exact relation yield per certified curve and a within-group permutation test of whether
the certified conductor f_E explains differences in normalised yield.

  python3 exploratory_yield.py     # writes exploratory_yield.json, exploratory_yield.csv
"""
import csv
import json
import os
import sys
import time

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from cgp_field import Fq  # noqa: E402
from cgp_curve import Curve  # noqa: E402
from paired_trials import sample_P0  # noqa: E402

N_PERM = 20000
PERM_SEED = 20261030


def exact_yield(F, a, b, r, h):
    E = Curve(F, a, b)
    base = []
    for x in range(F.p):
        y = F.sqrt(E.f(x))
        if y is not None:
            base.append((x, y))
    P0 = sample_P0(E, h, 0)
    G = E.mul(h, P0)
    sub = set()
    cur = None
    for _ in range(r - 1):
        cur = E.add(cur, G)
        sub.add(cur)
    assert E.add(cur, G) is None  # G has order r
    hits = set()
    for i, P1 in enumerate(base):
        for P2 in base[i:]:
            for Q1 in (P1, E.neg(P1)):
                for Q2 in (P2, E.neg(P2)):
                    S = E.add(Q1, Q2)
                    if S is not None and S in sub:
                        hits.add(S)
    return len(hits) / (r - 1), len(base)


def between_ss(values, labels, groups):
    tot = 0.0
    for g in set(groups):
        idx = [i for i, gg in enumerate(groups) if gg == g]
        v = np.array([values[i] for i in idx])
        lab = [labels[i] for i in idx]
        m = v.mean()
        for l in set(lab):
            sel = np.array([x for x, ll in zip(v, lab) if ll == l])
            tot += len(sel) * (sel.mean() - m) ** 2
    return tot


def mean_abs_contrast(values, labels, groups):
    diff, same = [], []
    for g in set(groups):
        idx = [i for i, gg in enumerate(groups) if gg == g]
        for a_ in range(len(idx)):
            for b_ in range(a_ + 1, len(idx)):
                d = abs(values[idx[a_]] - values[idx[b_]])
                (same if labels[idx[a_]] == labels[idx[b_]] else diff).append(d)
    return (np.mean(diff) - np.mean(same)) if diff and same else 0.0, len(diff), len(same)


def perm_test(values, labels, groups, stat, rng):
    obs = stat(values, labels, groups)
    obs = obs[0] if isinstance(obs, tuple) else obs
    gidx = {}
    for i, g in enumerate(groups):
        gidx.setdefault(g, []).append(i)
    ge = 0
    for _ in range(N_PERM):
        lab = list(labels)
        for idx in gidx.values():
            perm = rng.permutation(len(idx))
            sub = [labels[idx[k]] for k in perm]
            for i, l in zip(idx, sub):
                lab[i] = l
        s = stat(values, lab, groups)
        s = s[0] if isinstance(s, tuple) else s
        ge += s >= obs - 1e-15
    return obs, (ge + 1) / (N_PERM + 1)


def main():
    t0 = time.process_time()
    receipt = json.load(open(os.path.join(HERE, "selection_receipt.json")))
    gids = {g["group"] for g in receipt["groups"] if g.get("pair")}
    fields = {}
    rows = []
    with open(os.path.join(HERE, "curves.jsonl")) as fh:
        for line in fh:
            c = json.loads(line)
            if c["group"] not in gids or not c["trial_eligible"] or c["fE"] is None:
                continue
            F = fields.setdefault(c["p"], Fq(c["p"]))
            y, fb = exact_yield(F, c["a"], c["b"], c["r"], c["cofactor"])
            rows.append({"id": c["id"], "group": c["group"], "p": c["p"], "fE": c["fE"], "f_pi": c["f_pi"],
                         "r": c["r"], "cofactor": c["cofactor"], "fb_size": fb, "yield": y,
                         "z": y / fb ** 2})
    # keep groups with >= 2 distinct f_E
    by = {}
    for r_ in rows:
        by.setdefault(r_["group"], []).append(r_)
    keep = [r_ for g, v in by.items() if len({x["fE"] for x in v}) >= 2 for r_ in v]
    groups = [r_["group"] for r_ in keep]
    labels = [r_["fE"] for r_ in keep]
    parity = [i % 2 for i in range(len(keep))]
    rng = np.random.default_rng(PERM_SEED)
    out = {"label": "EXPLORATORY, post hoc (PROTOCOL.md Amendment 3); outside every gate",
           "n_groups_with_pair": len(gids), "n_curves": len(rows), "n_curves_in_contrast_groups": len(keep),
           "n_contrast_groups": len(set(groups)), "n_perm": N_PERM, "tests": {}}
    for name, vals in (("z_normalised_yield", [r_["z"] for r_ in keep]), ("yield", [r_["yield"] for r_ in keep])):
        for sname, stat in (("between_conductor_SS", between_ss), ("mean_abs_diff_contrast", mean_abs_contrast)):
            obs, p = perm_test(vals, labels, groups, stat, rng)
            out["tests"][f"{name}/{sname}"] = {"observed": obs, "p_one_sided": p}
        obsp, pp = perm_test(vals, parity, groups, between_ss, rng)
        out["tests"][f"{name}/calibration_parity_label_between_SS"] = {"observed": obsp, "p_one_sided": pp}
    d, nd, ns = mean_abs_contrast([r_["z"] for r_ in keep], labels, groups)
    out["z_contrast"] = {"mean_abs_diff_different_fE_minus_same_fE": d, "pairs_different_fE": nd, "pairs_same_fE": ns}
    zs = np.array([r_["z"] for r_ in keep])
    out["z_summary"] = {"mean": float(zs.mean()), "sd": float(zs.std(ddof=1)), "min": float(zs.min()), "max": float(zs.max())}
    # trial-pair curves and controls, for reading next to the sampled yields
    out["cpu_seconds"] = time.process_time() - t0
    with open(os.path.join(HERE, "exploratory_yield.json"), "w") as fh:
        json.dump(out, fh, indent=1)
    with open(os.path.join(HERE, "exploratory_yield.csv"), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)
    print(json.dumps({k: v for k, v in out.items() if k not in ("tests",)}, indent=1))
    for k, v in out["tests"].items():
        print(f"{k:55s} obs={v['observed']:.6g} p={v['p_one_sided']:.4f}")


if __name__ == "__main__":
    main()
