#!/usr/bin/env python3
"""Render frozen mixed-twist evidence as SVG, PNG, Markdown, and PDF.

This is a presentation script. It reads the sealed summaries and comparisons;
it does not execute or reprice an ECDLP method.
"""

import datetime as dt
import json
import statistics
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

HERE = Path(__file__).resolve().parent
REPORTS = HERE / "reports"
CURVE_ORDER = (101, 211, 307, 509, 1009, 2003)
R_TO_P = {107: 101, 223: 211, 331: 307, 503: 509, 1013: 1009, 1933: 2003}


def render_pdf(folder, title, paragraphs, caption):
    body = [
        '#set page(width: 210mm, height: 297mm, margin: 17mm)',
        '#set text(font: "DejaVu Sans", size: 10pt)',
        f'= {title}',
    ]
    body.extend(paragraphs)
    body += ['#image("figure.svg", width: 100%)', caption]
    source = folder / "report.typ"
    source.write_text("\n\n".join(body) + "\n")
    typst.compile(str(source), output=str(folder / "report.pdf"))


def save_figure(fig, folder):
    svg = folder / "figure.svg"
    fig.savefig(svg, bbox_inches="tight")
    # Matplotlib wraps path coordinates with spaces before newlines. Keep the
    # SVG valid while satisfying the repository's whitespace gate.
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(folder / "figure.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def ecbench_reports():
    for b in (12, 24, 36):
        for suffix, tag in (("", "strict"), ("_pin", "pin"),
                            ("_pin_reg", "pin-reg")):
            pin = suffix != ""
            name = f"ecbench_b{b}{suffix}_l0"
            session_dir = HERE / "sessions" / name
            session = json.loads((session_dir / "session.json").read_text())
            host = json.loads((session_dir / "host.json").read_text())
            plan = json.loads((session_dir / "plan.json").read_text())
            comparison = json.loads((session_dir / "comparisons" /
                                     "rho-neg__ic-native.json").read_text())
            started = dt.datetime.fromtimestamp(
                session["started_unix_ms"] / 1000, dt.timezone.utc
            ).strftime("%Y%m%dT%H%MZ")
            run_dir = REPORTS / f"run-{started}-b{b}-{tag}"
            run_dir.mkdir(parents=True, exist_ok=True)
            cells = {R_TO_P[round(2 ** c["log2_r"])]: c
                     for c in comparison["curves"] if c["pairs"] and c["ci95"]}
            points = [p for p in CURVE_ORDER if p in cells]
            ratios = [cells[p]["ratio_b_over_a"] for p in points]
            low = [ratios[i] - cells[p]["ci95"][0]
                   for i, p in enumerate(points)]
            high = [cells[p]["ci95"][1] - ratios[i]
                    for i, p in enumerate(points)]
            fig, ax = plt.subplots(figsize=(8.2, 4.6))
            ax.errorbar(range(len(points)), ratios, yerr=[low, high],
                        fmt="o-", capsize=4, color="#315a89", linewidth=1.8)
            ax.axhline(1, color="#8b3d3d", linestyle="--", linewidth=1)
            ax.set_xticks(range(len(points)), [str(p) for p in points])
            ax.set_xlabel("Prime field p")
            ax.set_ylabel("Native IC / rho normalized S")
            ax.set_title(f"{name}: counted lower-bound ratio with cluster 95% intervals")
            ax.grid(axis="y", alpha=0.25)
            fig.subplots_adjust(bottom=0.17)
            save_figure(fig, run_dir)
            statuses = session["status_counts"]
            status_text = ", ".join(f"{v} {k}" for k, v in statuses.items())
            missing = "p=101 is absent from the comparable IC/rho graph because all 18 strict IC executions exhausted." if (b == 36 and not pin) else "All six curves have comparable verified IC/rho pairs."
            summary = (
                f"Session {session['session_id']} used spec {session['spec_id']} "
                f"(SHA-256 {session['spec_sha256']}) and binary SHA-256 "
                f"{session['binary_sha256']}. The host was "
                f"{host['stable']['cpu_model']}. Isolation earned L0 "
                f"(--cpus none --allow-busy); wall time is descriptive. "
                f"All curve registry flags were "
                f"{'true' if all(w['curve']['registered'] for w in plan['workloads']) else 'false'}."
            )
            measured = (
                f"The session requested {session['executions_planned']} runs: "
                f"{status_text}. Three public targets per curve, one warmup "
                f"and five measured rounds compared rho-neg, ic-native, "
                f"and an A/A rho arm. The saved comparison has "
                f"{comparison['ops']['pairs']} measured IC/rho pairs; "
                f"its counted S ratio is {comparison['ops']['ratio_b_over_a']:.3f}. "
                f"Both arms have unpriced native counters, so that ratio "
                f"is a lower-bound diagnostic. {missing}"
            )
            caption = ("Figure: per-curve counted S ratios and cluster 95% "
                       "intervals from the saved ecbench comparison. L0 wall "
                       "time is not admitted; no twist method ran in this session.")
            md = (f"# {name} run report\n\n{summary}\n\n{measured}\n\n"
                  f"![Per-curve S ratios](figure.svg)\n\n{caption}\n\n"
                  f"Sources: [sealed session](../../sessions/{name}/), "
                  f"[comparison](../../sessions/{name}/comparisons/rho-neg__ic-native.json), "
                  f"[audit](../../ecbench_b{b}{suffix}.audit.json).\n")
            (run_dir / "report.md").write_text(md)
            render_pdf(run_dir, f"{name} run report", [summary, measured], caption)


def exact_reports():
    summary = json.loads((HERE / "all_target_cost_summary.json").read_text())
    folder = REPORTS / "run-20261021T0529Z-all-target"
    folder.mkdir(parents=True, exist_ok=True)
    fig, axes = plt.subplots(1, 3, figsize=(11.5, 4), sharey=True)
    for ax, b in zip(axes, (12, 24, 36)):
        rows = [r for r in summary["rows"] if r["B"] == b]
        for arm, color in (("native-trace", "#315a89"),
                           ("mixed-rect", "#b15438")):
            y = [r["arms"][arm]["median_total_us"] for r in rows]
            lower = [y[i] - r["arms"][arm]["p05_total_us"]
                     for i, r in enumerate(rows)]
            upper = [r["arms"][arm]["p95_total_us"] - y[i]
                     for i, r in enumerate(rows)]
            ax.errorbar(range(6), y, yerr=[lower, upper], fmt="o-",
                        capsize=2.5, label=arm, color=color, linewidth=1.5)
        ax.set_xticks(range(6), [str(p) for p in CURVE_ORDER], rotation=45)
        ax.set_title(f"B={b}")
        ax.set_xlabel("Prime field p")
        ax.grid(axis="y", alpha=0.25)
    axes[0].set_ylabel("Complete cold time (µs), L0")
    axes[2].legend(loc="upper left", fontsize=8)
    fig.suptitle("Every nonzero target: median and 5th–95th percentiles")
    fig.subplots_adjust(bottom=0.22, top=0.86)
    save_figure(fig, folder)
    paragraph = (
        "The all-target run rebuilt each cold factor base and solved every "
        "nonzero public target for six curves and three base sizes. It emitted "
        "49,248 verified IC solves and 12,312 matched rho solves. The "
        "independent Python audit checked all 61,579 records, target points, "
        "recovered scalars, phase sums, and complete target coverage."
    )
    caption = ("Figure: recorded all-target complete-time medians and "
               "nearest-rank 5th–95th percentiles, L0. The raw SHA-256 is "
               f"{summary['raw_sha256']}. This run is descriptive; counted "
               "and isolated comparison is reported separately.")
    (folder / "report.md").write_text(
        "# Exhaustive all-target cost run\n\n" + paragraph + "\n\n"
        "![Complete cost distribution](figure.svg)\n\n" + caption + "\n\n"
        "Sources: [raw](../../all_target_cost.ndjson.gz), "
        "[summary](../../all_target_cost_summary.json), "
        "[auditor](../../summarize_all_target_cost.py).\n"
    )
    render_pdf(folder, "Exhaustive all-target cost run", [paragraph], caption)
    return folder


def frozen_protocol_reports():
    initial = [json.loads(line) for line in
               (HERE / "mixed_component_results.ndjson").read_text().splitlines()]
    strict = [json.loads(line) for line in
              (HERE / "mixed_full_rank_control.ndjson").read_text().splitlines()]
    folder = REPORTS / "run-20261021T0532Z-predeclared"
    folder.mkdir(parents=True, exist_ok=True)
    fig, ax = plt.subplots(figsize=(8.2, 4.6))
    for arm, color in (("native-B", "#6a7c37"),
                       ("native-trace", "#315a89"),
                       ("mixed-rect", "#b15438"),
                       ("mixed-unique", "#754d91")):
        points = [statistics.median(row["total_ns"] / 1000 for row in initial
                                    if row["kind"] == "solve" and
                                    row["p"] == p and row["base_size"] == 24 and
                                    row["arm"] == arm)
                  for p in CURVE_ORDER]
        ax.plot(range(6), points, "o-", label=arm, color=color)
    ax.set_xticks(range(6), [str(p) for p in CURVE_ORDER])
    ax.set_xlabel("Prime field p")
    ax.set_ylabel("Complete cold time, 3-target median (µs), L0")
    ax.set_title("Frozen three-target protocol at B=24")
    ax.grid(axis="y", alpha=0.25)
    ax.legend(fontsize=8)
    fig.subplots_adjust(bottom=0.17)
    save_figure(fig, folder)
    paragraph = (
        "The frozen three-target protocol emitted 72 exhaustive relation "
        "censuses, 216 correct IC solves, and 54 matched rho solves. "
        "All target scalars were independently checked. Mixed-unique used "
        "the explicitly amended target-row span rule; the original "
        "full-rank failures are preserved separately."
    )
    caption = ("Figure: recorded median complete cold time across three "
               "predeclared public targets per curve at B=24. L0 wall time "
               "is descriptive. The full raw file includes all three B values.")
    (folder / "report.md").write_text(
        "# Frozen three-target protocol run\n\n" + paragraph + "\n\n"
        "![Three-target cost](figure.svg)\n\n" + caption + "\n\n"
        "Sources: [raw](../../mixed_component_results.ndjson), "
        "[protocol](../../MIXED_PROTOCOL.md), "
        "[auditor](../../validate_mixed_results.py).\n"
    )
    render_pdf(folder, "Frozen three-target protocol", [paragraph], caption)

    folder = REPORTS / "run-20261021T0532Z-full-rank-control"
    folder.mkdir(parents=True, exist_ok=True)
    stalled = [row for row in strict if row["kind"] == "solve"
               and row["arm"] == "mixed-unique"]
    assert len(stalled) == 54 and all(row["recovered"] is None for row in stalled)
    assert {row["base_size"]: row["trace_rank"] for row in stalled} == {
        12: 10, 24: 22, 36: 34,
    }
    fig, ax = plt.subplots(figsize=(7.4, 4.2))
    ax.plot((12, 24, 36), (12, 24, 36), "o--", label="full-rank goal",
            color="#315a89")
    ax.plot((12, 24, 36), (10, 22, 34), "o-", label="achieved trace rank",
            color="#b15438")
    ax.set_xticks((12, 24, 36))
    ax.set_xlabel("Factor-base columns B")
    ax.set_ylabel("Trace-column rank")
    ax.set_title("All 54 mixed-unique strict-policy attempts stalled at B−2")
    ax.grid(axis="y", alpha=0.25)
    ax.legend()
    save_figure(fig, folder)
    paragraph = (
        "The final-source --full-rank-control run reproduced 54 of 54 "
        "mixed-unique failures: six curves × three base sizes × three "
        "targets. Achieved trace ranks were 10/12, 22/24, and 34/36. "
        "All 162 native-B, native-trace, and mixed-rect attempts solved. "
        "The historical first failure file is retained, but its exact "
        "intermediate source revision was not frozen."
    )
    caption = ("Figure: recorded strict-policy mixed-unique rank and "
               "full-rank goal. Every B point represents 18 failed "
               "target attempts. This is a rank result, not a wall comparison.")
    (folder / "report.md").write_text(
        "# Full-rank failure control\n\n" + paragraph + "\n\n"
        "![Trace-rank stall](figure.svg)\n\n" + caption + "\n\n"
        "Sources: [replayable raw](../../mixed_full_rank_control.ndjson), "
        "[historical raw](../../mixed_full_rank_attempt.ndjson), "
        "[protocol](../../MIXED_PROTOCOL.md).\n"
    )
    render_pdf(folder, "Full-rank failure control", [paragraph], caption)


def mimic_report():
    audit = json.loads((HERE / "native_mimic_audit.json").read_text())
    folder = REPORTS / "run-20261021T0545Z-native-mimic"
    folder.mkdir(parents=True, exist_ok=True)
    fig, axes = plt.subplots(1, 3, figsize=(11.5, 4), sharey=True)
    for ax, b in zip(axes, (12, 24, 36)):
        rows = [r for r in audit["rows"] if r["B"] == b]
        for key, label, color in (
            ("native_mimic_median_total_us", "rational mimic", "#315a89"),
            ("mixed_unique_median_total_us", "mixed-unique", "#b15438"),
        ):
            ax.plot(range(6), [r[key] for r in rows], "o-", label=label,
                    color=color, linewidth=1.5)
        ax.set_xticks(range(6), [str(p) for p in CURVE_ORDER], rotation=45)
        ax.set_title(f"B={b}")
        ax.set_xlabel("Prime field p")
        ax.grid(axis="y", alpha=0.25)
    axes[0].set_ylabel("Complete cold median (µs), L0")
    axes[2].legend(loc="upper left", fontsize=8)
    fig.suptitle("Same relation set and rational additions, with and without twist arithmetic")
    fig.subplots_adjust(bottom=0.22, top=0.86)
    save_figure(fig, folder)
    paragraph = (
        "The matched rational-only control tested every nonzero public "
        "target at six curves and three base sizes: 12,312 target pairs. "
        "The paired audit confirmed equal projected relation sets, setup "
        "trials, descent paths, recovered scalars, and rational group-law "
        "addition counts. The mimic performed zero twist additions; the "
        "mixed arm performed positive twist additions on every pair."
    )
    caption = ("Figure: recorded complete cold medians across every target "
               "per curve and B; L0 time is descriptive and includes "
               "direct-extension witness checks. Raw SHA-256: "
               f"{audit['raw_sha256']}.")
    (folder / "report.md").write_text(
        "# Matched rational-only mimic run\n\n" + paragraph + "\n\n"
        "![Mimic versus mixed cost](figure.svg)\n\n" + caption + "\n\n"
        "Sources: [raw](../../native_mimic_control.ndjson.gz), "
        "[audit](../../native_mimic_audit.json), "
        "[auditor](../../audit_native_mimic.py).\n"
    )
    render_pdf(folder, "Matched rational-only mimic", [paragraph], caption)
    return folder


def result_report(exact_folder, mimic_folder):
    summary = json.loads((HERE / "all_target_cost_summary.json").read_text())
    ratios = [row["arms"]["mixed-rect"]["median_total_us"] /
              row["arms"]["native-trace"]["median_total_us"]
              for row in summary["rows"]]
    fig_svg = exact_folder / "figure.svg"
    fig_png = exact_folder / "figure.png"
    result_svg = HERE / "MIXED_COST.svg"
    result_png = HERE / "MIXED_COST.png"
    result_svg.write_bytes(fig_svg.read_bytes())
    result_png.write_bytes(fig_png.read_bytes())
    (HERE / "MIMIC_COST.svg").write_bytes((mimic_folder / "figure.svg").read_bytes())
    (HERE / "MIMIC_COST.png").write_bytes((mimic_folder / "figure.png").read_bytes())
    folder = HERE
    paragraph = (
        "On six prime-order E/Fp curves, three base sizes, and every nonzero "
        "target, the exact mixed extension-field solver recovered all 49,248 "
        "IC target-arm scalars. The full relation census also covered every "
        "nonzero target for all four arms. Mixed-unique trace rank was B−2 "
        "in all 18 cells; mixed-rect trace rank was B/3. The native projected "
        "base admits every exact mixed relation by Frobenius projection."
    )
    paragraph2 = (
        "Complete cold L0 time is descriptive. Mixed-rect median time was "
        f"{min(ratios):.2f}–{max(ratios):.2f} times its matched native-trace "
        "median across 18 cells. "
        "Nine sealed ecbench native IC/rho sessions reproduced their counted "
        "work; one strict B=36 session retained 18 exhausted IC runs, and "
        "the later target-pin session recovered them. There is no isolated "
        "mixed-versus-rho speed claim: the mixed candidate has not been "
        "ported into ecbench."
    )
    paragraph3 = (
        "The matched rational-only mimic control reproduced the complete "
        "mixed-unique relation set and all 12,312 target descent paths with "
        "identical rational additions. It used zero twist additions; "
        "mixed-unique used positive twist additions in every paired run. "
        "Its L0 median time was lower in all 18 curve/base cells."
    )
    caption = ("Figure: complete cold mixed-rect and native-trace runtimes "
               "for every nonzero target. Error bars show nearest-rank "
               "5th–95th percentiles; L0 wall time is descriptive.")
    source = HERE / "MIXED_RESULT.typ"
    source.write_text("\n\n".join([
        '#set page(width: 210mm, height: 297mm, margin: 17mm)',
        '#set text(font: "DejaVu Sans", size: 10pt)',
        '= Mixed-component quadratic-twist index calculus',
        paragraph, paragraph2, paragraph3,
        '#image("MIXED_COST.svg", width: 100%)', caption,
        '#image("MIMIC_COST.svg", width: 100%)',
        'Figure: matched rational-only mimic and mixed-unique complete cold medians; L0 time is descriptive.',
        'Full records, methods, exact tables, audit receipts, and caveats are in MIXED_REPORT.md.',
    ]) + "\n")
    typst.compile(str(source), output=str(HERE / "MIXED_RESULT.pdf"))


if __name__ == "__main__":
    REPORTS.mkdir(exist_ok=True)
    ecbench_reports()
    frozen_protocol_reports()
    result_report(exact_reports(), mimic_report())
