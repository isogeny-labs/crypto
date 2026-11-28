#!/usr/bin/env python3
"""Lay out the recorded values of research/tuned_gae_20261020 as graphs.

Plots recorded values only (AGENTS.md "no Python": nothing here computes a
result). Inputs, all committed beside this directory:
  sweep/gae_reference.jsonl         gae-reference sweep (figure 1)
  reprice/table_b.json              gae-reference reprice-ledger (figure 2)
  tables/<session>.table.md         `ecbench table --dir sessions/<session>` (figure 3)
Run from the repository root:
  python3 research/tuned_gae_20261020/figures/make_figures.py
Writes fig1_ns_per_add.{svg,png}, fig2_table_b.{svg,png}, fig3_rho_prime_fast.{svg,png}
(PNG at 150 dpi, from the same code as the SVG).
"""
import json
import pathlib
import re

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
STUDY = HERE.parent
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25})
C_REF, C_TUN, C_NEG, C_OLD, C_NEW = "#d95f02", "#1b9e77", "#555555", "#e7298a", "#1b9e77"


def save(fig, name):
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


def fig1():
    rows = [json.loads(l) for l in open(STUDY / "sweep/gae_reference.jsonl")]
    fig, axs = plt.subplots(1, 2, figsize=(10, 4), sharey=True)
    for ax, fam, title in ((axs[0], "prime", "Prime field (Montgomery u64)"),
                           (axs[1], "binary", "GF(2^n) (pclmulqdq)")):
        rs = sorted((r for r in rows if r["family"] == fam), key=lambda r: r["field_bits"])
        for r in rs:
            x = r["field_bits"]
            ax.plot([x, x], [r["ns_per_add_tuned_min"], r["ns_per_add_tuned_max"]], color=C_TUN, lw=1, alpha=.6)
            lo, hi = min(r["ns_per_add_reference_runs"]), max(r["ns_per_add_reference_runs"])
            ax.plot([x, x], [lo, hi], color=C_REF, lw=1, alpha=.6)
        mk = lambda r: "s" if r["curve"].startswith("koblitz") else "o"
        for m, lab in (("o", "random / generated"), ("s", "Koblitz")):
            sel = [r for r in rs if mk(r) == m]
            if not sel:
                continue
            ax.scatter([r["field_bits"] for r in sel], [r["ns_per_add_reference"] for r in sel], marker=m, color=C_REF,
                       s=26, label=f"reference add (CountedGroup::add), {lab}", zorder=3)
            ax.scatter([r["field_bits"] for r in sel], [r["ns_per_add_tuned"] for r in sel], marker=m, color=C_TUN,
                       s=26, label=f"tuned batched add, {lab}", zorder=3)
        ax.set_yscale("log")
        ax.set_xlabel("field bits")
        ax.set_title(title)
        ax.legend(fontsize=7, loc="upper left" if fam == "prime" else "center left")
    axs[0].set_ylabel("ns per affine addition (median of passes; bars: min-max)")
    fig.suptitle("Fig. 1. Reference vs tuned affine addition by field size (gae-reference sweep, 33 rows, "
                 "Xeon @ 2.80 GHz, L0 shared VM)", fontsize=9)
    fig.tight_layout()
    save(fig, "fig1_ns_per_add")


def short(inst):
    return (inst.replace("random-binary-", "bin-").replace("generated-", "gen-")
            .replace("-b524b", "").replace("-b6507", "").replace("-b1b6f3b", "")
            .replace("-b5fc9da", "").replace("-b845462", "").replace("-3290411", "").replace("-10935329", ""))


def fig2():
    t = json.load(open(STUDY / "reprice/table_b.json"))
    ins = t["instances"]
    names = [short(i["instance"]) for i in ins]
    lg = [i["legacy_best"]["legacy_s_recorded"] for i in ins]
    tn = [i["legacy_best"]["tuned_s"] for i in ins]
    glg = [i["legacy_best"]["legacy_gap_to_rho"] for i in ins]
    gtn = [i["legacy_best"]["tuned_gap_to_rho"] for i in ins]
    x = range(len(ins))
    w = .38
    fig, axs = plt.subplots(2, 1, figsize=(10, 6.2), sharex=True)
    axs[0].bar([i - w / 2 for i in x], lg, w, color=C_REF, label="S, legacy unit (recorded)")
    axs[0].bar([i + w / 2 for i in x], tn, w, color=C_TUN, label="S, tuned unit (same recipe, repriced)")
    axs[0].set_yscale("log")
    axs[0].set_ylabel("IC S (GAE per solve)")
    axs[0].legend(fontsize=8)
    axs[1].bar([i - w / 2 for i in x], glg, w, color=C_REF, label="gap to matched rho, legacy")
    axs[1].bar([i + w / 2 for i in x], gtn, w, color=C_TUN, label="gap to matched rho, tuned")
    axs[1].set_ylabel("IC S / matched rho S")
    axs[1].legend(fontsize=8)
    axs[1].set_xticks(list(x))
    axs[1].set_xticklabels(names, rotation=35, ha="right")
    fig.suptitle("Fig. 2. Table B best recipe per instance under the legacy and the tuned GAE unit "
                 "(reprice/table_b.json; matched rho unchanged; bench = prime, bin = random binary)", fontsize=9)
    fig.tight_layout()
    save(fig, "fig2_table_b")


def parse(session):
    out = {}
    for l in open(STUDY / f"tables/{session}.table.md"):
        c = [s.strip() for s in l.strip().strip("|").split("|")]
        if len(c) < 15 or not c[0].startswith("ECBS"):
            continue
        lo, hi = re.findall(r"[\d.]+", c[7])[:2]
        out[(c[2], c[3])] = dict(log2r=float(c[4]), s=float(c[6]), lo=float(lo), hi=float(hi), wall=float(c[13]))
    return out


def fig3():
    old, new = parse("rho_prime_fast"), parse("rho_prime_fast_v2_n256")
    slugs = sorted({k[1] for k in new}, key=lambda s: new[("rho.negation", s)]["log2r"])
    arms = [("rho.negation", new, C_NEG, "rho.negation (v2_n256 session, 256 targets/curve)"),
            ("rho.prime_fast", old, C_OLD, "rho.prime_fast, old config (rho_prime_fast, 32 targets x 3 rounds/curve)"),
            ("rho.prime_fast", new, C_NEW, "rho.prime_fast, tuned config (rho_prime_fast_v2_n256, 256 targets/curve)")]
    fig, axs = plt.subplots(1, 2, figsize=(10, 4.2))
    w = .26
    for j, (m, src, col, lab) in enumerate(arms):
        xs = [i + (j - 1) * w for i in range(len(slugs))]
        d = [src[(m, s)] for s in slugs]
        axs[0].bar(xs, [v["s"] for v in d], w, color=col, label=lab,
                   yerr=[[v["s"] - v["lo"] for v in d], [v["hi"] - v["s"] for v in d]], capsize=2, ecolor="k")
        axs[1].bar(xs, [v["wall"] for v in d], w, color=col, label=lab)
    labels = [f"{s.split('-')[1]}\n(log2 r {new[('rho.negation', s)]['log2r']:.1f})" for s in slugs]
    for ax in axs:
        ax.set_xticks(range(len(slugs)))
        ax.set_xticklabels(labels, fontsize=8)
        ax.set_xlabel("prime field (bits; icv1 slugs fp28, fp32, fp36, fp40)")
    axs[0].axhline(0.886, color="k", ls=":", lw=1)
    axs[0].text(3.45, 0.80, "floor 0.886", fontsize=7, ha="right")
    axs[0].set_ylim(0, 3.0)
    axs[0].set_ylabel("mean S (GAE per solve), 95% interval as recorded")
    axs[0].legend(fontsize=6.5, loc="upper left")
    axs[1].set_yscale("log")
    axs[1].set_ylabel("median online solve wall (ms), L0")
    fig.suptitle("Fig. 3. rho.negation vs rho.prime_fast, old and tuned config (ecbench table, "
                 "all records verified, L0, --cpus none --allow-busy)", fontsize=9)
    fig.tight_layout()
    save(fig, "fig3_rho_prime_fast")


if __name__ == "__main__":
    fig1()
    fig2()
    fig3()
