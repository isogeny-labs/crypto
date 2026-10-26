#!/usr/bin/env python3
"""Document layout only; plot recorded native values, never compute results."""
from pathlib import Path
import importlib.util
import json
import sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst

ROOT = Path(__file__).resolve().parent
FIG = ROOT / "figures"
spec = importlib.util.spec_from_file_location(
    "prior_layout", ROOT.parent / "s4_twist_elimination/render_reports.py")
layout = importlib.util.module_from_spec(spec)
spec.loader.exec_module(layout)

def saved(fig, name):
    FIG.mkdir(exist_ok=True)
    fig.savefig(FIG / f"{name}.svg")
    svg = FIG / f"{name}.svg"
    svg.write_text("\n".join(line.rstrip() for line in svg.read_text().splitlines()) + "\n")
    fig.savefig(FIG / f"{name}.png", dpi=150)
    plt.close(fig)

def verification_gate_figure():
    fig, ax = plt.subplots(figsize=(9.8, 3.5), layout="constrained")
    ax.axis("off")
    boxes = [(.16, "Required proof and IDC\n10 Lean theorems pass\nStronger IDC replay passes"),
             (.50, "Native software gate\n77 tests pass, zero failures\nRunner copied and hash checked"),
             (.84, "21 saved-tape audits\nSeparate science receipts\nNo collector or BSGS repeated")]
    for x,label in boxes:
        ax.text(x,.62,label,transform=ax.transAxes,ha="center",va="center",fontsize=10,
                bbox={"boxstyle":"round,pad=.65","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for x in (.33,.67):
        ax.annotate("",xy=(x+.025,.62),xytext=(x-.025,.62),xycoords="axes fraction",
                    arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.text(.5,.16,"fp-pruning-audit-v1: a software pass does not establish useful pruning or a lower IC price.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"pruning-software-gate")

def historical_failures_figure():
    fig, ax = plt.subplots(figsize=(9.8, 3.4), layout="constrained")
    ax.axis("off")
    boxes = [(.25, "Historical match v1\nBuild passes; match exits 101\nRaw/public query shapes differ"),
             (.75, "Historical match v2\nBuild exits 101\nDiagnostic JSON syntax error")]
    for x,label in boxes:
        ax.text(x,.65,label,transform=ax.transAxes,ha="center",va="center",fontsize=11,
                bbox={"boxstyle":"round,pad=.75","facecolor":"#f7eee8","edgecolor":"#9b4380"})
    ax.text(.5,.18,"Both failures are preserved. Neither is a passed fixture match or a failed mathematical relation.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"historical-match-failures")

def pruning_outcome_figure():
    v = json.loads((ROOT / "results/fp-pruning-v1-headline.json").read_text())
    rows = v["arms"]
    labels = ["Phi", "Doubling", "Pure L0"]
    fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.9), layout="constrained")
    for ax, key, title in [(axes[0], "case_local_count_sums", "Entire 9,658-residual tape"),
                            (axes[1], "through_original_full_rank_sums", "Through first full rank")]:
        values = [r[key]["proposed_finite_rejections"] for r in rows]
        ax.bar(labels, values, color=["#147d92", "#9b4380", "#b07c28"])
        for i,value in enumerate(values): ax.text(i,value+.05,str(value),ha="center")
        ax.set_ylim(0,2.5); ax.set_yticks([0,1,2]); ax.set_ylabel("Recorded proposed finite rejections")
        ax.set_title(title,fontsize=10); ax.grid(axis="y",alpha=.2)
    fig.suptitle("fp-pruning-audit-v1: 21 cases per arm; exact sets/ranks preserved; no measured price",fontsize=10)
    saved(fig,"pruning-outcome")

def external_reconciliation_figure():
    fig, ax = plt.subplots(figsize=(9.8,3.3),layout="constrained")
    ax.axis("off")
    boxes = [(.16,"Original root audit\nUnchanged input and basis\nTimeout 124 preserved"),
             (.50,"V1 duplicate detected\nActual native child stopped\nPartial output preserved"),
             (.84,"Corrected V2\nBind exact hashes and argv\nReuse failures; audit new bases")]
    for x,label in boxes:
        ax.text(x,.64,label,transform=ax.transAxes,ha="center",va="center",fontsize=10,
                bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for x in (.33,.67):
        ax.annotate("",xy=(x+.025,.64),xytext=(x-.025,.64),xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.text(.5,.16,"An owner stop or a successful external process does not establish complete physical roots.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"external-reconciliation")

def uniform_population_figure():
    v = json.loads((ROOT / "results/uniform-zero-v2-summary.json").read_text())
    names = ["original", "phi-l0-active", "double-l0-active", "l0-peel"]
    labels = ["Original L2", "Phi", "Doubling", "Pure L0"]
    colors = ["#68717a", "#147d92", "#9b4380", "#b07c28"]
    fig, axes = plt.subplots(1,2,figsize=(9.4,3.9),layout="constrained")
    for case in v["population_cases"]:
        for i,name in enumerate(names):
            a = next(a for a in case["arms"] if a["arm"]==name)
            x = i+(case["seed"]-1)*.055
            for ax,key in zip(axes,["support","second_moment_sum"]):
                ax.scatter([x],[a[key]],color=colors[i],marker=["o","s","^"][case["seed"]],s=34)
    for ax in axes:
        ax.set_xticks(range(4),labels); ax.grid(axis="y",alpha=.2)
    axes[0].set_ylabel("Nonempty residuals (recorded count)")
    axes[1].set_ylabel("Sum of squared yields (recorded integer)")
    fig.suptitle("uniform-zero-v2: complete p37 population; all three seed presentations, no IC timing",fontsize=10)
    saved(fig,"uniform-population")

def five_zero_figure():
    v = json.loads((ROOT / "results/five-zero-v1-summary.json").read_text())
    primes = [37,73,97,157,229,241,337]
    arms = [("l0-peel","Pure L0","#b07c28",-.22),
            ("phi-l0-active","Phi","#147d92",0),
            ("double-l0-active","Doubling","#9b4380",.22)]
    fig, ax = plt.subplots(figsize=(9.4,4.1),layout="constrained")
    for record in v["records"]:
        case = record["summary"]
        for name,label,color,offset in arms:
            a = next(a for a in case["arms"] if a["arm"]==name)
            x = primes.index(case["p"])+offset+(case["seed"]-1)*.045
            ax.scatter([x],[a["factor_log_rank"]],color=color,marker=["o","s","^"][case["seed"]],s=34,
                       label=label if case["p"]==37 and case["seed"]==0 else None)
        a = case["arms"][0]
        ax.scatter([primes.index(case["p"])+(case["seed"]-1)*.045],[a["unanchored_factor_columns"]],
                   color="#68717a",marker="_",s=85,label="Factor columns" if case["p"]==37 and case["seed"]==0 else None)
    ax.set_xticks(range(7),[str(p) for p in primes]); ax.set_xlabel("p (three seed presentations per curve)")
    ax.set_ylabel("Factor-log rank / number of columns"); ax.set_ylim(-.5,15); ax.grid(axis="y",alpha=.2)
    ax.legend(ncol=4,fontsize=8,loc="upper left")
    fig.suptitle("five-zero-v1: all 21 cases; oracle census, independent completeness replay separate",fontsize=10)
    saved(fig,"five-zero-rank")

def zero_identity_figure():
    v = json.loads((ROOT / "results/zero-identity-audit-v1-summary.json").read_text())
    assert v["status"] == "pass" and not v["missing"]
    primes = [37,73,97,157,229,241,337]
    fig, axes = plt.subplots(1,2,figsize=(9.4,3.8),layout="constrained")
    for case in v["cases"]:
        x = primes.index(case["p"])+(case["seed"]-1)*.045
        axes[0].scatter([x-.06],[case["predicted_signed_relations"]],color="#68717a",marker="_",s=70,
                        label="Doubled triangles" if case["p"]==37 and case["seed"]==0 else None)
        axes[0].scatter([x+.06],[case["observed_signed_relations"]],color="#9b4380",marker=["o","s","^"][case["seed"]],s=26,
                        label="All observed tuples" if case["p"]==37 and case["seed"]==0 else None)
        axes[1].scatter([x],[case["observed_rank"]],color="#9b4380",marker=["o","s","^"][case["seed"]],s=26)
    for ax in axes:
        ax.set_xticks(range(7),primes); ax.set_xlabel("p (three seed presentations)"); ax.grid(axis="y",alpha=.2)
    axes[0].set_ylabel("Signed physical tuples"); axes[0].legend(fontsize=8)
    axes[1].set_ylabel("Observed factor-log rank"); axes[1].set_yticks([0,1]); axes[1].set_ylim(-.15,1.3)
    fig.suptitle("zero-identity-audit-v1: all 21 pass; predicted rows are zero; target column is zero",fontsize=10)
    saved(fig,"zero-identity-outcome")

def external_terminal_figure():
    v = json.loads((ROOT / "results/external-audit-completion-v2-inventory.json").read_text())
    records = {r["id"]:r for r in v["cases"]}
    arms = [("l0-peel","Pure L0"),("phi-l0-active","Phi"),("double-l0-active","Doubling")]
    variants = [("global-finite-domain","Global finite"),("global-geometric","Global geometric"),
                ("orbit-1","T = 1"),("orbit-10","T = 10"),("orbit-26","T = 26"),
                ("variant-system","Variant finite"),("variant-geometric","Variant geometric")]
    fig, ax = plt.subplots(figsize=(9.2,4.5),layout="constrained")
    for x,(arm,_) in enumerate(arms):
        for y,(variant,_) in enumerate(variants):
            key = f"{arm}-{variant}"
            case = records[key]
            if case["external_process_exit"] != 0:
                label,color = "msolve 124", "#f4e1d5"
            elif case["native_audit_exit"] != 0:
                label,color = "Roots 124", "#f4e1d5"
            else:
                audit = json.loads((ROOT / "runs/external-audit-completion-v2/inventory" / f"{key}.native-summary.json").read_text())
                assert audit["all_physical_sets_equal"]
                label,color = f'Pass: rank {audit["rank"]}/{audit["columns"]}', "#dfefed"
            ax.add_patch(plt.Rectangle((x-.47,y-.43),.94,.86,facecolor=color,edgecolor="white"))
            ax.text(x,y,label,ha="center",va="center",fontsize=10)
    ax.set_xticks(range(3),[a[1] for a in arms]); ax.set_yticks(range(7),[v[1] for v in variants])
    ax.set_xlim(-.5,2.5); ax.set_ylim(6.5,-.5); ax.tick_params(length=0)
    for spine in ax.spines.values(): spine.set_visible(False)
    fig.suptitle("All 21 terminal positions: 11 physical-set passes; 10 censored, never zero-yield claims",fontsize=10)
    saved(fig,"external-terminal-outcomes")

def publication_gate_figure():
    meta = ROOT / "runs/publication-gate-v1/metadata"
    for phase in ["library-compile","library-tests","examples-compile","quartic_twist_s4_integer",
                  "quartic_twist_s4_structure","curve-id-selftest","curve-names","validation-gate-tests",
                  "validation-gate","large-files"]:
        assert (meta / f"{phase}.exit").read_text().strip() == "0"
    fig, ax = plt.subplots(figsize=(9.8,3.5),layout="constrained")
    ax.axis("off")
    boxes = [(.16,"Root + examples\n4,488 library tests pass\n119 existing ignores\n1 + 3 example tests pass"),
             (.50,"Metadata gates\nIdentity and naming pass\nValidation and size pass\nAll ten phases exit zero"),
             (.84,"Separate native run\n81 tests pass\nOwn source and runner hash\nPublic-point replay scope")]
    for x,label in boxes:
        ax.text(x,.63,label,transform=ax.transAxes,ha="center",va="center",fontsize=10,
                bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    ax.text(.5,.12,"Software validation only: no admitted timing, scientific promotion, GitHub CI or merge implied.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"publication-software-gates")

def complete_checkpoint_figure():
    v = json.loads((ROOT / "results/publication/complete-archive-verify-v3/complete-verification.json").read_text())
    p = json.loads((ROOT / "results/publication/complete-store-publish-v3.json").read_text())
    assert v["status"] == p["status"] == "pass" and v["byte_identical_repack"]
    fig, ax = plt.subplots(figsize=(10,3.2),layout="constrained")
    ax.axis("off")
    labels = ["43 reconciled runs\nLiteral manifest\nFailures retained",
              f'{v["exact_regular_file_comparisons"]:,} files\nFresh extraction\nEvery native cmp passes',
              "Identical repack\nWhole SHA-256 matches\nNo program executed",
              "Canonical S3 verified\nSize and full checksum\nPrior object retained"]
    for x,label in zip([.12,.37,.63,.88],labels):
        ax.text(x,.63,label,transform=ax.transAxes,ha="center",va="center",fontsize=9,
                bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for x in [.245,.50,.755]:
        ax.annotate("",xy=(x+.02,.63),xytext=(x-.02,.63),xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.text(.5,.14,"Artifact integrity and preservation only; no new scientific completeness or performance claim.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"complete-checkpoint-verification")

def latest_main_gate_figure():
    meta = ROOT / "results/publication/post-latest-main-v3/metadata"
    for phase in ["library-compile","library-tests","examples-compile","quartic_twist_s4_integer",
                  "quartic_twist_s4_structure","curve-id-selftest","curve-names","validation-gate-tests",
                  "validation-gate","large-files"]:
        assert (meta / f"{phase}.exit").read_text().strip() == "0"
    assert "4492 passed; 0 failed; 119 ignored" in (meta/"library-tests.stdout").read_text()
    fig, ax = plt.subplots(figsize=(9.8,3.3),layout="constrained")
    ax.axis("off")
    labels = ["V2 launcher\nPATH lacks rustc\nExit 127 before compilation",
              "V3 uses existing PATH\n4,492 library tests pass\n119 existing ignores; 1+3 examples",
              "All ten phases pass\nIdentity, naming, validation, size\nNo scientific run repeated"]
    for x,label in zip([.16,.50,.84],labels):
        ax.text(x,.63,label,transform=ax.transAxes,ha="center",va="center",fontsize=9,
                bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    ax.text(.5,.14,"Current-main software validation; no new solver timing, GitHub CI or merge inferred.",
            transform=ax.transAxes,ha="center",fontsize=10)
    saved(fig,"latest-main-software-gates")

def figures():
    v = json.loads((ROOT / "results/algebra-v1-validation.json").read_text())
    rows = v["arms"]
    labels = ["Phi", "Doubling", "Pure L0\npartial symmetry"]
    colors = ["#147d92", "#9b4380", "#b07c28"]
    fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.9), layout="constrained")
    axes[0].bar(labels, [r["total_degree"]["max"] for r in rows], color=colors)
    axes[0].set_ylabel("Recorded total degree")
    axes[1].bar(labels, [r["extension_terms"]["max"] for r in rows], color=colors)
    axes[1].set_ylabel("Recorded extension monomials (max)")
    for ax in axes: ax.grid(axis="y", alpha=.2)
    fig.suptitle("algebra-v1: 297 constructions / 21 cases; polynomial size, not solver cost", fontsize=11)
    saved(fig, "construction-profile")
    v = json.loads((ROOT / "runs/verification-v8/variants/l0-peel/summary.json").read_text())
    fig, axes = plt.subplots(1, 2, figsize=(8.8, 3.6), layout="constrained")
    before, after = v["unreduced_relation_stats"], v["relation_stats"]
    axes[0].bar(["Partial", "Full four-factor"], [before["total_degree"], after["total_degree"]], color=["#68717a", "#b07c28"])
    axes[0].set_ylabel("Recorded total degree")
    axes[1].bar(["Partial", "Full four-factor"], [before["extension_terms"], after["extension_terms"]], color=["#68717a", "#b07c28"])
    axes[1].set_ylabel("Recorded extension monomials")
    for ax in axes: ax.grid(axis="y", alpha=.2)
    fig.suptitle("verification-v8: pure L0, p37 / seed0 / query2; construction only", fontsize=10)
    saved(fig, "pure-symmetry")
    records = [json.loads((ROOT / "results/source-block-regression" / f"{arm}-audit.json").read_text())
               for arm in ("l0-peel", "phi-l0-active", "double-l0-active")]
    fig, axes = plt.subplots(1, 2, figsize=(8.8, 3.7), layout="constrained")
    labels = ["Pure L0", "Phi", "Doubling"]
    axes[0].bar(labels, [r["actual_signed_relations"] for r in records], color=["#b07c28", "#147d92", "#9b4380"])
    axes[0].set_ylabel("Distinct signed physical relations")
    axes[1].bar(labels, [r["rank"] for r in records], color=["#b07c28", "#147d92", "#9b4380"])
    axes[1].axhline(records[0]["columns"], color="#68717a", linestyle="--", label="3 columns")
    axes[1].set_ylabel("Independent rank")
    axes[1].legend()
    for ax in axes:
        ax.set_ylim(0, 3.5)
        ax.set_yticks([0, 1, 2, 3])
        ax.grid(axis="y", alpha=.2)
    fig.suptitle("Complete source-block unions: p37 / seed0 / query2, one selected positive residual", fontsize=10)
    saved(fig, "source-block-rank")
    fig, ax = plt.subplots(figsize=(10.4, 3.9), layout="constrained")
    ax.axis("off")
    nodes = [(.13, "Public curve + G,Q\nHash-checked query [a,b]\nNo factor-log oracle"),
             (.5, "Joint S5 construction\nGlobal / source blocks / inverse\nFull symmetry for pure control"),
             (.87, "Complete physical sets\nIndependent rank and recovery\nThen full cold cost + rho")]
    for x, label in nodes:
        ax.text(x, .60, label, transform=ax.transAxes, ha="center", va="center", fontsize=9,
                bbox={"boxstyle":"round,pad=.65", "facecolor":"#eef3f7", "edgecolor":"#245e7d"})
    for x in (.31, .68):
        ax.annotate("", xy=(x+.04,.60), xytext=(x-.04,.60), xycoords="axes fraction",
                    arrowprops={"arrowstyle":"->", "color":"#245e7d"})
    ax.text(.5,.15,"A smaller polynomial or more presentations alone does not certify an index-calculus gain.",
            transform=ax.transAxes, ha="center", fontsize=10)
    fig.suptitle("Joint-image study: each arrow has a separate verification obligation", fontsize=11)
    saved(fig, "joint-mechanism")

def pdf(path):
    source = layout.typst(path.read_text(), path.parent)
    source = source.replace('.svg", width:', '.png", width:')
    path.with_suffix(".typ").write_text(source)
    typst.compile(str(path.with_suffix(".typ")), output=str(path.with_suffix(".pdf")), root=str(ROOT))

def zero_figure():
    fig, ax = plt.subplots(figsize=(10.6, 4.8), layout="constrained")
    ax.axis("off")
    boxes = [(0.17,.76,"Four distinct abscissae in Fp\nZero group sum\nS4 necessity: group bridge"),
             (.53,.76,"H4 = C0 + C1 B + C2 B² + C3 B³\nDegree of B is four\nHence every Ci = 0"),
             (.85,.76,"2048 D = sum Ai Ci\nExact integer certificate\nLean and IDC pass\nMutations rejected"),
             (.70,.28,"2048 nonzero implies D = 0\nField cancellation\nLean gate passes"),
             (.23,.28,"Distinct roots imply D is nonzero\nProduct of squared differences\nQuartic identity: Lean gate passes")]
    for x,y,label in boxes:
        ax.text(x,y,label,transform=ax.transAxes,ha="center",va="center",fontsize=13,
                bbox={"boxstyle":"round,pad=.65","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for a,b in [((.34,.76),(.37,.76)),((.69,.76),(.70,.76)),((.85,.58),(.74,.43))]:
        ax.annotate("",xy=b,xytext=a,xycoords="axes fraction",arrowprops={"arrowstyle":"->","color":"#245e7d"})
    ax.annotate("contradiction",xy=(.42,.28),xytext=(.50,.28),xycoords="axes fraction",ha="center",fontsize=11,
                arrowprops={"arrowstyle":"->","color":"#9b4380"})
    fig.suptitle("Pure zero fiber: conditional algebraic obstruction; no nonzero-target or cost result",fontsize=11)
    saved(fig,"pure-zero-obstruction")

def variant_population_figure():
    v = json.loads((ROOT / "results/variant-population-v1-validation.json").read_text())
    assert v["status"] == "pass"
    fig, axes = plt.subplots(1, 2, figsize=(9.4, 3.9), layout="constrained")
    arms = [("phi-l0-active", "Phi", "#147d92"),
            ("double-l0-active", "Doubling", "#9b4380"),
            ("l0-peel", "Pure full symmetry", "#b07c28")]
    for i,(arm,label,color) in enumerate(arms):
        rows = [r for r in v["successful_records"] if r["arm"] == arm]
        axes[0].scatter([i for r in rows], [r["relation_stats"]["total_degree"] for r in rows], color=color, alpha=.3)
        axes[1].scatter([i for r in rows], [r["relation_stats"]["extension_terms"] for r in rows], color=color, alpha=.3)
    for ax in axes:
        ax.set_xticks([0,1,2], [arm[1] for arm in arms])
        ax.set_xlim(-.5,2.5)
        ax.grid(axis="y", alpha=.2)
    axes[0].set_ylabel("Recorded total degree")
    axes[1].set_ylabel("Recorded extension monomials (log scale)")
    axes[1].set_yscale("log")
    fig.suptitle(f'variant-population-v1: {v["requested_positions"]} validated positions; markers overlap; no root/cost result', fontsize=10)
    saved(fig,"variant-population")

def square_figure():
    fig, ax = plt.subplots(figsize=(10.6, 4.6), layout="constrained")
    ax.axis("off")
    nodes = [(.17,.76,"Four abscissae in Fp\nH4 = 0; B has degree four\nHence all Ci = 0\nGroup/basis bridge is separate"),
             (.52,.76,"64 S³ = sum Ai Ci\nNative exact search passes\nIndependent IDC passes"),
             (.85,.76,"8 is nonzero\nHence S = 0\nCombined Lean: 10 pass"),
             (.83,.28,"512 f = 8 q²\nEvery root of f roots q\nSeven-theorem bridge passes"),
             (.47,.28,"Three distinct roots\nCannot fit a quadratic\nField contradiction"),
             (.13,.28,"Retain infinity\nCM triangles can sum to O\nVerify group presentations")]
    for x,y,label in nodes:
        ax.text(x,y,label,transform=ax.transAxes,ha="center",va="center",fontsize=12,
            bbox={"boxstyle":"round,pad=.6","facecolor":"#eef3f7","edgecolor":"#245e7d"})
    for a,b in [((.33,.76),(.35,.76)),((.68,.76),(.70,.76)),
                ((.85,.58),(.84,.45)),((.68,.28),(.65,.28))]:
        ax.annotate("",xy=b,xytext=a,xycoords="axes fraction",
            arrowprops={"arrowstyle":"->","color":"#245e7d"})
    fig.suptitle("Stronger S4 obstruction: exact scope and checks; no measured cost gain",fontsize=11)
    saved(fig,"square-obstruction")

if __name__ == "__main__":
    if sys.argv[1:] == ["--complete-checkpoint-figure"]:
        complete_checkpoint_figure()
    elif sys.argv[1:] == ["--latest-main-gate-figure"]:
        latest_main_gate_figure()
    elif sys.argv[1:] == ["--zero-identity-figure"]:
        zero_identity_figure()
    elif sys.argv[1:] == ["--external-terminal-figure"]:
        external_terminal_figure()
    elif sys.argv[1:] == ["--publication-gate-figure"]:
        publication_gate_figure()
    elif sys.argv[1:] == ["--external-reconciliation-figure"]:
        external_reconciliation_figure()
    elif sys.argv[1:] == ["--pruning-outcome-figure"]:
        pruning_outcome_figure()
    elif sys.argv[1:] == ["--uniform-population-figure"]:
        uniform_population_figure()
    elif sys.argv[1:] == ["--five-zero-figure"]:
        five_zero_figure()
    elif sys.argv[1:] == ["--verification-gate-figure"]:
        verification_gate_figure()
    elif sys.argv[1:] == ["--historical-failures-figure"]:
        historical_failures_figure()
    elif sys.argv[1:] == ["--zero-figure"]:
        zero_figure()
    elif sys.argv[1:] == ["--square-figure"]:
        square_figure()
    elif sys.argv[1:] == ["--variant-population-figure"]:
        variant_population_figure()
    elif len(sys.argv) > 1:
        for name in sys.argv[1:]:
            path = (ROOT / name).resolve()
            assert path.is_relative_to(ROOT) and path.suffix == ".md"
            pdf(path)
    else:
        figures()
        for path in [ROOT / "README.md", ROOT / "MECHANISMS_AND_CONTROLS.md",
                     *sorted((ROOT / "reports").glob("*/REPORT.md"))]: pdf(path)
