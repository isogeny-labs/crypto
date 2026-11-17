"""Render native finite-action counts and explicitly pending correspondence steps."""
from pathlib import Path
from collections import Counter
import ast,re,json,html
h=Path(__file__).resolve().parent
log=(h/'runs/fiber51-01/native.log').read_text();data=[]
for cycles,count in re.findall(r'^FIBER51_CYCLE_TYPE (\[.*?\]) count=(\d+)$',log,re.M):
    data.append({'cycles':ast.literal_eval(cycles),'count':int(count)})
assert len(data)==8
(h/'cycle-plot-data.json').write_text(json.dumps({'source':'runs/fiber51-01/native.log','native_SL2_matrices':336,'native_QR_determinant_matrices':1008,'native_longest_cycle':14,'native_16_cycles':0,'rows':data},indent=2)+'\n')
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="920" height="430" viewBox="0 0 920 430"><rect width="920" height="430" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#17304a">','<text x="20" y="30" font-size="22">Exact QR7 action: no degree-16 cycle</text>','<text x="20" y="56" font-size="14">All 336 SL2(F7) matrices; all 1008 square-determinant normalizations checked.</text>']
for i,row in enumerate(data):
    label=' '.join(str(k)+(f'^{v}' if v!=1 else '') for k,v in sorted(Counter(row['cycles']).items()))
    y=80+35*i;w=row['count']/84*520
    svg += [f'<text x="20" y="{y+18}" font-size="15">{html.escape(label)}</text>',f'<rect x="180" y="{y}" width="{w:.3f}" height="24" fill="#2b718f"/>',f'<text x="{190+w:.3f}" y="{y+18}" font-size="15">{row["count"]}</text>']
svg += ['<text x="20" y="386" font-size="14">Counts are complete enumeration, not sampled frequencies. Longest cycle: 14.</text>','<text x="20" y="412" font-size="14">Application to ordinary fibers requires the recorded Frobenius/QR-action identification.</text>','</g></svg>']
(h/'cycles51.svg').write_text('\n'.join(svg)+'\n')
r=json.loads((h/'results.json').read_text())
status=next((x['status'] for x in reversed(r['runs']) if x['attempt'].startswith('hecke3-51-')),'prepared')
nodes=[('Generic Tate degree-3 quotient control',status),('Transport marked point and QR packet','not yet constructed'),('Actual T3 action on retained Jacobian module','not yet computed'),('Integral zero-eigenspace projector / scalar d','proposed Bezout or Smith certificate'),('Selected target action and divisor saturation','requires actual source-bound certificates')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="920" height="470" viewBox="0 0 920 470"><rect width="920" height="470" fill="white"/><g font-family="DejaVu Sans,sans-serif" fill="#17304a">','<text x="20" y="30" font-size="22">Direct relations: use T3 as an annihilator constraint</text>']
for i,(label,state) in enumerate(nodes):
    y=58+77*i;color='#d4eddf' if state=='accepted' else '#edf0f3'
    svg += [f'<rect x="20" y="{y}" width="875" height="61" rx="7" fill="{color}"/>',f'<text x="34" y="{y+24}" font-size="17">{html.escape(label)}</text>',f'<text x="34" y="{y+47}" font-size="14">{html.escape(state)}</text>']
    if i<4:svg += [f'<text x="440" y="{y+75}" font-size="18">↓</text>']
svg += ['<text x="20" y="458" font-size="13">CM prime 3 is inert; this is not an inferred nonzero CM map. See NEW-LEADS51.md.</text>','</g></svg>']
(h/'hecke-lead51.svg').write_text('\n'.join(svg)+'\n')
p=h/'report.typ';s=p.read_text();s+='''
#pagebreak()
= Exact criterion and the next correspondence
#image("cycles51.svg", width: 100%)
The finite group certificate rules out a single16-cycle in the stated action.
It does not prove or obstruct generic connectedness. The actual native prime
filter gives23 for level77 and11 for level161 as the first good odd split
candidates;selected residual eigenvalues remain to be checked.
#image("hecke-lead51.svg", width: 100%)
Primary references and the pinned external formalization/toolchain comparison
are recorded in NEW-LEADS51.md. Local accepted Lean declarations use the
original4.34.1/mathlibd13f23b pins. No external theorem is counted as locally
compiled or audited.
''';p.write_text(s)
