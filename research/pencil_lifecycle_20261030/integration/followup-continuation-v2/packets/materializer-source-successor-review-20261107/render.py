from pathlib import Path
import fitz,textwrap,hashlib,json
p=Path(__file__).resolve().parent
d=fitz.open();lines=[]
for line in (p/'report.md').read_text().splitlines():
 line=line.replace('`','').replace('—','-')
 lines+=textwrap.wrap(line,94,break_long_words=True) or ['']
for start in range(0,len(lines),53):
 page=d.new_page(width=612,height=792);page.insert_text((35,40),'\n'.join(lines[start:start+53]),fontsize=9.2,lineheight=1.35)
page=d.new_page(width=792,height=400);svg=fitz.open(stream=(p/'pipeline.svg').read_bytes(),filetype='svg');s=fitz.open('pdf',svg.convert_to_pdf());page.show_pdf_page(fitz.Rect(16,50,776,301),s,0)
d.save(p/'report.pdf');d.close()
x=fitz.open(p/'report.pdf')
for i,page in enumerate(x):
 page.get_pixmap(matrix=fitz.Matrix(1,1)).save('/var/tmp/materializer-review-page-'+str(i)+'.png')
 print('page',i+1,'text_chars',len(page.get_text()))

files={f.name:{'bytes':f.stat().st_size,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()} for f in sorted(p.iterdir()) if f.is_file() and f.name!='manifest.json'}
(p/'manifest.json').write_text(json.dumps({'schema':'artifact.manifest/v1','files':files},indent=2)+'\n');print('manifest_entries',len(files))
