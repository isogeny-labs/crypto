from pathlib import Path
import json
import fitz
from PIL import Image, ImageDraw

root=Path(__file__).resolve().parents[1]
pdf=root/'Elliptic_Curve_Twists_Visual_Guide.pdf'
preview=root/'.build'/'previews'
preview.mkdir(parents=True,exist_ok=True)
doc=fitz.open(pdf)
assert len(doc)==16, len(doc)
assert len(doc.get_toc())==16, len(doc.get_toc())
assert sum(len(p.get_links()) for p in doc)>=5
summary=[]
thumbs=[]
for i,page in enumerate(doc):
    text=page.get_text()
    assert '\ufffd' not in text, (i+1,'replacement glyph')
    assert len(text.split())>60,(i+1,'suspiciously little text')
    assert not page.get_images(),(i+1,'unexpected raster diagram')
    outside=[]
    for block in page.get_text('dict')['blocks']:
        if 'lines' not in block: continue
        for line in block['lines']:
            for span in line['spans']:
                x0,y0,x1,y1=span['bbox']
                if x0<20 or x1>592 or y0<10 or y1>780:
                    outside.append((span['text'],[round(v,1) for v in span['bbox']]))
    summary.append({'page':i+1,'words':len(text.split()),'outside_safe_bounds':outside})
    pix=page.get_pixmap(matrix=fitz.Matrix(1.6,1.6),alpha=False)
    image=Image.frombytes('RGB',[pix.width,pix.height],pix.samples)
    image.save(preview/f'page_{i+1:02d}.png')
    thumb=image.resize((306,396),Image.Resampling.LANCZOS)
    tile=Image.new('RGB',(322,420),'#D7DFE4')
    tile.paste(thumb,(8,5))
    ImageDraw.Draw(tile).text((12,403),f'Page {i+1:02d}',fill='#172E40')
    thumbs.append(tile)

sheet=Image.new('RGB',(4*322,4*420),'#D7DFE4')
for i,tile in enumerate(thumbs): sheet.paste(tile,((i%4)*322,(i//4)*420))
sheet.save(preview/'contact_sheet.png')
(root/'.build'/'pdf_validation.json').write_text(json.dumps(summary,indent=2))
print(json.dumps({'pages':len(doc),'bookmarks':len(doc.get_toc()),
                  'links':sum(len(p.get_links()) for p in doc),
                  'pdf_bytes':pdf.stat().st_size,
                  'out_of_bounds':[r for r in summary if r['outside_safe_bounds']],
                  'contact_sheet':str(preview/'contact_sheet.png')},indent=2))
