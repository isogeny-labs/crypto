#!/usr/bin/env python3
"""Lay out frozen native quartic-polynomial records as run reports and visuals.

This script reads recorded values only. It performs no cryptanalytic search,
verification, or measurement. The native producers and auditors own those.
"""
from __future__ import annotations

import datetime as dt
import hashlib
import json
import subprocess
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
REPORTS = ROOT / "reports"
TYPST = Path("/tmp/typst-x86_64-unknown-linux-musl/typst")
SOURCE = ROOT.parent.parent / "examples"
PILOTS = [
    "quartic_twist_tagged_s5_k0_p37_v1.json",
    "quartic_twist_tagged_s5_k1_p37_v1.json",
    "quartic_twist_tagged_s5_k2_p37_v1.json",  # retained solver failure
    "quartic_twist_tagged_s5_k2_p37_v2.json",
    "quartic_twist_tagged_s5_k3_p37_v1.json",
    "quartic_twist_tagged_s5_k4_p37_v1.json",
    "quartic_twist_tagged_s5_k0_p37_aa_v1.json",
    "quartic_twist_tagged_s5_k1_p37_aa_v1.json",
    "quartic_twist_tagged_s5_k2_p37_aa_v2.json",
    "quartic_twist_tagged_s5_k3_p37_aa_v1.json",
    "quartic_twist_tagged_s5_k4_p37_aa_v1.json",
    "quartic_twist_tagged_s5_k0_p37_aa2_v1.json",
    "quartic_twist_tagged_s5_k1_p37_aa2_v1.json",
    "quartic_twist_tagged_s5_k2_p37_aa2_v2.json",
    "quartic_twist_tagged_s5_k3_p37_aa2_v1.json",
    "quartic_twist_tagged_s5_k4_p37_aa2_v1.json",
]
DEGREES = [
    f"quartic_twist_tagged_s5_k{k}_q2_d{degree}_p37_v1.json"
    for k in (0, 4) for degree in (30, 31)
]
FULL = [
    "quartic_twist_mixed_polynomial_ic_p37_v1.json",
    "quartic_twist_mixed_polynomial_ic_p37_aa_v1.json",
    "quartic_twist_mixed_polynomial_ic_p37_v2.json",
    "quartic_twist_mixed_polynomial_ic_p37_aa_v2.json",
    "quartic_twist_mixed_polynomial_ic_p37_v3.json",
    "quartic_twist_mixed_polynomial_ic_k12_p37_v1.json",
    "quartic_twist_mixed_polynomial_ic_k12_p37_aa_v1.json",
    "quartic_twist_mixed_polynomial_ic_k1_p37_v1.json",
    "quartic_twist_mixed_polynomial_ic_k1_p37_aa_v1.json",
]
SOURCES = {
    "pilot": SOURCE / "quartic_twist_tagged_s5.rs",
    "degree": SOURCE / "quartic_twist_tagged_s5.rs",
    "full": SOURCE / "quartic_twist_mixed_polynomial_ic.rs",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(file: str, kind: str) -> None:
    raw = ROOT / file
    if not raw.is_file():
        raise SystemExit(f"missing run: {raw}")
    record = json.loads(raw.read_text())
    started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
    run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
    dest = REPORTS / run_id
    if dest.exists():
        print(f"preserved {dest.relative_to(ROOT)}")
        return
    dest.mkdir(parents=True)
    figure, ax = plt.subplots(figsize=(8.2, 3.8), layout="constrained")
    if kind == "full":
        trace = record["trace"]
        xs = [r["query_index"] for r in trace]
        ys = [sum(a["internal_fp_muls"] for a in r["patterns"]) for r in trace]
        ax.bar(xs, ys, color="#31688e")
        ax.set_xlabel("Frozen residual index")
        ax.set_ylabel("Polynomial internal Fp multiplications")
        patterns = record.get("enabled_patterns", [1, 2, 3])
        ax.set_title(f"p=37, k={patterns}, {len(trace)} queries")
        command = ("target/release/examples/quartic_twist_mixed_polynomial_ic "
                   "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson")
        if "enabled_patterns" in record:
            command += f" --patterns={','.join(map(str, patterns))}"
        details = [
            f"Status: {record['status']}; anchored rank {record['anchored_rank']}/{record['folded_columns']}; "
            f"recovered scalar {record['recovered']}.",
            f"Processed {len(trace)} frozen residuals and retained {len(record['selected_relations'])} "
            "rank-raising signed relations; all factor logs verified."
        ]
        if "counted_ic_fp_muls" in record:
            details.append(f"Counted IC: {record['counted_ic_fp_muls']:,} Fp multiplications; "
                           f"{record['counted_ic_group_ops']:,} group operations. "
                           f"Same-process twist24 rho: {record['rho']['fp_muls']:,} Fp "
                           f"multiplications; {record['rho']['group_ops']:,} group operations.")
        else:
            details.append(f"Recorded polynomial stream: {record['stream_internal_fp_muls']:,} "
                           f"internal Fp multiplications; {record['stream_group_ops']:,} "
                           "stream group operations. The total appears in the separate "
                           "independent audit and final-source record, not as a field of "
                           "this historical producer schema.")
        if "aa" in raw.stem:
            details.append("Final-source A/A control: compare all non-timing JSON fields "
                           "against the other final-source repeat. The initial pilot source "
                           "is preserved separately.")
    elif kind == "pilot":
        rows = record["rows"]
        xs = [r["query_index"] for r in rows]
        ys = [r["internal_fp_muls"] for r in rows]
        ax.bar(xs, ys, color="#35b779")
        ax.set_xlabel("Frozen residual index")
        ax.set_ylabel("Polynomial internal Fp multiplications")
        ax.set_title(f"p=37, k={record['k']}, {len(rows)} frozen residuals")
        command = ("target/release/examples/quartic_twist_tagged_s5 "
                   f"--k {record['k']} research/twist_index_calculus_20261020/"
                   "quartic_twist_mixed_v1.ndjson")
        details = [
            f"Producer status: {record['status']}; exact signed-set agreements "
            f"{sum(r['agree'] for r in rows)}/{len(rows)}; "
            f"system failures {sum(r['system_failures'] for r in rows)}.",
            f"Signed polynomial relations {sum(len(r['polynomial_signed_relations']) for r in rows)}; "
            f"field solutions {sum(r['field_solutions'] for r in rows)}; "
            f"systems attempted {sum(r['systems'] for r in rows)}.",
            "An agreement on an empty set does not make a failed system complete. "
            "The independent scalar-keyed pilot audit classifies each arm."
        ]
    else:
        k = record["k"]
        degree = record["s5_macaulay_degree"]
        prior = json.loads((ROOT / f"quartic_twist_tagged_s5_k{k}_p37_v1.json").read_text())
        comparable = [r for r in prior["rows"] if r["query_index"] == 2][0]
        values = [(29, comparable["internal_fp_muls"]), (degree, record["rows"][0]["internal_fp_muls"])]
        ax.bar([str(x) for x, _ in values], [y for _, y in values], color="#440154")
        ax.set_xlabel("Macaulay degree; same residual index 2")
        ax.set_ylabel("Polynomial internal Fp multiplications")
        ax.set_title(f"p=37, k={k}: degree closure diagnostic")
        command = ("target/release/examples/quartic_twist_tagged_s5 "
                   f"--k {k} research/twist_index_calculus_20261020/"
                   f"quartic_twist_mixed_v1.ndjson --query-index=2 --s5-degree={degree}")
        row = record["rows"][0]
        details = [
            f"Producer status {record['status']}; system outcome: "
            f"{row['system_stats'][0]['outcome']}; failure count {row['system_failures']}.",
            f"Signed-set agreement {row['agree']}; polynomial relations "
            f"{len(row['polynomial_signed_relations'])}; reference relations "
            f"{len(row['reference_signed_relations'])}.",
            "This bounded degree increase is a failed diagnostic. It does not prove the "
            "S5 system has no solution or that a different solver cannot find one."
        ]
    ax.grid(axis="y", alpha=.2)
    figure.savefig(dest / "graph.svg")
    figure.savefig(dest / "graph.png", dpi=150)
    plt.close(figure)
    digest = sha256(raw)
    source_digest = sha256(SOURCES[kind])
    if kind == "pilot" and "_aa" not in raw.stem:
        measurement_source = "frozen source commit 98fb8e09f; current source SHA below is a rendering-time snapshot"
    elif kind == "full" and ("_p37_v1" in raw.stem or "_p37_aa_v1" in raw.stem):
        measurement_source = "frozen source commit ade6c1916; current source SHA below is a rendering-time snapshot"
    elif kind == "full" and ("_p37_v2" in raw.stem or "_p37_aa_v2" in raw.stem):
        measurement_source = "intermediate modified checkout; exact historical source hash not retained; final-source parity tested separately"
    else:
        measurement_source = "final source; current SHA below is the rendering-time snapshot"
    host = "Intel Xeon Platinum 6975P-C; shared host; L0 exploratory run"
    body = [f"# {run_id}", "", f"Raw record: [{file}](../../{file}) "
            f"(SHA-256 `{digest}`).", "", f"Source: [{SOURCES[kind].name}]"
            f"(../../../../examples/{SOURCES[kind].name}) "
            f"(current SHA-256 `{source_digest}`). Measurement source: {measurement_source}.", "", "## Workload and execution", "",
            "Frozen protocol: [quartic twist line protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md).",
            "", f"Host and isolation: {host}.", "", "One public known-answer target [271]G; "
            "p=37, b0=2, r=1,873,633, seed=0. For the pilot and diagnostic, "
            "the first four registered residuals or index 2 are used; for the full "
            "producer the existing 61-residual stream is the cap.", "",
            f"Command (with an 8 GiB address-space cap and 1,200 s watchdog): `{command}`.",
            "", "## Recorded observations", "", *[f"- {line}" for line in details],
            "", "![Recorded field-work graph](graph.svg)", "",
            "The graph plots native recorded counts. It does not include native hash, allocator "
            "or partition CPU. L0 wall time in the raw record is not isolated. No priced "
            "IC/rho or fastest-solver speed claim follows from this run.", "",
            "[PDF copy](report.pdf).", ""]
    (dest / "README.md").write_text("\n".join(body))
    typ_lines = [
        '#set page(paper: "a4", margin: 19mm)',
        '#set text(size: 10pt)',
        f'= {run_id}',
        f'Raw: {file}  |  SHA-256: {digest}',
        f'Current source SHA-256: {source_digest}',
        f'Measurement source: {measurement_source}.',
        f'{host}. Public target [271]G; p=37, b0=2, r=1873633, seed=0.',
        '== Execution',
        f'`{command}`',
        'Cap: 8 GiB address space, 1200 s watchdog.',
        '== Recorded observations',
        *[f'- {line}' for line in details],
        '#image("graph.svg", width: 100%)',
        'Counted native field work only. L0 wall time is not isolated; native hash, '
        'allocator and partition CPU are missing. No priced IC/rho speed claim.'
    ]
    (dest / "report.typ").write_text("\n".join(typ_lines) + "\n")
    subprocess.run([str(TYPST), "compile", "report.typ", "report.pdf"], cwd=dest, check=True)
    print(dest.relative_to(ROOT))


if __name__ == "__main__":
    for path in PILOTS:
        build(path, "pilot")
    for path in DEGREES:
        build(path, "degree")
    for path in FULL:
        build(path, "full")
    lines = ["# Mixed-polynomial run-report index", "",
             "Each native producer invocation has a write-once report with its raw SHA-256, "
             "exact workload/command, recorded-value graph and inspected PDF. A/A copies, "
             "the failed first k=2 solver and all degree retries are retained. The "
             "[results report](MIXED_POLYNOMIAL_REPORT.md) links independent native audits "
             "and compares the counted paths.", "",
             "| Native record | Report and PDF |", "| --- | --- |"]
    for file in PILOTS + DEGREES + FULL:
        raw = ROOT / file
        started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
        run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
        lines.append(f"| [{file}]({file}) | [report](reports/{run_id}/README.md) · "
                     f"[PDF](reports/{run_id}/report.pdf) · "
                     f"[graph](reports/{run_id}/graph.svg) |")
    lines += ["", "## Independent native audits", "",
              "- [Five-arm pilot scalar-keyed audit](quartic_twist_tagged_s5_audit_p37_v1.json) "
              "and [final-source audit](quartic_twist_tagged_s5_audit_p37_final_v1.json): "
              "all 1,873,633 subgroup logs; 4/5 arms have exact sets, but k=0 and k=4 "
              "are incomplete because at least one algebraic system failed.",
              "- [Full k=1,2,3 audit](quartic_twist_mixed_polynomial_ic_audit_p37_v3.json), "
              "[k=1,2 audit](quartic_twist_mixed_polynomial_ic_k12_audit_p37_v1.json), "
              "and [k=1 audit](quartic_twist_mixed_polynomial_ic_k1_audit_p37_v1.json) "
              "independently replay complete processed signed sets, ranks, all factor logs, "
              "targets and rho.",
              "- The final-source five-arm pilot A/A diffs are "
              "`quartic_twist_tagged_s5_k{0,…,4}_p37_aa_aa2_v*.diff`; all are empty. "
              "The initial pilot and final-source replay differ only in the source/status "
              "metadata and remain separate historical records.", ""]
    (ROOT / "MIXED_POLYNOMIAL_RUNS.md").write_text("\n".join(lines))
