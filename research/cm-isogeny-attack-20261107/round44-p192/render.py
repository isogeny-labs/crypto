from pathlib import Path
import json,math,typst,pymupdf
r=Path(__file__).parent
flow=[('P-192: canonical cache -> labelled collision equation','840 transitions;1512 point updates;10 planted extractions checked'),('Equal raw storage: five-point cache vs three ordinary states','Generic field-cost ratio >=3 before ideal sqrt(5) step reduction'),('Global order-five linear carrier requires four coordinates','Marked-subgroup-only actions and shared arithmetic remain outside this bound')]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="255"><rect width="1000" height="255" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+i*84;s.append(f'<rect x="10" y="{y}" width="980" height="74" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+28}" font-family="sans-serif" font-size="21">{a.replace("->","→").replace(">=","≥")}</text><text x="25" y="{y+56}" font-family="sans-serif" font-size="18">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
data=[]
for inv in range(0,241,4):
 candidate=27+inv;same=3+inv;equal=(15+inv)/3
 data.append({'I_weight':inv,'same_walkers':candidate/same/math.sqrt(5),'equal_raw_storage':candidate/equal/math.sqrt(5)})
(r/'plot-data.json').write_text(json.dumps({'assumptions':'B=1,M=S=1, I varies; ideal sqrt5 step reduction; field-only model','rows':data},indent=2)+'\n')
xx=lambda x:70+x*3.5;yy=lambda y:260-y*50
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="330"><rect width="1000" height="330" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="21">Round44: illustrative field-work ratios after ideal sqrt(5) gain</text>']
for y in [0,1,2,3,4]:s.append(f'<path d="M70 {yy(y)} H910" stroke="#dddddd"/><text x="35" y="{yy(y)+5}" font-family="sans-serif" font-size="16">{y}</text>')
for x in [0,60,120,180,240]:s.append(f'<text x="{xx(x)-12}" y="284" font-family="sans-serif" font-size="16">{x}</text>')
for key,col in [('same_walkers','#39799b'),('equal_raw_storage','#a76637')]:s.append('<polyline fill="none" stroke="'+col+'" stroke-width="3" points="'+' '.join(f'{xx(z["I_weight"])},{yy(z[key])}' for z in data)+'"/>')
s.append('<text x="20" y="306" font-family="sans-serif" font-size="16">Blue: same walker count. Brown: equal raw storage. Ratio1 is parity.</text><text x="20" y="327" font-family="sans-serif" font-size="15">x: inversion weight with M=S=1; B=1. Derived cost model, not timing or walk statistics.</text></svg>');(r/'counts.svg').write_text(''.join(s))
s=['#set page(paper: "a4", margin: 16mm)','#set text(size: 9.5pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):s+=['#heading(level: 1, '+json.dumps(para[2:])+')','#image("flow.svg", width: 100%)']
 else:
  s+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Equal-storage theorem.'):s+=['#image("counts.svg", width: 100%)']
(r/'report.typ').write_text('\n'.join(s));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));d=pymupdf.open(r/'report.pdf')
for i,page in enumerate(d):page.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(d))
