#!/usr/bin/env python3
"""Build sections/openprs.tex from data/open_prs.tsv (a snapshot of open pull
requests whose subject is ECDLP index calculus or its baselines).

    gh pr list --repo aburan28/crypto --state open --limit 400 \
      --json number,title,headRefName,updatedAt,headRefOid  # then filter
    python3 tools/open_prs.py
"""
import csv
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from catalogue import tex, unknown  # noqa: E402

HERE = Path(__file__).resolve().parent.parent
REPO = "https://github.com/aburan28/crypto/pull/"


def main():
    with open(HERE / "data" / "open_prs.tsv", newline="") as f:
        rows = list(csv.DictReader(f, delimiter="\t"))
    L = [r"\section{Work in open pull requests}\label{sec:openprs}", "",
         r"At the paper's date, %d open pull requests in the repository concern "
         r"elliptic-curve index calculus or its baselines. Their results are "
         r"not on \texttt{main}, have not been reviewed here, and are not used "
         r"anywhere in the paper; the titles below are quoted as their authors "
         r"filed them and are claims, not findings. Two bear on the text: "
         r"\#2359 and \#2597 report plane-quartic IC/rho ratios beyond the "
         r"plain-rho comparison on \texttt{main} (\S\ref{sec:transfer}), and "
         r"\#2598 runs Smith's isogeny on divisor classes; and \#2324 records that a Joux--Vitse "
         r"sieve replication failed its acceptance rule and withheld its "
         r"decisions. As these merge, they are folded into the sections above "
         r"and the catalogue under the same rules as everything else. Source: "
         r"\art{papers/ecdlp-index-calculus-program/data/open_prs.tsv}." % len(rows),
         "", r"{\footnotesize\setlength{\tabcolsep}{3pt}",
         r"\begin{longtable}{@{}r l >{\raggedright\arraybackslash}p{0.78\linewidth}@{}}",
         r"\toprule PR & Updated & Title as filed\\ \midrule \endhead",
         r"\bottomrule \endfoot"]
    for r in rows:
        L.append(r"\href{%s%s}{\#%s} & %s & %s\\" % (
            REPO, r["number"], r["number"], r["updated"], tex(r["title"])))
    L += [r"\end{longtable}}", ""]
    (HERE / "sections" / "openprs.tex").write_text("\n".join(L) + "\n")
    if unknown:
        print("unmapped characters: %s" % " ".join(sorted(unknown)),
              file=sys.stderr)
    print("wrote openprs.tex (%d PRs)" % len(rows), file=sys.stderr)


if __name__ == "__main__":
    main()
