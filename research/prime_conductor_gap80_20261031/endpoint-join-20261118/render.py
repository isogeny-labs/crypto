#!/usr/bin/env python3
"""Presentation only: all displayed counts come from checked metadata."""
import json
from pathlib import Path
root=Path(__file__).resolve().parent
v=json.loads((root/'summary.json').read_text())
out=root/'reports'
out.mkdir(exist_ok=True)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="960" height="235" viewBox="0 0 960 235">
<rect width="960" height="235" fill="#f7f9fc"/>
<g font-family="DejaVu Sans,sans-serif" text-anchor="middle" fill="#172d43">
<text x="480" y="26" font-size="19">Endpoint join: original identities and observations</text>
'''
boxes=[(20,'Re-expand spec','Exact input plan'),(255,'Match endpoints','r / target / round / seed'),(490,'Check observations','Hashes / seals / identities'),(725,'Keep original JSON','Failures / missing / warmups')]
for x,a,b in boxes:
    svg+=f'<rect x="{x}" y="51" width="215" height="75" rx="9" fill="#e0ecf6" stroke="#52728e"/><text x="{x+107.5}" y="81" font-size="16">{a}</text><text x="{x+107.5}" y="106" font-size="12">{b}</text>'
    if x<725:svg+=f'<path d="M{x+218} 89h15m-5-5 5 5-5 5" stroke="#52728e" fill="none"/>'
svg+='''<rect x="170" y="151" width="620" height="61" rx="8" fill="#fff0dc" stroke="#b37b36"/>
<text x="480" y="176" font-size="16">Structural join only — no admitted comparison</text>
<text x="480" y="198" font-size="13">Explicit map, audit/replay, pricing, censoring, statistics and wall gates remain</text>
</g></svg>'''
(out/'join.svg').write_text(svg)
rows='\n'.join(f'[{r["selection"]}], [{r["planned_pairs"]}], [{r["targets"]}], [{r["warmup_pairs"]}], [{r["observed_records"]}],' for r in v['selections'])
text='''#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9pt)
#set heading(numbering:none)
= Endpoint record join
Original ecbench CLI; native codec development, 18 November 2026.

The command joins distinct endpoint workloads without rewriting their identities.
It retains each selected raw record token, including decimal spellings, and
keeps missing, failed, lower-bound, unpriced and warmup rows visible.
No result, conductor gap or cost/wall ratio is admitted. [pcc:unverified]

#image("join.svg",width:100%)

== Qualification from frozen inputs
'''
text+=f'{v["focused_tests_passed"]} focused integration controls pass against the actual library and CLI module.\n'
text+='''The checks cover raw decimal/seal preservation, complete and interrupted
synthetic codec fixtures, other-arm rows, duplicates, tampered identities,
units and session counters/digests. Synthetic outcome labels are test inputs;
no solver or timing API was invoked. Legacy and paired historical plans remain
byte-identical. Source/input/executable post-call hashes match. [pcc:unverified]

#table(columns:(2fr,1fr,1fr,1fr,1fr),inset:4pt,stroke:0.35pt,
 [*Frozen selection*],[*Pairs*],[*Targets*],[*Warmups*],[*Records*],
'''+rows+'''
)
The field202 and field96 image selections use the existing small-degree map
fixtures; coordinate selections are controls. The input plan has '''+str(v['workloads'])+' workloads and '+str(v['executions'])+''' executions. These are planned counts, with absent observations;
they are not measured repetitions or an 80-bit crossing. [pcc:unverified]

== Contract and limits
The spec is rebuilt with the existing paired schedule. The selected arm is
matched by exact subgroup order, target seed/index, round and warmup flag,
with shared algorithm seed/planted-scalar checks. Observation mode checks
finished/interrupted session digests, every record seal and planned identity,
status counts, execution count and last sequence. Other input arms/curves are
validated, not relabelled. A complete session with missing records rejects.
Plan-only mode refuses a session or nonempty observations. [pcc:unverified]

Hash consistency is not independent audit or authentication. Equal subgroup
order, scalar or seed does not certify an isogeny, conductor or identical walk.
The original at-least-80-bit prime conductor crossing needs an explicit map,
endpoint conductor/subgroup certificates and matched measured sessions.
Statistical censoring, pricing bands and target-cluster estimates, plus
isolation/A-A/registration/sample-size checks for wall results, remain open.
[pcc:unverified]

== Reproduction and retained failures
Use qualify.sh with the identified CLI and focused test executable; it starts
no session. Build attempt-a fails on a test API misuse. First qualification
retains seven passing controls and one wrong fixture-arm selection; corrected
attempt-c and qualification-b pass. Earlier receipts and source versions are
preserved. This component does not resolve cloud admission or S3 publication.
[pcc:unverified]

Producer evidence: PCC-20261118-c121f7; diagnostic failure PCC-20261118-368507.
Both are unsigned producer attestations, unverified/not covered by signed CI.
Sources: actual spec/runner/record/compare/stats contracts and measurement
protocol; see README.md. The full library test suite is not claimed passing.
[pcc:unverified]
'''
(out/'report.typ').write_text(text)
