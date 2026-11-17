#!/usr/bin/env python3
"""Render exact census observations; no relation or cost computation."""
from __future__ import annotations

import datetime as dt
import hashlib
import json
import subprocess
from pathlib import Path

import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
REPO = ROOT.parent.parent
REPORTS = ROOT / "reports"
TYPST = Path("/tmp/typst-x86_64-unknown-linux-musl/typst")
SOURCE = REPO / "examples/quartic_twist_k1_census.rs"
AUDITOR = REPO / "examples/quartic_twist_k1_census_audit.rs"
FILES = [f"quartic_twist_k1_census_p{p}_seed{s}_{suffix}.json"
         for p in (73, 97) for s in range(3) for suffix in ("v1", "aa_v1")]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def audit_path(raw: Path) -> Path:
    if raw.name.endswith("_aa_v1.json"):
        return raw.with_name(raw.name.replace("_aa_v1.json", "_audit_aa_v1.json"))
    return raw.with_name(raw.name.replace("_v1.json", "_audit_v1.json"))


def check(rec: dict, audit: dict) -> None:
    assert rec["status"] == audit["status"] == "pass"
    assert (rec["p"], rec["seed"], rec["queries"], rec["total_k1_signed_relations"]) == (
        audit["p"], audit["seed"], audit["queries"], audit["total_k1_signed_relations"])
    assert len(rec["trace"]) == rec["queries"]
    assert sum(q["k1_signed_relations"] for q in rec["trace"]) == rec["total_k1_signed_relations"]
    assert audit["per_query_signed_set_digests_verified"] == rec["queries"]


def build(file: str) -> str | None:
    raw = ROOT / file
    aud = audit_path(raw)
    if not raw.is_file() or not aud.is_file() or not raw.stat().st_size or not aud.stat().st_size:
        return None
    rec = json.loads(raw.read_text())
    audit = json.loads(aud.read_text())
    check(rec, audit)
    started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
    run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
    dest = REPORTS / run_id
    if dest.exists():
        return run_id
    dest.mkdir(parents=True)
    trace = rec["trace"]
    xs = [q["query_index"] for q in trace]
    count = [q["k1_signed_relations"] for q in trace]
    cumulative = [q["cumulative_k1_signed_relations"] for q in trace]
    lam = rec["uniform_expected_per_query_numerator"] / rec["uniform_expected_per_query_denominator"]
    fig, axes = plt.subplots(1, 2, figsize=(10.4, 3.8), layout="constrained")
    axes[0].plot(xs, count, linestyle="none", marker="|", markersize=4.5, color="#31688e")
    axes[0].set_ylabel("Exact signed k=1 set size")
    axes[0].set_xlabel("Frozen residual index")
    axes[1].plot(xs, cumulative, color="#31688e", label="Exact census")
    axes[1].plot(xs, [lam * (x + 1) for x in xs], color="#d95f02", linestyle="--",
                 label="Uniform-occupancy expectation")
    axes[1].set_ylabel("Cumulative signed relations")
    axes[1].set_xlabel("Frozen residual index")
    axes[1].legend(fontsize=7)
    for ax in axes:
        ax.grid(alpha=.2)
    fig.suptitle(f"{run_id}: p={rec['p']}, seed={rec['seed']}; "
                 f"{rec['total_k1_signed_relations']} exact relations", fontsize=10)
    fig.savefig(dest / "graph.svg")
    fig.savefig(dest / "graph.png", dpi=150)
    plt.close(fig)
    command = ("target/release/examples/quartic_twist_k1_census "
               "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson "
               f"{rec['p']} {rec['seed']}")
    observations = [
        f"Exact point-keyed group census and independent scalar-keyed audit both pass: "
        f"{rec['total_k1_signed_relations']} signed k=1 relations across "
        f"{rec['queries']} frozen residuals; {rec['zero_k1_queries']} have zero.",
        f"First hit at zero-based query {rec['first_k1_hit']}; uniform-occupancy heuristic "
        f"{rec['uniform_expected_total']:.3f} relations, observed/heuristic "
        f"{rec['observed_over_uniform_expected']:.3f}.",
        f"Independent BSGS verified {audit['factor_logs_verified']} physical factor logs "
        f"and the target log using a {audit['independent_bsgs_baby_steps']}-point baby table; "
        f"all {audit['per_query_signed_set_digests_verified']} complete signed-set digests match.",
        f"Point-keyed census charged {rec['setup_fp_muls']:,} setup and "
        f"{rec['query_fp_muls']:,} query Fp multiplications; these are reference-oracle "
        "costs, not polynomial IC costs."
    ]
    host = "Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run"
    md = [f"# {run_id}", "", f"[Raw record](../../{file}) SHA-256 `{digest(raw)}`; "
          f"[independent audit](../../{aud.name}) SHA-256 `{digest(aud)}`.", "",
          "Census protocol commit `09bf98c033`; independent scalar-audit amendment "
          "`547a52330a`; "
          f"[point-keyed producer](../../../../examples/{SOURCE.name}) SHA-256 `{digest(SOURCE)}`; "
          f"[scalar-keyed auditor](../../../../examples/{AUDITOR.name}) SHA-256 `{digest(AUDITOR)}`.",
          "", "## Execution", "", f"{host}. Entire frozen group-oracle stream, including "
          "queries after polynomial IC stopping.", "", f"Command: `{command}` with 8 GiB "
          "address-space cap and 1,200 s watchdog.", "", "## Recorded observations", "",
          *[f"- {x}" for x in observations], "",
          "![Exact signed occupancy and cumulative heuristic](graph.svg)", "",
          "The dashed curve is a first-moment heuristic, not a fitted model or a "
          "confidence interval. Relations within a stream can be dependent. "
          "The group-oracle cost is outside the candidate polynomial IC pipeline.", "",
          "[PDF](report.pdf).", ""]
    (dest / "README.md").write_text("\n".join(md))
    typ = ['#set page(paper: "a4", margin: 18mm)', '#set text(size: 9.4pt)',
           f'= {run_id}', f'Raw SHA-256: {digest(raw)}', f'Audit SHA-256: {digest(aud)}',
           f'Census protocol 09bf98c033; scalar-audit amendment 547a52330a; {host}.',
           '== Execution', f'`{command}`',
           '8 GiB address-space cap; 1200 s watchdog.', '== Recorded observations',
           *[f'- {x}' for x in observations], '#image("graph.svg", width: 100%)',
           'Dashed expectation is a first-moment heuristic, not a fitted model or '
           'confidence interval. Group-oracle work is outside candidate IC cost.']
    (dest / "report.typ").write_text("\n".join(typ) + "\n")
    subprocess.run([str(TYPST), "compile", "report.typ", "report.pdf"], cwd=dest, check=True)
    print(dest.relative_to(ROOT))
    return run_id


if __name__ == "__main__":
    run_ids = {file: build(file) for file in FILES}
    rows = ["# Full-stream k=1 occupancy census run reports", "",
            "Exact point-keyed group relation sets across all frozen residuals. "
            "The scalar-keyed BSGS auditor independently checks every signed-set digest. "
            "First and A/A records are retained separately.", "",
            "| Native run | Report |", "| --- | --- |"]
    for file, run_id in run_ids.items():
        if run_id:
            rows.append(f"| [{file}]({file}) | [report](reports/{run_id}/README.md) · "
                        f"[PDF](reports/{run_id}/report.pdf) · "
                        f"[graph](reports/{run_id}/graph.svg) |")
    rows.append("")
    (ROOT / "K1_CENSUS_RUNS.md").write_text("\n".join(rows))
