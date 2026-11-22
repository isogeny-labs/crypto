#!/usr/bin/env python3
"""(Adapted for the merge-depth study, PROTOCOL.md section 4: multiple settings, best = lowest
genus-3 p=32771 la ratio, ratios also per (setting,g,p) for all cells.)
Paired comparison of `X` and `X+sge(W,F)` runs of one `genus_ladder run` session.

Usage: python3 paired.py sge.jsonl paired.json

Reads the recorded counts only. Pairs every run of the base arm with the run of each
merged arm on the same (p, g, curve_seed, seed), checks that the relation-phase
fields are identical (P1) and that both certify with the planted logarithm (P2), and
reports per (setting, genus) and per (setting, genus, p) the mean of the per-pair
ratios of `la_mul` (merge work included) and of `fpm_mid`
(`cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul`, the primary reading of the
first study) with a hierarchical bootstrap 95% interval (curves, then seeds within a
curve; 2000 resamples, seed 20261121). Statistic and resampling fixed in PROTOCOL.md
section 4 before any measured run.
"""
import json
import random
import sys
from collections import defaultdict

BASE = "C_double_lp+pre"
RELATION_FIELDS = [
    "trials", "candidates", "prefilter_rejects", "splits", "relations", "rels_by_lp",
    "rel_mul", "rel_add", "group_ops", "setup_ops", "walk_ops", "verify_ops",
    "cantor_mul", "cantor_inv", "attempts", "core_rows", "core_cols", "fb_cols",
]
BOOT, SEED = 2000, 20261121


def fpm_mid(r):
    g = r["g"]
    return r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + g * g * r["la_mul"]


def boot(pairs):
    """pairs: list of (curve, ratio). Mean and hierarchical-bootstrap 95% interval."""
    by = defaultdict(list)
    for c, x in pairs:
        by[c].append(x)
    curves = sorted(by)
    mean = sum(x for _, x in pairs) / len(pairs)
    rng = random.Random(SEED)
    ms = []
    for _ in range(BOOT):
        s, n = 0.0, 0
        for _ in curves:
            v = by[rng.choice(curves)]
            for _ in v:
                s += rng.choice(v)
                n += 1
        ms.append(s / n)
    ms.sort()
    return {"mean": mean, "lo": ms[int(0.025 * (BOOT - 1) + 0.5)],
            "hi": ms[int(0.975 * (BOOT - 1) + 0.5)], "pairs": len(pairs),
            "curves": len(curves)}


def main():
    src, dst = sys.argv[1], sys.argv[2]
    runs = [json.loads(l) for l in open(src) if l.strip()]
    runs = [r for r in runs if r.get("kind") == "run" and r.get("family") == "hyperelliptic"]
    base = {(r["p"], r["g"], r["curve_seed"], r["seed"]): r for r in runs if r["arm"] == BASE}
    merged = [r for r in runs if r["arm"].startswith(BASE + "+sge(")]
    out = {"schema": "genus_ladder.sge_paired/v1", "input": src, "base_arm": BASE,
           "pairs": 0, "relation_identical": 0, "relation_mismatch": [],
           "certified_both": 0, "same_t": 0, "unpaired": 0, "by_genus": [], "by_cell": [],
           "structure": []}
    groups_g = defaultdict(lambda: {"la": [], "fpm": [], "wied": []})
    groups_c = defaultdict(lambda: {"la": [], "fpm": [], "wied": []})
    struct = defaultdict(lambda: defaultdict(float))
    for m in merged:
        k = (m["p"], m["g"], m["curve_seed"], m["seed"])
        b = base.get(k)
        if b is None:
            out["unpaired"] += 1
            continue
        out["pairs"] += 1
        same = all(m[f] == b[f] for f in RELATION_FIELDS)
        if same:
            out["relation_identical"] += 1
        else:
            out["relation_mismatch"].append({"key": k, "arm": m["arm"], "fields": [
                f for f in RELATION_FIELDS if m[f] != b[f]]})
        if m["certified"] and b["certified"]:
            out["certified_both"] += 1
        if m["ok"] and b["ok"]:
            out["same_t"] += 1
        s = m["arm"][len(BASE) + 1:]
        la = m["la_mul"] / b["la_mul"]
        fp = fpm_mid(m) / fpm_mid(b)
        wd = m["la_wied_mul"] / b["la_wied_mul"]
        for grp in (groups_g[(s, m["g"])], groups_c[(s, m["g"], m["p"])]):
            grp["la"].append((m["curve_seed"], la))
            grp["fpm"].append((m["curve_seed"], fp))
            grp["wied"].append((m["curve_seed"], wd))
        st = struct[(s, m["g"], m["p"])]
        st["n"] += 1
        for f in ["sge_rows_in", "sge_cols_in", "sge_nnz_in", "sge_generic_in", "sge_rows_out",
                  "sge_cols_out", "sge_nnz_out", "sge_generic_out", "sge_merged", "sge_peeled",
                  "sge_empty", "sge_duplicates", "sge_refused", "la_sge_mul", "la_sge_add",
                  "la_wied_mul", "la_mul"]:
            st[f] += m[f]
        st["base_la_mul"] += b["la_mul"]
        st["base_la_wied_mul"] += b["la_wied_mul"]
        st["base_fpm_mid"] += fpm_mid(b)
        st["fpm_mid"] += fpm_mid(m)
        st["base_la_share_fpm"] += m["g"] ** 2 * b["la_mul"] / fpm_mid(b)
    for (s, g), v in sorted(groups_g.items()):
        out["by_genus"].append({"setting": s, "g": g, "la_mul_ratio": boot(v["la"]),
                                "fpm_mid_ratio": boot(v["fpm"]),
                                "la_wied_mul_ratio": boot(v["wied"])})
    for (s, g, p), v in sorted(groups_c.items()):
        out["by_cell"].append({"setting": s, "g": g, "p": p, "la_mul_ratio": boot(v["la"]),
                               "fpm_mid_ratio": boot(v["fpm"]),
                               "la_wied_mul_ratio": boot(v["wied"])})
    for (s, g, p), st in sorted(struct.items()):
        n = st.pop("n")
        out["structure"].append({"setting": s, "g": g, "p": p, "runs": int(n),
                                 **{f"mean_{k}": v / n for k, v in st.items()}})
    # best setting (PROTOCOL.md 4): lowest mean la_mul ratio at genus 3, p = 32771
    c3 = {c["setting"]: c["la_mul_ratio"]["mean"] for c in out["by_cell"]
          if c["g"] == 3 and c["p"] == 32771}
    if c3:
        out["best_setting"] = min(c3, key=c3.get)
    json.dump(out, open(dst, "w"), indent=1, sort_keys=True)
    print(json.dumps({k: out[k] for k in ["pairs", "relation_identical", "certified_both",
                                          "same_t", "unpaired"]} | {"best": out.get("best_setting")}))


if __name__ == "__main__":
    main()
