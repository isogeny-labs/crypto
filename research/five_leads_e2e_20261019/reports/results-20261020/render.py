#!/usr/bin/env python3
"""Lay out frozen native results; no arithmetic or verification is done here."""
import json
import shutil
from datetime import datetime, timezone
from pathlib import Path

import matplotlib.pyplot as plt
import typst

STUDY = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
PROJECTIVE = json.loads((STUDY / "results/genus2-projective-summary.json").read_text())
MIXED = json.loads((STUDY / "results/mixed-fiber-dlp-native-v3-summary.json").read_text())
MIXED_POLY = json.loads((STUDY / "results/mixed-fiber-polynomial-dlp-summary.json").read_text())
MIXED_F4 = json.loads((STUDY / "results/mixed-fiber-f4-dlp-lower-bound-summary.json").read_text())
TIAN = json.loads((STUDY / "results/tian-km-arithmetic-gate-summary.json").read_text())
LINEAR = json.loads((STUDY / "results/tian-km-linear-dlp-summary.json").read_text())
SIEVE = json.loads((STUDY / "results/tian-quartic-line-sieve-summary.json").read_text())
MAPPED = json.loads((STUDY / "results/tian-quartic-line-sieve-mapped-summary.json").read_text())


def plot(path, labels, values, title, ylabel, session, horizontal=False):
    fig, ax = plt.subplots(figsize=(7.2, 3.6))
    colors = ["#16697a", "#a63a50", "#d48532", "#74598d", "#54894a"]
    if horizontal:
        ax.barh(labels, values, color=colors[:len(labels)])
        ax.axvline(1, color="black", linestyle="--", linewidth=1)
        ax.set_xscale("log")
        ax.set_xlabel(ylabel)
        for j, value in enumerate(values):
            ax.text(value * 1.04, j, f"{value:.3f}×", va="center", fontsize=9)
        ax.invert_yaxis()
        ax.set_xlim(left=0.4, right=max(values) * 1.7)
    else:
        ax.bar(labels, values, color=colors[:len(labels)])
        ax.axhline(1, color="black", linestyle="--", linewidth=1)
        ax.set_ylabel(ylabel)
        ax.set_ylim(0, max(values) * 1.22)
        for j, value in enumerate(values):
            ax.text(j, value + max(values) * .015, f"{value:.3f}×", ha="center", fontsize=9)
    ax.set_title(title, loc="left", fontsize=11)
    fig.text(.02, .01, session, fontsize=8, color="#444")
    fig.tight_layout(rect=(0, .035, 1, 1))
    fig.savefig(path.with_suffix(".svg"))
    fig.savefig(path.with_suffix(".png"), dpi=150)
    plt.close(fig)


def compile_report(directory, source):
    directory.mkdir(parents=True, exist_ok=True)
    (directory / "report.typ").write_text(source)
    typst.compile(str(directory / "report.typ"), output=str(directory / "report.pdf"))


def run_reports():
    for session in PROJECTIVE["sessions"]:
        bits = session["bits"]
        row = next(item for item in PROJECTIVE["summary"] if item["kind"] == "bits" and item["label"] == str(bits))
        receipt = json.loads((STUDY / f"sessions_projective_chart_v2/order-genus-{bits}/session.json").read_text())
        started = datetime.fromtimestamp(receipt["started_unix_ms"] / 1000, timezone.utc).strftime("%Y%m%dT%H%MZ")
        out = STUDY / f"reports/run-{started}-order-genus-{bits}"
        out.mkdir(parents=True, exist_ok=True)
        values = [row["representative"]["projective_over_affine"], row["representative"]["projective_over_rho"]]
        plot(out / "cost-ratios", ["Affine collector", "Source negation rho"], values,
             f"Projective chart · {bits}-bit source order · 180 targets", "candidate / reference cost", receipt["session_id"])
        source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Projective chart run: {bits}-bit source order

*Session:* `{receipt["session_id"]}`. *Spec:* `{receipt["spec_id"]}`.
*Binary SHA-256:* `{receipt["binary_sha256"]}`.
*Started:* {started} UTC. *Isolation:* L0 operation counts, `--cpus none --allow-busy`; wall time is unqualified.

The frozen session requested and completed {receipt["executions_planned"]} arm records on 180 exact-order targets: projective chart, unchanged affine two-choice collector, and source negation rho. All {receipt["records_written"]} records were source-verified; no failure or exhaustion was recorded. The independent `verify --replay-all` audit is in `replay_projective_chart_v2/order-genus-{bits}.json`.

At M+0.5S+10I and 0.1M per declared native logical operation, the candidate/reference ratios are {values[0]:.6f} against affine and {values[1]:.6f} against rho. Each ratio divides sums of complete cold one-target costs over the same 180 targets.

#figure(image("cost-ratios.svg", width: 100%), caption: [Recorded pooled operation-cost ratios; 180 paired targets; session {receipt["session_id"]}. Dashed line is equal cost.])

*Raw evidence:* `sessions_projective_chart_v2/order-genus-{bits}` (packed in `sessions_projective_chart_v2.tar.zst`); native analysis source `native/genus2_projective_analyze.rs`. Price-grid scope is M/S/I plus declared native logical operations; it does not establish wall speed or compare an external implementation.
'''
        compile_report(out, source)


def results_report():
    out = HERE
    out.mkdir(parents=True, exist_ok=True)
    sizes = [item for item in PROJECTIVE["summary"] if item["kind"] == "bits"]
    fig, ax = plt.subplots(figsize=(7.2, 3.7))
    x = list(range(len(sizes)))
    affine = [item["representative"]["projective_over_affine"] for item in sizes]
    rho = [item["representative"]["projective_over_rho"] for item in sizes]
    ax.bar([j-.18 for j in x], affine, width=.35, color="#16697a", label="Projective / affine")
    ax.bar([j+.18 for j in x], rho, width=.35, color="#a63a50", label="Projective / source rho")
    ax.axhline(1, color="black", linestyle="--", linewidth=1)
    ax.set_xticks(x, [item["label"] for item in sizes])
    ax.set_xlabel("Source subgroup order (bits); 180 paired targets per size")
    ax.set_ylabel("Complete one-target operation-cost ratio")
    ax.set_title("Genus-2 projective chart across the original 1,260 targets", loc="left", fontsize=11)
    ax.legend(frameon=False)
    fig.tight_layout()
    fig.savefig(out / "projective-cost-ratios.svg")
    fig.savefig(out / "projective-cost-ratios.png", dpi=150)
    plt.close(fig)
    mixed = [row for row in MIXED["summary"] if row["kind"] == "all"]
    plot(out / "mixed-fiber-cost-ratios", [row["arm"] for row in mixed],
         [row["representative"]["candidate_over_rho"] for row in mixed],
         "Mixed-fiber pair-table DLP · 180 targets per arm", "candidate / source rho cost",
         "mixed-fiber-dlp/v2 · 540 candidate records")
    polynomial = [row for row in MIXED_POLY["summary"] if row["kind"] == "all"]
    plot(out / "mixed-fiber-polynomial-cost-ratios", [row["arm"] for row in polynomial],
         [row["representative"]["candidate_over_rho"] for row in polynomial],
         "Mixed-fiber Semaev-S4 DLP · 180 targets per arm", "candidate / source rho cost",
         "540 source-verified polynomial all-roots records · F4 separately measured")
    f4 = [row for row in MIXED_F4["summary"] if row["kind"] == "all"]
    plot(out / "mixed-fiber-f4-lower-bound", [row["arm"] for row in f4],
         [row["representative"]["candidate_over_rho"] for row in f4],
         "Mixed-fiber native F4 DLP · 540 verified records", "candidate lower bound / source rho cost",
         "2,290 exact F4 query root sets · row-reduction field ops only", horizontal=True)
    gate = TIAN["representative"]
    plot(out / "tian-arithmetic-gate", ["One quartic double", "Known-scalar 1987S"],
         [gate["one_jacobian_double_over_mean_rho"], gate["known_scalar_1987_multiplication_over_mean_rho"]],
         "Tian F523 quartic arithmetic gate · no transfer DLP", "operation cost / mean source rho cost",
         "5 independently verified source rho seeds · published destination classes", horizontal=True)
    plot(out / "tian-linear-dlp", ["Published-class linear search"],
         [LINEAR["representative"]["cold_candidate_over_mean_rho"]],
         "Tian F523 destination DLP · published S,T only", "cold operation cost / mean source rho",
         "one source-verified fixed target · source map and index calculus unexecuted", horizontal=True)
    plot(out / "tian-line-sieve", ["Published-class line sieve"],
         [SIEVE["representative"]["cold_candidate_over_mean_rho"]],
         "Tian F523 destination index calculus · published S,T", "cold operation cost / mean source rho",
         "one source- and destination-verified target · source map unexecuted", horizontal=True)
    plot(out / "tian-mapped-line-sieve", ["Published-class", "Mapped source classes"],
         [SIEVE["representative"]["cold_candidate_over_mean_rho"], MAPPED["representative"]["cold_candidate_over_mean_rho"]],
         "Tian F523 cubic transfer · index calculus DLP", "cold operation cost / mean source rho",
         "101 source-map class checks · one mapped source- and destination-verified DLP", horizontal=True)
    pooled = next(row for row in PROJECTIVE["summary"] if row["kind"] == "all")
    p = pooled["representative"]
    source = f'''#set page(paper: "a4", margin: 19mm)
#set text(font: "Liberation Sans", size: 9.5pt)
= Five ECDLP follow-ups: verified operation evidence

*Evidence date:* 2026-10-20. *Pricing:* M+0.5S+10I; 0.1M per declared native logical operation except the cubic arithmetic gate, which has no native charge. All ratios below are operation counts, not qualified wall speed.

== Two complete genus-2 cohorts

The two-choice collector and the projective numerator chart each ran on all 1,260 original exact-order source targets (21 curves, 60 targets each, 16–40 bits). For the projective chart, all 3,780 arm records were source-verified and independently replayed. At the stated price, projective/affine is *{p["projective_over_affine"]:.6f}×* and projective/source-rho is *{p["projective_over_rho"]:.6f}×* pooled. The full 12×3 price grid and per-curve rows are in `results/genus2-projective-price-grid.csv`.

#figure(image("projective-cost-ratios.svg", width: 100%), caption: [Complete paired operation-cost ratios by order size; 180 targets at each size.])

== Mixed fibers: pair table and polynomial all-roots solvers

All 540 pair-table candidate records on nine eligible prime-order curves were source-verified; native reanalysis retained every price and curve. Homogeneous, mixed and matched-random pooled ratios to rho are {mixed[0]["representative"]["candidate_over_rho"]:.4f}×, {mixed[1]["representative"]["candidate_over_rho"]:.4f}× and {mixed[2]["representative"]["candidate_over_rho"]:.4f}×.

The Semaev-S4 exhaustive-polynomial solver also recovered all 540 scalars on the same 180 targets and checked every answer on the source. Homogeneous, mixed and matched-random pooled costs are {polynomial[0]["representative"]["candidate_over_rho"]:.4f}×, {polynomial[1]["representative"]["candidate_over_rho"]:.4f}× and {polynomial[2]["representative"]["candidate_over_rho"]:.4f}× rho. All 36 pooled price cells lose to rho and the corresponding pair-table arm.

The native F4 solver additionally returned exact rational root sets on 2,290 nonidentity DLP queries; 26 identity right-hand sides needed no polynomial system. Its replay recovered and source-verified all 540 scalars. Counting F4 row-reduction multiplications and collector costs gives homogeneous/mixed/random *{f4[0]["representative"]["candidate_over_rho"]:.0f}×/{f4[1]["representative"]["candidate_over_rho"]:.0f}×/{f4[2]["representative"]["candidate_over_rho"]:.0f}×* rho as lower bounds; all 36 pooled cells lose. Symbolic and substitution operations are uncounted, and external msolve remains unavailable.

#figure(image("mixed-fiber-cost-ratios.svg", width: 100%), caption: [Complete DLP cost ratios; 180 exact-order targets per arm.])

#figure(image("mixed-fiber-polynomial-cost-ratios.svg", width: 76%), caption: [Complete exhaustive Semaev-S4 all-roots DLP costs; 180 exact-order targets per arm.])

#figure(image("mixed-fiber-f4-lower-bound.svg", width: 76%), caption: [F4-backed DLP lower bounds from counted row-reduction field multiplications; 180 targets per arm.])

== Cubic transfer and genus-3 odd-kernel escape

For Tian's published F523 cubic fixture, the 3-kernel and smooth quartic were checked. The six published map functions were evaluated on 100 successive source multiples and Q; all 101 Frobenius-traced divisor classes match the expected Jacobian multiples. The line sieve recovered 1987 from the mapped P and Q classes and checked the answer on source and destination. Its complete cold cost, including formula evaluation and trace construction, is *{MAPPED["representative"]["cold_candidate_over_mean_rho"]:.2f}×* mean source rho at M+0.5S+10I+0.1A; all 36 price cells lose, spanning {MAPPED["cold_ratio_band"][0]:.2f}–{MAPPED["cold_ratio_band"][1]:.2f}×. The published-class-only line sieve costs {SIEVE["representative"]["cold_candidate_over_mean_rho"]:.2f}×. The 20 Smith-negative genus-3 holdouts were materialized, but odd kernels and end-to-end attacks on them remain untested. An independent quotient-from-kernel proof for the cubic fixture remains open.

#figure(image("tian-arithmetic-gate.svg", width: 76%), caption: [Arithmetic diagnostic only; one quartic doubling and a known-scalar multiplication divided by the mean of five complete source-rho solves.])

#figure(image("tian-linear-dlp.svg", width: 76%), caption: [Cold cost of one verified DLP on published destination classes; no source-to-Jacobian map or index calculus was run.])

#figure(image("tian-line-sieve.svg", width: 76%), caption: [Cold cost of published-class line-sieve index calculus, including all relation and descent attempts. Source map absent; operation pricing includes modular additions.])

#figure(image("tian-mapped-line-sieve.svg", width: 76%), caption: [Mapped source-to-answer line-sieve DLP against five optimized source-rho seeds. The published-class-only control omits source mapping.])

*Evidence:* `REPORT.md`, `results/`, `correctness/`, `sessions_projective_chart_v2.tar.zst`, and the seven linked `reports/run-*` PDFs. Negative outcomes and unexecuted routes remain explicit in the report. Literature: #link("https://arxiv.org/html/2012.07173")[Tian's cubic transfer] and #link("https://arxiv.org/html/2007.03172")[Tian's genus-3 odd-isogeny construction].
'''
    compile_report(out, source)


def unsealed_report():
    sessions = []
    for bits in (16, 20, 24, 28, 32, 36, 40):
        receipt = json.loads((STUDY / f"sessions_projective_chart/order-genus-{bits}/session.json").read_text())
        sessions.append((bits, receipt))
    out = STUDY / "reports/run-20261020T0604Z-unsealed-projective-v1"
    out.mkdir(parents=True, exist_ok=True)
    fig, ax = plt.subplots(figsize=(7.2, 3.3))
    ax.bar([str(bits) for bits, _ in sessions], [v["records_written"] for _, v in sessions], color="#8f8f8f")
    ax.set_ylim(0, 630)
    ax.set_ylabel("Source-verified arm records")
    ax.set_xlabel("Source subgroup order (bits)")
    ax.set_title("Developmental projective run · excluded from cost ranking", loc="left", fontsize=11)
    fig.tight_layout()
    fig.savefig(out / "records.svg")
    fig.savefig(out / "records.png", dpi=150)
    plt.close(fig)
    ids = ", ".join(v["session_id"] for _, v in sessions)
    source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Developmental projective run: retained, not ranked

*Started:* 2026-10-20T06:04Z UTC. *Sessions:* {ids}.

The first projective-chart run completed seven ecbench sessions with 540 source-verified arm records at each of the 16–40-bit order sizes. All 3,780 records were independently replayed; the receipts are in `replay_projective_chart/` and the raw archive is `sessions_projective_chart_unsealed_v1.tar.zst`.

The adapter inserted two zero-valued projective counters and an addition-backend detail label into the unchanged affine arm. Arithmetic and recovered scalars were unaffected, but the old method's record was changed. This archive is therefore excluded from all cost rankings. The corrected run is in `sessions_projective_chart_v2.tar.zst`; its independent audit enforces exact baseline-counter equality.

#figure(image("records.svg", width: 100%), caption: [Recorded verified arm counts in the unsealed developmental sessions; no operation-cost conclusion is drawn from this archive.])

*Isolation:* busy, unpinned L0 operation run; wall times are unqualified. *Reference:* the corrected projective chart [results report](../results-20261020/report.pdf).
'''
    compile_report(out, source)


def native_run_reports():
    polynomial = [row for row in MIXED_POLY["summary"] if row["kind"] == "all"]
    f4 = [row for row in MIXED_F4["summary"] if row["kind"] == "all"]
    mixed_out = STUDY / "reports/run-20261020T0609Z-mixed-fiber-dlp-v2"
    mixed_out.mkdir(parents=True, exist_ok=True)
    for extension in ("svg", "png"):
        shutil.copyfile(HERE / f"mixed-fiber-cost-ratios.{extension}", mixed_out / f"cost-ratios.{extension}")
    mixed = [row for row in MIXED["summary"] if row["kind"] == "all"]
    mixed_source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber pair-table DLP run

*Receipt written:* 2026-10-20T06:09Z UTC. *Command:* `mixed-fiber-dlp` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; operation counts are L0, wall time unqualified.

The native solver ran homogeneous, mixed and matched-random factor bases on nine eligible prime-order curves over F97, F193 and F257. Each arm received 20 distinct exact-order targets per curve. All 540 candidate scalars and all matched rho controls were source-verified; there were no failures or exhaustions. Exact byte replay is bound by `correctness/mixed-fiber-dlp-v2-replay.sha256`.

At M+0.5S+10I and 0.1M per declared native logical operation, homogeneous/rho is {mixed[0]["representative"]["candidate_over_rho"]:.6f}×, mixed/rho is {mixed[1]["representative"]["candidate_over_rho"]:.6f}×, and random/rho is {mixed[2]["representative"]["candidate_over_rho"]:.6f}×. Each ratio divides summed complete one-target costs for 180 matched targets.

#figure(image("cost-ratios.svg", width: 100%), caption: [Recorded pooled pair-table DLP cost ratios; 180 targets per arm; dashed line is equal cost.])

*Raw source:* `correctness/mixed-fiber-dlp-v2.jsonl`; native auditor `native/mixed_fiber_dlp_analyze.rs`; full price grid `results/mixed-fiber-dlp-native-v3-price-grid.csv`. This run tests a pair-table oracle, not a heterogeneous polynomial/F4 solver.
'''
    compile_report(mixed_out, mixed_source)

    rho_rows = [json.loads(line) for line in (STUDY / "correctness/tian-cubic-rho-v2.jsonl").read_text().splitlines()]
    rho_out = STUDY / "reports/run-20261020T0644Z-tian-cubic-rho"
    rho_out.mkdir(parents=True, exist_ok=True)
    fig, ax = plt.subplots(figsize=(7.2, 3.4))
    ax.bar([str(row["seed"]) for row in rho_rows], [row["counts"]["m"] for row in rho_rows], color="#16697a")
    ax.set_title("Tian F523³ source rho · 5 verified seeds", loc="left", fontsize=11)
    ax.set_ylabel("Base-field multiplications (M)")
    ax.set_xlabel("Seed")
    fig.tight_layout()
    fig.savefig(rho_out / "rho-multiplications.svg")
    fig.savefig(rho_out / "rho-multiplications.png", dpi=150)
    plt.close(fig)
    rho_source = '''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian F523³ source-rho control

*Receipt written:* 2026-10-20T06:44Z UTC. *Command:* `tian-cubic-rho` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

Five independent seeded negation-rho walks recovered the published scalar 1987 for Q=1987P on the prime-order elliptic curve over F523³. Every answer was checked by recomputing kP. The cubic-field inversion uses the norm formula and charges one base-field inversion; 1,024 sampled nonzero inverses were multiplied back to one before the runs. The five complete M/S/I vectors, native walk counts and source points are in `correctness/tian-cubic-rho-v2.jsonl`.

#figure(image("rho-multiplications.svg", width: 100%), caption: [Recorded base-field multiplication counts for five verified source-rho solves; squarings and inversions are retained separately in the raw vectors.])

The destination group-law check is a separate run. These controls alone do not execute the cubic transfer attack; no wall-speed or external-system comparison is claimed.
'''
    compile_report(rho_out, rho_source)

    km_out = STUDY / "reports/run-20261020T0705Z-tian-quartic-km"
    km_out.mkdir(parents=True, exist_ok=True)
    for extension in ("svg", "png"):
        shutil.copyfile(HERE / f"tian-arithmetic-gate.{extension}", km_out / f"arithmetic-gate.{extension}")
    gate = TIAN["representative"]
    km_source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic Jacobian arithmetic gate

*Receipt written:* 2026-10-20T07:05Z UTC. *Command:* `tian-quartic-km` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The native evaluation-space Khuri–Makdisi implementation constructed Tian's published quartic over F523 and the published classes S,T. It checked T=1987S, rS=0, inverse, commutative and associative controls. The known scalar 1987 is an input to this certificate; the run is not a destination-side DLP solver. The raw receipt is `correctness/tian-quartic-km-v2.json`.

At M+0.5S+10I, one measured quartic doubling costs {gate["one_jacobian_double_GAE"]:.0f} base-field multiplication equivalents, {gate["one_jacobian_double_over_mean_rho"]:.3f}× the mean of five complete source-rho controls. Computing the deliberately known scalar 1987S costs {gate["known_scalar_1987_multiplication_over_mean_rho"]:.3f}× that mean. The complete price grid is `results/tian-km-arithmetic-gate-price-grid.csv`.

#figure(image("arithmetic-gate.svg", width: 100%), caption: [Recorded arithmetic gate ratios to the five-seed mean source-rho cost; no transfer DLP attack ratio is shown.])

The source-to-Jacobian map, quotient kernel verification and a destination DLP solver remain unexecuted. No wall-speed or external-system comparison is claimed.
'''
    compile_report(km_out, km_source)

    linear_out = STUDY / "reports/run-20261020T0721Z-tian-km-linear"
    linear_out.mkdir(parents=True, exist_ok=True)
    for extension in ("svg", "png"):
        shutil.copyfile(HERE / f"tian-linear-dlp.{extension}", linear_out / f"cost-ratio.{extension}")
    linear = LINEAR["representative"]
    linear_source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic: bounded published-class DLP control

*Receipt written:* 2026-10-20T07:21Z UTC. *Command:* `tian-quartic-km --dlp-linear` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The solver received Tian's published quartic classes S,T and a cap of 10,000, without the published scalar. It tested consecutive multiples of S, recovered 1987 at the 1,987th candidate, and recomputed 1987P=Q on the source curve. This is one unusually small published scalar. The raw source-verified receipt is `correctness/tian-km-linear-dlp-v1.json`; the native 12-price analysis is `results/tian-km-linear-dlp-price-grid.csv`.

At M+0.5S+10I the complete cold candidate costs {linear["cold_candidate_GAE"]:.0f} base-field multiplication equivalents, or {linear["cold_candidate_over_mean_rho"]:.2f}× the mean of five complete optimized source-rho solves. The ratio spans {LINEAR["cold_ratio_band"][0]:.2f}–{LINEAR["cold_ratio_band"][1]:.2f}× over the 12 arithmetic prices. Native logical work is unpriced in this comparison and wall time is unqualified.

#figure(image("cost-ratio.svg", width: 100%), caption: [Recorded cold cost of the one published-class destination DLP relative to mean source rho; dashed line is equal cost.])

The source-to-Jacobian map was not executed; the published S,T are direct inputs. The proposed genus-3 index-calculus solver was not run. This control does not establish a general cubic-transfer attack cost or an external-system speed comparison.
'''
    compile_report(linear_out, linear_source)

    sieve_out = STUDY / "reports/run-20261020T0745Z-tian-line-sieve"
    sieve_out.mkdir(parents=True, exist_ok=True)
    for extension in ("svg", "png"):
        shutil.copyfile(HERE / f"tian-line-sieve.{extension}", sieve_out / f"cost-ratio.{extension}")
    sieve = SIEVE["representative"]
    sieve_source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic: published-class line-sieve DLP

*Command:* `tian-quartic-km --line-sieve-dlp` and `tian-line-sieve-analyze` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The solver received only the published destination classes S,T, without 1987. It enumerated all 522 rational affine points of the quartic and 11,116 fully split lines, obtaining relation rank 521 modulo the prime subgroup order. A rational degree-three divisor split for S and another for 6T after five failed target attempts. The divisor relations were independently checked by constructing the rational divisor classes and comparing them in the Jacobian. The resulting scalar 1987 was verified on the destination and by recomputing 1987P=Q on the source.

At M+0.5S+10I+0.1A, the full cold cost is {sieve["cold_candidate_GAE"]:.1f} multiplication equivalents versus {sieve["mean_source_rho_GAE"]:.2f} for the mean of five optimized source-rho solves, a {sieve["cold_candidate_over_mean_rho"]:.2f}× loss. Across 36 prices this solver loses in every cell ({SIEVE["cold_ratio_band"][0]:.2f}–{SIEVE["cold_ratio_band"][1]:.2f}×). Quartic enumeration, line relations, modular elimination, failed descents and both verifications are included. Allocation, comparisons and control flow are unpriced, so wall speed is unknown.

#figure(image("cost-ratio.svg", width: 100%), caption: [Complete counted cold operation cost for one published-class destination DLP versus mean source rho.])

*Raw receipt:* `correctness/tian-quartic-line-sieve-dlp-v1.json`. *Full 36-price grid:* `results/tian-quartic-line-sieve-price-grid.csv`. The source-to-Jacobian map was not executed: S,T were direct published inputs, so this is not a complete transferred source attack or a general-size result.
'''
    compile_report(sieve_out, sieve_source)

    mapped_out = STUDY / "reports/run-20261020T0843Z-tian-mapped-line-sieve"
    mapped_out.mkdir(parents=True, exist_ok=True)
    for extension in ("svg", "png"):
        shutil.copyfile(HERE / f"tian-mapped-line-sieve.{extension}", mapped_out / f"cost-ratio.{extension}")
    mapped = MAPPED["representative"]
    mapped_source = f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian cubic: mapped source-to-answer line-sieve DLP

*Commands:* `tian-cubic-map-gate`, `tian-quartic-km --trace-map-pq`, `tian-quartic-km --line-sieve-mapped-dlp`, `tian-line-sieve-mapped-analyze`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The published source P and Q coordinates were mapped with Tian's six rational functions; Q was supplied directly, not generated from the planted scalar. Their degree-three cuts were checked on the smooth quartic and their Frobenius traces represented as Jacobian classes. A separate cohort gate matched 100 successive source multiples and Q to destination classes. The line-sieve DLP consumed the mapped P and Q classes, recovered 1987 without it as a solver input, and verified the answer on source and destination.

At M+0.5S+10I+0.1A, the complete cold pipeline costs {mapped["cold_candidate_GAE"]:.1f} multiplication equivalents versus {mapped["mean_source_rho_GAE"]:.2f} for mean source rho, a {mapped["cold_candidate_over_mean_rho"]:.2f}× loss. All 36 price cells lose ({MAPPED["cold_ratio_band"][0]:.2f}–{MAPPED["cold_ratio_band"][1]:.2f}×). The {SIEVE["representative"]["cold_candidate_over_mean_rho"]:.2f}× published-class-only control omits source mapping.

#figure(image("cost-ratio.svg", width: 100%), caption: [Mapped cubic source-to-answer index calculus versus optimized source rho; the published-class-only result is shown for comparison.])

*Evidence:* `correctness/tian-cubic-map-gate-v2.jsonl`, `correctness/tian-cubic-trace-map-pq-v1.json`, `correctness/tian-quartic-line-sieve-mapped-dlp-v1.json`, and `results/tian-quartic-line-sieve-mapped-price-grid.csv`. Memory and control flow are unpriced. This is one published fixture; an independent quotient-from-kernel proof and growing-size comparison remain open.
'''
    compile_report(mapped_out, mapped_source)

    polynomial_out=STUDY / "reports/run-20261020T0750Z-mixed-fiber-polynomial"
    polynomial_out.mkdir(parents=True,exist_ok=True)
    for extension in ("svg","png"):
        shutil.copyfile(HERE / f"mixed-fiber-polynomial-cost-ratios.{extension}",polynomial_out / f"cost-ratios.{extension}")
    poly_source=f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber Semaev-S4 all-roots DLP

*Command:* `mixed-fiber-polynomial-dlp` and `mixed-fiber-polynomial-analyze` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The native solver built the Semaev-S4 resultant for every tuple in each prescribed three-fiber system, enumerated all rational polynomial zeros, tested every sign lift, inserted only group-law-valid relations, solved the modular rank system, and recomputed kP for each recovered scalar. It completed all 540 candidate arm records on 180 exact-order targets across nine eligible prime-field curves. The same target/fiber/rho records match the earlier pair-table cohort exactly. It performed 3,783,011 polynomial evaluations, found 21,849 polynomial roots, and attempted 165,669 sign lifts, retaining failed lifts.

At M+0.5S+10I and 0.1M per declared native logical operation, homogeneous/mixed/matched-random cost {polynomial[0]["representative"]["candidate_over_rho"]:.4f}×/{polynomial[1]["representative"]["candidate_over_rho"]:.4f}×/{polynomial[2]["representative"]["candidate_over_rho"]:.4f}× source rho. Against the corresponding pair-table arms the ratios are {polynomial[0]["representative"]["candidate_over_pair_table"]:.4f}×/{polynomial[1]["representative"]["candidate_over_pair_table"]:.4f}×/{polynomial[2]["representative"]["candidate_over_pair_table"]:.4f}×. All 36 pooled price cells lose to both references for all three arms.

#figure(image("cost-ratios.svg", width: 100%), caption: [Complete cold operation-cost ratios versus matched source rho; 180 targets per arm.])

*Raw records:* `correctness/mixed-fiber-polynomial-dlp-v1.jsonl`; *full 36-price per-curve grid:* `results/mixed-fiber-polynomial-dlp-price-grid.csv`. This exhaustive all-roots polynomial evaluator is not F4/msolve. The fastest external Gröbner comparison and qualified wall-speed comparison remain open.
'''
    compile_report(polynomial_out,poly_source)

    f4_out=STUDY / "reports/run-20261020T0814Z-mixed-fiber-f4"
    f4_out.mkdir(parents=True,exist_ok=True)
    for extension in ("svg","png"):
        shutil.copyfile(HERE / f"mixed-fiber-f4-lower-bound.{extension}",f4_out / f"cost-ratios.{extension}")
    f4_source=f'''#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Mixed-fiber F4-backed DLP: counted lower bounds

*Commands:* `five_leads_mixed_f4 --dlp-cohort`, `mixed-fiber-polynomial-dlp --f4-replay`, and `mixed-fiber-f4-analyze`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts and unqualified internal F4 wall times.

The native F4 solver built the Semaev-S4 resultant plus three fiber equations and returned complete rational roots for 2,290 nonidentity relation queries across the 540 original one-target arms. Another 26 identity right-hand sides needed no polynomial system. Every F4 root set matched exhaustive polynomial enumeration; there were no timeouts or mismatches. The collector consumed these roots, sign-lifted them, recovered all 540 scalars and source-verified every answer. The analyzer matched each target, query list, relation count and answer to the pair-table and exhaustive-polynomial controls.

F4 reported 467,482,105,353 prime-field row-reduction multiplications. At M+0.5S+10I and 0.1M per declared native logical operation, the resulting *lower bounds* against matched rho are {f4[0]["representative"]["candidate_over_rho"]:.0f}× homogeneous, {f4[1]["representative"]["candidate_over_rho"]:.0f}× mixed and {f4[2]["representative"]["candidate_over_rho"]:.0f}× matched random. All 36 pooled pricing cells lose to rho and the pair-table control. Mixed/homogeneous is {MIXED_F4["cross_arm"][0]["representative"]:.5f}× in the counted lower bound, with 36/36 losses.

#figure(image("cost-ratios.svg", width: 100%), caption: [F4-backed DLP counted lower bounds versus matched source rho; 180 targets per arm.])

*Raw F4 roots:* `correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl`; *verified DLPs:* `correctness/mixed-fiber-f4-dlp-v1.jsonl`; *full grid:* `results/mixed-fiber-f4-dlp-lower-bound-price-grid.csv`. Symbolic preprocessing, substitution field operations, allocation and control flow are uncounted. The summed F4 internal wall time was 356.1 s on a busy host, not a qualified rho wall-time ratio. No external msolve comparison ran.
'''
    compile_report(f4_out,f4_source)


if __name__ == "__main__":
    run_reports()
    results_report()
    unsealed_report()
    native_run_reports()
