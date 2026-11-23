#set page(paper: "a4", margin: 15mm)
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
