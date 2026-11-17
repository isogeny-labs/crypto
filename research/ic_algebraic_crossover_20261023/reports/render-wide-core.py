"""Render retained native core/profile data. No research computation here."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study = Path(__file__).resolve().parent.parent
receipt = study / "pipeline-receipts/wide-core-audit-20261025"
profile = json.loads((receipt / "constructor-profile-audit.json").read_text())
assert profile["status"] == "raw_totals_manifests_binaries_and_outputs_agree"
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})

def save(fig, folder, stem):
    folder.mkdir(exist_ok=True)
    for ext in ("svg", "png", "pdf"):
        p = folder / (stem + "." + ext)
        fig.savefig(p, bbox_inches="tight", dpi=150)
        if ext == "svg":
            p.write_text("\n".join(l.rstrip() for l in p.read_text().splitlines()) + "\n")
    plt.close(fig)

def render(folder, title, body):
    tex = r"""\documentclass[10pt]{article}
\usepackage[margin=17mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\section*{""" + title + "}\n" + body + "\n\\end{document}\n"
    (folder / "report.tex").write_text(tex)
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"],
                       cwd=folder, stdout=log, stderr=subprocess.STDOUT, check=True)

folder = study / "reports/run-wide-constructor-profiles-20261025"
arms = profile["arms"]
fig, ax = plt.subplots(figsize=(8.1, 2.7))
ax.barh([a["arm"] for a in arms], [a["whole_process_ir"] for a in arms], color=["#ac6146", "#167d8d"])
for i, arm in enumerate(arms):
    ax.text(arm["whole_process_ir"] + 90000, i, f"{arm['whole_process_ir']:,}", va="center")
ax.set_xlim(0, 11000000)
ax.invert_yaxis()
ax.set_xlabel("Callgrind whole-process Ir (simulated instructions)")
ax.set_title("Same wide m=6 constructor input; identical returned field")
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
save(fig, folder, "constructor-instructions")
rows = "\n".join(f"{r['m']} & {r['base_modulus']} & {r['extension_coefficients']}" + r" \\" for r in profile["field_constructor_preflights"])
id_path = study / "constructor-difference.identity.json"
identity = json.loads(id_path.read_text())["id"] if id_path.exists() else "certificate pending"
render(folder, "Wide quartic constructor: instruction attribution", rf"""
Native retained runs, 2026-10-25; one before and one after, exact command
\texttt{{ic_quartic_core_probe wide 6}}. Frozen binaries:
before source 335d168aa, SHA prefix 81ab248653df; after source c46089aed,
SHA prefix df54670d4d21. AMD EPYC 9V74, Linux x86-64, shared managed host,
L0. Valgrind 3.24.0 Callgrind; one Rayon thread, no hardware PMU.

\begin{{center}}\includegraphics[width=\linewidth]{{constructor-instructions.pdf}}\end{{center}}
Measured simulated instruction totals are 9,130,856 and 575,951. A native
auditor independently parses raw summary/totals, matches hotspot JSON and
manifests, rehashes both frozen executables, and requires byte-identical
stdout. Both return base modulus 67 and extension coefficients [2,1,0,1].
Separate narrow/wide \texttt{{both 6}} controls also return identical output.

The implementation skips $X^4+c_1X+c_0$ when m is even and the perfect-fourth-
power case for every m. The readable exclusion is in CONSTRUCTOR_EXCLUSION.md;
six checked Lean dependencies include the conditional root-two-cycle theorem.
The finite-field/Frobenius/irreducibility bridge is not formalized. Identity
\texttt{{{identity}}} certifies the integer expansion only, with a faithful
Lean proof pointer; the v1 odd-prime evaluator is not a characteristic-two proof.

\begin{{center}}\begin{{tabular}}{{rrl}}\toprule
m & Base modulus (integer) & Extension coefficients $[c_0,c_1,c_2,c_3]$\\\midrule
{rows}
\bottomrule\end{{tabular}}\end{{center}}
All three separate field-constructor preflights exit zero under 12-GiB
address-space and 900-second limits. They establish representations only.

\textbf{{Limits.}} These instruction totals include loader/startup and the
Valgrind allocator. A/A repeats, five interleaved A/B rounds, the fastest
external field-constructor comparison and full-pipeline pricing are missing.
No native runtime, throughput, full-IC speedup or crossover is claimed.
The recorded profiler wall seconds are diagnostic and are not plotted.

\begin{{flushleft}}\footnotesize
Raw profiles:\par
\path{{../../profiles/wide-constructor-before-20261025/}}\par
\path{{../../profiles/wide-constructor-after-20261025/}}\par
Native audit:\par
\path{{../../pipeline-receipts/wide-core-audit-20261025/constructor-profile-audit.json}}\par
Protocols: WIDE_CONSTRUCTOR_PROFILE_PROTOCOL.md; WIDE_FIELD_CONSTRUCTOR_PROTOCOL.md;
WIDE_GROUP_AUDIT_PROTOCOL.md. Additional cloud VM allocation: none.
\end{{flushleft}}
""")

folder = study / "reports/run-wide-core-audit-20261025"
data = []
for m in (16, 17, 18):
    row = json.loads((receipt / f"m{m}/output.json").read_text())
    assert row["status"] == "verified"
    assert json.loads((receipt / f"m{m}/status.json").read_text())["exit_code"] == 0
    replay = json.loads((receipt / f"replay/m{m}.status.json").read_text())
    assert replay == {"exit_code": 0, "byte_compare_exit": 0}
    data.append(row)
fig, ax = plt.subplots(figsize=(8.1, 2.5))
xs = list(range(3))
ax.bar([x-.18 for x in xs], [r["independent_field_products"] for r in data], width=.36, label="independent field products", color="#167d8d")
ax.bar([x+.18 for x in xs], [r["independent_scalar_controls"] for r in data], width=.36, label="independent scalar controls", color="#ac6146")
ax.set_xticks(xs, [f"m={r['m']} / n={r['n']}" for r in data])
ax.set_ylabel("checks per exact selected curve")
ax.set_ylim(0, 80)
ax.legend(frameon=False, fontsize=9, ncol=2)
ax.spines[["top", "right"]].set_visible(False)
fig.tight_layout()
save(fig, folder, "wide-controls")
rows = "\n".join(f"{r['m']} & {r['instance_seed']} & {r['group_order_decimal']} & {r['r_decimal']} & {r['cofactor_decimal']}" + r" \\" for r in data)
names = "\n".join(r"\path{" + r["curve_name"] + r"}\par" for r in data)
render(folder, "Wide quartic core: order, arithmetic and encoding", rf"""
Native correctness audit, 2026-10-25. Source c41a03785; frozen binary SHA
prefix f5180b12e9fc. Inputs m=16,17,18, first qualifying instance from seed 1,
cofactor cap 16; source-pinned internal search, not a priced candidate census.
Each rung and independent replay has 12-GiB address-space and 900-second caps.
All six processes exit zero; the three replay outputs are byte-identical.

\begin{{center}}\includegraphics[width=\linewidth]{{wide-controls.pdf}}\end{{center}}
\begin{{center}}\small\begin{{tabular}}{{rrrrr}}\toprule
m & Seed & Full group order & Prime r & h\\\midrule
{rows}
\bottomrule\end{{tabular}}\end{{center}}
Each r is independently checked using BigUint modular exponentiation at the
first twelve prime Miller--Rabin bases, within the explicit deterministic
bound below $2^{{73}}$. A nonidentity on-curve G satisfies [r]G=O, independently
in polynomial-basis BinaryCurve arithmetic. Its unique multiple of r in the
exact Hasse interval certifies the full group order. Exact tower and
polynomial-basis coefficients, modulus and G are in each native JSON.

Addition, negation and six scalar controls agree with independent arithmetic;
scalars include $2^{{65}}+3$ and $2^{{70}}+2^{{64}}+123$. The first test point
has its highest field-coordinate bit set. Packing, decoding, full-width
decimal/hex strings and a planted four-point S5 relation are checked.
Old-rung field/point/scalar/native-count parity and exhaustive base-field
inversion/Artin--Schreier controls at m=16--18 pass in the library suite.
The full release library gate reports 4,634 passed, zero failed, 119 ignored;
the src/ tree remains identical to the frozen control revision c46089aed.

\textbf{{Limits.}} This is core correctness at L0 on the shared AMD EPYC
9V74 Linux x86-64 host. The legacy u64 CountedGroup/ecbench stage adapters
remain unintegrated. Full factor bases, oracle/relation collection, linear
algebra, complete DLP, matched rho and native physical prices at these sizes
are unmeasured. The original unsupported wider ecbench specs remain refusals.
No crossover or conclusion for prime-degree/prime fields follows.

\begin{{flushleft}}\footnotesize
Exact unregistered research-model ICV1 names:\par
{names}
Receipts: \path{{../../pipeline-receipts/wide-core-audit-20261025/}}\par
Full-suite metadata: \path{{../../pipeline-receipts/wide-core-library-gate-20261025/}}\par
Protocols: WIDE_CORE_PROTOCOL.md and WIDE_GROUP_AUDIT_PROTOCOL.md.
No additional paid VM was launched.
\end{{flushleft}}
""")
print("Rendered two native wide-core/profile reports; inspect both PDFs before registration.")
