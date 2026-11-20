"""Render the two complete source-model u=0 components."""
from pathlib import Path
import json
import typst

h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text())
assert [x['status'] for x in r['runs']]==['accepted','accepted']
assert 'total_dim=32 v0_dim=16 v1_dim=16' in r['runs'][0]['result_line']
assert r['runs'][1]['theorems']==3 and r['runs'][1]['failing']==r['runs'][1]['trusted']==0
rows=[{'quadratic_place':'v=0','model_length':16,'reduced_degree':2,'model_power':8,'factor':'(lambda²+lambda+1)^8'},
      {'quadratic_place':'v=1','model_length':16,'reduced_degree':1,'model_power':16,'factor':'lambda^16'}]
assert sum(x['model_length'] for x in rows)==32
(h/'plot-data78.json').write_text(json.dumps(rows,indent=2)+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="760" height="250" viewBox="0 0 760 250">'+\
    '<rect width="760" height="250" fill="white"/><text x="18" y="30" font-family="sans-serif" font-size="20" fill="#17384c">Full u=0 source-model fiber: 32 dimensions</text>'+\
    '<text x="20" y="86" font-family="sans-serif" font-size="20" fill="#17384c">v=0</text><rect x="105" y="58" width="320" height="35" fill="#205984"/><text x="440" y="84" font-family="sans-serif" font-size="17" fill="#17384c">16; F4 reduction; power 8</text>'+\
    '<text x="20" y="154" font-family="sans-serif" font-size="20" fill="#17384c">v=1</text><rect x="105" y="126" width="320" height="35" fill="#b87337"/><text x="440" y="152" font-family="sans-serif" font-size="17" fill="#17384c">16; F2 reduction; power 16</text>'+\
    '<text x="18" y="216" font-family="sans-serif" font-size="15" fill="#435569">These are unnormalized model components; powers do not certify ramification.</text></svg>'
(h/'full-u0-78.svg').write_text(svg+'\n')
report='''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R78: complete degree-32 source-model fiber at u=0
R77's $z=infinity$ is R50's finite-$u$ chart at $u=0$: the overlap gives $U=u^2 V=v$. The accepted full-source specialization is $F(lambda,v,0)=lambda^16+(v+1)lambda^8+v+1$. The two quadratic components are $(lambda^2+lambda+1)^8$ at $v=0$ and $lambda^16$ at $v=1$.

#image("full-u0-78.svg", width: 100%)

The full model fiber has dimension 32, split 16+16; its reductions have degrees two and one. One native run passed the exact identity, dimension and wrong-factor gates. Three pinned Lean chart, eighth-power and arithmetic theorems compiled and passed the axiom audit with zero failing and zero trusted. Eleven source-byte pins and six donor hashes match.

The repeated powers are local lengths in the R50 source model. Normalization at $u=0$ is the next computation before assigning individual degree-32 branch residue degrees, ramification indices or coordinate orders.
'''
(h/'report.typ').write_text(report)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'model_dimension':32,'components':2,'lengths':[16,16]}))
