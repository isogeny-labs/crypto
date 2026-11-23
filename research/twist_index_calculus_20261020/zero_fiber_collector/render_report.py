#!/usr/bin/env python3
"""PDF layout and plots of recorded native values only; no result computation."""
from pathlib import Path
import hashlib
import json
import re
import shutil
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

ROOT = Path(__file__).resolve().parent
FIG = ROOT / "figures"
DATA = ROOT / "results/panel-v2/native-summary.json"
RUN = ROOT / "reports/run-20261024T2003Z-polynomial-panel"
BENCH = ROOT.parents[2] / "benchmarks/20261024-twist-zero-fiber-collector"
ARMS = ("phi-l0-active", "double-l0-active")
COLORS = ("#147d92", "#9b4380")


def save(fig, name):
    fig.savefig(FIG / (name + ".svg"))
    p = FIG / (name + ".svg")
    p.write_text("\n".join(line.rstrip() for line in p.read_text().splitlines()) + "\n")
    fig.savefig(FIG / (name + ".png"), dpi=150)
    plt.close(fig)


def charts(records):
    labels = [f"{p}/{s}" for p in (37,73,97,157,229,241,337) for s in (0,1,2)]
    ids = [f"p{p}-seed{s}" for p in (37,73,97,157,229,241,337) for s in (0,1,2)]
    indexed = {(r["case"], r["arm"]): r for r in records}
    fig, axes = plt.subplots(2, 1, figsize=(10.4, 5.3), sharex=True, layout="constrained")
    for arm, color, offset, label in zip(ARMS, COLORS, (-.19,.19), ("Phi", "Doubling")):
        rows = [indexed[(i,arm)] for i in ids]
        axes[0].bar([x+offset for x in range(21)], [r["signed_physical_relations"] for r in rows], width=.35, color=color, label=label)
        axes[1].bar([x+offset for x in range(21)], [r["factor_log_rank"] for r in rows], width=.35, color=color, label=label)
    axes[1].plot(range(21), [indexed[(i,ARMS[0])]["unanchored_factor_columns"] for i in ids], color="#61686e", marker=".", label="Unknown factor columns")
    axes[0].set_ylabel("Unique signed tuples")
    axes[1].set_ylabel("Factor-log rank / columns")
    for ax in axes:
        ax.grid(axis="y", alpha=.2)
        ax.legend(fontsize=8, loc="upper left")
    axes[1].set_xticks(range(21), labels, rotation=55, fontsize=7)
    axes[1].set_xlabel("p / seed presentation; seven curves, not 21 independent secrets")
    fig.suptitle("panel-v2: exact finite physical sets and independently checked ranks", fontsize=11)
    save(fig, "physical-yield-and-rank")
    fig, axes = plt.subplots(2, 1, figsize=(10.4,5.1), sharex=True, layout="constrained")
    for arm,color,offset,label in zip(ARMS,COLORS,(-.13,.13),("Phi","Doubling")):
        rows=[indexed[(i,arm)] for i in ids]
        axes[0].scatter([x+offset for x in range(21)], [r["systems_attempted"] for r in rows], color=color, s=20, label=label)
        axes[1].scatter([x+offset for x in range(21)], [r["f4_row_reduction_fp_muls"] for r in rows], color=color, s=20, label=label)
    axes[0].set_ylabel("Complete S4 systems")
    axes[1].set_ylabel("F4 row Fp-multiplication counter")
    for ax in axes:
        ax.set_yscale("log")
        ax.grid(axis="y",alpha=.2)
        ax.legend(fontsize=8,loc="upper left")
    axes[1].set_xticks(range(21), labels, rotation=55, fontsize=7)
    axes[1].set_xlabel("p / seed presentation; counters are not complete calibrated IC costs")
    fig.suptitle("panel-v2: recorded system counts and F4 row work; elapsed time unadmitted",fontsize=10)
    save(fig,"systems-and-f4-row-work")
    fig,ax=plt.subplots(figsize=(10.4,2.5),layout="constrained")
    ax.axis("off")
    boxes=[(.11,"Sealed public input\nCold lifts / image / actions"),(.37,"Pair representatives\nExact S4 / roots / signs"),(.63,"Physical tuples\nFolded factor-log matrix"),(.89,"Independent replay\nScalar maps / separate RREF")]
    for x,label in boxes:
        ax.text(x,.65,label,ha="center",va="center",fontsize=8.5,bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"},transform=ax.transAxes)
    for x in (.24,.50,.76):
        ax.annotate("",xy=(x+.025,.65),xytext=(x-.025,.65),xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.text(.5,.16,"Zero-only equations have zero target coefficient. Charged target descent and complete IC pricing remain open.",ha="center",fontsize=9,transform=ax.transAxes)
    save(fig,"collection-and-replay")


def t(s):
    return "#text("+json.dumps(str(s),ensure_ascii=False)+")"


def table(rows, size=8):
    return '#text(size: '+str(size)+'pt)[#table(columns: '+str(len(rows[0]))+', inset: 4pt, '+', '.join(t(c)[1:] for row in rows for c in row)+')]'


def render():
    value=json.loads(DATA.read_text())
    if value["status"] != "complete-independent-physical-replay":
        raise RuntimeError("A complete-result report requires the terminal native verdict")
    FIG.mkdir(exist_ok=True)
    RUN.mkdir(parents=True,exist_ok=True)
    BENCH.mkdir(parents=True,exist_ok=True)
    charts(value["records"])
    digest=hashlib.sha256(DATA.read_bytes()).hexdigest()
    totals=value["audited_case_local_count_sums"]
    headlines=[["Arm","Signed tuples","Projective rows","Zero rows","S4 systems"]]
    work=[["Arm","Recorded F4 row Fp-multiplication counter"]]
    for arm,label in zip(ARMS,("Phi","Doubling")):
        a=totals[arm]
        headlines.append([label,a["signed_physical_relations"],a["unique_projective_augmented_rows"],a["zero_augmented_rows"],a["systems_attempted"]])
        work.append([label,a["f4_row_reduction_fp_muls"]])
    indexed={(r["case"],r["arm"]):r for r in value["records"]}
    rows=[["p / seed","Phi tuples","Phi rank / cols","Double tuples","Double rank / cols","Phi systems","Double systems"]]
    mdrows=["| p / seed | Phi tuples | Phi rank / cols | Double tuples | Double rank / cols | Phi systems | Double systems |","|---|---:|---|---:|---|---:|---:|"]
    for p in (37,73,97,157,229,241,337):
        for seed in (0,1,2):
            a=indexed[(f"p{p}-seed{seed}",ARMS[0])]; b=indexed[(f"p{p}-seed{seed}",ARMS[1])]
            cells=[f"{p} / {seed}",a["signed_physical_relations"],f'{a["factor_log_rank"]}/{a["unanchored_factor_columns"]}',b["signed_physical_relations"],f'{b["factor_log_rank"]}/{b["unanchored_factor_columns"]}',a["systems_attempted"],b["systems_attempted"]]
            rows.append(cells)
            mdrows.append("| "+" | ".join(str(c) for c in cells)+" |")
    intro="The registered 42 case-arms and their independent audits complete successfully. Each mixed arm reproduces the immutable physical census. These are case-local count sums over seven curves in three public scalar-271 presentations, not independent secret samples or an amortized DLP price."
    boundary="The 24-action pair quotient is available to both arms. Actual lifts, chosen map, action checks, complete S4 roots/signs, point sums, folded rows and factor-point verification are executed. No factor logs, BSGS or reference sets enter collection. The separate post-run verifier reads the immutable census previously checked by point-keyed enumeration."
    interpretation="The new evidence is a complete polynomial implementation and exact physical replay of the earlier structural census. Phi determines both unanchored factor columns at p37, versus one for doubling. At p157 the ranks are 2/8 versus 1/8; at p337 they are 1/14 versus 3/14. Other tested primes give zero factor-log rank. Equal aggregate tuple/row counts do not imply equal useful rank. Transported copies supply proportional folded rows."
    limits="All equations are zero-only and have zero target coefficient. No target recovery or complete IC cost is established. F4 counters cover row reduction only; other field/group counters and memory telemetry are retained separately. Shared-host elapsed diagnostics include durable journaling and are not admitted timing. Calibrated 0/0.1/1 GAE, strongest matched rho, complete external-solver pricing, independent secret sampling and scaling remain open."
    verification="The audit independently constructs scalar-multiplication action maps and the signed-pair partition, checks each representative's physical relations and their transported union, reconciles recorded F4 work and uses separate RREF. Public binding, incomplete systems, missing system, duplicate pair, counter mutation and nonempty-set tuple/sign mutations are rejected. Field/curve arithmetic and the exact S4 equation deriver are shared; the full curve/group and field-embedding Lean bridges remain open."
    history="All failed launches and native tests remain. An outer reservation deadlocked the old panel before its first collector; zero native case records existed. That service was stopped with receipts retained. The corrected driver refuses the same misuse with exit 64. The journalled source passes 84 native tests with source checks. No completed old census, target stream or successful upload was repeated."
    base=['#set page(paper: "a4", margin: 16mm, numbering: "1")','#set text(size: 9pt)','#set par(justify: true)']
    body=base+["= Polynomial collection on the zero fiber",t("panel-v2 | 2026-10-24 | producer 57cde8bee873b43d50cf2dc4e1e74a6962604f1a"),t(intro),"",table(headlines),t("Native case-local sums; signed tuples and projective rows are unique within each case."),t(boundary),'#image("figures/physical-yield-and-rank.svg", width: 100%)',t(interpretation),'#pagebreak()',"= All registered cases",t("Ranks and complete system counts are copied verbatim from the native panel summary. All three presentations are shown."),table(rows,7.1),'#image("figures/systems-and-f4-row-work.svg", width: 100%)','#pagebreak()',"= Evidence boundary and next requirement",'#image("figures/collection-and-replay.svg", width: 100%)',table(work),t(verification),"",t(limits),"",t(history),"== Remaining requirements",table([["Requirement","Status"],["Complete zero-only polynomial collection","Verified on all 42 case-arms"],["Independent complete physical-set / matrix replay","Verified on all 42 case-arms"],["Charged nonzero target descent","Not yet implemented or executed"],["Matched complete IC / rho and calibrated price","Open"],["Different-twist selection and general advantage","Untested; no universal conclusion"]]),"== Reproduction and provenance",t("Native summary SHA-256: "+digest),t("Frozen production SHA-256: ba3b7c62dd51dbb158a5407e75780cd4065bbd63d9d6baa8400e5468051b4a5f"),t("Protocol: PROTOCOL.md. Exact commands, all public inputs, independent reference hashes, per-system journals, native summary and terminal source checks are retained in the raw archive; publication details are in ARTIFACTS.md. Older 189+21 evidence remains unchanged."),t("The field, model, subgroup, generator, target, factor coordinates and trace are bound by each exact public-input hash and frozen source; p/seed are existing case aliases, not new curve identities.")]
    (ROOT/"REPORT.typ").write_text("\n\n".join(body)+"\n")
    typst.compile(str(ROOT/"REPORT.typ"),output=str(ROOT/"REPORT.pdf"),root=str(ROOT))
    md=["# Polynomial collection on the zero fiber","",intro,"", "## Recorded native totals","", "| Arm | Signed tuples | Projective rows | Zero rows | S4 systems |","|---|---:|---:|---:|---:|"]
    md += ["| "+" | ".join(str(c) for c in row)+" |" for row in headlines[1:]]
    md += ["", "![Physical yield and rank](figures/physical-yield-and-rank.svg)","",interpretation,"","## All registered cases","",*mdrows,"","![Systems and F4 row work](figures/systems-and-f4-row-work.svg)","","## Validation and boundary","",boundary,"",verification,"",limits,"",history,"","## Provenance","", "Native summary SHA-256: `"+digest+"`.","", "[Protocol](PROTOCOL.md) · [Execution history](EXECUTION.md) · [Raw artifacts](ARTIFACTS.md) · [PDF](REPORT.pdf)."]
    (ROOT/"REPORT.md").write_text("\n".join(md)+"\n")
    runmd="# Complete polynomial panel: panel-v2\n\n"+intro+"\n\n"+boundary+"\n\n"+verification+"\n\n"+limits+"\n\nSource and exact command: metadata/executed-script.sh in the raw run. Frozen source 57cde8bee873b43d50cf2dc4e1e74a6962604f1a; one science worker, 8 GiB and 1,200 s outer watchdog per case-arm, 1,180 s native deadline. Shared migrated Spot host, no admitted timing.\n\n[Results report](../../REPORT.md) · [PDF](REPORT.pdf).\n"
    (RUN/"REPORT.md").write_text(runmd)
    runbody=base+["= Complete zero-fiber polynomial panel",t("panel-v2 | start 2026-10-24T20:03:47Z | all 42 case-arms complete"),t(intro),table(headlines),'#image("../../figures/physical-yield-and-rank.svg", width: 100%)',t(verification),t(limits),t("One science worker; 8 GiB; 1,200 s outer watchdog / 1,180 s native deadline. Exact commands, source/runner/input hashes, journals, failures and independent references remain in the raw archive. No new admitted timing."),t("Producer: 57cde8bee873b43d50cf2dc4e1e74a6962604f1a. Native summary: "+digest)]
    (RUN/"REPORT.typ").write_text("\n\n".join(runbody)+"\n")
    typst.compile(str(RUN/"REPORT.typ"),output=str(RUN/"REPORT.pdf"),root=str(ROOT))
    for name in ("physical-yield-and-rank","systems-and-f4-row-work","collection-and-replay"):
        for ext in ("svg","png"):
            shutil.copyfile(FIG/(name+"."+ext),BENCH/(name+"."+ext))
    shutil.copyfile(ROOT/"REPORT.pdf",BENCH/"REPORT.pdf")
    shutil.copyfile(RUN/"REPORT.pdf",BENCH/"run-panel-v2-REPORT.pdf")
    print("Rendered native values only; PDF inspection remains required.")


if __name__=="__main__":
    render()
