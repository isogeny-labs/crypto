"""Render the exact six-branch b,c valuation table and its degree check."""
from pathlib import Path
import json
import typst

h = Path(__file__).resolve().parent
result = json.loads((h / 'results.json').read_text())
assert len(result['runs']) == 1 and result['runs'][0]['status'] == 'accepted'
factors = [dict(x.split('=', 1) for x in s.split()[1:])
           for s in result['runs'][0]['factor_lines']]
assert [(x['fiber'], x['mask'], x['V']) for x in factors] == [
    ('0','1','square_zero_nonzero'), ('0','3','square_zero_nonzero'),
    ('1','6','1'), ('1','7','1'), ('1','8','0'), ('1','10','0')]
vals = [(21,14),(3,2),(2,2),(14,14),(1,1),(7,7)]
assert sum(2*b for b,c in vals) == 96 and sum(2*c for b,c in vals) == 80
data = [{'fiber':x['fiber'],'mask':x['mask'],'e':int(x['e']),'V':x['V'],
         'v_b':b,'v_c':c,'residue_degree':2} for x,(b,c) in zip(factors,vals)]
(h/'plot-data76.json').write_text(json.dumps(data,indent=2)+'\n')
bars=[]
for i,row in enumerate(data):
    y=66+i*42
    label=('z' if row['fiber']=='0' else 'z+1')+' / '+row['mask']
    bars.append(f'<text x="14" y="{y+16}" font-family="sans-serif" font-size="15" fill="#17384c">{label}</text>'
                f'<rect x="125" y="{y}" width="{row["v_b"]*14}" height="15" fill="#205984"/>'
                f'<rect x="125" y="{y+19}" width="{row["v_c"]*14}" height="15" fill="#b87337"/>'
                f'<text x="450" y="{y+17}" font-family="sans-serif" font-size="15" fill="#17384c">b {row["v_b"]}, c {row["v_c"]}</text>')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="690" height="350" viewBox="0 0 690 350">'+\
    '<rect width="690" height="350" fill="white"/><text x="14" y="28" font-family="sans-serif" font-size="20" fill="#17384c">Finite valuations of b and c</text>'+\
    ''.join(bars)+'<text x="14" y="337" font-family="sans-serif" font-size="14" fill="#435569">Each branch has absolute residue degree 2; weighted sums: b 96, c 80.</text></svg>'
(h/'valuations76.svg').write_text(svg+'\n')
report='''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R76: exact finite zero divisors for b and c
The accepted full-table calculation classifies all six rational omitted branches by the integral source coordinate $V$. Above $z+1$, masks 6 and 7 have $V=1$; masks 8 and 10 have $V=0$. Both $z=0$ factors have nonzero square-zero $V$. The source relation and quadratic norms give the displayed valuations.

#image("valuations76.svg", width: 100%)

The quadratic equation is Eisenstein at $z=0$. At $z+1$ it is etale, and $R+1=(z+1)(V+z^2+z+1)$ with norm of the second factor $z+1$. The source norms are $N(R)=z$ and $N(R+1)=(z+1)^3$, so these are the complete finite zero supports. Each row has residue degree two; total finite zero degrees are 96 for $b$ and 80 for $c$. The proper pole divisors have these total degrees, while individual orders at infinity remain to be resolved.

One native attempt passed exit 0, exact marker, zero-error, six source-byte pins and five donor hashes. The pinned Lean compile and axiom audit passed three authored norm/degree theorems with zero failing and zero trusted. A failed launcher attempt before compilation is retained. The local valuation bridge is stated in CM.md.
'''
(h/'report.typ').write_text(report)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'branches':6,'degree_b':96,'degree_c':80}))
