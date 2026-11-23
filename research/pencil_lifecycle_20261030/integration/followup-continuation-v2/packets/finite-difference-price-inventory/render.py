from pathlib import Path
import json,html,textwrap,subprocess
P=Path(__file__).resolve().parent;j=json.loads((P/'inventory.json').read_text());parts=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1250" viewBox="0 0 1000 1250"><rect width="1000" height="1250" fill="white"/><g font-family="sans-serif" fill="#172b3e">']
def text(x,y,s,size=18):parts.append(f'<text x="{x}" y="{y}" font-size="{size}">{html.escape(s)}</text>')
def para(y,s):
 for line in textwrap.wrap(s,width=100):text(45,y,line,17);y+=26
 return y+15
text(45,52,'Finite-difference sieve: proposed event inventory',29)
text(45,86,'T-1520 | 2026-11-03 | source reading, no new measurements',17)
y=para(128,'Status: interpretation proposed for independent review. Historical nine-category prices are preserved. Integrated native source begins 061f4f9f; exact bindings are in inventory.json.')
text(45,y+8,'Symbolic delta: candidate minus original Horner evaluation',21);y+=50
headers=['Degree','Active reads','Base M delta','Base add delta','Visit delta'];xs=[50,175,355,535,735]
for x,h in zip(xs,headers):text(x,y,h,17)
parts.append(f'<line x1="45" y1="{y+14}" x2="940" y2="{y+14}" stroke="#557385"/>');y+=52
for r in j['low_degree_delta_new_minus_original']:
 vals=[r['degree'],r['active_reads'],r['delta_base_muls'],r['delta_base_adds'],r['delta_visits']]
 for x,v in zip(xs,vals):text(x,y,str(v),19)
 y+=48
y=para(y+14,'All entries are symbolic source-counter identities. The candidate performs two setup additions and three additions for every x, including the final unused update. The native caller uses degree two; lower degrees are component controls.')
text(45,y,'Proposed classification and source sites',22);y+=36
y=para(y,'Lines 963-968: charge k=2d+1 active coefficient reads once. Inactive array tails are ignored. Local scalar Copy and zero padding create no collection elements under the proposed baseline-consistent convention.')
y=para(y,'Lines 964-967: four get/first optional-access expressions add bounds/Option machinery. Proposed exclusion: API/control machinery, not extra collection comparisons. This requires explicit independent policy review; it does not claim those operations are free machine instructions.')
y=para(y,'Lines 970-979: two initialization additions and 3p recurrence additions enter the existing base_adds field. Replaced Horner multiplications are non-inverse work. Existing analyzer fields already map these events; no invented weight or new field is needed.')
y=para(y,'Existing exceptional predicates, batch inversion and bucket operations retain their selected event classes. Two categories change: field arithmetic and collection/graph/elimination. Seven historical categories are unaffected by the recurrence, subject to final integrated route binding.')
text(45,y,'Required next binding',22);y+=36
y=para(y,'A non-author reviewer must accept or correct the Option/copy classification and exact primitive inventory. Rebind final source sites, mapper, protocol and candidate/reference binaries. Preserve old p739 as old-arm evidence; require fresh candidate results across all 1,260 original targets.')
y=para(y,'This packet admits no scientific execution or new 36-cell ratios. Allocation, reporting, harness and infrastructure remain separate resource units under the existing policy. CSV and JSON retain every classification for review.')
parts.append('</g></svg>');assert y<1240,y
(P/'report.svg').write_text(''.join(parts));subprocess.run(['rsvg-convert','-f','pdf','-o',str(P/'report.pdf'),str(P/'report.svg')],check=True)
