from pathlib import Path
import json,html,textwrap,subprocess
p=Path(__file__).resolve().parent;r=json.loads((p/'review.json').read_text())
s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1200" viewBox="0 0 1000 1200"><rect width="1000" height="1200" fill="white"/><g font-family="sans-serif" fill="#18354a">']
def text(x,y,t,size=18):s.append(f'<text x="{x}" y="{y}" font-size="{size}">{html.escape(t)}</text>')
def para(y,t):
 for line in textwrap.wrap(t,95):text(45,y,line,18);y+=27
 return y+15
text(45,50,'Finite-difference accounting: policy review',29)
text(45,83,'T-1528 | non-author, context-informed | source review only',17)
y=para(127,'Accepted at the local finite-event boundary. All seven source/policy bindings match. Prior producer context was available; this is not a blind review or scientific admission.')
for x,title,body in [(45,'Native recurrence','2 setup adds + 3p updates'),(360,'Recorded fields','base_adds / base_muls / visits'),(675,'Existing mapper','same finite valuation policy')]:
 s.append(f'<rect x="{x}" y="{y}" width="280" height="100" rx="8" fill="#e7f0f4" stroke="#507b94"/>');text(x+12,y+32,title,19);text(x+12,y+68,body,14)
for x in [328,643]:text(x,y+55,'→',26)
y+=145
text(45,y,'Candidate minus original: symbolic counter differences',21);y+=44
xs=[45,220,450,690]
for x,t in zip(xs,['Degree','Multiplications','Additions','Entry visits']):text(x,y,t,18)
y+=42
for row in r['counter_delta']:
 for x,t in zip(xs,[str(row['degree']),row['delta_base_muls'],row['delta_base_adds'],row['delta_visits']]):text(x,y,t,20)
 y+=41
y=para(y+10,'Degree zero and one increase additions. The native caller selects degree two. The final unused update is counted. These identities do not supply measured prices or timing ratios.')
text(45,y,'Policy correspondence',23);y+=37
y=para(y,'Active coefficient loads count once. Local scalar copies preserve the original peval read convention and create no collection elements. Padding does not consume inactive array entries.')
y=para(y,'Bounds and Option checks are access mechanics under the existing API/control convention. Algorithmic predicates still count at their selected sites. Exclusion from this finite event inventory does not assert zero machine cost.')
y=para(y,'The mapper already consumes the affected counters. Inverse normalization remains separate; the removed Horner multiplications are outside inverse expansion. No new valuation coefficient is introduced.')
text(45,y,'Remaining bindings',23);y+=37
y=para(y,'Final integrated-route category coverage, source/binary/protocol/reference bindings, fresh 1,260-target candidate results and complete resource evidence remain required. Historical prices are unchanged. Mathematical proof review is separate.')
y=para(y,'Source hashes, findings and disclosure: review.json. Review table: classification.csv. No tests, compiler invocations, scientific trials or new ratios were produced in this review.')
assert y<1180,y
s.append('</g></svg>');(p/'report.svg').write_text(''.join(s));subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
