#!/usr/bin/env python3
"""Lay out frozen Rust k=1 scaling runs; no relation or cost computation."""
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
SOURCE = REPO / "examples/quartic_twist_k1_scale.rs"
AUDITOR = REPO / "examples/quartic_twist_k1_scale_audit.rs"
FILES = [f"quartic_twist_k1_scale_p{p}_seed{seed}_{suffix}_v1.json"
         for p in (73, 97) for seed in range(3) for suffix in ("", "aa")]
FILES = [f.replace("__", "_").replace("seed0__", "seed0_") for f in FILES]
FILES.append("quartic_twist_k1_scale_p37_seed0_v1.json")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def audit_path(raw: Path) -> Path:
    name = raw.name.replace("_aa_v1.json", "_audit_aa_v1.json")
    if name == raw.name:
        name = raw.name.replace("_v1.json", "_audit_v1.json")
    return ROOT / name


def build(file: str) -> None:
    raw = ROOT / file
    audit_file = audit_path(raw)
    if not raw.is_file() or raw.stat().st_size == 0 or not audit_file.is_file():
        return
    rec = json.loads(raw.read_text())
    audit = json.loads(audit_file.read_text())
    assert rec["p"] == audit["p"] and rec["seed"] == audit["seed"]
    started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
    run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
    dest = REPORTS / run_id
    if dest.exists():
        return
    dest.mkdir(parents=True)
    trace = rec["trace"]
    xs = [q["query_index"] for q in trace]
    rank = [q["rank_after"] for q in trace]
    work = [q["patterns"][0]["internal_fp_muls"] for q in trace]
    fig, axes = plt.subplots(1, 2, figsize=(10.4, 3.8), layout="constrained")
    axes[0].step(xs, rank, where="post", color="#31688e")
    axes[0].set_ylim(-.3, rec["folded_columns"] + .5)
    axes[0].set_ylabel("Audited anchored rank")
    axes[0].set_xlabel("Frozen residual index")
    axes[1].plot(xs, work, color="#35b779", linewidth=.9)
    axes[1].set_ylabel("Recorded internal Fp M per residual")
    axes[1].set_xlabel("Frozen residual index")
    for ax in axes:
        ax.grid(alpha=.2)
    fig.suptitle(f"{run_id}: p={rec['p']}, seed={rec['seed']}, "
                 f"{len(trace)} one-target queries; {audit['status']}", fontsize=10)
    fig.savefig(dest / "graph.svg")
    fig.savefig(dest / "graph.png", dpi=150)
    plt.close(fig)
    command = ("target/release/examples/quartic_twist_k1_scale "
               "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson "
               f"{rec['p']} {rec['seed']}")
    observations = [
        f"Native status `{rec['status']}`; independent audit `{audit['status']}`; "
        f"{len(trace)} of {rec['recorded_stream_cap']} frozen residuals; "
        f"anchored rank {rec['anchored_rank']}/{rec['folded_columns']}; "
        f"recovered {rec['recovered']}.",
        f"Independent exact k=1 signed sets: {audit['expected_signed_relations']} reference, "
        f"{audit['polynomial_signed_relations']} polynomial, "
        f"{audit['missing_signed_relations']} missing; "
        f"{audit['system_failures']} algebraic system failures.",
        f"Counted IC: {rec['counted_ic_fp_muls']:,} Fp multiplications and "
        f"{rec['counted_ic_group_ops']:,} group operations. Same-process twist24 rho: "
        f"{rec['rho']['fp_muls']:,} Fp multiplications and "
        f"{rec['rho']['group_ops']:,} group operations.",
        f"All {audit['base_logs_verified']} physical factor logs were independently "
        f"computed with a {audit['independent_bsgs_baby_steps']}-point BSGS baby table; "
        f"the separate exact oracle built {audit['pair_entries']} signed L0 pair entries.",
        f"Public generator anchor log: {rec['anchor_log']}; the first signed physical "
        "factor may be the generator or its negation."
    ]
    host = "Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run"
    md = [f"# {run_id}", "", f"[Raw record](../../{file}) SHA-256 `{digest(raw)}`; "
          f"[independent audit](../../{audit_file.name}) SHA-256 `{digest(audit_file)}`.",
          "", "Measurement source: commit `a815598a57`; "
          f"[producer](../../../../examples/{SOURCE.name}) SHA-256 `{digest(SOURCE)}`; "
          f"[auditor](../../../../examples/{AUDITOR.name}) SHA-256 `{digest(AUDITOR)}`.",
          "", "## Execution", "", f"{host}. One public synthetic target `[271]G`.", "",
          f"Command: `{command}` with 8 GiB address-space cap and 1,200 s watchdog.",
          "", "## Recorded observations", "", *[f"- {x}" for x in observations],
          "", "![Recorded rank and polynomial field-work trace](graph.svg)", "",
          "The graph plots native recorded values only. L0 wall times in the raw record "
          "are not isolated, and hash/allocator CPU and calibrated GAE are absent. This "
          "is not a priced IC/rho or external-solver speed comparison.", "",
          "[PDF](report.pdf).", ""]
    (dest / "README.md").write_text("\n".join(md))
    typ = ['#set page(paper: "a4", margin: 18mm)', '#set text(size: 9.4pt)',
           f'= {run_id}', f'Raw SHA-256: {digest(raw)}',
           f'Audit SHA-256: {digest(audit_file)}',
           f'Source commit a815598a57; {host}. Public target [271]G.',
           '== Execution', f'`{command}`',
           '8 GiB address-space cap; 1200 s watchdog.',
           '== Recorded observations', *[f'- {x}' for x in observations],
           '#image("graph.svg", width: 100%)',
           'L0 wall is not isolated; hash/allocator CPU and calibrated GAE are absent. '
           'No priced IC/rho or external-solver claim.']
    (dest / "report.typ").write_text("\n".join(typ) + "\n")
    subprocess.run([str(TYPST), "compile", "report.typ", "report.pdf"], cwd=dest, check=True)
    print(dest.relative_to(ROOT))


if __name__ == "__main__":
    for file in FILES:
        build(file)
    rows = ["# Larger-curve k=1 polynomial run reports", "",
            "One public target per curve and seed. Each report has the native record, "
            "separately keyed exact signed-set audit, recorded-value graph and PDF. "
            "Partial and censored runs stay visible.", "",
            "| Native run | Report |", "| --- | --- |"]
    for file in FILES:
        raw = ROOT / file
        if not raw.is_file() or raw.stat().st_size == 0 or not audit_path(raw).is_file():
            continue
        started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
        run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
        rows.append(f"| [{file}]({file}) | [report](reports/{run_id}/README.md) · "
                    f"[PDF](reports/{run_id}/report.pdf) · "
                    f"[graph](reports/{run_id}/graph.svg) |")
    rows.append("")
    (ROOT / "K1_SCALE_RUNS.md").write_text("\n".join(rows))
