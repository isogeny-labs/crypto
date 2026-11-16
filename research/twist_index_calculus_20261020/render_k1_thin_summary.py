#!/usr/bin/env python3
"""Render the validated native thinning grid without altering its evidence."""
from __future__ import annotations

import json
import subprocess
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
TYPST = Path("/tmp/typst-x86_64-unknown-linux-musl/typst")


def main() -> None:
    summary = json.loads((ROOT / "K1_THIN_NATIVE_VALIDATION.json").read_text())
    rows = summary["rows"]
    assert summary["status"] == "pass" and len(rows) == 21
    assert summary["full_rank_recoveries"] == 21
    assert summary["full_base_parity_checks"] == 6
    assert summary["mutation_rejected"]

    fig, axes = plt.subplots(2, 2, figsize=(11, 7.5), layout="constrained")
    colors = ("#31688e", "#35b779", "#de725c")
    for column, p in enumerate((73, 97)):
        for seed in range(3):
            cases = [r for r in rows if r["p"] == p and r["seed"] == seed]
            cases.sort(key=lambda r: r["retained_l2_orbits"])
            x = [r["retained_l2_orbits"] for r in cases]
            axes[0, column].plot(x, [r["full_base_over_candidate_fp_ratio"] for r in cases],
                                 "o-", color=colors[seed], label=f"seed {seed}")
            axes[1, column].plot(x, [r["attempted_s4_systems"] for r in cases],
                                 "o-", color=colors[seed], label=f"seed {seed}")
            for r in cases:
                if r["status"] != "pass":
                    axes[0, column].plot(r["retained_l2_orbits"],
                                         r["full_base_over_candidate_fp_ratio"],
                                         "x", color="black", markersize=8, mew=1.5)
        axes[0, column].axhline(1, color="gray", linewidth=.8, linestyle="--")
        axes[0, column].set_title(f"p = {p}; three frozen target streams")
        axes[0, column].set_ylabel("Full base / candidate counted IC Fp M")
        axes[1, column].set_ylabel("Attempted native S4 systems")
        axes[1, column].set_xlabel("Retained complete L2 orbits")
        for ax in axes[:, column]:
            ax.set_xticks(range(1, 5 if p == 73 else 4))
            ax.grid(alpha=.2)
            ax.legend(fontsize=8)
    fig.suptitle("Native quartic twist orbit thinning: 21 one-target cases; x marks partial relation sets")
    fig.savefig(ROOT / "k1_thin_native_overview.svg")
    fig.savefig(ROOT / "k1_thin_native_overview.png", dpi=170)
    plt.close(fig)

    partial = [r for r in rows if r["status"] != "pass"]
    assert len(partial) == 4 and all(r["p"] == 73 and r["seed"] == 1 for r in partial)
    best73 = max((r for r in rows if r["p"] == 73 and r["retained_l2_orbits"] < 4),
                 key=lambda r: r["full_base_over_candidate_fp_ratio"])
    best97 = max((r for r in rows if r["p"] == 97 and r["retained_l2_orbits"] < 3),
                 key=lambda r: r["full_base_over_candidate_fp_ratio"])
    md = ["# Native orbit-complete thinning in quartic-twist polynomial index calculus", "",
          "23 October 2026. This is the complete native test of the 21 choices frozen in the "
          "[protocol](QUARTIC_TWIST_LINE_PROTOCOL.md), following the "
          "[ideal exact-oracle survey](K1_ORBIT_THINNING.md). The public synthetic "
          "target is `[271]G` on each of six frozen p=73/97 residual streams. "
          "The [run index](K1_THIN_NATIVE_RUNS.md) links all 42 first/A/A raw records, "
          "independent audits, plots, PDFs, and exact source and record hashes. "
          "The [machine validator](K1_THIN_NATIVE_VALIDATION.json) recomputes "
          "the grid summary from those records.", "",
          "## Question and method", "",
          "The 24-action on the quartic twist makes each complete nonzero `L2` orbit "
          "12 physical factors but one folded log column. The restricted mixed `k=1` "
          "collector peels each signed `L2` factor, solves one symmetrised `S4` "
          "system for three `L0` factors, and keeps every rank-increasing verified "
          "relation. Thinning reduces systems and unknown columns, but can reduce "
          "relation yield and increase the number of residuals needed. The ideal "
          "oracle predicted the tradeoff; this native run executes the polynomial "
          "solver on every preselected subset, without selecting an apparent winner "
          "after seeing its hits.", "",
          "Every candidate retains all `L0` points and the first `j` complete `L2` "
          "orbits by original physical index. It constructs the full base and verified "
          "full orbit map first, then charges the proper-subset map too. The target, "
          "full-base signs, residual `(a,b)` order, interpolation RNG, solver, row "
          "selection, 8 GiB address-space cap and 1,200 s per-process watchdog are "
          "frozen. Each first run has an A/A repeat. The independent point-keyed "
          "auditor reconstructs each subset from the original full base, computes "
          "complete signed sets for every processed residual, independently obtains "
          "factor logs by BSGS, rebuilds the folded matrix, and checks `[271]G=Q`.", "",
          "![Counted field-work ratios and attempted S4 systems across every native subset](k1_thin_native_overview.svg)", "",
          "## Every native one-target result", "",
          "`Exact/poly` compares complete signed group relations with the native "
          "polynomial output on the *processed prefix*. `Full/candidate` is the "
          "same-seed full-base counted IC `Fp M` divided by the candidate count. "
          "`IC/rho` is candidate counted `Fp M` divided by the same-target "
          "twist24 rho count. Neither ratio is a priced `S` or timing ratio. "
          "Every row reaches full anchored rank and independently recovers 271.", "",
          "| p/seed | L2 orbits | Physical base / columns | Residuals | Exact/poly | S4 attempts/failures | Counted IC Fp M | IC group ops | Full/candidate Fp M | IC/rho Fp M | Audit |",
          "|:--|--:|:--|--:|:--|:--|--:|--:|--:|--:|:--|"]
    for r in rows:
        raw = f"quartic_twist_k1_thin_p{r['p']}_seed{r['seed']}_j{r['retained_l2_orbits']}_v1.json"
        base = json.loads((ROOT / raw).read_text())
        md.append(f"| {r['p']}/{r['seed']} | {r['retained_l2_orbits']} | "
                  f"{base['base_size']}/{base['folded_columns']} | {r['processed_queries']} | "
                  f"{r['exact_prefix_relations']}/{r['polynomial_prefix_relations']} | "
                  f"{r['attempted_s4_systems']:,}/{r['system_failures']} | "
                  f"{r['ic_counted_fp_muls']:,} | {r['ic_counted_group_ops']:,} | "
                  f"{r['full_base_over_candidate_fp_ratio']:.3f}× | "
                  f"{r['ic_counted_fp_muls']/r['rho_counted_fp_muls']:,.0f}× | "
                  f"[{r['status']}]({raw.replace('_v1.json', '_audit_v1.json')}) |")
    md += ["", "The largest *observed* same-seed full/candidate counted `Fp M` "
           f"ratio among proper subsets is {best73['full_base_over_candidate_fp_ratio']:.3f}× "
           f"at p=73 seed {best73['seed']}, j={best73['retained_l2_orbits']}, "
           f"and {best97['full_base_over_candidate_fp_ratio']:.3f}× "
           f"at p=97 seed {best97['seed']}, j={best97['retained_l2_orbits']}. "
           "These maxima are descriptive post hoc statistics; they are not "
           "a preselected one-target speed claim. The plotted curves and table "
           "retain every slower or partial choice.", "",
           "## Audit and failure accounting", "",
           f"The validator confirms {summary['full_rank_recoveries']}/21 full-rank "
           "scalar recoveries, exact phase-sum field and group counters, 21 empty "
           "first/A/A non-timing diffs, byte-identical independent audit outputs, "
           "six exact full-base source parities against the older producer, and "
           "a rejected subset-index mutation. It checks the ideal oracle's exact "
           "prefix count, the S4 peel-attempt count, and every recorded exit code. "
           f"There are {summary['exact_relation_set_runs']}/21 exact native "
           "relation-set cases. All four p=73 seed-one subset cases retain one "
           "failed S4 system and miss two real signed relations on their "
           "processed prefix; all still recover the target. They are marked "
           "`partial`, not exact relation-finder passes.", "",
           "## Cost boundary and interpretation", "",
           "The native counted IC `Fp M` interval includes model, twist geometry "
           "and eigenvalues, full-base construction, the full orbit map, proper "
           "subset orbit map, S4 precomputation, residual generation, every "
           "attempted polynomial system, matrix-path verification and answer "
           "checks. Group additions and modular matrix work are separate counters. "
           "The public known-answer input construction is explicit but excluded "
           "from both compared target-solving intervals. The ideal exact oracle "
           "is a reference audit, not candidate relation work.", "",
           "Hash/tree and allocator CPU, requested peak memory, rho native hash "
           "CPU, calibrated group/field exchange band, isolated interleaved "
           "timing and same-input msolve/Magma controls remain unmeasured. The "
           "field-work ratios are a measured arithmetic-path comparison on the "
           "frozen small-field cases, not a priced whole-method `S`, wall-time "
           "or external-solver result. The per-row IC/rho counted field-work "
           "gap remains large, so these data do not establish an ECDLP "
           "advantage over matched rho or an asymptotic crossover. The open "
           "[pricing and solver-completeness problem](../../problems/quartic-twist-polynomial-ic-pricing.md) "
           "records the remaining gates.", ""]
    (ROOT / "K1_THIN_NATIVE_REPORT.md").write_text("\n".join(md))

    typ = ['#set page(paper: "a4", flipped: true, margin: 14mm)',
           '#set text(size: 8.2pt)', '= Native orbit-complete quartic-twist polynomial IC',
           '23 October 2026. Public synthetic one-target p=73/97 panel; 21 preregistered '
           'native subset cases, three seeds per curve; Q=[271]G.',
           'The full Markdown report, frozen protocol, raw records, separate audits and '
           'run PDFs are in `K1_THIN_NATIVE_REPORT.md` and `K1_THIN_NATIVE_RUNS.md`.',
           '== Mechanism and comparison',
           'One complete L2 twist orbit has 12 physical factors but one folded log column. '
           'The k=1 collector peels signed L2 factors and solves S4 on three L0 factors. '
           'The full physical base and map are charged even for subsets. Full/candidate '
           'below is same-seed counted IC Fp multiplications; IC/rho uses same-target '
           'twist24 rho, also in counted Fp multiplications.',
           '#image("k1_thin_native_overview.svg", width: 82%)',
           '== Complete native grid',
           '#table(columns: (1fr, .6fr, .65fr, .8fr, .75fr, 1.1fr, 1fr, .9fr, .65fr), '
           'inset: 2.5pt,',
           '[*p/seed*], [*j*], [*rank*], [*query*], [*exact/poly*], '
           '[*S4 attempts/fail*], [*IC Fp M*], [*full/IC*], [*audit*],']
    for r in rows:
        typ.append(f'[{r["p"]}/{r["seed"]}], [{r["retained_l2_orbits"]}], '
                   f'[{r["rank"]}], [{r["processed_queries"]}], '
                   f'[{r["exact_prefix_relations"]}/{r["polynomial_prefix_relations"]}], '
                   f'[{r["attempted_s4_systems"]:,}/{r["system_failures"]}], '
                   f'[{r["ic_counted_fp_muls"]:,}], '
                   f'[{r["full_base_over_candidate_fp_ratio"]:.3f}], '
                   f'[{r["status"]}],')
    typ += [')', 'All 21 runs reach full anchored rank and independently recover 271. '
            'The four p=73 seed-one runs miss two real signed relations from one failed '
            'S4 system and remain partial relation finders. First/A/A non-timing '
            'records, independent audits, six full-base source parities and a '
            'mutation rejection pass.',
            '== Claim boundary',
            'Hash/tree and allocator CPU, peak memory, calibrated GAE, isolated '
            'ABAB timing and same-input msolve/Magma are missing. Counted field '
            'ratios are not priced S or wall-time speed ratios. No matched-rho '
            'crossover or deployed-curve claim follows. The complete Markdown '
            'table includes separate group counts and IC/rho field-work gaps.']
    (ROOT / "K1_THIN_NATIVE_REPORT.typ").write_text("\n".join(typ) + "\n")
    subprocess.run([str(TYPST), "compile", "K1_THIN_NATIVE_REPORT.typ",
                    "K1_THIN_NATIVE_REPORT.pdf"], cwd=ROOT, check=True)
    print(f"rendered {len(rows)} native cases and report PDF")


if __name__ == "__main__":
    main()
