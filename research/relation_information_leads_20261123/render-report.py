from pathlib import Path
import json,re
s=Path('/workspace/relation-ideas-live/research/relation_information_leads_20261123')
boxes=[(12,12,315,85,'Coordinate construction','Proposal: shared point pool','#edf2f7'),(382,12,315,85,'Information audit','Reject forced label cycles','#e5eef5'),(752,12,315,85,'Degree-closure trace','W_D is distinct from V_D','#e5eef5'),(382,150,315,85,'Complete-cost comparison','Future work; no 15× result','#f3eee6')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="255" viewBox="0 0 1080 255"><rect width="1080" height="255" fill="white"/>']
for x,y,w,h,a,b,col in boxes:
 svg.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="8" fill="{col}" stroke="#9baebf"/>')
 for k,t in enumerate([a,b]):svg.append(f'<text x="{x+14}" y="{y+30+k*26}" font-family="DejaVu Sans" font-size="18" fill="#23384d">{t}</text>')
svg+=['<path d="M327 54 H374 M697 54 H744 M540 98 V142" fill="none" stroke="#526d86" stroke-width="2"/>','<text x="16" y="246" font-family="DejaVu Sans" font-size="15" fill="#526d86">Arrows describe proposed checks, not measured gains. All collection, selection, proof and failure costs remain charged.</text>','</svg>'];(s/'lead-map.svg').write_text(''.join(svg))
# Presentation-only Markdown subset renderer. It performs no mathematical calculation.
lines=(s/'REPORT_405.md').read_text().splitlines();out=['#set page(paper:"a4",margin:17mm,numbering:"1")','#set text(font:"DejaVu Sans",size:10pt)','#set par(leading:0.4em)','#set heading(numbering:none)']
def q(t):return json.dumps(t,ensure_ascii=False)
def inline(t):
 t=t.replace('**','').replace('`','');parts=[];last=0
 for m in re.finditer(r'\[([^\]]+)\]\(([^)]+)\)',t):
  if m.start()>last:parts.append('#text('+q(t[last:m.start()])+')')
  parts.append('#link('+q(m[2])+')[#text('+q(m[1])+')]');last=m.end()
 if last<len(t):parts.append('#text('+q(t[last:])+')')
 return ''.join(parts)
i=0;paragraph=[]
def flush():
 if paragraph:out.append('#par['+inline(' '.join(paragraph))+']');paragraph.clear()
while i<len(lines):
 l=lines[i]
 if l.startswith('```'):
  flush();block=[];i+=1
  while i<len(lines) and not lines[i].startswith('```'):block.append(lines[i]);i+=1
  out.append('#block(width:100%,fill:rgb("f3f5f7"),inset:7pt)[#set text(size:8.7pt)\n#raw('+q('\n'.join(block))+',block:true)]')
 elif l.startswith('!['):
  flush();out.append('#image("lead-map.svg",width:100%)')
 elif l.startswith('#'):
  flush();n=len(l)-len(l.lstrip('#'));title=l[n:].strip()
  out.append('#heading(level:'+str(n)+','+q(title)+')')
 elif l.startswith('|'):
  flush();table=[]
  while i<len(lines) and lines[i].startswith('|'):
   cells=[x.strip() for x in lines[i].strip('|').split('|')]
   if not all(re.fullmatch(r'[:\- ]+',x) for x in cells):table.append(cells)
   i+=1
  args=[]
  for row in table:
   for cell in row:args.append('['+inline(cell)+']')
  out.append('#table(columns:(20%,43%,37%),inset:5pt,'+','.join(args)+')');continue
 elif l.startswith('- '):flush();out.append('#block['+inline('• '+l[2:])+']')
 elif not l.strip():flush()
 else:paragraph.append(l)
 i+=1
flush();(s/'report.typ').write_text('\n\n'.join(out)+'\n')
import typst
typst.compile(str(s/'report.typ'),output=str(s/'report.pdf'),root=str(s))
