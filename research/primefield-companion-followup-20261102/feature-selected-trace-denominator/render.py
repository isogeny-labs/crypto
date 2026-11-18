from pathlib import Path
import typst,fitz
p=Path(__file__).resolve().parent
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="900" height="300"><rect width="900" height="300" fill="white"/><g font-family="sans-serif" fill="#18334a"><text x="20" y="26" font-size="19">T-1946 · scaled recurrence, selected normalization</text>'''
for x,title,sub in [(20,'Maintain V = U · c','U stays a unit'),(310,'Advance scaled window','Strip characteristic factors exactly'),(600,'Normalize requested indices','At most 2d inversions per subset')]:
 svg+=f'<rect x="{x}" y="44" width="270" height="65" fill="#e4eff8" rx="6"/><text x="{x+10}" y="70" font-size="17">{title}</text><text x="{x+10}" y="96" font-size="13">{sub}</text>'
svg+='<text x="20" y="145" font-size="16">Validation01 · 29 fixed public cases · inversion counts</text>'
for y,n,label,color in [(165,2654208,'Per-step reference','#527c9b'),(210,589824,'Selected normalization','#28796c')]:
 w=n/5500
 svg+=f'<text x="20" y="{y+20}" font-size="15">{label}</text><rect x="230" y="{y}" width="{w}" height="28" fill="{color}"/><text x="{240+w}" y="{y+20}" font-size="15">{n:,}</text>'
svg+='<text x="20" y="275" font-size="14">Modular products rise: 38,436,864 → 53,059,596. Inversion moduli differ; no timing ranking.</text></g></svg>'; (p/'flow.svg').write_text(svg);typst.compile(str(p/'report.typ'),output=str(p/'report.pdf'));doc=fitz.open(p/'report.pdf');assert len(doc)==1;doc[0].get_pixmap(matrix=fitz.Matrix(1.2,1.2)).save(p/'report-preview.png');print('pages',len(doc))
