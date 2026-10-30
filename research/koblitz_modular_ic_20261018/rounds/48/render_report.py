from pathlib import Path
import json,typst
h=Path(__file__).resolve().parent;r=json.loads((h/'results.json').read_text());tests=r['old_line_comparisons'];svg=['<svg xmlns="http://www.w3.org/2000/svg" width="780" height="190"><rect width="780" height="190" fill="white"/><text x="20" y="25" font-family="sans-serif" font-size="16">Complete old source-line equality tests</text>'];data=[]
for k in range(11):svg.append(f'<text x="{140+k*50}" y="55" font-family="sans-serif" font-size="12">{k}</text>')
for row,c in enumerate([7,8]):
 y=75+row*40;svg.append(f'<text x="20" y="{y+18}" font-family="sans-serif" font-size="13">new P{c} vs U{c}</text>')
 for k in range(11):
  t=next(x for x in tests if x['label']==f'new{c}-old{c}-multiple-{k}');eq=t['equivalent'];color='#367e88'if eq else'#e6e9eb';svg.append(f'<rect x="{130+k*50}" y="{y}" width="38" height="27" fill="{color}"/>');data.append({'component':c,'scalar':k,'equivalent':eq,'witness_dimension':t['witness_dimension']})
svg.append('<text x="20" y="171" font-family="sans-serif" font-size="11">compare48-01: filled cells are exact matches; F=U7+6U8; chosen-target image unevaluated</text></svg>');(h/'round48.svg').write_text('\n'.join(svg)+'\n');(h/'plot-data.json').write_text(json.dumps(data,indent=2)+'\n');typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
