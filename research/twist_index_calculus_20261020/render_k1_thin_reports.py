#!/usr/bin/env python3
"""Render frozen native polynomial thinning runs; no relation computation."""
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
SOURCE = REPO / "examples/quartic_twist_k1_thin.rs"
AUDITOR = REPO / "examples/quartic_twist_k1_thin_audit.rs"
FILES = [f"quartic_twist_k1_thin_p{p}_seed{seed}_j{j}_{suffix}.json"
         for p, maximum in ((73, 4), (97, 3))
         for seed in range(3) for j in range(1, maximum + 1)
         for suffix in ("v1", "aa_v1")]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def audit_path(raw: Path) -> Path:
    if raw.name.endswith("_aa_v1.json"):
        return raw.with_name(raw.name.replace("_aa_v1.json", "_audit_aa_v1.json"))
    return raw.with_name(raw.name.replace("_v1.json", "_audit_v1.json"))


def build(file: str) -> str | None:
    raw = ROOT / file
    aud = audit_path(raw)
    if not raw.is_file() or not aud.is_file() or not raw.stat().st_size or not aud.stat().st_size:
        return None
    rec = json.loads(raw.read_text())
    audit = json.loads(aud.read_text())
    assert (rec["p"], rec["seed"], rec["retained_l2_orbits"]) == (
        audit["p"], audit["seed"], audit["retained_l2_orbits"])
    assert rec["status"] == audit["status"]
    assert audit["subset_coordinates_verified"]
    started = dt.datetime.fromtimestamp(raw.stat().st_mtime, dt.timezone.utc)
    run_id = f"run-{started:%Y%m%dT%H%MZ}-{raw.stem}"
    dest = REPORTS / run_id
    if dest.exists():
        return run_id
    dest.mkdir(parents=True)
    trace = rec["trace"]
    xs = [q["query_index"] for q in trace]
    rank = [q["rank_after"] for q in trace]
    internal = [q["patterns"][0]["internal_fp_muls"] for q in trace]
    fig, axes = plt.subplots(1, 2, figsize=(10.4, 3.8), layout="constrained")
    axes[0].step(xs, rank, where="post", color="#31688e")
    axes[0].set_ylim(-.3, rec["folded_columns"] + .5)
    axes[0].set_ylabel("Independently audited anchored rank")
    axes[0].set_xlabel("Frozen residual index")
    axes[1].plot(xs, internal, color="#35b779", linewidth=.9)
    axes[1].set_ylabel("Recorded S4 internal Fp M / residual")
    axes[1].set_xlabel("Frozen residual index")
    for ax in axes:
        ax.grid(alpha=.2)
    fig.suptitle(f"{run_id}: p={rec['p']}, seed={rec['seed']}, "
                 f"{rec['retained_l2_orbits']} L2 orbits; {audit['status']}", fontsize=10)
    fig.savefig(dest / "graph.svg")
    fig.savefig(dest / "graph.png", dpi=150)
    plt.close(fig)
    baseline_file = ROOT / f"quartic_twist_k1_scale_p{rec['p']}_seed{rec['seed']}_v1.json"
    baseline = json.loads(baseline_file.read_text())
    ratio = baseline["counted_ic_fp_muls"] / rec["counted_ic_fp_muls"]
    systems = sum(q["patterns"][0]["systems"] for q in trace)
    command = ("target/release/examples/quartic_twist_k1_thin "
               "research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson "
               f"{rec['p']} {rec['seed']} {rec['retained_l2_orbits']}")
    observations = [
        f"Native `{rec['status']}`; independent point-keyed/BSGS audit `{audit['status']}`; "
        f"{len(trace)} residuals, anchored rank {rec['anchored_rank']}/{rec['folded_columns']}, "
        f"recovered {rec['recovered']}.",
        f"Physical base {rec['base_size']} retained points from {rec['full_base_size']} "
        f"constructed points; {rec['retained_l2_orbits']} complete L2 orbits. "
        "Full-base construction and map, plus a proper-subset map, are charged.",
        f"Exact signed k=1 sets on processed prefix: "
        f"{audit['expected_signed_relations']} reference, "
        f"{audit['polynomial_signed_relations']} polynomial, "
        f"{audit['missing_signed_relations']} missing; "
        f"{systems:,} attempted S4 systems, {audit['system_failures']} failures.",
        f"Counted candidate: {rec['counted_ic_fp_muls']:,} Fp multiplications and "
        f"{rec['counted_ic_group_ops']:,} group operations; same-seed full-base "
        f"control {baseline['counted_ic_fp_muls']:,} Fp multiplications "
        f"(full/candidate {ratio:.3f}).",
        f"Same-target twist24 rho: {rec['rho']['fp_muls']:,} Fp multiplications "
        f"and {rec['rho']['group_ops']:,} group operations. Independent BSGS "
        f"verified {audit['base_logs_verified']} retained physical factor logs."
    ]
    host = "Shared Intel Xeon Platinum 6975P-C host; L0 exploratory run"
    md = [f"# {run_id}", "", f"[Raw record](../../{file}) SHA-256 `{digest(raw)}`; "
          f"[independent audit](../../{aud.name}) SHA-256 `{digest(aud)}`.", "",
          "Frozen native-thinning protocol commit `cadd7b9001`; "
          f"[producer](../../../../examples/{SOURCE.name}) SHA-256 `{digest(SOURCE)}`; "
          f"[auditor](../../../../examples/{AUDITOR.name}) SHA-256 `{digest(AUDITOR)}`.",
          "", "## Execution", "", f"{host}. One public target `[271]G`.", "",
          f"Command: `{command}` with 8 GiB address-space cap and 1,200 s watchdog.",
          "", "## Recorded observations", "", *[f"- {x}" for x in observations],
          "", "![Recorded rank and polynomial field-work trace](graph.svg)", "",
          "The graph plots native recorded values only. Hash/tree and allocator CPU, "
          "calibrated GAE and isolated timing are absent. Field-operation ratios "
          "do not constitute a priced IC/rho or external-solver speed claim.", "",
          "[PDF](report.pdf).", ""]
    (dest / "README.md").write_text("\n".join(md))
    typ = ['#set page(paper: "a4", margin: 18mm)', '#set text(size: 9.4pt)',
           f'= {run_id}', f'Raw SHA-256: {digest(raw)}', f'Audit SHA-256: {digest(aud)}',
           f'Protocol cadd7b9001; {host}. Public target [271]G.',
           '== Execution', f'`{command}`', '8 GiB address-space cap; 1200 s watchdog.',
           '== Recorded observations', *[f'- {x}' for x in observations],
           '#image("graph.svg", width: 100%)',
           'Hash/tree and allocator CPU, calibrated GAE and isolated timing are absent. '
           'No priced IC/rho or external-solver claim.']
    (dest / "report.typ").write_text("\n".join(typ) + "\n")
    subprocess.run([str(TYPST), "compile", "report.typ", "report.pdf"], cwd=dest, check=True)
    print(dest.relative_to(ROOT))
    return run_id


if __name__ == "__main__":
    run_ids = {file: build(file) for file in FILES}
    assert len(run_ids) == 42 and all(run_ids.values()), "incomplete native thinning report grid"
    rows = ["# Native polynomial orbit-thinning run reports", "",
            "One public target for each `(p,seed,j)` case; first and A/A records remain "
            "separate. The point-keyed auditor checks every processed signed set and "
            "derives orbit-complete subsets from the frozen full physical base.", "",
            "| Native run | Report |", "| --- | --- |"]
    for file, run_id in run_ids.items():
        if run_id:
            rows.append(f"| [{file}]({file}) | [report](reports/{run_id}/README.md) · "
                        f"[PDF](reports/{run_id}/report.pdf) · "
                        f"[graph](reports/{run_id}/graph.svg) |")
    rows.append("")
    (ROOT / "K1_THIN_NATIVE_RUNS.md").write_text("\n".join(rows))
