#!/usr/bin/env python3
"""Render recorded native values and execution receipts; no research computation."""
from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent
TYPST = Path(os.environ.get("TWIST_REPORT_TYPST", "/tmp/typst-x86_64-unknown-linux-musl/typst"))
JOBS = [
    ("20261023-015937-twist-s4-structure-00b8fb", "Build failed; no derivation", "The sparse source snapshot omitted tools/volcano-cli. Build exit 101. No mathematical experiment ran.", "failure"),
    ("20261023-020454-twist-s4-structure-v2-afc5d6", "Checkout failed; no derivation", "GitHub requested credentials unavailable to this runner. The probe never compiled or executed.", "failure"),
    ("20261023-021219-twist-s4-integer-c988e0", "Exact integer derivation", "Native standard-library derivation: 191 expanded and 49 symmetric monomials; zero reconstruction remainder. The coefficient mutation test passed.", "integer"),
    ("20261023-022455-twist-s4-profile-30c47f", "Complete coefficient profile", "All 46,200 finite peels match independently interpolated coefficients. All original and linearly reduced component degree profiles remain [4,4,4,4]. IDC and 13 native tests passed.", "profile"),
    ("20261023-023024-twist-s4-full-proof-dadb56", "INVALID proof receipt; exact derivation retained", "The 18-term integer derivation passed. Lean extraction exhausted /tmp. Empty version/build/audit logs and absent olean disqualify the apparent done/0 status. No proof pass is credited.", "full-invalid"),
    ("20261023-023821-twist-s4-f4-roots-c91c59", "Eight exact root-set comparisons", "All eight F4 field-root sets match complete Fp^3 enumeration and signed curve checks. Both missed relations are recovered. A coefficient mutation fails the frozen input hash before solving. 48 tests passed.", "roots"),
    ("20261023-024635-twist-s4-proof-disk-678af3", "Theorems compiled; audit failed", "Both theorem artifacts compiled and print only standard axioms. The separate audit failed because its subprocess could not locate lean on PATH. Whole-job exit 1; no audit pass credited.", "failure"),
    ("20261023-025202-twist-s4-f4-prefix-9b3f7b", "Collector built; test build failed", "The bundle omitted an include_str certificate fixture required by the existing tests. No collector ran. The corrected bundle retains the test and its input.", "failure"),
    ("20261023-025323-twist-s4-proof-audit-734f57", "Both kernel proofs and axiom audits passed", "Lean 4.34.1: 2 theorems, 0 failing, 0 compiler-trusted. Nonempty compiled artifacts, both theorem names and positive audit count are required before success.", "proof"),
    ("20261023-025542-twist-s4-prefix-all-48c275", "Complete first/A/A prefix collectors", "Two byte-identical complete collectors; 46,200 systems each; seven exact signed-set oracle matches; seven full ranks and all target/factor logarithms verified. Zero undetermined systems; 48 tests passed.", "prefix"),
]


def read(path: Path):
    return json.loads(path.read_text())


def save(fig, dest: Path, name="graph"):
    fig.savefig(dest / f"{name}.svg")
    svg = dest / f"{name}.svg"
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines())+"\n")
    fig.savefig(dest / f"{name}.png", dpi=150)
    plt.close(fig)


def overview():
    data = read(ROOT / "results/prefixes.json")["runs"]
    labels = [f"{r['p']}/{r['seed']}" for r in data]
    old_rank = [read(ROOT.parent / f"quartic_twist_k1_scale_p{r['p']}_seed{r['seed']}_v1.json")["trace"][-1]["query_index"] for r in data]
    # All y values are copied from frozen native records, including query indices.
    fig, ax = plt.subplots(1, 2, figsize=(10.5, 3.9), layout="constrained")
    x = list(range(len(data)))
    ax[0].bar([i-.17 for i in x], [r["old_baseline_relations"] for r in data], width=.34, label="Original polynomial collector", color="#6b7280")
    ax[0].bar([i+.17 for i in x], [r["signed_relations"] for r in data], width=.34, label="F4; exact oracle equal", color="#187c6a")
    ax[0].set_ylabel("Signed relations on original prefix (count)")
    ax[0].legend(fontsize=7)
    ax[1].plot(x, old_rank, "o", label="Original full-rank query", color="#6b7280")
    ax[1].plot(x, [r["first_full_rank_query"] for r in data], "+", markersize=11, label="F4 full-rank query", color="#187c6a")
    ax[1].set_yscale("log")
    ax[1].set_ylabel("First full-rank query index (zero based)")
    ax[1].legend(fontsize=7)
    for a in ax:
        a.set_xticks(x, labels)
        a.set_xlabel("Prime p / seed; one public target per run")
        a.grid(axis="y", alpha=.2)
    fig.suptitle("Frozen-prefix completeness and rank: two identical native runs", fontsize=11)
    save(fig, ROOT, "overview")
    (ROOT / "overview-data.json").write_text(json.dumps({"source":"results/prefixes.json", "runs":data, "original_full_rank_query_indices":old_rank}, indent=2)+"\n")


def graph(raw: Path, dest: Path, job: str, title: str, notes: str, kind: str):
    if kind == "prefix":
        for suffix in ("svg", "png"):
            (dest / f"graph.{suffix}").write_bytes((ROOT / f"overview.{suffix}").read_bytes())
        return
    fig, ax = plt.subplots(figsize=(9.7, 3.4), layout="constrained")
    if kind in ("integer", "full-invalid"):
        data = read(raw / "out/structure.json")
        names = ["Expanded R", "First-three symmetric H"]
        values = [data["expanded_resultant_terms"], data["symmetric_terms"]]
        if kind == "full-invalid":
            names.append("Fully symmetric H4")
            values.append(data["full_symmetric_terms"])
        ax.bar(names, values, color="#31688e")
        ax.set_ylabel("Recorded nonzero integer monomials (count)")
        ax.text(.98,.98,"Exact algebra only; no solving-cost inference",transform=ax.transAxes,ha="right",va="top",fontsize=8)
    elif kind == "profile":
        data = read(raw / "out/profile-v1/profile.json")
        ax.bar([f"{r['p']}/{r['seed']}" for r in data["runs"]], [r["coefficient_matches"] for r in data["runs"]], color="#31688e")
        ax.set_xlabel("Prime p / seed")
        ax.set_ylabel("Exact component-polynomial matches (systems)")
    elif kind == "roots":
        data = read(raw / "out/fixtures-v1/summary.json")["cases"]
        labels = [f"{r['p']}/{r['seed']} q{r['query']}" for r in data]
        ax.bar(labels, [r["full_field_assignments"] for r in data], color="#31688e")
        ax.tick_params(axis="x", labelsize=8)
        ax.set_ylabel("Exhaustive rational assignments (count)")
        ax.text(.02,.98,"Every complete F4 root set equals the exhaustive reference",transform=ax.transAxes,va="top",fontsize=8)
    else:
        ax.axis("off")
        color = "#187c6a" if kind == "proof" else "#a33a37"
        ax.text(.5,.88,title,ha="center",fontsize=13,color=color,weight="bold",transform=ax.transAxes)
        for x, text in ((.16,"Source and toolchain"),(.5,"Execute / compile"),(.84,"Check actual artifacts")):
            ax.text(x,.52,text,ha="center",va="center",bbox={"boxstyle":"round,pad=.7","facecolor":"#edf1f3","edgecolor":color},transform=ax.transAxes,fontsize=10)
        for x in (.31,.65):
            ax.annotate("",xy=(x+.05,.52),xytext=(x-.05,.52),xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":color})
        ax.text(.5,.15,"Execution diagram: fewer than two comparable numeric observations.\nSee the report for the precise successful or failed stage.",ha="center",transform=ax.transAxes,fontsize=10)
    if kind in ("integer", "full-invalid", "profile", "roots"):
        ax.grid(axis="y",alpha=.2)
    fig.suptitle(job, fontsize=10)
    save(fig,dest)


def plain(s: str) -> str:
    s = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"\1 (\2)", s)
    return s.replace("**", "").replace("`", "")


def typst(md: str, image_base: Path) -> str:
    """A document-layout conversion, with research numbers left verbatim."""
    out = ['#set page(paper: "a4", margin: 17mm)', '#set text(size: 9pt)', '#set par(justify: true)', '#show raw: set text(size: 7.6pt)']
    lines = md.splitlines()
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith("```"):
            block=[]; i+=1
            while i<len(lines) and not lines[i].startswith("```"):
                block.append(lines[i]); i+=1
            out.append("#raw("+json.dumps("\n".join(block),ensure_ascii=False)+", block: true)")
        elif line.startswith("!["):
            path = re.search(r"\]\(([^)]+)\)",line).group(1)
            out.append('#image('+json.dumps(path)+', width: 100%)')
        elif line.startswith("|"):
            table=[]
            while i<len(lines) and lines[i].startswith("|"):
                cells=[plain(s.strip()) for s in lines[i].strip().strip("|").split("|")]
                if not all(re.fullmatch(r"[-: ]+",s) for s in cells): table.append(cells)
                i+=1
            out.append('#text(size: 6.8pt)[#table(columns: '+str(len(table[0]))+', inset: 4pt, '+', '.join('text('+json.dumps(c,ensure_ascii=False)+')' for row in table for c in row)+')]')
            continue
        elif line.startswith("#"):
            n=len(line)-len(line.lstrip("#"))
            out.append("="*min(n,3)+" #text("+json.dumps(plain(line[n:].strip()),ensure_ascii=False)+")")
        elif not line.strip():
            out.append("")
        else:
            para=[line]; i+=1
            while i<len(lines) and lines[i].strip() and not lines[i].startswith(("#","|","```","![")):
                para.append(lines[i]); i+=1
            out.append("#text("+json.dumps(plain(" ".join(para)),ensure_ascii=False)+")\n")
            continue
        i+=1
    return "\n".join(out).rstrip()+"\n"


def compile_pdf(dest: Path, md: str):
    (dest / "report.typ").write_text(typst(md,dest))
    subprocess.run([str(TYPST),"compile","report.typ","report.pdf"],cwd=dest,check=True)


def main():
    overview()
    reports=ROOT/"reports"; reports.mkdir(exist_ok=True)
    index=["# Exact S4 round: all launched-job reports", "", "Raw job trees are extracted from ../runs.tar.xz. Failures and invalid receipts remain visible.", "", "| Job | Outcome and report |", "| --- | --- |"]
    for job,title,notes,kind in JOBS:
        raw=ROOT/"runs"/job
        dest=reports/job; dest.mkdir(exist_ok=True)
        status=read(raw/"status.json")
        graph(raw,dest,job,title,notes,kind)
        sha=hashlib.sha256((raw/"cmd.sh").read_bytes()).hexdigest()
        host="c7i.2xlarge Spot" if job.startswith(("20261023-015937","20261023-020454")) else "c7i.large Spot"
        (dest/"command.sh").write_bytes((raw/"cmd.sh").read_bytes())
        md=f"""# {title}

Job `{job}`. Native execution and evidence only; no admitted elapsed-time comparison.

## Exact execution

Protocol commits: `509df89c5e`, `3ff04c056`, `40fbae5a7`, `e6fc49a7d`, as applicable to each stage. The actual executed source snapshot is retained in this job's raw tree.

Host: {host}. Started {status['started']}. Terminal phase `{status['phase']}`, exit `{status['exit_code']}`. All launched instances terminated. Exact command: [command.sh](command.sh), SHA-256 `{sha}`. Host, environment and rusage receipts are in the extracted raw job tree.

## Recorded observations

{notes}

![Recorded native observations or execution diagram](graph.svg)

## Scope and limits

Structural and collector processes have an 8 GiB address-space cap and 1,200 s watchdog where invoked by the recorded command. Fixed native source and exact raw inputs are retained. No seed or failed stage is dropped. Proof-wrapper setup and extraction do not use the theorem process's cap. Counts shown are recorded native values, with no estimated intervals or timing ratios. F4 row-reduction counters are not complete solver or index-calculus costs. The final prefix A/A checks deterministic output, not timing isolation. Full cost, five-round isolated timing, strongest matched rho calibration and msolve/Magma comparison remain separate uncompleted gates.

[Round report](../../REPORT.md). [PDF](report.pdf).
"""
        (dest/"README.md").write_text(md)
        compile_pdf(dest,md)
        index.append(f"| {job} | [{title}]({job}/README.md) · [PDF]({job}/report.pdf) |")
    (reports/"README.md").write_text("\n".join(index)+"\n")
    compile_pdf(ROOT,(ROOT/"REPORT.md").read_text())


if __name__=="__main__":
    main()
