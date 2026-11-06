#!/usr/bin/env python3
"""Presentation only: render the native validator's wall result or a documented null."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"
VALID = RESULTS / "validated" / "validation.json"
report = json.loads(VALID.read_text()) if VALID.exists() else None
ratios = report.get("round_candidate_over_rho", []) if report else []
full = bool(report and report.get("full_schedule_valid"))
observed = (report.get("valid_observations", 0) + report.get("invalid_observations", 0)) if report else 0
aa = report.get("aa_ratio") if full else None
if not full:
    ratios = []

# Fixed-scale plot of the five predeclared paired rounds; absent rounds stay absent.
lo = min([0.75, *ratios]) if ratios else 0.75
hi = max([1.25, *ratios]) if ratios else 1.25
pad = (hi-lo)*0.12
lo -= pad
hi += pad
left, right, top, bottom = 92, 950, 66, 366
ys = lambda y: bottom-(y-lo)/(hi-lo)*(bottom-top)
xs = lambda i: left+(right-left)*(i-1)/4
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="440" viewBox="0 0 1040 440">',
       '<rect width="1040" height="440" fill="white"/>',
       '<text x="520" y="32" text-anchor="middle" font-family="sans-serif" font-size="20">Frozen N12 paired wall cost: candidate / rho</text>',
       f'<line x1="{left}" y1="{top}" x2="{left}" y2="{bottom}" stroke="#333"/>',
       f'<line x1="{left}" y1="{bottom}" x2="{right}" y2="{bottom}" stroke="#333"/>',
       f'<line x1="{left}" y1="{ys(1):.1f}" x2="{right}" y2="{ys(1):.1f}" stroke="#777" stroke-dasharray="7,5"/>',
       f'<text x="{right+12}" y="{ys(1)+5:.1f}" font-family="sans-serif" font-size="13">1.00</text>']
for i in range(1,6):
    svg.append(f'<text x="{xs(i):.1f}" y="{bottom+24}" text-anchor="middle" font-family="sans-serif" font-size="13">round {i}</text>')
for y in [lo,(lo+hi)/2,hi]:
    svg.append(f'<text x="{left-12}" y="{ys(y)+4:.1f}" text-anchor="end" font-family="sans-serif" font-size="12">{y:.2f}</text>')
if ratios:
    points=' '.join(f'{xs(i):.1f},{ys(v):.1f}' for i,v in enumerate(ratios,1))
    svg.append(f'<polyline points="{points}" fill="none" stroke="#164e91" stroke-width="3"/>')
    for i,v in enumerate(ratios,1):
        svg.append(f'<circle cx="{xs(i):.1f}" cy="{ys(v):.1f}" r="5" fill="#164e91"><title>round {i}: {v:.9f}</title></circle>')
        svg.append(f'<text x="{xs(i):.1f}" y="{ys(v)-12:.1f}" text-anchor="middle" font-family="sans-serif" font-size="12">{v:.3f}</text>')
else:
    svg.append(f'<text x="520" y="217" text-anchor="middle" font-family="sans-serif" font-size="17" fill="#555">{observed:,} / 5,040 timed target-arm rows; wall ratio is null</text>')
svg.append(f'<text x="520" y="420" text-anchor="middle" font-family="sans-serif" font-size="13">A/A rho ratio: {aa:.6f}' if aa is not None else '<text x="520" y="420" text-anchor="middle" font-family="sans-serif" font-size="13">A/A rho ratio: null')
svg.append('</text></svg>')
(ROOT / 'wall-ratios.svg').write_text('\n'.join(svg)+'\n')

status = ('Full 12-process schedule valid' if full else 'No valid isolated 12-process schedule')
median = report.get('median_candidate_over_rho') if full else None
minimum = report.get('minimum_candidate_over_rho') if full else None
fmt = lambda v: f'{v:.9f}' if v is not None else 'null'
gate_note = ('The complete schedule passed the recorded isolation and row checks.' if full else
             'At the 09:03 UTC start gate the benchmark lock remained occupied, so the wall ratio is null.')
md = f'''# N12 frozen held-out paired wall comparison

**Observed status:** {status}; {observed:,}/5,040 timed target-arm rows. The A/A ratio is {fmt(aa)}. The five candidate/rho round ratios are {', '.join(fmt(x) for x in ratios) if ratios else 'null'}. Their median is {fmt(median)} and minimum is {fmt(minimum)}. These are wall-process ratios only. The prior independent-host operation-count ratio (2.4799052224835276 at its representative price) is a separate result and is not pooled here.

The fixed 420-target list and `fb1-any-j32` selector are identified in [PROTOCOL.md](PROTOCOL.md). The protocol requires two rho A/A processes, five alternating candidate/rho pairs, source-scalar verification in each target row, matching counters against the frozen source, reserved CPUs 0 and 16, an uncontended isolation receipt, and I/O pressure at most 5% at both boundaries. [wall-ratios.svg](wall-ratios.svg) shows all valid round ratios or the null status. [DATA_DICTIONARY.md](DATA_DICTIONARY.md) defines each field and null.

The source and frozen selector were not retuned. The result is limited to this host and cohort. No wall ratio is inferred from incomplete or contended observations. {gate_note} [OBSTRUCTION.md](OBSTRUCTION.md) and [the problem record](../../problems/n12-heldout-wall-isolation.md) give the exact observed condition and exit criteria.
'''
(ROOT/'REPORT.md').write_text(md)

def tex_escape(s):
    return s.replace('\\','\\textbackslash{}').replace('_','\\_').replace('%','\\%').replace('&','\\&')
rows='\n'.join(f'{i} & {fmt(v)} \\\\' for i,v in enumerate(ratios,1)) if ratios else '1--5 & null \\\\'
coords=' '.join(f'({i},{v:.9f})' for i,v in enumerate(ratios,1))
plot = f'\\addplot+[mark=*,thick] coordinates {{{coords}}};' if ratios else '\\node at (axis cs:3,1) {No valid isolated paired schedule};'
tex=r'''\\documentclass[10pt]{article}
\\usepackage[margin=0.8in]{geometry}
\\usepackage{booktabs,pgfplots}
\\pgfplotsset{compat=1.18}
\\title{N12: Frozen held-out paired wall comparison}
\\author{Reproducibility packet}
\\date{25 October 2026}
\\begin{document}
\\maketitle
\\noindent\\textbf{Observed status.} STATUS; OBSERVED/5,040 timed target-arm rows. The A/A rho-process ratio is AA. Candidate/rho round median is MEDIAN and minimum is MINIMUM. No ratio is emitted unless all 5,040 target-arm rows and the isolation receipt pass. The independent-host operation-count ratio 2.4799052224835276 is separate.
\\begin{center}
\\begin{tabular}{rr}\\toprule Paired round & candidate/rho process wall ratio \\\\ \\midrule
ROWS
\\bottomrule\\end{tabular}
\\end{center}
\\begin{center}
\\begin{tikzpicture}
\\begin{axis}[width=0.93\\linewidth,height=6.1cm,xmin=0.8,xmax=5.2,ymin=LO,ymax=HI,xlabel={paired round},ylabel={candidate / rho},xtick={1,2,3,4,5},grid=major]
\\addplot[dashed,gray] coordinates {(1,1)(5,1)};
PLOT
\\end{axis}
\\end{tikzpicture}
\\end{center}
\\noindent The frozen selector and 420 target IDs, source SHA-256 hashes, host manifest, schedule, and null rules are in the adjacent protocol and data dictionary. GATE_NOTE The adjacent obstruction note gives the exact lock and pressure observations. This PDF reports observed wall data only; it does not pool wall time with operation-count prices.
\\end{document}
'''
tex=tex.replace('\\\\','\\')
tex=tex.replace('STATUS',tex_escape(status)).replace('OBSERVED',f'{observed:,}').replace('GATE_NOTE',tex_escape(gate_note)).replace('AA',fmt(aa)).replace('MEDIAN',fmt(median)).replace('MINIMUM',fmt(minimum)).replace('ROWS',rows).replace('PLOT',plot).replace('LO',f'{lo:.4f}').replace('HI',f'{hi:.4f}')
(ROOT/'REPORT.tex').write_text(tex)
