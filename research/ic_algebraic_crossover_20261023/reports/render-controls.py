"""Lay out recorded native receipts only; no experiments or result computation."""
from pathlib import Path
import json
import subprocess
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

ROOT = Path(__file__).resolve().parent.parent
DATA = json.loads((ROOT / "report-data.json").read_text())
plt.rcParams.update({"font.size": 10, "svg.fonttype": "none"})
PREAMBLE = r"""\documentclass[10pt]{article}
\usepackage[margin=18mm]{geometry}
\usepackage{graphicx,booktabs,hyperref}
\usepackage[strings]{underscore}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
"""

def diagram(folder, name, source):
    folder.mkdir(parents=True, exist_ok=True)
    source = 'digraph G { graph [rankdir=LR,bgcolor="white"]; node [shape=box,style="rounded,filled",fillcolor="#e7f1f3",fontname="Helvetica",fontsize=12]; edge [color="#526a73"];\n' + source + "\n}\n"
    (folder / (name + ".dot")).write_text(source)
    for suffix in ("svg", "pdf", "png"):
        subprocess.run(["dot", "-T" + suffix, "-Gdpi=150", str(folder / (name + ".dot")),
                        "-o", str(folder / (name + "." + suffix))], check=True)

def pdf(folder, title, body):
    folder.mkdir(parents=True, exist_ok=True)
    (folder / "report.tex").write_text(PREAMBLE + r"\section*{" + title + "}\n" + body + "\n\\end{document}\n")
    with (folder / "render.log").open("w") as log:
        subprocess.run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "report.tex"], cwd=folder,
                        stdout=log, stderr=subprocess.STDOUT, check=True)

folder = ROOT / "reports/run-controls-and-refusals-20261023"
diagram(folder, "control-paths", r'''
  inputs [label="Public known-answer inputs\noriginal and small fields"];
  exact [label="Independent exact count\n0,6,6,0,18,0,0"];
  small [label="Small exhaustive controls\n2,304 assignments\n373 symmetric roots\n2,880 signed lifts"];
  replay [label="Original A/A\n12 deterministic rows equal"];
  inputs -> exact; inputs -> small; inputs -> replay;
''')
body = r"""Cohorts: selftest-20261023-01/02/03, fixture-replay-20261023-01,
aa-original-20261023-01 and query-audit[-d2/-d3]-20261023-01.
These are correctness and replay controls, not independent performance trials.

The final selftest passes for n=5,7, ell=1,2 and all four basis families.
All seven original fixture counts agree independently, including checked
witnesses on all three SAT draws. Original n19/ell4/geometric/rep0 A/A
checks all 12 rows after excluding only diagnostic seconds: every other
field equals the original panel. Independent direct group enumeration checks
every completed extraction in each of the three structural panels, with
zero failures. Partial extractions stay partial.

\begin{center}\includegraphics[width=.97\linewidth]{control-paths.pdf}\end{center}

The first selftest attempt exited 127 because GNU time was absent; the first
structural orchestration failed at jq argument parsing before a native
measurement. Their empty output is not a zero-cost result. The final
degree-four structural shell had a separate syntax error after all 96
native commands succeeded, caused by editing its active runner on disk.
Later panels use preserved frozen runner copies. These setup failures and
the raw shell-exit receipt remain in the archive.

Native source: \path{examples/ic_algebraic_followup.rs}; each run pins its
own source/binary hashes, and the later instrument build is d67f6085.
Local limits are 12 GiB and
900 seconds per process on the shared x86-64 host, L0. Timings are diagnostic.
Native report recomputation: \path{examples/ic_campaign_report_data.rs};
values and raw SHA-256 pointers in \path{../../report-data.json}.

\newpage\section*{Guess caps, width refusals and certificate}
Balanced k=0/1/2 attempts ran every branch across the four original UNSAT
draws, 4/8/16 processes respectively. All 28 return column-cap, not a
degree verdict. Eight initial k=3 draw-0 processes returned engine-cap
under the library's default 40,000-column cap; they are unscored. Raising
only the internal cap to the already declared instrument limit permits the
separate complete eight-branch run, whose report is a sibling.

Native ecbench plan at m=16,17,18 returns exit 2, explicitly outside the
supported quartic base-field range 4--15. Coefficients, sentinels, packed
coordinates and scalars need a wider implementation; this is not a
mathematical exclusion. No quartic solve or cost follows from a refusal.

The summand-exchange check, certificate IDC1ha27a77e72057f9cb, independently
normalizes both S3 integer polynomials (six terms, maximum coefficient two),
checks the standard finite-field certificate, and checks the bit permutation
on all 308 equations of the seven original inputs. The two generated JSON
artifacts reproduce byte for byte in a later process. Its formal-proof
pointer is unset. It relates degree verdicts for [7,23,24] and [15,23,24],
not their measured costs or an untested minimum.

Raw records: the \path{runs/} tree in
\path{../../evidence/runs-local-20261023.tar.xz}, including commands,
source revisions, stderr, exit codes, host and memory receipts. The native
inventory lists all 33 run directories and hashes their raw rows. Refusal
sidecars are included separately. The frozen protocol and its amendments
identify the setup and degree-floor deviations.

Certificate source and artifacts: \path{examples/ic_guess_symmetry.rs},
\path{../../proofs/s3-swap.identity.json}, and
\path{../../proofs/guess-swap.receipt.json}.
No comparable timing samples are admitted, so the visual is a control-path
diagram. No fastest-CAS, GPU, asymptotic or security claim follows.
"""
pdf(folder, "Correctness controls and retained refusals", body)

folder = ROOT / "reports/run-fresh-fixtures-20261023"
folder.mkdir(parents=True, exist_ok=True)
rows = DATA["fresh_fixture_outcomes"]
fig, axes = plt.subplots(3, 3, figsize=(8, 5.5), sharey=True)
cells = sorted({(r["m"], r["n"], r["ell"]) for r in rows})
families = ["prefix", "geometric", "geomtraceu", "random"]
for ax, cell in zip(axes.flat, cells):
    ordered = [next(r for r in rows if (r["m"], r["n"], r["ell"]) == cell and r["family"] == family)
               for family in families]
    ax.bar(range(4), [r["sat"] for r in ordered], label="positive exact chain count", color="#147d92")
    ax.bar(range(4), [r["unsat"] for r in ordered], bottom=[r["sat"] for r in ordered],
           label="zero exact chain count", color="#d7a258")
    ax.set_xticks(range(4), ["P", "G", "T", "R"], fontsize=8)
    ax.set_yticks([0, 16, 32], ["0", "16", "32"], fontsize=8)
    ax.set_ylim(0, 34)
    ax.set_title(f"m={cell[0]}, n={cell[1]}, ell={cell[2]}", fontsize=9)
fig.supylabel("recorded public draws per family", fontsize=10)
handles, labels = axes.flat[0].get_legend_handles_labels()
fig.legend(handles, labels, frameon=False, loc="upper center", ncol=2, fontsize=9)
fig.tight_layout(rect=(.02, 0, 1, .95))
for suffix in ("svg", "pdf", "png"):
    fig.savefig(folder / ("fixture-outcomes." + suffix), bbox_inches="tight", dpi=150)
    if suffix == "svg":
        path = folder / "fixture-outcomes.svg"
        path.write_text("\n".join(line.rstrip() for line in path.read_text().splitlines()) + "\n")
plt.close(fig)
pdf(folder, "Fresh fixture generation: 36 frozen cells", r"""
Run fixtures-20261023-01: 36 native commands, 32 public draws each, all
exited zero. There are 1,152 frozen draws in nine parameter cells, each with
prefix, geometric, trace-zero geometric and random bases. Original seed is
20261023; no draws are dropped when a class is absent.

\begin{center}\includegraphics[width=\linewidth]{fixture-outcomes.pdf}\end{center}

P=prefix, G=geometric, T=trace-zero geometric, R=random. Values are native
recorded counts, not sampled degree measurements or performance comparisons.

The exact algebraic chain counter uses a quadratic Artin--Schreier solve and
meet-in-the-middle at m=3,4, forward dynamic programming for $m>4$. Small-field
controls agree with exhaustive roots and the legacy chain counter. Counts
are not point decompositions or Gröbner results. The larger positive-count
fixtures have not all received independent Boolean-witness replay; their
closure/guess experiments remain open.

The first four zero-count and first three positive-count draws are frozen
per cell/family in inputs/fresh/selection.json; missing classes remain
explicit. Parameter cells include the actual upstream N=58 and N=60
continuations and an N=100 cell. Their generators do not certify a low
solving degree. Limits were 12 GiB and 900 s per native command; L0.

Source: examples/ic_algebraic_followup.rs and exact.rs, built at d67f6085.
Raw rows, commands, input/binary/source hashes and host receipts are in
the fixtures-20261023-01 directory in the evidence archive. The committed
input JSONL files retain field modulus, seed, basis, target, count and
oracle name; their SHA-256 list is inputs/fresh/SHA256SUMS. Native histogram
recomputation and visual values are in report-data.json. No timing or
scaling conclusion follows from this generation run.
""")

folder = ROOT / "reports/run-structured-n7-support-20261025"
audit = json.loads((ROOT / "pipeline-receipts/structured-n7-support.json").read_text())
diagram(folder, "finite-support", r'''
  fb [label="Seeded n=7 factor base\n4 signed points"];
  triples [label="All 64 ordered triples\nincluding repeats"];
  noic [label="0 sums in prime subgroup\nno 3-point relation support",fillcolor="#f7e6d1"];
  group [label="All 29 subgroup points"];
  canon [label="3 canonical points\nincluding infinity"];
  nodp [label="0 pass dp_bits=4\nno stored collisions",fillcolor="#f7e6d1"];
  fb -> triples -> noic; group -> canon -> nodp;
''')
pdf(folder, "Exact support diagnosis of the n=7 smoke", r"""
Post-outcome correctness diagnosis, 2026-10-25, protocol amendment 5.
The failed frozen session is ECBS1h1572078f47bd; it exhausted all ten
records including both rho controls. This diagnosis changes none of them.

The K0 model has field degree 7, group order 116, prime subgroup r=29 and
cofactor 4: icv1-f2m7-t13-616700dd. Its geometric dimension-2 base, seed 17,
has four signed points. Exhaustive enumeration of all 64 ordered triples
including repeated points finds zero sums in the prime subgroup. Hence
none of these three-summand arms can produce a prime-subgroup relation on
this exact base; this is a finite support fact, not a failure of F4.
Exact identity: \path{ICV1:f2m-7-e1a6f45a:13:116:0x1:-7:unk:r:616700ddc6fe};
subgroup generator (0x26,0x5).

\begin{center}\includegraphics[width=.98\linewidth]{finite-support.pdf}\end{center}

Enumerating every scalar 0--28 gives three signed-Frobenius canonical
representatives including infinity. The low hexadecimal digits of their
source-pinned distinguished hashes are 2,3,7. None passes the frozen
four-bit distinguished predicate. No canonical point can enter its rho
collision table under this configuration. This explains the independent
reference failure without treating it as zero rho cost.

Native source: examples/ic_structured_small_audit.rs. Complete coordinates,
r-multiples, canonical points and hashes, plus source SHA-256 pins, are in
pipeline-receipts/structured-n7-support.json. It reuses actual base construction
and public strong-rho canonicalization, and implements the literal private
hash from the pinned source. Re-run the native example to check the finite
enumeration. No timings are reported; the diagram shows separate failure paths
because the two counts have different domains.

This does not establish failure on any other basis or field, retune the
original reference, qualify a performance comparison, or prove a generic
obstruction. A separately frozen viable tiny fixture is needed for that role.
""")

print("Rendered controls/refusals, fresh fixtures and exact n7 support reports.")
