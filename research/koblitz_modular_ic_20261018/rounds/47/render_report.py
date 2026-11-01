from pathlib import Path
import json,typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());c=r['cofactor_class'];svg=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="210"><rect width="780" height="210" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="16">Actual degree13collector class: source witnesses</text>'];data=[]
for i,(label,principal)in enumerate([('Signed F',r['signed_class']['signed_class_principal']),('61 F',c['sixty_one_class_principal']),('3721 F',c['cofactor_class_principal']),('11 (3721 F)',c['eleven_cofactor_principal'])]):
 y=45+i*36;color='#3b7f89'if not principal else'#a16c46';text='nonprincipal'if not principal else'principal'
 svg.append(f'<rect x="20" y="{y}" width="220" height="27" fill="{color}"/><text x="30" y="{y+18}" fill="white" font-family="sans-serif" font-size="13">{label}</text><text x="260" y="{y+18}" font-family="sans-serif" font-size="13">{text}</text>');data.append({'class':label,'principal':principal})
svg.append('<text x="20" y="202" font-family="sans-serif" font-size="11">signed47-shift0-01/cofactor47-shift0-01; retained proper/RRbridge; no chosen target image</text></svg>');(h/'round47.svg').write_text('\n'.join(svg)+'\n');(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n');typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
