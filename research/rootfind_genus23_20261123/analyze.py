#!/usr/bin/env python3
"""Analysis for PROTOCOL.md: reads raw/runs.jsonl and the lab rows named in cells.json.
Writes analysis.json and tables.md. All numbers [pcc:unverified]."""
import json
import math
import os
import statistics as stt
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
W_INV = 10


def cnt(m, i):
    return m + W_INV * i


runs = [json.loads(l) for l in open(os.path.join(HERE, "raw", "runs.jsonl"))]
by = defaultdict(dict)
for r in runs:
    by[(r["g"], r["p"], r["curve_seed"])][r["arm_label"]] = r

LAB = {
    (3, 65537): ("research/genus_ladder_wallclock_vs_rho_20261122/raw/run_65537.jsonl",
                 "C_double_lp+pre+sge(8,3)"),
    (2, 65537): ("research/genus_ladder_scale_20261115/scale_g2.jsonl", "C_double_lp+pre"),
    (2, 524309): ("research/genus_ladder_scale_20261115/scale_g2b.jsonl", "C_double_lp+pre"),
}


def wald(k1, n1, k2, n2):
    a, b = k1 / n1, k2 / n2
    se = math.sqrt(a * (1 - a) / n1 + b * (1 - b) / n2)
    return a, b, a - b, (a - b - 1.96 * se, a - b + 1.96 * se)


out = {"cells": {}, "standing": "[pcc:unverified] L0 Python counts"}
lines = ["# Tables (`analyze.py`) [pcc:unverified]", ""]
lines.append("## T1 Counted Cantor cost (mults + 10 inv) per candidate, per curve")
lines.append("")
lines.append("| g | p | curve | A /step (mul, inv) | lab /group-op (mul, inv) | c (sibling marginal / step) | N sib / A step | B/A per candidate | AA/A per step | sib dup | B exact mismatches | verified ok |")
lines.append("|--:|--:|--:|--|--|--:|--:|--:|--:|--:|--:|--:|")
cellagg = defaultdict(lambda: defaultdict(list))
for key in sorted(by):
    g, p, cs = key
    d = by[key]
    A, B, N, AA = d["A"], d["B"], d["N"], d["AA"]
    assert A["fwd_split"] == B["fwd_split"] == N["fwd_split"], "paired walks differ"
    stepA = cnt(A["fwd_mul"], A["fwd_inv"]) / A["steps"]
    totA = cnt(A["fwd_mul"] + A["restart_mul"], A["fwd_inv"] + A["restart_inv"])
    totB = cnt(B["fwd_mul"] + B["pm_mul"] + B["restart_mul"], B["fwd_inv"] + B["pm_inv"] + B["restart_inv"])
    walkB_only = cnt(B["fwd_mul"] + B["pm_mul"], B["fwd_inv"] + B["pm_inv"])
    walkA_only = cnt(A["fwd_mul"], A["fwd_inv"])
    c = (walkB_only - walkA_only) / B["sibs"] / stepA
    nsib = cnt(N["sib_mul"], N["sib_inv"]) / N["sibs"] / stepA
    candA = A["steps"]
    candB = B["steps"] + B["sibs"]
    rho_w = (totB / candB) / (totA / candA)
    aa = (cnt(AA["fwd_mul"], AA["fwd_inv"]) / AA["steps"]) / stepA
    # lab calibration: mean over this curve's lab IC rows
    fn, arm = LAB[(g, p)]
    labrows = [json.loads(l) for l in open(os.path.join(ROOT, fn))]
    ic = [r for r in labrows if r.get("kind") == "run" and r.get("arm") == arm and r.get("curve_seed") == cs and r.get("p") == p]
    lab_m = sum(r["cantor_mul"] for r in ic) / sum(r["group_ops"] for r in ic)
    lab_i = sum(r["cantor_inv"] for r in ic) / sum(r["group_ops"] for r in ic)
    my_m_all = (A["fwd_mul"] + A["restart_mul"]) / A["steps"]
    lines.append(
        f"| {g} | {p} | {cs} | {A['fwd_mul']/A['steps']:.1f}, {A['fwd_inv']/A['steps']:.2f} | {lab_m:.1f}, {lab_i:.2f} | {c:.3f} | {nsib:.3f} | {rho_w:.3f} | {aa:.4f} | {B['dup_sib']} | {B['n_mismatch']} | {B['verified']-B['verify_fail']}/{B['verified']} |")
    agg = cellagg[(g, p)]
    for nm, val in [("c", c), ("nsib", nsib), ("rho_w", rho_w), ("aa", aa), ("stepA_mul", A["fwd_mul"] / A["steps"]),
                    ("stepA_inv", A["fwd_inv"] / A["steps"]), ("lab_mul", lab_m), ("lab_inv", lab_i)]:
        agg[nm].append(val)
    for nm in ["fwd_cand", "fwd_split", "sib_cand", "sib_split", "sibs", "steps", "dup_sib", "n_mismatch", "verified", "verify_fail"]:
        agg[nm].append(B[nm])
    agg["aa_split"].append(AA["fwd_split"])
    agg["aa_cand"].append(AA["fwd_cand"])
    agg["aa_steps"].append(AA["steps"])

lines += ["", "## T2 Split rates: forward vs sibling (pooled over curves), A/A", "",
          "| g | p | fwd split rate | sibling split rate | diff [95%] | half-width / rate | A/A diff [95%] | sibling emission rate | dup |",
          "|--:|--:|--:|--:|--|--:|--|--:|--:|"]
for (g, p), agg in sorted(cellagg.items()):
    kf, nf = sum(agg["fwd_split"]), sum(agg["fwd_cand"])
    ks, ns = sum(agg["sib_split"]), sum(agg["sib_cand"])
    a, b, dlt, ci = wald(ks, ns, kf, nf)
    ka, na = sum(agg["aa_split"]), sum(agg["aa_cand"])
    _, _, dAA, ciAA = wald(ka, na, kf, nf)
    hw = (ci[1] - ci[0]) / 2 / b
    em = sum(agg["sibs"]) / sum(agg["steps"])
    lines.append(f"| {g} | {p} | {b:.4f} | {a:.4f} | {dlt:+.4f} [{ci[0]:+.4f}, {ci[1]:+.4f}] | {hw:.3f} | {dAA:+.4f} [{ciAA[0]:+.4f}, {ciAA[1]:+.4f}] | {em:.4f} | {sum(agg['dup_sib'])} |")
    cellout = {k: v for k, v in agg.items()}
    cellout.update(dict(fwd_rate=b, sib_rate=a, diff=dlt, diff_ci=ci, half_width_rel=hw, aa_diff=dAA, aa_ci=ciAA,
                        emission=em, c_mean=stt.mean(agg["c"]), rho_w_mean=stt.mean(agg["rho_w"]),
                        nsib_mean=stt.mean(agg["nsib"])))
    out["cells"][f"g{g}_p{p}"] = cellout

# ---- derived projection on the lab's rows, and distance to matched rho ----
# Corrected after the red-team review (2026-11-23). The first version priced native work at w per field
# multiplication on top of cantor_mul + 10 cantor_inv and left out rho's `native` counter. That is not the
# section 5a band. T3 now uses the lab's own readings from src/bin/genus_ladder_analyze.rs `costs()`:
#   gae_w  = group_ops + w (rel_mul + la_mul + native),  w in {0, 0.1, 1}   (AGENTS.md 5a band)
#   fpm_*  = cantor_mul + I cantor_inv + rel_mul + kappa la_mul             (cross-genus reading)
# with the same arm-mean / rho-mean ratio. The sibling projection scales only the walk's Cantor work:
#   GAE: walk_ops -> walk_ops x rho_w(I=10)   (setup and verify ops unchanged; native unchanged)
#   FPM: cantor_mul + I cantor_inv -> (cantor_mul + I cantor_inv) x rho_w(I)  (rho_w recomputed at each I)
# Rho (R_rho_neg) is unchanged, its `native` work included.


def rho_w_at(g, p, inv_w):
    """Cell-mean B/A Cantor cost per candidate with an inversion priced at inv_w mults."""
    vals = []
    for (gg, pp, cs), d in by.items():
        if (gg, pp) != (g, p):
            continue
        A, B = d["A"], d["B"]
        tA = (A["fwd_mul"] + A["restart_mul"]) + inv_w * (A["fwd_inv"] + A["restart_inv"])
        tB = (B["fwd_mul"] + B["pm_mul"] + B["restart_mul"]) + inv_w * (B["fwd_inv"] + B["pm_inv"] + B["restart_inv"])
        vals.append((tB / (B["steps"] + B["sibs"])) / (tA / A["steps"]))
    return stt.mean(vals)


def fv(r, k):
    v = r.get(k)
    return float(v) if v is not None else 0.0


FPM = (("fpm_low", 1.0, False), ("fpm_mid", 10.0, True), ("fpm_high", 30.0, True))
lines += ["", "## T3 Distance to matched rho under the lab's section 5a GAE band (derived projection, not a measurement)", "",
          "Readings as in `src/bin/genus_ladder_analyze.rs` `costs()`: gae_w = group_ops + w (rel_mul + la_mul + native), "
          "for the IC arm and for rho (`R_rho_neg`, its `native` canonicalisation work included); ratio = arm mean / rho mean "
          "over the same curves and seeds. Sibling projection: the IC arm's walk_ops scaled by rho_w (B/A per candidate, "
          "inversion = 10 mults); setup/verify ops, relation phase and LA unchanged (same relations needed). "
          "The current-arm column is this study's recomputation; it reproduces genus_ladder_scale_20261115/tables_g2.md on the g = 2 rows.", "",
          "| g | p | lab arm | runs | rho_w | w | IC / rho, current arm (gae_w, recomputed here) | IC+sib / rho (projected) | projected / current arm total | group-op share of arm total |",
          "|--:|--:|--|--:|--:|--:|--:|--:|--:|--:|"]
fpm_lines = ["", "## T3b Same rows in the lab's FPM readings (cross-genus; `genus_ladder_analyze` fpm_low/mid/high)", "",
             "fpm = cantor_mul + I cantor_inv + rel_mul + kappa la_mul; low: I = 1, kappa = 1; mid (the lab's primary): I = 10, "
             "kappa = g^2; high: I = 30, kappa = g^2. Rho has no rel/la, so its fpm is its Cantor work only. Sibling projection: "
             "the Cantor part times rho_w recomputed at the same I.", "",
             "| g | p | reading | rho_w(I) | IC / rho | IC+sib / rho (projected) | projected / current |",
             "|--:|--:|--|--:|--:|--:|--:|"]
old_lines = ["", "## T3c Superseded: this study's first pricing (not the section 5a band; kept only for the record)", "",
             "W_cantor + w (rel_mul + la_mul) against rho's W_cantor with rho's `native` left out, w per field multiplication. "
             "The red-team review showed this is not the lab's band; no conclusion in README.md rests on it.", "",
             "| g | p | w | IC / rho | IC+sib / rho | projected / current |", "|--:|--:|--:|--:|--:|--:|"]
out["projection"] = {}
out["projection_method"] = ("section 5a GAE band as genus_ladder_analyze gae_w: group_ops + w (rel_mul + la_mul + native), "
                            "rho native included; sibling scales walk_ops by rho_w. FPM readings as genus_ladder_analyze fpm_*.")
for (g, p), (fn, arm) in sorted(LAB.items()):
    rows = [json.loads(l) for l in open(os.path.join(ROOT, fn))]
    ic = {(r["curve_seed"], r["seed"]): r for r in rows if r.get("kind") == "run" and r.get("arm") == arm and r.get("p") == p}
    rho = {(r["curve_seed"], r["seed"]): r for r in rows if r.get("kind") == "run" and r.get("arm") == "R_rho_neg" and r.get("p") == p}
    keys = sorted(set(ic) & set(rho))
    assert len(keys) == len(ic) == len(rho), "unpaired lab rows"
    rw = out["cells"][f"g{g}_p{p}"]["rho_w_mean"]
    for w in (0.0, 0.1, 1.0):
        base = proj = rh = grp = 0.0
        for k_ in keys:
            r = ic[k_]
            nat = fv(r, "rel_mul") + fv(r, "la_mul") + fv(r, "native")
            go, wo = fv(r, "group_ops"), fv(r, "walk_ops")
            base += go + w * nat
            proj += (go - wo) + wo * rw + w * nat
            grp += go
            rr = rho[k_]
            rh += fv(rr, "group_ops") + w * (fv(rr, "rel_mul") + fv(rr, "la_mul") + fv(rr, "native"))
        lines.append(f"| {g} | {p} | {arm} | {len(keys)} | {rw:.3f} | {w} | {base/rh:.4g} | {proj/rh:.4g} | {proj/base:.4f} | {grp/base:.4f} |")
        out["projection"][f"g{g}_p{p}_w{w}"] = dict(runs=len(keys), ic_over_rho=base / rh, sib_over_rho=proj / rh,
                                                     proj_over_current=proj / base, group_op_share=grp / base, arm=arm, source=fn,
                                                     reading=f"gae_w{w}")
        # superseded pricing, for the record only
        ob = op = orh = 0.0
        for k_ in keys:
            r = ic[k_]
            Wc = cnt(r["cantor_mul"], r["cantor_inv"])
            nat2 = fv(r, "rel_mul") + fv(r, "la_mul")
            ob += Wc + w * nat2
            op += Wc * rw + w * nat2
            orh += cnt(rho[k_]["cantor_mul"], rho[k_]["cantor_inv"])
        old_lines.append(f"| {g} | {p} | {w} | {ob/orh:.4f} | {op/orh:.4f} | {op/ob:.4f} |")
        out["projection"][f"g{g}_p{p}_w{w}_superseded_first_pricing"] = dict(ic_over_rho=ob / orh, sib_over_rho=op / orh,
                                                                              proj_over_current=op / ob)
    for name, I, kg in FPM:
        kap = g * g if kg else 1.0
        rwI = rho_w_at(g, p, I)
        base = proj = rh = 0.0
        for k_ in keys:
            r = ic[k_]
            cw = fv(r, "cantor_mul") + I * fv(r, "cantor_inv")
            rest = fv(r, "rel_mul") + kap * fv(r, "la_mul")
            base += cw + rest
            proj += cw * rwI + rest
            rr = rho[k_]
            rh += fv(rr, "cantor_mul") + I * fv(rr, "cantor_inv") + fv(rr, "rel_mul") + kap * fv(rr, "la_mul")
        fpm_lines.append(f"| {g} | {p} | {name} | {rwI:.3f} | {base/rh:.4g} | {proj/rh:.4g} | {proj/base:.4f} |")
        out["projection"][f"g{g}_p{p}_{name}"] = dict(rho_w=rwI, ic_over_rho=base / rh, sib_over_rho=proj / rh,
                                                       proj_over_current=proj / base, arm=arm, source=fn)
lines += fpm_lines + old_lines

json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1, default=float)
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(lines) + "\n")
print("\n".join(lines))
