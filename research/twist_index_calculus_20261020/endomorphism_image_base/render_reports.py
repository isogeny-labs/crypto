#!/usr/bin/env python3
"""Document layout only: every numerical result is copied from native reports.

Use a scratch venv with matplotlib, typst and pymupdf. No solver, statistical
estimator, or numerical validator belongs in this file.
"""
from __future__ import annotations

import csv
import importlib.util
import json
import shutil
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.colors import LogNorm
import typst

ROOT = Path(__file__).resolve().parent
DATA = ROOT / "results"
FIG = ROOT / "figures"
ARMS = ["original", "psi-control", "phi-l2", "phi-l0", "union-l2",
        "union-all", "union-l0", "phi-l0-active", "l0-peel", "double-l0-active"]
LABELS = ["Original", "Psi control", "Phi L2", "Phi L0 / full cols",
          "Union L2", "Union all", "Union L0", "Phi L0 active", "Pure L0", "Doubling active"]
COLORS = {"original": "#68717a", "phi-l0-active": "#147d92",
          "l0-peel": "#b07c28", "double-l0-active": "#9b4380"}


def read(p):
    return json.loads(p.read_text())


def saved(fig, directory, name):
    directory.mkdir(exist_ok=True, parents=True)
    svg = directory / f"{name}.svg"
    fig.savefig(svg)
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(directory / f"{name}.png", dpi=150)
    plt.close(fig)


spec = importlib.util.spec_from_file_location("prior_layout", ROOT.parent / "s4_twist_elimination/render_reports.py")
layout = importlib.util.module_from_spec(spec)
spec.loader.exec_module(layout)


def pdf(md_path):
    source = layout.typst(md_path.read_text(), md_path.parent)
    # Embed the matching 150 dpi render. Some Typst SVG imports misplace
    # this diagram's vector glyphs; the SVG remains a separate figure.
    source = source.replace('.svg", width:', '.png", width:')
    typ_path = md_path.with_suffix(".typ")
    typ_path.write_text(source)
    typst.compile(str(typ_path), output=str(md_path.with_suffix(".pdf")), root=str(ROOT))


def diagram(directory, name, title, subtitle, execution=False):
    fig, ax = plt.subplots(figsize=(10, 3.25), layout="constrained")
    ax.axis("off")
    nodes = [(0.15, "Declared prime subgroup H\nL0 factors and known coefficients"),
             (0.5, "Explicit image map\nPhi = 1 - psi or doubling"),
             (0.85, "Complete S4 peels\nDeduplicate rows; verify rank")]
    if execution:
        nodes = [(0.15, "Saved source / launch command"),
                 (0.5, "Recorded execution stage\nSee the precise outcome below"),
                 (0.85, "Saved exits and artifacts\nNo pass inferred from job state")]
    for x, label in nodes:
        ax.text(x, .58, label, transform=ax.transAxes, ha="center", va="center", fontsize=9,
                bbox={"boxstyle": "round,pad=.7", "facecolor": "#eef3f7", "edgecolor": "#245e7d"})
    for x in (.32, .67):
        ax.annotate("", xy=(x+.05, .58), xytext=(x-.05, .58), xycoords="axes fraction",
                    arrowprops={"arrowstyle": "->", "color": "#245e7d"})
    ax.text(.5, .12, subtitle, transform=ax.transAxes, ha="center", fontsize=9, wrap=True)
    fig.suptitle(title, fontsize=11)
    saved(fig, directory, name)


def figures(s, external):
    diagram(FIG, "mechanism", "Mixed-component image base: actual relation-finding experiment",
            "All factors stay in H. Image logs use the original columns. Orbit copies alone add no projective row.")
    a = s["arm_totals"]
    x = list(range(len(a)))
    fig, axes = plt.subplots(1, 2, figsize=(11, 4.5), layout="constrained")
    axes[0].bar(x, [r["unique_signed_tuples"] for r in a], color="#147d92")
    axes[1].bar(x, [r["unique_projective_augmented_rows"] for r in a], color="#9b4380")
    for ax, label in zip(axes, ["Unique signed physical tuples (count)", "Distinct projective augmented rows (count)"]):
        ax.set_ylabel(label)
        ax.set_xticks(x, LABELS, rotation=62, ha="right", fontsize=7)
        ax.grid(axis="y", alpha=.2)
    fig.suptitle("All 9,658 frozen residuals / 21 seeded records; exact independent set and rank checks", fontsize=10)
    saved(fig, FIG, "full-stream-yield")
    pairs = [r for r in s["paired_costs"] if r["reference_arm"] == "double-l0-active" and r["candidate_arm"] == "phi-l0-active"]
    assert len(pairs) == 21 and all(r["admitted"] for r in pairs)
    fig, ax = plt.subplots(figsize=(11, 4), layout="constrained")
    for i, r in enumerate(pairs):
        v = r["reference_over_candidate_cold"]
        ax.plot([i, i], [v["min"], v["max"]], color="#147d92")
        ax.plot(i, v["median"], "o", color="#147d92")
    ax.axhline(1, color="#68717a", linestyle="--")
    ax.set_yscale("log")
    ax.set_ylabel("Doubling cold / phi cold (paired elapsed ratio)")
    ax.set_xticks(list(range(21)), [f"{r['p']}/{r['seed']}" for r in pairs], rotation=55, ha="right", fontsize=8)
    ax.set_xlabel("Prime p / seed; five complete same-origin paired rounds per record")
    ax.grid(axis="y", alpha=.2)
    ax.set_title("Dots: median paired ratio; bars: observed five-round range. Above 1 favors phi.", fontsize=10)
    saved(fig, FIG, "paired-cold-cost")
    rows = s["cold_rows"]
    assert len(rows) == 210 and all(r["isolation_admitted"] for r in rows)
    cases = [(r["p"], r["seed"]) for r in rows if r["arm"] == "original"]
    by = {(r["p"], r["seed"], r["arm"]): r for r in rows}
    fig, ax = plt.subplots(figsize=(12, 7), layout="constrained")
    # Unit conversion for display only; no estimator is calculated here.
    im = ax.imshow([[by[p, seed, arm]["median_cold_ns"] / 1e9 for arm in ARMS] for p, seed in cases], aspect="auto", cmap="YlGnBu", norm=LogNorm())
    for i, (p, seed) in enumerate(cases):
        ax.text(3, i, "*", ha="center", va="center", color="#a33131", fontsize=12)
    ax.set_xticks(list(range(10)), LABELS, rotation=48, ha="right", fontsize=8)
    ax.set_yticks(list(range(21)), [f"{p}/{seed}" for p, seed in cases], fontsize=8)
    ax.set_ylabel("Prime p / seed")
    ax.set_title("Median instrumented cold elapsed seconds, five admitted rounds; * rank-deficient cap, not a solution", fontsize=10)
    fig.colorbar(im, ax=ax, label="Seconds, logarithmic colors (descriptive medians)")
    saved(fig, FIG, "all-cold-costs")
    with (DATA / "rank-traces.csv").open() as f:
        traces = list(csv.DictReader(f))
    for part, subset in enumerate((cases[:12], cases[12:]), 1):
        fig, axes = plt.subplots(4 if part == 1 else 3, 3, figsize=(11, 10 if part == 1 else 8), layout="constrained")
        for ax, (p, seed) in zip(axes.flat, subset):
            for arm, color in COLORS.items():
                q = [r for r in traces if int(r["p"]) == p and int(r["seed"]) == seed and r["arm"] == arm]
                ax.step([int(r["queries"]) for r in q], [int(r["rank"]) for r in q], where="post", color=color, label=arm, linewidth=1)
            ax.set_title(f"p={p}, seed={seed}", fontsize=9)
            ax.set_xlabel("Queries consumed (count)", fontsize=8)
            ax.set_ylabel("Matrix rank", fontsize=8)
            ax.tick_params(labelsize=7)
            ax.grid(alpha=.2)
        handles, labels = axes.flat[0].get_legend_handles_labels()
        fig.legend(handles, labels, loc="outside lower center", ncol=4, fontsize=8)
        fig.suptitle("Full frozen-stream rank traces; verified independently. Pure / image active columns differ from original.", fontsize=10)
        saved(fig, FIG, f"rank-progress-{part}")
    fig, ax = plt.subplots(figsize=(11, 4), layout="constrained")
    for i, r in enumerate(external["groups"]):
        v = r["native_over_msolve"]
        ax.plot([i, i], [v["min"], v["max"]], color="#9b4380")
        ax.plot(i, v["median"], "o", color="#9b4380")
    ax.axhline(1, linestyle="--", color="#68717a")
    ax.set_xticks(list(range(21)), [f"{r['p']} / {r['arm']}" for r in external["groups"]], rotation=65, ha="right", fontsize=7)
    ax.set_ylabel("Native / msolve internal GB time (ratio)")
    ax.set_title("18 fixed GB inputs per group; five-round median and observed range; above 1 favors msolve", fontsize=10)
    ax.grid(axis="y", alpha=.2)
    saved(fig, FIG, "external-gb")


def run_reports(external):
    index = ["# Reports for every launched study job", "", "All terminal jobs, including interruptions, resource refusals and build failures. Raw data: [ARTIFACTS.md](../ARTIFACTS.md).", "", "| Job | State / report |", "| --- | --- |"]
    for entry in read(DATA / "job-observations/index.json")["jobs"]:
        r = read(DATA / "job-observations" / f"{entry['job']}.json")
        job = r["job"]
        d = ROOT / "reports" / job
        d.mkdir(parents=True, exist_ok=True)
        if (d / "README.pdf").exists():
            # Rendering another terminal job does not rewrite an earlier run.
            index.append(f"| {job} | [{r['status']['phase']}](./{job}/README.md) / [PDF](./{job}/README.pdf) |")
            continue
        records = [v for v in r["cold_records"] if v["resource_admitted"] and not v["copied_from_other_job"]]
        if len(records) >= 2:
            fig, ax = plt.subplots(figsize=(10, 4), layout="constrained")
            for arm, label in zip(ARMS, LABELS):
                hit = [v for v in records if v["arm"] == arm]
                if hit:
                    ax.plot([f"{v['p']}/{v['seed']}/{v['pass']}" for v in hit], [v["cold_ns"] / 1e9 for v in hit], "x" if arm == "phi-l0" else ".", label=label)
            ax.set_yscale("log")
            ax.set_ylabel("Admitted instrumented cold elapsed seconds")
            ax.set_xlabel("Recorded prime / seed / pass (categorical; not a shared-target batch)")
            ax.tick_params(axis="x", labelrotation=75, labelsize=5)
            ax.legend(fontsize=6, ncol=3)
            ax.grid(axis="y", alpha=.2)
            ax.set_title(job + "\nAll originating admitted records; x marks the censored full-column control", fontsize=9)
            saved(fig, d, "graph")
        elif len(r["diagnostic_records"]) >= 2:
            fig, axes = plt.subplots(1, 2, figsize=(10, 3.8), layout="constrained")
            for arm in ARMS:
                hit = [(i, v) for i, v in enumerate(r["diagnostic_records"]) if v["arm"] == arm]
                if hit:
                    axes[0].plot([i for i, v in hit], [v["signed_relations"] for i, v in hit], ".", label=arm, markersize=3)
                    axes[1].plot([i for i, v in hit], [v["rank"] for i, v in hit], ".", markersize=3)
                    axes[1].plot([i for i, v in hit], [v["columns"] for i, v in hit], "x", color="#68717a", markersize=3)
            axes[0].set_ylabel("Recorded signed relations (count)")
            axes[1].set_ylabel("Recorded rank (dots) / columns (x)")
            for ax in axes:
                ax.set_xlabel("Summary record ordinal; exact paths in native JSON")
                ax.grid(axis="y", alpha=.2)
            axes[0].legend(fontsize=5, ncol=2)
            fig.suptitle(job + "\nDiagnostic counts only; reference full-stream and candidate-prefix populations are labeled in JSON", fontsize=9)
            saved(fig, d, "graph")
        elif r["kind"] == "external":
            for suffix in ("svg", "png"):
                shutil.copyfile(FIG / f"external-gb.{suffix}", d / f"graph.{suffix}")
        else:
            diagram(d, "graph", job, "Execution diagram: fewer than two comparable numerical observations. Actual stage and outcome are recorded below.", execution=True)
        status = r["status"]
        md = f"""# {r['kind']}: recorded execution

Job `{job}`. Outcome: **{status['phase']}**, recorded exit `{status.get('exit_code')}`.
Host class: `{status.get('instance_type', 'not recorded')}`, lifecycle `{status.get('lifecycle', 'not recorded')}`.
Started `{status.get('started', 'not recorded')}`; last status update `{status.get('updated', 'not recorded')}`.

{r['notes']}

![Recorded observations](graph.svg)

## Command, controls and evidence

Exact command SHA-256: `{r.get('command_sha256')}`. The command, native raw output, exit receipts, setup/build logs and host metadata are in `runs/jobs/{job}/` after extracting the content-addressed archive. Job snapshots record the source revision; frozen cold source is bundled separately. The registry specifies all arms, fields and seeds through these commands; the panel protocol and its supplements define the controls and caps.

This job has {r['admitted_timings_originating_here']} originating resource-admitted cold-pipeline timed records. Copied checkpoints are not additional samples. A cold graph displays every originating admitted cold record, identified by the native JSON. A diagnostic count graph displays the recorded summary's signed relations and rank/columns, with each population labeled in the JSON; it establishes no wall-time advantage. Replayed summaries are deterministic checks rather than independent targets. Medians and paired ranges used for the final comparison are independently recomputed across complete whole rounds in `results/summary.json`; a per-record resource pass alone does not admit an incomplete round. The full-column phi-L0 arm is censored consumed cost, not a verified solution. External GB timers are a separate population described by the external report, not included in the cold-record count.

Recorded exit-count map: `{json.dumps(r['exit_counts'], sort_keys=True)}`.
Recorded summary-status map: `{json.dumps(r['summary_status_counts'], sort_keys=True)}`.
Recorded independent audit-status map: `{json.dumps(r['audit_status_counts'], sort_keys=True)}`.

Raw observations: `results/job-observations/{job}.json`. Partial JSON records, failed exits and refused isolation are retained in the raw archive. Missing isolation is missing, never an admitted time. No performance conclusion follows from a job's done/0 state alone. Build and test gate durations are not algorithm timings.

See [protocol](../../PROTOCOL.md), [cost boundary](../../BOUNDARY.md), [final report](../../README.md) and [raw reproduction](../../ARTIFACTS.md). The msolve job alone measures internal GB timers on 378 fixed systems; it does not run a complete external DLP solver. Other jobs with fewer than two comparable admitted timings use an execution diagram rather than an invented numerical plot.
"""
        (d / "README.md").write_text(md)
        pdf(d / "README.md")
        index.append(f"| {job} | [{status['phase']}](./{job}/README.md) / [PDF](./{job}/README.pdf) |")
    (ROOT / "reports/INDEX.md").write_text("\n".join(index) + "\n")


def main_report(s, e):
    totals = s["arm_totals"]
    pdiag = s["panel_diagnostics"]
    pairs = [r for r in s["paired_costs"] if r["reference_arm"] == "double-l0-active" and r["candidate_arm"] == "phi-l0-active"]
    cold = {(r["p"], r["seed"], r["arm"]): r for r in s["cold_rows"]}
    md = f"""# Twist image bases: completed relation, rank and cold-cost controls

2026-10-23. Seven quartic extension fields, three seeded records each, ten arms. The requested resumed 189-case run and 21 doubling controls are complete, with 210 independent exact-set/rank/recovery audits. Every polynomial run has zero undetermined systems. All 105 complete timed units (ten arms plus original A/A) and all 630 non-timing replays have been reconciled after Spot interruption. Resource admission is per record and whole round, not inferred from job success.

## What is attributable to the twist?

On the identical 9,658 frozen residuals, phi=(1-psi) supplies **{pdiag['phi_minus_double_unique_signed_tuples']} extra signed physical tuples**, a measured diagnostic change of **{pdiag['phi_signed_tuple_change_percent']:.4f}%**, relative to the same-size doubling image. Its distinct projective augmented row delta is **{pdiag['phi_minus_double_unique_projective_rows']}** ({pdiag['phi_projective_row_change_percent']:.4f}%). Both active bases reach full rank in 21/21 cases and recover the target and every usable factor logarithm. Distinct projective rows need not be linearly independent.

Through each arm's first full-rank query, phi consumes **{pdiag['phi_system_count_change_percent']:.4f}% more finite polynomial systems** in aggregate than doubling. These aggregates describe separate targets; they are not an amortized DLP price. Per-target complete elapsed ratios below go in both directions. This experiment establishes no consistent twist-specific complete-cost advantage. A geometry change, extra decompositions and a lower stopping cost are separate properties.

The mixed-component image base is explicitly constructed and verified. Its L0+L3 image is disjoint from the old L0/L2 base, and its logs use existing columns. Generic doubling also creates a disjoint known-log image of the same size. The exact phi coordinate equation is degree three; the current peeled S4 solver does not retain that equation in its solve. See [the mathematical argument](MATHEMATICS.md) and certificate `IDC1h7e0104841476f369`.

![Mechanism](figures/mechanism.svg)

## Requested requirements and evidence

| Requirement | Status and evidence |
| --- | --- |
| Finish resumed 189 cases and deterministic A/A | Verified complete: 189 first and 189 A/A exits 0, byte-identical outputs |
| Finish the doubling control | Verified complete: 21 reference and 21 polynomial cases, first/A/A |
| Unique physical and folded relation yield | Verified complete: exact same 9,658 residuals; independent signed-set equality |
| Matrix rank and recovery | Verified complete: 210 independent audits, 189 successful full-rank cases and 21 deliberately inactive-column censored controls |
| Complete cost of this instrumented implementation | Verified complete under [actual fixture boundary](BOUNDARY.md): 105 admitted whole timed rounds and 630 non-timing replays |
| Twist-specific attribution | Measured comparison against psi, pure L0 and generic doubling; no uniform positive result |
| Complete GAE price, matched strong rho, independent scalar sampling, growing-size security conclusion | Unresolved; not supplied by these controls |
| External complete GB comparison | 378 fixed systems, eight passes, {e['executions']} checked basis pairs; component comparison only |

## Unique yield on the full frozen stream

![Full-stream yield](figures/full-stream-yield.svg)

| Arm | Unique signed tuples | Distinct projective rows | Finite signed peels | Full rank / 21 |
| --- | --- | --- | --- | --- |
"""
    for r in totals:
        md += f"| {r['arm']} | {r['unique_signed_tuples']} | {r['unique_projective_augmented_rows']} | {r['finite_signed_peels']} | {r['full_rank_cases']} |\n"
    md += """
The psi control reproduces exactly the original signed sets: automorphism copies do not add folded rank. The full-column phi-L0 arm retains unused L2 unknowns and is rank deficient in every case. The active-column arm checks that discarded coefficients are zero; it solves every case. Unions gain raw relations by doing more systems and carrying more columns; they must be priced by their complete stopping costs.

## Cost per public target: twist image versus doubling

One target per row, five paired whole rounds, alternating arm order. Ratio = doubling cold / phi cold within each same-origin round. The table gives the median and observed range of those five ratios, not a ratio of independently selected fastest runs. Above 1 favors phi. These are complete instrumented elapsed intervals for the supplied coordinate-encoding fixture; parsing, startup and known-answer construction are excluded. See the [boundary clarification](BOUNDARY.md).

![Paired complete cold ratios](figures/paired-cold-cost.svg)

| p / seed | Phi cold median (s) | Doubling cold median (s) | Paired ratio median | Five-round range | Original A/A range |
| --- | --- | --- | --- | --- | --- |
"""
    for r in pairs:
        p, seed = r["p"], r["seed"]
        a = cold[p, seed, "phi-l0-active"]
        b = cold[p, seed, "double-l0-active"]
        aa = cold[p, seed, "original"]["aa_repeat_over_first"]
        v = r["reference_over_candidate_cold"]
        md += f"| {p} / {seed} | {a['median_cold_ns']/1e9:.6f} | {b['median_cold_ns']/1e9:.6f} | {v['median']:.6f} | {v['min']:.6f}–{v['max']:.6f} | {aa['min']:.6f}–{aa['max']:.6f} |\n"
    md += """
The same declared 8 GiB address-space cap, 1,200 s watchdog, one Rayon thread and reserved whole SMT core apply to every measured arm on c7i.2xlarge / Intel Xeon Platinum 8488C hosts. Every admitted interval has exit 0, uncontended isolation and no user thread left on its reserved core. Complete successful intervals include construction/lifts, images and coefficients, symbolic setup, all failed peels, F4, roots/group checks, matrix work, recovery and independent point verification. Setup and five exclusive online phases are stored separately and their exact sums checked. Memory fields include process peak RSS in KiB and separately measured Rust allocation telemetry. No process RSS is inferred from allocation counts.

![Every arm's cold cost](figures/all-cold-costs.svg)

All ten arms and every per-target cost remain in [cold-costs.csv](results/cold-costs.csv), [paired-cold-costs.csv](results/paired-cold-costs.csv) and [summary.json](results/summary.json). An asterisk denotes consumed censored work, never a successful cheap solution. Complete rounds are selected chronologically under the [frozen resume rule](SUPPLEMENT_COLD_RESUME.md). Original failed, interrupted and contended records remain in the archive. A copied checkpoint is not a new timing sample.

## Rank trajectories

![Rank progress, part 1](figures/rank-progress-1.svg)

![Rank progress, part 2](figures/rank-progress-2.svg)

The rank axis uses the actual matrix dimension for each arm. Original uses L0 and L2 columns; pure and active-image controls use the justified L0 columns. Query counts alone cannot compare their work, because peel counts and solve costs differ. [full-stream.csv](results/full-stream.csv) records exact columns, first full-rank indices and finite systems. The full-rank query field is zero based; these figures show the count of queries consumed.

## External complete-GB control

Official msolve 0.11.0 commit `cc044551f8e202d00691990fd3fd0b1226a88ea2`, one thread, seed 0, grevlex, flags `-t 1 -v 2 -g 2 -r 1`, is compared on exactly 378 frozen positions (21 records × three arms × three fixed peel positions × two signs). Five positions have nonunit ideals; 373 do not. Inputs were selected before solving. Warmup, first, A/A and five timed passes yield 3,024 matching native/msolve reduced bases, each independently checked by input and S-pair reduction. The timer patch only increases printed precision. Build, options, library/executable hashes and raw outputs are preserved.

![Native versus msolve](figures/external-gb.svg)

The comparison uses internal complete-basis timers on the same isolated c7i.xlarge host, excluding startup/parsing/printing. Each group contains the same 18 fixed systems in each round. Native/msolve above 1 means native is slower. This does not price an msolve-based complete DLP solver. Magma, FGb, Singular and OpenF4 were not reproduced; fastest-external status is unknown. The observed msolve gap remains a [backlog item](../../../backlog/XP-20261023-quartic-twist-image-base/README.md).

| p / arm | Native internal ms | msolve internal ms | Native/msolve median | Five-round range | Native A/A | msolve A/A |
| --- | --- | --- | --- | --- | --- | --- |
"""
    for r in e["groups"]:
        v = r["native_over_msolve"]
        md += f"| {r['p']} / {r['arm']} | {r['native_internal_ms']['median']:.6f} | {r['msolve_internal_ms']['median']:.6f} | {v['median']:.6f} | {v['min']:.6f}–{v['max']:.6f} | {r['native_aa_over_first']:.6f} | {r['msolve_aa_over_first']:.6f} |\n"
    md += """
## Interpretation and outstanding questions

For a uniform residual, a fixed peel is a translation and leaves the residual distribution unchanged. Equal-size disjoint phi and doubling images therefore have the same exact first moment of signed physical tuples; their correlations and rank trajectories may differ. The historical stream is not uniform, so the measured tuple difference is conditional on this tape. Every seeded record also uses the same public scalar 271. Three seeds are trajectory variations, not independent secret samples, and timing repeats are repeated measurements of the same problem. No confidence interval for a population of curves or unknown scalars is inferred.

The actual distinguishable mathematical feature is phi's low-degree image equation. A joint S5/elimination solver could retain it and share work across image peels. That solver is not implemented or measured here. Orbit representatives alone exchange a peel loop for a transformed-residual loop; they do not remove those systems. No asymptotic obstruction to all joint algorithms is claimed.

The frozen cold supplement's assertion that no precomputed lift is available needs a precise correction: encoded public points are present for sign alignment and checks, while every required lift is recomputed and charged. The candidate does not read the tape's known scalar or reference hits/ranks, but this is source-audited rather than a process-isolated blinded input. The [boundary document](BOUNDARY.md) preserves that distinction and names every excluded cost.

The field/group and F4 row-reduction counters are partial. Complete instrumented elapsed time is available; a complete GAE price, the 0/0.1/1 price band and a protocol-matched strongest rho reference are missing. No `VALIDATION.json` is fabricated and no new headline enters the leaderboard or `benchmarks/README.md`. Benchmark PDFs and figures are copied as unlisted diagnostics. The precise remaining problem is [quartic-twist-polynomial-ic-pricing](../../../problems/quartic-twist-polynomial-ic-pricing.md).

## Reproduction, controls and publication

[ARTIFACTS.md](ARTIFACTS.md) gives the content-addressed raw archive, frozen source hashes, extraction and exact native analysis commands. [All launched-job reports](reports/INDEX.md) preserve build failures, timeouts, interruptions and invalid resource receipts. [MATHEMATICS.md](MATHEMATICS.md) separates proved coordinate algebra, checked group actions, elementary counting arguments and untested joint-solver hypotheses. [VALIDATION_STATUS.md](VALIDATION_STATUS.md) records the actual software gates, including failed and interrupted attempts. The archive also retains the rejected coefficient/record mutations, corrupted input hash control, original baseline parity and whole-round resume control.

Canonical graph review: `docs/ic/leaderboard.json`, `docs/ic-leaderboard.html`, `docs/ic/progress-timeline.json`, `docs/index-calculus-scoreboard.html` and the curve registry were checked for the new image study. No new curve transport or validated operation-price/rho result exists, so their existing values remain current. The new measured coverage is represented in the study-local figures and benchmark copies only. This research deliverable is part of [PR #2368](https://github.com/aburan28/crypto/pull/2368); software review status is distinct from mathematical validation.
"""
    (ROOT / "README.md").write_text(md)


def main():
    s, e = read(DATA / "summary.json"), read(DATA / "external-gb-summary.json")
    figures(s, e)
    main_report(s, e)
    run_reports(e)
    for name in ("README", "MATHEMATICS", "BOUNDARY", "VALIDATION_STATUS"):
        pdf(ROOT / f"{name}.md")
    benchmark = ROOT.parents[2] / "benchmarks/20261023-twist-image-base"
    benchmark.mkdir(parents=True, exist_ok=True)
    for p in FIG.iterdir():
        if p.suffix in (".svg", ".png"):
            shutil.copyfile(p, benchmark / p.name)
    for p in ROOT.glob("*.pdf"):
        shutil.copyfile(p, benchmark / p.name)
    for d in (ROOT / "reports").iterdir():
        if d.is_dir():
            target = benchmark / "reports" / d.name
            target.mkdir(parents=True, exist_ok=True)
            for p in d.iterdir():
                if p.suffix in (".pdf", ".svg", ".png"):
                    shutil.copyfile(p, target / p.name)
    (benchmark / "README.md").write_text("# Twist image-base diagnostics\n\nSource of truth: [study](../../research/twist_index_calculus_20261020/endomorphism_image_base/README.md).\n\nReports and all figures are copied here under the research reporting rule. These fixture-level complete elapsed intervals have no complete GAE band or matched strongest rho reference. The validation gate therefore prevents promotion to `benchmarks/README.md`; no headline row has been added. See the study's boundary and validation status.\n")


if __name__ == "__main__":
    main()
