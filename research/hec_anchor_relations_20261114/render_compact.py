"""Derived plots/report for the frozen compact pilot; no experimental execution."""
from pathlib import Path
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, Image
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/"benchmarks/20261114-hec-anchor-forest"
OUT.mkdir(parents=True,exist_ok=True)
rows=json.loads((ROOT/"research/hec_anchor_relations_20261114/compact-run/summary.json").read_text())
fig,ax=plt.subplots(figsize=(9,3.2))
x=range(len(rows))
ax.plot(x,[r["vs_orbit"] for r in rows],"o-",label="Reference orbit / compact")
ax.plot(x,[r["vs_table"] for r in rows],"s-",label="Frozen table / compact")
ax.axhline(1,color="#444",linewidth=1)
ax.set_xticks(list(x),[f'g{r["g"]}\np={r["p"]}' for r in rows])
ax.set_ylabel("Ratio of five-seed median cold totals")
ax.set_title("Compact quotient pilot: same signed relation stream, L0 macOS")
ax.legend(fontsize=8);ax.grid(axis="y",alpha=.2);fig.tight_layout()
fig.savefig(OUT/"compact.svg");fig.savefig('/tmp/hec-compact-plot.png',dpi=150)
styles=getSampleStyleSheet();styles['BodyText'].fontSize=8.5;styles['BodyText'].leading=11
story=[Paragraph("Compact norm quotient: measured addendum",styles['Title'])]
def p(s):story.extend([Paragraph(s,styles['BodyText']),Spacer(1,4)])
p("2026-11-15 UTC | source b2fddfd8b2 | record 20261115T0625Z-hec-compact-orbit | [pcc:unverified]")
p("The compact kernel removes full norm/remainder computation and per-tuple heap allocation inside interpolation. It preserves every signed output and downstream relation decision. Total gains over the reference orbit are modest (0.83-1.32x); the p=101 genus-2 arm regresses. This does not establish the mathematical 10x objective or novelty.")
story.append(Image('/tmp/hec-compact-plot.png',width=510,height=162))
data=[["Genus","p","Compact ms","vs orbit","vs table"]]
for r in rows:
 a=next(a for a in r['arms'] if a['arm']=='compact')
 data.append([r['g'],r['p'],f'{a["median_ns"]/1e6:.4f}',f'{r["vs_orbit"]:.2f}x',f'{r["vs_table"]:.2f}x'])
t=Table(data,colWidths=[55,65,110,100,100]);t.setStyle(TableStyle([('BACKGROUND',(0,0),(-1,0),colors.HexColor('#183a4d')),('TEXTCOLOR',(0,0),(-1,0),colors.white),('FONTSIZE',(0,0),(-1,-1),9),('BOTTOMPADDING',(0,0),(-1,-1),4),('ROWBACKGROUNDS',(0,1),(-1,-1),[colors.HexColor('#eef4f6'),colors.white])]))
story.append(t);story.append(Spacer(1,8))
p("Gate: 22,128 exhaustive small-field sign cases (branch places included) and 8,000 random cases equal reference orbit and fresh Newton/full norm division. Full 55-instance/seed comparisons agree on 14 fields; all generated compact orbits equal the reference. The compact arm checks 15,115 principal and 1,290 projected anchor identities. All 165 timed solves pass known-answer verification. These are producer self-checks, not independent scientific replay.")
p("Method: for monic A of degree g+1 and N=f-h^2 divisible by A, set q_g=1 and q_k=N_(g+1+k)-sum_(j=k+1..g) a_(g+1+k-j)q_j. Only high coefficients of the norm are needed. Fixed arrays retain g=2,3,4 and exactly the original Gray order. Standard quotient algebra; primary reference: flintlib.org/doc/nmod_poly/divrem.html. Original source and raw records are unchanged.")
p("Costs: all cold tables/catalogue, anchor selection, inverses, descents, attempted relations, forest, solve and final group verification are included. All three arms share the same instance and target; order rotates/reverses by seed. One L0 ARM host, five seeds per size. Min/max observations remain in summary.json; no isolation or confidence claim. Do not multiply these ratios by a previous run's ratios. External comparison and independent replay are pending.")
p("Decision: retain this optional kernel and its regression; no default change. Stop equivalent arithmetic rewrites. Next discriminating work is held-out families and a growing-size ladder for the anchor/forest mechanism, with resource caps and rank failures preserved. Protocol, harness and raw: research/hec_anchor_relations_20261114/{COMPACT-PROTOCOL.md,compact_bench.rs,compact-run/}. No benchmark-index or leaderboard promotion.")
SimpleDocTemplate(str(OUT/'COMPACT-REPORT.pdf'),pagesize=(612,792),leftMargin=42,rightMargin=42,topMargin=30,bottomMargin=30).build(story)
