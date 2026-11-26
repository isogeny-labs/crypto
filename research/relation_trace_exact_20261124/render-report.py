from pathlib import Path
import json,re
s=Path(__file__).resolve().parent
# Presentation-only Markdown subset renderer. It performs no mathematical calculation.
lines=(s/'REPORT.md').read_text().splitlines();out=['#set page(paper:"a4",margin:17mm,numbering:"1")','#set text(font:"DejaVu Sans",size:10pt)','#set par(leading:0.4em)','#set heading(numbering:none)']
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
  flush();out.append('#image("trace-map.svg",width:100%)')
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
  out.append('#table(columns:(32%,5%,21%,21%,21%),inset:4pt,'+','.join(args)+')');continue
 elif l.startswith('- '):flush();out.append('#block['+inline('• '+l[2:])+']')
 elif not l.strip():flush()
 else:paragraph.append(l)
 i+=1
flush();(s/'report.typ').write_text('\n\n'.join(out)+'\n')
try:
 import typst
except ModuleNotFoundError:
 # Presentation-only fallback to tools already installed on this host.
 import subprocess,tempfile
 subprocess.run(['inkscape',str(s/'trace-map.svg'),'--export-type=pdf',
                 '--export-filename='+str(s/'trace-map.pdf')],check=True)
 with tempfile.TemporaryDirectory(prefix='relation-render-',dir='/dev/shm') as tmp:
  md=Path(tmp)/'report.md'
  md.write_text((s/'REPORT.md').read_text().replace('](trace-map.svg)','](trace-map.pdf)'))
  subprocess.run(['pandoc',str(md),'--resource-path='+str(s),'--pdf-engine=xelatex',
                  '-V','mainfont=DejaVu Sans','-V','monofont=DejaVu Sans Mono',
                  '-V','geometry:margin=17mm','-V','fontsize=10pt','-V','papersize=a4',
                  '-o',str(s/'report.pdf')],check=True)
else:
 typst.compile(str(s/'report.typ'),output=str(s/'report.pdf'),root=str(s))
