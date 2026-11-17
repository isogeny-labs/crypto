#!/usr/bin/env python3
"""Render neutral source-parity SVG and TeX from standardized evidence."""
import argparse
import csv
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent

def render():
    rows=list(csv.DictReader((HERE/'ROWS.csv').open(newline='')))
    summary=json.loads((HERE/'SUMMARY.json').read_text())
    assert len(rows)==4
    assert sum(int(r['rows']) for r in rows)==34704
    assert sum(int(r['author_match']) for r in rows)==summary['counts']['author_match']
    assert sum(int(r['invalid_weight']) for r in rows)==summary['counts']['invalid_weight']
    assert sum(int(r['undefined_division']) for r in rows)==summary['counts']['undefined_division']
    lines=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="440" viewBox="0 0 900 440" role="img" aria-label="F7 and F11 public author to Rust parity counts">',
           '<rect width="900" height="440" fill="#fff"/>',
           '<text x="40" y="39" font-family="sans-serif" font-size="23" fill="#182638">Public weighted-law finite source parity</text>',
           '<text x="40" y="67" font-family="sans-serif" font-size="14" fill="#526171">Valid ordered pairs, two weight assignments; author source evaluated, Magma not run</text>',
           '<line x1="255" y1="100" x2="255" y2="351" stroke="#8b99a6"/>']
    for i,row in enumerate(rows):
        p,w=int(row['p']),int(row['weight_index'])
        n,good=int(row['rows']),int(row['author_match'])
        bad=n-good
        y=122+i*62
        width=round(455*n/12996)
        lines.extend([
            f'<text x="40" y="{y+18}" font-family="sans-serif" font-size="16" fill="#182638">F{p}, weight {w}</text>',
            f'<rect x="255" y="{y}" width="{width}" height="26" fill="#39749a"/>',
            f'<text x="{265+width}" y="{y+19}" font-family="sans-serif" font-size="15" fill="#182638">{good:,}/{n:,} matched</text>',
            f'<text x="255" y="{y+46}" font-family="sans-serif" font-size="12" fill="#9a5d33">{bad} mismatches; {row["invalid_weight"]} invalid weights; {row["undefined_division"]} undefined divisions</text>'])
    lines.extend(['<text x="40" y="403" font-family="sans-serif" font-size="13" fill="#526171">Bars show counts, not speed. Full per-pair rows and 108 branch-path cells are retained.</text>','</svg>'])
    svg='\n'.join(lines)+'\n'
    table='\n'.join(f'F{r["p"]}, {r["weight_index"]} & {int(r["ordered_pairs"]):,} & {int(r["author_match"]):,} & {r["invalid_weight"]} & {r["undefined_division"]} \\\\' for r in rows)
    tex=r'''\documentclass[11pt]{article}
\usepackage[margin=0.77in]{geometry}
\usepackage[T1]{fontenc}
\begin{document}
\title{Finite public-source parity for a weighted genus-2 law}
\author{Source-bound review packet}
\date{25 October 2026}
\maketitle
\noindent\textbf{Protocol.} The pinned author Magma projective functions
\texttt{ADDDIVISOR} and their 18 dependencies were translated by a strict
syntax-limited parser and evaluated in $\mathbf F_7$ and $\mathbf F_{11}$.
All valid reduced divisors of $y^2=x^5+x+2$ were enumerated independently
of the compiled Rust enumerator. Every ordered pair was checked with unit
weights and with $(z_1,w_1,z_2,w_2)=(2,3,3,5)$.
The author result was normalized and compared with compiled Rust; Rust was
compared with independent Cantor arithmetic. A 1,024-row bounded gate
preceded the complete pass.

\begin{center}
\begin{tabular}{lrrrr}
Field, weight & Pairs & Author/Rust matches & Invalid weights & Undefined divisions\\\hline
@@TABLE@@
\end{tabular}
\end{center}

\noindent\textbf{Observed.} There are 17,352 ordered pairs and 34,704
weighted checks. All 34,704 author outputs matched Rust, and all 34,704
Rust outputs matched Cantor. There were zero invalid author output weights,
zero undefined field divisions, and zero other invalid intermediates.
The branch matrix has 108 joint path/label cells, covering 28 observed
author call paths and 16 Rust labels. These are finite counts, not timing.

\noindent\textbf{Evidence.} \texttt{ROWS.csv} carries the four aggregate rows;
\texttt{branch-matrix.csv} gives the joint paths and labels;
\texttt{F7-weight*.csv.xz} and \texttt{F11-weight*.csv.xz} contain every
standardized pair row. \texttt{SUMMARY.json} contains full raw digests,
source hashes, and counts. \texttt{run.py} regenerates the data and checks
it byte for byte. The author source SHA-256 is
\texttt{@@AUTHOR@@}; the Rust source SHA-256 is
\texttt{@@RUST@@}.

\noindent\textbf{Limit.} Magma was unavailable, so the pinned Magma source
was translated and evaluated, not run by Magma. The parser checks its exact
source bytes and supported syntax, but this is not an independent Magma
execution. One curve over each of two small fields cannot establish
universal formula correctness, full Rust semantics, or the Picard-group
bridge. The frozen 1,260-target costs are unchanged.
\end{document}
'''.replace('@@TABLE@@',table).replace('@@AUTHOR@@',summary['author_sha256']).replace('@@RUST@@',summary['rust_sha256'])
    return {'GRAPH.svg':svg.encode(),'REPORT.tex':tex.encode()}

def main():
    parser=argparse.ArgumentParser(); parser.add_argument('--check',action='store_true')
    args=parser.parse_args()
    for name,data in render().items():
        path=HERE/name
        if args.check: assert path.read_bytes()==data,name
        else: path.write_bytes(data)
    print('render inputs match' if args.check else 'rendered SVG and TeX')

if __name__=='__main__': main()
