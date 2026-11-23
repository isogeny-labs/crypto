from pathlib import Path
import json,hashlib,html
import fitz
D=Path(__file__).resolve().parent
r=json.loads((D/'review.json').read_text());flow=json.loads((D/'flow.json').read_text())
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="'+str(len(flow)*90+45)+'" viewBox="0 0 1000 '+str(len(flow)*90+45)+'"><rect width="100%" height="100%" fill="#f4f7fa"/>'
for i,t in enumerate(flow):
 y=20+i*90
 svg+=f'<rect x="55" y="{y}" width="890" height="62" rx="8" fill="#deecf5" stroke="#285b7b"/><text x="500" y="{y+38}" text-anchor="middle" font-family="sans-serif" font-size="22">{html.escape(t)}</text>'
 if i+1<len(flow):svg+=f'<path d="M500 {y+63} v23 m-5 -6 l5 6 5 -6" fill="none" stroke="#285b7b" stroke-width="2"/>'
svg+='</svg>'; (D/'flow.svg').write_text(svg)
doc=fitz.open();page=doc.new_page();y=45
page.insert_text((40,y),'Independent source review',fontsize=19);y+=29
page.insert_text((40,y),r['source_packet'],fontsize=10);y+=22
page.insert_text((40,y),'Source-only; no compiler, payload or measurement executed.',fontsize=10);y+=25
for idx,c in enumerate(r['checks']):
 text=f'{idx+1}. {c}'
 rect=fitz.Rect(40,y,550,y+105)
 needed=page.insert_textbox(rect,text,fontsize=10,lineheight=1.3)
 if needed<0:raise RuntimeError('overflow')
 y+=105-needed+16
 if y>650 and idx+1<len(r['checks']):page=doc.new_page();y=45
page=doc.new_page();page.insert_text((40,40),'Argument and custody map',fontsize=18)
vector=fitz.open(stream=svg.encode(),filetype='svg');pdf=fitz.open('pdf',vector.convert_to_pdf());page.show_pdf_page(fitz.Rect(40,60,550,390),pdf,0)
text='Remaining step: '+' '.join(r['remaining'])+'\n\nInputs and precise hashes: review.json and snapshot/.\n\n'+r['kb']+'\n\n'+r['trust']
assert page.insert_textbox(fitz.Rect(40,420,550,760),text,fontsize=11,lineheight=1.4)>=0
doc.save(D/'report.pdf');doc.close()
files={str(p.relative_to(D)):{'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size} for p in D.rglob('*') if p.is_file() and p.name!='manifest.json'}
(D/'manifest.json').write_text(json.dumps(files,indent=2)+'\n')
with fitz.open(D/'report.pdf') as pdf:
 for i,p in enumerate(pdf):p.get_pixmap(matrix=fitz.Matrix(1,1)).save('/var/tmp/'+D.name+'-'+str(i)+'.png')
 print(D.name,len(pdf),'pages',hashlib.sha256((D/'review.json').read_bytes()).hexdigest())
