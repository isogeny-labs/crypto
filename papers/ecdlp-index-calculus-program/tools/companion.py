#!/usr/bin/env python3
"""Build the companion-repository appendix (sections/companion_catalogue.tex)
from data/companion.tsv, and check per-repository coverage.

    python3 tools/companion.py                       # write the appendix
    python3 tools/companion.py --check REPO UNITS    # units of REPO with no row

data/companion.tsv has the columns of data/catalogue.tsv with a leading
`repo` column; paths are relative to that repository at the commit pinned in
data/README.md. Rows a reader wrote take precedence over scripted rows for
the same (repo, path). Spec-only experiments, evidence records and
corrections that no reader summarised are listed by name under one grouped
row per repository and kind, so every unit appears without a row of its own.
"""
import csv
import re
import sys
from collections import OrderedDict, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from catalogue import FAMILIES, tex, status_cell, unknown  # noqa: E402

HERE = Path(__file__).resolve().parent.parent
TSV = HERE / "data" / "companion.tsv"
OUT = HERE / "sections" / "companion_catalogue.tex"

REPOS = OrderedDict([
    ("crypto-autoresearcher", ("ab0609ebee", "public")),
    ("cryptanalysis", ("c5aa7b39bf", "private")),
    ("ecc-challenges", ("86f8a57a5e", "public")),
    ("ml-cryptanalysis", ("58494969fd", "private")),
    ("cryptanalysis-rl", ("24d90557b2", "private")),
    ("ecbench", ("0657f29b3d", "public")),
    ("isogeny-labs", ("d613806737", "public")),
    ("isolab", ("eb3f285cc1", "private")),
    ("ecdlp-prime-field-smallness", ("4f20be9b36", "private")),
    ("crypto-lean", ("148f5dc389", "public")),
])
FAM = OrderedDict(list(FAMILIES.items()) + [("unclassified", "Not yet classified")])


def load():
    with open(TSV, newline="") as f:
        return [{k: (v or "").strip() for k, v in r.items()}
                for r in csv.DictReader(f, delimiter="\t")]


def name(r):
    return r["path"].rstrip("/")


def grouped_kind(r):
    """Scripted rows that are listed by name rather than given a row."""
    if r["status"] == "not run":
        return "experiment specifications that have no run"
    if r["class"].startswith("evidence"):
        return "ledger evidence records"
    if r["class"] == "correction":
        return "ledger correction records"
    return None


def render(rows):
    by_repo = defaultdict(list)
    for r in rows:
        by_repo[r["repo"]].append(r)
    L = [r"\section{Catalogue of the companion repositories}\label{sec:companion-catalogue}", "",
         r"This appendix extends Appendix~\ref{sec:catalogue} to the laboratory's "
         r"other repositories, at the commits listed in Table~\ref{tab:repos}. "
         r"Paths are relative to each repository. Rows were compiled by the same "
         r"procedure as the main catalogue; in crypto-autoresearcher, rows for "
         r"experiment specifications, evidence records and corrections were "
         r"extracted from the records themselves, and its claim tiers are quoted "
         r"as the repository states them. An approved specification is not a "
         r"result: specifications with no run are listed by name only. Records "
         r"that are process rather than results (hypotheses, decisions, "
         r"proposals, hand-offs, goals, questions) and the literature records are "
         r"counted. Studies that are the same work as one in the main repository "
         r"say so in their topic.", ""]
    for repo, (sha, vis) in REPOS.items():
        rs = by_repo.get(repo, [])
        if not rs:
            continue
        clusters = {r["path"][len("CLUSTER:"):]: r for r in rs
                    if r["path"].startswith("CLUSTER:")}
        members = defaultdict(list)
        groups = defaultdict(list)
        singles = defaultdict(list)
        oos = []
        for r in rs:
            if r["path"].startswith("CLUSTER:"):
                continue
            if r["cluster"] not in ("", "-") and r["cluster"] in clusters:
                members[r["cluster"]].append(r)
            elif grouped_kind(r):
                groups[grouped_kind(r)].append(r)
            elif r["family"] == "out-of-scope":
                oos.append(r)
            else:
                singles[r["family"] or "unclassified"].append(r)
        n = sum(1 for r in rs if not r["path"].startswith("CLUSTER:"))
        L.append(r"\subsection{%s (%s, commit \texttt{%s})}" % (tex(repo), vis, sha))
        L.append(r"%d entries." % n)
        by_fam = defaultdict(list)
        for cname, c in clusters.items():
            by_fam[c["family"] or "unclassified"].append((cname, c))
        for fam, title in FAM.items():
            cl = sorted(by_fam.get(fam, []), key=lambda x: x[1]["date"])
            si = sorted(singles.get(fam, []), key=lambda r: (r["date"], r["path"]))
            if not cl and not si:
                continue
            L.append(r"\paragraph{%s.}" % title)
            L.append(r"{\footnotesize\setlength{\tabcolsep}{3pt}")
            L.append(r"\begin{longtable}{@{}>{\raggedright}p{0.25\linewidth}"
                     r">{\raggedright}p{0.10\linewidth}"
                     r">{\raggedright}p{0.44\linewidth}"
                     r">{\raggedright\arraybackslash}p{0.17\linewidth}@{}}")
            L.append(r"\toprule Study & Class & Result & Status\\ \midrule \endhead")
            L.append(r"\bottomrule \endfoot")
            for cname, c in cl:
                mem = sorted(members[cname], key=name)
                L.append(r"\textbf{%s} (%d entries, %s) & %s & %s & %s\\*" % (
                    tex(c["topic"] or cname), len(mem), tex(c["date"]),
                    tex(c["class"]), tex(c["headline"]), status_cell(c)))
                L.append(r"\multicolumn{4}{@{}p{0.98\linewidth}@{}}{\scriptsize "
                         r"\hspace*{1em}Members: %s}\\[2pt]" % ", ".join(
                             r"\art{%s}" % name(m) for m in mem))
            for r in si:
                L.append(r"\art{%s}\newline {\scriptsize %s %s} & %s & %s & %s\\[1pt]" % (
                    name(r), tex(r["date"]), tex(r["topic"]), tex(r["class"]),
                    tex(r["headline"]), status_cell(r)))
            L.append(r"\end{longtable}}")
        for kind, gs in sorted(groups.items()):
            L.append(r"\paragraph{%d %s.} {\scriptsize %s.}" % (
                len(gs), tex(kind), ", ".join(
                    r"\art{%s}" % name(g).split("/")[-1] for g in sorted(gs, key=name))))
        if oos:
            L.append(r"\paragraph{Out of scope (%d).} {\scriptsize %s.}" % (
                len(oos), "; ".join(r"\art{%s} (%s)" % (name(r), tex(r["topic"][:60]))
                                     for r in sorted(oos, key=name))))
        L.append("")
    return "\n".join(L) + "\n"


def check(repo, units_file, rows):
    have = {name(r) for r in rows if r["repo"] == repo}
    miss = [u.split("\t")[0].rstrip("/") for u in Path(units_file).read_text().split("\n")
            if u.strip() and u.split("\t")[0].rstrip("/") not in have]
    for m in miss:
        print(m)
    print("%s missing: %d" % (repo, len(miss)), file=sys.stderr)
    return 1 if miss else 0


def main():
    rows = load()
    if len(sys.argv) == 4 and sys.argv[1] == "--check":
        sys.exit(check(sys.argv[2], sys.argv[3], rows))
    OUT.write_text(render(rows))
    if unknown:
        print("unmapped characters: %s" % " ".join(sorted(unknown)), file=sys.stderr)
    print("wrote %s (%d rows)" % (OUT, len(rows)), file=sys.stderr)


if __name__ == "__main__":
    main()
