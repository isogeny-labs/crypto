"""Render recorded native measurements; no research execution or result math."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import typst
p=Path(__file__).resolve().parent
rows=json.loads((p/'figure-data.json').read_text())['rows']
fig,axes=plt.subplots(1,2,figsize=(10,4))
for ax,key,title in zip(axes,['natural_solver_ms','outer_wall_ms'],['Native solver interval','Supervised outer wall']):
    available=[r for r in rows if r.get(key) is not None and r["status"].startswith("PASS")]
    ax.bar([r['id'] for r in available],[r[key] for r in available],color=['#23798e' if r.get('columns')==2048 else '#9c661f' for r in available])
    ax.set_ylabel('Milliseconds'); ax.set_title(title); ax.tick_params(axis='x',labelrotation=65)
fig.suptitle('Native N83 800K continuation | n=1 per base/resource | descriptive clocks, no ranking')
fig.tight_layout()
for ext in ['svg','png']: fig.savefig(p/f'cap-continuation.{ext}',dpi=150)
plt.close(fig)
lines=['#text(size:8pt)[#table(columns:(1.5fr,1fr,1.5fr,1.6fr,1.6fr),inset:3pt,[Run],[Conflicts],[Audit],[Solver ms],[Outer ms],']
for r in [r for r in rows if r['status'].startswith('PASS')]:
    c=r.get('terminal_counters') or {}
    text=[r['id'],str(c.get('conflicts','missing')),'PASS bounded zero',str(r.get('natural_solver_ms','missing')),str(r.get('outer_wall_ms','missing'))]
    lines.append(','.join('['+t+']' for t in text)+',')
lines.append(')]'); (p/'runs.typ').write_text('\n'.join(lines)+'\n')
fig,ax=plt.subplots(figsize=(10,3.6))
available=[r for r in rows if r.get("outer_wall_ms") is not None]
refusal=json.loads((p/"refusal-outer.json").read_text())
available.insert(2,{"id":"k2048-v13-declaration-refusal","status":"DECLARATION_REFUSED","outer_wall_ms":refusal["outer_wall_ms"]})
labels=[r["id"] for r in available]
bars=ax.bar(labels,[r["outer_wall_ms"] for r in available],color="#23798e")
for bar,r in zip(bars,available):
    if not r["status"].startswith("PASS"): bar.set_facecolor("#9c661f");bar.set_edgecolor("#222");bar.set_hatch("///")
ax.annotate(f"{refusal['outer_wall_ms']} ms",xy=(2,refusal["outer_wall_ms"]),xytext=(2,350000),ha="center",fontsize=8,arrowprops={"arrowstyle":"->","color":"#333"})
ax.set_ylabel("Milliseconds");ax.set_title("Recorded outer attempt cost | hatched = failed attempt, no search admission")
ax.tick_params(axis="x",labelrotation=35);fig.tight_layout()
for ext in ["svg","png"]: fig.savefig(p/f"attempt-costs.{ext}",dpi=150)
plt.close(fig)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'),font_paths=[str(Path(matplotlib.__file__).parent/'mpl-data/fonts/ttf')])
