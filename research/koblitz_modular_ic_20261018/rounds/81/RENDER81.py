"""Render the complete four-map candidate quotient diagnostic."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
r = json.loads((h / 'results.json').read_text())
maps = r['candidate_maps']
assert len(maps) == 4 and all(x['ideal_equality_verified'] for x in maps)
assert [x['source_xyp_codes'] for x in maps] == [[2, 0, 0], [2, 0, 0], [0, 1, 0], [0, 1, 0]]
data = {'maps': [{'component': x['component'], 'prime': x['prime'],
                  'generator_index': x['generator_index'],
                  'codes': x['codes_0_1_g_gplus1']} for x in maps],
        'code_legend': ['0', '1', 'g', 'g+1'],
        'distinguishing_indices': [maps[0]['distinguishing_variable_indices'],
                                   maps[2]['distinguishing_variable_indices']],
        'normality_certified': False}
(h / 'plot-data81.json').write_text(json.dumps(data, indent=2) + '\n')
colors = ['#e8eff3', '#acc9d9', '#205984', '#b87337']
labels = ['y=0, P1', 'y=0, P2', 'y=1, P1', 'y=1, P2']
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="820" height="280" viewBox="0 0 820 280">',
       '<rect width="820" height="280" fill="white"/>',
       '<text x="18" y="30" font-family="sans-serif" font-size="21" fill="#17384c">Candidate F4 quotient maps: 22 coordinate codes</text>']
for row, item in enumerate(maps):
    y = 57 + row * 43
    svg.append(f'<text x="15" y="{y+19}" font-family="sans-serif" font-size="15" fill="#17384c">{labels[row]}</text>')
    for j, code in enumerate(item['codes_0_1_g_gplus1']):
        x = 103 + j * 30
        svg.append(f'<rect x="{x}" y="{y}" width="27" height="27" fill="{colors[code]}"/>')
        svg.append(f'<text x="{x+13.5}" y="{y+19}" text-anchor="middle" font-family="sans-serif" font-size="12" fill="{"white" if code in (2,3) else "#17384c"}">{["0","1","g","+"][code]}</text>')
svg.append('<text x="103" y="244" font-family="sans-serif" font-size="13" fill="#435569">T auxiliaries (1–19)</text>')
svg.append('<text x="673" y="244" font-family="sans-serif" font-size="13" fill="#435569">x  y  p</text>')
svg.append('<text x="18" y="270" font-family="sans-serif" font-size="13" fill="#435569">+ denotes g+1. Pairwise differences occur only in auxiliary columns.</text>')
svg.append('</svg>')
(h / 'candidate81.svg').write_text('\n'.join(svg) + '\n')
report = '''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R81: four explicit candidate F4 maps
For each R80 prime quotient, the native program chose a generator $g$ with $g^2+g+1=0$, printed all 22 variable images in $\\{0,1,g,g+1\\}$, and verified the resulting map ideal equals the prime ideal in both directions.

#image("candidate81.svg", width: 100%)

The source coordinate triple $(x,y,p)$ is the same for both primes over each component: $(g,0,0)$ at $y=0$ and $(0,1,0)$ at $y=1$. Five auxiliary coordinates distinguish the first pair; six distinguish the second. These are exact maps of the rejected R50 candidate algebra. Source-field equality and local normality must be checked before the split is used as branch geometry.
'''
(h / 'report.typ').write_text(report)
typst.compile(str(h / 'report.typ'), output=str(h / 'report.pdf'))
print(json.dumps({'candidate_maps': 4, 'ideal_equalities': 4}))
