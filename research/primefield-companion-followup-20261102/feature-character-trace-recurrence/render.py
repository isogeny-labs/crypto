from pathlib import Path
import typst,fitz
p=Path(__file__).resolve().parent
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="300"><rect width="900" height="300" fill="white"/><g font-family="sans-serif" fill="#19334a"><text x="20" y="27" font-size="20">T-1944 · selected traces from a lifted coefficient stream</text>'''
for x,title,sub in [(20,'Bounded sign subsets','Small polynomial for each subset'),(310,'Rolling coefficient window','Lose precision at characteristic factors'),(600,'Two trace sums','Endpoint correction + phase check')]:
 svg+=f'<rect x="{x}" y="45" width="270" height="65" fill="#e6eff8" rx="6"/><text x="{x+10}" y="70" font-size="17">{title}</text><text x="{x+10}" y="96" font-size="12">{sub}</text>'
svg+='<text x="20" y="143" font-size="16">Validation02 · 29 fixed public cases · outcome counts</text>'
for y,n,label,color in [(160,12,'Extracted + verified','#267769'),(200,11,'Identity / endpoint','#497b9d'),(240,6,'Subset cap refusal','#a36439')]:
 svg+=f'<text x="20" y="{y+20}" font-size="15">{label}</text><rect x="220" y="{y}" width="{n*40}" height="27" fill="{color}"/><text x="{230+n*40}" y="{y+20}" font-size="15">{n}</text>'
svg+='</g></svg>'; (p/'flow.svg').write_text(svg);typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'));doc=fitz.open(p/'report.pdf');assert len(doc)==1;doc[0].get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(p/'report-preview.png');print('pages',len(doc))
