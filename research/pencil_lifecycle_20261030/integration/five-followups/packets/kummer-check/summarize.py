from pathlib import Path
import json,hashlib
D=Path(__file__).parent;x=json.loads((D/'run2.stdout').read_text());rows=x['rows']
summary={'schema':'research.software-equivalence-result/v1','evidence_kind':'fixed-fixture software validation','scientific_execution':False,'models':len(set(r['eager']['p'] for r in rows)),'transition_rows':len(rows),'regular_rows':sum(not r['eager']['forced'] for r in rows),'forced_fallback_rows':sum(r['eager']['forced'] for r in rows),'matched_semantic_fields':['next','choice','labels','fallback','fallback_checks','transition_checks','validation_failed'],'successful_repair_slots':sum(z is not None for r in rows for z in r['eager']['repairs']),'dispatcher_cases':x['dispatcher_cases'],'counter_deltas':{str(f):{k:sorted(set(r['counter_delta'][k] for r in rows if r['eager']['forced']==f)) for k in ['m','a','i']} for f in [False,True]},'cost_ratio':None,'missing_coverage':['successful geometric repair','full collector run','timing or end-to-end cost comparison'],'attempts':[{'exit_code':101,'phase':'wrapper compilation','error':'E0308 map key type; corrected in additive amendment'},{'exit_code':0,'phase':'compile and all fixed checks'}]}
(D/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
report='''# Kummer branch-directed software equivalence

Fixed-fixture validation, 2026-10-30. No scientific solve or benchmark. All 84 adjacent-transition rows across 21 retained models matched the selected pair, branch choice, updated labels, fallback flag/check count and transition-validation result. This comprises 63 ordinary transitions and 21 forced-zero-difference fallbacks. Each ordinary step invokes the unchanged source's signed expected-pair validation. Forced fallback invokes its unchanged signed reconstruction and validity checks; this harness compares its outputs between versions rather than independently reimplementing that reconstruction. The 21 forced cases traverse unsuccessful repair attempts before signed fallback. No successful geometric repair was observed; this coverage remains open.

A separate 54-case dispatcher control enumerates two branches and three possible outcomes for each of three calls, including None and repair-marker outcomes. It checks Option selection logic only; it does not establish geometric repaired-coordinate correctness.

## Accounting observations
For every ordinary transition the pruned source records 24 fewer multiplications, 36 fewer additions and 4 fewer inversions in the walk counters. For each forced fallback it records 304 fewer multiplications, 376 fewer additions and 24 fewer inversions, because the discarded double and its failed repair attempts are skipped. These are observed counter differences on fixed software fixtures, not measured time or complete collector costs. Signed reconstruction/fallback output matches. Both arms retain identical signed recovery code. Source operation counters do not charge every multiplication inside modular exponentiation as a multiplication; therefore these are native counter values, not an independent field-operation census.

## Custody and failures
Before execution PLAN.md fixed the fixture selection, assertion fields, scope and120s command bound. Source snapshots retain the unchanged historical N19 collector and a copy differing only in step pruning; wrappers run transitions, never collector solve loops. Theta data was recovered privately from the tracked Git blob. One authorized artifact GET retrieved generic-models.jsonl;483118 bytes and SHA2560e71f5493f217a1524104cd8706208355f1730ce76a584ccf69d82eda89ba469 matched before use. Fixtures and source files were hashed before execution.
The first bounded build exited101 at a wrapper Map key type mismatch, before any checks ran. AMENDMENT-1.md fixes only conversion of that key to String. The second offline locked build and all checks exited0. Both receipts and stderr are preserved. No original source, frozen result, cloud job or deployment was changed.

## Interpretation and next checks
This finite check supports the selected-pair/fallback equivalence for the specified fixtures. It does not prove all arithmetic cases, subgroup input guarantees, successful geometric repair coverage, production compilation correspondence, or a full collector cost ratio. Before scientific admission, add fixed independently witnessed successful repair cases and a new candidate method identity with explicit unexecuted repair-slot telemetry. Preserve the existing N19 cohort and keep any later cohort prospective.
'''
(D/'REPORT.md').write_text(report)
svg='''<svg xmlns="http://www.w3.org/2000/svg" width="800" height="260"><rect width="800" height="260" fill="white"/><g font-family="sans-serif" font-size="17"><text x="30" y="28">Fixed software checks: matched semantic outcomes</text><text x="30" y="65">Ordinary</text><rect x="200" y="45" width="504" height="30" fill="#316a9a"/><text x="720" y="66">63</text><text x="30" y="110">Forced fallback</text><rect x="200" y="90" width="168" height="30" fill="#775c9d"/><text x="385" y="111">21</text><text x="30" y="155">Successful repair</text><line x1="200" y1="135" x2="200" y2="165" stroke="#666"/><text x="220" y="156">0 observed; coverage open</text><text x="30" y="205">Counts are fixed transition fixtures, not scientific targets or timing.</text><text x="30" y="238">Evidence: run2.stdout; 21 retained models, 84 rows.</text></g></svg>'''
(D/'coverage.svg').write_text(svg)
typ='#set page(paper:"a4",margin:18mm)\n#set text(size:10pt)\n#image("coverage.svg",width:100%)\n'
for line in report.splitlines():
 if line.startswith('# '):typ+='\n= '+line[2:]+'\n'
 elif line.startswith('## '):typ+='\n== '+line[3:]+'\n'
 elif line:typ+='\n#text('+json.dumps(line)+')\n\n'
(D/'report.typ').write_text(typ)
