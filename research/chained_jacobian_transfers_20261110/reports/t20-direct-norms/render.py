from pathlib import Path
import json,subprocess
p=Path(__file__).resolve().parent
d=json.loads((p/'counts.json').read_text())
subprocess.run(['dot','-Tsvg',str(p/'chain.dot'),'-o',str(p/'chain.svg')],check=True)
f=p/'chain.svg';f.write_text('\n'.join(x.rstrip() for x in f.read_text().splitlines())+'\n')
pieces=['<svg xmlns="http://www.w3.org/2000/svg" width="850" height="300" viewBox="0 0 850 300">','<rect width="850" height="300" fill="white"/>','<g font-family="DejaVu Sans" fill="#233943">','<text x="25" y="28" font-size="19">Complete rational input population · full43/85436f</text>','<text x="25" y="50" font-size="13">Unit: classes · n=12317 · disjoint degree categories · exact counts</text>']
for i,value in enumerate(d['balanced_degree']):
 y=77+42*i;w=570*value/max(d['balanced_degree'])
 pieces += [f'<text x="25" y="{y+18}" font-size="16">degree{i}</text>',f'<rect x="120" y="{y}" width="{max(2,w)}" height="26" fill="#397b64"/>',f'<text x="{132+w}" y="{y+19}" font-size="16">{value}</text>']
pieces+=['<text x="25" y="228" font-size="15">Wrong adjoint sign:12316 different /1 equal (identity)</text>','<text x="25" y="252" font-size="14">Repeated118 · nonsplit5116 · pole inputs2 per cover</text>','<text x="25" y="276" font-size="13">Additional controls overlap;not sampling estimates or cost observations.</text>','</g></svg>']
(p/'counts.svg').write_text('\n'.join(pieces)+'\n')
