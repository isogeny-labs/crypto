#!/usr/bin/env python3
"""Build the appendix catalogue (sections/catalogue.tex) from data/catalogue.tsv,
and check that every research/ study directory on main is accounted for.

    python3 tools/catalogue.py            # write sections/catalogue.tex
    python3 tools/catalogue.py --check DIRS.txt   # list dirs missing from the TSV

The TSV has one row per study directory, benchmark row, problem file or note,
plus CLUSTER:<name> summary rows; columns are documented in data/README.md.
"""
import csv
import re
import sys
from collections import OrderedDict, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
TSV = HERE / "data" / "catalogue.tsv"
OUT = HERE / "sections" / "catalogue.tex"

COLS = ["path", "date", "family", "topic", "class", "headline", "label",
        "boundary", "comparator", "status", "correcting_path", "cluster"]

FAMILIES = OrderedDict([
    ("koblitz", "Koblitz curves"),
    ("binary", "Random and prime-degree binary curves"),
    ("prime", "Prime-field curves"),
    ("extension-cover", "Extension fields, covers and transfers"),
    ("rho-harness", "Rho, the harness and validation"),
    ("cross-cutting", "Engines, factor-base tooling and literature"),
])

UNI = {
    "×": r"$\times$", "≤": r"$\le$", "≥": r"$\ge$", "≈": r"$\approx$",
    "→": r"$\to$", "←": r"$\leftarrow$", "−": "-", "–": "--", "—": "---",
    "±": r"$\pm$", "τ": r"$\tau$", "φ": r"$\phi$", "ρ": r"$\rho$",
    "σ": r"$\sigma$", "ℓ": r"$\ell$", "√": r"$\surd$", "∈": r"$\in$",
    "π": r"$\pi$", "λ": r"$\lambda$", "μ": r"$\mu$", "θ": r"$\theta$",
    "δ": r"$\delta$", "κ": r"$\kappa$", "α": r"$\alpha$", "β": r"$\beta$",
    "γ": r"$\gamma$", "ε": r"$\varepsilon$", "ω": r"$\omega$",
    "Δ": r"$\Delta$", "Σ": r"$\Sigma$", "∞": r"$\infty$", "≠": r"$\ne$",
    "·": r"$\cdot$", "…": r"\ldots{}", "’": "'", "‘": "`", "“": "``",
    "”": "''", "′": "'", "″": "''", "²": r"$^2$", "³": r"$^3$",
    "⁴": r"$^4$", "½": "1/2", "¼": "1/4", "¾": "3/4", "₂": r"$_2$",
    "ö": r"\"o", "é": r"\'e", "è": r"\`e", "á": r"\'a", "ü": r"\"u",
    "ä": r"\"a", "ç": r"\c{c}", "Ö": r"\"O", "∑": r"$\Sigma$",
    "⌈": r"$\lceil$", "⌉": r"$\rceil$", "⌊": r"$\lfloor$", "⌋": r"$\rfloor$",
    "∼": r"$\sim$", "≫": r"$\gg$", "≪": r"$\ll$", "⊂": r"$\subset$",
    "∩": r"$\cap$", "∪": r"$\cup$", "°": r"$^\circ$", "✓": "yes", "✗": "no",
    "Φ": r"$\Phi$", "₀": r"$_0$", "₁": r"$_1$", "ψ": r"$\psi$",
    " ": " ", " ": " ", " ": " ",
}
SPECIAL = {"\\": r"\textbackslash{}", "&": r"\&", "%": r"\%", "$": r"\$",
           "#": r"\#", "_": r"\_\allowbreak{}", "/": r"/\allowbreak{}", "{": r"\{", "}": r"\}",
           "~": r"\textasciitilde{}", "^": r"\textasciicircum{}",
           "<": r"\textless{}", ">": r"\textgreater{}", "|": r"\textbar{}"}

unknown = set()


def tex(s):
    out = []
    for ch in s:
        if ch in SPECIAL:
            out.append(SPECIAL[ch])
        elif ch in UNI:
            out.append(UNI[ch])
        elif ord(ch) < 128:
            out.append(ch)
        else:
            unknown.add(ch)
            out.append("?")
    return "".join(out)


def art(path):
    p = path.strip().rstrip("/")
    return r"\art{%s}" % p.replace("{", "").replace("}", "").replace("%", "")


def load():
    rows = []
    with open(TSV, newline="") as f:
        for r in csv.DictReader(f, delimiter="\t"):
            rows.append({c: (r.get(c) or "").strip() for c in COLS})
    return rows


def short(path):
    return re.sub(r"^(research|benchmarks)/", "", path.rstrip("/"))


def status_cell(r):
    st = r["status"] or "current"
    if r["correcting_path"] not in ("", "-"):
        return tex(st) + r"; see " + art(r["correcting_path"])
    return tex(st)


def render(rows):
    clusters = {r["path"][len("CLUSTER:"):]: r for r in rows
                if r["path"].startswith("CLUSTER:")}
    members = defaultdict(list)
    singles = defaultdict(list)
    out_of_scope = []
    for r in rows:
        if r["path"].startswith("CLUSTER:"):
            continue
        if r["family"] == "out-of-scope":
            out_of_scope.append(r)
        elif r["cluster"] not in ("", "-") and r["cluster"] in clusters:
            members[r["cluster"]].append(r)
        else:
            singles[r["family"]].append(r)
    by_fam_clusters = defaultdict(list)
    for name, c in clusters.items():
        by_fam_clusters[c["family"]].append((name, c))

    L = []
    L.append(r"\section{Catalogue of the programme's studies}\label{sec:catalogue}")
    L.append("")
    n_items = sum(1 for r in rows if not r["path"].startswith("CLUSTER:"))
    L.append(
        r"This appendix lists every study directory under \art{research/} on "
        r"\texttt{main} at the paper's date, every benchmark row under "
        r"\art{benchmarks/}, every open-problem record under \art{problems/}, "
        r"and the research notes and \art{docs/ic/} programme documents, "
        r"%d entries in all, grouped by curve family. Campaigns of many "
        r"directories are collapsed to one row, with every member path "
        r"listed under it. Each row gives the study's own headline number "
        r"with its class and status; a number marked superseded or "
        r"corrected must be read through the correcting path. Rows were "
        r"compiled from each study's README and results files and are "
        r"summaries, not re-measurements; where a study states no number, "
        r"none is given. Generated tables (leaderboard rows, the registry) "
        r"are summarised through \art{docs/ic/LEADERBOARD.md} rather than "
        r"listed file by file. The machine-readable source is "
        r"\art{papers/ecdlp-index-calculus-program/data/catalogue.tsv}, "
        r"built by \art{papers/ecdlp-index-calculus-program/tools/catalogue.py}, "
        r"which also checks that no directory is missing." % n_items)
    L.append("")
    for fam, title in FAMILIES.items():
        cl = sorted(by_fam_clusters.get(fam, []), key=lambda x: x[1]["date"])
        si = sorted(singles.get(fam, []), key=lambda r: (r["date"], r["path"]))
        if not cl and not si:
            continue
        nmem = sum(len(members[n]) for n, _ in cl)
        L.append(r"\subsection{%s}" % title)
        L.append(r"%d entries: %d campaigns collapsed from %d entries, and %d "
                 r"single entries." % (nmem + len(si), len(cl), nmem, len(si)))
        L.append(r"{\footnotesize\setlength{\tabcolsep}{3pt}")
        L.append(r"\begin{longtable}{@{}>{\raggedright}p{0.25\linewidth}"
                 r">{\raggedright}p{0.10\linewidth}"
                 r">{\raggedright}p{0.44\linewidth}"
                 r">{\raggedright\arraybackslash}p{0.17\linewidth}@{}}")
        L.append(r"\toprule Study & Class & Result & Status\\ \midrule \endhead")
        L.append(r"\bottomrule \endfoot")
        for name, c in cl:
            mem = sorted(members[name], key=lambda r: r["path"])
            span = c["date"] or ""
            L.append(r"\textbf{%s} (%d entries, %s) & %s & %s & %s\\*" % (
                tex(c["topic"] or name), len(mem), tex(span), tex(c["class"]),
                tex(c["headline"]), status_cell(c)))
            names = ", ".join(r"\art{%s}" % short(m["path"]) for m in mem)
            L.append(r"\multicolumn{4}{@{}p{0.98\linewidth}@{}}{\scriptsize "
                     r"\hspace*{1em}Members: %s}\\[2pt]" % names)
        for r in si:
            L.append(r"%s\newline {\scriptsize %s %s} & %s & %s & %s\\[1pt]" % (
                art(r["path"]), tex(r["date"]), tex(r["topic"]),
                tex(r["class"]), tex(r["headline"]), status_cell(r)))
        L.append(r"\end{longtable}}")
        L.append("")
    if out_of_scope:
        L.append(r"\subsection{Out of scope}")
        L.append(r"These entries are in the repository but are not elliptic-curve "
                 r"index calculus or its baselines: " + "; ".join(
                     r"%s (%s)" % (art(r["path"]), tex(r["topic"]))
                     for r in sorted(out_of_scope, key=lambda r: r["path"])) + ".")
        L.append("")
    return "\n".join(L) + "\n"


def check(dirs_file, rows):
    have = {r["path"].rstrip("/") for r in rows}
    missing = [d for d in Path(dirs_file).read_text().split()
               if d.rstrip("/") not in have]
    for d in missing:
        print(d)
    print("missing: %d" % len(missing), file=sys.stderr)
    return 1 if missing else 0


def main():
    rows = load()
    if len(sys.argv) == 3 and sys.argv[1] == "--check":
        sys.exit(check(sys.argv[2], rows))
    OUT.write_text(render(rows))
    if unknown:
        print("unmapped characters: %s" % " ".join(sorted(unknown)),
              file=sys.stderr)
    print("wrote %s (%d rows)" % (OUT, len(rows)), file=sys.stderr)


if __name__ == "__main__":
    main()
