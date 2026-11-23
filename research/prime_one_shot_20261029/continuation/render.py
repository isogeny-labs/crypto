"""Layout of recorded development counts and derived-status diagram only."""
from pathlib import Path
import json,re,sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
plt.rcParams["svg.fonttype"]="none"
plt.rcParams["svg.hashsalt"]="prime-continuation-20261030"
HERE=Path(__file__).resolve().parent
# Recorded complete unit-model counts from solver/README.md; no fit or timing data.
labels=["t1,c1","t2,c1","t3,c1","t3,c16","t4,c1"]
sat=[0,0,6,18,18];unsat=[18,18,12,0,0]
fig,ax=plt.subplots(figsize=(8,3.6))
ax.bar(labels,sat,color="#176D99",label="SAT")
ax.bar(labels,unsat,bottom=sat,color="#D3D3D3",label="UNSAT")
ax.set(ylim=(0,20),ylabel="Targets (complete census)",title="Actual unknown solving: all 90 model sets match the oracle")
ax.set_yticks([0,6,12,18]);ax.legend(ncols=2)
fig.text(.12,.015,"One fixed E/F17 of order19; 400 complete models. Development checks, no scaling or timing.",fontsize=8)
fig.tight_layout(rect=(0,.05,1,1));fig.savefig(HERE/"model-census.svg");fig.savefig(HERE/"model-census.png",dpi=150);plt.close(fig)
fig,ax=plt.subplots(figsize=(9,4.5));ax.axis("off")
boxes=[(.02,.62,.28,.25,"REVIEWED DERIVATIONS\nRationality, coverage, setup\nOdd prime whole-order E/Fp","#DFEDF4"),(.36,.62,.27,.25,"WORKING BASELINE\nComplete IC on E/F17\nPDP loop: p(p-1)","#E8EEF0"),(.70,.62,.28,.25,"NORM-CANDIDATE GAP\nInitial implicit elimination\nProved cost/yield bound","#F8E1CC"),(.20,.13,.73,.25,"NORM CANDIDATE: CONDITIONAL REDUCTION\nIts initial elimination cost is unresolved.\nThe separate existing tensor theorem chain is reviewed in the text.\nOur coefficient baseline is exponential.","#F8E1CC")]
for x,y,w,h,t,c in boxes:
 ax.add_patch(plt.Rectangle((x,y),w,h,facecolor=c,edgecolor="#444"));ax.text(x+w/2,y+h/2,t,ha="center",va="center",fontsize=9)
for a,b in [((.30,.745),(.36,.745)),((.63,.745),(.70,.745)),((.84,.62),(.84,.38))]:ax.annotate("",xy=b,xytext=a,arrowprops={"arrowstyle":"->","lw":1.5})
fig.tight_layout();fig.savefig(HERE/"conditional-route.svg");fig.savefig(HERE/"conditional-route.png",dpi=150);plt.close(fig)
for svg in [HERE/"model-census.svg",HERE/"conditional-route.svg"]:svg.write_text("\n".join(s.rstrip() for s in svg.read_text().splitlines())+"\n")

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
        if text.startswith('An independently implemented native verifier'):
            source.append('#image("model-census.svg", width: 100%)')
    i+=1
(HERE / "report.typ").write_text('\n\n'.join(source).rstrip()+'\n')
sys.path.insert(0,"/tmp/prime-report-deps")
import typst
typst.compile(str(HERE / "report.typ"), output=str(HERE / "REPORT.pdf"))
