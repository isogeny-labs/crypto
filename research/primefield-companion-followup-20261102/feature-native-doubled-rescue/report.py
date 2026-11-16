from pathlib import Path
import json,re
p=Path(__file__).resolve().parent
raw=(p/'validation01/check.stdout').read_text().splitlines()
lines=[s for s in raw if s.startswith('doubled_')]
rows=[{k:int(v) for k,v in re.findall(r'(\w+)=(\d+)',s)} for s in lines if s.startswith('doubled_public')]
d=json.loads((p/'validation01/receipt.json').read_text());assert d['compile_exit']==d['check_exit']==0
md='''# Conditional doubled-order rescue — T-1879

Producer observations; independent signed verdict pending. [pcc:unverified]

validation01 passed compilation and native execution on the first attempt, including inherited checks. Seven configurations (r,m): (41,5), (73,3), (97,3), (193,3), (241,3), (241,5), (281,5). The 10503 scalar-mode values cover every scalar and radii 1,2,3. The 5103 arbitrary sign vectors are 729 per configuration: exhaustive for m=3, partial for m=5. The wrong-sign control has 4050 mismatches; 21 coefficient sections and 10 invalid inputs are checked.

## Public panels

| Cap | Direct calls | Rescue calls | Saved calls | Recovered, each | Rescue contexts | Phase candidates |
|---|---:|---:|---:|---:|---:|---:|
'''
for x in rows:md+=f"| {x['cap']} | {x['reference_source_calls']} | {x['source_calls']} | {x['reference_source_calls']-x['source_calls']} | {x['recovered']}/281 | {x['contexts']} | {x['phase_candidates']} |\n"
md+='''
These are exact logical source-call counts on fixed fixtures, not timings. All 1686 target/cap executions assert no per-target call increase and preservation of every reference success. The cap-8 panel retains 10 unresolved targets on both paths. At cap 64 the helper is never invoked, so all original early recoveries remain intact. Fifteen nonopposite-schedule fixtures preserve the reference scalar, route and call count.

## Added work and misses

At caps 10,16,20: 260 extra coefficient contexts and 2080 upper F0 deliveries per panel. At cap 32: 205 contexts and 1640 deliveries. These deliveries are only the initial upper F0 stream, excluding F1/F2 and setup. At cap 8, all 265 attempts reject before constructing a context. At caps 10/16, 40 attempts miss the materialization cap; at caps 20/32, 40 are nonsingletons. No zero denominator is observed on these public fixtures, so that branch is not claimed covered by this panel. Retained reference state makes each miss fall back without querying again.

Additional costs include eta/exact-order validation, separate product setup, folded arrays, polynomial contexts, phase evaluation and verification. Therefore source-query reduction alone does not establish total-work or elapsed-time improvement. No new source grid is built for the rescue; all original source setup remains charged.

```
'''+ '\n'.join(lines) +'\n```\n\n'
md+=f"Compiler: {d['compiler']}. Native dependencies: {len(d['input_hashes'])}. PCC-20261107-67816b pins DERIVATION.md; independent signed verdict pending. Original cohort unchanged; no cloud/timing work.\n"
(p/'RESULTS.md').write_text(md)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="245"><rect width="960" height="245" fill="#f5f8fc"/><g font-family="sans-serif" fill="#17324d"><text x="20" y="28" font-size="20">Conditional rescue · T-1879 · exact public observations</text>'
for x,title,sub in [(20,'Original first radius','Keep every original early exit'),(340,'Opposite radius reached','Combine +, −, + low modes'),(660,'Order-2m singleton?','Verify scalar; otherwise fall back')]:
 svg+=f'<rect x="{x}" y="60" width="280" height="85" rx="8" fill="#dceaf7" stroke="#477298"/><text x="{x+12}" y="89" font-size="17">{title}</text><text x="{x+12}" y="118" font-size="14">{sub}</text>'
svg+='<path d="M300 103h35m285 0h35" stroke="#17324d" stroke-width="3"/><text x="20" y="180" font-size="16">No additional source queries; original compressed state and history remain available.</text><text x="20" y="210" font-size="15">Added work: coefficient context, half-degree phase supports and phase verification.</text></g></svg>'
(p/'flow.svg').write_text(svg)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="300"><rect width="960" height="300" fill="white"/><g font-family="sans-serif" fill="#17324d" font-size="14"><text x="15" y="22" font-size="17">Logical source calls · 281 targets per cap · validation01 · exact totals</text>'
for j,x in enumerate(rows):
 y=43+40*j;svg+=f'<text x="15" y="{y+15}">cap {x["cap"]}</text>'
 for offset,key,color in [(0,'reference_source_calls','#899aaf'),(16,'source_calls','#247ba0')]:
  v=x[key];w=v/5.5;svg+=f'<rect x="90" y="{y+offset}" width="{w}" height="13" fill="{color}"/><text x="{100+w}" y="{y+offset+12}">{v}</text>'
svg+='<text x="15" y="295">Gray: direct reference. Blue: conditional rescue. Cap 8 retains 10 unresolved targets on each path.</text></g></svg>'; (p/'calls.svg').write_text(svg)
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 14mm)
#set text(size: 9.5pt)
= Conditional doubled-order phase rescue
T-1879 · Prime-field compressed recovery · 2026-11-07

For odd m and 8m dividing r−1, opposite m-point orbits form one 2m-point orbit. Their three low observations combine with signs +, −, +. The new helper uses that identity after the first-radius path defers, and keeps the original state as fallback.

#image("flow.svg", width: 100%)

*Identity checks.* 10,503 scalar-mode values; 5,103 reindexed sign vectors (exhaustive for m=3, partial for m=5); 21 coefficient sections; 4,050 wrong-sign controls; 10 invalid inputs. Fifteen nonopposite public fallbacks match the original path.

#image("calls.svg", width: 100%)

*Public panel.* E/F₂₅₇: y²=x³+x+7; P=(1,3); r=281; m=5. Radii 1,280,2,3; extras 0 through 16; call cap 128. Every target/cap pair has no source-call increase and preserves every reference success. Five caps recover all 281 targets; cap 8 leaves 10 unresolved on both paths. At cap 64 no rescue context is constructed.

*Added work.* Caps 10,16,20 build 260 extra contexts and deliver 2,080 upper F0 coefficients each. Cap 32 builds 205 contexts, with 1,640 such deliveries. These exclude F1/F2 and setup. Cap misses and nonsingletons are retained. Exact-order checks, polynomial work, copies, allocations and final group verification are charged; query savings are not a runtime claim.

*Evidence.* validation01 passes on its first attempt with 94 native dependency hashes and inherited checks. PCC-20261107-67816b is a producer attestation; independent signed verdict pending. No cloud or timed workload. Original cohort unchanged. DERIVATION.md, RESULTS.md and README.md provide scope and replay.

*Sources and next.* Exact paired/direct predecessor; NIST DLMF 26.3; FFTW transform-definition conventions. Finite-field recombination is derived explicitly; KB unavailable. Next reduce or reuse the added coefficient setup, then account total arithmetic before default adoption.
''')
