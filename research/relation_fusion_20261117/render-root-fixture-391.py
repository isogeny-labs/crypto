from pathlib import Path
import json
s=Path('research/relation_fusion_20261117');v=s/'visual-391';v.mkdir(exist_ok=True)
md='''# Root fixture recovery for the relation lane

The alias-order repair and geometric positive controls pass all seven original tests in the two actual changed source modules. The full root runtime remains unqualified: run380 was deliberately stopped after two failures. This adds correctness evidence, with no new comparative relation-pipeline timing or 15× admission. [pcc:unverified]

| Check | Retained outcome |
|---|---|
| Full8216-test runtime380 | 995 observed passes,2 failed,30ignored; intentional SIGTERM,exit-15,no final summary |
| Exact diagnostic384 | Both original failures reproduce,exit101 |
| Alias before/after385 | Old layout misses one existing generated31key; sorted layout checks all45664 aliases with zero misses |
| Geometry386 | All2400 original seeded cases plus144 genuine positives pass exact oracle/witness checks |
| Actual modules388 | All7 original tests pass against frozen actual-root352 public dependencies |
| Final actual modules390 | All7 pass after helper formatting/import finalisation;0failed/ignored/filtered |
| Full root qualification | Not established; no second full rebuild/runtime |

Every listed result is unsigned producer evidence. [pcc:unverified]

## What changed and why

`src/cryptanalysis/curve_aliases.json` contained `generated-31bit-2147483647` between two generated24bit keys. Production lookup binary-searches the raw JSON and correctly requires sorted keys. The existing generator `scripts/curve_id.py:alias_map_text` sorts aliases. Restore this ordering only: all45664 identities and all metadata are unchanged, as is the production lookup algorithm.385 independently exercises every canonical key, every key-plus-tilde gap and the original unknown/escaped inputs before and after. The direct original private-lookup/map test also passes in390. [pcc:unverified]

`src/cryptanalysis/binary_radical_root.rs` had a test described as planting positive targets, but its scan stops at2^min(n,14) and falls back to1. The old equivalence/witness assertions reached n19yes10/no190 without a mismatch, then failed the positive-count threshold.386 retains all original draws across all12trials, including previously unreached n23 random-base trials with0and1legacy positives. Do not interpret these correctness sample counts as a relation-yield estimate. [pcc:unverified]

The cfg(test)-only helper lifts rational points over the existing factor-base span using the existing ArtinSchreier solver, then sums triples with the existing affine group law on y²+xy=x³+x²+b. Require three distinct input x values,12distinct target x values outside the span, exact input/sum curve membership and S4 identities. Target construction never asks either decomposition oracle which targets to retain. Both actual production oracles must find each constructed target, with full returned-witness checks. All200 original seeded cases per trial, RNG seeds/draw order, negative-case pair checks and original yes>10/no>10 assertion remain; no production oracle algorithm changed. [pcc:unverified]

The binary-field fixture uses existing n13,17,19,23 fields. It does not supply evidence of speed, solving-degree reduction, prime-field relation yield, coefficient-tracked logarithm recovery, or the original p31/p53 performance goal. [pcc:unverified]

## Reproduction and limits

Source389 pins the actual changed source files, module-test harness and exact cached root352 dependency rlibs. Rustc compiles the actual source paths including the actual JSON literal and all7 original unit bodies. Plans388/390 pin executables, rosters, CPU0/one thread and600s/12GiB/output limits before calls; outcomes include raw hashes and unchanged source/ELF checks. Frozen full-root350 executable remains intact. Current disk space motivated this scoped qualifier, not a claim of a new full root build. [pcc:unverified]

Run380, isolated384failures, old/sorted385contexts, initial386PATH launch failure and both qualifying final source contexts remain immutable. Packet392 will retain the exact source/contexts/logs and replay executables; extracted replay393 checks its scope separately. No signed execution or independent-host attestation is claimed. [pcc:unverified]

Sources checked: existing oracle/test contracts, `koblitz_ic_leads::lambda::{add,points}`, `ic_boundary::ArtinSchreier`, and `scripts/curve_id.py:alias_map_text`. KB unavailable: repository wrapper has no configured executable; no novelty claim. Conductor binary is unavailable, as covered by docs/CONDUCTOR.md. No new external scientific mechanism is proposed.

## Remaining work

Original15× goal remains active. Stronger matched draft372 retains all36 eligible controls, all six p31/p53 n3/4/6 seed7 LAST-star cases, repeated rounds/A/A and complete proofs. This change leaves those gates and all frozen evidence/keys untouched. No leaderboard/progress graph changes: no eligible new performance result.

Real comparative timing remains dependent on canonical protocol access or an explicit narrow waiver accepting published draft372. The required [experiment-protocol SKILL.md](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/experiment-protocol/SKILL.md) says “Read the canonical SKILL.md and follow it.” Current authenticated encrypted protocol remains unavailable; supported key routes381 and bounded CI artifact alternatives382 supplied no accessible copy. The pending protocol question has no recorded answer. Any later accepted protocol still needs a separate final prospective tool/source/input declaration, strongest matched controls and original full-boundary/proof/repeat/A/A/15× gates. LocalCPU0/one numerical thread/zero cloud and checkpoint/S3 waiver persist.
'''
(s/'ROOT_FIXTURE_REPORT_391.md').write_text(md)
labels=[('Full runtime380','2 failures; incomplete','#f5e4e4'),('Exact diagnosis384','coverage + key order','#e9eff6'),('Small root fixes','all original cases kept','#e9eff6'),('Actual modules390','7/7; full suite open','#e9eff6')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="138" viewBox="0 0 1100 138"><rect width="1100" height="138" fill="white"/>']
for i,(a,b,c) in enumerate(labels):
 x=i*280;svg.append(f'<rect x="{x+2}" y="8" width="251" height="78" rx="6" fill="{c}"/>')
 for j,t in enumerate((a,b)):svg.append(f'<text x="{x+12}" y="{35+j*25}" font-family="DejaVu Sans" font-size="16" fill="#263b50">{t}</text>')
 if i<3:svg.append(f'<text x="{x+258}" y="51" font-family="DejaVu Sans" font-size="20">→</text>')
svg.append('<text x="5" y="119" font-family="DejaVu Sans" font-size="17" fill="#263b50">Correctness recovery only · 0 new comparative timing observations · original 15× goal active</text></svg>');(v/'root-recovery-scope.svg').write_text(''.join(svg))
t='''#set page(paper: "a4", margin: 15mm)
#set text(size: 9.8pt, font: "DejaVu Sans")
#set par(leading: 0.3em)
#text(size: 18pt, weight: "bold")[Root fixtures: preserve cases, repair coverage]

*Seven original module tests pass. The full root suite remains unqualified.*

Final check 390 runs every original test in the two changed source modules: seven pass, with none failed, ignored or filtered. The changes restore alias ordering and add genuine positive controls to a test. Production oracle algorithms are unchanged. These are correctness results; they establish no new speedup. [pcc:unverified]

#image("visual-391/root-recovery-scope.svg",width:100%)
#table(columns:(29%,71%),inset:4pt,
[*Check*],[*Recorded outcome*],
[Full root run 380],[995 observed passes; two failures; 30 ignored. Deliberately stopped with exit −15 and no final summary.],
[Exact replay 384],[Both failures reproduce: insufficient positive coverage and one missing alias lookup.],
[Alias checks 385],[Old ordering misses one key. Sorting preserves all 45,664 mappings and metadata; every key, gap and unknown-input check passes.],
[Geometric controls 386],[2,400 original seeded cases plus 144 genuine positives pass both production oracles and witness checks.],
[Actual modules 388 / 390],[All seven original tests pass twice. Final source is pinned by context 389. No new full root build or runtime.],
)

*Restore the required ordering.* Production lookup binary-searches raw JSON. One generated 31-bit key had been inserted among 24-bit keys. The existing generator, scripts/curve_id.py:alias_map_text, sorts aliases. The fix preserves every value, all metadata and the lookup algorithm. The original lookup/map and curve-identity tests pass. [pcc:unverified]

*Add genuine positives.* The old test's alleged planted targets used a root scan capped at 2^min(n,14), then fell back to 1. Some larger-field trials produced only zero or one positives. Every original random case, seed, draw order, equivalence assertion, witness check, negative-case pair count and coverage threshold remains. After the original stream, add 12 deterministic positive controls per trial. [pcc:unverified]

Construct controls by lifting rational points only over the existing factor-base span and summing triples with the existing group law. Require distinct input x values, distinct target x values outside the span, and exact curve-membership and S4 checks. Neither oracle selects targets. Both must then find each target and return valid witnesses. Counts describe correctness coverage, not relation yield. [pcc:unverified]

*Qualification scope.* These binary-field fixtures use n = 13, 17, 19, 23. They do not measure prime-field solving degree or the original p31/p53 performance goal. Rustc compiles the actual changed source files and JSON literal against pinned public dependencies from frozen root build 352. Context 389 and plans/outcomes 388/390 bind sources, executables, CPU 0, one thread, and resource limits. The frozen full-root executable and every earlier failure remain intact. Disk space motivated the scoped test; the full root suite remains unqualified. [pcc:unverified]

Packet 392 retains sources, contexts, raw logs and replay executables; extraction 393 checks its contents and replay scope separately. No independent-host or signed-execution attestation is claimed. Sources reviewed: existing oracle contracts, lambda::add/points, ArtinSchreier and the alias generator. The KB executable and Conductor are unavailable; no novelty claim is made. [pcc:unverified]

*Remaining dependency.* The original 15× goal stays active. Required #link("https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/experiment-protocol/SKILL.md")[experiment-protocol SKILL.md] says “Read the canonical SKILL.md and follow it.” The current encrypted protocol remains inaccessible; the pending question offers a narrow document waiver accepting published draft 372. Real comparisons require final prospective pins, all 36 strongest matched controls, all six declared cases, full proofs, repeated rounds/A/A and the original 15× gates. No eligible new timing result changes the leaderboard.

PR 4989 · CPU 0 · one numerical thread · zero cloud · checkpoint/S3 waiver persists · [pcc:unverified]
'''
(s/'root-fixture-report-391.typ').write_text(t)
import typst
typst.compile(str(s/'root-fixture-report-391.typ'),output=str(s/'root-fixture-report-391.pdf'),root=str(s.resolve()))
