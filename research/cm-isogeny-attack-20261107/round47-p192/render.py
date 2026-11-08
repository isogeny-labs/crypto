from pathlib import Path
import json,typst,pymupdf
r=Path(__file__).parent;c=json.loads((r/'certificate.json').read_text())
flow=[('Translate three coordinates with tracked labels','One batch inversion for 3B additions'),('Lockstep tables and signed-digit ladder','102 further inversion layers; exceptions recorded'),('Canonicalize and update labels','32 checked transitions; eight independently recounted cost vectors')]
(r/'flow.json').write_text(json.dumps(flow,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="255"><rect width="1000" height="255" fill="white"/>']
for i,(a,b) in enumerate(flow):
 y=4+i*84;s.append(f'<rect x="10" y="{y}" width="980" height="74" rx="5" fill="#e7f0ec" stroke="#49617a"/><text x="25" y="{y+28}" font-family="sans-serif" font-size="21">{a}</text><text x="25" y="{y+56}" font-family="sans-serif" font-size="18">{b}</text>')
s.append('</svg>');(r/'flow.svg').write_text(''.join(s))
data=[{'power':i,'kind':'step','D':z['cost']['M'],'A':z['cost']['S']} for i,z in enumerate(c['steps'])];(r/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n')
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="325"><rect width="1000" height="325" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="21">Round47: field counts per four-walker transition</text>']
for i,z in enumerate(data):
 x=30+i*120;hd=z['D']*.055;ha=z['A']*.055
 s.append(f'<rect x="{x}" y="{235-hd}" width="85" height="{hd}" fill="#356f99"/><rect x="{x}" y="{235-hd-ha}" width="85" height="{ha}" fill="#ac783e"/><text x="{x}" y="{228-hd-ha}" font-family="sans-serif" font-size="16">{z["D"]}M+{z["A"]}S</text><text x="{x}" y="255" font-family="sans-serif" font-size="16">{z["power"]}:{z["kind"][:4]}</text>')
s.append('<text x="20" y="286" font-family="sans-serif" font-size="17">Blue: field multiplications. Brown: field squarings.</text><text x="20" y="312" font-family="sans-serif" font-size="16">Each step also uses 103 inversions; setup and scalar labels separate.</text></svg>');(r/'counts.svg').write_text(''.join(s))
s=['#set page(paper: "a4", margin: 16mm)','#set text(size: 9.5pt)']
for para in (r/'report.md').read_text().split('\n\n'):
 if para.startswith('# '):s+=['#heading(level: 1, '+json.dumps(para[2:])+')','#image("flow.svg", width: 100%)']
 else:
  s+=['#text('+json.dumps(para)+')','#parbreak()']
  if para.startswith('Exact fixture checks.'):s+=['#image("counts.svg", width: 100%)']
(r/'report.typ').write_text('\n'.join(s));typst.compile(str(r/'report.typ'),output=str(r/'report.pdf'));d=pymupdf.open(r/'report.pdf')
for i,page in enumerate(d):page.get_pixmap().save(r/f'page-{i+1}.png')
print('pages',len(d))
