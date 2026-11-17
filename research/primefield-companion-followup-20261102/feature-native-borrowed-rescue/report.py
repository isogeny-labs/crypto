from pathlib import Path
import json
p=Path(__file__).resolve().parent
r=json.loads((p/'validation02/receipt.json').read_text());life=json.loads((p/'lifetime01/receipt.json').read_text())
assert r['compile_exit']==r['check_exit']==0 and life['expected_escape_rejection']
lines=(p/'validation02/check.stdout').read_text().splitlines()[:8]
(p/'RESULTS.md').write_text('''# Borrowed interval ownership — T-1881

Producer observations; independent signed verdict pending. [pcc:unverified]

## Attempts

- validation01: compiler exit 1, E0106 (missing lifetime) in reciprocal-control.rs. Native checks did not run. Changed source and generator plus source hashes are retained.
- validation02: compiler and native exit 0 after the owned control field was annotated. All inherited checks pass.
- lifetime01: compiler exit 1 as required, diagnostic E0515 from the actual for_doubled API returning a borrow of a local owner. The validator accepts that diagnostic specifically; its check_exit is 0. The cfg fixture is compile-only.

## Ownership and layout

Three nested borrowing panels pass, sharing the same cell address across m,2m,4m wrappers and preserving the owner after inner scopes end. The production ownership constructors contain neither Rc nor a heap constructor; this is a source property, not a global allocation measurement. The generic engine vectors still allocate.

| Structure | Bytes on the checked compiler/target |
|---|---:|
| IntervalProducts engine metadata | 224 |
| Owned-or-borrowed storage enum | 240 |
| RecoveryProducts context | 264 |
| Prior Rc-based context layout | 32 |

The enum's borrowed variant still reserves space for the owned variant. Consequently this representation trades ownership allocation/reference-count work for larger inline contexts. These sizes do not measure peak stack, peak memory or elapsed time.

## Retained arithmetic

Twenty scalar-stream panels retain 760 coefficient equalities, 120 key/borrow rejections and 20 interleaved pairs. Shared polynomial/progression setups remain 16/16 versus 32/32 separately. Equal-width batches remain 10147; separate native-width batches remain 9953. The width tradeoff is unchanged.

All 1686 public target/cap comparisons retain predecessor scalar results, source calls and routes. Cap 8 retains ten unresolved targets per implementation; all other panels recover 281/281. Public shares remain cold. The source-call totals are 4053,3278,3228,3224,3068,2881 at caps 8,10,16,20,32,64 respectively.

```
'''+ '\n'.join(lines)+ '\n```\n\n'+f"Compiler: {r['compiler']}; dependency hashes: {len(r['input_hashes'])}, checked again before the escape probe. PCC-20261107-2a715d pins the derivation; independent signed verdict pending. No cloud or timing work.\n")
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="235"><rect width="960" height="235" fill="#f5f8fc"/><g font-family="sans-serif" fill="#17324d"><text x="20" y="28" font-size="20">Scoped engine borrowing · T-1881</text>'
for x,w,title,sub in [(20,280,'Original context owns engine','Owned(RefCell&lt;Engine&gt;)'),(350,270,'Doubled context borrows it','Borrowed(&amp;RefCell&lt;Engine&gt;)'),(670,270,'Owner cannot be escaped','Compile-only probe: E0515')]:
 svg+=f'<rect x="{x}" y="62" width="{w}" height="86" rx="8" fill="#dceaf7" stroke="#477298"/><text x="{x+12}" y="92" font-size="16">{title}</text><text x="{x+12}" y="124" font-size="14">{sub}</text>'
svg+='<path d="M300 105h45m275 0h45" stroke="#17324d" stroke-width="3"/><text x="20" y="184" font-size="16">Same cell, cache identity, width and query policy; no Rc ownership operations.</text><text x="20" y="214" font-size="15">Tradeoff: context layout 264 bytes versus 32-byte prior Rc handle structure.</text></g></svg>'; (p/'flow.svg').write_text(svg)
svg='<svg xmlns="http://www.w3.org/2000/svg" width="960" height="135"><rect width="960" height="135" fill="white"/><g font-family="sans-serif" fill="#17324d" font-size="16"><text x="15" y="25">Context layout bytes · checked native target · not peak memory</text>'
for y,label,n,color in [(45,'Prior Rc context',32,'#899aaf'),(86,'Inline/borrowed context',264,'#247ba0')]:
 svg+=f'<text x="15" y="{y+20}">{label}</text><rect x="230" y="{y}" width="{n*2.2}" height="28" fill="{color}"/><text x="{240+n*2.2}" y="{y+20}">{n}</text>'
svg+='</g></svg>'; (p/'layout.svg').write_text(svg)
(p/'report.typ').write_text('''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Borrowed interval ownership
T-1881 · Prime-field coefficient infrastructure · 2026-11-07

The original coefficient context owns its interval engine inline. A doubled context borrows that same cell through an explicit lifetime. The ownership constructor no longer allocates an Rc or performs reference-count operations; arithmetic vectors retain their allocations.

#image("flow.svg", width: 100%)

*Lifetime checks.* Three nested borrowing panels retain one cell address and leave the owner usable. The actual for_doubled API rejects an attempted return of a local owner's borrow with E0515 under the escape_probe cfg. The ordinary native binary compiles and runs the same source without that deliberately invalid fixture.

#image("layout.svg", width: 100%)

*Layout tradeoff.* Engine metadata is 224 bytes, the enum 240 bytes and the full context 264 bytes on the checked target. Borrowed variants reserve space for the owned variant. These are structure sizes, not peak-stack or elapsed-time measurements. No total-memory reduction is claimed.

*Retained arithmetic.* 760 coefficient equalities over 20 panels, 120 key/borrow rejections and 20 interleaved pairs pass. Shared polynomial/progression setups stay 16/16 versus 32/32 separately. Batches stay 10,147 at equal widths and 9,953 with separate native widths. All 1,686 public target/cap comparisons preserve results, routes and source calls. Cap 8 retains ten unresolved targets; other panels recover 281/281.

*Attempts.* validation01 fails compilation with E0106 in an inherited control field; its changed source is retained. validation02 passes after explicitly annotating that owned field. lifetime01 rejects the escape with the expected E0515. The successful dependency hashes are checked before the negative compiler probe.

*Evidence.* PCC-20261107-2a715d is a producer attestation; independent signed verdict pending. Original cohort unchanged. No cloud or timed workload. DERIVATION.md, RESULTS.md and README.md provide scope, failures and replay.

*Sources and next.* Exact shared-engine predecessor and official Rust lifetime documentation (doc.rust-lang.org/book/ch10-03-lifetime-syntax.html); KB unavailable. Next choose a declared possible-doubling width before first use, accounting upfront work and no-rescue cases against saved later batches.
''')
