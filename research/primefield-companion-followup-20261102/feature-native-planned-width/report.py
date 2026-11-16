from pathlib import Path
import json,re,ast
p=Path(__file__).resolve().parent
raw=(p/'validation01/check.stdout').read_text().splitlines();lines=[s for s in raw if s.startswith('width_')]
summary=next(s for s in lines if s.startswith('width_summary'))
metrics={k:ast.literal_eval(v) for k,v in re.findall(r'(no_rescue_old|no_rescue_planned|doubled_old|doubled_planned|doubled_separate)=(\[[^]]+\])',summary)}
r=json.loads((p/'validation01/receipt.json').read_text());assert r['compile_exit']==r['check_exit']==0
text='''# Planned interval width — T-1884

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passes on the first attempt. Twenty scalar-stream panels compare 760 coefficient deliveries, with four invalid-plan rejections and twelve wrapper/fallback checks. No-rescue snapshots are taken before any doubled stream is created. Values agree in all three arms.

## Scalar-stream accounting

Counts and slot sums across the 20 independent panels:

| Metric | No rescue: old | No rescue: planned | Doubled: old shared | Doubled: planned shared | Doubled: separate native |
|---|---:|---:|---:|---:|---:|
'''
for i,name in enumerate(['Polynomial setups','Progression setups','Evaluation batches','Convolution calls','Query-tree calls','Retained live slots','Retained capacity slots']):text+='| '+name+' | '+' | '.join(str(metrics[k][i]) for k in metrics)+' |\n'
text+='''
Planning reduces combined batches by 365 versus old shared width and by 171 versus separate native widths on these panels. Combined shared retained capacity rises by 78 slots (454 to 532). The no-rescue snapshots differ: planned progression setup occurs in only 11 panels versus 16, so aggregate retained capacity there is lower despite a weakly larger width. Preserve the per-panel rows below; do not infer one monotone memory effect from width alone.

## Public recovery

All 2248 target/cap comparisons preserve scalar outcome, route and source-call count. The m=5 results remain 4053,3278,3228,3224,3068,2881 calls across caps 8,10,16,20,32,64. Only cap 8 leaves ten unresolved targets. No retained interval cache is constructed in those short-product panels.

At m=35, both caps 64 and 128 recover all 281 targets with 9486 source calls each, and neither invokes a successful doubled-phase rescue. In each panel both policies have 245 polynomial setups, 245 progression setups and 4655 batches. Planning changes convolution/query-tree counts from 490/4165 to 980/3675. Final retained capacity sums rise from 5390 to 5880 slots; the maximum per-context final capacity rises from 22 to 24 slots. These are retained cache observations, not peak memory, temporary allocations or weighted work.

The public warm-cache control demonstrates changed evaluation mix without fewer batches or queries. A convolution and a query tree need not cost the same, and their sizes differ across widths. No timing or aggregate arithmetic gain is claimed. All source grids, coefficient setup, tails, copies, inversions and group verification remain charged.

## Exact native rows

```
'''+ '\n'.join(lines)+'\n```\n\n'+f"Compiler: {r['compiler']}; native source hashes: {len(r['input_hashes'])}. PCC-20261107-fb6d9b pins the derivation; independent signed verdict pending. Original cohort unchanged; no compiler/native/render failure or cloud/timing work.\n"
(p/'RESULTS.md').write_text(text)
(p/'metrics.json').write_text(json.dumps(metrics,indent=2)+'\n')
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="215"><rect width="960" height="215" fill="#f5f8fc"/><g font-family="sans-serif" fill="#17324d"><text x="20" y="28" font-size="20">Public width plan before first use · T-1884</text>'
for x,title,sub in [(20,'Eligible public schedule','Odd m; opposite radii; 8m | r−1'),(340,'Plan width from 2m','Bound by the same block cap'),(660,'One engine for both strides','Retain separate stream identities')]:
 svg+=f'<rect x="{x}" y="60" width="280" height="80" rx="8" fill="#dceaf7" stroke="#477298"/><text x="{x+12}" y="90" font-size="16">{title}</text><text x="{x+12}" y="119" font-size="14">{sub}</text>'
svg+='<path d="M300 100h35m285 0h35" stroke="#17324d" stroke-width="3"/><text x="20" y="178" font-size="16">Record first-stream costs even if no rescue occurs; do not select width from outcomes.</text><text x="20" y="203" font-size="14">Exact coefficients and source queries stay fixed; setup, cache size and evaluation mix can change.</text></g></svg>'; (p/'flow.svg').write_text(svg)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="175"><rect width="960" height="175" fill="white"/><g font-family="sans-serif" fill="#17324d" font-size="16"><text x="15" y="24">Combined evaluation batches · 20 panels · validation01 · exact counts</text>'
for j,(label,key,color) in enumerate([('Old shared width','doubled_old','#899aaf'),('Planned shared width','doubled_planned','#247ba0'),('Separate native widths','doubled_separate','#b09a75')]):
 y=40+j*42;v=metrics[key][2];w=v/17;svg+=f'<text x="15" y="{y+20}">{label}</text><rect x="230" y="{y}" width="{w}" height="28" fill="{color}"/><text x="{240+w}" y="{y+20}">{v}</text>'
svg+='</g></svg>'; (p/'batches.svg').write_text(svg)
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Public interval-width planning
T-1884 · Prime-field coefficient construction · 2026-11-07

The optional policy chooses a bounded interval width from 2m before the first stride-m stream. It uses only public eligibility and the prescribed opposite radii. The same borrowed engine serves both streams; no initialized cache is resized.

#image("flow.svg", width: 100%)

*Exact checks.* Twenty scalar-stream panels, 760 coefficient equalities, four invalid-plan controls and twelve recovery-wrapper checks pass. All 2,248 public target/cap comparisons preserve scalar outcomes, routes and source calls.

#image("batches.svg", width: 100%)

*Scalar-stream costs.* Planned sharing reduces combined batches from 10,147 to 9,782 and query trees from 809 to 659. Retained capacity sums rise from 454 to 532 slots. The separate no-rescue snapshots have 5,040 versus 4,869 batches and 454 versus 359 capacity slots: planned progression setup occurs in 11 panels instead of 16. Per-panel rows are retained; these sums are not peak memory.

*Public warm-cache control.* E/F₂₅₇: y²=x³+x+7, P=(1,3), r=281. At m=35, caps 64 and 128 each recover 281 targets in 9,486 source calls with no successful doubled-phase rescue. Both widths use 4,655 batches. Planning changes convolutions/query trees from 490/4,165 to 980/3,675; maximum final retained capacity rises from 22 to 24 slots. A changed evaluation mix is not a priced arithmetic gain.

*Retained small-order panels.* The six m=5 panels keep their source totals and routes; only cap 8 leaves ten unresolved targets. These short products create no retained interval cache. The original cohort is unchanged.

*Evidence and limits.* validation01 passes first attempt with 102 source hashes. PCC-20261107-fb6d9b is an unsigned producer attestation; independent signed verdict pending. No cloud or timing run. Source grids, coefficients, tails, inversions, copies and final verification remain charged. The policy stays optional.

*Sources and next.* Exact interval/progression/borrowed predecessor and FLINT polynomial allocation/evaluation conventions; KB unavailable. Next count primitive field work and temporary allocations, since batches, convolutions and query trees have different costs. DERIVATION.md and RESULTS.md provide details.
''')
