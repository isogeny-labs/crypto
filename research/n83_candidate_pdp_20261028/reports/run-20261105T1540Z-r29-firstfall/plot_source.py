from pathlib import Path
import csv
rows=list(csv.DictReader(open(Path(__file__).with_name("plot.csv"))))
W,H=960,520; left,top=190,90; scale=30; bar=34; gap=60
colors={"exact_ffd_2":"#2563eb","ffd_lower_bound_ge_5":"#d97706"}
labels={"exact_ffd_2":"Exact FFD = 2","ffd_lower_bound_ge_5":"FFD lower bound >= 5"}
s=[f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">','<rect width="100%" height="100%" fill="white"/>','<style>text{font-family:Arial,sans-serif;fill:#172033}.title{font-size:24px;font-weight:bold}.lab{font-size:16px}.small{font-size:13px}</style>','<text x="40" y="42" class="title">Spike29 first-fall outcomes (16 systems per family)</text>']
for i,r in enumerate(rows):
 y=top+i*130; s.append(f'<text x="40" y="{y+25}" class="lab">{r["family"].title()}</text>')
 for j,key in enumerate(["exact_ffd_2","ffd_lower_bound_ge_5"]):
  yy=y+j*42; n=int(r[key]); wid=n*scale
  s.append(f'<rect x="{left}" y="{yy}" width="{wid}" height="{bar}" fill="{colors[key]}"/>')
  s.append(f'<text x="{left+wid+10}" y="{yy+23}" class="lab">{n}/16</text>')
 s.append(f'<text x="{left}" y="{y+100}" class="small">{labels["exact_ffd_2"]} (blue) / lower bound (orange)</text>')
s.append(f'<line x1="{left}" y="{top-10}" x2="{left}" y="{top+2*130+75}" stroke="#334155"/>')
s.append('<text x="190" y="500" class="small">Counts are certificate-backed first-fall labels; no exact LFD or affine Dreg labels were collected.</text></svg>')
Path(__file__).with_name("plot.svg").write_text("\n".join(s))
