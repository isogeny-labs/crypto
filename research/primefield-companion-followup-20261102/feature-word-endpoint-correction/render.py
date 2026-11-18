from pathlib import Path
import typst,fitz
p=Path(__file__).resolve().parent
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="310"><rect width="900" height="310" fill="white"/><g font-family="sans-serif" fill="#16324a"><text x="20" y="26" font-size="20">T-1943 · linear factors with explicit endpoint correction</text>'''
for x,title,sub in [(20,'Linear word product','No character-section square'),(310,'Two raw traces','Subtract endpoint moments'),(600,'Corrected singleton','Bézout phase + point check')]:
 svg+=f'<rect x="{x}" y="45" width="270" height="65" rx="6" fill="#e5eff8"/><text x="{x+10}" y="70" font-size="17">{title}</text><text x="{x+10}" y="96" font-size="14">{sub}</text>'
svg+='<text x="20" y="147" font-size="16">Validation01 · 29 fixed public cases · polynomial monomial-pair counts</text>'
for y,n,label,color in [(170,2600842,'Zero-safe reference','#527798'),(213,2388639,'Corrected factors','#277767')]:
 w=n/6000
 svg+=f'<text x="20" y="{y+20}" font-size="15">{label}</text><rect x="210" y="{y}" width="{w}" height="28" fill="{color}"/><text x="{220+w}" y="{y+20}" font-size="15">{n:,}</text>'
svg+='<text x="20" y="278" font-size="14">Additional correction: 19,844 field multiplications; other operation classes remain extra.</text><text x="20" y="301" font-size="14">Fixed deterministic checks; no timing ranking or sampling uncertainty.</text></g></svg>'
(p/'flow.svg').write_text(svg);typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'));doc=fitz.open(p/'report.pdf');assert len(doc)==1;doc[0].get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(p/'report-preview.png');print('pages',len(doc))
