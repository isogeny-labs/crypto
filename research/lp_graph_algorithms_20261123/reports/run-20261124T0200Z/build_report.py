#!/usr/bin/env python3
"""Lay out retained native M31 records. No crypto, rank, or timing computation.

Run only after the coordinator registers the unsigned evidence claim:
  python3 build_report.py --evidence-dir PATH --metadata report_metadata.json
The metadata file must name the claim and committed receipt. All plotted and
tabulated mathematical values are copied from the original native JSON fields.
"""
import argparse
import json
import re
from pathlib import Path
from xml.sax.saxutils import escape

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.enums import TA_LEFT, TA_RIGHT
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import inch
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image, PageBreak,
)

HERE = Path(__file__).resolve().parent
PREFIXES = (256, 512, 613)
ARMS = (("forest", "Exact forest", "forest_filter"),
        ("pivot", "Pivot", "pivot_filter"),
        ("mst", "MST", "mst_filter"))
PALETTE = {"forest": "#24628A", "pivot": "#8460A9", "mst": "#B67521"}
SOURCE_COMMIT = "3ba271b1f7100bdfd06eafbac8af283ec9d55094"
PRODUCER_COMMIT = "577d6c2c1c4a527604d965e6fb62c44df7902f36"
STUDY = "research/lp_graph_algorithms_20261123"
REPO_URL = "https://github.com/isogeny-labs/crypto"
RUN_ID = 45309117216
JOB_ID = 138160138786


def read_json(path):
    return json.loads(path.read_text())


def text_number(value):
    return "n/a" if value is None else f"{value:,}"


def collect_records(evidence, meta):
    """Select recorded fields; do not derive mathematical quantities."""
    if not re.fullmatch(r"PCC-\d{8}-[0-9a-f]{6}", meta["claim_id"]):
        raise ValueError("A registered claim ID is required before rendering")
    if not re.fullmatch(r"[0-9a-f]{40}", meta["receipt_commit"]):
        raise ValueError("An exact published receipt commit is required")
    audit = read_json(evidence / "derived/conversion-audit.json")
    if audit["schema"] != "prime31-b677-capture-audit/v1":
        raise ValueError("Unexpected native audit schema")
    reports, rows = [], []
    for prefix in PREFIXES:
        report = read_json(evidence / f"graph-{prefix}.json")
        if report["schema"] != "lp-graph-eval/v1" or report["rows"] != prefix:
            raise ValueError("Wrong source-order prefix or evaluator schema")
        reports.append({"prefix": prefix, "file": f"graph-{prefix}.json"})
        for arm_id, arm_label, native_key in ARMS:
            arm = report[native_key]
            exact = arm.get("exact_projected_rank") or {}
            rows.append({
                "prefix": prefix, "arm": arm_id, "label": arm_label,
                "native_key": native_key, "refused": arm["refused"],
                "lp_rank": arm.get("lp_rank"),
                "lp_left_nullity": arm.get("lp_left_nullity"),
                "projected_rows": exact.get("input_rows"),
                "useful_rank": exact.get("useful_small_rank"),
                "augmented_rank": exact.get("augmented_rank"),
                "source_terms": arm.get("source_terms"),
                "residual_nonzeros": arm.get("residual_nonzeros"),
                "max_row_weight": arm.get("max_row_weight"),
                "replayed": arm.get("replayed"),
                "replay_failures": arm.get("replay_failures"),
            })
    controls = []
    for key, label in (("field", "Changed field metadata"),
                       ("query", "Changed query coefficient"),
                       ("sign", "Flipped source-row sign")):
        controls.append({
            "label": label, "key": key,
            "exit": int((evidence / f"reject-{key}.exit").read_text()),
            "stdout_bytes": (evidence / f"reject-{key}.stdout").stat().st_size,
            "stderr": (evidence / f"reject-{key}.stderr").read_text().strip(),
        })
    return {
        "schema": "m31-report-presentation-data/v1",
        "presentation_only": True,
        "scientific_values": "Copied from native JSON; no recomputation in this renderer",
        "standing": "unverified", "metadata": meta,
        "counts": audit["counts"], "query_joins": audit["query_joins"],
        "reports": reports, "rows": rows, "controls": controls,
        "custody_limits": audit["custody_limits"],
    }


def draw_plot(data):
    """Plot existing integer count fields. Coordinate arithmetic is layout only."""
    plt.rcParams.update({
        "font.family": "DejaVu Sans", "font.size": 9,
        "svg.fonttype": "none", "axes.spines.top": False,
        "axes.spines.right": False, "axes.labelcolor": "#243340",
        "text.color": "#243340", "axes.edgecolor": "#CDD6DD",
    })
    fig, axes = plt.subplots(1, 2, figsize=(8.2, 3.0))
    metrics = (("source_terms", "Source witness terms"),
               ("residual_nonzeros", "Residual coefficient nonzeros"))
    lookup = {(row["prefix"], row["arm"]): row for row in data["rows"]}
    for axis, (metric, title) in zip(axes, metrics):
        for index, (arm_id, arm_label, _) in enumerate(ARMS):
            x = [i + (index - 1) * 0.25 for i in range(len(PREFIXES))]
            records = [lookup[(prefix, arm_id)] for prefix in PREFIXES]
            values = [record[metric] if not record["refused"] else float("nan")
                      for record in records]
            bars = axis.bar(x, values, width=0.22, color=PALETTE[arm_id],
                            label=arm_label, zorder=3)
            labels = ["" if record["refused"] else text_number(record[metric])
                      for record in records]
            axis.bar_label(bars, labels=labels, fontsize=6.5, padding=3, rotation=90)
            for position, record in zip(x, records):
                if record["refused"]:
                    axis.annotate("refused", (position, 0), xytext=(0, 4),
                                  textcoords="offset points", ha="center", fontsize=7,
                                  rotation=90, color=PALETTE[arm_id])
        axis.set_title(title, fontsize=10, fontweight="semibold", loc="left", pad=11)
        axis.set_xticks(range(len(PREFIXES)), [str(prefix) for prefix in PREFIXES])
        axis.set_xlabel("Admitted rows in source-order prefix", fontsize=8)
        axis.set_ylabel("Count", fontsize=8)
        axis.grid(axis="y", color="#E8EDF1", linewidth=0.6, zorder=0)
        axis.tick_params(axis="both", labelsize=8, length=0)
        axis.margins(y=0.30)
    handles, labels = axes[0].get_legend_handles_labels()
    fig.legend(handles, labels, loc="upper center", ncol=3, frameon=False,
               bbox_to_anchor=(0.5, 1.04), fontsize=8)
    fig.tight_layout(pad=1.1, rect=(0, 0, 1, 0.97))
    fig.text(0.02, -0.035,
             f"M31 | native retained-input counts | run {RUN_ID} | "
             f"{data['metadata']['claim_id']} | [pcc:unverified]",
             fontsize=6.5, color="#4C5E6D")
    fig.savefig(HERE / "native_payload_counts.svg", bbox_inches="tight")
    fig.savefig(HERE / "native_payload_counts.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def source_links(meta):
    return [
        ("1", "Native run and job", f"{REPO_URL}/actions/runs/{RUN_ID}/job/{JOB_ID}"),
        ("2", "Committed native receipt", f"{REPO_URL}/blob/{meta['receipt_commit']}/{meta['receipt_path']}"),
        ("3", "Frozen replay protocol", f"{REPO_URL}/blob/{SOURCE_COMMIT}/{STUDY}/M31_CAPTURE_REPLAY_20261124.md"),
        ("4", "Immutable producer capture", f"{REPO_URL}/blob/{PRODUCER_COMMIT}/research/prime31_graph_ecdlp_20261123/evidence/mitm-b677-control-01/ten-query.json"),
        ("5", "Original graph evaluator", f"{REPO_URL}/blob/{SOURCE_COMMIT}/examples/lp_graph_eval.rs"),
    ]


def markdown_report(data):
    meta, counts = data["metadata"], data["counts"]
    lp_summary = "; ".join(f"{row['prefix']} rows: {row['lp_rank']}" for row in data["rows"] if row["arm"] == "forest")
    rank_lines = ["| Prefix rows | Arm | Projected rows | Useful rank | Augmented rank | Replay failures |",
                  "|---:|---|---:|---:|---:|---:|"]
    payload_lines = ["| Prefix rows | Arm | Source terms | Residual nonzeros | Max row weight | Replayed outputs |",
                     "|---:|---|---:|---:|---:|---:|"]
    for row in data["rows"]:
        label = row["label"] + (" (refused)" if row["refused"] else "")
        rank_lines.append("| " + " | ".join([str(row["prefix"]), label] +
            [text_number(row[key]) for key in
             ("projected_rows", "useful_rank", "augmented_rank", "replay_failures")]) + " |")
        payload_lines.append("| " + " | ".join([str(row["prefix"]), label] +
            [text_number(row[key]) for key in
             ("source_terms", "residual_nonzeros", "max_row_weight", "replayed")]) + " |")
    controls = ["| Corruption of the same M31 input | Exit | Capture stdout bytes |",
                "|---|---:|---:|"]
    controls.extend(f"| {row['label']} | {row['exit']} | {row['stdout_bytes']} |"
                    for row in data["controls"])
    artifact = meta["artifact"]
    links = "\n\n".join(f"[{number}] [{label}]({url})" for number, label, url in source_links(meta))
    return f"""# M31 retained capture: native rank and payload

Run {RUN_ID} | Job {JOB_ID} | 24 November 2026 UTC | PR [#5076]({REPO_URL}/pull/5076)

**Standing: unsigned claim `{meta['claim_id']}`; all native observations remain [pcc:unverified].**

## Recorded outcome

The native converter replayed {counts['replayed_rows']} signed source equations on the fixed M31 curve. It retained {counts['kept_rows']} rows with at most two LP terms and recorded all {counts['excluded_rows']} higher-arity exclusions after group replay. The original group-admission bridge and graph evaluator then consumed the same retained source-order prefixes. These are observations from one retained ten-query capture, not repeated samples. [1-4] [pcc:unverified]

{meta.get('result_note', '')}

## Exact instance and scope

The field is **p = 2^31 - 1 = 2147483647**. The curve is y^2 = x^3 + 847514846x + 2087444724 over that field; its relation/scalar modulus is ell = 2147446831. G = (625037715,2100627251), Q = (976979610,1228979761). The target's original orientation is preserved. The source uses Dickson B509 and 168 Y7 extras, with original hybrid point indices retained. [3-4]

This is analysis of a saved generic exhaustive-pair baseline on public queries and a known-answer public target. The producer owns the capture; no collector was invoked by this replay. The Rust converter uses the original arithmetic and fixture construction. Producer-reported rank and recovered-scalar fields are ignored. [3-4]

## Native rank and replay observations

"Useful rank" is the native evaluator's `exact_projected_rank.useful_small_rank` on retained small columns, including its protected target column. Augmented rank also includes the right-hand side. Each prefix is the first recorded admitted rows, with original row identities preserved. Values below are native outputs, not the producer's bespoke rank report. [2,5] [pcc:unverified]

Native LP coefficient ranks recorded by the exact forest: **{lp_summary}**. [2] [pcc:unverified]

{chr(10).join(rank_lines)}

## Payload observations

![Native payload counts](native_payload_counts.svg)

Figure 1. Native source-witness term counts and residual coefficient nonzero counts, by arm and source-order prefix; run {RUN_ID}. One retained capture, three prefixes, exact native integer counts. No sampling interval or repeated-run uncertainty was recorded. No timing field is plotted or interpreted. [2] [pcc:unverified]

{chr(10).join(payload_lines)}

## Same-input rejection controls

{chr(10).join(controls)}

The changed field, query coefficient and signed term are corruptions of this same M31 input. They are refusal checks, not extra small-field workloads. The raw command, stderr and exit files are retained. [2] [pcc:unverified]

## Execution and custody

- Exact tested checkout: `{SOURCE_COMMIT}`.
- Native run/job: `{RUN_ID}` / `{JOB_ID}`; GitHub-hosted `ubuntu-latest` development CI.
- The retained-capture stage has a 300-second outer deadline, 4 GiB virtual-address cap and one build/test thread. Each converter/admission/evaluator call is bounded separately by the frozen script. This was not an isolated timing qualification. [3]
- Producer input: commit `{PRODUCER_COMMIT}`; blob `7fd27b9888058399d8814cef373d373243e6a1e5`; 165511 bytes; SHA-256 `e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab`. [4]
- Artifact `{artifact['id']}`: {artifact['bytes']} bytes, SHA-256 `{artifact['sha256']}`; expires `{artifact['expires_at']}`. Exact archive/file identities and compact native outputs are in the committed receipt. [2]

The original collector does not provide a complete per-probe rejection trace or its original binary/toolchain attestation. Its reported arithmetic counter omits separate fixture setup and bespoke graph arithmetic. Consumer replay does not fill those provenance or complete cold-cost gaps. These graph counts establish neither a timing comparison nor end-to-end ECDLP advantage. The next technical gate is a collector-custodied stream with complete collection/recovery accounting and a matched reference. [3-4]

Canonical performance scoreboards and progress plots receive no new timing or speedup point from this development replay. The earlier planted-control report remains a separate historical record; it is not overwritten. This report contains source-linked unsigned native observations only. [pcc:unverified]

## Sources

{links}

## Reproduce the presentation

`build_report.py` reads the retained native JSON and exit records, selects the exact recorded fields into `report_data.json`, and writes this report, the SVG/PNG and PDF. Python performs document layout only; no crypto, rank, scalar recovery or timing computation occurs. The committed presentation data also supports re-rendering after the temporary Actions archive expires:

```sh
python3 build_report.py --presentation-data report_data.json
```

To repeat the original selection from the extracted native archive:

```sh
python3 build_report.py --evidence-dir /path/to/m31-retained-capture --metadata report_metadata.json
```
"""


def pdf_report(data):
    meta, counts = data["metadata"], data["counts"]
    lp_summary = "; ".join(f"{row['prefix']} rows: {row['lp_rank']}" for row in data["rows"] if row["arm"] == "forest")
    font_root = next(root for root in (Path("/usr/share/fonts/truetype/liberation2"),
                                      Path("/usr/share/fonts/truetype/liberation"))
                     if (root / "LiberationSans-Regular.ttf").is_file())
    for name, file in (("ReportSans", "LiberationSans-Regular.ttf"),
                       ("ReportBold", "LiberationSans-Bold.ttf"),
                       ("ReportMono", "LiberationMono-Regular.ttf")):
        pdfmetrics.registerFont(TTFont(name, str(font_root / file)))
    pdfmetrics.registerFontFamily("ReportSans", normal="ReportSans", bold="ReportBold",
                                 italic="ReportSans", boldItalic="ReportBold")
    styles = getSampleStyleSheet()
    styles.add(ParagraphStyle(name="ReportTitle", fontName="ReportBold", fontSize=22,
                              leading=26, textColor=colors.HexColor("#173E57"), spaceAfter=8))
    styles.add(ParagraphStyle(name="ReportH2", fontName="ReportBold", fontSize=11.5,
                              leading=15, textColor=colors.HexColor("#173E57"), spaceBefore=11, spaceAfter=6))
    styles.add(ParagraphStyle(name="ReportBody", fontName="ReportSans", fontSize=9,
                              leading=12.2, spaceAfter=7, textColor=colors.HexColor("#263744")))
    styles.add(ParagraphStyle(name="ReportSmall", fontName="ReportSans", fontSize=7.6,
                              leading=10, spaceAfter=5, textColor=colors.HexColor("#4C5E6D")))
    styles.add(ParagraphStyle(name="ReportTable", fontName="ReportSans", fontSize=8.1,
                              leading=10.2, textColor=colors.HexColor("#263744")))
    styles.add(ParagraphStyle(name="ReportTableHead", fontName="ReportBold", fontSize=7.5,
                              leading=9.1, textColor=colors.white))
    styles.add(ParagraphStyle(name="ReportMono", fontName="ReportMono", fontSize=7.1,
                              leading=9.4, spaceAfter=5, wordWrap="CJK"))
    doc = SimpleDocTemplate(str(HERE / "M31_Retained_Capture_20261124.pdf"),
                            pagesize=(595.28, 841.89), rightMargin=40, leftMargin=40,
                            topMargin=38, bottomMargin=38,
                            title="M31 retained capture: native rank and payload",
                            author="Isogeny Labs research", pageCompression=1)
    width = doc.width
    story = []

    def p(text, style="ReportBody"):
        return Paragraph(text, styles[style])

    def table(headers, rows, widths):
        cells = [[p(escape(str(cell)), "ReportTableHead") for cell in headers]]
        cells += [[p(escape(str(cell)), "ReportTable") for cell in row] for row in rows]
        item = Table(cells, colWidths=widths, repeatRows=1, hAlign="LEFT")
        item.setStyle(TableStyle([
            ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#214D68")),
            ("ROWBACKGROUNDS", (0, 1), (-1, -1), [colors.white, colors.HexColor("#F1F5F8")]),
            ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
            ("LEFTPADDING", (0, 0), (-1, -1), 7), ("RIGHTPADDING", (0, 0), (-1, -1), 7),
            ("TOPPADDING", (0, 0), (-1, -1), 4), ("BOTTOMPADDING", (0, 0), (-1, -1), 4),
            ("LINEBELOW", (0, 0), (-1, 0), 0.6, colors.HexColor("#214D68")),
        ]))
        return item

    story += [p("M31 retained capture", "ReportTitle"),
              p("Native rank and payload | 24 November 2026 UTC | Run 45309117216", "ReportSmall"),
              p(f"<b>{escape(meta['claim_id'])}</b> - unsigned producer observations. <b>[pcc:unverified]</b>", "ReportSmall"),
              p(f"The native converter replayed <b>{counts['replayed_rows']} equations</b>, retained "
                f"<b>{counts['kept_rows']} rows</b> with at most two LP terms, and recorded "
                f"<b>{counts['excluded_rows']} exclusions</b> after group replay. The original admission "
                "bridge and graph evaluator consumed identical source-order prefixes. [1-4]"),
              p("<b>Exact instance.</b> p = 2147483647 = 2^31 - 1; ell = 2147446831.<br/>"
                "y^2 = x^3 + 847514846x + 2087444724; "
                "G = (625037715,2100627251); Q = (976979610,1228979761). "
                "The original target orientation and hybrid point indices are retained. [3-4]", "ReportSmall"),
              p("Rank and replay observations", "ReportH2"),
              p("Native LP coefficient ranks (exact forest): " + lp_summary + ". [2]", "ReportSmall")]
    rank_rows = [[str(row["prefix"]), row["label"] + (" / refused" if row["refused"] else "")] +
                 [text_number(row[key]) for key in
                  ("projected_rows", "useful_rank", "augmented_rank", "replay_failures")]
                 for row in data["rows"]]
    story += [table(["Prefix rows", "Arm", "Projected rows", "Useful rank", "Augmented rank", "Replay failures"],
                    rank_rows, [60, 90, 90, 80, 94, width - 414]),
              Spacer(1, 6),
              p("Useful rank is the native rank on retained small columns, including the protected target column. "
                "Augmented rank includes the right-hand side. Values are copied from the original evaluator, "
                "not the producer's bespoke rank fields. One retained capture; no repeated-sample interval. [2,5]", "ReportSmall"),
              p("Same-input rejection checks", "ReportH2"),
              table(["Corruption of this M31 input", "Exit", "Capture stdout bytes"],
                    [[row["label"], row["exit"], row["stdout_bytes"]] for row in data["controls"]],
                    [width - 178, 62, 116]),
              Spacer(1, 6),
              p("These are refusals of corrupted copies, not additional small-field workloads. "
                "The raw commands, stderr and exit files remain in the receipt. [2]", "ReportSmall"),
              Spacer(1, 5),
              p("<b>Source-bound development scope.</b> Saved generic exhaustive-pair baseline; ten public queries and a known-answer public target. "
                "No collector was rerun. Original Rust arithmetic and fixture construction are reused. "
                "The source's recovered scalar and rank are not used to admit rows. [3-4]", "ReportSmall"),
              p("Tested checkout: " + SOURCE_COMMIT + "<br/>"
                f"Job {JOB_ID}; GitHub-hosted ubuntu-latest. One build/test thread, 4 GiB virtual-address cap, "
                "300-second retained-replay deadline. This is development CI, without isolated timing qualification. [1,3]",
                "ReportSmall"),
              PageBreak(), p("Payload and evidence boundary", "ReportTitle")]
    plot = Image(str(HERE / "native_payload_counts.png"))
    image_width, image_height = plot.imageWidth, plot.imageHeight
    plot.drawWidth = width
    plot.drawHeight = width * image_height / image_width
    story.append(plot)
    story.append(p(f"Figure 1. Native integer counts from run {RUN_ID}; one capture, three prefixes. "
                   "No evaluator timing is interpreted. Units are source witness terms and residual coefficient nonzeros. "
                   "No uncertainty interval was recorded. [2] [pcc:unverified]", "ReportSmall"))
    payload_rows = [[str(row["prefix"]), row["label"] + (" / refused" if row["refused"] else "")] + [text_number(row[key]) for key in
                    ("source_terms", "residual_nonzeros", "max_row_weight", "replayed")]
                    for row in data["rows"]]
    story.append(table(["Prefix rows", "Arm", "Source terms", "Residual nonzeros", "Max row weight", "Replayed outputs"],
                       payload_rows, [60, 90, 90, 90, 84, width - 414]))
    if meta.get("result_note"):
        story += [Spacer(1, 6), p(escape(meta["result_note"]), "ReportSmall")]
    story += [p("Limits and next technical gate", "ReportH2"),
              p("The collector lacks a complete per-probe rejection trace and its original binary/toolchain attestation. "
                "Its arithmetic counter omits separate fixture setup and bespoke graph arithmetic. Consumer replay "
                "does not fill those provenance or complete cold-cost gaps. A collector-custodied stream with full "
                "collection/recovery accounting and a matched reference remains the next gate. No canonical "
                "performance graph receives a timing or speedup point from this replay. [3-4]", "ReportSmall")]
    artifact = meta["artifact"]
    story += [p(f"Artifact {artifact['id']}: {artifact['bytes']} bytes; expires {escape(artifact['expires_at'])}. "
                "The receipt pins the archive and per-file identities. [2]", "ReportSmall"),
              p("ZIP SHA-256: " + escape(artifact["sha256"]), "ReportMono"),
              p("<b>Sources</b>", "ReportSmall")]
    for number, label, url in source_links(meta):
        story.append(p(f'[{number}] <a href="{escape(url)}" color="#24628A">{escape(label)}</a>', "ReportSmall"))

    def page_footer(canvas, document):
        canvas.saveState()
        canvas.setStrokeColor(colors.HexColor("#D4DEE6"))
        canvas.line(40, 28, 555, 28)
        canvas.setFont("ReportSans", 7)
        canvas.setFillColor(colors.HexColor("#556B7A"))
        canvas.drawString(40, 17, f"M31 / PR #5076 / {meta['claim_id']} / [pcc:unverified]")
        canvas.drawRightString(555, 17, str(document.page))
        canvas.restoreState()

    doc.build(story, onFirstPage=page_footer, onLaterPages=page_footer)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    inputs = parser.add_mutually_exclusive_group(required=True)
    inputs.add_argument("--evidence-dir", type=Path)
    inputs.add_argument("--presentation-data", type=Path)
    parser.add_argument("--metadata", type=Path)
    args = parser.parse_args()
    if args.evidence_dir:
        if args.metadata is None:
            parser.error("--evidence-dir requires --metadata")
        meta = read_json(args.metadata)
        data = collect_records(args.evidence_dir, meta)
    else:
        data = read_json(args.presentation_data)
        if data["schema"] != "m31-report-presentation-data/v1":
            raise ValueError("Unexpected presentation-data schema")
    (HERE / "report_data.json").write_text(json.dumps(data, indent=2) + "\n")
    draw_plot(data)
    (HERE / "REPORT.md").write_text(markdown_report(data))
    pdf_report(data)
    print("Presentation rendered from retained native fields only.")


if __name__ == "__main__":
    main()
