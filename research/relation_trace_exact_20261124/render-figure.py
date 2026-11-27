from pathlib import Path
import json,html
s=Path(__file__).resolve().parent;summary=json.loads((s/'SCAN_SUMMARY_422.json').read_text());r={x['presentation']:x for x in summary['records'] if x['case']=='p2147483647-n6' and x['degree']==4}
v=['<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="315" viewBox="0 0 1200 315"><rect width="1200" height="315" fill="white"/>']
def text(x,y,t,size=18,color='#23384d'):
 v.append(f'<text x="{x}" y="{y}" font-family="DejaVu Sans" font-size="{size}" fill="{color}">{html.escape(str(t))}</text>')
for x,w in [(12,574),(612,574)]:v.append(f'<rect x="{x}" y="12" width="{w}" height="228" rx="9" fill="#edf2f7" stroke="#a8b8c7"/>')
text(30,44,'Recorded p31, n6, D4 marginal ranks',21);text(245,77,'Raw W gain',17);text(410,77,'Closed V gain',17)
for y,key,label in [(117,'single-0','Cubic difference'),(172,'single-1','Cubic syzygy')]:
 text(30,y+17,label,17)
 for x,field,col in [(245,'raw_gain','#355f83'),(410,'closed_gain','#6f91ab')]:
  value=r[key][field]
  if value:v.append(f'<rect x="{x}" y="{y}" width="{value*12}" height="23" fill="{col}"/>')
  text(x+value*12+9,y+18,value,17)
text(30,221,'Numbers describe this degree-space diagnostic only.',15)
text(632,44,'Preserve P/Q information during pruning',21);text(636,91,'Full augmented rows:  [ R | a | b ]',20)
v.append('<path d="M640 113 H1145" stroke="#9baebf" stroke-width="1.5"/>')
text(636,146,'Sparse basis of the same augmented row space',18);text(636,179,'Replay original-row dependency: A + d B = 0',18);text(636,219,'Curve/subgroup validity is a separate obligation.',15)
v.append('<rect x="12" y="254" width="1174" height="46" rx="7" fill="#f3eee6"/>')
text(28,283,'Next diagnostic: joint pruning that preserves replayed V_D and augmented information. Same-host, unsigned evidence.',16)
v.append('</svg>');(s/'trace-map.svg').write_text(''.join(v))
