# Root fixture recovery for the relation lane

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
