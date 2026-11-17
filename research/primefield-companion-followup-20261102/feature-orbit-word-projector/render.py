from pathlib import Path
import typst,fitz
p=Path(__file__).resolve().parent
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="310" viewBox="0 0 900 310"><rect width="900" height="310" fill="white"/><g font-family="sans-serif" fill="#17304a"><text x="20" y="26" font-size="19">T-1940 · exact orbit-word construction</text>'''
for x,title,sub in [(20,'Public sign words','Zero-safe selectors'),(310,'Orbit-average J₀','Normalize by m / stabilizer'),(600,'Count + first moment','Singleton → Bézout → point check')]:
 svg+=f'<rect x="{x}" y="45" width="270" height="70" rx="6" fill="#e4eff8"/><text x="{x+12}" y="72" font-size="17">{title}</text><text x="{x+12}" y="99" font-size="14">{sub}</text>'
svg+='''<text x="20" y="150" font-size="16">Validation02 public outcomes · 29 fixed cases · counts, no sampling uncertainty</text>'''
for y,n,label,color in [(170,18,'Extracted + verified','#267568'),(215,11,'Identity / endpoint','#447aa3')]:
 svg+=f'<text x="20" y="{y+20}" font-size="15">{label}</text><rect x="220" y="{y}" width="{n*28}" height="28" fill="{color}"/><text x="{230+n*28}" y="{y+20}" font-size="15">{n}</text>'
svg+='<text x="20" y="287" font-size="14">Large transcript: 64-term cap reached; sparse node representation remains the dependency.</text></g></svg>'
(p/'flow.svg').write_text(svg)
typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'))
doc=fitz.open(p/'report.pdf');assert len(doc)==1;doc[0].get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(p/'report-preview.png');print('rendered pages',len(doc))
