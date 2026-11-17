"""Render the bounded candidate-fiber probe and retained failure."""
from pathlib import Path
import json
import typst

h=Path(__file__).resolve().parent
r=json.loads((h/'results.json').read_text())
assert [x['status'] for x in r['runs']]==['rejected','accepted']
assert r['candidate_component_dimensions']==[16,16] and not r['normality_certified']
data={'attempts':[{'id':x['attempt'],'status':x['status'],'error_count':x['error_count']} for x in r['runs']],
      'candidate_components':[{'quadratic_place':'y=0','dimension':16},{'quadratic_place':'y=1','dimension':16}],
      'normality_certified':False}
(h/'plot-data79.json').write_text(json.dumps(data,indent=2)+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="780" height="270" viewBox="0 0 780 270">'+\
    '<rect width="780" height="270" fill="white"/><text x="18" y="30" font-family="sans-serif" font-size="21" fill="#17384c">Rejected R50 candidate: specialized fiber dimensions</text>'+\
    '<text x="20" y="86" font-family="sans-serif" font-size="19" fill="#17384c">y=0</text><rect x="105" y="58" width="320" height="35" fill="#205984"/><text x="440" y="84" font-family="sans-serif" font-size="17" fill="#17384c">16 dimensions</text>'+\
    '<text x="20" y="154" font-family="sans-serif" font-size="19" fill="#17384c">y=1</text><rect x="105" y="126" width="320" height="35" fill="#b87337"/><text x="440" y="152" font-family="sans-serif" font-size="17" fill="#17384c">16 dimensions</text>'+\
    '<text x="18" y="218" font-family="sans-serif" font-size="15" fill="#435569">Attempt 01: parser error retained. Attempt 02: accepted, source factors vanish.</text>'+\
    '<text x="18" y="245" font-family="sans-serif" font-size="15" fill="#435569">Candidate-only result; no normality or branch-ramification certificate.</text></svg>'
(h/'candidate79.svg').write_text(svg+'\n')
report='''#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R79: bounded u=0 probe of the rejected normalP candidate
The retained R50 finite-$u$ candidate has 22 variables and 400 relations. After specializing $p=0$, its $y=0$ and $y=1$ quotient fibers are both finite of dimension 16 over F2; the respective R78 source factors reduce to zero. Their dimensions account for the full degree-32 budget.

#image("candidate79.svg", width: 100%)

The first run was rejected by the strict error gate after Singular parser errors at an ideal constructor. Its raw log is retained. The frozen repair passed exit 0, exact marker, zero-error, 12 historical byte pins and six donor hashes. The result is a diagnostic of a previously rejected normalization candidate; its reduced factors and normality remain to be checked before any branch claim.
'''
(h/'report.typ').write_text(report)
typst.compile(str(h/'report.typ'),output=str(h/'report.pdf'))
print(json.dumps({'candidate_dimensions':[16,16],'accepted':1,'rejected':1}))
