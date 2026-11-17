from pathlib import Path
import json
p=Path(__file__).resolve().parent
rows=json.loads((p/'counts.json').read_text())['rows']
stages=[('initial_geometry','Initial geometry'),('initial_transport','Initial transport'),('initial_character','Initial residuals'),('extra_geometry','Extra geometry'),('extra_transport','Extra transport'),('extra_character','Extra residual'),('verify','Forward check')]
values=[sum(x['value']for x in rows if x['label']=='k25'and x['stage']==s and x['op']=='products')for s,_ in stages]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="900" height="320" viewBox="0 0 900 320"><rect width="900" height="320" fill="white"/><g font-family="Arial,sans-serif" font-size="17" fill="#182c40"><text x="15" y="25">Recorded modular-product helper calls: Q=[25]P, F509/r479</text>']
for i,((_,label),n)in enumerate(zip(stages,values)):
 y=46+i*32;w=560*n/max(values);svg.append(f'<text x="15" y="{y+17}">{label}</text><rect x="200" y="{y}" width="{w}" height="22" fill="#315f88"/><text x="{210+w}" y="{y+17}">{n:,}</text>')
svg.append('<text x="15" y="294" font-size="14">counts.json, label k25; exact event totals, no timing or uncertainty estimate</text></g></svg>')
(p/'products.svg').write_text(''.join(svg))
