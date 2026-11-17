"""Presentation only: render the native N12 replay summary and target records."""
import html
import json
import math
from pathlib import Path

ROOT = Path(__file__).parent
RESULTS = ROOT / "results" / "validated"
summary = json.loads((RESULTS / "summary.json").read_text())
targets = [json.loads(s) for s in (RESULTS / "targets.jsonl").read_text().splitlines()]
by_scope = {s["scope"]: s for s in summary["summary"]}
bits = [16, 20, 24, 28, 32, 36, 40]

# The seven columns retain every target; log spacing avoids clipping long tails.
w, h = 960, 530
left, right, top, bottom = 85, 900, 90, 450
levels = [0.5, 1, 2, 4, 8, 16, 32]
def y(r):
    return bottom - (math.log2(r) + 1) / 6 * (bottom - top)
def x(b):
    return left + 45 + bits.index(b) * 112
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">',
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="85" y="29" font-family="sans-serif" font-size="18" font-weight="bold">N12 independent-host replay: candidate / matched rho</text>',
       '<text x="85" y="50" font-family="sans-serif" font-size="12" fill="#555">EKS job n12-heldout-replay-20261025-v3 · 420 targets, 60 per bit size</text>']
for lv in levels:
    yy = y(lv)
    dash = ' stroke-dasharray="5 4"' if lv == 1 else ''
    color = '#555' if lv == 1 else '#d8dde4'
    svg.append(f'<line x1="{left}" y1="{yy:.1f}" x2="{right}" y2="{yy:.1f}" stroke="{color}"{dash}/>')
    svg.append(f'<text x="{left-12}" y="{yy+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="12" fill="#444">{lv:g}</text>')
for b in bits:
    xx = x(b)
    svg.append(f'<text x="{xx}" y="{bottom+24}" text-anchor="middle" font-family="sans-serif" font-size="13">{b}</text>')
    rs = [r for r in targets if r['bits'] == b]
    for r in rs:
        jitter = (int(r['workload_id'][1:], 16) % 41) - 20
        ratio = r['candidate_over_rho']
        cx, cy = xx + jitter, y(ratio)
        title = html.escape(f"{r['workload_id']}: {ratio:.6f}")
        svg.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="3.1" fill="#3467a0" fill-opacity="0.48"><title>{title}</title></circle>')
    aggregate = by_scope[f'bits-{b}']['representative']['candidate_over_rho']
    ay = y(aggregate)
    ax = xx + 29
    svg.append(f'<polygon points="{ax},{ay-6:.1f} {ax+6},{ay:.1f} {ax},{ay+6:.1f} {ax-6},{ay:.1f}" fill="#d35b20" stroke="white" stroke-width="1"><title>ratio of sums: {aggregate:.6f}</title></polygon>')
svg += ['<text x="490" y="502" text-anchor="middle" font-family="sans-serif" font-size="13">Curve bit size</text>',
        '<text x="20" y="255" transform="rotate(-90 20 255)" text-anchor="middle" font-family="sans-serif" font-size="13">Cost ratio (log base 2)</text>',
        '<circle cx="690" cy="68" r="4" fill="#3467a0"/><text x="700" y="72" font-family="sans-serif" font-size="12">individual target</text>',
        '<polygon points="825,62 831,68 825,74 819,68" fill="#d35b20"/><text x="837" y="72" font-family="sans-serif" font-size="12">ratio of sums</text>',
        '</svg>']
(ROOT / "ratio-by-size.svg").write_text('\n'.join(svg) + '\n')

rows = []
for b in bits:
    s = by_scope[f'bits-{b}']
    rows.append(f"{b} & {s['verified']}/60 & {s['representative']['candidate_over_rho']:.6f} & {s['verified_target_wins']} & {s['wins']}/{s['losses']} \\\\")
all_s = by_scope['all']
tex = r'''\documentclass[10pt]{article}
\usepackage[margin=0.8in]{geometry}
\usepackage{graphicx,booktabs}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\title{N12: Independent-host replay of the frozen held-out factor-base evaluation}
\author{Reproducibility packet}
\date{25 October 2026}
\begin{document}
\maketitle
\noindent\textbf{Observed result.} The frozen \texttt{fb1-any-j32} policy and matched source-negation rho each returned 420/420 source-verified scalars on seven held-out models. Every target identity and complete operation-count record matches the first host. The seven raw JSONL files are byte identical. At $S/M=0.5$, $I/M=10$, and logical price $0.1$, the ratio of summed candidate cost to summed rho cost is \textbf{2.479905}. The aggregate grid has 0 winning, 36 losing, and 0 tied cells. Individual target ratios fall below one on 19/420 targets.

\begin{center}
\begin{tabular}{rrrrr}
\toprule
Bits & Verified & Ratio of sums & Target wins & Grid wins/losses \\
\midrule
''' + '\n'.join(rows) + r'''
\midrule
All & 420/420 & 2.479905 & 19 & 0/36 \\
\bottomrule
\end{tabular}
\end{center}

\begin{center}\includegraphics[width=0.96\linewidth]{ratio-by-size.png}\end{center}
\noindent\small Blue points are target-level ratios; orange diamonds are ratios of summed costs by bit size. The vertical scale is logarithmic. The dashed line marks equal cost.\normalsize

\paragraph{Replay boundary.} The original run used EC2 instance \texttt{i-09774109223fde89e}; this replay used EKS node instance \texttt{i-0df0361be2629437d}. Both used the same frozen selector, source workloads, native policy, and price grid. The replay's 11 input members passed a native SHA-256 manifest check before computation. A native validator rebuilt each source target and checked both recovered scalars, phase totals, identity, operation counts, and all 36 aggregate price cells.

\paragraph{Attempt accounting.} Jobs v1 and v2 exited before target execution because the pinned image lacks \texttt{hostname} and \texttt{tar}, respectively. Their exit receipts are retained. Job v3 used per-file S3 transfer and completed 7/7 slices, 60 targets each, with exit code zero. Its 2-CPU, 4-GiB, 3,600-second, no-retry boundary was unchanged.

\paragraph{Cost and limits.} Costs are cold single-target source-addition equivalents, including construction, transfer, collection, elimination, and source verification. The 36-cell grid uses $S/M\in\{0,0.5,1\}$, $I/M\in\{1,10,100,1000\}$, and logical price $\{0,0.1,1\}$. Wall-time receipts are diagnostic and are not substituted for operation costs. This packet tests the frozen seven-model, 420-target cohort; it does not estimate a population confidence interval.
\end{document}
'''
(ROOT / "REPORT.tex").write_text(tex)
