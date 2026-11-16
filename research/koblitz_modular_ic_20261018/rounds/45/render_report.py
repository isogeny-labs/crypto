from pathlib import Path
import json
import typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());cases=r['collector_cases']
largest=max(x['deck_orbits']for x in cases); svg=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="270" viewBox="0 0 780 270"><rect width="780" height="270" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="16">Actual source deck orbits: fixed n=5 carrier</text>']
for i,x in enumerate(cases):
    yy=55+i*44; width=580*x['deck_orbits']/largest
    svg.append(f'<text x="20" y="{yy+17}" font-family="sans-serif" font-size="12">degree {x["extension_degree"]}</text><rect x="100" y="{yy}" width="{width}" height="25" fill="#337e88"/><text x="{110+width}" y="{yy+17}" font-family="sans-serif" font-size="12">{x["deck_orbits"]}</text>')
svg.append('<text x="20" y="255" font-family="sans-serif" font-size="11">control7-01 / collect9-01 / collect11-01 / collect13-01; Picard images unevaluated</text></svg>')
(h/'round45.svg').write_text('\n'.join(svg)+'\n')
(h/'plot-data.json').write_text(json.dumps([{'attempt':x['attempt'],'degree':x['extension_degree'],'deck_orbits':x['deck_orbits']}for x in cases],indent=2)+'\n')
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
