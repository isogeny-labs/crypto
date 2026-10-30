"""Render recorded data and the report. No solver or research computation."""
from pathlib import Path
import json
import re
import sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
rows = [json.loads(s) for s in (ROOT / "research/pkm_symmetric_tower_20261026/rows_m3.jsonl").read_text().splitlines() if s.strip()]
fig, ax = plt.subplots(figsize=(8, 4.4))
for arm, color, label in [("plain-chain", "#176D99", "Plain chain"), ("symmetric", "#D06A22", "Symmetric coefficients")]:
    chosen = [r for r in rows if r["arm"] == arm and r["target"] == 0 and r["t"] <= 4]
    ax.plot([r["N"] for r in chosen], [r["max_cols"] for r in chosen], "o-", color=color, label=label)
ax.set(yscale="log", xlabel="Recorded tower variables N (m = 3)", ylabel="Widest matrix (columns)", title="Recorded widths: encodings have different root constraints")
ax.set_xticks([6, 9, 12])
ax.grid(alpha=.2)
ax.legend()
fig.text(.10, .015, "Three refutations/arm; raw field targets, unfiltered x. Distinctness differs. No timing claim.", fontsize=8)
fig.tight_layout(rect=(0,.04,1,1))
fig.savefig(HERE / "recorded-widths.svg")
fig.savefig(HERE / "recorded-widths.png", dpi=150)
plt.close(fig)

fig, ax = plt.subplots(figsize=(9, 4.8))
ax.axis("off")
boxes = [(0.02,.62,.27,.23,"KNOWN INGREDIENT\nImplicit prime-field base\n+ elliptic norm constraint", "#DFEDF4"),
         (.36,.62,.27,.23,"PROPOSED EXTENSION\nShared coefficients\nwith growing arity m,t", "#E8EEF0"),
         (.70,.62,.28,.23,"MISSING RESULT\nUniform root/refutation\nsolver: exp(o(n)) work", "#F8E1CC"),
         (.36,.16,.62,.24,"CONDITIONAL ONLY\nIf construction, coverage, solving and rank all cost exp(o(n)),\nthen complete cold ECDLP could be sub-exponential.\nNo such solver or complexity theorem is established.", "#F8E1CC")]
for x,y,w,h,t,c in boxes:
    ax.add_patch(plt.Rectangle((x,y),w,h,facecolor=c,edgecolor="#444",lw=1))
    ax.text(x+w/2,y+h/2,t,ha="center",va="center",fontsize=9)
for a,b in [((.29,.735),(.36,.735)),((.63,.735),(.70,.735)),((.84,.62),(.84,.40))]:
    ax.annotate("",xy=b,xytext=a,arrowprops={"arrowstyle":"->","lw":1.5})
ax.text(.02,.30,"n = bit length\nSub-exponential means exp(o(n)).\nA compact input circuit\nis not a fast solver.",fontsize=10)
fig.tight_layout()
fig.savefig(HERE / "conditional-route.svg")
fig.savefig(HERE / "conditional-route.png",dpi=150)
plt.close(fig)
for svg in (HERE / "recorded-widths.svg", HERE / "conditional-route.svg"):
    svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines())+'\n')

q = lambda s: json.dumps(s, ensure_ascii=False)
lines = (HERE / "README.md").read_text().splitlines()
source = ['#set page(paper: "a4", margin: 19mm, numbering: "1")', '#set text(font: "DejaVu Sans", size: 9pt)', '#set par(justify: false, leading: 0.65em)', '#set heading(numbering: none)']
i=0
while i<len(lines):
    line=lines[i]
    if line.startswith("#"):
        depth=len(line)-len(line.lstrip("#"))
        source.append(f'#heading(level: {depth}, text({q(line[depth:].strip())}))')
        if depth==1:
            source.append('#image("conditional-route.svg", width: 100%)')
    elif line.startswith("|"):
        table=[]
        while i<len(lines) and lines[i].startswith("|"):
            parts=[v.strip() for v in lines[i].strip("|").split("|")]
            if not all(re.fullmatch(r"[-: ]+",v) for v in parts): table.append(parts)
            i+=1
        cols=len(table[0])
        source.append('#table(columns: ('+', '.join(['1fr']*cols)+'), inset: 4pt, '+', '.join('text(size: 8pt, '+q(cell)+')' for row in table for cell in row)+')')
        continue
    elif line.startswith("    "):
        code=[]
        while i<len(lines) and lines[i].startswith("    "):
            code.append(lines[i][4:]);i+=1
        source.append('#raw('+q('\n'.join(code))+', block: true)')
        continue
    elif line.strip():
        para=[line]
        while i+1<len(lines) and lines[i+1].strip() and not lines[i+1].startswith(("#","|","    ")):
            i+=1;para.append(lines[i])
        text=' '.join(para)
        # Keep every word and URL; Markdown is the canonical formatted text.
        text=re.sub(r"\*\*(.*?)\*\*",r"\1",text)
        source.append('#text('+q(text)+')\n\n')
        if text.startswith('The figure uses recorded'):
            source.append('#image("recorded-widths.svg", width: 100%)')
    i+=1
(HERE / "report.typ").write_text('\n\n'.join(source).rstrip()+'\n')
sys.path.insert(0,"/tmp/prime-report-deps")
import typst
typst.compile(str(HERE / "report.typ"), output=str(HERE / "REPORT.pdf"))
