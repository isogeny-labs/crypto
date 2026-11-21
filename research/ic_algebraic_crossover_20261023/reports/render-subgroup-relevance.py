"""Render source-bound native membership checks, including retained unknowns."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study = Path(__file__).resolve().parent.parent
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
for name, version in [("fixture-subgroups-20261025", 1),
                      ("fixture-subgroups-enumerated-20261025", 2)]:
    receipt = study / "pipeline-receipts" / name
    data = json.loads((receipt / "receipt.json").read_text())
    assert data["status"] == "all_lifts_trace_orders_and_membership_checks_agree"
    assert json.loads((receipt / "replay-status.json").read_text())["byte_identical"]
    source = (receipt / "source-commit.txt").read_text().strip()[:12]
    folder = study / "reports" / ("run-" + name)
    folder.mkdir(exist_ok=True)
    totals = data["totals"]
    key = "in_unique_generated_subgroup" if version == 1 else "in_generated_subgroup"
    outside = "outside_unique_generated_subgroup" if version == 1 else "outside_generated_subgroup"
    originals = [r for r in data["targets"] if r["input"] == "inputs/guess-ell8.jsonl"]
    assert len(originals) == 7
    fig, axes = plt.subplots(1, 2, figsize=(8.5, 2.5), gridspec_kw={"width_ratios": [1.1, 1]})
    values = [totals[key], totals[outside], totals["generated_subgroup_unknown"]]
    labels = ["in declared <G>", "outside declared <G>", "unknown"]
    axes[0].barh(labels, values, color=["#167d8d", "#ac6146", "#8a8f99"])
    for i, value in enumerate(values):
        axes[0].text(value + 12, i, str(value), va="center")
    axes[0].set_xlim(0, 1200)
    axes[0].invert_yaxis()
    axes[0].set_xlabel("all 1,159 original/fresh target x-values")
    axes[0].set_title("Explicit reference/twist models")
    positions = list(range(7))
    for model_index, color, label, offset in [(0, "#167d8d", "E0, r=991", -.18),
                                               (1, "#ac6146", "twist, r=2,097,349", .18)]:
        counts = [r["models"][model_index]["base_r_torsion_abscissae"] for r in originals]
        axes[1].bar([p + offset for p in positions], counts, width=.36, color=color, label=label)
    axes[1].set_xticks(positions)
    axes[1].set_xlabel("original draw")
    axes[1].set_ylabel("base x-values with points in <G>")
    axes[1].set_title("Original GF(2^22) bases")
    axes[1].legend(fontsize=8, frameon=False)
    axes[1].set_ylim(0, 85)
    for ax in axes:
        ax.spines[["top", "right"]].set_visible(False)
    fig.tight_layout()
    for ext in ("svg", "png", "pdf"):
        p = folder / ("subgroup-status." + ext)
        fig.savefig(p, bbox_inches="tight", dpi=150)
        if ext == "svg":
            p.write_text("\n".join(l.rstrip() for l in p.read_text().splitlines()) + "\n")
    plt.close(fig)
    def member(model):
        if not model["y_roots"]:
            return "no lift"
        value = model["in_generated_subgroup"]
        return "unknown" if value is None else ("yes" if value else "no")
    rows = "\n".join(f"{r['draw']} & {r['boolean_solutions']} & {member(r['models'][0])} & "
                     f"{member(r['models'][1])} & {r['models'][0]['base_r_torsion_abscissae']} & "
                     f"{r['models'][1]['base_r_torsion_abscissae']}" + r" \\" for r in originals)
    method = ("Coprime r/cofactor proves uniqueness; the 152 non-coprime cases remain unknown. "
              "The later enumerated audit is a separate, explicitly amended run."
              if version == 1 else
              "Coprime r/cofactor proves uniqueness. For the remaining GF($2^{10}$), r=11 "
              "model, complete enumeration of the actual 11-point subgroup resolves all "
              "152 earlier nulls. The first receipt is unchanged.")
    witness = ("The first receipt conservatively leaves usable-subgroup witness counts null."
               if version == 1 else
               "The amended audit explicitly finds zero usable-subgroup relations for each "
               "selected witness; at least one summand is outside the declared subgroup.")
    control = ("The repeated-prime control keeps non-coprime membership unknown."
               if version == 1 else
               "The repeated-prime control rejects r-torsion outside the actual G subgroup.")
    tex = rf"""\documentclass[10pt]{{article}}
\usepackage[margin=17mm]{{geometry}}
\usepackage{{graphicx,booktabs,hyperref}}
\usepackage[strings]{{underscore}}
\setlength{{\parindent}}{{0pt}}\setlength{{\parskip}}{{4pt}}
\begin{{document}}
\section*{{X-only fixtures: declared-subgroup audit v{version}}}
Native relevance check, 2026-10-25, source {source}. All seven original and
1,152 fresh targets are retained. Each x is checked on E0 ($a=0,b=1$) and a
trace-one-a quadratic twist in its exact field, with the largest odd prime r
and an explicitly verified generator G. These choices qualify the fixtures;
they do not retrofit a curve, subgroup or query law into the original input.

\begin{{center}}\includegraphics[width=\linewidth]{{subgroup-status.pdf}}\end{{center}}
{method}

\begin{{center}}\begin{{tabular}}{{rrrrrr}}\toprule
Draw & Boolean roots & E0 target in G & twist target in G & E0 base x & twist base x\\\midrule
{rows}
\bottomrule\end{{tabular}}\end{{center}}
The original field is GF($2^{{22}}$), modulus $z^{{22}}+z+1$.
E0 has order $991\cdot4232$; the chosen twist has a=$2^{{21}}$, order
$2\cdot2,097,349$. Base columns count distinct abscissae supporting points
in the indicated subgroup. Original draw 6 has none on either model; its
zero-guess refutation remains finite Boolean evidence, not a verified
ordinary subgroup-query result.

All three deterministically selected original SAT witnesses give two
signed curve relations on their matching model, after testing all eight
signed tuples. {witness} This checks one witness per SAT draw, not every
solution; fresh SAT witnesses are not tested by this run.

Orders use the exact E0/F2 Frobenius recurrence and complementary twist
order, controlled by exhaustive small-field point counts. Full exact
factorizations and deterministic u64 primality checks bind r. Every target
root passes substitution, an independent trace criterion, library point
membership and full-order annihilation. G is nonidentity and [r]G=O.
{control}
The native JSON binds all models, unregistered EC1 record identities,
generators and 37 input hashes. A fresh native rerun is byte-identical.

Scope: model-specific finite membership, not an ordinary query distribution,
general curve validity, closure-degree law, full relation enumeration,
complete DLP price or speedup. Local shared x86-64 host, L0; 12-GiB
address-space and 900-second limits, exit 0. No paid allocation.
Protocol: SUBGROUP_RELEVANCE_PROTOCOL.md{'; amendment: SUBGROUP_RELEVANCE_AMENDMENT.md' if version == 2 else ''}.
Receipt: \path{{../../pipeline-receipts/{name}/receipt.json}}.
\end{{document}}
"""
    (folder / "report.tex").write_text(tex)
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)
print("Rendered both native subgroup audits; initial unknowns retained.")
