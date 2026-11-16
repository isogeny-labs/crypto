from pathlib import Path
import json,re
p=Path(__file__).resolve().parent
raw=(p/'validation01/check.stdout').read_text().splitlines();lines=[s for s in raw if s.startswith('shared_rescue_')]
r=json.loads((p/'validation01/receipt.json').read_text());assert r['compile_exit']==r['check_exit']==0
rows=[dict((k,int(v)) for k,v in re.findall(r'(\w+)=(\d+)',s)) for s in lines[1:]]
text='''# Shared interval engine — T-1880

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passes on its first attempt, including inherited checks. Twenty scalar-stream panels compare 760 delivered coefficients across shared, separate equal-width and separate native-width engines. All values agree. There are 120 rejected-key/borrow controls and 20 interleaved-stream pairs; owner cleanup and bounded batch size are checked.

| Count across 20 panels | Shared original width | Separate equal width | Separate native widths |
|---|---:|---:|---:|
| Polynomial setups | 16 | 32 | 32 |
| Progression setups | 16 | 32 | 32 |
| Evaluation batches | 10147 | 10147 | 9953 |

Sharing halves the two setup counts on these panels. The equal-width comparison isolates sharing: evaluation batches are unchanged. Keeping the original width costs 194 more batches than native sizing across these panels. No total-operation or elapsed-time conclusion follows from these counts.

## Public recovery

| Degree cap | Source calls | Recovered | Shared engines | Warm / cold shares |
|---|---:|---:|---:|---:|
'''
for x in rows:text+=f"| {x['cap']} | {x['source_calls']} | {x['recovered']}/281 | {x['shared_engines']} | {x['warm_shares']} / {x['cold_shares']} |\n"
text+='''
All 1686 target/cap comparisons match the fresh-engine rescue on scalar result, source calls and route. Cap 8 retains ten unresolved targets on each path; the other panels recover all targets. All observed public shares are cold: m=5 and 2m=10 intervals use the direct product path below length 32. Therefore these public fixtures do not demonstrate avoided warm polynomial/progression setup. The larger scalar-stream panels do exercise warm reuse.

Rc introduces an allocation for each original engine and reference-count operations. The previous inline RefCell did not require that Rc allocation. This report does not claim reduced allocation count or peak memory. Coefficient setup/advancement, width-related batches, exact-order checks, source work and final verification remain charged.

```
'''+ '\n'.join(lines)+'\n```\n\n'
text+=f"Compiler: {r['compiler']}; native source pins: {len(r['input_hashes'])}. PCC-20261107-2184b1 pins the derivation; independent signed verdict pending. No compile/native/render failure, cloud job or timing run.\n"
(p/'RESULTS.md').write_text(text)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="230"><rect width="960" height="230" fill="#f5f8fc"/><g font-family="sans-serif" fill="#17324d"><text x="20" y="30" font-size="20">One bounded engine, two validated stream identities · T-1880</text>'
for x,y,w,title,sub in [(20,65,270,'Stream (r,m,cap)','Original recurrence and cursor'),(660,65,280,'Stream (r,2m,cap)','Doubled recurrence and cursor'),(335,65,290,'Shared interval engine','Cache identity: r, width, cap')]:
 svg+=f'<rect x="{x}" y="{y}" width="{w}" height="82" rx="8" fill="#dceaf7" stroke="#477298"/><text x="{x+12}" y="{y+29}" font-size="17">{title}</text><text x="{x+12}" y="{y+57}" font-size="14">{sub}</text>'
svg+='<path d="M290 105h40m300 0h25" stroke="#17324d" stroke-width="3"/><text x="20" y="181" font-size="16">Share polynomial and progression tables; reject mismatched keys and borrow conflicts.</text><text x="20" y="210" font-size="15">Retain original width: fewer setups can coexist with more evaluation batches.</text></g></svg>'; (p/'flow.svg').write_text(svg)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="220"><rect width="960" height="220" fill="white"/><g font-family="sans-serif" fill="#17324d" font-size="16"><text x="15" y="24">Recorded setup counts · 20 panels · validation01 · exact totals</text>'
for j,(label,v,color) in enumerate([('Shared polynomial',16,'#247ba0'),('Separate polynomial',32,'#899aaf'),('Shared progression',16,'#247ba0'),('Separate progression',32,'#899aaf')]):
 y=40+39*j;svg+=f'<text x="15" y="{y+20}">{label}</text><rect x="220" y="{y}" width="{v*18}" height="27" fill="{color}"/><text x="{230+v*18}" y="{y+20}">{v}</text>'
svg+='</g></svg>'; (p/'setups.svg').write_text(svg)
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Shared interval setup for doubled rescue
T-1880 · Prime-field coefficient construction · 2026-11-07

The coefficient stream keeps its stride-specific identity. The interval-product engine can be shared because its polynomial and progression cache depend only on field, retained width and request cap.

#image("flow.svg", width: 100%)

*Checks.* Twenty larger-stride panels, 760 coefficient equalities, 120 rejected-key/borrow controls and 20 interleaved stream pairs pass. Shared ownership cleanup and bounded evaluation batches are checked.

#image("setups.svg", width: 100%)

*Width control.* Shared and separate equal-width engines both use 10,147 evaluation batches. Separate engines at their native widths use 9,953: retaining the smaller original width adds 194 batches. These are exact counts on the supplied panels, not timing measurements.

*Public recovery.* All 1,686 target/cap comparisons retain the fresh-engine rescue result, route and calls. Calls at caps 8,10,16,20,32,64 are 4,053; 3,278; 3,228; 3,224; 3,068; 2,881. Cap 8 leaves ten unresolved targets on each path; all other panels recover 281/281. Shared-engine counts are 0,260,260,260,205,0. All public shares are cold: m=5 and 2m=10 stay below the blocked-product threshold. Warm setup reuse is exercised by the larger-stride controls.

*Costs.* Rc adds a heap allocation for the original engine and reference-count operations; no allocation or peak-memory reduction is claimed. Smaller widths can add batches. All stream, source, quotient and verification work remains charged. This stays an optional adapter.

*Evidence.* validation01 passes first attempt; 98 native source hashes; inherited checks pass. PCC-20261107-2184b1 is an unsigned producer attestation; independent signed verdict pending. Original cohort unchanged; no cloud or timed workload. See DERIVATION.md and RESULTS.md.

*Sources and next.* Exact interval/context/doubled predecessor; official Rust Rc and RefCell documentation. KB unavailable. Next remove Rc ownership overhead or choose width before the first stream with an explicit doubling policy and matched controls.
''')
